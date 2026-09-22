// Collision formulas retain conventional notation, including Möller-Trumbore's h/a/f/s/q/u/v/t.
#![allow(clippy::many_single_char_names)]

use crate::cube::collider::Collider;
use crate::cube::mat4::Mat4;
use crate::cube::mesh::Mesh;
use crate::cube::vec3::Vec3;

const CONTACT_EPSILON: f32 = 1e-6;

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
        Self::from_transformed_corners(transform, &corners)
    }

    // Mesh-collider AABB: the mesh-local AABB (cached on the Mesh)
    // lifted by transforming its 8 corners. For rotated meshes this is
    // the bounding box of the rotated local box — conservative, so the
    // broad phase stays correct without re-transforming every vertex
    // per frame.
    pub fn from_mesh(mesh: &Mesh, transform: &Mat4) -> Self {
        let local = mesh.local_aabb();
        let corners = [
            Vec3 {
                x: local.min.x,
                y: local.min.y,
                z: local.min.z,
            },
            Vec3 {
                x: local.max.x,
                y: local.min.y,
                z: local.min.z,
            },
            Vec3 {
                x: local.min.x,
                y: local.max.y,
                z: local.min.z,
            },
            Vec3 {
                x: local.max.x,
                y: local.max.y,
                z: local.min.z,
            },
            Vec3 {
                x: local.min.x,
                y: local.min.y,
                z: local.max.z,
            },
            Vec3 {
                x: local.max.x,
                y: local.min.y,
                z: local.max.z,
            },
            Vec3 {
                x: local.min.x,
                y: local.max.y,
                z: local.max.z,
            },
            Vec3 {
                x: local.max.x,
                y: local.max.y,
                z: local.max.z,
            },
        ];
        Self::from_transformed_corners(transform, &corners)
    }

    fn from_transformed_corners(transform: &Mat4, corners: &[Vec3; 8]) -> Self {
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

        for c in corners {
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

    pub fn overlaps(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
}

// World-space contact record produced by the narrow phase. The normal
// points from `b` toward `a`, matching the user-facing convention.
#[derive(Clone, Copy, Debug)]
pub struct ContactGeom {
    pub point: Vec3,
    pub normal: Vec3,
    pub depth: f32,
}

// Components with abs(value) < 1e-9 count as zero. A zero core produces
// a sphere; (0, h, 0) produces a local-Y capsule; anything else produces
// a rounded box. The radius rounds every family.
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
// rounded-box path with zero size.
pub fn collider_aabb(collider: &Collider, transform: &Mat4) -> Aabb {
    if let Some(mesh_rc) = &collider.mesh {
        let mesh = rc_ref!(mesh_rc);
        return Aabb::from_mesh(&mesh, transform);
    }
    let size = *rc_ref!(&collider.size);
    Aabb::from_rounded_box(transform, size, collider.radius)
}

// Pair narrow-phase tests

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
        if depth <= CONTACT_EPSILON {
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
    // combined radius.
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
    // Reverse the sphere/capsule argument order to get the capsule-to-sphere normal.
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
// points from the box toward the capsule. A core-crossing segment needs
// enough separation to clear its full extent, not only the closest point.
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

    let (on_seg, on_box) = closest_points_segment_aabb(top, bot, half);
    let on_seg_world = box_world.mul_vec_value(&on_seg);
    if on_seg == on_box {
        let dx = top.x - bot.x;
        let dy = top.y - bot.y;
        let dz = top.z - bot.z;
        // Faces of the box swept by the segment have box-face normals or
        // segment/box-edge cross normals. The cores already intersect, so
        // the shortest full projection clearance plus radii is a pushout.
        let axes = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, dz, -dy],
            [-dz, 0.0, dx],
            [dy, -dx, 0.0],
        ];
        let reach = cap_r.max(0.0) + box_r.max(0.0);
        let mut depth = f32::INFINITY;
        let mut normal = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        for [nx, ny, nz] in axes {
            let len = (nx * nx + ny * ny + nz * nz).sqrt();
            if len <= f32::EPSILON {
                continue;
            }
            let (nx, ny, nz) = (nx / len, ny / len, nz / len);
            let extent = half.x * nx.abs() + half.y * ny.abs() + half.z * nz.abs();
            let top_distance = top.x * nx + top.y * ny + top.z * nz;
            let bot_distance = bot.x * nx + bot.y * ny + bot.z * nz;
            let forward = extent - top_distance.min(bot_distance) + reach;
            let backward = extent + top_distance.max(bot_distance) + reach;
            let (candidate, sign) = if backward < forward {
                (backward, -1.0)
            } else {
                (forward, 1.0)
            };
            if candidate < depth {
                depth = candidate;
                normal = Vec3 {
                    x: nx * sign,
                    y: ny * sign,
                    z: nz * sign,
                };
            }
        }

        if !depth.is_finite() || depth <= CONTACT_EPSILON {
            return None;
        }
        return Some(ContactGeom {
            point: on_seg_world,
            normal: box_world.mul_dir_value(&normal),
            depth,
        });
    }

    // sphere_vs_rounded_obb's normal points box → "sphere" = box → capsule.
    sphere_vs_rounded_obb(on_seg_world, cap_r.max(0.0), box_world, half, box_r)
}

// Exact core distance from edge/box closest points in both directions.
// These cover vertex/face and edge/edge minima without an iterative solver.
// Cache the rigid relative frames for repeated translations during a sweep.
pub(crate) struct ObbPairDistance {
    world_a: Mat4,
    world_b: Mat4,
    inv_a: Mat4,
    inv_b: Mat4,
    a_in_b: [Vec3; 8],
    b_in_a: [Vec3; 8],
    half_a: Vec3,
    half_b: Vec3,
}

impl ObbPairDistance {
    pub(crate) fn new(world_a: &Mat4, half_a: Vec3, world_b: &Mat4, half_b: Vec3) -> Self {
        let inv_a = world_a.inverse_value();
        let inv_b = world_b.inverse_value();
        let vertices = |world: &Mat4, inverse: &Mat4, half: Vec3| {
            std::array::from_fn(|i| {
                let point = Vec3 {
                    x: if i & 1 == 0 { -half.x } else { half.x },
                    y: if i & 2 == 0 { -half.y } else { half.y },
                    z: if i & 4 == 0 { -half.z } else { half.z },
                };
                inverse.mul_vec_value(&world.mul_vec_value(&point))
            })
        };
        Self {
            world_a: *world_a,
            world_b: *world_b,
            inv_a,
            inv_b,
            a_in_b: vertices(world_a, &inv_b, half_a),
            b_in_a: vertices(world_b, &inv_a, half_b),
            half_a,
            half_b,
        }
    }

