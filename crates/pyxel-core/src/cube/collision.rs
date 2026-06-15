// Collision routines follow the standard short-letter notation
// (Moeller-Trumbore: h/a/f/s/q/u/v/t). Renaming hurts traceability
// against the reference formulation.
#![allow(clippy::many_single_char_names)]

use crate::cube::collider::Collider;
use crate::cube::mat4::Mat4;
use crate::cube::mesh_data::MeshData;
use crate::cube::vec3::Vec3;

// Axis-aligned bounding box used by the broad phase and by ray hit
// pre-filtering. Computed per-frame from each collider's current world
// transform; the result is a value type so callers can stash it in a
// Vec without paying for an Rc per record.

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn from_sphere(center: Vec3, radius: f32) -> Self {
        let r = radius.max(0.0);
        Self {
            min: Vec3 {
                x: center.x - r,
                y: center.y - r,
                z: center.z - r,
            },
            max: Vec3 {
                x: center.x + r,
                y: center.y + r,
                z: center.z + r,
            },
        }
    }

    // Rounded-box family AABB in world space. The local extent is
    // (size/2 + radius) along each axis; transform the 8 corners and
    // take the world-space min / max.
    pub fn from_rounded_box(transform: &Mat4, size: Vec3, radius: f32) -> Self {
        let hx = size.x.abs() * 0.5 + radius.max(0.0);
        let hy = size.y.abs() * 0.5 + radius.max(0.0);
        let hz = size.z.abs() * 0.5 + radius.max(0.0);
        let corners = [
            Vec3 {
                x: -hx,
                y: -hy,
                z: -hz,
            },
            Vec3 {
                x: hx,
                y: -hy,
                z: -hz,
            },
            Vec3 {
                x: -hx,
                y: hy,
                z: -hz,
            },
            Vec3 {
                x: hx,
                y: hy,
                z: -hz,
            },
            Vec3 {
                x: -hx,
                y: -hy,
                z: hz,
            },
            Vec3 {
                x: hx,
                y: -hy,
                z: hz,
            },
            Vec3 {
                x: -hx,
                y: hy,
                z: hz,
            },
            Vec3 {
                x: hx,
                y: hy,
                z: hz,
            },
        ];
        let mut min = Vec3 {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        };
        let mut max = Vec3 {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
            z: f32::NEG_INFINITY,
        };
        for c in &corners {
            let wc = transform.mul_vec_value(c);
            min.x = min.x.min(wc.x);
            min.y = min.y.min(wc.y);
            min.z = min.z.min(wc.z);
            max.x = max.x.max(wc.x);
            max.y = max.y.max(wc.y);
            max.z = max.z.max(wc.z);
        }
        Self { min, max }
    }

    // MeshData-collider AABB: union of every part's transformed positions.
    pub fn from_mesh(mesh: &MeshData, transform: &Mat4) -> Self {
        let world_per_part = mesh.compose_world_transforms(transform);
        let mut min = Vec3 {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        };
        let mut max = Vec3 {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
            z: f32::NEG_INFINITY,
        };
        let mut any = false;
        for (idx, world_t) in world_per_part.iter().enumerate() {
            let Some(geom_rc) = &mesh.primitives[idx] else {
                continue;
            };
            let geom = rc_ref!(geom_rc);
            let positions = &geom.positions;
            for chunk in positions.chunks_exact(3) {
                let p = Vec3 {
                    x: chunk[0],
                    y: chunk[1],
                    z: chunk[2],
                };
                let wp = world_t.mul_vec_value(&p);
                min.x = min.x.min(wp.x);
                min.y = min.y.min(wp.y);
                min.z = min.z.min(wp.z);
                max.x = max.x.max(wp.x);
                max.y = max.y.max(wp.y);
                max.z = max.z.max(wp.z);
                any = true;
            }
        }
        if !any {
            let p = transform.mul_vec_value(&Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            });
            min = p;
            max = p;
        }
        Self { min, max }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn contains_point(&self, p: Vec3) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }
}

// World-space contact record produced by the narrow phase. The normal
// points from `b` toward `a`, matching the user-facing § 12 convention.

#[derive(Clone, Copy, Debug)]
pub struct ContactGeom {
    pub point: Vec3,
    pub normal: Vec3,
    pub depth: f32,
}

// Shape classification per cube-design.md § 11.1: size == 0 → sphere,
// size = (0, h, 0) → capsule along local Y (segment half-length h/2),
// anything else → rounded box with core half-extents size/2. The
// rounding radius applies to every family (sphere radius when the core
// is a point).

#[derive(Clone, Copy, Debug)]
pub enum ColliderShape {
    Sphere { r: f32 },
    Capsule { half_h: f32, r: f32 },
    RoundedBox { half: Vec3, r: f32 },
}

pub fn classify_shape(size: Vec3, radius: f32) -> ColliderShape {
    const EPS: f32 = 1e-9;
    let r = radius.max(0.0);
    let (sx, sy, sz) = (size.x.abs(), size.y.abs(), size.z.abs());
    if sx < EPS && sz < EPS {
        if sy < EPS {
            ColliderShape::Sphere { r }
        } else {
            ColliderShape::Capsule {
                half_h: sy * 0.5,
                r,
            }
        }
    } else {
        ColliderShape::RoundedBox {
            half: Vec3 {
                x: sx * 0.5,
                y: sy * 0.5,
                z: sz * 0.5,
            },
            r,
        }
    }
}

// Resolve a Collider's world-space AABB. The collider may carry a
// rounded-box family shape or a static mesh; sphere falls out of the
// rounded-box path with size = Vec3::ZERO.
pub fn collider_aabb(collider: &Collider, transform: &Mat4) -> Aabb {
    if let Some(mesh_rc) = &collider.mesh {
        let mesh = rc_ref!(mesh_rc);
        return Aabb::from_mesh(mesh, transform);
    }
    let size = *rc_ref!(&collider.size);
    Aabb::from_rounded_box(transform, size, collider.radius)
}

// Sphere vs sphere intersection. Returns Some(ContactGeom) when the
// spheres overlap. The normal points from b toward a, and the contact
// point is on the surface of b along that normal.
pub fn sphere_vs_sphere(c_a: Vec3, r_a: f32, c_b: Vec3, r_b: f32) -> Option<ContactGeom> {
    let dx = c_a.x - c_b.x;
    let dy = c_a.y - c_b.y;
    let dz = c_a.z - c_b.z;
    let dist_sq = dx * dx + dy * dy + dz * dz;
    let r_sum = r_a + r_b;
    if dist_sq >= r_sum * r_sum {
        return None;
    }
    let dist = dist_sq.sqrt();
    let depth = r_sum - dist;
    let normal = if dist > 1e-12 {
        Vec3 {
            x: dx / dist,
            y: dy / dist,
            z: dz / dist,
        }
    } else {
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    };
    let point = Vec3 {
        x: c_b.x + normal.x * r_b,
        y: c_b.y + normal.y * r_b,
        z: c_b.z + normal.z * r_b,
    };
    Some(ContactGeom {
        point,
        normal,
        depth,
    })
}

// Sphere vs axis-aligned box. The box is passed as an AABB in world
// space; this is the world-space approximation cube uses for the
// rounded-box family (size > 0).
pub fn sphere_vs_aabb(
    sphere_center: Vec3,
    sphere_radius: f32,
    box_aabb: &Aabb,
) -> Option<ContactGeom> {
    let cx = sphere_center.x.clamp(box_aabb.min.x, box_aabb.max.x);
    let cy = sphere_center.y.clamp(box_aabb.min.y, box_aabb.max.y);
    let cz = sphere_center.z.clamp(box_aabb.min.z, box_aabb.max.z);
    let dx = sphere_center.x - cx;
    let dy = sphere_center.y - cy;
    let dz = sphere_center.z - cz;
    let dist_sq = dx * dx + dy * dy + dz * dz;
    if dist_sq >= sphere_radius * sphere_radius {
        return None;
    }
    let dist = dist_sq.sqrt();
    let (normal, depth) = if dist > 1e-12 {
        // Sphere center outside the box: normal points from the closest
        // surface point toward the sphere center; depth is how far the
        // sphere overhangs the surface along that normal.
        let n = Vec3 {
            x: dx / dist,
            y: dy / dist,
            z: dz / dist,
        };
        (n, sphere_radius - dist)
    } else {
        // Sphere center inside the box: pick the axis of smallest
        // penetration so the push-back leaves the box quickly. depth
        // must cover both the center-to-surface distance (min_pen) and
        // the sphere radius — otherwise an embedded sphere only emerges
        // partially per frame.
        let pen_x_min = sphere_center.x - box_aabb.min.x;
        let pen_x_max = box_aabb.max.x - sphere_center.x;
        let pen_y_min = sphere_center.y - box_aabb.min.y;
        let pen_y_max = box_aabb.max.y - sphere_center.y;
        let pen_z_min = sphere_center.z - box_aabb.min.z;
        let pen_z_max = box_aabb.max.z - sphere_center.z;
        let candidates = [
            (
                pen_x_min,
                Vec3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ),
            (
                pen_x_max,
                Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ),
            (
                pen_y_min,
                Vec3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
            ),
            (
                pen_y_max,
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ),
            (
                pen_z_min,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
            ),
            (
                pen_z_max,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            ),
        ];
        let mut min_pen = candidates[0].0;
        let mut min_normal = candidates[0].1;
        for (pen, n) in &candidates[1..] {
            if *pen < min_pen {
                min_pen = *pen;
                min_normal = *n;
            }
        }
        (min_normal, sphere_radius + min_pen)
    };
    Some(ContactGeom {
        point: Vec3 {
            x: cx,
            y: cy,
            z: cz,
        },
        normal,
        depth,
    })
}

