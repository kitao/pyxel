// Collision routines follow the standard short-letter notation
// (Moeller-Trumbore: h/a/f/s/q/u/v/t). Renaming hurts traceability
// against the reference formulation.
#![allow(clippy::many_single_char_names)]

use crate::cube::collider::Collider;
use crate::cube::mat4::Mat4;
use crate::cube::mesh::Mesh;
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
            let wc_rc = transform.mul_vec(c);
            let wc = *rc_ref!(&wc_rc);
            min.x = min.x.min(wc.x);
            min.y = min.y.min(wc.y);
            min.z = min.z.min(wc.z);
            max.x = max.x.max(wc.x);
            max.y = max.y.max(wc.y);
            max.z = max.z.max(wc.z);
        }
        Self { min, max }
    }

    // Mesh-collider AABB: union of every part's transformed positions.
    pub fn from_mesh(mesh: &Mesh, transform: &Mat4) -> Self {
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
                let wp_rc = world_t.mul_vec(&p);
                let wp = *rc_ref!(&wp_rc);
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
            let p_rc = transform.mul_vec(&Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            });
            let p = *rc_ref!(&p_rc);
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

// Closest point on triangle to p (Ericson, Real-Time Collision
// Detection §5.1.5). Returns the barycentric point on the triangle
// or its nearest edge / vertex.
fn closest_point_on_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
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

// AABB vs triangle via the Separating-Axis Theorem with 13 axes:
//   - 3 AABB face normals (X, Y, Z)
//   - 1 triangle face normal
//   - 9 cross products of AABB edges with triangle edges
// Returns ContactGeom with the triangle face normal oriented toward
// the AABB center, and the minimum overlap along the 13 axes as the
// depth (a good practical proxy at PS1 scale).
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
    // Triangle vertices relative to AABB center.
    let p0 = Vec3 {
        x: v0.x - center.x,
        y: v0.y - center.y,
        z: v0.z - center.z,
    };
    let p1 = Vec3 {
        x: v1.x - center.x,
        y: v1.y - center.y,
        z: v1.z - center.z,
    };
    let p2 = Vec3 {
        x: v2.x - center.x,
        y: v2.y - center.y,
        z: v2.z - center.z,
    };
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
    // to a point and yields zero overlap even when the AABB straddles
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
            sat_overlap(&axis, &p0, &p1, &p2, &extents)?;
        }
    }
    for a in &aabb_axes {
        sat_overlap(a, &p0, &p1, &p2, &extents)?;
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
    sat_overlap(&face_normal, &p0, &p1, &p2, &extents)?;
    // Orient the normal toward the AABB center (= origin in this
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
    // Penetration depth along the (oriented) face normal: AABB
    // half-extent along the normal minus the signed distance from
    // the AABB center (origin) to the triangle plane.
    let r_along_normal =
        extents.x * normal.x.abs() + extents.y * normal.y.abs() + extents.z * normal.z.abs();
    let plane_offset = normal.x * p0.x + normal.y * p0.y + normal.z * p0.z;
    let depth = (r_along_normal - plane_offset.abs()).max(0.0);
    Some(ContactGeom {
        point: Vec3 {
            x: (v0.x + v1.x + v2.x) / 3.0,
            y: (v0.y + v1.y + v2.y) / 3.0,
            z: (v0.z + v1.z + v2.z) / 3.0,
        },
        normal,
        depth,
    })
}

// SAT projection helper. Returns Some(overlap) if the projections of
// the AABB and triangle along `axis` overlap; None if they are
// separating (which means the SAT verdict is "no collision").
fn sat_overlap(axis: &Vec3, p0: &Vec3, p1: &Vec3, p2: &Vec3, extents: &Vec3) -> Option<f32> {
    let pr0 = p0.x * axis.x + p0.y * axis.y + p0.z * axis.z;
    let pr1 = p1.x * axis.x + p1.y * axis.y + p1.z * axis.z;
    let pr2 = p2.x * axis.x + p2.y * axis.y + p2.z * axis.z;
    let tri_min = pr0.min(pr1).min(pr2);
    let tri_max = pr0.max(pr1).max(pr2);
    let r = extents.x * axis.x.abs() + extents.y * axis.y.abs() + extents.z * axis.z.abs();
    if tri_max < -r || tri_min > r {
        return None;
    }
    Some((tri_max.min(r) - tri_min.max(-r)).max(0.0))
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
}