    // Offset translates A in world space; B stays fixed.
    pub(crate) fn closest_points(&self, offset: Vec3) -> (Vec3, Vec3) {
        let offset_b = self.inv_b.mul_dir_value(&offset);
        let offset_a = self.inv_a.mul_dir_value(&offset);
        let mut best = (self.world_a.pos_value(), self.world_b.pos_value());
        let mut best_dist2 = f32::INFINITY;
        for (vertices, half, shift, world, reverse) in [
            (&self.a_in_b, self.half_b, offset_b, &self.world_b, false),
            (
                &self.b_in_a,
                self.half_a,
                Vec3 {
                    x: -offset_a.x,
                    y: -offset_a.y,
                    z: -offset_a.z,
                },
                &self.world_a,
                true,
            ),
        ] {
            for bit in [1, 2, 4] {
                for i in 0..8 {
                    if i & bit != 0 {
                        continue;
                    }
                    let shifted = |p: Vec3| Vec3 {
                        x: p.x + shift.x,
                        y: p.y + shift.y,
                        z: p.z + shift.z,
                    };
                    let (on_edge, on_box) = closest_points_segment_aabb(
                        shifted(vertices[i]),
                        shifted(vertices[i | bit]),
                        half,
                    );
                    let dx = on_edge.x - on_box.x;
                    let dy = on_edge.y - on_box.y;
                    let dz = on_edge.z - on_box.z;
                    let dist2 = dx * dx + dy * dy + dz * dz;
                    if dist2 < best_dist2 {
                        best_dist2 = dist2;
                        let mut edge = world.mul_vec_value(&on_edge);
                        let mut point = world.mul_vec_value(&on_box);
                        if reverse {
                            edge = Vec3 {
                                x: edge.x + offset.x,
                                y: edge.y + offset.y,
                                z: edge.z + offset.z,
                            };
                            point = Vec3 {
                                x: point.x + offset.x,
                                y: point.y + offset.y,
                                z: point.z + offset.z,
                            };
                            best = (point, edge);
                        } else {
                            best = (edge, point);
                        }
                        if dist2 == 0.0 {
                            return best;
                        }
                    }
                }
            }
        }
        best
    }
}