// Sphere vs rounded OBB. `box_world` carries the box's rotation +
// translation (no scale, matching engine collider conventions); `half`
// is the core half-extent, `box_r` the rounding radius. The normal
// points from the box toward the sphere. Touching returns None.
pub fn sphere_vs_rounded_obb(
    c_sphere: Vec3,
    r_sphere: f32,
    box_world: &Mat4,
    half: Vec3,
    box_r: f32,
) -> Option<ContactGeom> {
    let box_r = box_r.max(0.0);
    if !vec_is_finite(c_sphere)
        || !vec_is_finite(half)
        || !r_sphere.is_finite()
        || !box_r.is_finite()
    {
        return None;
    }
    let inv = box_world.inverse_value();
    let p = inv.mul_vec_value(&c_sphere);
    if !vec_is_finite(p) {
        return None;
    }
    let q = Vec3 {
        x: p.x.clamp(-half.x, half.x),
        y: p.y.clamp(-half.y, half.y),
        z: p.z.clamp(-half.z, half.z),
    };
    let dx = p.x - q.x;
    let dy = p.y - q.y;
    let dz = p.z - q.z;
    let dist2 = dx * dx + dy * dy + dz * dz;
    let reach = r_sphere + box_r;
    if dist2 > f32::EPSILON {
        // Center outside the core box.
        let dist = dist2.sqrt();
        let depth = reach - dist;
        if depth <= 0.0 {
            return None;
        }
        let n_local = Vec3 {
            x: dx / dist,
            y: dy / dist,
            z: dz / dist,
        };
        let n_world = box_world.mul_dir_value(&n_local);
        let surf_local = Vec3 {
            x: q.x + n_local.x * box_r,
            y: q.y + n_local.y * box_r,
            z: q.z + n_local.z * box_r,
        };
        let point = box_world.mul_vec_value(&surf_local);
        return Some(ContactGeom {
            point,
            normal: n_world,
            depth,
        });
    }
    // Center inside the core box: push along the local axis with the
    // smallest margin; depth covers the interior margin plus the full
    // combined radius (mirrors the sphere_vs_aabb interior fallback).
    let margins = [
        (
            half.x - p.x,
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        ),
        (
            p.x + half.x,
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
        ),
        (
            half.y - p.y,
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ),
        (
            p.y + half.y,
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
        ),
        (
            half.z - p.z,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        ),
        (
            p.z + half.z,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        ),
    ];
    let mut margin = margins[0].0;
    let mut n_local = margins[0].1;
    for (candidate_margin, candidate_normal) in &margins[1..] {
        if *candidate_margin < margin {
            margin = *candidate_margin;
            n_local = *candidate_normal;
        }
    }
    let n_world = box_world.mul_dir_value(&n_local);
    let point = box_world.mul_vec_value(&p);
    Some(ContactGeom {
        point,
        normal: n_world,
        depth: margin + reach,
    })
}

// Capsule (local-Y segment, half-length `half_h`, radius `cap_r`) vs
// sphere. Reduces to sphere-vs-sphere at the closest segment point.
// The normal points from the capsule toward the sphere.
pub fn capsule_vs_sphere(
    cap_world: &Mat4,
    half_h: f32,
    cap_r: f32,
    c_sphere: Vec3,
    r_sphere: f32,
) -> Option<ContactGeom> {
    let top = cap_world.mul_vec_value(&Vec3 {
        x: 0.0,
        y: half_h,
        z: 0.0,
    });
    let bot = cap_world.mul_vec_value(&Vec3 {
        x: 0.0,
        y: -half_h,
        z: 0.0,
    });
    let on_axis = closest_point_on_segment(c_sphere, top, bot);
    // sphere_vs_sphere(a=sphere, b=capsule-point) yields a normal from
    // b toward a, i.e. capsule → sphere, which is what we document.
    sphere_vs_sphere(c_sphere, r_sphere, on_axis, cap_r.max(0.0))
}

// Capsule vs capsule: sphere-vs-sphere at the closest segment points
// (Ericson § 5.1.9). Normal points from b toward a.
pub fn capsule_vs_capsule(
    world_a: &Mat4,
    half_h_a: f32,
    r_a: f32,
    world_b: &Mat4,
    half_h_b: f32,
    r_b: f32,
) -> Option<ContactGeom> {
    let a_top = world_a.mul_vec_value(&Vec3 {
        x: 0.0,
        y: half_h_a,
        z: 0.0,
    });
    let a_bot = world_a.mul_vec_value(&Vec3 {
        x: 0.0,
        y: -half_h_a,
        z: 0.0,
    });
    let b_top = world_b.mul_vec_value(&Vec3 {
        x: 0.0,
        y: half_h_b,
        z: 0.0,
    });
    let b_bot = world_b.mul_vec_value(&Vec3 {
        x: 0.0,
        y: -half_h_b,
        z: 0.0,
    });
    let (on_a, on_b) = closest_points_segment_segment(a_top, a_bot, b_top, b_bot);
    sphere_vs_sphere(on_a, r_a.max(0.0), on_b, r_b.max(0.0))
}

// Capsule vs rounded OBB: take the capsule segment into the box-local
// frame, find the closest segment/box point pair, then resolve as
// sphere (capsule radius) vs rounded surface (box radius). The normal
// points from the box toward the capsule. Segments whose closest point
// lies inside the core box fall back to the sphere interior path.
pub fn capsule_vs_rounded_obb(
    cap_world: &Mat4,
    half_h: f32,
    cap_r: f32,
    box_world: &Mat4,
    half: Vec3,
    box_r: f32,
) -> Option<ContactGeom> {
    let inv = box_world.inverse_value();
    let to_local = |p: &Vec3| -> Vec3 { inv.mul_vec_value(p) };
    let top_world = cap_world.mul_vec_value(&Vec3 {
        x: 0.0,
        y: half_h,
        z: 0.0,
    });
    let top = to_local(&top_world);
    let bot_world = cap_world.mul_vec_value(&Vec3 {
        x: 0.0,
        y: -half_h,
        z: 0.0,
    });
    let bot = to_local(&bot_world);
    let (on_seg, _) = closest_points_segment_aabb(top, bot, half);
    // Resolve at the segment's closest point: identical contract to a
    // sphere of the capsule radius centered there (handles both the
    // outside gap and the inside fallback uniformly).
    let on_seg_world = box_world.mul_vec_value(&on_seg);
    // sphere_vs_rounded_obb's normal points box → "sphere" = box → capsule.
    sphere_vs_rounded_obb(on_seg_world, cap_r.max(0.0), box_world, half, box_r)
}

// Rounded OBB vs rounded OBB via 15-axis SAT (3 + 3 face axes, 9 edge
// cross products). The rounding radius is a sphere sweep, so it adds
// to the projection radius on every axis. Near-parallel edge pairs
// produce near-zero cross products and are skipped. Normal points from
// b toward a; the contact point sits on b's swept surface along the
// normal, pulled back by half the overlap — matching aabb_vs_aabb's
// overlap-midpoint convention on the normal axis (the tangential
// placement uses b's center, a practical proxy for rotated boxes).
pub fn rounded_obb_vs_rounded_obb(
    world_a: &Mat4,
    half_a: Vec3,
    r_a: f32,
    world_b: &Mat4,
    half_b: Vec3,
    r_b: f32,
) -> Option<ContactGeom> {
    let axis = |m: &Mat4, v: Vec3| -> Vec3 { m.mul_dir_value(&v) };
    let ax = [
        axis(
            world_a,
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        ),
        axis(
            world_a,
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ),
        axis(
            world_a,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        ),
    ];
    let bx = [
        axis(
            world_b,
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        ),
        axis(
            world_b,
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ),
        axis(
            world_b,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        ),
    ];
    let ha = [half_a.x, half_a.y, half_a.z];
    let hb = [half_b.x, half_b.y, half_b.z];
    let ca_rc = world_a.pos();
    let ca = *rc_ref!(&ca_rc);
    let cb_rc = world_b.pos();
    let cb = *rc_ref!(&cb_rc);
    let delta = Vec3 {
        x: ca.x - cb.x,
        y: ca.y - cb.y,
        z: ca.z - cb.z,
    };
    let dot = |u: &Vec3, v: &Vec3| u.x * v.x + u.y * v.y + u.z * v.z;
    let cross = |u: &Vec3, v: &Vec3| Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    };
    let r_sum = r_a.max(0.0) + r_b.max(0.0);

    let mut min_overlap = f32::INFINITY;
    let mut min_axis = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut test_axis = |n: Vec3| -> bool {
        let len2 = dot(&n, &n);
        if len2 < 1e-8 {
            return true; // degenerate cross axis: skip
        }
        let inv_len = 1.0 / len2.sqrt();
        let n = Vec3 {
            x: n.x * inv_len,
            y: n.y * inv_len,
            z: n.z * inv_len,
        };
        let dist = dot(&delta, &n).abs();
        let proj_a: f32 = (0..3).map(|i| (dot(&ax[i], &n) * ha[i]).abs()).sum();
        let proj_b: f32 = (0..3).map(|i| (dot(&bx[i], &n) * hb[i]).abs()).sum();
        let overlap = proj_a + proj_b + r_sum - dist;
        if overlap <= 0.0 {
            return false; // separating axis found
        }
        if overlap < min_overlap {
            min_overlap = overlap;
            // Orient from b toward a.
            min_axis = if dot(&delta, &n) >= 0.0 {
                n
            } else {
                Vec3 {
                    x: -n.x,
                    y: -n.y,
                    z: -n.z,
                }
            };
        }
        true
    };

    for a in &ax {
        if !test_axis(*a) {
            return None;
        }
    }
    for b in &bx {
        if !test_axis(*b) {
            return None;
        }
    }
    for a in &ax {
        for b in &bx {
            if !test_axis(cross(a, b)) {
                return None;
            }
        }
    }
    if !min_overlap.is_finite() || min_overlap <= 0.0 {
        return None;
    }
    // Contact point: b's swept-surface support point along the normal,
    // pulled back by half the overlap (midpoint of the interpenetration
    // band, mirroring aabb_vs_aabb).
    let support_b: f32 = (0..3)
        .map(|i| (dot(&bx[i], &min_axis) * hb[i]).abs())
        .sum::<f32>()
        + r_b.max(0.0);
    let point = Vec3 {
        x: cb.x + min_axis.x * (support_b - min_overlap * 0.5),
        y: cb.y + min_axis.y * (support_b - min_overlap * 0.5),
        z: cb.z + min_axis.z * (support_b - min_overlap * 0.5),
    };
    Some(ContactGeom {
        point,
        normal: min_axis,
        depth: min_overlap,
    })
}

// AABB vs AABB. Returns the contact along the axis of smallest overlap,
// with the normal pointing from b toward a.
pub fn aabb_vs_aabb(a: &Aabb, b: &Aabb) -> Option<ContactGeom> {
    if !a.overlaps(b) {
        return None;
    }
    let dx = a.max.x.min(b.max.x) - a.min.x.max(b.min.x);
    let dy = a.max.y.min(b.max.y) - a.min.y.max(b.min.y);
    let dz = a.max.z.min(b.max.z) - a.min.z.max(b.min.z);
    let cx_a = (a.min.x + a.max.x) * 0.5;
    let cy_a = (a.min.y + a.max.y) * 0.5;
    let cz_a = (a.min.z + a.max.z) * 0.5;
    let cx_b = (b.min.x + b.max.x) * 0.5;
    let cy_b = (b.min.y + b.max.y) * 0.5;
    let cz_b = (b.min.z + b.max.z) * 0.5;
    let (depth, normal) = if dx <= dy && dx <= dz {
        let n = if cx_a < cx_b { -1.0 } else { 1.0 };
        (
            dx,
            Vec3 {
                x: n,
                y: 0.0,
                z: 0.0,
            },
        )
    } else if dy <= dz {
        let n = if cy_a < cy_b { -1.0 } else { 1.0 };
        (
            dy,
            Vec3 {
                x: 0.0,
                y: n,
                z: 0.0,
            },
        )
    } else {
        let n = if cz_a < cz_b { -1.0 } else { 1.0 };
        (
            dz,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: n,
            },
        )
    };
    let point = Vec3 {
        x: (a.min.x.max(b.min.x) + a.max.x.min(b.max.x)) * 0.5,
        y: (a.min.y.max(b.min.y) + a.max.y.min(b.max.y)) * 0.5,
        z: (a.min.z.max(b.min.z) + a.max.z.min(b.max.z)) * 0.5,
    };
    Some(ContactGeom {
        point,
        normal,
        depth,
    })
}

// Ray-sphere intersection. Returns the (t, point, normal) of the
// closest hit in [0, max_distance], or None.
pub fn ray_vs_sphere(
    origin: Vec3,
    dir: Vec3,
    center: Vec3,
    radius: f32,
    max_distance: f32,
) -> Option<(f32, Vec3, Vec3)> {
    let oc = Vec3 {
        x: origin.x - center.x,
        y: origin.y - center.y,
        z: origin.z - center.z,
    };
    let a = dir.x * dir.x + dir.y * dir.y + dir.z * dir.z;
    if a < 1e-12 {
        return None;
    }
    let b = 2.0 * (oc.x * dir.x + oc.y * dir.y + oc.z * dir.z);
    let c = oc.x * oc.x + oc.y * oc.y + oc.z * oc.z - radius * radius;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);
    let t = if t1 >= 0.0 {
        t1
    } else if t2 >= 0.0 {
        t2
    } else {
        return None;
    };
    if t > max_distance {
        return None;
    }
    let point = Vec3 {
        x: origin.x + dir.x * t,
        y: origin.y + dir.y * t,
        z: origin.z + dir.z * t,
    };
    let nx = point.x - center.x;
    let ny = point.y - center.y;
    let nz = point.z - center.z;
    let nlen = (nx * nx + ny * ny + nz * nz).sqrt();
    let normal = if nlen > 1e-12 {
        Vec3 {
            x: nx / nlen,
            y: ny / nlen,
            z: nz / nlen,
        }
    } else {
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    };
    Some((t, point, normal))
}

// Ray-AABB intersection via the slab method. Returns (t, point, normal)
// of the entry face hit in [0, max_distance], or None.
pub fn ray_vs_aabb(
    origin: Vec3,
    dir: Vec3,
    aabb: &Aabb,
    max_distance: f32,
) -> Option<(f32, Vec3, Vec3)> {
    let mut tmin = 0.0_f32;
    let mut tmax = max_distance;
    let mut entry_axis: usize = 0;
    let mut entry_sign = 1.0_f32;

    let bounds = [
        (origin.x, dir.x, aabb.min.x, aabb.max.x),
        (origin.y, dir.y, aabb.min.y, aabb.max.y),
        (origin.z, dir.z, aabb.min.z, aabb.max.z),
    ];

    for (axis, (o, d, bmin, bmax)) in bounds.iter().enumerate() {
        if d.abs() < 1e-12 {
            if o < bmin || o > bmax {
                return None;
            }
            continue;
        }
        let t1 = (bmin - o) / d;
        let t2 = (bmax - o) / d;
        // t1 < t2 iff d > 0 (bmin < bmax guaranteed); the d == 0 case
        // is handled above. So the entry face's outward normal is the
        // axis sign opposite the ray's direction.
        let (t_near, t_far, sign_near) = if t1 < t2 {
            (t1, t2, -1.0)
        } else {
            (t2, t1, 1.0)
        };
        if t_near > tmin {
            tmin = t_near;
            entry_axis = axis;
            entry_sign = sign_near;
        }
        if t_far < tmax {
            tmax = t_far;
        }
        if tmin > tmax {
            return None;
        }
    }
    if tmin < 0.0 || tmin > max_distance {
        return None;
    }
    let point = Vec3 {
        x: origin.x + dir.x * tmin,
        y: origin.y + dir.y * tmin,
        z: origin.z + dir.z * tmin,
    };
    let normal = match entry_axis {
        0 => Vec3 {
            x: entry_sign,
            y: 0.0,
            z: 0.0,
        },
        1 => Vec3 {
            x: 0.0,
            y: entry_sign,
            z: 0.0,
        },
        _ => Vec3 {
            x: 0.0,
            y: 0.0,
            z: entry_sign,
        },
    };
    Some((tmin, point, normal))
}

// Ray-triangle intersection (Möller-Trumbore). Returns (t, point, normal)
// where the normal is the geometric face normal (cross of edges) facing
// the ray origin's half-space.
pub fn ray_vs_triangle(
    origin: Vec3,
    dir: Vec3,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    max_distance: f32,
) -> Option<(f32, Vec3, Vec3)> {
    let edge1 = Vec3 {
        x: v1.x - v0.x,
        y: v1.y - v0.y,
        z: v1.z - v0.z,
    };
    let edge2 = Vec3 {
        x: v2.x - v0.x,
        y: v2.y - v0.y,
        z: v2.z - v0.z,
    };
    let h = Vec3 {
        x: dir.y * edge2.z - dir.z * edge2.y,
        y: dir.z * edge2.x - dir.x * edge2.z,
        z: dir.x * edge2.y - dir.y * edge2.x,
    };
    let a = edge1.x * h.x + edge1.y * h.y + edge1.z * h.z;
    if a.abs() < 1e-9 {
        return None;
    }
    let f = 1.0 / a;
    let s = Vec3 {
        x: origin.x - v0.x,
        y: origin.y - v0.y,
        z: origin.z - v0.z,
    };
    let u = f * (s.x * h.x + s.y * h.y + s.z * h.z);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = Vec3 {
        x: s.y * edge1.z - s.z * edge1.y,
        y: s.z * edge1.x - s.x * edge1.z,
        z: s.x * edge1.y - s.y * edge1.x,
    };
    let v = f * (dir.x * q.x + dir.y * q.y + dir.z * q.z);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = f * (edge2.x * q.x + edge2.y * q.y + edge2.z * q.z);
    if t < 0.0 || t > max_distance {
        return None;
    }
    let point = Vec3 {
        x: origin.x + dir.x * t,
        y: origin.y + dir.y * t,
        z: origin.z + dir.z * t,
    };
    let mut nx = edge1.y * edge2.z - edge1.z * edge2.y;
    let mut ny = edge1.z * edge2.x - edge1.x * edge2.z;
    let mut nz = edge1.x * edge2.y - edge1.y * edge2.x;
    let nlen = (nx * nx + ny * ny + nz * nz).sqrt();
    if nlen > 1e-12 {
        nx /= nlen;
        ny /= nlen;
        nz /= nlen;
    }
    // Flip the normal to face the ray origin.
    if nx * dir.x + ny * dir.y + nz * dir.z > 0.0 {
        nx = -nx;
        ny = -ny;
        nz = -nz;
    }
    Some((
        t,
        point,
        Vec3 {
            x: nx,
            y: ny,
            z: nz,
        },
    ))
}

// Sphere-vs-triangle: find the closest point on the triangle to the
// sphere center, then accept the hit when that point is inside the
// sphere. Returns ContactGeom with normal pointing from the triangle
// toward the sphere center.
pub fn sphere_vs_triangle(c: Vec3, r: f32, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<ContactGeom> {
    let closest = closest_point_on_triangle(c, v0, v1, v2);
    let dx = c.x - closest.x;
    let dy = c.y - closest.y;
    let dz = c.z - closest.z;
    let dist_sq = dx * dx + dy * dy + dz * dz;
    if dist_sq >= r * r {
        return None;
    }
    let dist = dist_sq.sqrt();
    let normal = if dist > 1e-9 {
        Vec3 {
            x: dx / dist,
            y: dy / dist,
            z: dz / dist,
        }
    } else {
        // Sphere center is on the triangle; fall back to the face
        // normal. The orientation is arbitrary here — terrain shells
        // never let a thin sphere center land exactly on the surface
        // in practice.
        let e1 = Vec3 {
            x: v1.x - v0.x,
            y: v1.y - v0.y,
            z: v1.z - v0.z,
        };
        let e2 = Vec3 {
            x: v2.x - v0.x,
            y: v2.y - v0.y,
            z: v2.z - v0.z,
        };
        let nx = e1.y * e2.z - e1.z * e2.y;
        let ny = e1.z * e2.x - e1.x * e2.z;
        let nz = e1.x * e2.y - e1.y * e2.x;
        let nl = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-9);
        Vec3 {
            x: nx / nl,
            y: ny / nl,
            z: nz / nl,
        }
    };
    Some(ContactGeom {
        point: closest,
        normal,
        depth: r - dist,
    })
}