// SAT rejects separated projections and handles overlapping sharp cores.
// Separated rounded cores need their Euclidean distance: expanding only
// the 15 SAT projections would fill in the rounded edges and corners.
// Normal points from b toward a. For intersecting cores, the contact
// point retains b's center as a practical tangential placement proxy.
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
    let ca = world_a.pos_value();
    let cb = world_b.pos_value();
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
    if min_overlap < r_sum {
        let pair = ObbPairDistance::new(world_a, half_a, world_b, half_b);
        let (on_a, on_b) = pair.closest_points(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let delta = Vec3 {
            x: on_a.x - on_b.x,
            y: on_a.y - on_b.y,
            z: on_a.z - on_b.z,
        };
        let distance = dot(&delta, &delta).sqrt();
        if distance >= r_sum {
            return None;
        }
        if distance > 0.0 {
            let normal = Vec3 {
                x: delta.x / distance,
                y: delta.y / distance,
                z: delta.z / distance,
            };
            let depth = r_sum - distance;
            let reach = r_b.max(0.0) - depth * 0.5;
            return Some(ContactGeom {
                point: Vec3 {
                    x: on_b.x + normal.x * reach,
                    y: on_b.y + normal.y * reach,
                    z: on_b.z + normal.z * reach,
                },
                normal,
                depth,
            });
        }
    }
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

// Ray intersections

// Ray-sphere intersection. Returns the (t, point, normal) of the
// closest hit in [0, max_distance], or None.
pub fn ray_vs_sphere(
    origin: Vec3,
    dir: Vec3,
    center: Vec3,
    radius: f32,
    max_distance: f32,
) -> Option<(f32, Vec3, Vec3)> {
    ray_vs_sphere_filtered(origin, dir, center, radius, max_distance, |_| true)
}

// A composed surface may hide the near root while exposing the far root.
pub(crate) fn ray_vs_sphere_filtered(
    origin: Vec3,
    dir: Vec3,
    center: Vec3,
    radius: f32,
    max_distance: f32,
    accepts: impl Fn(Vec3) -> bool,
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
    for t in [t1, t2].into_iter().filter(|t| *t >= 0.0) {
        if t > max_distance {
            return None;
        }

        let point = Vec3 {
            x: origin.x + dir.x * t,
            y: origin.y + dir.y * t,
            z: origin.z + dir.z * t,
        };

        if !accepts(point) {
            continue;
        }

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
        return Some((t, point, normal));
    }
    None
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
        // With nonzero d and bmin < bmax, the entry face's outward normal
        // has the axis sign opposite the ray direction.
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
    if tmin > max_distance {
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

// Shape-vs-triangle tests

// Sphere-vs-triangle: find the closest point on the triangle to the
// sphere center, then accept the hit when that point is inside the
// sphere. Returns ContactGeom with normal pointing from the triangle
// toward the sphere center.
pub fn sphere_vs_triangle(c: Vec3, r: f32, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<ContactGeom> {
    let (closest, face_normal) = closest_triangle_point_and_normal(c, v0, v1, v2);
    let dx = c.x - closest.x;
    let dy = c.y - closest.y;
    let dz = c.z - closest.z;
    let dist_sq = dx * dx + dy * dy + dz * dz;
    if dist_sq >= r * r {
        return None;
    }

    let dist = dist_sq.sqrt();
    let normal = if let Some(n) = face_normal {
        // A face contact keeps the plane's normal. Subtracting two large
        // nearby positions would give rolling bodies a changing normal.
        let length = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
        let sign = if dx * n.x + dy * n.y + dz * n.z < 0.0 {
            -1.0
        } else {
            1.0
        };
        Vec3 {
            x: sign * n.x / length,
            y: sign * n.y / length,
            z: sign * n.z / length,
        }
    } else if dist > 1e-9 {
        Vec3 {
            x: dx / dist,
            y: dy / dist,
            z: dz / dist,
        }
    } else {
        // A center on the triangle uses its winding normal.
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

// Capsule vs triangle. A crossing axis needs enough face-normal pushout
// to clear both ends; otherwise the closest pair gives a sphere contact.
pub fn capsule_vs_triangle(
    cap_world: &Mat4,
    half_h: f32,
    cap_r: f32,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> Option<ContactGeom> {
    let cap_r = cap_r.max(0.0);
    if cap_r == 0.0 {
        return None;
    }

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

    let (on_seg, on_tri) = closest_points_segment_triangle(top, bot, v0, v1, v2);
    if on_seg == on_tri {
        let (ax, ay, az) = (v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let (bx, by, bz) = (v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
        let (nx, ny, nz) = (ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx);
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        if len > 1e-9 {
            let mut normal = Vec3 {
                x: nx / len,
                y: ny / len,
                z: nz / len,
            };

            let distance = |p: Vec3| {
                (p.x - v0.x) * normal.x + (p.y - v0.y) * normal.y + (p.z - v0.z) * normal.z
            };
            let top_distance = distance(top);
            let bot_distance = distance(bot);
            let forward = cap_r - top_distance.min(bot_distance);
            let backward = cap_r + top_distance.max(bot_distance);
            let depth = if backward < forward {
                normal.x = -normal.x;
                normal.y = -normal.y;
                normal.z = -normal.z;
                backward
            } else {
                forward
            };
            return Some(ContactGeom {
                point: on_tri,
                normal,
                depth,
            });
        }
    }

    sphere_vs_triangle(on_seg, cap_r, v0, v1, v2)
}

// Closest-point helpers

pub(crate) fn closest_points_segment_triangle(
    a0: Vec3,
    a1: Vec3,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> (Vec3, Vec3) {
    let direction = Vec3 {
        x: a1.x - a0.x,
        y: a1.y - a0.y,
        z: a1.z - a0.z,
    };
    if let Some((_, point, _)) = ray_vs_triangle(a0, direction, v0, v1, v2, 1.0) {
        return (point, point);
    }

    let mut best = (a0, closest_point_on_triangle(a0, v0, v1, v2));
    let mut best_dist_sq = point_dist2(best.0, best.1);
    let on_tri = closest_point_on_triangle(a1, v0, v1, v2);
    let dist_sq = point_dist2(a1, on_tri);
    if dist_sq < best_dist_sq {
        best = (a1, on_tri);
        best_dist_sq = dist_sq;
    }

    for (edge0, edge1) in [(v0, v1), (v1, v2), (v2, v0)] {
        let candidate = closest_points_segment_segment(a0, a1, edge0, edge1);
        let dist_sq = point_dist2(candidate.0, candidate.1);
        if dist_sq < best_dist_sq {
            best = candidate;
            best_dist_sq = dist_sq;
        }
    }

    best
}

// Closest point on triangle to p (Ericson, Real-Time Collision
// Detection §5.1.5). Returns the barycentric point on the triangle
// or its nearest edge / vertex.
pub(crate) fn closest_point_on_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    closest_triangle_point_and_normal(p, a, b, c).0
}

fn closest_triangle_point_and_normal(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> (Vec3, Option<Vec3>) {
    // Triangle edges can be much longer than the contacting body. Calculate
    // the region and its closest point without cancelling f32 world positions.
    let difference = |a: Vec3, b: Vec3| {
        [
            f64::from(a.x) - f64::from(b.x),
            f64::from(a.y) - f64::from(b.y),
            f64::from(a.z) - f64::from(b.z),
        ]
    };
    let ab = difference(b, a);
    let ac = difference(c, a);
    let ap = difference(p, a);
    let d1 = ab[0] * ap[0] + ab[1] * ap[1] + ab[2] * ap[2];
    let d2 = ac[0] * ap[0] + ac[1] * ap[1] + ac[2] * ap[2];
    if d1 <= 0.0 && d2 <= 0.0 {
        return (a, None);
    }

    let bp = difference(p, b);
    let d3 = ab[0] * bp[0] + ab[1] * bp[1] + ab[2] * bp[2];
    let d4 = ac[0] * bp[0] + ac[1] * bp[1] + ac[2] * bp[2];
    if d3 >= 0.0 && d4 <= d3 {
        return (b, None);
    }

    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return (
            Vec3 {
                x: (f64::from(a.x) + ab[0] * v) as f32,
                y: (f64::from(a.y) + ab[1] * v) as f32,
                z: (f64::from(a.z) + ab[2] * v) as f32,
            },
            None,
        );
    }

    let cp = difference(p, c);
    let d5 = ab[0] * cp[0] + ab[1] * cp[1] + ab[2] * cp[2];
    let d6 = ac[0] * cp[0] + ac[1] * cp[1] + ac[2] * cp[2];
    if d6 >= 0.0 && d5 <= d6 {
        return (c, None);
    }

    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return (
            Vec3 {
                x: (f64::from(a.x) + ac[0] * w) as f32,
                y: (f64::from(a.y) + ac[1] * w) as f32,
                z: (f64::from(a.z) + ac[2] * w) as f32,
            },
            None,
        );
    }

    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return (
            Vec3 {
                x: (f64::from(b.x) + (f64::from(c.x) - f64::from(b.x)) * w) as f32,
                y: (f64::from(b.y) + (f64::from(c.y) - f64::from(b.y)) * w) as f32,
                z: (f64::from(b.z) + (f64::from(c.z) - f64::from(b.z)) * w) as f32,
            },
            None,
        );
    }

    // Inside the face, project along its normal. Reconstructing the point
    // from barycentric coordinates loses tangential precision on large
    // triangles and makes a flat floor exert a sideways contact force.
    let nx = ab[1] * ac[2] - ab[2] * ac[1];
    let ny = ab[2] * ac[0] - ab[0] * ac[2];
    let nz = ab[0] * ac[1] - ab[1] * ac[0];
    let distance = (ap[0] * nx + ap[1] * ny + ap[2] * nz) / (nx * nx + ny * ny + nz * nz);
    (
        Vec3 {
            x: (f64::from(p.x) - nx * distance) as f32,
            y: (f64::from(p.y) - ny * distance) as f32,
            z: (f64::from(p.z) - nz * distance) as f32,
        },
        Some(Vec3 {
            x: nx as f32,
            y: ny as f32,
            z: nz as f32,
        }),
    )
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
    // A short capsule axis can meet a very long triangle edge. Keep the
    // intermediate parameters precise before storing world-space points.
    let coordinates = |v: Vec3| [f64::from(v.x), f64::from(v.y), f64::from(v.z)];
    let p1d = coordinates(p1);
    let p2d = coordinates(p2);
    let q1d = coordinates(q1);
    let q2d = coordinates(q2);
    let d1 = std::array::from_fn(|i| q1d[i] - p1d[i]);
    let d2 = std::array::from_fn(|i| q2d[i] - p2d[i]);
    let r = std::array::from_fn(|i| p1d[i] - p2d[i]);
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let a = dot(d1, d1);
    let e = dot(d2, d2);
    let f = dot(d2, r);
    let (s, t);
    if a <= f64::from(f32::EPSILON) && e <= f64::from(f32::EPSILON) {
        return (p1, p2);
    }

    if a <= f64::from(f32::EPSILON) {
        s = 0.0;
        t = (f / e).clamp(0.0, 1.0);
    } else {
        let c = dot(d1, r);
        if e <= f64::from(f32::EPSILON) {
            t = 0.0;
            s = (-c / a).clamp(0.0, 1.0);
        } else {
            let b = dot(d1, d2);
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
            x: (p1d[0] + d1[0] * s) as f32,
            y: (p1d[1] + d1[1] * s) as f32,
            z: (p1d[2] + d1[2] * s) as f32,
        },
        Vec3 {
            x: (p2d[0] + d2[0] * t) as f32,
            y: (p2d[1] + d2[1] * t) as f32,
            z: (p2d[2] + d2[2] * t) as f32,
        },
    )
}

fn clip_segment_axis(
    start: f32,
    delta: f32,
    min: f32,
    max: f32,
    enter: &mut f32,
    exit: &mut f32,
) -> bool {
    if delta.abs() <= f32::EPSILON {
        return start >= min && start <= max;
    }

    let inv_delta = 1.0 / delta;
    let mut t0 = (min - start) * inv_delta;
    let mut t1 = (max - start) * inv_delta;
    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }
    *enter = (*enter).max(t0);
    *exit = (*exit).min(t1);
    *enter <= *exit
}

fn segment_point(a: Vec3, d: Vec3, t: f32) -> Vec3 {
    Vec3 {
        x: a.x + d.x * t,
        y: a.y + d.y * t,
        z: a.z + d.z * t,
    }
}

fn clamp_point_to_aabb(p: Vec3, half: Vec3) -> Vec3 {
    Vec3 {
        x: p.x.clamp(-half.x, half.x),
        y: p.y.clamp(-half.y, half.y),
        z: p.z.clamp(-half.z, half.z),
    }
}

fn point_dist2(a: Vec3, b: Vec3) -> f32 {
    (a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)
}

fn push_segment_param(params: &mut [f32; 8], len: &mut usize, t: f32) {
    if t > 0.0 && t < 1.0 && t.is_finite() {
        params[*len] = t;
        *len += 1;
    }
}

fn sort_unique_segment_params(params: &mut [f32; 8], len: usize) -> usize {
    for i in 1..len {
        let value = params[i];
        let mut j = i;
        while j > 0 && params[j - 1] > value {
            params[j] = params[j - 1];
            j -= 1;
        }
        params[j] = value;
    }

    let mut unique_len = 0;
    for i in 0..len {
        if unique_len == 0 || (params[i] - params[unique_len - 1]).abs() > 1e-6 {
            params[unique_len] = params[i];
            unique_len += 1;
        }
    }
    unique_len
}

fn consider_segment_aabb_candidate(
    a: Vec3,
    d: Vec3,
    half: Vec3,
    t: f32,
    best_t: &mut f32,
    best_dist2: &mut f32,
) {
    let on_seg = segment_point(a, d, t);
    let on_box = clamp_point_to_aabb(on_seg, half);
    let dist2 = point_dist2(on_seg, on_box);
    if dist2 < *best_dist2 - 1e-7 || ((dist2 - *best_dist2).abs() <= 1e-7 && t < *best_t) {
        *best_dist2 = dist2;
        *best_t = t;
    }
}

fn add_segment_aabb_axis_quadratic(
    start: f32,
    delta: f32,
    mid: f32,
    half: f32,
    denom: &mut f32,
    numer: &mut f32,
) {
    let bound = if mid < -half {
        -half
    } else if mid > half {
        half
    } else {
        return;
    };
    *denom += delta * delta;
    *numer += delta * (start - bound);
}

// Exact closest points between a segment and an origin-centered AABB in
// the box-local frame. Ericson's point/AABB clamp gives the distance at
// a fixed segment parameter; slab crossings split the parameter range
// into pieces where that distance is a simple quadratic.
pub(crate) fn closest_points_segment_aabb(a: Vec3, b: Vec3, half: Vec3) -> (Vec3, Vec3) {
    let half = Vec3 {
        x: half.x.max(0.0),
        y: half.y.max(0.0),
        z: half.z.max(0.0),
    };
    let d = Vec3 {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    if d.x * d.x + d.y * d.y + d.z * d.z <= f32::EPSILON {
        return (a, clamp_point_to_aabb(a, half));
    }

    let mut enter: f32 = 0.0;
    let mut exit: f32 = 1.0;
    if clip_segment_axis(a.x, d.x, -half.x, half.x, &mut enter, &mut exit)
        && clip_segment_axis(a.y, d.y, -half.y, half.y, &mut enter, &mut exit)
        && clip_segment_axis(a.z, d.z, -half.z, half.z, &mut enter, &mut exit)
    {
        let on_seg = segment_point(a, d, (enter + exit) * 0.5);
        return (on_seg, on_seg);
    }

    // Two endpoints plus one crossing for each of the six slab faces.
    let mut params = [0.0; 8];
    let mut param_len = 0;
    params[param_len] = 0.0;
    param_len += 1;
    params[param_len] = 1.0;
    param_len += 1;

    if d.x.abs() > f32::EPSILON {
        push_segment_param(&mut params, &mut param_len, (-half.x - a.x) / d.x);
        push_segment_param(&mut params, &mut param_len, (half.x - a.x) / d.x);
    }
    if d.y.abs() > f32::EPSILON {
        push_segment_param(&mut params, &mut param_len, (-half.y - a.y) / d.y);
        push_segment_param(&mut params, &mut param_len, (half.y - a.y) / d.y);
    }
    if d.z.abs() > f32::EPSILON {
        push_segment_param(&mut params, &mut param_len, (-half.z - a.z) / d.z);
        push_segment_param(&mut params, &mut param_len, (half.z - a.z) / d.z);
    }
    let param_len = sort_unique_segment_params(&mut params, param_len);

    let mut best_t = 0.0;
    let mut best_dist2 = f32::INFINITY;
    for &t in params.iter().take(param_len) {
        consider_segment_aabb_candidate(a, d, half, t, &mut best_t, &mut best_dist2);
    }

    for i in 0..param_len - 1 {
        let lo = params[i];
        let hi = params[i + 1];
        if hi - lo <= 1e-6 {
            continue;
        }

        let mid_t = (lo + hi) * 0.5;
        let mid = segment_point(a, d, mid_t);
        let mut denom = 0.0;
        let mut numer = 0.0;
        add_segment_aabb_axis_quadratic(a.x, d.x, mid.x, half.x, &mut denom, &mut numer);
        add_segment_aabb_axis_quadratic(a.y, d.y, mid.y, half.y, &mut denom, &mut numer);
        add_segment_aabb_axis_quadratic(a.z, d.z, mid.z, half.z, &mut denom, &mut numer);
        if denom > f32::EPSILON {
            let t = (-numer / denom).clamp(lo, hi);
            consider_segment_aabb_candidate(a, d, half, t, &mut best_t, &mut best_dist2);
        }
    }

    let on_seg = segment_point(a, d, best_t);
    let on_box = clamp_point_to_aabb(on_seg, half);
    (on_seg, on_box)
}

// Box-triangle SAT core

// Box-local triangle test via the Separating-Axis Theorem with 13 axes:
//   - 3 box face normals (X, Y, Z)
//   - 1 triangle face normal
//   - 9 cross products of box edges with triangle edges
// The box is origin-centered with half-extents `half` and swept outward
// by the rounding radius `r` (applied as a uniform extension on every
// SAT axis: exact on faces and edges, over-reporting by at most
// r·(√3−1) in corner regions). Triangle vertices are given in the same
// box-local frame. Choose the shortest separating translation, including
// triangle edges: a box resting on a ledge must go up, not sideways through
// the opposite ledge merely because it also touches a vertical triangle.
pub fn local_box_vs_triangle(
    half: Vec3,
    r: f32,
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
) -> Option<ContactGeom> {
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
    let cross = |a: Vec3, b: Vec3| Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    };
    let length_squared = |a: Vec3| a.x * a.x + a.y * a.y + a.z * a.z;
    let face_normal = cross(edges[0], edges[1]);
    if length_squared(face_normal) < 1e-18 {
        return None;
    }
    // Test the face first so exact ties retain the surface normal.
    let mut best = box_triangle_axis(face_normal, p0, p1, p2, half, r.max(0.0))?;
    for axis in [
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
    ] {
        for candidate in std::iter::once(axis).chain(edges.map(|edge| cross(axis, edge))) {
            if length_squared(candidate) < 1e-18 {
                continue;
            }
            let hit = box_triangle_axis(candidate, p0, p1, p2, half, r.max(0.0))?;
            if hit.0 < best.0 {
                best = hit;
            }
        }
    }
    Some(ContactGeom {
        point: closest_point_on_triangle(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            p0,
            p1,
            p2,
        ),
        normal: best.1,
        depth: best.0,
    })
}

// Return the distance and direction needed to separate the projections.
// Unlike interval intersection length, this also handles a thin triangle
// whose face-normal projection is a point inside the box's projection.
fn box_triangle_axis(
    axis: Vec3,
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    half: Vec3,
    radius: f32,
) -> Option<(f32, Vec3)> {
    let dot = |p: Vec3| p.x * axis.x + p.y * axis.y + p.z * axis.z;
    let (a, b, c) = (dot(p0), dot(p1), dot(p2));
    let length = dot(axis).sqrt();
    let extent =
        half.x * axis.x.abs() + half.y * axis.y.abs() + half.z * axis.z.abs() + radius * length;
    let positive = a.max(b).max(c) + extent;
    let negative = extent - a.min(b).min(c);
    if positive < 0.0 || negative < 0.0 {
        return None;
    }
    let (depth, sign) = if positive <= negative {
        (positive, 1.0)
    } else {
        (negative, -1.0)
    };
    Some((
        depth / length,
        Vec3 {
            x: axis.x * sign / length,
            y: axis.y * sign / length,
            z: axis.z * sign / length,
        },
    ))
}

fn vec_is_finite(v: Vec3) -> bool {
    v.x.is_finite() && v.y.is_finite() && v.z.is_finite()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1e-4 exceeds the f32 rounding of these unit-scale values and stays below any expected difference
    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    fn assert_vec3_close(actual: Vec3, expected: Vec3) {
        assert!(
            approx_eq(actual.x, expected.x)
                && approx_eq(actual.y, expected.y)
                && approx_eq(actual.z, expected.z),
            "actual=({:.6}, {:.6}, {:.6}) expected=({:.6}, {:.6}, {:.6})",
            actual.x,
            actual.y,
            actual.z,
            expected.x,
            expected.y,
            expected.z
        );
    }

    fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn translation(v: Vec3) -> Mat4 {
        *rc_ref!(&Mat4::from_translation(&v))
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
        assert_eq!(aabb.min, vec3(0.5, 1.5, 2.5));
        assert_eq!(aabb.max, vec3(1.5, 2.5, 3.5));
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
        assert_vec3_close(r.normal, vec3(-1.0, 0.0, 0.0));
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
    fn test_ray_vs_sphere_hit() {
        let (t, point, normal) = ray_vs_sphere(
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
        assert_vec3_close(point, vec3(0.0, 0.0, -1.0));
        assert_eq!(normal, vec3(0.0, 0.0, -1.0));
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

        let (t, point, normal) = ray_vs_aabb(
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
        assert_vec3_close(point, vec3(0.0, 0.0, -1.0));
        assert_vec3_close(normal, vec3(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_ray_vs_triangle_hit() {
        let (t, point, normal) = ray_vs_triangle(
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
        assert_vec3_close(point, vec3(0.0, 0.0, 0.0));
        assert_vec3_close(normal, vec3(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_sphere_vs_triangle_hit_above_face() {
        // Triangle in the z=0 plane; sphere center hovers above the
        // face at z=+0.3 with radius 0.5 → 0.2 penetration depth.
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
        assert_vec3_close(r.normal, vec3(0.0, 0.0, 1.0));
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
        assert_eq!((r.normal.x, r.normal.y, r.normal.z), (0.0, 1.0, 0.0));
        assert_eq!(r.depth, 2.0);
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
        // Ray hits at t=4 with cap=4.5, misses with cap=3.5.
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
    fn test_ray_vs_aabb_negative_x_and_positive_y_normals() {
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
        assert_eq!((n.x, n.y, n.z), (-1.0, 0.0, 0.0));

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
        assert_eq!((n.x, n.y, n.z), (0.0, 1.0, 0.0));
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
        // Sphere center sits over the AB edge at (0.5, -0.2, 0). The
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
        assert_vec3_close(r.point, vec3(0.5, 0.0, 0.0));
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
        assert_eq!(r.point, vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_flat_triangle_contacts_do_not_add_sideways_force() {
        // A wide terrain triangle used to lose several ulps when rebuilding
        // the closest point. That noise tipped rolling stacks at rest.
        for axis in 0..3 {
            let rotate = |v: Vec3| match axis {
                0 => v,
                1 => vec3(v.y, v.z, v.x),
                _ => vec3(v.z, v.x, v.y),
            };
            let a = rotate(vec3(-96.0, 52.0, -112.0));
            let b = rotate(vec3(-96.0, 52.0, -70.0));
            let c = rotate(vec3(96.0, 52.0, -70.0));
            for x in [-71.3, -23.7, 18.2, 39.0] {
                for z in [-80.9, -78.3, -73.1] {
                    let hit = sphere_vs_triangle(rotate(vec3(x, 58.84, z)), 7.0, a, b, c).unwrap();
                    assert_eq!(hit.normal, rotate(vec3(0.0, 1.0, 0.0)));
                    assert_eq!(hit.point, rotate(vec3(x, 52.0, z)));
                }
            }
        }
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
        assert_eq!(cp, a);

        // Outside near vertex b.
        let p = Vec3 {
            x: 2.0,
            y: -1.0,
            z: 0.0,
        };
        let cp = closest_point_on_triangle(p, a, b, c);
        assert_eq!(cp, b);

        // Outside near vertex c.
        let p = Vec3 {
            x: -1.0,
            y: 2.0,
            z: 0.0,
        };
        let cp = closest_point_on_triangle(p, a, b, c);
        assert_eq!(cp, c);

        // Interior projects straight down to the plane.
        let p = Vec3 {
            x: 0.25,
            y: 0.25,
            z: 5.0,
        };
        let cp = closest_point_on_triangle(p, a, b, c);
        assert_vec3_close(cp, vec3(0.25, 0.25, 0.0));
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

        let aabb = collider_aabb(&rc_ref!(&coll), &transform);
        assert_vec3_close(aabb.min, vec3(0.5, 1.5, 2.5));
        assert_vec3_close(aabb.max, vec3(1.5, 2.5, 3.5));
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
        let half_diagonal = std::f32::consts::FRAC_1_SQRT_2;
        assert_vec3_close(aabb.min, vec3(-half_diagonal, -0.5, -half_diagonal));
        assert_vec3_close(aabb.max, vec3(half_diagonal, 0.5, half_diagonal));
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
        assert!(matches!(s, ColliderShape::Sphere { r } if r == 0.5));
    }

    #[test]
    fn test_classify_capsule_local_y() {
        // A local-Y core segment plus radius produces a capsule.
        let s = classify_shape(
            Vec3 {
                x: 0.0,
                y: 1.2,
                z: 0.0,
            },
            0.3,
        );
        assert!(matches!(s, ColliderShape::Capsule { half_h, r } if half_h == 0.6 && r == 0.3));
    }

    #[test]
    fn test_classify_shape_uses_strict_zero_tolerance() {
        let below = classify_shape(
            Vec3 {
                x: 0.5e-9,
                y: 0.0,
                z: 0.0,
            },
            0.5,
        );
        assert!(matches!(below, ColliderShape::Sphere { .. }));

        let boundary = classify_shape(
            Vec3 {
                x: 1e-9,
                y: 0.0,
                z: 0.0,
            },
            0.5,
        );
        assert!(matches!(boundary, ColliderShape::RoundedBox { .. }));
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
                if (half.x, half.y, half.z, r) == (1.0, 0.5, 0.25, 0.1)
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
        assert!(matches!(s0, ColliderShape::RoundedBox { r, .. } if r == 0.0));

        // A planar size with nonzero X/Z is still a box.
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
        assert_vec3_close(mid, vec3(0.0, 0.5, 0.0));

        let below = closest_point_on_segment(
            Vec3 {
                x: 0.0,
                y: -9.0,
                z: 0.0,
            },
            a,
            b,
        );
        assert_vec3_close(below, a);

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
        assert_eq!(pt, a);
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
        assert_vec3_close(p, vec3(0.0, 0.0, 0.0));
        assert_vec3_close(q, vec3(0.0, 0.0, 1.0));

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
    fn test_sphere_vs_rounded_obb_axis_aligned() {
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
        assert_vec3_close(geom.normal, vec3(1.0, 0.0, 0.0));
        assert!(approx_eq(geom.point.x, 1.0));
    }

    #[test]
    fn test_sphere_vs_rounded_obb_rotated_45_no_corner_inflation() {
        // Unit-half cube rotated 45° about Y. Its true reach along the world
        // diagonal u=(1,0,1)/√2 is 1.0 (one local axis aligns with u, the
        // other horizontal axis is perpendicular).
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
        assert_vec3_close(geom.normal, vec3(inv_sqrt2, 0.0, inv_sqrt2));
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
        // separation axis, depth covers radius + interior penetration.
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
        assert_vec3_close(geom.normal, vec3(1.0, 0.0, 0.0));
        // Interior margin to +X face = 0.1; depth = 0.1 + 0.3.
        assert!(approx_eq(geom.depth, 0.4));
    }

    #[test]
    fn test_sphere_vs_plate_pushout_resolves_contact() {
        let plate = Mat4::identity_value();
        let half = vec3(2.0, 0.1, 2.0);
        let center = vec3(0.0, 0.55, 0.0);
        let geom = sphere_vs_rounded_obb(center, 0.5, &plate, half, 0.0).unwrap();
        assert_vec3_close(geom.normal, vec3(0.0, 1.0, 0.0));
        assert!(approx_eq(geom.depth, 0.05), "depth={}", geom.depth);

        let resolved = vec3(
            center.x + geom.normal.x * geom.depth,
            center.y + geom.normal.y * geom.depth,
            center.z + geom.normal.z * geom.depth,
        );
        assert!(sphere_vs_rounded_obb(resolved, 0.5, &plate, half, 0.0).is_none());
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
        assert_vec3_close(geom.normal, vec3(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_capsule_vs_sphere_cap_hit_rotated() {
        // The rotated capsule lies along X; its nearest cap center is (1, 0, 0).
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
        assert_vec3_close(geom.normal, vec3(1.0, 0.0, 0.0));
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
        // The parallel segments are 0.5 apart: depth = 0.6 - 0.5 = 0.1.
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
        assert_vec3_close(geom.normal, vec3(-1.0, 0.0, 0.0));
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
        let m_b_rc = rc_ref!(&shift_rc).mul_mat(&rc_ref!(&rot_rc));
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
    fn test_segment_vs_local_aabb_endpoint_touch_returns_zero_pair() {
        let (on_seg, on_box) = closest_points_segment_aabb(
            vec3(0.0, 0.0, 1.0),
            vec3(-1.0, 0.0, 1.2),
            vec3(1.0, 1.0, 1.0),
        );
        assert_eq!(on_seg, vec3(0.0, 0.0, 1.0));
        assert_eq!(on_box, vec3(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_segment_vs_local_aabb_through_box_returns_coincident_midpoint() {
        let (on_seg, on_box) = closest_points_segment_aabb(
            vec3(-2.0, 0.25, 0.5),
            vec3(2.0, 0.25, 0.5),
            vec3(1.0, 1.0, 1.0),
        );
        assert_vec3_close(on_seg, vec3(0.0, 0.25, 0.5));
        assert_vec3_close(on_box, vec3(0.0, 0.25, 0.5));
    }

    #[test]
    fn test_segment_vs_local_aabb_edge_region_minimizes_quadratic() {
        let (on_seg, on_box) = closest_points_segment_aabb(
            vec3(0.0, 2.0, 1.0),
            vec3(0.0, 0.0, 2.0),
            vec3(1.0, 1.0, 1.0),
        );
        assert_vec3_close(on_seg, vec3(0.0, 1.2, 1.4));
        assert_vec3_close(on_box, vec3(0.0, 1.0, 1.0));
    }

    #[test]
    fn test_segment_vs_local_aabb_degenerate_segment_clamps_point() {
        let (on_seg, on_box) = closest_points_segment_aabb(
            vec3(2.0, 0.5, -2.0),
            vec3(2.0, 0.5, -2.0),
            vec3(1.0, 1.0, 1.0),
        );
        assert_eq!(on_seg, vec3(2.0, 0.5, -2.0));
        assert_eq!(on_box, vec3(1.0, 0.5, -1.0));
    }

    #[test]
    fn test_capsule_vs_rounded_obb_standing_on_box() {
        // The capsule bottom is 1.75 - 0.5 - 0.3 = 0.95, penetrating the box top by 0.05.
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
        assert_vec3_close(geom.normal, vec3(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_capsule_crossing_box_clears_full_segment() {
        let half = vec3(10.0, 0.1, 10.0);
        let rotation = *rc_ref!(&Mat4::from_euler(&vec3(0.0, 0.0, 35.0)));
        let lying = *rc_ref!(&Mat4::from_euler(&vec3(0.0, 0.0, 90.0)));

        for box_world in [Mat4::identity_value(), rotation] {
            for (local_rotation, center_y, depth) in [
                (Mat4::identity_value(), -0.95, 0.25),
                (Mat4::identity_value(), -0.25, 0.95),
                (Mat4::identity_value(), 0.0, 1.2),
                (Mat4::identity_value(), 0.25, 0.95),
                (Mat4::identity_value(), 0.95, 0.25),
                (lying, 0.02, 0.18),
            ] {
                let local = translation(vec3(0.0, center_y, 0.0)).mul_mat_value(&local_rotation);
                let capsule = box_world.mul_mat_value(&local);
                let geom =
                    capsule_vs_rounded_obb(&capsule, 1.0, 0.1, &box_world, half, 0.0).unwrap();
                let sign = if center_y < 0.0 { -1.0 } else { 1.0 };
                assert_vec3_close(geom.normal, box_world.mul_dir_value(&vec3(0.0, sign, 0.0)));
                assert!(approx_eq(geom.depth, depth), "depth={}", geom.depth);

                let shift = vec3(
                    geom.normal.x * geom.depth,
                    geom.normal.y * geom.depth,
                    geom.normal.z * geom.depth,
                );
                let resolved = translation(shift).mul_mat_value(&capsule);
                assert!(
                    capsule_vs_rounded_obb(&resolved, 1.0, 0.1, &box_world, half, 0.0).is_none()
                );
            }
        }
    }

    #[test]
    fn test_capsule_crossing_box_uses_edge_separation_axis() {
        // A diagonal segment through the box exits perpendicular to its axis,
        // past the (1,-1) edge, rather than pushing either endpoint past a face.
        let half = vec3(1.0, 1.0, 10.0);
        let box_world = Mat4::identity_value();
        let capsule = *rc_ref!(&Mat4::from_euler(&vec3(0.0, 0.0, -45.0)));
        let geom = capsule_vs_rounded_obb(&capsule, 3.0, 0.1, &box_world, half, 0.2).unwrap();
        let axis = std::f32::consts::FRAC_1_SQRT_2;
        assert_vec3_close(geom.normal, vec3(axis, -axis, 0.0));
        assert!(approx_eq(geom.depth, std::f32::consts::SQRT_2 + 0.3));

        let shift = vec3(
            geom.normal.x * geom.depth,
            geom.normal.y * geom.depth,
            geom.normal.z * geom.depth,
        );
        let resolved = translation(shift).mul_mat_value(&capsule);
        let top = resolved.mul_vec_value(&vec3(0.0, 3.0, 0.0));
        let bot = resolved.mul_vec_value(&vec3(0.0, -3.0, 0.0));
        let (on_seg, on_box) = closest_points_segment_aabb(top, bot, half);
        assert!(approx_eq(point_dist2(on_seg, on_box).sqrt(), 0.3));
        assert!(capsule_vs_rounded_obb(&resolved, 3.0, 0.1, &box_world, half, 0.2).is_none());
    }

    #[test]
    fn test_capsule_vs_wall_side_pushout_is_horizontal_and_resolves() {
        let wall = Mat4::identity_value();
        let wall_half = vec3(0.2, 0.8, 6.0);
        let capsule = translation(vec3(0.52, 0.75, 0.0));
        let geom = capsule_vs_rounded_obb(&capsule, 0.4, 0.35, &wall, wall_half, 0.0).unwrap();
        assert_vec3_close(geom.normal, vec3(1.0, 0.0, 0.0));
        assert!(approx_eq(geom.depth, 0.03), "depth={}", geom.depth);

        let moved = translation(vec3(
            geom.normal.x * geom.depth,
            geom.normal.y * geom.depth,
            geom.normal.z * geom.depth,
        ));
        let resolved = *rc_ref!(&moved.mul_mat(&capsule));
        if let Some(after) = capsule_vs_rounded_obb(&resolved, 0.4, 0.35, &wall, wall_half, 0.0) {
            panic!(
                "pushout left contact normal=({:.6}, {:.6}, {:.6}) depth={:.9}",
                after.normal.x, after.normal.y, after.normal.z, after.depth
            );
        }
    }

    #[test]
    fn test_capsule_vs_wall_face_normal_stays_horizontal_across_overlap_band() {
        let wall = Mat4::identity_value();
        let wall_half = vec3(0.2, 0.8, 6.0);
        for center_y in [0.45, 0.75, 1.05] {
            let capsule = translation(vec3(0.52, center_y, 0.0));
            let geom = capsule_vs_rounded_obb(&capsule, 0.4, 0.35, &wall, wall_half, 0.0).unwrap();
            assert!(
                geom.normal.x > 0.999 && geom.normal.y.abs() < 1e-4 && geom.normal.z.abs() < 1e-4,
                "center_y={center_y} normal=({:.6}, {:.6}, {:.6})",
                geom.normal.x,
                geom.normal.y,
                geom.normal.z
            );
        }
    }

    #[test]
    fn test_capsule_vs_rotated_obb_no_corner_inflation() {
        // Box rotated 45° about Y (true diagonal reach 1.0, AABB reach 2.0).
        // Vertical capsule axis at 1.5 along u=(1,0,1)/√2 with r=0.3 →
        // gap 0.5 - 0.3 = 0.2: no contact.
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
    fn test_obb_vs_obb_axis_aligned() {
        // Unit-half boxes with centers 1.8 apart on X overlap 0.2 on X.
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
        assert_vec3_close(geom.normal, vec3(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_obb_vs_obb_rotated_45_diagonal_gap() {
        // Along u=(1,0,1)/√2, combined reach is √2 + 1: centers 2.2 apart
        // overlap, while centers 2.6 apart separate on B's aligned axis.
        let m_a = Mat4::identity_value();
        let rot_rc = Mat4::from_euler(&Vec3 {
            x: 0.0,
            y: 45.0,
            z: 0.0,
        });
        let d = 2.2 / std::f32::consts::SQRT_2;
        let shift_rc = Mat4::from_translation(&Vec3 { x: d, y: 0.0, z: d });
        let m_b_rc = rc_ref!(&shift_rc).mul_mat(&rc_ref!(&rot_rc));
        let m_b = *rc_ref!(&m_b_rc);
        let one = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        assert!(rounded_obb_vs_rounded_obb(&m_a, one, 0.0, &m_b, one, 0.0).is_some());

        let d2 = 2.6 / std::f32::consts::SQRT_2;
        let shift2_rc = Mat4::from_translation(&Vec3 {
            x: d2,
            y: 0.0,
            z: d2,
        });
        let m_b2_rc = rc_ref!(&shift2_rc).mul_mat(&rc_ref!(&rot_rc));
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
    fn test_rounded_obb_separated_core_features() {
        let identity = Mat4::identity_value();
        let half = vec3(1.0, 1.0, 1.0);
        for (center, distance, normal) in [
            (vec3(2.1, 0.0, 0.0), 0.1, vec3(-1.0, 0.0, 0.0)),
            (
                vec3(2.1, 2.1, 0.0),
                0.1 * 2.0_f32.sqrt(),
                vec3(-1.0, -1.0, 0.0),
            ),
            (
                vec3(2.1, 2.1, 2.1),
                0.1 * 3.0_f32.sqrt(),
                vec3(-1.0, -1.0, -1.0),
            ),
        ] {
            let target = translation(center);
            let contact =
                rounded_obb_vs_rounded_obb(&identity, half, 0.1, &target, half, 0.1).unwrap();
            assert!(approx_eq(contact.depth, 0.2 - distance));
            let length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
            assert_vec3_close(
                contact.normal,
                vec3(normal.x / length, normal.y / length, normal.z / length),
            );
            let reverse =
                rounded_obb_vs_rounded_obb(&target, half, 0.1, &identity, half, 0.1).unwrap();
            assert!(approx_eq(reverse.depth, contact.depth));
            assert_vec3_close(
                reverse.normal,
                vec3(-contact.normal.x, -contact.normal.y, -contact.normal.z),
            );
        }
        for center in [vec3(2.15, 2.15, 0.0), vec3(2.12, 2.12, 2.12)] {
            assert!(rounded_obb_vs_rounded_obb(
                &identity,
                half,
                0.1,
                &translation(center),
                half,
                0.1
            )
            .is_none());
        }
        // Exact face touching stays excluded from discrete overlap.
        assert!(rounded_obb_vs_rounded_obb(
            &identity,
            half,
            0.125,
            &translation(vec3(2.25, 0.0, 0.0)),
            half,
            0.125
        )
        .is_none());
    }

    #[test]
    fn test_rounded_obb_rotated_vertex_distance() {
        let identity = Mat4::identity_value();
        let half = vec3(1.0, 1.0, 1.0);
        let rotation = *rc_ref!(&Mat4::from_euler(&vec3(0.0, 0.0, 45.0)));
        // B's leftmost vertex is beyond A's upper-right corner. Both
        // adjacent B edges recede from that corner, so its distance is
        // hypot(dx, dy), including when all 15 expanded axes overlap.
        for (dx, dy) in [(0.15, 0.1), (0.19, 0.07)] {
            let mut target = rotation;
            target.data[0][3] = 1.0 + 2.0_f32.sqrt() + dx;
            target.data[1][3] = 1.0 + dy;
            let contact = rounded_obb_vs_rounded_obb(&identity, half, 0.1, &target, half, 0.1);
            let distance = (dx * dx + dy * dy).sqrt();
            if distance < 0.2 {
                let contact = contact.unwrap();
                assert!(approx_eq(contact.depth, 0.2 - distance));
                assert_vec3_close(contact.normal, vec3(-dx / distance, -dy / distance, 0.0));
            } else {
                assert!(contact.is_none());
            }
        }
    }

    #[test]
    fn test_box_on_a_ledge_separates_upward_at_its_vertical_edge() {
        let half = vec3(28.0, 4.0, 10.0);
        for side in [-1.0, 1.0] {
            let x = side * 25.0;
            let hit = local_box_vs_triangle(
                half,
                0.0,
                vec3(x, -3.84, -25.0),
                vec3(x, -11.84, 25.0),
                vec3(x, -3.84, 25.0),
            )
            .unwrap();
            assert_vec3_close(hit.normal, vec3(0.0, 1.0, 0.0));
            assert!(approx_eq(hit.depth, 0.16));
        }
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
        assert_vec3_close(geom.normal, vec3(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_capsule_crossing_triangle_face_clears_plane() {
        let v0 = vec3(-10.0, 0.0, -10.0);
        let v1 = vec3(10.0, 0.0, -10.0);
        let v2 = vec3(0.0, 0.0, 10.0);

        for (b, c, tie_sign) in [(v1, v2, -1.0), (v2, v1, 1.0)] {
            for (y, angle, expected_depth) in [
                (0.0, 0.0, 1.1),
                (0.25, 0.0, 0.85),
                (-0.25, 0.0, 0.85),
                (0.25, 45.0, std::f32::consts::FRAC_1_SQRT_2 - 0.15),
            ] {
                let rotation = Mat4::from_axis_angle_value(&vec3(0.0, 0.0, 1.0), angle);
                let cap = translation(vec3(0.0, y, 0.0)).mul_mat_value(&rotation);
                let geom = capsule_vs_triangle(&cap, 1.0, 0.1, v0, b, c)
                    .expect("capsule axis crosses triangle face");
                let sign = if y == 0.0 { tie_sign } else { y.signum() };
                assert_vec3_close(geom.normal, vec3(0.0, sign, 0.0));
                assert!(
                    approx_eq(geom.depth, expected_depth),
                    "depth={}",
                    geom.depth
                );
                assert!(approx_eq(geom.point.y, 0.0));

                let push = translation(vec3(
                    geom.normal.x * geom.depth,
                    geom.normal.y * geom.depth,
                    geom.normal.z * geom.depth,
                ));
                let resolved = push.mul_mat_value(&cap);
                let remaining = capsule_vs_triangle(&resolved, 1.0, 0.1, v0, b, c)
                    .map_or(0.0, |contact| contact.depth);
                assert!(remaining < CONTACT_EPSILON, "remaining depth={remaining}");

                assert!(capsule_vs_triangle(&cap, 1.0, 0.0, v0, b, c).is_none());
            }
        }
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
        let cap_rc = rc_ref!(&shift_rc).mul_mat(&rc_ref!(&rot_rc));
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
        assert_vec3_close(geom.normal, vec3(0.0, 1.0, 0.0));
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