// Capsule vs triangle: global closest pair between the capsule segment
// and the triangle, then a sphere test of the capsule radius at that
// segment point (Ericson § 5.1.10 decomposition: closest point on the
// triangle to each segment endpoint, plus segment-vs-each-edge pairs).
// Normal points from the triangle toward the capsule, matching
// sphere_vs_triangle.
pub fn capsule_vs_triangle(
    cap_world: &Mat4,
    half_h: f32,
    cap_r: f32,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> Option<ContactGeom> {
    let top = cap_world.mul_vec_value(&Vec3 {
        x: 0.0,
        y: half_h,
        z: 0.0,
    });
    let bot = cap_world.mul_vec_value(&Vec3 {
        x: 0.0,
        y: -half_h,
        z: 0.0,
    });
    let d2 = |p: &Vec3, q: &Vec3| (p.x - q.x).powi(2) + (p.y - q.y).powi(2) + (p.z - q.z).powi(2);
    // Candidate pairs: (segment endpoint → triangle interior/edges) and
    // (segment ↔ each triangle edge).
    let mut best_on_seg = top;
    let mut best_dist2 = f32::INFINITY;
    for p in [top, bot] {
        let q = closest_point_on_triangle(p, v0, v1, v2);
        let d = d2(&p, &q);
        if d < best_dist2 {
            best_dist2 = d;
            best_on_seg = p;
        }
    }
    for (e0, e1) in [(v0, v1), (v1, v2), (v2, v0)] {
        let (s, q) = closest_points_segment_segment(top, bot, e0, e1);
        let d = d2(&s, &q);
        if d < best_dist2 {
            best_dist2 = d;
            best_on_seg = s;
        }
    }
    // sphere_vs_triangle re-derives the exact closest triangle point and
    // handles the depth/normal conventions (incl. the interior case).
    sphere_vs_triangle(best_on_seg, cap_r.max(0.0), v0, v1, v2)
}

// Closest point on triangle to p (Ericson, Real-Time Collision
// Detection §5.1.5). Returns the barycentric point on the triangle
// or its nearest edge / vertex.
pub(crate) fn closest_point_on_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let ab = Vec3 {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    let ac = Vec3 {
        x: c.x - a.x,
        y: c.y - a.y,
        z: c.z - a.z,
    };
    let ap = Vec3 {
        x: p.x - a.x,
        y: p.y - a.y,
        z: p.z - a.z,
    };
    let d1 = ab.x * ap.x + ab.y * ap.y + ab.z * ap.z;
    let d2 = ac.x * ap.x + ac.y * ap.y + ac.z * ap.z;
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }
    let bp = Vec3 {
        x: p.x - b.x,
        y: p.y - b.y,
        z: p.z - b.z,
    };
    let d3 = ab.x * bp.x + ab.y * bp.y + ab.z * bp.z;
    let d4 = ac.x * bp.x + ac.y * bp.y + ac.z * bp.z;
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return Vec3 {
            x: a.x + ab.x * v,
            y: a.y + ab.y * v,
            z: a.z + ab.z * v,
        };
    }
    let cp = Vec3 {
        x: p.x - c.x,
        y: p.y - c.y,
        z: p.z - c.z,
    };
    let d5 = ab.x * cp.x + ab.y * cp.y + ab.z * cp.z;
    let d6 = ac.x * cp.x + ac.y * cp.y + ac.z * cp.z;
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return Vec3 {
            x: a.x + ac.x * w,
            y: a.y + ac.y * w,
            z: a.z + ac.z * w,
        };
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return Vec3 {
            x: b.x + (c.x - b.x) * w,
            y: b.y + (c.y - b.y) * w,
            z: b.z + (c.z - b.z) * w,
        };
    }
    let denom = 1.0 / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    Vec3 {
        x: a.x + ab.x * v + ac.x * w,
        y: a.y + ab.y * v + ac.y * w,
        z: a.z + ab.z * v + ac.z * w,
    }
}

// Closest point on segment [a, b] to point p (Ericson § 5.1.2).
fn closest_point_on_segment(p: Vec3, a: Vec3, b: Vec3) -> Vec3 {
    let ab = Vec3 {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    let ab_len2 = ab.x * ab.x + ab.y * ab.y + ab.z * ab.z;
    if ab_len2 <= f32::EPSILON {
        return a;
    }
    let ap = Vec3 {
        x: p.x - a.x,
        y: p.y - a.y,
        z: p.z - a.z,
    };
    let t = ((ap.x * ab.x + ap.y * ab.y + ap.z * ab.z) / ab_len2).clamp(0.0, 1.0);
    Vec3 {
        x: a.x + ab.x * t,
        y: a.y + ab.y * t,
        z: a.z + ab.z * t,
    }
}

// Closest points between segments [p1, q1] and [p2, q2]
// (Ericson § 5.1.9, ClosestPtSegmentSegment).
pub(crate) fn closest_points_segment_segment(
    p1: Vec3,
    q1: Vec3,
    p2: Vec3,
    q2: Vec3,
) -> (Vec3, Vec3) {
    let d1 = Vec3 {
        x: q1.x - p1.x,
        y: q1.y - p1.y,
        z: q1.z - p1.z,
    };
    let d2 = Vec3 {
        x: q2.x - p2.x,
        y: q2.y - p2.y,
        z: q2.z - p2.z,
    };
    let r = Vec3 {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
        z: p1.z - p2.z,
    };
    let a = d1.x * d1.x + d1.y * d1.y + d1.z * d1.z;
    let e = d2.x * d2.x + d2.y * d2.y + d2.z * d2.z;
    let f = d2.x * r.x + d2.y * r.y + d2.z * r.z;
    let (s, t);
    if a <= f32::EPSILON && e <= f32::EPSILON {
        return (p1, p2);
    }
    if a <= f32::EPSILON {
        s = 0.0;
        t = (f / e).clamp(0.0, 1.0);
    } else {
        let c = d1.x * r.x + d1.y * r.y + d1.z * r.z;
        if e <= f32::EPSILON {
            t = 0.0;
            s = (-c / a).clamp(0.0, 1.0);
        } else {
            let b = d1.x * d2.x + d1.y * d2.y + d1.z * d2.z;
            let denom = a * e - b * b;
            // denom == 0 means parallel segments: any s works, pick 0.
            let s0 = if denom == 0.0 {
                0.0
            } else {
                ((b * f - c * e) / denom).clamp(0.0, 1.0)
            };
            let t0 = (b * s0 + f) / e;
            if t0 < 0.0 {
                t = 0.0;
                s = (-c / a).clamp(0.0, 1.0);
            } else if t0 > 1.0 {
                t = 1.0;
                s = ((b - c) / a).clamp(0.0, 1.0);
            } else {
                t = t0;
                s = s0;
            }
        }
    }
    (
        Vec3 {
            x: p1.x + d1.x * s,
            y: p1.y + d1.y * s,
            z: p1.z + d1.z * s,
        },
        Vec3 {
            x: p2.x + d2.x * t,
            y: p2.y + d2.y * t,
            z: p2.z + d2.z * t,
        },
    )
}

// Closest points between a segment and an origin-centered AABB, both
// in the box-local frame. Alternating projection between the two
// convex sets (clamp to box, project back to segment) converges to the
// global minimum pair; 8 rounds are ample for f32 contact resolution.
pub(crate) fn closest_points_segment_aabb(a: Vec3, b: Vec3, half: Vec3) -> (Vec3, Vec3) {
    let mut on_seg = Vec3 {
        x: (a.x + b.x) * 0.5,
        y: (a.y + b.y) * 0.5,
        z: (a.z + b.z) * 0.5,
    };
    let mut on_box = on_seg;
    for _ in 0..8 {
        on_box = Vec3 {
            x: on_seg.x.clamp(-half.x, half.x),
            y: on_seg.y.clamp(-half.y, half.y),
            z: on_seg.z.clamp(-half.z, half.z),
        };
        on_seg = closest_point_on_segment(on_box, a, b);
    }
    (on_seg, on_box)
}

// AABB vs triangle: thin wrapper over the box-local SAT core. Shifts
// the triangle into the AABB-centered frame, runs the sharp-box (r=0)
// test, and shifts the contact point back to world.
pub fn aabb_vs_triangle(aabb: &Aabb, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<ContactGeom> {
    let center = Vec3 {
        x: (aabb.min.x + aabb.max.x) * 0.5,
        y: (aabb.min.y + aabb.max.y) * 0.5,
        z: (aabb.min.z + aabb.max.z) * 0.5,
    };
    let extents = Vec3 {
        x: (aabb.max.x - aabb.min.x) * 0.5,
        y: (aabb.max.y - aabb.min.y) * 0.5,
        z: (aabb.max.z - aabb.min.z) * 0.5,
    };
    let shift = |v: Vec3| Vec3 {
        x: v.x - center.x,
        y: v.y - center.y,
        z: v.z - center.z,
    };
    let geom = local_box_vs_triangle(extents, 0.0, shift(v0), shift(v1), shift(v2))?;
    Some(ContactGeom {
        point: Vec3 {
            x: geom.point.x + center.x,
            y: geom.point.y + center.y,
            z: geom.point.z + center.z,
        },
        normal: geom.normal,
        depth: geom.depth,
    })
}

// Box-local triangle test via the Separating-Axis Theorem with 13 axes:
//   - 3 box face normals (X, Y, Z)
//   - 1 triangle face normal
//   - 9 cross products of box edges with triangle edges
// The box is origin-centered with half-extents `half` and swept outward
// by the rounding radius `r` (applied as a uniform extension on every
// SAT axis: exact on faces and edges, over-reporting by at most
// r·(√3−1) in corner regions). Triangle vertices are given in the same
// box-local frame; the returned ContactGeom is in that frame with the
// triangle face normal oriented toward the box center, and the
// penetration depth measured along it (a good practical proxy at PS1
// scale).
pub fn local_box_vs_triangle(
    half: Vec3,
    r: f32,
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
) -> Option<ContactGeom> {
    let r = r.max(0.0);
    let edges = [
        Vec3 {
            x: p1.x - p0.x,
            y: p1.y - p0.y,
            z: p1.z - p0.z,
        },
        Vec3 {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
            z: p2.z - p1.z,
        },
        Vec3 {
            x: p0.x - p2.x,
            y: p0.y - p2.y,
            z: p0.z - p2.z,
        },
    ];
    let aabb_axes = [
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    ];
    // SAT 13-axis pass: any separating axis disqualifies the pair.
    // The overlaps themselves are not used as depth — the triangle
    // is a thin shell, so the SAT face-normal projection collapses
    // to a point and yields zero overlap even when the box straddles
    // the plane. Depth is computed below along the face normal.
    for a in &aabb_axes {
        for e in &edges {
            let axis = Vec3 {
                x: a.y * e.z - a.z * e.y,
                y: a.z * e.x - a.x * e.z,
                z: a.x * e.y - a.y * e.x,
            };
            if axis.x.abs() < 1e-9 && axis.y.abs() < 1e-9 && axis.z.abs() < 1e-9 {
                continue;
            }
            sat_overlap(&axis, &p0, &p1, &p2, &half, r)?;
        }
    }
    for a in &aabb_axes {
        sat_overlap(a, &p0, &p1, &p2, &half, r)?;
    }
    let face_normal = {
        let e1 = edges[0];
        let e2 = Vec3 {
            x: p2.x - p0.x,
            y: p2.y - p0.y,
            z: p2.z - p0.z,
        };
        Vec3 {
            x: e1.y * e2.z - e1.z * e2.y,
            y: e1.z * e2.x - e1.x * e2.z,
            z: e1.x * e2.y - e1.y * e2.x,
        }
    };
    let fnlen_sq = face_normal.x * face_normal.x
        + face_normal.y * face_normal.y
        + face_normal.z * face_normal.z;
    if fnlen_sq < 1e-18 {
        return None;
    }
    sat_overlap(&face_normal, &p0, &p1, &p2, &half, r)?;
    // Orient the normal toward the box center (= origin in this
    // local frame). Without this, the result depends on the triangle
    // winding, which the caller cannot guarantee for arbitrary mesh
    // colliders.
    let centroid_dot = ((p0.x + p1.x + p2.x) * face_normal.x
        + (p0.y + p1.y + p2.y) * face_normal.y
        + (p0.z + p1.z + p2.z) * face_normal.z)
        / 3.0;
    let face_normal = if centroid_dot > 0.0 {
        Vec3 {
            x: -face_normal.x,
            y: -face_normal.y,
            z: -face_normal.z,
        }
    } else {
        face_normal
    };
    let fnlen = fnlen_sq.sqrt();
    let normal = Vec3 {
        x: face_normal.x / fnlen,
        y: face_normal.y / fnlen,
        z: face_normal.z / fnlen,
    };
    // Penetration depth along the (oriented) face normal: swept box
    // half-extent along the normal minus the signed distance from
    // the box center (origin) to the triangle plane.
    let r_along_normal =
        half.x * normal.x.abs() + half.y * normal.y.abs() + half.z * normal.z.abs() + r;
    let plane_offset = normal.x * p0.x + normal.y * p0.y + normal.z * p0.z;
    let depth = (r_along_normal - plane_offset.abs()).max(0.0);
    Some(ContactGeom {
        point: Vec3 {
            x: (p0.x + p1.x + p2.x) / 3.0,
            y: (p0.y + p1.y + p2.y) / 3.0,
            z: (p0.z + p1.z + p2.z) / 3.0,
        },
        normal,
        depth,
    })
}

// SAT projection helper. Returns Some(overlap) if the projections of
// the (swept) box and triangle along `axis` overlap; None if they are
// separating (which means the SAT verdict is "no collision"). `swell`
// is the world-unit sweep radius; SAT axes arrive unnormalized, so it
// is scaled by the axis length to stay in the axis' projection units.
fn sat_overlap(
    axis: &Vec3,
    p0: &Vec3,
    p1: &Vec3,
    p2: &Vec3,
    extents: &Vec3,
    swell: f32,
) -> Option<f32> {
    let pr0 = p0.x * axis.x + p0.y * axis.y + p0.z * axis.z;
    let pr1 = p1.x * axis.x + p1.y * axis.y + p1.z * axis.z;
    let pr2 = p2.x * axis.x + p2.y * axis.y + p2.z * axis.z;
    let tri_min = pr0.min(pr1).min(pr2);
    let tri_max = pr0.max(pr1).max(pr2);
    let swell_proj = if swell > 0.0 {
        swell * (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt()
    } else {
        0.0
    };
    let r =
        extents.x * axis.x.abs() + extents.y * axis.y.abs() + extents.z * axis.z.abs() + swell_proj;
    if tri_max < -r || tri_min > r {
        return None;
    }
    Some((tri_max.min(r) - tri_min.max(-r)).max(0.0))
}

fn vec_is_finite(v: Vec3) -> bool {
    v.x.is_finite() && v.y.is_finite() && v.z.is_finite()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn test_aabb_from_sphere() {
        let aabb = Aabb::from_sphere(
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            0.5,
        );
        assert!(approx_eq(aabb.min.x, 0.5));
        assert!(approx_eq(aabb.max.x, 1.5));
    }

    #[test]
    fn test_aabb_overlap() {
        let a = Aabb::from_sphere(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        );
        let b = Aabb::from_sphere(
            Vec3 {
                x: 1.5,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        );
        assert!(a.overlaps(&b));
        let c = Aabb::from_sphere(
            Vec3 {
                x: 5.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        );
        assert!(!a.overlaps(&c));
    }

    #[test]
    fn test_sphere_vs_sphere_hit() {
        let r = sphere_vs_sphere(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            Vec3 {
                x: 1.5,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        )
        .unwrap();
        assert!(approx_eq(r.depth, 0.5));
        assert!(approx_eq(r.normal.x, -1.0));
    }

    #[test]
    fn test_sphere_vs_sphere_miss() {
        let r = sphere_vs_sphere(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            Vec3 {
                x: 3.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        );
        assert!(r.is_none());
    }

    #[test]
    fn test_sphere_vs_aabb_hit_from_side() {
        let aabb = Aabb {
            min: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Vec3 {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
        };
        let r = sphere_vs_aabb(
            Vec3 {
                x: -0.5,
                y: 1.0,
                z: 1.0,
            },
            1.0,
            &aabb,
        )
        .unwrap();
        assert!(approx_eq(r.depth, 0.5));
        assert!(approx_eq(r.normal.x, -1.0));
    }

    #[test]
    fn test_aabb_vs_aabb_hit() {
        let a = Aabb {
            min: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Vec3 {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
        };
        let b = Aabb {
            min: Vec3 {
                x: 1.5,
                y: 0.5,
                z: 0.5,
            },
            max: Vec3 {
                x: 3.5,
                y: 1.5,
                z: 1.5,
            },
        };
        let r = aabb_vs_aabb(&a, &b).unwrap();
        assert!(approx_eq(r.depth, 0.5));
        assert!(approx_eq(r.normal.x, -1.0));
    }

    #[test]
    fn test_ray_vs_sphere_hit() {
        let (t, _, _) = ray_vs_sphere(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            f32::INFINITY,
        )
        .unwrap();
        assert!(approx_eq(t, 4.0));
    }

    #[test]
    fn test_ray_vs_aabb_hit() {
        let aabb = Aabb {
            min: Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            max: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        };
        let (t, _, normal) = ray_vs_aabb(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            &aabb,
            f32::INFINITY,
        )
        .unwrap();
        assert!(approx_eq(t, 4.0));
        assert!(approx_eq(normal.z, -1.0));
    }

    #[test]
    fn test_ray_vs_triangle_hit() {
        let (t, _, _) = ray_vs_triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            f32::INFINITY,
        )
        .unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn test_sphere_vs_triangle_hit_above_face() {
        // Triangle in the z=0 plane; sphere center hovers above the
        // centroid at z=+0.3 with radius 0.5 → 0.2 penetration depth.
        let v0 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v1 = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let r = sphere_vs_triangle(
            Vec3 {
                x: 0.25,
                y: 0.25,
                z: 0.3,
            },
            0.5,
            v0,
            v1,
            v2,
        )
        .unwrap();
        assert!(approx_eq(r.depth, 0.2));
        assert!(approx_eq(r.normal.z, 1.0));
    }

    #[test]
    fn test_sphere_vs_triangle_miss_outside_face() {
        let v0 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v1 = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let r = sphere_vs_triangle(
            Vec3 {
                x: 5.0,
                y: 5.0,
                z: 0.3,
            },
            0.5,
            v0,
            v1,
            v2,
        );
        assert!(r.is_none());
    }

    #[test]
    fn test_aabb_vs_triangle_hit_through_face() {
        // Triangle in z=0 plane; AABB straddles z=0.
        let v0 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v1 = Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        };
        let aabb = Aabb {
            min: Vec3 {
                x: 0.5,
                y: 0.5,
                z: -0.5,
            },
            max: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.5,
            },
        };
        let r = aabb_vs_triangle(&aabb, v0, v1, v2).unwrap();
        assert!(r.depth > 0.0);
    }

    #[test]
    fn test_aabb_vs_triangle_miss_far_away() {
        let v0 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v1 = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let aabb = Aabb {
            min: Vec3 {
                x: 10.0,
                y: 10.0,
                z: 10.0,
            },
            max: Vec3 {
                x: 11.0,
                y: 11.0,
                z: 11.0,
            },
        };
        let r = aabb_vs_triangle(&aabb, v0, v1, v2);
        assert!(r.is_none());
    }

    #[test]
    fn test_sphere_vs_aabb_interior_fallback_depth_covers_radius_plus_pen() {
        // Sphere center fully inside the box. Pre-fix the depth was
        // sphere_radius (= 1.0) regardless of how deep the center sat,
        // so a centred sphere never popped out in one frame. The fix
        // ensures depth = radius + min_pen so the push-back fully
        // clears the box.
        let aabb = Aabb {
            min: Vec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0,
            },
            max: Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        };
        let center = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let r = sphere_vs_aabb(center, 0.5, &aabb).unwrap();
        // Minimum penetration is along ±X (X half-extent = 1.0 is the
        // smallest), so min_pen = 1.0 and depth should be 1.5.
        assert!(
            (r.depth - 1.5).abs() < 1e-4,
            "interior depth {} != 1.5",
            r.depth
        );
        assert!(r.normal.x.abs() > 0.999, "normal not on ±X axis");
        assert!(r.normal.y.abs() < 1e-4);
        assert!(r.normal.z.abs() < 1e-4);
    }

    #[test]
    fn test_sphere_vs_aabb_interior_picks_smallest_axis() {
        // Off-centre interior: min_pen along +Y direction (y half =
        // 1.0, center at y = 0.6 → +Y face at 0.4 distance is smallest).
        let aabb = Aabb {
            min: Vec3 {
                x: -2.0,
                y: -1.0,
                z: -2.0,
            },
            max: Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0,
            },
        };
        let center = Vec3 {
            x: 0.0,
            y: 0.6,
            z: 0.0,
        };
        let r = sphere_vs_aabb(center, 0.5, &aabb).unwrap();
        // Push toward +Y so depth = radius + (1.0 - 0.6) = 0.9.
        assert!(r.normal.y > 0.99, "normal should point +Y");
        assert!((r.depth - 0.9).abs() < 1e-4);
    }

    #[test]
    fn test_sphere_vs_sphere_touching_returns_none() {
        // Distance == r_sum is the touching limit (not penetrating).
        let r = sphere_vs_sphere(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            Vec3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        );
        assert!(r.is_none());
    }

    #[test]
    fn test_sphere_vs_sphere_coincident_centers_uses_fallback_normal() {
        let r = sphere_vs_sphere(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
        )
        .unwrap();
        // Fallback normal points along +Y when centers coincide.
        assert!((r.normal.y - 1.0).abs() < 1e-4);
        assert!((r.depth - 2.0).abs() < 1e-4);
    }

    #[test]
    fn test_aabb_vs_aabb_picks_smallest_axis() {
        // Overlap is 2 on X, 0.5 on Y, 1 on Z → Y wins.
        let a = Aabb {
            min: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0,
            },
        };
        let b = Aabb {
            min: Vec3 {
                x: 0.0,
                y: 0.5,
                z: 1.0,
            },
            max: Vec3 {
                x: 2.0,
                y: 1.5,
                z: 3.0,
            },
        };
        let r = aabb_vs_aabb(&a, &b).unwrap();
        assert!(r.normal.y.abs() > 0.99);
        assert!((r.depth - 0.5).abs() < 1e-4);
    }

    #[test]
    fn test_aabb_vs_aabb_no_overlap() {
        let a = Aabb {
            min: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        };
        let b = Aabb {
            min: Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
            max: Vec3 {
                x: 6.0,
                y: 6.0,
                z: 6.0,
            },
        };
        assert!(aabb_vs_aabb(&a, &b).is_none());
    }

    #[test]
    fn test_ray_vs_sphere_miss() {
        let r = ray_vs_sphere(
            Vec3 {
                x: 0.0,
                y: 5.0,
                z: -5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            f32::INFINITY,
        );
        assert!(r.is_none());
    }

    #[test]
    fn test_ray_vs_sphere_zero_direction_rejected() {
        let r = ray_vs_sphere(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            f32::INFINITY,
        );
        assert!(r.is_none());
    }

    #[test]
    fn test_ray_vs_sphere_max_distance_cap() {
        // Ray hits at t=4 with infinite cap, misses with cap=3.5.
        let origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        };
        let dir = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let center = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert!(ray_vs_sphere(origin, dir, center, 1.0, 4.5).is_some());
        assert!(ray_vs_sphere(origin, dir, center, 1.0, 3.5).is_none());
    }

    #[test]
    fn test_ray_vs_aabb_misses_when_pointed_away() {
        let aabb = Aabb {
            min: Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            max: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        };
        let r = ray_vs_aabb(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            &aabb,
            f32::INFINITY,
        );
        assert!(r.is_none());
    }

    #[test]
    fn test_ray_vs_aabb_parallel_outside_misses() {
        // Ray parallel to X axis, origin above the box.
        let aabb = Aabb {
            min: Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            max: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        };
        let r = ray_vs_aabb(
            Vec3 {
                x: -5.0,
                y: 5.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            &aabb,
            f32::INFINITY,
        );
        assert!(r.is_none());
    }

    #[test]
    fn test_ray_vs_aabb_face_normal_each_axis() {
        let aabb = Aabb {
            min: Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            max: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        };
        // From -X: normal = (-1, 0, 0)
        let (_, _, n) = ray_vs_aabb(
            Vec3 {
                x: -5.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            &aabb,
            f32::INFINITY,
        )
        .unwrap();
        assert!((n.x - (-1.0)).abs() < 1e-4);
        // From +Y: normal = (0, 1, 0)
        let (_, _, n) = ray_vs_aabb(
            Vec3 {
                x: 0.0,
                y: 5.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            &aabb,
            f32::INFINITY,
        )
        .unwrap();
        assert!((n.y - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_ray_vs_triangle_parallel_misses() {
        let (v0, v1, v2) = (
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
        // Ray parallel to the z=0 triangle plane.
        let r = ray_vs_triangle(
            Vec3 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            v0,
            v1,
            v2,
            f32::INFINITY,
        );
        assert!(r.is_none());
    }

    #[test]
    fn test_ray_vs_triangle_outside_uv_misses() {
        let (v0, v1, v2) = (
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
        // Ray crosses the plane but well outside the triangle's UV.
        let r = ray_vs_triangle(
            Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            v0,
            v1,
            v2,
            f32::INFINITY,
        );
        assert!(r.is_none());
    }

    #[test]
    fn test_sphere_vs_triangle_edge_hit() {
        // Sphere center sits over the AB edge at (0.5, -0.4, 0). The
        // closest point on the triangle is on the edge, not a vertex
        // and not the face interior.
        let (v0, v1, v2) = (
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
        let r = sphere_vs_triangle(
            Vec3 {
                x: 0.5,
                y: -0.2,
                z: 0.0,
            },
            0.5,
            v0,
            v1,
            v2,
        )
        .unwrap();
        // Closest point should be on the edge segment AB at (0.5, 0, 0).
        assert!((r.point.x - 0.5).abs() < 1e-4);
        assert!(r.point.y.abs() < 1e-4);
        // Depth = sphere radius - distance from center to closest point.
        assert!((r.depth - 0.3).abs() < 1e-4);
    }

    #[test]
    fn test_sphere_vs_triangle_vertex_hit() {
        let (v0, v1, v2) = (
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
        let r = sphere_vs_triangle(
            Vec3 {
                x: -0.2,
                y: -0.2,
                z: 0.0,
            },
            0.5,
            v0,
            v1,
            v2,
        )
        .unwrap();
        // Closest is vertex a (0, 0, 0).
        assert!(r.point.x.abs() < 1e-4);
        assert!(r.point.y.abs() < 1e-4);
    }

    #[test]
    fn test_closest_point_on_triangle_regions() {
        let a = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let c = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        // Outside near vertex a.
        let p = Vec3 {
            x: -1.0,
            y: -1.0,
            z: 0.0,
        };
        let cp = closest_point_on_triangle(p, a, b, c);
        assert!(cp.x.abs() < 1e-4 && cp.y.abs() < 1e-4);
        // Outside near vertex b.
        let p = Vec3 {
            x: 2.0,
            y: -1.0,
            z: 0.0,
        };
        let cp = closest_point_on_triangle(p, a, b, c);
        assert!((cp.x - 1.0).abs() < 1e-4 && cp.y.abs() < 1e-4);
        // Outside near vertex c.
        let p = Vec3 {
            x: -1.0,
            y: 2.0,
            z: 0.0,
        };
        let cp = closest_point_on_triangle(p, a, b, c);
        assert!(cp.x.abs() < 1e-4 && (cp.y - 1.0).abs() < 1e-4);
        // Interior projects straight down to the plane.
        let p = Vec3 {
            x: 0.25,
            y: 0.25,
            z: 5.0,
        };
        let cp = closest_point_on_triangle(p, a, b, c);
        assert!((cp.x - 0.25).abs() < 1e-4);
        assert!((cp.y - 0.25).abs() < 1e-4);
        assert!(cp.z.abs() < 1e-4);
    }

    #[test]
    fn test_collider_aabb_for_sphere_collider() {
        let coll = crate::cube::Collider::new(
            crate::cube::Vec3::zero(),
            0.5,
            None,
            false,
            false,
            1.0,
            0.0,
            0.5,
            crate::cube::Vec3::zero(),
            crate::cube::Vec3::zero(),
        );
        let transform_rc = crate::cube::Mat4::from_translation(&Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let transform = *rc_ref!(&transform_rc);
        let aabb = collider_aabb(rc_ref!(&coll), &transform);
        assert!((aabb.min.x - 0.5).abs() < 1e-4); // 1 - 0.5
        assert!((aabb.max.x - 1.5).abs() < 1e-4);
        assert!((aabb.min.y - 1.5).abs() < 1e-4);
        assert!((aabb.max.y - 2.5).abs() < 1e-4);
    }

    #[test]
    fn test_aabb_from_rounded_box_rotated_grows_extent() {
        // A 1x1x1 box rotated 45° around Y projects to a larger AABB
        // along X / Z (full diagonal ≈ sqrt(2)).
        let rot_rc = crate::cube::Mat4::from_axis_angle(
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            45.0,
        );
        let rot = *rc_ref!(&rot_rc);
        let aabb = Aabb::from_rounded_box(
            &rot,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            0.0,
        );
        let extent_x = aabb.max.x - aabb.min.x;
        // sqrt(2) ≈ 1.414; the non-rotated extent would be 1.0.
        assert!(extent_x > 1.2);
    }

    #[test]
    fn test_classify_sphere() {
        let s = classify_shape(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            0.5,
        );
        assert!(matches!(s, ColliderShape::Sphere { r } if approx_eq(r, 0.5)));
    }

    #[test]
    fn test_classify_capsule_local_y() {
        // size=(0, h, 0) + radius → capsule, half_h = h/2 (cube-design.md § 11.1).
        let s = classify_shape(
            Vec3 {
                x: 0.0,
                y: 1.2,
                z: 0.0,
            },
            0.3,
        );
        assert!(
            matches!(s, ColliderShape::Capsule { half_h, r } if approx_eq(half_h, 0.6) && approx_eq(r, 0.3))
        );
    }

    #[test]
    fn test_classify_rounded_box_and_sharp_box() {
        let s = classify_shape(
            Vec3 {
                x: 2.0,
                y: 1.0,
                z: 0.5,
            },
            0.1,
        );
        assert!(matches!(
            s,
            ColliderShape::RoundedBox { half, r }
                if approx_eq(half.x, 1.0) && approx_eq(half.y, 0.5) && approx_eq(half.z, 0.25) && approx_eq(r, 0.1)
        ));
        // radius = 0 stays in the box family (sharp box).
        let s0 = classify_shape(
            Vec3 {
                x: 2.0,
                y: 1.0,
                z: 0.5,
            },
            0.0,
        );
        assert!(matches!(s0, ColliderShape::RoundedBox { r, .. } if approx_eq(r, 0.0)));
        // A planar size (one zero component among x/z) is still a box.
        let plate = classify_shape(
            Vec3 {
                x: 2.0,
                y: 0.0,
                z: 2.0,
            },
            0.0,
        );
        assert!(matches!(plate, ColliderShape::RoundedBox { .. }));
    }

    #[test]
    fn test_closest_point_on_segment_clamps_to_endpoints() {
        let a = Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };
        let b = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let mid = closest_point_on_segment(
            Vec3 {
                x: 3.0,
                y: 0.5,
                z: 0.0,
            },
            a,
            b,
        );
        assert!(approx_eq(mid.y, 0.5));
        let below = closest_point_on_segment(
            Vec3 {
                x: 0.0,
                y: -9.0,
                z: 0.0,
            },
            a,
            b,
        );
        assert!(approx_eq(below.y, -1.0));
        // Degenerate zero-length segment returns the endpoint.
        let pt = closest_point_on_segment(
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            a,
            a,
        );
        assert!(approx_eq(pt.y, -1.0));
    }

    #[test]
    fn test_segment_segment_crossing_and_parallel() {
        // Perpendicular cross at distance 1 on Z.
        let (p, q) = closest_points_segment_segment(
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 1.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            },
        );
        assert!(approx_eq(p.x, 0.0) && approx_eq(p.z, 0.0));
        assert!(approx_eq(q.y, 0.0) && approx_eq(q.z, 1.0));
        // Parallel overlapping segments: any valid pair has distance 2 on X.
        let (p2, q2) = closest_points_segment_segment(
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 2.0,
                y: -1.0,
                z: 0.0,
            },
            Vec3 {
                x: 2.0,
                y: 1.0,
                z: 0.0,
            },
        );
        let d = ((p2.x - q2.x).powi(2) + (p2.y - q2.y).powi(2) + (p2.z - q2.z).powi(2)).sqrt();
        assert!(approx_eq(d, 2.0));
    }

    #[test]
    fn test_sphere_vs_rounded_obb_axis_aligned_matches_aabb_path() {
        // Identity transform, half=(1,1,1), box_r=0: must agree with the
        // sphere_vs_aabb result for the same configuration.
        let m = Mat4::identity_value();
        let geom = sphere_vs_rounded_obb(
            Vec3 {
                x: 1.4,
                y: 0.0,
                z: 0.0,
            },
            0.5,
            &m,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            0.0,
        )
        .unwrap();
        // Closest box point (1, 0, 0); gap 0.4; depth = 0.5 - 0.4 = 0.1.
        assert!(approx_eq(geom.depth, 0.1));
        assert!(approx_eq(geom.normal.x, 1.0));
        assert!(approx_eq(geom.point.x, 1.0));
    }

    #[test]
    fn test_sphere_vs_rounded_obb_rotated_45_no_corner_inflation() {
        // Unit-half cube rotated 45° about Y. Its true reach along the world
        // diagonal u=(1,0,1)/√2 is 1.0 (one local axis aligns with u, the
        // other horizontal axis is perpendicular). The world-AABB
        // approximation reached 2.0 along u and reported phantom contacts.
        let rot_rc = Mat4::from_euler(&Vec3 {
            x: 0.0,
            y: 45.0,
            z: 0.0,
        });
        let rot = *rc_ref!(&rot_rc);
        let d = 1.5 / std::f32::consts::SQRT_2;
        let c = Vec3 { x: d, y: 0.0, z: d }; // 1.5 along u, true gap 0.5
        assert!(sphere_vs_rounded_obb(
            c,
            0.3,
            &rot,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            0.0,
        )
        .is_none());
        // 1.2 along u with r=0.3 → depth = 0.3 - 0.2 = 0.1, normal = u.
        let d2 = 1.2 / std::f32::consts::SQRT_2;
        let c2 = Vec3 {
            x: d2,
            y: 0.0,
            z: d2,
        };
        let geom = sphere_vs_rounded_obb(
            c2,
            0.3,
            &rot,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            0.0,
        )
        .unwrap();
        assert!(approx_eq(geom.depth, 0.1));
        let inv_sqrt2 = 1.0 / std::f32::consts::SQRT_2;
        assert!(approx_eq(geom.normal.x, inv_sqrt2) && approx_eq(geom.normal.z, inv_sqrt2));
    }

    #[test]
    fn test_sphere_vs_rounded_obb_radius_adds_to_surface() {
        // box_r=0.2: surface sits 0.2 outside the core box face.
        let m = Mat4::identity_value();
        let geom = sphere_vs_rounded_obb(
            Vec3 {
                x: 1.6,
                y: 0.0,
                z: 0.0,
            },
            0.5,
            &m,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            0.2,
        )
        .unwrap();
        // Gap from core point (1,0,0) is 0.6; depth = (0.5 + 0.2) - 0.6 = 0.1.
        assert!(approx_eq(geom.depth, 0.1));
        // Contact point on the ROUNDED surface: core point + normal * box_r.
        assert!(approx_eq(geom.point.x, 1.2));
    }

    #[test]
    fn test_sphere_vs_rounded_obb_center_inside_uses_min_axis() {
        // Sphere center inside the core box: push out along the smallest
        // separation axis, depth covers radius + interior penetration
        // (mirrors test_sphere_vs_aabb_interior_*).
        let m = Mat4::identity_value();
        let geom = sphere_vs_rounded_obb(
            Vec3 {
                x: 0.9,
                y: 0.0,
                z: 0.0,
            },
            0.3,
            &m,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            0.0,
        )
        .unwrap();
        assert!(approx_eq(geom.normal.x, 1.0));
        // Interior margin to +X face = 0.1; depth = 0.1 + 0.3.
        assert!(approx_eq(geom.depth, 0.4));
    }

    #[test]
    fn test_sphere_vs_rounded_obb_nan_center_misses_without_panic() {
        let m = Mat4::identity_value();
        let result = sphere_vs_rounded_obb(
            Vec3 {
                x: f32::NAN,
                y: 0.0,
                z: 0.0,
            },
            0.3,
            &m,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            0.0,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_capsule_vs_sphere_side_hit() {
        // Vertical capsule (half_h=1, r=0.3) at origin; sphere r=0.5 at
        // (0.7, 0.5, 0): closest segment point (0, 0.5, 0), gap 0.7,
        // depth = 0.8 - 0.7 = 0.1; normal from capsule toward sphere = +X.
        let m = Mat4::identity_value();
        let geom = capsule_vs_sphere(
            &m,
            1.0,
            0.3,
            Vec3 {
                x: 0.7,
                y: 0.5,
                z: 0.0,
            },
            0.5,
        )
        .unwrap();
        assert!(approx_eq(geom.depth, 0.1));
        assert!(approx_eq(geom.normal.x, 1.0));
    }

    #[test]
    fn test_capsule_vs_sphere_cap_hit_rotated() {
        // Capsule rotated 90° about Z lies along world X; its +local-Y cap
        // ends up at world (-1, 0, 0) or (1, 0, 0) depending on rotation
        // sign — probe via the cap nearest to the sphere at (1.7, 0, 0):
        // closest segment point is (1, 0, 0) either way.
        let rot_rc = Mat4::from_euler(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: 90.0,
        });
        let rot = *rc_ref!(&rot_rc);
        let geom = capsule_vs_sphere(
            &rot,
            1.0,
            0.3,
            Vec3 {
                x: 1.7,
                y: 0.0,
                z: 0.0,
            },
            0.5,
        )
        .unwrap();
        assert!(approx_eq(geom.depth, 0.1));
        assert!(approx_eq(geom.normal.x, 1.0));
    }

    #[test]
    fn test_capsule_vs_sphere_miss() {
        let m = Mat4::identity_value();
        assert!(capsule_vs_sphere(
            &m,
            1.0,
            0.3,
            Vec3 {
                x: 3.0,
                y: 0.0,
                z: 0.0,
            },
            0.5,
        )
        .is_none());
    }

    #[test]
    fn test_capsule_vs_capsule_parallel_side_contact() {
        // Two vertical capsules (half_h=1, r=0.3) with centers 0.5 apart on
        // X: segment distance 0.5, depth = 0.6 - 0.5 = 0.1, normal ±X.
        let m_a = Mat4::identity_value();
        let m_b_rc = Mat4::from_translation(&Vec3 {
            x: 0.5,
            y: 0.0,
            z: 0.0,
        });
        let m_b = *rc_ref!(&m_b_rc);
        let geom = capsule_vs_capsule(&m_a, 1.0, 0.3, &m_b, 1.0, 0.3).unwrap();
        assert!(approx_eq(geom.depth, 0.1));
        // Normal from b toward a = -X.
        assert!(approx_eq(geom.normal.x, -1.0));
    }

    #[test]
    fn test_capsule_vs_capsule_crossed_miss() {
        // Crossed capsules with segment distance 1.0 > r_a + r_b. B's
        // local-Y segment rotates onto world Z, so the offset must be
        // perpendicular to BOTH segments (= along X) to set the distance.
        let m_a = Mat4::identity_value();
        let rot_rc = Mat4::from_euler(&Vec3 {
            x: 90.0,
            y: 0.0,
            z: 0.0,
        });
        let shift_rc = Mat4::from_translation(&Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });
        let m_b_rc = rc_ref!(&shift_rc).mul_mat(rc_ref!(&rot_rc));
        let m_b = *rc_ref!(&m_b_rc);
        assert!(capsule_vs_capsule(&m_a, 1.0, 0.3, &m_b, 1.0, 0.3).is_none());
    }

    #[test]
    fn test_segment_vs_local_aabb_parallel_face() {
        // Vertical segment beside the +X face: closest box point x = 1.
        let (on_seg, on_box) = closest_points_segment_aabb(
            Vec3 {
                x: 1.5,
                y: -0.5,
                z: 0.0,
            },
            Vec3 {
                x: 1.5,
                y: 0.5,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        );
        assert!(approx_eq(on_box.x, 1.0));
        assert!(approx_eq(on_seg.x, 1.5));
        assert!((on_seg.y - on_box.y).abs() < 1e-4);
    }

    #[test]
    fn test_segment_vs_local_aabb_matches_brute_force() {
        // Oblique segment near a corner: alternating projection must land
        // within 1e-3 of a dense parameter sweep.
        let a = Vec3 {
            x: 0.5,
            y: 1.8,
            z: 1.4,
        };
        let b = Vec3 {
            x: 2.2,
            y: 0.2,
            z: 0.6,
        };
        let half = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let (on_seg, on_box) = closest_points_segment_aabb(a, b, half);
        let dist = ((on_seg.x - on_box.x).powi(2)
            + (on_seg.y - on_box.y).powi(2)
            + (on_seg.z - on_box.z).powi(2))
        .sqrt();
        let mut best = f32::INFINITY;
        for i in 0..=2000 {
            let t = i as f32 / 2000.0;
            let p = Vec3 {
                x: a.x + (b.x - a.x) * t,
                y: a.y + (b.y - a.y) * t,
                z: a.z + (b.z - a.z) * t,
            };
            let q = Vec3 {
                x: p.x.clamp(-half.x, half.x),
                y: p.y.clamp(-half.y, half.y),
                z: p.z.clamp(-half.z, half.z),
            };
            let d = ((p.x - q.x).powi(2) + (p.y - q.y).powi(2) + (p.z - q.z).powi(2)).sqrt();
            best = best.min(d);
        }
        assert!(
            (dist - best).abs() < 1e-3,
            "alt-projection {dist} vs brute {best}"
        );
    }

    #[test]
    fn test_capsule_vs_rounded_obb_standing_on_box() {
        // Vertical capsule (half_h=0.5, r=0.3) hovering 0.05 into the top
        // of a unit-half box: bottom cap reach = center.y - 0.5 - 0.3.
        // Capsule center at y = 1.75 → reach to y=0.95, box top at 1.0 →
        // depth = 0.05; normal from box toward capsule = +Y.
        let cap_rc = Mat4::from_translation(&Vec3 {
            x: 0.0,
            y: 1.75,
            z: 0.0,
        });
        let cap = *rc_ref!(&cap_rc);
        let m_box = Mat4::identity_value();
        let geom = capsule_vs_rounded_obb(
            &cap,
            0.5,
            0.3,
            &m_box,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            0.0,
        )
        .unwrap();
        assert!(approx_eq(geom.depth, 0.05));
        assert!(approx_eq(geom.normal.y, 1.0));
    }

    #[test]
    fn test_capsule_vs_rotated_obb_no_corner_inflation() {
        // Box rotated 45° about Y (true diagonal reach 1.0, AABB reach 2.0).
        // Vertical capsule axis at 1.5 along u=(1,0,1)/√2 with r=0.3 →
        // gap 0.5 - 0.3 = 0.2: no contact. The pre-fix AABB path reported one.
        let rot_rc = Mat4::from_euler(&Vec3 {
            x: 0.0,
            y: 45.0,
            z: 0.0,
        });
        let rot = *rc_ref!(&rot_rc);
        let d = 1.5 / std::f32::consts::SQRT_2;
        let cap_rc = Mat4::from_translation(&Vec3 { x: d, y: 0.0, z: d });
        let cap = *rc_ref!(&cap_rc);
        assert!(capsule_vs_rounded_obb(
            &cap,
            0.5,
            0.3,
            &rot,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            0.0,
        )
        .is_none());
    }

    #[test]
    fn test_obb_vs_obb_axis_aligned_matches_aabb_numbers() {
        // Identity rotations reproduce test_aabb_vs_aabb_hit: unit-half
        // boxes with centers 1.8 apart on X → overlap 0.2 on X.
        let m_a = Mat4::identity_value();
        let m_b_rc = Mat4::from_translation(&Vec3 {
            x: 1.8,
            y: 0.0,
            z: 0.0,
        });
        let m_b = *rc_ref!(&m_b_rc);
        let one = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let geom = rounded_obb_vs_rounded_obb(&m_a, one, 0.0, &m_b, one, 0.0).unwrap();
        assert!(approx_eq(geom.depth, 0.2));
        // Normal from b toward a = -X.
        assert!(approx_eq(geom.normal.x, -1.0));
    }

    #[test]
    fn test_obb_vs_obb_rotated_45_diagonal_gap() {
        // B rotated 45° about Y, centers 2.2 apart along u=(1,0,1)/√2.
        // Reach along u: A contributes √2 (corner), B contributes 1.0
        // (aligned axis) → 2.41 > 2.2: overlapping on u, and no other
        // axis separates either (corner of A pokes B). Verify a contact
        // IS reported (SAT-completeness sanity check), then verify the
        // separated placement at 2.6 along u IS rejected (B's aligned
        // axis: 2.6 > 2.414).
        let m_a = Mat4::identity_value();
        let rot_rc = Mat4::from_euler(&Vec3 {
            x: 0.0,
            y: 45.0,
            z: 0.0,
        });
        let d = 2.2 / std::f32::consts::SQRT_2;
        let shift_rc = Mat4::from_translation(&Vec3 { x: d, y: 0.0, z: d });
        let m_b_rc = rc_ref!(&shift_rc).mul_mat(rc_ref!(&rot_rc));
        let m_b = *rc_ref!(&m_b_rc);
        let one = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert!(rounded_obb_vs_rounded_obb(&m_a, one, 0.0, &m_b, one, 0.0).is_some());
        // And at 2.6 along u the same pair must be separated.
        let d2 = 2.6 / std::f32::consts::SQRT_2;
        let shift2_rc = Mat4::from_translation(&Vec3 {
            x: d2,
            y: 0.0,
            z: d2,
        });
        let m_b2_rc = rc_ref!(&shift2_rc).mul_mat(rc_ref!(&rot_rc));
        let m_b2 = *rc_ref!(&m_b2_rc);
        assert!(rounded_obb_vs_rounded_obb(&m_a, one, 0.0, &m_b2, one, 0.0).is_none());
    }

    #[test]
    fn test_obb_vs_obb_rounding_radii_add() {
        // Sharp gap 0.2 on X; radii 0.15 + 0.15 = 0.3 close it → depth 0.1.
        let m_a = Mat4::identity_value();
        let m_b_rc = Mat4::from_translation(&Vec3 {
            x: 2.2,
            y: 0.0,
            z: 0.0,
        });
        let m_b = *rc_ref!(&m_b_rc);
        let one = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let geom = rounded_obb_vs_rounded_obb(&m_a, one, 0.15, &m_b, one, 0.15).unwrap();
        assert!(approx_eq(geom.depth, 0.1));
    }

    #[test]
    fn test_local_box_vs_triangle_matches_existing_aabb_path() {
        // Reproduce test_aabb_vs_triangle_hit_through_face numbers through
        // the new core (r = 0, verts pre-shifted into box-local frame).
        let v0 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v1 = Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        };
        let aabb = Aabb {
            min: Vec3 {
                x: 0.5,
                y: 0.5,
                z: -0.5,
            },
            max: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.5,
            },
        };
        let old = aabb_vs_triangle(&aabb, v0, v1, v2).unwrap();
        let center = Vec3 {
            x: 0.75,
            y: 0.75,
            z: 0.0,
        };
        let extents = Vec3 {
            x: 0.25,
            y: 0.25,
            z: 0.5,
        };
        let shift = |v: Vec3| Vec3 {
            x: v.x - center.x,
            y: v.y - center.y,
            z: v.z - center.z,
        };
        let new = local_box_vs_triangle(extents, 0.0, shift(v0), shift(v1), shift(v2)).unwrap();
        assert!(approx_eq(new.depth, old.depth));
        assert!(approx_eq(new.normal.x, old.normal.x));
        assert!(approx_eq(new.normal.y, old.normal.y));
        assert!(approx_eq(new.normal.z, old.normal.z));
    }

    #[test]
    fn test_local_box_vs_triangle_radius_extends_reach() {
        // Triangle plane at x = 1.2 beside a unit-half box: sharp box (r=0)
        // misses; r=0.3 reaches → depth = (1 + 0.3) - 1.2 = 0.1.
        let v0 = Vec3 {
            x: 1.2,
            y: -2.0,
            z: -2.0,
        };
        let v1 = Vec3 {
            x: 1.2,
            y: 2.0,
            z: -2.0,
        };
        let v2 = Vec3 {
            x: 1.2,
            y: 0.0,
            z: 2.0,
        };
        let half = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert!(local_box_vs_triangle(half, 0.0, v0, v1, v2).is_none());
        let geom = local_box_vs_triangle(half, 0.3, v0, v1, v2).unwrap();
        assert!(approx_eq(geom.depth, 0.1));
    }

    #[test]
    fn test_capsule_vs_triangle_above_face() {
        // Horizontal triangle at y=0; vertical capsule bottom cap reaching
        // y = 0.75 - 0.5 - 0.3 = -0.05 → depth 0.05, normal +Y (triangle
        // normal side hosting the capsule).
        let v0 = Vec3 {
            x: -2.0,
            y: 0.0,
            z: -2.0,
        };
        let v1 = Vec3 {
            x: 2.0,
            y: 0.0,
            z: -2.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 2.0,
        };
        let cap_rc = Mat4::from_translation(&Vec3 {
            x: 0.0,
            y: 0.75,
            z: 0.0,
        });
        let cap = *rc_ref!(&cap_rc);
        let geom = capsule_vs_triangle(&cap, 0.5, 0.3, v0, v1, v2).unwrap();
        assert!(approx_eq(geom.depth, 0.05));
        assert!(approx_eq(geom.normal.y, 1.0));
    }

    #[test]
    fn test_capsule_vs_triangle_edge_contact_lying_capsule() {
        // Capsule rotated 90° about Z lies along X above the triangle edge
        // from (-2,0,-2) to (2,0,-2): minimum distance is segment-to-edge.
        // Place the axis at y=0.25, z=-2 → distance 0.25 < r=0.3 → depth 0.05.
        let rot_rc = Mat4::from_euler(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: 90.0,
        });
        let shift_rc = Mat4::from_translation(&Vec3 {
            x: 0.0,
            y: 0.25,
            z: -2.0,
        });
        let cap_rc = rc_ref!(&shift_rc).mul_mat(rc_ref!(&rot_rc));
        let cap = *rc_ref!(&cap_rc);
        let v0 = Vec3 {
            x: -2.0,
            y: 0.0,
            z: -2.0,
        };
        let v1 = Vec3 {
            x: 2.0,
            y: 0.0,
            z: -2.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 2.0,
        };
        let geom = capsule_vs_triangle(&cap, 0.5, 0.3, v0, v1, v2).unwrap();
        assert!(approx_eq(geom.depth, 0.05));
        assert!(approx_eq(geom.normal.y, 1.0));
    }

    #[test]
    fn test_capsule_vs_triangle_miss() {
        let v0 = Vec3 {
            x: -2.0,
            y: 0.0,
            z: -2.0,
        };
        let v1 = Vec3 {
            x: 2.0,
            y: 0.0,
            z: -2.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 2.0,
        };
        let cap_rc = Mat4::from_translation(&Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        });
        let cap = *rc_ref!(&cap_rc);
        assert!(capsule_vs_triangle(&cap, 0.5, 0.3, v0, v1, v2).is_none());
    }
}
