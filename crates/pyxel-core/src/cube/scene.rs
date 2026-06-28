use std::cell::Cell;

use crate::cube::camera::RcCamera;
use crate::cube::collider::RcCollider;
use crate::cube::collision::{
    capsule_vs_capsule, capsule_vs_rounded_obb, capsule_vs_sphere, capsule_vs_triangle,
    classify_shape, closest_point_on_triangle, closest_points_segment_aabb,
    closest_points_segment_segment, collider_aabb, local_box_vs_triangle, ray_vs_aabb,
    ray_vs_sphere, ray_vs_triangle, rounded_obb_vs_rounded_obb, sphere_vs_rounded_obb,
    sphere_vs_sphere, sphere_vs_triangle, Aabb, ColliderShape, ContactGeom,
};
use crate::cube::contact::{Contact, RcContact};
use crate::cube::mat4::Mat4;
use crate::cube::mesh::RcMesh;
use crate::cube::node::{Node, RcNode};
use crate::cube::quat::Quat;
use crate::cube::raster::{ClipRect, Mat4x4};
use crate::cube::vec3::Vec3;
use crate::image::RcImage;

// Per-frame rasterizer context shared between Node::draw and each Node's
// draw commands. Built at the start of Node::draw (depth buffer moved out
// of the effective camera's cache for the duration of the traversal),
// looked up by Node draw commands through `with_draw_context`, torn down
// on draw end (depth buffer moved back into the camera).

// One transformed vertex in the prim scratch cache: world position and
// screen projection (None = behind the near plane).
pub type ProjectedVertex = (Vec3, Option<(f32, f32, f32)>);

pub struct DrawContext {
    pub target: RcImage,
    pub vp: Mat4x4,
    pub vp_x: f32,
    pub vp_y: f32,
    pub vp_w: f32,
    pub vp_h: f32,
    pub clip: ClipRect,
    pub camera: RcCamera,
    // Depth buffer (and its dimensions) owned by ctx for the duration of
    // one draw. The effective camera caches the allocation between frames;
    // ctx takes it in at draw entry and returns it on exit.
    pub depth: Vec<f32>,
    pub depth_w: u32,
    pub depth_h: u32,
    // Per-draw scratch holding each vertex's world position and screen
    // projection. Cleared (not reallocated) at every prim call, so
    // indexed tables with shared vertices project each vertex once.
    pub vertex_cache: Vec<ProjectedVertex>,
    // Per-on_draw state modifiers, mutated via Node.dither / depth_test /
    // depth_write / depth_offset / shaded setters; reset to defaults before
    // each Node's on_draw via reset_draw_state(). Rasterizers consult ctx
    // for these fields directly.
    pub dither_alpha: f32,
    pub depth_test: bool,
    pub depth_write: bool,
    // World-unit depth bias applied at projection time: a draw's depth is
    // computed as if it were shifted this many units along the camera's
    // view direction (negative = toward the camera), while its screen
    // position is left unchanged. 0.0 = no bias.
    pub depth_offset: f32,
    pub shaded: bool,
}

thread_local! {
    // Current draw context, set by Scene::draw for the duration of the
    // tree traversal. Single-threaded by design (cube runs on Pyxel's
    // main thread); thread_local is the minimal carrier for the
    // Rust-Python boundary.
    static CURRENT_DRAW_CONTEXT: Cell<Option<DrawContext>> = const { Cell::new(None) };
}

pub fn set_draw_context(ctx: DrawContext) {
    CURRENT_DRAW_CONTEXT.with(|c| c.set(Some(ctx)));
}

pub fn clear_draw_context() {
    CURRENT_DRAW_CONTEXT.with(|c| c.set(None));
}

// Move the active draw context out of the thread-local, returning it to
// the caller. After this call, with_draw_context returns None until
// set_draw_context is called again. Used by Node::draw to recover the
// depth buffer after traverse_draw completes.
pub fn take_draw_context() -> Option<DrawContext> {
    CURRENT_DRAW_CONTEXT.with(Cell::take)
}

// Reset the per-on_draw state modifiers on the active draw context to
// their defaults. Called by the binding's traverse_draw before invoking
// each Node.on_draw so state never leaks across siblings or children.
pub fn reset_draw_state() {
    with_draw_context(|ctx| {
        ctx.dither_alpha = 1.0;
        ctx.depth_test = true;
        ctx.depth_write = true;
        ctx.depth_offset = 0.0;
        ctx.shaded = true;
    });
}

// Run `f` with mutable access to the current draw context, returning
// None when no context is active (i.e., outside Scene::draw).
pub fn with_draw_context<R>(f: impl FnOnce(&mut DrawContext) -> R) -> Option<R> {
    CURRENT_DRAW_CONTEXT.with(|cell| {
        let mut ctx = cell.take()?;
        let result = f(&mut ctx);
        cell.set(Some(ctx));
        Some(result)
    })
}

// Pipeline data types passed across the binding boundary.

pub struct ContactPair {
    pub node_a: RcNode,
    pub node_b: RcNode,
    pub contact_a: RcContact,
    pub contact_b: RcContact,
}

pub struct RaycastHitInfo {
    pub node: RcNode,
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}

// Zero-sized namespace for the core pipeline + spatial-query functions.
// Scene used to be a per-frame state container; depth buffer ownership
// has moved onto the Node receiving draw(), and the Python-visible
// surface no longer exposes Scene. The methods below remain associated
// with this type purely for namespacing — they take an `RcNode` subtree
// root and operate on it.
pub struct Scene;

// Pipeline + spatial-query implementation. The Python-visible steps
// (on_update, on_collide, on_destroy) stay in the binding layer; this
// block covers the deterministic core stages. Several stage helpers keep
// explicit physics inputs instead of allocating per-frame option structs.

#[allow(clippy::too_many_arguments)]
impl Scene {
    // Step 8 (cube-design.md § 16): collect every destroyed node in
    // the subtree in post-order (= leaf first), so that callers can
    // run on_destroy + detach in dependency order. The collector is
    // pure (no detachment); the binding wires the actual on_destroy
    // notification and detachment loop on top of this list.
    pub fn collect_destroyed_post_order(scene_root: &RcNode) -> Vec<RcNode> {
        let mut out: Vec<RcNode> = Vec::new();
        Self::collect_destroyed_recursive(scene_root, &mut out);
        out
    }

    fn collect_destroyed_recursive(node: &RcNode, out: &mut Vec<RcNode>) {
        let children = rc_ref!(node).children.clone();
        for child in &children {
            Self::collect_destroyed_recursive(child, out);
        }
        if rc_ref!(node).destroyed {
            out.push(node.clone());
        }
    }

    // Pop a destroyed node out of its parent. Safe to call repeatedly
    // (no-op if already detached).
    pub fn detach_destroyed(node: &RcNode) {
        Node::detach(node);
    }

    // Step 2: motion integration. Walks the active subtree and applies
    // each collider's velocity / angular_velocity to its node transform.
    pub fn integrate_motion(scene_root: &RcNode) {
        let mut stack: Vec<RcNode> = vec![scene_root.clone()];
        while let Some(node) = stack.pop() {
            if !Node::effective_active(&node) {
                continue;
            }
            let coll_opt = rc_ref!(&node).collider.clone();
            if let Some(coll_rc) = coll_opt {
                let coll = rc_ref!(&coll_rc);
                let vel = *rc_ref!(&coll.velocity);
                let avel = *rc_ref!(&coll.angular_velocity);
                let cur_t = rc_ref!(&node).transform.clone();
                // Velocity is a world-space displacement: left-multiply
                // by a translation so the body moves along world axes
                // regardless of its current orientation. Mat4::translate
                // (spec § 5.6) applies a local-frame shift and would
                // bend a rotating ball's path through its own basis.
                let t_vel = Mat4::from_translation(&vel);
                let translated = rc_ref!(&t_vel).mul_mat(rc_ref!(&cur_t));
                let alen_sq = avel.x * avel.x + avel.y * avel.y + avel.z * avel.z;
                let final_t = if alen_sq > 1e-12 {
                    let len = alen_sq.sqrt();
                    let axis = Vec3 {
                        x: avel.x / len,
                        y: avel.y / len,
                        z: avel.z / len,
                    };
                    // Spin is applied as a local-frame rotation
                    // (right-multiplication): the body rotates around
                    // its own origin and its world position stays where
                    // `translated` placed it. Left-multiplying R would
                    // rotate the body about the world origin and drag
                    // its position along the orbit.
                    let r = Mat4::from_axis_angle(&axis, len);
                    rc_ref!(&translated).mul_mat(rc_ref!(&r))
                } else {
                    translated
                };
                rc_mut!(&node).transform = final_t;
            }
            let children = rc_ref!(&node).children.clone();
            for c in children.into_iter().rev() {
                stack.push(c);
            }
        }
    }

    // Steps 3-6: AABB refresh, broad phase, narrow phase, response
    // resolution. Returns the contact pairs the binding layer feeds to
    // on_collide.
    pub fn detect_contacts(scene_root: &RcNode) -> Vec<ContactPair> {
        let mut entries: Vec<(RcNode, Mat4, Aabb)> = Vec::new();
        Self::collect_collider_entries(scene_root, &mut entries, true);
        let n = entries.len();
        let mut pairs: Vec<ContactPair> = Vec::new();
        // O(N^2) broad phase; PS1-scale (~100 dynamics) is in budget.
        for i in 0..n {
            for j in (i + 1)..n {
                if !entries[i].2.overlaps(&entries[j].2) {
                    continue;
                }
                let coll_a_opt = rc_ref!(&entries[i].0).collider.clone();
                let coll_b_opt = rc_ref!(&entries[j].0).collider.clone();
                let (Some(coll_a_rc), Some(coll_b_rc)) = (coll_a_opt, coll_b_opt) else {
                    continue;
                };
                let (mass_a, trigger_a) = {
                    let coll = rc_ref!(&coll_a_rc);
                    (
                        if coll.mesh.is_some() { 0.0 } else { coll.mass },
                        coll.trigger,
                    )
                };
                let (mass_b, trigger_b) = {
                    let coll = rc_ref!(&coll_b_rc);
                    (
                        if coll.mesh.is_some() { 0.0 } else { coll.mass },
                        coll.trigger,
                    )
                };
                if mass_a == 0.0 && mass_b == 0.0 && !trigger_a && !trigger_b {
                    continue;
                }
                let contact =
                    Self::narrow_phase(&entries[i].1, &coll_a_rc, &entries[j].1, &coll_b_rc);
                if let Some(geom) = contact {
                    let pair = Self::build_contact_pair(
                        &entries[i].0,
                        &coll_a_rc,
                        &entries[j].0,
                        &coll_b_rc,
                        geom,
                    );
                    pairs.push(pair);
                }
            }
        }
        pairs
    }

    fn collect_collider_entries(node: &RcNode, out: &mut Vec<(RcNode, Mat4, Aabb)>, swept: bool) {
        if !Node::effective_active(node) {
            return;
        }
        let coll_opt = rc_ref!(node).collider.clone();
        if let Some(coll_rc) = coll_opt {
            let coll = rc_ref!(&coll_rc);
            let world = Node::world_transform_value(node);
            let mut aabb = collider_aabb(coll, &world);
            if swept && coll.mesh.is_none() {
                let velocity = *rc_ref!(&coll.velocity);
                aabb = Self::swept_aabb(aabb, velocity);
            }
            out.push((node.clone(), world, aabb));
        }
        let children = rc_ref!(node).children.clone();
        for c in &children {
            Self::collect_collider_entries(c, out, swept);
        }
    }

    fn swept_aabb(aabb: Aabb, velocity: Vec3) -> Aabb {
        let previous = Aabb {
            min: Vec3 {
                x: aabb.min.x - velocity.x,
                y: aabb.min.y - velocity.y,
                z: aabb.min.z - velocity.z,
            },
            max: Vec3 {
                x: aabb.max.x - velocity.x,
                y: aabb.max.y - velocity.y,
                z: aabb.max.z - velocity.z,
            },
        };
        Aabb {
            min: Vec3 {
                x: aabb.min.x.min(previous.min.x),
                y: aabb.min.y.min(previous.min.y),
                z: aabb.min.z.min(previous.min.z),
            },
            max: Vec3 {
                x: aabb.max.x.max(previous.max.x),
                y: aabb.max.y.max(previous.max.y),
                z: aabb.max.z.max(previous.max.z),
            },
        }
    }

    fn narrow_phase(
        world_a: &Mat4,
        coll_a: &RcCollider,
        world_b: &Mat4,
        coll_b: &RcCollider,
    ) -> Option<ContactGeom> {
        use ColliderShape as S;
        let (size_a, r_a, has_mesh_a, mesh_a) = {
            let a = rc_ref!(coll_a);
            (
                *rc_ref!(&a.size),
                a.radius.max(0.0),
                a.mesh.is_some(),
                a.mesh.clone(),
            )
        };
        let (size_b, r_b, has_mesh_b, mesh_b) = {
            let b = rc_ref!(coll_b);
            (
                *rc_ref!(&b.size),
                b.radius.max(0.0),
                b.mesh.is_some(),
                b.mesh.clone(),
            )
        };
        // Mesh-vs-mesh is unsupported: both sides are static terrain
        // with no resolution payload.
        if has_mesh_a && has_mesh_b {
            return None;
        }
        if has_mesh_a {
            let mesh_rc = mesh_a.unwrap();
            // Helper returns the normal pointing from mesh toward the
            // dynamic body. The contact-pair builder's convention is
            // "from b toward a"; since the mesh sits on the a-side
            // here, flip the normal.
            let geom = narrow_phase_mesh_vs_dynamic(world_a, &mesh_rc, world_b, size_b, r_b)
                .or_else(|| {
                    Self::swept_sphere_vs_mesh(
                        world_a, &mesh_rc, world_b, size_b, r_b, coll_b, coll_a,
                    )
                })
                .or_else(|| {
                    Self::swept_capsule_vs_mesh(
                        world_a, &mesh_rc, world_b, size_b, r_b, coll_b, coll_a,
                    )
                })
                .or_else(|| {
                    Self::swept_rounded_obb_vs_mesh(
                        world_a, &mesh_rc, world_b, size_b, r_b, coll_b, coll_a,
                    )
                })?;
            return Some(ContactGeom {
                point: geom.point,
                normal: Vec3 {
                    x: -geom.normal.x,
                    y: -geom.normal.y,
                    z: -geom.normal.z,
                },
                depth: geom.depth,
            });
        }
        if has_mesh_b {
            let mesh_rc = mesh_b.unwrap();
            return narrow_phase_mesh_vs_dynamic(world_b, &mesh_rc, world_a, size_a, r_a).or_else(
                || {
                    Self::swept_sphere_vs_mesh(
                        world_b, &mesh_rc, world_a, size_a, r_a, coll_a, coll_b,
                    )
                    .or_else(|| {
                        Self::swept_capsule_vs_mesh(
                            world_b, &mesh_rc, world_a, size_a, r_a, coll_a, coll_b,
                        )
                    })
                    .or_else(|| {
                        Self::swept_rounded_obb_vs_mesh(
                            world_b, &mesh_rc, world_a, size_a, r_a, coll_a, coll_b,
                        )
                    })
                },
            );
        }
        let c_a = world_a.pos_value();
        let c_b = world_b.pos_value();
        // Shape-exact dispatch (cube-design.md § 11.1): each pair is
        // solved in the body frame of the box side (or on segments for
        // capsules), so rotation is honored instead of being inflated
        // into a world AABB. Pair helpers document their own normal
        // orientation; arms that solve with the roles swapped flip back
        // to the b → a contract.
        let flip = |g: ContactGeom| ContactGeom {
            point: g.point,
            normal: Vec3 {
                x: -g.normal.x,
                y: -g.normal.y,
                z: -g.normal.z,
            },
            depth: g.depth,
        };
        match (classify_shape(size_a, r_a), classify_shape(size_b, r_b)) {
            (S::Sphere { r: ra }, S::Sphere { r: rb }) => sphere_vs_sphere(c_a, ra, c_b, rb)
                .or_else(|| Self::swept_sphere_vs_sphere(c_a, ra, coll_a, c_b, rb, coll_b)),
            (S::Sphere { r: ra }, S::RoundedBox { half, r }) => {
                // Normal box → sphere = b → a: no flip.
                sphere_vs_rounded_obb(c_a, ra, world_b, half, r).or_else(|| {
                    Self::swept_sphere_vs_rounded_obb(c_a, ra, coll_a, world_b, half, r, coll_b)
                })
            }
            (S::RoundedBox { half, r }, S::Sphere { r: rb }) => {
                sphere_vs_rounded_obb(c_b, rb, world_a, half, r)
                    .or_else(|| {
                        Self::swept_sphere_vs_rounded_obb(c_b, rb, coll_b, world_a, half, r, coll_a)
                    })
                    .map(flip)
            }
            (S::Sphere { r: ra }, S::Capsule { half_h, r }) => {
                // Normal capsule → sphere = b → a: no flip.
                capsule_vs_sphere(world_b, half_h, r, c_a, ra).or_else(|| {
                    Self::swept_sphere_vs_capsule(c_a, ra, coll_a, world_b, half_h, r, coll_b)
                })
            }
            (S::Capsule { half_h, r }, S::Sphere { r: rb }) => {
                capsule_vs_sphere(world_a, half_h, r, c_b, rb)
                    .or_else(|| {
                        Self::swept_sphere_vs_capsule(c_b, rb, coll_b, world_a, half_h, r, coll_a)
                    })
                    .map(flip)
            }
            (S::Capsule { half_h: ha, r: ra }, S::Capsule { half_h: hb, r: rb }) => {
                capsule_vs_capsule(world_a, ha, ra, world_b, hb, rb).or_else(|| {
                    Self::swept_capsule_vs_capsule(world_a, ha, ra, coll_a, world_b, hb, rb, coll_b)
                })
            }
            (S::Capsule { half_h, r }, S::RoundedBox { half, r: br }) => {
                // Normal box → capsule = b → a: no flip.
                capsule_vs_rounded_obb(world_a, half_h, r, world_b, half, br).or_else(|| {
                    Self::swept_capsule_vs_rounded_obb(
                        world_a, half_h, r, coll_a, world_b, half, br, coll_b,
                    )
                })
            }
            (S::RoundedBox { half, r: br }, S::Capsule { half_h, r }) => {
                capsule_vs_rounded_obb(world_b, half_h, r, world_a, half, br)
                    .or_else(|| {
                        Self::swept_capsule_vs_rounded_obb(
                            world_b, half_h, r, coll_b, world_a, half, br, coll_a,
                        )
                    })
                    .map(flip)
            }
            (S::RoundedBox { half: ha, r: ra }, S::RoundedBox { half: hb, r: rb }) => {
                rounded_obb_vs_rounded_obb(world_a, ha, ra, world_b, hb, rb).or_else(|| {
                    Self::swept_rounded_obb_vs_rounded_obb(
                        world_a, ha, ra, coll_a, world_b, hb, rb, coll_b,
                    )
                })
            }
        }
    }

    // Continuous collision helpers

    fn swept_sphere_vs_sphere(
        c_a: Vec3,
        r_a: f32,
        coll_a: &RcCollider,
        c_b: Vec3,
        r_b: f32,
        coll_b: &RcCollider,
    ) -> Option<ContactGeom> {
        let vel_a_rc = rc_ref!(coll_a).velocity.clone();
        let vel_b_rc = rc_ref!(coll_b).velocity.clone();
        let vel_a = *rc_ref!(&vel_a_rc);
        let vel_b = *rc_ref!(&vel_b_rc);
        let d0 = Vec3 {
            x: c_a.x - vel_a.x - (c_b.x - vel_b.x),
            y: c_a.y - vel_a.y - (c_b.y - vel_b.y),
            z: c_a.z - vel_a.z - (c_b.z - vel_b.z),
        };
        let rel = Vec3 {
            x: vel_a.x - vel_b.x,
            y: vel_a.y - vel_b.y,
            z: vel_a.z - vel_b.z,
        };
        let a = rel.x * rel.x + rel.y * rel.y + rel.z * rel.z;
        if a < 1e-12 {
            return None;
        }
        let r_sum = r_a + r_b;
        let b = 2.0 * (d0.x * rel.x + d0.y * rel.y + d0.z * rel.z);
        let c = d0.x * d0.x + d0.y * d0.y + d0.z * d0.z - r_sum * r_sum;
        if c <= 0.0 {
            return None;
        }
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }
        let toi = (-b - disc.sqrt()) / (2.0 * a);
        if !(0.0..=1.0).contains(&toi) {
            return None;
        }
        let ca_hit = Vec3 {
            x: c_a.x - vel_a.x + vel_a.x * toi,
            y: c_a.y - vel_a.y + vel_a.y * toi,
            z: c_a.z - vel_a.z + vel_a.z * toi,
        };
        let cb_hit = Vec3 {
            x: c_b.x - vel_b.x + vel_b.x * toi,
            y: c_b.y - vel_b.y + vel_b.y * toi,
            z: c_b.z - vel_b.z + vel_b.z * toi,
        };
        let nx = ca_hit.x - cb_hit.x;
        let ny = ca_hit.y - cb_hit.y;
        let nz = ca_hit.z - cb_hit.z;
        let nlen = (nx * nx + ny * ny + nz * nz).sqrt();
        if nlen < 1e-12 {
            return None;
        }
        let normal = Vec3 {
            x: nx / nlen,
            y: ny / nlen,
            z: nz / nlen,
        };
        let point = Vec3 {
            x: cb_hit.x + normal.x * r_b,
            y: cb_hit.y + normal.y * r_b,
            z: cb_hit.z + normal.z * r_b,
        };
        Some(ContactGeom {
            point,
            normal,
            depth: 0.0,
        })
    }

    fn swept_sphere_vs_rounded_obb(
        c_sphere: Vec3,
        r_sphere: f32,
        sphere_coll: &RcCollider,
        box_world: &Mat4,
        half: Vec3,
        box_r: f32,
        box_coll: &RcCollider,
    ) -> Option<ContactGeom> {
        let sphere_vel_rc = rc_ref!(sphere_coll).velocity.clone();
        let box_vel_rc = rc_ref!(box_coll).velocity.clone();
        let sphere_vel = *rc_ref!(&sphere_vel_rc);
        let box_vel = *rc_ref!(&box_vel_rc);
        let rel_vel = Vec3 {
            x: sphere_vel.x - box_vel.x,
            y: sphere_vel.y - box_vel.y,
            z: sphere_vel.z - box_vel.z,
        };
        let rel_len_sq = rel_vel.x * rel_vel.x + rel_vel.y * rel_vel.y + rel_vel.z * rel_vel.z;
        if rel_len_sq < 1e-12 {
            return None;
        }
        let inv = box_world.inverse_value();
        let current_local = inv.mul_vec_value(&c_sphere);
        let rel_local = inv.mul_dir_value(&rel_vel);
        let previous_local = Vec3 {
            x: current_local.x - rel_local.x,
            y: current_local.y - rel_local.y,
            z: current_local.z - rel_local.z,
        };
        let reach = r_sphere + box_r.max(0.0);
        let expanded = Aabb {
            min: Vec3 {
                x: -half.x - reach,
                y: -half.y - reach,
                z: -half.z - reach,
            },
            max: Vec3 {
                x: half.x + reach,
                y: half.y + reach,
                z: half.z + reach,
            },
        };
        let (toi, _, normal_local) = ray_vs_aabb(previous_local, rel_local, &expanded, 1.0)?;
        if toi <= 0.0 {
            return None;
        }
        let sphere_hit = Vec3 {
            x: c_sphere.x - sphere_vel.x + sphere_vel.x * toi,
            y: c_sphere.y - sphere_vel.y + sphere_vel.y * toi,
            z: c_sphere.z - sphere_vel.z + sphere_vel.z * toi,
        };
        let normal = box_world.mul_dir_value(&normal_local);
        let point = Vec3 {
            x: sphere_hit.x - normal.x * r_sphere,
            y: sphere_hit.y - normal.y * r_sphere,
            z: sphere_hit.z - normal.z * r_sphere,
        };
        Some(ContactGeom {
            point,
            normal,
            depth: 0.0,
        })
    }

    fn swept_sphere_vs_capsule(
        c_sphere: Vec3,
        r_sphere: f32,
        sphere_coll: &RcCollider,
        cap_world: &Mat4,
        half_h: f32,
        cap_r: f32,
        cap_coll: &RcCollider,
    ) -> Option<ContactGeom> {
        let sphere_vel_rc = rc_ref!(sphere_coll).velocity.clone();
        let cap_vel_rc = rc_ref!(cap_coll).velocity.clone();
        let sphere_vel = *rc_ref!(&sphere_vel_rc);
        let cap_vel = *rc_ref!(&cap_vel_rc);
        let rel_vel = Vec3 {
            x: sphere_vel.x - cap_vel.x,
            y: sphere_vel.y - cap_vel.y,
            z: sphere_vel.z - cap_vel.z,
        };
        let rel_len_sq = rel_vel.x * rel_vel.x + rel_vel.y * rel_vel.y + rel_vel.z * rel_vel.z;
        if rel_len_sq < 1e-12 {
            return None;
        }
        let previous = Vec3 {
            x: c_sphere.x - rel_vel.x,
            y: c_sphere.y - rel_vel.y,
            z: c_sphere.z - rel_vel.z,
        };
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
        let reach = r_sphere + cap_r.max(0.0);
        let mut best = swept_sphere_vs_capsule_axis(previous, rel_vel, reach, cap_r, top, bot);
        for end in [top, bot] {
            if let Some((toi, mut geom)) = swept_sphere_vs_point(previous, rel_vel, reach, end) {
                geom.point = Vec3 {
                    x: end.x + geom.normal.x * cap_r,
                    y: end.y + geom.normal.y * cap_r,
                    z: end.z + geom.normal.z * cap_r,
                };
                if best.as_ref().is_none_or(|(best_toi, _)| toi < *best_toi) {
                    best = Some((toi, geom));
                }
            }
        }
        best.map(|(_, geom)| geom)
    }

    fn swept_capsule_vs_capsule(
        world_a: &Mat4,
        half_h_a: f32,
        r_a: f32,
        coll_a: &RcCollider,
        world_b: &Mat4,
        half_h_b: f32,
        r_b: f32,
        coll_b: &RcCollider,
    ) -> Option<ContactGeom> {
        let top_a = world_a.mul_vec_value(&Vec3 {
            x: 0.0,
            y: half_h_a,
            z: 0.0,
        });
        let bot_a = world_a.mul_vec_value(&Vec3 {
            x: 0.0,
            y: -half_h_a,
            z: 0.0,
        });
        let top_b = world_b.mul_vec_value(&Vec3 {
            x: 0.0,
            y: half_h_b,
            z: 0.0,
        });
        let bot_b = world_b.mul_vec_value(&Vec3 {
            x: 0.0,
            y: -half_h_b,
            z: 0.0,
        });
        let vel_a_rc = rc_ref!(coll_a).velocity.clone();
        let vel_b_rc = rc_ref!(coll_b).velocity.clone();
        let vel_a = *rc_ref!(&vel_a_rc);
        let vel_b = *rc_ref!(&vel_b_rc);
        let rel_vel = Vec3 {
            x: vel_a.x - vel_b.x,
            y: vel_a.y - vel_b.y,
            z: vel_a.z - vel_b.z,
        };
        let prev_top_a = Vec3 {
            x: top_a.x - rel_vel.x,
            y: top_a.y - rel_vel.y,
            z: top_a.z - rel_vel.z,
        };
        let prev_bot_a = Vec3 {
            x: bot_a.x - rel_vel.x,
            y: bot_a.y - rel_vel.y,
            z: bot_a.z - rel_vel.z,
        };
        swept_segment_vs_segment(
            prev_top_a,
            prev_bot_a,
            rel_vel,
            r_a.max(0.0) + r_b.max(0.0),
            r_b.max(0.0),
            top_b,
            bot_b,
        )
        .map(|(_, geom)| geom)
    }

    fn swept_capsule_vs_rounded_obb(
        cap_world: &Mat4,
        half_h: f32,
        cap_r: f32,
        cap_coll: &RcCollider,
        box_world: &Mat4,
        half: Vec3,
        box_r: f32,
        box_coll: &RcCollider,
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
        let cap_vel_rc = rc_ref!(cap_coll).velocity.clone();
        let box_vel_rc = rc_ref!(box_coll).velocity.clone();
        let cap_vel = *rc_ref!(&cap_vel_rc);
        let box_vel = *rc_ref!(&box_vel_rc);
        let rel_vel = Vec3 {
            x: cap_vel.x - box_vel.x,
            y: cap_vel.y - box_vel.y,
            z: cap_vel.z - box_vel.z,
        };
        let inv = box_world.inverse_value();
        let prev_top = Vec3 {
            x: top.x - rel_vel.x,
            y: top.y - rel_vel.y,
            z: top.z - rel_vel.z,
        };
        let prev_bot = Vec3 {
            x: bot.x - rel_vel.x,
            y: bot.y - rel_vel.y,
            z: bot.z - rel_vel.z,
        };
        let prev_top_local = inv.mul_vec_value(&prev_top);
        let prev_bot_local = inv.mul_vec_value(&prev_bot);
        let rel_local = inv.mul_dir_value(&rel_vel);
        swept_segment_vs_aabb(
            prev_top_local,
            prev_bot_local,
            rel_local,
            cap_r.max(0.0) + box_r.max(0.0),
            box_r.max(0.0),
            half,
            box_world,
        )
        .map(|(_, geom)| geom)
    }

    fn swept_rounded_obb_vs_rounded_obb(
        world_a: &Mat4,
        half_a: Vec3,
        r_a: f32,
        coll_a: &RcCollider,
        world_b: &Mat4,
        half_b: Vec3,
        r_b: f32,
        coll_b: &RcCollider,
    ) -> Option<ContactGeom> {
        let vel_a_rc = rc_ref!(coll_a).velocity.clone();
        let vel_b_rc = rc_ref!(coll_b).velocity.clone();
        let vel_a = *rc_ref!(&vel_a_rc);
        let vel_b = *rc_ref!(&vel_b_rc);
        let rel_vel = Vec3 {
            x: vel_a.x - vel_b.x,
            y: vel_a.y - vel_b.y,
            z: vel_a.z - vel_b.z,
        };
        swept_obb_vs_obb(world_a, half_a, r_a, rel_vel, world_b, half_b, r_b)
    }

    fn swept_sphere_vs_mesh(
        world_mesh: &Mat4,
        mesh: &RcMesh,
        world_sphere: &Mat4,
        size_sphere: Vec3,
        r_sphere: f32,
        sphere_coll: &RcCollider,
        mesh_coll: &RcCollider,
    ) -> Option<ContactGeom> {
        let ColliderShape::Sphere { r } = classify_shape(size_sphere, r_sphere) else {
            return None;
        };
        let sphere_vel_rc = rc_ref!(sphere_coll).velocity.clone();
        let mesh_vel_rc = rc_ref!(mesh_coll).velocity.clone();
        let sphere_vel = *rc_ref!(&sphere_vel_rc);
        let mesh_vel = *rc_ref!(&mesh_vel_rc);
        let rel_vel = Vec3 {
            x: sphere_vel.x - mesh_vel.x,
            y: sphere_vel.y - mesh_vel.y,
            z: sphere_vel.z - mesh_vel.z,
        };
        let rel_len_sq = rel_vel.x * rel_vel.x + rel_vel.y * rel_vel.y + rel_vel.z * rel_vel.z;
        if rel_len_sq < 1e-12 {
            return None;
        }

        let current = world_sphere.pos_value();
        let previous = Vec3 {
            x: current.x - rel_vel.x,
            y: current.y - rel_vel.y,
            z: current.z - rel_vel.z,
        };
        let swept_world = Aabb {
            min: Vec3 {
                x: (current.x.min(previous.x)) - r,
                y: (current.y.min(previous.y)) - r,
                z: (current.z.min(previous.z)) - r,
            },
            max: Vec3 {
                x: (current.x.max(previous.x)) + r,
                y: (current.y.max(previous.y)) + r,
                z: (current.z.max(previous.z)) + r,
            },
        };
        let mesh_inv = world_mesh.inverse_value();
        let query_local = transform_aabb_to_local(&mesh_inv, &swept_world);
        let m = rc_ref!(mesh);
        let mut best: Option<(f32, ContactGeom)> = None;
        m.with_collision_bvh(|bvh| {
            bvh.query_aabb(&query_local, |tri| {
                let v0 = mul_point(world_mesh, bvh.positions[tri[0] as usize]);
                let v1 = mul_point(world_mesh, bvh.positions[tri[1] as usize]);
                let v2 = mul_point(world_mesh, bvh.positions[tri[2] as usize]);
                let Some((toi, geom)) = swept_sphere_vs_triangle(previous, rel_vel, r, v0, v1, v2)
                else {
                    return;
                };
                match best {
                    None => best = Some((toi, geom)),
                    Some((prev_toi, _)) if toi < prev_toi => best = Some((toi, geom)),
                    _ => {}
                }
            });
        });
        best.map(|(_, geom)| geom)
    }

    fn swept_capsule_vs_mesh(
        world_mesh: &Mat4,
        mesh: &RcMesh,
        world_capsule: &Mat4,
        size_capsule: Vec3,
        r_capsule: f32,
        capsule_coll: &RcCollider,
        mesh_coll: &RcCollider,
    ) -> Option<ContactGeom> {
        let ColliderShape::Capsule { half_h, r } = classify_shape(size_capsule, r_capsule) else {
            return None;
        };
        let capsule_vel_rc = rc_ref!(capsule_coll).velocity.clone();
        let mesh_vel_rc = rc_ref!(mesh_coll).velocity.clone();
        let capsule_vel = *rc_ref!(&capsule_vel_rc);
        let mesh_vel = *rc_ref!(&mesh_vel_rc);
        let rel_vel = Vec3 {
            x: capsule_vel.x - mesh_vel.x,
            y: capsule_vel.y - mesh_vel.y,
            z: capsule_vel.z - mesh_vel.z,
        };
        let rel_len_sq = rel_vel.x * rel_vel.x + rel_vel.y * rel_vel.y + rel_vel.z * rel_vel.z;
        if rel_len_sq < 1e-12 {
            return None;
        }

        let top = world_capsule.mul_vec_value(&Vec3 {
            x: 0.0,
            y: half_h,
            z: 0.0,
        });
        let bot = world_capsule.mul_vec_value(&Vec3 {
            x: 0.0,
            y: -half_h,
            z: 0.0,
        });
        let prev_top = Vec3 {
            x: top.x - rel_vel.x,
            y: top.y - rel_vel.y,
            z: top.z - rel_vel.z,
        };
        let prev_bot = Vec3 {
            x: bot.x - rel_vel.x,
            y: bot.y - rel_vel.y,
            z: bot.z - rel_vel.z,
        };
        let swept_world = Aabb {
            min: Vec3 {
                x: top.x.min(bot.x).min(prev_top.x).min(prev_bot.x) - r,
                y: top.y.min(bot.y).min(prev_top.y).min(prev_bot.y) - r,
                z: top.z.min(bot.z).min(prev_top.z).min(prev_bot.z) - r,
            },
            max: Vec3 {
                x: top.x.max(bot.x).max(prev_top.x).max(prev_bot.x) + r,
                y: top.y.max(bot.y).max(prev_top.y).max(prev_bot.y) + r,
                z: top.z.max(bot.z).max(prev_top.z).max(prev_bot.z) + r,
            },
        };
        let mesh_inv = world_mesh.inverse_value();
        let query_local = transform_aabb_to_local(&mesh_inv, &swept_world);
        let m = rc_ref!(mesh);
        let mut best: Option<(f32, ContactGeom)> = None;
        m.with_collision_bvh(|bvh| {
            bvh.query_aabb(&query_local, |tri| {
                let v0 = mul_point(world_mesh, bvh.positions[tri[0] as usize]);
                let v1 = mul_point(world_mesh, bvh.positions[tri[1] as usize]);
                let v2 = mul_point(world_mesh, bvh.positions[tri[2] as usize]);
                let Some((toi, geom)) =
                    swept_segment_vs_triangle(prev_top, prev_bot, rel_vel, r, v0, v1, v2)
                else {
                    return;
                };
                match best {
                    None => best = Some((toi, geom)),
                    Some((prev_toi, _)) if toi < prev_toi => best = Some((toi, geom)),
                    _ => {}
                }
            });
        });
        best.map(|(_, geom)| geom)
    }

    fn swept_rounded_obb_vs_mesh(
        world_mesh: &Mat4,
        mesh: &RcMesh,
        world_box: &Mat4,
        size_box: Vec3,
        r_box: f32,
        box_coll: &RcCollider,
        mesh_coll: &RcCollider,
    ) -> Option<ContactGeom> {
        let ColliderShape::RoundedBox { half, r } = classify_shape(size_box, r_box) else {
            return None;
        };
        let box_vel_rc = rc_ref!(box_coll).velocity.clone();
        let mesh_vel_rc = rc_ref!(mesh_coll).velocity.clone();
        let box_vel = *rc_ref!(&box_vel_rc);
        let mesh_vel = *rc_ref!(&mesh_vel_rc);
        let rel_vel = Vec3 {
            x: box_vel.x - mesh_vel.x,
            y: box_vel.y - mesh_vel.y,
            z: box_vel.z - mesh_vel.z,
        };
        let rel_len_sq = rel_vel.x * rel_vel.x + rel_vel.y * rel_vel.y + rel_vel.z * rel_vel.z;
        if rel_len_sq < 1e-12 {
            return None;
        }
        let corners = rounded_obb_corners(world_box, half);
        let swept_world = swept_points_aabb(&corners, rel_vel, r);
        let mesh_inv = world_mesh.inverse_value();
        let query_local = transform_aabb_to_local(&mesh_inv, &swept_world);
        let m = rc_ref!(mesh);
        let mut best: Option<(f32, ContactGeom)> = None;
        m.with_collision_bvh(|bvh| {
            bvh.query_aabb(&query_local, |tri| {
                let v0 = mul_point(world_mesh, bvh.positions[tri[0] as usize]);
                let v1 = mul_point(world_mesh, bvh.positions[tri[1] as usize]);
                let v2 = mul_point(world_mesh, bvh.positions[tri[2] as usize]);
                let Some((toi, geom)) =
                    swept_obb_vs_triangle(world_box, half, r, rel_vel, v0, v1, v2)
                else {
                    return;
                };
                match best {
                    None => best = Some((toi, geom)),
                    Some((prev_toi, _)) if toi < prev_toi => best = Some((toi, geom)),
                    _ => {}
                }
            });
        });
        best.map(|(_, geom)| geom)
    }

    fn build_contact_pair(
        node_a: &RcNode,
        coll_a: &RcCollider,
        node_b: &RcNode,
        coll_b: &RcCollider,
        geom: ContactGeom,
    ) -> ContactPair {
        let a = rc_ref!(coll_a);
        let b = rc_ref!(coll_b);
        // Mesh colliders are always immovable terrain (cube-design.md
        // § 11.1); ignore any user-set mass on the mesh side.
        let mass_a = if a.mesh.is_some() { 0.0 } else { a.mass };
        let mass_b = if b.mesh.is_some() { 0.0 } else { b.mass };
        let trigger_a = a.trigger;
        let trigger_b = b.trigger;
        let rolls_a = a.rolls;
        let rolls_b = b.rolls;
        let resti = a.restitution.max(b.restitution);
        let frict = (a.friction + b.friction) * 0.5;
        let vel_a = *rc_ref!(&a.velocity);
        let vel_b = *rc_ref!(&b.velocity);

        let (share_a, share_b) = if mass_a == 0.0 && mass_b == 0.0 {
            (0.0, 0.0)
        } else if mass_a == 0.0 {
            (0.0, 1.0)
        } else if mass_b == 0.0 {
            (1.0, 0.0)
        } else {
            let total = mass_a + mass_b;
            (mass_b / total, mass_a / total)
        };

        let zero = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let trigger_pair = trigger_a || trigger_b;
        let (depth_a, dv_a, dav_a) = if trigger_pair {
            (0.0, zero, zero)
        } else {
            Self::compute_deltas(geom, share_a, vel_a, vel_b, resti, frict, rolls_a, true)
        };
        let (depth_b, dv_b, dav_b) = if trigger_pair {
            (0.0, zero, zero)
        } else {
            Self::compute_deltas(geom, share_b, vel_b, vel_a, resti, frict, rolls_b, false)
        };

        let contact_a = Contact::new();
        {
            let c = rc_mut!(&contact_a);
            c.point = Vec3::new(geom.point.x, geom.point.y, geom.point.z);
            c.normal = Vec3::new(geom.normal.x, geom.normal.y, geom.normal.z);
            c.depth = depth_a;
            c.delta_velocity = Vec3::new(dv_a.x, dv_a.y, dv_a.z);
            c.delta_angular_velocity = Vec3::new(dav_a.x, dav_a.y, dav_a.z);
            c.delta_rotation = Quat::identity();
        }
        let contact_b = Contact::new();
        {
            let c = rc_mut!(&contact_b);
            c.point = Vec3::new(geom.point.x, geom.point.y, geom.point.z);
            c.normal = Vec3::new(-geom.normal.x, -geom.normal.y, -geom.normal.z);
            c.depth = depth_b;
            c.delta_velocity = Vec3::new(dv_b.x, dv_b.y, dv_b.z);
            c.delta_angular_velocity = Vec3::new(dav_b.x, dav_b.y, dav_b.z);
            c.delta_rotation = Quat::identity();
        }
        ContactPair {
            node_a: node_a.clone(),
            node_b: node_b.clone(),
            contact_a,
            contact_b,
        }
    }

    fn compute_deltas(
        geom: ContactGeom,
        share: f32,
        my_vel: Vec3,
        other_vel: Vec3,
        resti: f32,
        frict: f32,
        rolls: bool,
        i_am_a: bool,
    ) -> (f32, Vec3, Vec3) {
        let zero = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        if share == 0.0 {
            return (0.0, zero, zero);
        }
        let depth = geom.depth * share;
        let rel = Vec3 {
            x: my_vel.x - other_vel.x,
            y: my_vel.y - other_vel.y,
            z: my_vel.z - other_vel.z,
        };
        let normal_sign = if i_am_a { 1.0 } else { -1.0 };
        let n = Vec3 {
            x: geom.normal.x * normal_sign,
            y: geom.normal.y * normal_sign,
            z: geom.normal.z * normal_sign,
        };
        let v_n = rel.x * n.x + rel.y * n.y + rel.z * n.z;
        let impulse_n = if v_n < 0.0 {
            -(1.0 + resti) * v_n * share
        } else {
            0.0
        };
        let tan_x = rel.x - v_n * n.x;
        let tan_y = rel.y - v_n * n.y;
        let tan_z = rel.z - v_n * n.z;
        let frict_share = frict.clamp(0.0, 1.0) * share;
        let dv = Vec3 {
            x: n.x * impulse_n - tan_x * frict_share,
            y: n.y * impulse_n - tan_y * frict_share,
            z: n.z * impulse_n - tan_z * frict_share,
        };
        let dav = if rolls {
            Vec3 {
                x: tan_y * n.z - tan_z * n.y,
                y: tan_z * n.x - tan_x * n.z,
                z: tan_x * n.y - tan_y * n.x,
            }
        } else {
            zero
        };
        (depth, dv, dav)
    }

    // Spatial queries (§ 15.4).

    pub fn raycast(
        scene_root: &RcNode,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        hit_triggers: bool,
        tags_filter: Option<&[String]>,
    ) -> Option<RaycastHitInfo> {
        let direction = Self::normalize_ray_direction(direction)?;
        let mut entries: Vec<(RcNode, Mat4, Aabb)> = Vec::new();
        Self::collect_collider_entries(scene_root, &mut entries, false);
        let mut best: Option<RaycastHitInfo> = None;
        for (node, world, aabb) in &entries {
            let Some(hit) = Self::ray_test_one(
                node,
                world,
                aabb,
                origin,
                direction,
                max_distance,
                hit_triggers,
                tags_filter,
            ) else {
                continue;
            };
            if best.as_ref().is_none_or(|b| hit.distance < b.distance) {
                best = Some(hit);
            }
        }
        best
    }

    pub fn raycast_all(
        scene_root: &RcNode,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        hit_triggers: bool,
        tags_filter: Option<&[String]>,
    ) -> Vec<RaycastHitInfo> {
        let Some(direction) = Self::normalize_ray_direction(direction) else {
            return Vec::new();
        };
        let mut entries: Vec<(RcNode, Mat4, Aabb)> = Vec::new();
        Self::collect_collider_entries(scene_root, &mut entries, false);
        let mut hits: Vec<RaycastHitInfo> = Vec::new();
        for (node, world, aabb) in &entries {
            if let Some(hit) = Self::ray_test_one(
                node,
                world,
                aabb,
                origin,
                direction,
                max_distance,
                hit_triggers,
                tags_filter,
            ) {
                hits.push(hit);
            }
        }
        hits.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits
    }

    fn normalize_ray_direction(direction: Vec3) -> Option<Vec3> {
        let len_sq =
            direction.x * direction.x + direction.y * direction.y + direction.z * direction.z;
        if len_sq < 1e-12 {
            return None;
        }
        let inv_len = 1.0 / len_sq.sqrt();
        Some(Vec3 {
            x: direction.x * inv_len,
            y: direction.y * inv_len,
            z: direction.z * inv_len,
        })
    }

    fn ray_test_one(
        node: &RcNode,
        world: &Mat4,
        aabb: &Aabb,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        hit_triggers: bool,
        tags_filter: Option<&[String]>,
    ) -> Option<RaycastHitInfo> {
        let coll_rc = rc_ref!(node).collider.clone()?;
        let coll = rc_ref!(&coll_rc);
        if !hit_triggers && coll.trigger {
            return None;
        }
        let size = *rc_ref!(&coll.size);
        let radius = coll.radius.max(0.0);
        let has_mesh = coll.mesh.is_some();
        if !Self::tags_match(node, tags_filter) {
            return None;
        }
        if !has_mesh && ray_vs_aabb(origin, direction, aabb, max_distance).is_none() {
            return None;
        }
        let hit = if has_mesh {
            Self::ray_vs_mesh_collider(origin, direction, world, &coll_rc, max_distance)
        } else {
            match classify_shape(size, radius) {
                ColliderShape::Sphere { r } => {
                    ray_vs_sphere(origin, direction, world.pos_value(), r, max_distance)
                }
                ColliderShape::Capsule { half_h, r } => {
                    Self::ray_vs_capsule_collider(origin, direction, world, half_h, r, max_distance)
                }
                ColliderShape::RoundedBox { half, r } => Self::ray_vs_rounded_obb_collider(
                    origin,
                    direction,
                    world,
                    half,
                    r,
                    max_distance,
                ),
            }
        };
        let (t, point, normal) = hit?;
        Some(RaycastHitInfo {
            node: node.clone(),
            point,
            normal,
            distance: t,
        })
    }

    pub fn overlap_sphere(
        scene_root: &RcNode,
        center: Vec3,
        radius: f32,
        hit_triggers: bool,
        tags_filter: Option<&[String]>,
    ) -> Vec<RcNode> {
        let mut entries: Vec<(RcNode, Mat4, Aabb)> = Vec::new();
        Self::collect_collider_entries(scene_root, &mut entries, false);
        let radius = radius.max(0.0);
        let probe = Aabb::from_sphere(center, radius);
        let mut out: Vec<RcNode> = Vec::new();
        for (node, world, aabb) in &entries {
            let coll_rc_opt = rc_ref!(node).collider.clone();
            let Some(coll_rc) = coll_rc_opt else {
                continue;
            };
            let coll = rc_ref!(&coll_rc);
            if !hit_triggers && coll.trigger {
                continue;
            }
            let size = *rc_ref!(&coll.size);
            let r_other = coll.radius.max(0.0);
            let has_mesh = coll.mesh.is_some();
            if !Self::tags_match(node, tags_filter) {
                continue;
            }
            if !probe.overlaps(aabb) {
                continue;
            }
            let hit = if has_mesh {
                Self::mesh_overlaps_sphere(world, coll.mesh.as_ref().unwrap(), center, radius)
            } else {
                match classify_shape(size, r_other) {
                    ColliderShape::Sphere { r } => {
                        sphere_vs_sphere(center, radius, world.pos_value(), r).is_some()
                    }
                    ColliderShape::Capsule { half_h, r } => {
                        capsule_vs_sphere(world, half_h, r, center, radius).is_some()
                    }
                    ColliderShape::RoundedBox { half, r } => {
                        sphere_vs_rounded_obb(center, radius, world, half, r).is_some()
                    }
                }
            };
            if hit {
                out.push(node.clone());
            }
        }
        out
    }

    pub fn overlap_box(
        scene_root: &RcNode,
        transform: &Mat4,
        size: Vec3,
        hit_triggers: bool,
        tags_filter: Option<&[String]>,
    ) -> Vec<RcNode> {
        let mut entries: Vec<(RcNode, Mat4, Aabb)> = Vec::new();
        Self::collect_collider_entries(scene_root, &mut entries, false);
        let probe = Aabb::from_rounded_box(transform, size, 0.0);
        let probe_half = Vec3 {
            x: size.x.abs() * 0.5,
            y: size.y.abs() * 0.5,
            z: size.z.abs() * 0.5,
        };
        let mut out: Vec<RcNode> = Vec::new();
        for (node, world, aabb) in &entries {
            let coll_rc_opt = rc_ref!(node).collider.clone();
            let Some(coll_rc) = coll_rc_opt else {
                continue;
            };
            let coll = rc_ref!(&coll_rc);
            if !hit_triggers && coll.trigger {
                continue;
            }
            let other_size = *rc_ref!(&coll.size);
            let r_other = coll.radius.max(0.0);
            let has_mesh = coll.mesh.is_some();
            if !Self::tags_match(node, tags_filter) {
                continue;
            }
            if !probe.overlaps(aabb) {
                continue;
            }
            let hit = if has_mesh {
                Self::mesh_overlaps_box(world, coll.mesh.as_ref().unwrap(), transform, size)
            } else {
                match classify_shape(other_size, r_other) {
                    ColliderShape::Sphere { r } => {
                        sphere_vs_rounded_obb(world.pos_value(), r, transform, probe_half, 0.0)
                            .is_some()
                    }
                    ColliderShape::Capsule { half_h, r } => {
                        capsule_vs_rounded_obb(world, half_h, r, transform, probe_half, 0.0)
                            .is_some()
                    }
                    ColliderShape::RoundedBox { half, r } => {
                        rounded_obb_vs_rounded_obb(transform, probe_half, 0.0, world, half, r)
                            .is_some()
                    }
                }
            };
            if hit {
                out.push(node.clone());
            }
        }
        out
    }

    fn mesh_overlaps_sphere(world_mesh: &Mat4, mesh: &RcMesh, center: Vec3, radius: f32) -> bool {
        let mut probe = Mat4::identity_value();
        probe.data[0][3] = center.x;
        probe.data[1][3] = center.y;
        probe.data[2][3] = center.z;
        narrow_phase_mesh_vs_dynamic(
            world_mesh,
            mesh,
            &probe,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius,
        )
        .is_some()
    }

    fn mesh_overlaps_box(world_mesh: &Mat4, mesh: &RcMesh, transform: &Mat4, size: Vec3) -> bool {
        narrow_phase_mesh_vs_dynamic(world_mesh, mesh, transform, size, 0.0).is_some()
    }

    fn tags_match(node: &RcNode, filter: Option<&[String]>) -> bool {
        let Some(tags) = filter else {
            return true;
        };
        let node_tags = rc_ref!(node).tags.clone();
        tags.iter().any(|t| node_tags.iter().any(|nt| nt == t))
    }

    // Mesh raycast through the collision BVH. The ray is mapped into
    // mesh-local space (where the BVH lives) for pruning only; the hit
    // test runs in world space on the world-lifted triangle, matching
    // the mesh narrow phase. The affine map preserves the ray's t
    // parameterization, so pruning with `max_distance` stays valid.
    fn ray_vs_mesh_collider(
        origin: Vec3,
        direction: Vec3,
        world: &Mat4,
        coll: &RcCollider,
        max_distance: f32,
    ) -> Option<(f32, Vec3, Vec3)> {
        let mesh_rc = rc_ref!(coll).mesh.clone()?;
        let mesh = rc_ref!(&mesh_rc);
        let inv = world.inverse_value();
        let local_origin = inv.mul_vec_value(&origin);
        let local_direction = inv.mul_dir_value(&direction);
        let mut best: Option<(f32, Vec3, Vec3)> = None;
        mesh.with_collision_bvh(|bvh| {
            bvh.query_ray(local_origin, local_direction, max_distance, |tri| {
                let v0 = world.mul_vec_value(&bvh.positions[tri[0] as usize]);
                let v1 = world.mul_vec_value(&bvh.positions[tri[1] as usize]);
                let v2 = world.mul_vec_value(&bvh.positions[tri[2] as usize]);
                let cur_max = best.as_ref().map_or(max_distance, |(t, _, _)| *t);
                if let Some(hit) = ray_vs_triangle(origin, direction, v0, v1, v2, cur_max) {
                    best = Some(hit);
                }
            });
        });
        best
    }

    fn ray_vs_capsule_collider(
        origin: Vec3,
        direction: Vec3,
        world: &Mat4,
        half_h: f32,
        radius: f32,
        max_distance: f32,
    ) -> Option<(f32, Vec3, Vec3)> {
        let inv = world.inverse_value();
        let local_origin = inv.mul_vec_value(&origin);
        let local_direction = inv.mul_dir_value(&direction);
        let r = radius.max(0.0);
        let mut best: Option<(f32, Vec3, Vec3)> = None;
        for center in [
            Vec3 {
                x: 0.0,
                y: half_h,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: -half_h,
                z: 0.0,
            },
        ] {
            if let Some((t, point, normal)) =
                ray_vs_sphere(local_origin, local_direction, center, r, max_distance)
            {
                let hit = (t, world.mul_vec_value(&point), world.mul_dir_value(&normal));
                if best.as_ref().is_none_or(|(best_t, _, _)| t < *best_t) {
                    best = Some(hit);
                }
            }
        }
        let a = local_direction.x * local_direction.x + local_direction.z * local_direction.z;
        if a > 1e-12 {
            let b = 2.0 * (local_origin.x * local_direction.x + local_origin.z * local_direction.z);
            let c = local_origin.x * local_origin.x + local_origin.z * local_origin.z - r * r;
            let disc = b * b - 4.0 * a * c;
            if disc >= 0.0 {
                let sqrt_disc = disc.sqrt();
                for t in [(-b - sqrt_disc) / (2.0 * a), (-b + sqrt_disc) / (2.0 * a)] {
                    if !(0.0..=max_distance).contains(&t) {
                        continue;
                    }
                    let local_point = Vec3 {
                        x: local_origin.x + local_direction.x * t,
                        y: local_origin.y + local_direction.y * t,
                        z: local_origin.z + local_direction.z * t,
                    };
                    if local_point.y < -half_h || local_point.y > half_h {
                        continue;
                    }
                    let normal_len =
                        (local_point.x * local_point.x + local_point.z * local_point.z).sqrt();
                    if normal_len < 1e-12 {
                        continue;
                    }
                    let local_normal = Vec3 {
                        x: local_point.x / normal_len,
                        y: 0.0,
                        z: local_point.z / normal_len,
                    };
                    let hit = (
                        t,
                        world.mul_vec_value(&local_point),
                        world.mul_dir_value(&local_normal),
                    );
                    if best.as_ref().is_none_or(|(best_t, _, _)| t < *best_t) {
                        best = Some(hit);
                    }
                }
            }
        }
        best
    }

    fn ray_vs_rounded_obb_collider(
        origin: Vec3,
        direction: Vec3,
        world: &Mat4,
        half: Vec3,
        radius: f32,
        max_distance: f32,
    ) -> Option<(f32, Vec3, Vec3)> {
        let inv = world.inverse_value();
        let local_origin = inv.mul_vec_value(&origin);
        let local_direction = inv.mul_dir_value(&direction);
        let (t, local_point, local_normal) =
            ray_vs_rounded_box(local_origin, local_direction, half, radius, max_distance)?;
        Some((
            t,
            world.mul_vec_value(&local_point),
            world.mul_dir_value(&local_normal),
        ))
    }
}

fn ray_vs_rounded_box(
    origin: Vec3,
    direction: Vec3,
    half: Vec3,
    radius: f32,
    max_distance: f32,
) -> Option<(f32, Vec3, Vec3)> {
    let r = radius.max(0.0);
    if r <= 0.0 {
        let local_aabb = Aabb {
            min: Vec3 {
                x: -half.x,
                y: -half.y,
                z: -half.z,
            },
            max: Vec3 {
                x: half.x,
                y: half.y,
                z: half.z,
            },
        };
        return ray_vs_aabb(origin, direction, &local_aabb, max_distance);
    }
    let mut best: Option<(f32, Vec3, Vec3)> = None;
    for axis in 0..3 {
        for sign in [-1.0, 1.0] {
            if let Some(hit) =
                ray_vs_rounded_box_face(origin, direction, half, r, axis, sign, max_distance)
            {
                set_nearer_hit(&mut best, hit);
            }
        }
    }
    for axis in 0..3 {
        let other0 = (axis + 1) % 3;
        let other1 = (axis + 2) % 3;
        for sign0 in [-1.0, 1.0] {
            for sign1 in [-1.0, 1.0] {
                let mut a = Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let mut b = a;
                set_axis(&mut a, axis, -component(half, axis));
                set_axis(&mut b, axis, component(half, axis));
                set_axis(&mut a, other0, sign0 * component(half, other0));
                set_axis(&mut b, other0, sign0 * component(half, other0));
                set_axis(&mut a, other1, sign1 * component(half, other1));
                set_axis(&mut b, other1, sign1 * component(half, other1));
                if let Some(hit) = ray_vs_segment_capsule(origin, direction, a, b, r, max_distance)
                {
                    set_nearer_hit(&mut best, hit);
                }
            }
        }
    }
    for x in [-half.x, half.x] {
        for y in [-half.y, half.y] {
            for z in [-half.z, half.z] {
                let center = Vec3 { x, y, z };
                if let Some(hit) = ray_vs_sphere(origin, direction, center, r, max_distance) {
                    set_nearer_hit(&mut best, hit);
                }
            }
        }
    }
    best
}

fn ray_vs_rounded_box_face(
    origin: Vec3,
    direction: Vec3,
    half: Vec3,
    radius: f32,
    axis: usize,
    sign: f32,
    max_distance: f32,
) -> Option<(f32, Vec3, Vec3)> {
    let denom = component(direction, axis);
    if denom.abs() < 1e-12 {
        return None;
    }
    let plane = sign * (component(half, axis) + radius);
    let t = (plane - component(origin, axis)) / denom;
    if !(0.0..=max_distance).contains(&t) {
        return None;
    }
    let point = vec_add(origin, vec_mul(direction, t));
    for other in 0..3 {
        if other != axis && component(point, other).abs() > component(half, other) + 1e-6 {
            return None;
        }
    }
    Some((t, point, axis_normal(axis, sign)))
}

fn ray_vs_segment_capsule(
    origin: Vec3,
    direction: Vec3,
    a: Vec3,
    b: Vec3,
    radius: f32,
    max_distance: f32,
) -> Option<(f32, Vec3, Vec3)> {
    let mut best: Option<(f32, Vec3, Vec3)> = None;
    if let Some(hit) = ray_vs_sphere(origin, direction, a, radius, max_distance) {
        set_nearer_hit(&mut best, hit);
    }
    if let Some(hit) = ray_vs_sphere(origin, direction, b, radius, max_distance) {
        set_nearer_hit(&mut best, hit);
    }
    let axis = vec_sub(b, a);
    let len_sq = vec_len_sq(axis);
    if len_sq < 1e-12 {
        return best;
    }
    let len = len_sq.sqrt();
    let u = vec_mul(axis, 1.0 / len);
    let rel = vec_sub(origin, a);
    let s0 = vec_dot(rel, u);
    let sv = vec_dot(direction, u);
    let q0 = vec_sub(rel, vec_mul(u, s0));
    let qv = vec_sub(direction, vec_mul(u, sv));
    let qa = vec_len_sq(qv);
    if qa < 1e-12 {
        return best;
    }
    let qb = 2.0 * vec_dot(q0, qv);
    let qc = vec_len_sq(q0) - radius * radius;
    let disc = qb * qb - 4.0 * qa * qc;
    if disc < 0.0 {
        return best;
    }
    let sqrt_disc = disc.sqrt();
    for t in [
        (-qb - sqrt_disc) / (2.0 * qa),
        (-qb + sqrt_disc) / (2.0 * qa),
    ] {
        if !(0.0..=max_distance).contains(&t) {
            continue;
        }
        let s = s0 + sv * t;
        if s < -1e-5 || s > len + 1e-5 {
            continue;
        }
        let point = vec_add(origin, vec_mul(direction, t));
        let axis_point = vec_add(a, vec_mul(u, s.clamp(0.0, len)));
        let Some(normal) = normalize_axis(vec_sub(point, axis_point)) else {
            continue;
        };
        set_nearer_hit(&mut best, (t, point, normal));
    }
    best
}

fn set_nearer_hit(best: &mut Option<(f32, Vec3, Vec3)>, hit: (f32, Vec3, Vec3)) {
    if best.as_ref().is_none_or(|(best_t, _, _)| hit.0 < *best_t) {
        *best = Some(hit);
    }
}

// Mesh-vs-dynamic narrow phase. `world_mesh` is the mesh collider's
// world transform; `mesh` is the static terrain. `world_dyn` is the
// dynamic body's world transform; `size_dyn` / `r_dyn` describe its
// rounded-box family. Returns ContactGeom with normal pointing FROM
// the mesh TOWARD the dynamic body.
fn narrow_phase_mesh_vs_dynamic(
    world_mesh: &Mat4,
    mesh: &RcMesh,
    world_dyn: &Mat4,
    size_dyn: Vec3,
    r_dyn: f32,
) -> Option<ContactGeom> {
    let shape_dyn = classify_shape(size_dyn, r_dyn);
    let mesh_inv = world_mesh.inverse_value();
    // Dynamic body's AABB in world space, then mapped into the mesh-
    // local frame where the BVH lives. The AABB stays a broad filter;
    // the per-triangle test below is shape-exact.
    let dyn_aabb_world = if matches!(shape_dyn, ColliderShape::Sphere { .. }) {
        Aabb::from_sphere(world_dyn.pos_value(), r_dyn)
    } else {
        Aabb::from_rounded_box(world_dyn, size_dyn, r_dyn)
    };
    let query_local = transform_aabb_to_local(&mesh_inv, &dyn_aabb_world);
    // Body-local inverse (rounded-box arm) and world center (sphere arm),
    // computed once outside the per-triangle callback.
    let dyn_inv = world_dyn.inverse_value();
    let dyn_center = world_dyn.pos_value();
    let m = rc_ref!(mesh);
    let mut best: Option<ContactGeom> = None;
    m.with_collision_bvh(|bvh| {
        bvh.query_aabb(&query_local, |tri| {
            let v0_local = bvh.positions[tri[0] as usize];
            let v1_local = bvh.positions[tri[1] as usize];
            let v2_local = bvh.positions[tri[2] as usize];
            // Lift the triangle into world space for the actual hit
            // test.
            let v0 = mul_point(world_mesh, v0_local);
            let v1 = mul_point(world_mesh, v1_local);
            let v2 = mul_point(world_mesh, v2_local);
            let hit = match shape_dyn {
                ColliderShape::Sphere { r } => sphere_vs_triangle(dyn_center, r, v0, v1, v2),
                ColliderShape::Capsule { half_h, r } => {
                    capsule_vs_triangle(world_dyn, half_h, r, v0, v1, v2)
                }
                ColliderShape::RoundedBox { half, r } => {
                    // Solve in the body-local frame where the box is
                    // axis-aligned, then map the contact back to world
                    // (rotation + translation only, so depth carries).
                    let l0 = mul_point(&dyn_inv, v0);
                    let l1 = mul_point(&dyn_inv, v1);
                    let l2 = mul_point(&dyn_inv, v2);
                    local_box_vs_triangle(half, r, l0, l1, l2).map(|g| ContactGeom {
                        point: mul_point(world_dyn, g.point),
                        normal: world_dyn.mul_dir_value(&g.normal),
                        depth: g.depth,
                    })
                }
            };
            if let Some(h) = hit {
                match best {
                    None => best = Some(h),
                    Some(prev) if h.depth > prev.depth => best = Some(h),
                    _ => {}
                }
            }
        });
    });
    best
}

fn mul_point(m: &Mat4, v: Vec3) -> Vec3 {
    m.mul_vec_value(&v)
}

fn transform_aabb_to_local(inv: &Mat4, aabb: &Aabb) -> Aabb {
    let corners = [
        Vec3 {
            x: aabb.min.x,
            y: aabb.min.y,
            z: aabb.min.z,
        },
        Vec3 {
            x: aabb.max.x,
            y: aabb.min.y,
            z: aabb.min.z,
        },
        Vec3 {
            x: aabb.min.x,
            y: aabb.max.y,
            z: aabb.min.z,
        },
        Vec3 {
            x: aabb.max.x,
            y: aabb.max.y,
            z: aabb.min.z,
        },
        Vec3 {
            x: aabb.min.x,
            y: aabb.min.y,
            z: aabb.max.z,
        },
        Vec3 {
            x: aabb.max.x,
            y: aabb.min.y,
            z: aabb.max.z,
        },
        Vec3 {
            x: aabb.min.x,
            y: aabb.max.y,
            z: aabb.max.z,
        },
        Vec3 {
            x: aabb.max.x,
            y: aabb.max.y,
            z: aabb.max.z,
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
        let local = mul_point(inv, *c);
        min.x = min.x.min(local.x);
        min.y = min.y.min(local.y);
        min.z = min.z.min(local.z);
        max.x = max.x.max(local.x);
        max.y = max.y.max(local.y);
        max.z = max.z.max(local.z);
    }
    Aabb { min, max }
}

fn rounded_obb_corners(world: &Mat4, half: Vec3) -> [Vec3; 8] {
    [
        world.mul_vec_value(&Vec3 {
            x: -half.x,
            y: -half.y,
            z: -half.z,
        }),
        world.mul_vec_value(&Vec3 {
            x: half.x,
            y: -half.y,
            z: -half.z,
        }),
        world.mul_vec_value(&Vec3 {
            x: -half.x,
            y: half.y,
            z: -half.z,
        }),
        world.mul_vec_value(&Vec3 {
            x: half.x,
            y: half.y,
            z: -half.z,
        }),
        world.mul_vec_value(&Vec3 {
            x: -half.x,
            y: -half.y,
            z: half.z,
        }),
        world.mul_vec_value(&Vec3 {
            x: half.x,
            y: -half.y,
            z: half.z,
        }),
        world.mul_vec_value(&Vec3 {
            x: -half.x,
            y: half.y,
            z: half.z,
        }),
        world.mul_vec_value(&Vec3 {
            x: half.x,
            y: half.y,
            z: half.z,
        }),
    ]
}

fn swept_points_aabb(points: &[Vec3], velocity: Vec3, radius: f32) -> Aabb {
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
    let r = radius.max(0.0);
    for current in points {
        let previous = Vec3 {
            x: current.x - velocity.x,
            y: current.y - velocity.y,
            z: current.z - velocity.z,
        };
        for point in [*current, previous] {
            min.x = min.x.min(point.x - r);
            min.y = min.y.min(point.y - r);
            min.z = min.z.min(point.z - r);
            max.x = max.x.max(point.x + r);
            max.y = max.y.max(point.y + r);
            max.z = max.z.max(point.z + r);
        }
    }
    Aabb { min, max }
}

fn swept_segment_vs_segment(
    prev_a0: Vec3,
    prev_a1: Vec3,
    velocity: Vec3,
    reach: f32,
    target_radius: f32,
    b0: Vec3,
    b1: Vec3,
) -> Option<(f32, ContactGeom)> {
    if vec_len_sq(velocity) < 1e-12 {
        return None;
    }
    let mut lower = 0.0;
    let mut t = 0.0;
    for _ in 0..24 {
        let a0 = vec_add(prev_a0, vec_mul(velocity, t));
        let a1 = vec_add(prev_a1, vec_mul(velocity, t));
        let (on_a, on_b) = closest_points_segment_segment(a0, a1, b0, b1);
        let delta = vec_sub(on_a, on_b);
        let dist = vec_len(delta);
        if dist <= reach {
            return refine_swept_segment_vs_segment(
                prev_a0,
                prev_a1,
                velocity,
                reach,
                target_radius,
                b0,
                b1,
                lower,
                t,
            );
        }
        let normal = normalized_or(delta, vec_mul(velocity, -1.0))?;
        let closing = -vec_dot(velocity, normal);
        if closing <= 1e-6 {
            return None;
        }
        lower = t;
        t += ((dist - reach) / closing).max(1e-5);
        if t > 1.0 {
            return None;
        }
    }
    None
}

fn refine_swept_segment_vs_segment(
    prev_a0: Vec3,
    prev_a1: Vec3,
    velocity: Vec3,
    reach: f32,
    target_radius: f32,
    b0: Vec3,
    b1: Vec3,
    mut lo: f32,
    mut hi: f32,
) -> Option<(f32, ContactGeom)> {
    for _ in 0..24 {
        let mid = (lo + hi) * 0.5;
        let a0 = vec_add(prev_a0, vec_mul(velocity, mid));
        let a1 = vec_add(prev_a1, vec_mul(velocity, mid));
        let (on_a, on_b) = closest_points_segment_segment(a0, a1, b0, b1);
        if vec_len(vec_sub(on_a, on_b)) <= reach {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let a0 = vec_add(prev_a0, vec_mul(velocity, hi));
    let a1 = vec_add(prev_a1, vec_mul(velocity, hi));
    let (on_a, on_b) = closest_points_segment_segment(a0, a1, b0, b1);
    let normal = normalized_or(vec_sub(on_a, on_b), vec_mul(velocity, -1.0))?;
    Some((
        hi,
        ContactGeom {
            point: vec_add(on_b, vec_mul(normal, target_radius)),
            normal,
            depth: 0.0,
        },
    ))
}

fn swept_segment_vs_aabb(
    prev_a0: Vec3,
    prev_a1: Vec3,
    velocity: Vec3,
    reach: f32,
    target_radius: f32,
    half: Vec3,
    box_world: &Mat4,
) -> Option<(f32, ContactGeom)> {
    if vec_len_sq(velocity) < 1e-12 {
        return None;
    }
    let mut lower = 0.0;
    let mut t = 0.0;
    for _ in 0..24 {
        let a0 = vec_add(prev_a0, vec_mul(velocity, t));
        let a1 = vec_add(prev_a1, vec_mul(velocity, t));
        let (on_seg, on_box) = closest_points_segment_aabb(a0, a1, half);
        let delta = vec_sub(on_seg, on_box);
        let dist = vec_len(delta);
        if dist <= reach {
            return refine_swept_segment_vs_aabb(
                prev_a0,
                prev_a1,
                velocity,
                reach,
                target_radius,
                half,
                box_world,
                lower,
                t,
            );
        }
        let normal = normalized_or(delta, vec_mul(velocity, -1.0))?;
        let closing = -vec_dot(velocity, normal);
        if closing <= 1e-6 {
            return None;
        }
        lower = t;
        t += ((dist - reach) / closing).max(1e-5);
        if t > 1.0 {
            return None;
        }
    }
    None
}

fn refine_swept_segment_vs_aabb(
    prev_a0: Vec3,
    prev_a1: Vec3,
    velocity: Vec3,
    reach: f32,
    target_radius: f32,
    half: Vec3,
    box_world: &Mat4,
    mut lo: f32,
    mut hi: f32,
) -> Option<(f32, ContactGeom)> {
    for _ in 0..24 {
        let mid = (lo + hi) * 0.5;
        let a0 = vec_add(prev_a0, vec_mul(velocity, mid));
        let a1 = vec_add(prev_a1, vec_mul(velocity, mid));
        let (on_seg, on_box) = closest_points_segment_aabb(a0, a1, half);
        if vec_len(vec_sub(on_seg, on_box)) <= reach {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let a0 = vec_add(prev_a0, vec_mul(velocity, hi));
    let a1 = vec_add(prev_a1, vec_mul(velocity, hi));
    let (on_seg, on_box) = closest_points_segment_aabb(a0, a1, half);
    let normal_local = normalized_or(vec_sub(on_seg, on_box), vec_mul(velocity, -1.0))?;
    let point_local = vec_add(on_box, vec_mul(normal_local, target_radius));
    Some((
        hi,
        ContactGeom {
            point: box_world.mul_vec_value(&point_local),
            normal: box_world.mul_dir_value(&normal_local),
            depth: 0.0,
        },
    ))
}

fn swept_segment_vs_triangle(
    prev_a0: Vec3,
    prev_a1: Vec3,
    velocity: Vec3,
    reach: f32,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> Option<(f32, ContactGeom)> {
    if vec_len_sq(velocity) < 1e-12 {
        return None;
    }
    let mut lower = 0.0;
    let mut t = 0.0;
    for _ in 0..24 {
        let a0 = vec_add(prev_a0, vec_mul(velocity, t));
        let a1 = vec_add(prev_a1, vec_mul(velocity, t));
        let (on_seg, on_tri) = closest_points_segment_triangle(a0, a1, v0, v1, v2);
        let delta = vec_sub(on_seg, on_tri);
        let dist = vec_len(delta);
        if dist <= reach {
            return refine_swept_segment_vs_triangle(
                prev_a0, prev_a1, velocity, reach, v0, v1, v2, lower, t,
            );
        }
        let normal = normalized_or(delta, vec_mul(velocity, -1.0))?;
        let closing = -vec_dot(velocity, normal);
        if closing <= 1e-6 {
            return None;
        }
        lower = t;
        t += ((dist - reach) / closing).max(1e-5);
        if t > 1.0 {
            return None;
        }
    }
    None
}

fn refine_swept_segment_vs_triangle(
    prev_a0: Vec3,
    prev_a1: Vec3,
    velocity: Vec3,
    reach: f32,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    mut lo: f32,
    mut hi: f32,
) -> Option<(f32, ContactGeom)> {
    for _ in 0..24 {
        let mid = (lo + hi) * 0.5;
        let a0 = vec_add(prev_a0, vec_mul(velocity, mid));
        let a1 = vec_add(prev_a1, vec_mul(velocity, mid));
        let (on_seg, on_tri) = closest_points_segment_triangle(a0, a1, v0, v1, v2);
        if vec_len(vec_sub(on_seg, on_tri)) <= reach {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let a0 = vec_add(prev_a0, vec_mul(velocity, hi));
    let a1 = vec_add(prev_a1, vec_mul(velocity, hi));
    let (on_seg, on_tri) = closest_points_segment_triangle(a0, a1, v0, v1, v2);
    let normal = normalized_or(vec_sub(on_seg, on_tri), vec_mul(velocity, -1.0))?;
    Some((
        hi,
        ContactGeom {
            point: on_tri,
            normal,
            depth: 0.0,
        },
    ))
}

fn closest_points_segment_triangle(
    a0: Vec3,
    a1: Vec3,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> (Vec3, Vec3) {
    let mut best = (a0, closest_point_on_triangle(a0, v0, v1, v2));
    let mut best_dist_sq = vec_len_sq(vec_sub(best.0, best.1));
    let on_tri = closest_point_on_triangle(a1, v0, v1, v2);
    let dist_sq = vec_len_sq(vec_sub(a1, on_tri));
    if dist_sq < best_dist_sq {
        best = (a1, on_tri);
        best_dist_sq = dist_sq;
    }
    for (edge0, edge1) in [(v0, v1), (v1, v2), (v2, v0)] {
        let candidate = closest_points_segment_segment(a0, a1, edge0, edge1);
        let dist_sq = vec_len_sq(vec_sub(candidate.0, candidate.1));
        if dist_sq < best_dist_sq {
            best = candidate;
            best_dist_sq = dist_sq;
        }
    }
    best
}

fn swept_obb_vs_obb(
    world_a: &Mat4,
    half_a: Vec3,
    r_a: f32,
    velocity: Vec3,
    world_b: &Mat4,
    half_b: Vec3,
    r_b: f32,
) -> Option<ContactGeom> {
    if vec_len_sq(velocity) < 1e-12 {
        return None;
    }
    let ax = obb_axes(world_a);
    let bx = obb_axes(world_b);
    let ca = world_a.pos_value();
    let cb = world_b.pos_value();
    let delta0 = vec_sub(vec_sub(ca, velocity), cb);
    let half_a_arr = [half_a.x, half_a.y, half_a.z];
    let half_b_arr = [half_b.x, half_b.y, half_b.z];
    let r_sum = r_a.max(0.0) + r_b.max(0.0);
    let mut entry: f32 = 0.0;
    let mut exit: f32 = 1.0;
    let mut entry_normal: Option<Vec3> = None;
    for axis in swept_obb_axes(&ax, &bx) {
        let Some(axis) = normalize_axis(axis) else {
            continue;
        };
        let limit = obb_projection_radius(&ax, &half_a_arr, axis)
            + obb_projection_radius(&bx, &half_b_arr, axis)
            + r_sum;
        let dist0 = vec_dot(delta0, axis);
        let speed = vec_dot(velocity, axis);
        if speed.abs() < 1e-8 {
            if dist0.abs() > limit {
                return None;
            }
            continue;
        }
        let t0 = (-limit - dist0) / speed;
        let t1 = (limit - dist0) / speed;
        let axis_entry = t0.min(t1);
        let axis_exit = t0.max(t1);
        if axis_entry > entry {
            entry = axis_entry;
            let dist_at_entry = dist0 + speed * axis_entry;
            entry_normal = Some(if dist_at_entry >= 0.0 {
                axis
            } else {
                vec_mul(axis, -1.0)
            });
        }
        exit = exit.min(axis_exit);
        if entry > exit {
            return None;
        }
    }
    if !(0.0..=1.0).contains(&entry) {
        return None;
    }
    let normal = entry_normal?;
    let support_b = obb_projection_radius(&bx, &half_b_arr, normal) + r_b.max(0.0);
    Some(ContactGeom {
        point: vec_add(cb, vec_mul(normal, support_b)),
        normal,
        depth: 0.0,
    })
}

fn swept_obb_vs_triangle(
    world_box: &Mat4,
    half: Vec3,
    radius: f32,
    velocity: Vec3,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> Option<(f32, ContactGeom)> {
    if vec_len_sq(velocity) < 1e-12 {
        return None;
    }
    let box_axes = obb_axes(world_box);
    let tri_edges = [vec_sub(v1, v0), vec_sub(v2, v1), vec_sub(v0, v2)];
    let tri_normal = normalize_axis(vec_cross(tri_edges[0], vec_sub(v2, v0)))?;
    let axes = [
        box_axes[0],
        box_axes[1],
        box_axes[2],
        tri_normal,
        vec_cross(box_axes[0], tri_edges[0]),
        vec_cross(box_axes[0], tri_edges[1]),
        vec_cross(box_axes[0], tri_edges[2]),
        vec_cross(box_axes[1], tri_edges[0]),
        vec_cross(box_axes[1], tri_edges[1]),
        vec_cross(box_axes[1], tri_edges[2]),
        vec_cross(box_axes[2], tri_edges[0]),
        vec_cross(box_axes[2], tri_edges[1]),
        vec_cross(box_axes[2], tri_edges[2]),
    ];
    let center0 = vec_sub(world_box.pos_value(), velocity);
    let half_arr = [half.x, half.y, half.z];
    let mut entry: f32 = 0.0;
    let mut exit: f32 = 1.0;
    let mut entry_normal: Option<Vec3> = None;
    for axis in axes {
        let Some(axis) = normalize_axis(axis) else {
            continue;
        };
        let box_radius = obb_projection_radius(&box_axes, &half_arr, axis) + radius.max(0.0);
        let center_proj = vec_dot(center0, axis);
        let speed = vec_dot(velocity, axis);
        let (tri_min, tri_max) = triangle_projection(v0, v1, v2, axis);
        let low = tri_min - box_radius;
        let high = tri_max + box_radius;
        if speed.abs() < 1e-8 {
            if center_proj < low || center_proj > high {
                return None;
            }
            continue;
        }
        let t0 = (low - center_proj) / speed;
        let t1 = (high - center_proj) / speed;
        let axis_entry = t0.min(t1);
        let axis_exit = t0.max(t1);
        if axis_entry > entry {
            entry = axis_entry;
            let tri_mid = (tri_min + tri_max) * 0.5;
            let center_at_entry = center_proj + speed * axis_entry;
            entry_normal = Some(if center_at_entry >= tri_mid {
                axis
            } else {
                vec_mul(axis, -1.0)
            });
        }
        exit = exit.min(axis_exit);
        if entry > exit {
            return None;
        }
    }
    if !(0.0..=1.0).contains(&entry) {
        return None;
    }
    let normal = entry_normal?;
    let center = vec_add(center0, vec_mul(velocity, entry));
    let support = obb_projection_radius(&box_axes, &half_arr, normal) + radius.max(0.0);
    Some((
        entry,
        ContactGeom {
            point: vec_sub(center, vec_mul(normal, support)),
            normal,
            depth: 0.0,
        },
    ))
}

fn swept_obb_axes(ax: &[Vec3; 3], bx: &[Vec3; 3]) -> [Vec3; 15] {
    [
        ax[0],
        ax[1],
        ax[2],
        bx[0],
        bx[1],
        bx[2],
        vec_cross(ax[0], bx[0]),
        vec_cross(ax[0], bx[1]),
        vec_cross(ax[0], bx[2]),
        vec_cross(ax[1], bx[0]),
        vec_cross(ax[1], bx[1]),
        vec_cross(ax[1], bx[2]),
        vec_cross(ax[2], bx[0]),
        vec_cross(ax[2], bx[1]),
        vec_cross(ax[2], bx[2]),
    ]
}

fn obb_axes(world: &Mat4) -> [Vec3; 3] {
    [
        world.mul_dir_value(&Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }),
        world.mul_dir_value(&Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }),
        world.mul_dir_value(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }),
    ]
}

fn obb_projection_radius(axes: &[Vec3; 3], half: &[f32; 3], axis: Vec3) -> f32 {
    (0..3)
        .map(|i| (vec_dot(axes[i], axis) * half[i]).abs())
        .sum()
}

fn triangle_projection(v0: Vec3, v1: Vec3, v2: Vec3, axis: Vec3) -> (f32, f32) {
    let p0 = vec_dot(v0, axis);
    let p1 = vec_dot(v1, axis);
    let p2 = vec_dot(v2, axis);
    (p0.min(p1).min(p2), p0.max(p1).max(p2))
}

fn component(v: Vec3, axis: usize) -> f32 {
    match axis {
        0 => v.x,
        1 => v.y,
        _ => v.z,
    }
}

fn set_axis(v: &mut Vec3, axis: usize, value: f32) {
    match axis {
        0 => v.x = value,
        1 => v.y = value,
        _ => v.z = value,
    }
}

fn axis_normal(axis: usize, sign: f32) -> Vec3 {
    match axis {
        0 => Vec3 {
            x: sign,
            y: 0.0,
            z: 0.0,
        },
        1 => Vec3 {
            x: 0.0,
            y: sign,
            z: 0.0,
        },
        _ => Vec3 {
            x: 0.0,
            y: 0.0,
            z: sign,
        },
    }
}

fn normalized_or(v: Vec3, fallback: Vec3) -> Option<Vec3> {
    normalize_axis(v).or_else(|| normalize_axis(fallback))
}

fn normalize_axis(v: Vec3) -> Option<Vec3> {
    let len_sq = vec_len_sq(v);
    if len_sq < 1e-12 {
        return None;
    }
    Some(vec_mul(v, 1.0 / len_sq.sqrt()))
}

fn vec_add(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

fn vec_sub(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

fn vec_mul(v: Vec3, scalar: f32) -> Vec3 {
    Vec3 {
        x: v.x * scalar,
        y: v.y * scalar,
        z: v.z * scalar,
    }
}

fn vec_dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn vec_cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

fn vec_len(v: Vec3) -> f32 {
    vec_len_sq(v).sqrt()
}

fn vec_len_sq(v: Vec3) -> f32 {
    vec_dot(v, v)
}

fn swept_sphere_vs_triangle(
    previous_center: Vec3,
    velocity: Vec3,
    radius: f32,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> Option<(f32, ContactGeom)> {
    let mut best = swept_sphere_vs_triangle_face(previous_center, velocity, radius, v0, v1, v2);
    for (a, b) in [(v0, v1), (v1, v2), (v2, v0)] {
        if let Some(hit) = swept_sphere_vs_segment(previous_center, velocity, radius, a, b) {
            if best.as_ref().is_none_or(|(toi, _)| hit.0 < *toi) {
                best = Some(hit);
            }
        }
    }
    for p in [v0, v1, v2] {
        if let Some(hit) = swept_sphere_vs_point(previous_center, velocity, radius, p) {
            if best.as_ref().is_none_or(|(toi, _)| hit.0 < *toi) {
                best = Some(hit);
            }
        }
    }
    best
}

fn swept_sphere_vs_triangle_face(
    previous_center: Vec3,
    velocity: Vec3,
    radius: f32,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> Option<(f32, ContactGeom)> {
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
    let nlen = (nx * nx + ny * ny + nz * nz).sqrt();
    if nlen < 1e-12 {
        return None;
    }
    let mut normal = Vec3 {
        x: nx / nlen,
        y: ny / nlen,
        z: nz / nlen,
    };
    let side = (previous_center.x - v0.x) * normal.x
        + (previous_center.y - v0.y) * normal.y
        + (previous_center.z - v0.z) * normal.z;
    if side < 0.0 {
        normal = Vec3 {
            x: -normal.x,
            y: -normal.y,
            z: -normal.z,
        };
    }

    let dist0 = (previous_center.x - v0.x) * normal.x
        + (previous_center.y - v0.y) * normal.y
        + (previous_center.z - v0.z) * normal.z;
    let dist_delta = velocity.x * normal.x + velocity.y * normal.y + velocity.z * normal.z;
    if dist0 <= radius || dist_delta >= -1e-12 {
        return None;
    }
    let toi = (radius - dist0) / dist_delta;
    if !(0.0..=1.0).contains(&toi) {
        return None;
    }
    let center_hit = Vec3 {
        x: previous_center.x + velocity.x * toi,
        y: previous_center.y + velocity.y * toi,
        z: previous_center.z + velocity.z * toi,
    };
    let point = Vec3 {
        x: center_hit.x - normal.x * radius,
        y: center_hit.y - normal.y * radius,
        z: center_hit.z - normal.z * radius,
    };
    if !point_in_triangle(point, v0, v1, v2) {
        return None;
    }
    Some((
        toi,
        ContactGeom {
            point,
            normal,
            depth: 0.0,
        },
    ))
}

fn swept_sphere_vs_segment(
    previous_center: Vec3,
    velocity: Vec3,
    radius: f32,
    a: Vec3,
    b: Vec3,
) -> Option<(f32, ContactGeom)> {
    let axis = Vec3 {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    let len_sq = axis.x * axis.x + axis.y * axis.y + axis.z * axis.z;
    if len_sq < 1e-12 {
        return swept_sphere_vs_point(previous_center, velocity, radius, a);
    }
    let len = len_sq.sqrt();
    let u = Vec3 {
        x: axis.x / len,
        y: axis.y / len,
        z: axis.z / len,
    };
    let rel = Vec3 {
        x: previous_center.x - a.x,
        y: previous_center.y - a.y,
        z: previous_center.z - a.z,
    };
    let s0 = rel.x * u.x + rel.y * u.y + rel.z * u.z;
    let sv = velocity.x * u.x + velocity.y * u.y + velocity.z * u.z;
    let q0 = Vec3 {
        x: rel.x - u.x * s0,
        y: rel.y - u.y * s0,
        z: rel.z - u.z * s0,
    };
    let qv = Vec3 {
        x: velocity.x - u.x * sv,
        y: velocity.y - u.y * sv,
        z: velocity.z - u.z * sv,
    };
    let qa = qv.x * qv.x + qv.y * qv.y + qv.z * qv.z;
    if qa < 1e-12 {
        return None;
    }
    let qb = 2.0 * (q0.x * qv.x + q0.y * qv.y + q0.z * qv.z);
    let qc = q0.x * q0.x + q0.y * q0.y + q0.z * q0.z - radius * radius;
    if qc <= 0.0 {
        return None;
    }
    let disc = qb * qb - 4.0 * qa * qc;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    for toi in [
        (-qb - sqrt_disc) / (2.0 * qa),
        (-qb + sqrt_disc) / (2.0 * qa),
    ] {
        if !(0.0..=1.0).contains(&toi) {
            continue;
        }
        let s = s0 + sv * toi;
        if s < -1e-5 || s > len + 1e-5 {
            continue;
        }
        let point = Vec3 {
            x: a.x + u.x * s.clamp(0.0, len),
            y: a.y + u.y * s.clamp(0.0, len),
            z: a.z + u.z * s.clamp(0.0, len),
        };
        let center = Vec3 {
            x: previous_center.x + velocity.x * toi,
            y: previous_center.y + velocity.y * toi,
            z: previous_center.z + velocity.z * toi,
        };
        let nx = center.x - point.x;
        let ny = center.y - point.y;
        let nz = center.z - point.z;
        let nlen = (nx * nx + ny * ny + nz * nz).sqrt();
        if nlen < 1e-12 {
            continue;
        }
        return Some((
            toi,
            ContactGeom {
                point,
                normal: Vec3 {
                    x: nx / nlen,
                    y: ny / nlen,
                    z: nz / nlen,
                },
                depth: 0.0,
            },
        ));
    }
    None
}

fn swept_sphere_vs_capsule_axis(
    previous_center: Vec3,
    velocity: Vec3,
    reach: f32,
    capsule_radius: f32,
    a: Vec3,
    b: Vec3,
) -> Option<(f32, ContactGeom)> {
    let axis = Vec3 {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    let len_sq = axis.x * axis.x + axis.y * axis.y + axis.z * axis.z;
    if len_sq < 1e-12 {
        let (toi, mut geom) = swept_sphere_vs_point(previous_center, velocity, reach, a)?;
        geom.point = Vec3 {
            x: a.x + geom.normal.x * capsule_radius,
            y: a.y + geom.normal.y * capsule_radius,
            z: a.z + geom.normal.z * capsule_radius,
        };
        return Some((toi, geom));
    }
    let len = len_sq.sqrt();
    let u = Vec3 {
        x: axis.x / len,
        y: axis.y / len,
        z: axis.z / len,
    };
    let rel = Vec3 {
        x: previous_center.x - a.x,
        y: previous_center.y - a.y,
        z: previous_center.z - a.z,
    };
    let s0 = rel.x * u.x + rel.y * u.y + rel.z * u.z;
    let sv = velocity.x * u.x + velocity.y * u.y + velocity.z * u.z;
    let q0 = Vec3 {
        x: rel.x - u.x * s0,
        y: rel.y - u.y * s0,
        z: rel.z - u.z * s0,
    };
    let qv = Vec3 {
        x: velocity.x - u.x * sv,
        y: velocity.y - u.y * sv,
        z: velocity.z - u.z * sv,
    };
    let qa = qv.x * qv.x + qv.y * qv.y + qv.z * qv.z;
    if qa < 1e-12 {
        return None;
    }
    let qb = 2.0 * (q0.x * qv.x + q0.y * qv.y + q0.z * qv.z);
    let qc = q0.x * q0.x + q0.y * q0.y + q0.z * q0.z - reach * reach;
    if qc <= 0.0 {
        return None;
    }
    let disc = qb * qb - 4.0 * qa * qc;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    for toi in [
        (-qb - sqrt_disc) / (2.0 * qa),
        (-qb + sqrt_disc) / (2.0 * qa),
    ] {
        if !(0.0..=1.0).contains(&toi) {
            continue;
        }
        let s = s0 + sv * toi;
        if s < -1e-5 || s > len + 1e-5 {
            continue;
        }
        let axis_point = Vec3 {
            x: a.x + u.x * s.clamp(0.0, len),
            y: a.y + u.y * s.clamp(0.0, len),
            z: a.z + u.z * s.clamp(0.0, len),
        };
        let center = Vec3 {
            x: previous_center.x + velocity.x * toi,
            y: previous_center.y + velocity.y * toi,
            z: previous_center.z + velocity.z * toi,
        };
        let nx = center.x - axis_point.x;
        let ny = center.y - axis_point.y;
        let nz = center.z - axis_point.z;
        let nlen = (nx * nx + ny * ny + nz * nz).sqrt();
        if nlen < 1e-12 {
            continue;
        }
        let normal = Vec3 {
            x: nx / nlen,
            y: ny / nlen,
            z: nz / nlen,
        };
        return Some((
            toi,
            ContactGeom {
                point: Vec3 {
                    x: axis_point.x + normal.x * capsule_radius,
                    y: axis_point.y + normal.y * capsule_radius,
                    z: axis_point.z + normal.z * capsule_radius,
                },
                normal,
                depth: 0.0,
            },
        ));
    }
    None
}

fn swept_sphere_vs_point(
    previous_center: Vec3,
    velocity: Vec3,
    radius: f32,
    point: Vec3,
) -> Option<(f32, ContactGeom)> {
    let rel = Vec3 {
        x: previous_center.x - point.x,
        y: previous_center.y - point.y,
        z: previous_center.z - point.z,
    };
    let a = velocity.x * velocity.x + velocity.y * velocity.y + velocity.z * velocity.z;
    if a < 1e-12 {
        return None;
    }
    let b = 2.0 * (rel.x * velocity.x + rel.y * velocity.y + rel.z * velocity.z);
    let c = rel.x * rel.x + rel.y * rel.y + rel.z * rel.z - radius * radius;
    if c <= 0.0 {
        return None;
    }
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    let toi = (-b - disc.sqrt()) / (2.0 * a);
    if !(0.0..=1.0).contains(&toi) {
        return None;
    }
    let center = Vec3 {
        x: previous_center.x + velocity.x * toi,
        y: previous_center.y + velocity.y * toi,
        z: previous_center.z + velocity.z * toi,
    };
    let nx = center.x - point.x;
    let ny = center.y - point.y;
    let nz = center.z - point.z;
    let nlen = (nx * nx + ny * ny + nz * nz).sqrt();
    if nlen < 1e-12 {
        return None;
    }
    Some((
        toi,
        ContactGeom {
            point,
            normal: Vec3 {
                x: nx / nlen,
                y: ny / nlen,
                z: nz / nlen,
            },
            depth: 0.0,
        },
    ))
}

fn point_in_triangle(point: Vec3, tri_a: Vec3, tri_b: Vec3, tri_c: Vec3) -> bool {
    let edge_ac = Vec3 {
        x: tri_c.x - tri_a.x,
        y: tri_c.y - tri_a.y,
        z: tri_c.z - tri_a.z,
    };
    let edge_ab = Vec3 {
        x: tri_b.x - tri_a.x,
        y: tri_b.y - tri_a.y,
        z: tri_b.z - tri_a.z,
    };
    let rel = Vec3 {
        x: point.x - tri_a.x,
        y: point.y - tri_a.y,
        z: point.z - tri_a.z,
    };
    let dot00 = edge_ac.x * edge_ac.x + edge_ac.y * edge_ac.y + edge_ac.z * edge_ac.z;
    let dot01 = edge_ac.x * edge_ab.x + edge_ac.y * edge_ab.y + edge_ac.z * edge_ab.z;
    let dot02 = edge_ac.x * rel.x + edge_ac.y * rel.y + edge_ac.z * rel.z;
    let dot11 = edge_ab.x * edge_ab.x + edge_ab.y * edge_ab.y + edge_ab.z * edge_ab.z;
    let dot12 = edge_ab.x * rel.x + edge_ab.y * rel.y + edge_ab.z * rel.z;
    let denom = dot00 * dot11 - dot01 * dot01;
    if denom.abs() < 1e-12 {
        return false;
    }
    let inv = 1.0 / denom;
    let bary_u = (dot11 * dot02 - dot01 * dot12) * inv;
    let bary_v = (dot00 * dot12 - dot01 * dot02) * inv;
    let eps = 1e-5;
    bary_u >= -eps && bary_v >= -eps && bary_u + bary_v <= 1.0 + eps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_draw_context_outside_scope_returns_none() {
        // No context set: with_draw_context returns None.
        let result = with_draw_context(|_| 42);
        assert!(result.is_none());
    }

    #[test]
    fn reset_draw_state_outside_scope_is_noop() {
        // No context set: reset_draw_state must not panic, and
        // with_draw_context still returns None.
        reset_draw_state();
        assert!(with_draw_context(|_| ()).is_none());
    }

    #[test]
    fn test_sphere_above_mesh_floor_generates_contact() {
        use crate::cube::collider::Collider;
        use crate::cube::mesh::Mesh;
        use crate::cube::primitive::Primitive;

        let floor_mesh = Mesh::new();
        {
            let m = rc_mut!(&floor_mesh);
            let geom = Primitive::new();
            {
                let g = rc_mut!(&geom);
                g.positions = vec![
                    -5.0, 0.0, -5.0, 5.0, 0.0, -5.0, -5.0, 0.0, 5.0, 5.0, 0.0, 5.0,
                ];
                g.indices = vec![0, 1, 2, 1, 3, 2];
            }
            m.primitives = vec![Some(geom)];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1];
        }

        let root = Node::new();
        let floor_node = Node::new();
        rc_mut!(&floor_node).collider = Some(Collider::new(
            Vec3::zero(),
            0.0,
            Some(floor_mesh),
            false,
            false,
            0.0,
            0.0,
            0.5,
            Vec3::zero(),
            Vec3::zero(),
        ));
        Node::add_child(&root, &floor_node);

        let ball_node = Node::new();
        rc_mut!(&ball_node).transform = Mat4::from_translation(&Vec3 {
            x: 0.0,
            y: 0.4,
            z: 0.0,
        });
        rc_mut!(&ball_node).collider = Some(Collider::new(
            Vec3::zero(),
            0.5,
            None,
            false,
            true,
            1.0,
            0.0,
            0.5,
            Vec3::zero(),
            Vec3::zero(),
        ));
        Node::add_child(&root, &ball_node);

        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
    }

    #[test]
    fn test_collect_destroyed_post_order_returns_leaves_before_parents() {
        let scene = Node::new();
        let mid = Node::new();
        let leaf = Node::new();
        Node::add_child(&scene, &mid);
        Node::add_child(&mid, &leaf);
        Node::destroy(&mid);
        let collected = Scene::collect_destroyed_post_order(&scene);
        assert_eq!(collected.len(), 2);
        // leaf first (post-order = children before parent)
        assert!(std::rc::Rc::ptr_eq(&collected[0], &leaf));
        assert!(std::rc::Rc::ptr_eq(&collected[1], &mid));
    }

    fn sphere_collider(radius: f32, mass: f32) -> crate::cube::RcCollider {
        crate::cube::Collider::new(
            crate::cube::Vec3::zero(),
            radius,
            None,
            false,
            false,
            mass,
            0.0,
            0.5,
            crate::cube::Vec3::zero(),
            crate::cube::Vec3::zero(),
        )
    }

    fn trigger_sphere_collider(radius: f32, mass: f32) -> crate::cube::RcCollider {
        crate::cube::Collider::new(
            crate::cube::Vec3::zero(),
            radius,
            None,
            true,
            false,
            mass,
            0.0,
            0.5,
            crate::cube::Vec3::zero(),
            crate::cube::Vec3::zero(),
        )
    }

    // Rounded-box family collider (box, rounded box, or capsule by size).
    fn box_family_collider(
        size: crate::cube::RcVec3,
        radius: f32,
        mass: f32,
    ) -> crate::cube::RcCollider {
        crate::cube::Collider::new(
            size,
            radius,
            None,
            false,
            false,
            mass,
            0.0,
            0.5,
            crate::cube::Vec3::zero(),
            crate::cube::Vec3::zero(),
        )
    }

    fn place_at(node: &RcNode, x: f32, y: f32, z: f32) {
        rc_mut!(node).transform = Mat4::from_translation(&Vec3 { x, y, z });
    }

    fn place_rotated_z_at(node: &RcNode, pos_x: f32, pos_y: f32, pos_z: f32, deg: f32) {
        let translation = Mat4::from_translation(&Vec3 {
            x: pos_x,
            y: pos_y,
            z: pos_z,
        });
        let rotation = Mat4::from_axis_angle(
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            deg,
        );
        rc_mut!(node).transform = rc_ref!(&translation).mul_mat(rc_ref!(&rotation));
    }

    #[test]
    fn test_detect_contacts_two_overlapping_spheres() {
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).collider = Some(sphere_collider(0.5, 1.0));
        rc_mut!(&b).collider = Some(sphere_collider(0.5, 1.0));
        place_at(&a, 0.0, 0.0, 0.0);
        place_at(&b, 0.5, 0.0, 0.0);
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        // Normal points from b toward a (= -X).
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.x < -0.99);
    }

    #[test]
    fn test_detect_contacts_swept_sphere_against_static_sphere() {
        let root = Node::new();
        let moving = Node::new();
        let wall = Node::new();
        let moving_collider = sphere_collider(0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(3.0, 0.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        rc_mut!(&wall).collider = Some(sphere_collider(0.25, 0.0));
        place_at(&moving, -1.5, 0.0, 0.0);
        place_at(&wall, 0.0, 0.0, 0.0);
        Node::add_child(&root, &moving);
        Node::add_child(&root, &wall);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.x < -0.99, "normal.x = {}", normal.x);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.x < 0.0,
            "delta_velocity.x = {}",
            delta_velocity.x
        );
    }

    #[test]
    fn test_detect_contacts_swept_sphere_against_static_box() {
        let root = Node::new();
        let moving = Node::new();
        let wall = Node::new();
        let moving_collider = sphere_collider(0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(3.0, 0.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        rc_mut!(&wall).collider = Some(box_family_collider(Vec3::new(0.2, 4.0, 4.0), 0.0, 0.0));
        place_at(&moving, -1.5, 0.0, 0.0);
        place_at(&wall, 0.0, 0.0, 0.0);
        Node::add_child(&root, &moving);
        Node::add_child(&root, &wall);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.x < -0.99, "normal.x = {}", normal.x);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.x < 0.0,
            "delta_velocity.x = {}",
            delta_velocity.x
        );
    }

    #[test]
    fn test_detect_contacts_swept_sphere_against_static_capsule() {
        let root = Node::new();
        let moving = Node::new();
        let post = Node::new();
        let moving_collider = sphere_collider(0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(3.0, 0.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        rc_mut!(&post).collider = Some(box_family_collider(Vec3::new(0.0, 2.0, 0.0), 0.2, 0.0));
        place_at(&moving, -1.5, 0.0, 0.0);
        place_at(&post, 0.0, 0.0, 0.0);
        Node::add_child(&root, &moving);
        Node::add_child(&root, &post);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.x < -0.99, "normal.x = {}", normal.x);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.x < 0.0,
            "delta_velocity.x = {}",
            delta_velocity.x
        );
    }

    #[test]
    fn test_detect_contacts_swept_capsule_against_static_box() {
        let root = Node::new();
        let moving = Node::new();
        let floor = Node::new();
        let moving_collider = box_family_collider(Vec3::new(0.0, 1.0, 0.0), 0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, -3.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        rc_mut!(&floor).collider = Some(box_family_collider(Vec3::new(4.0, 0.2, 4.0), 0.0, 0.0));
        place_at(&moving, 0.0, 1.5, 0.0);
        place_at(&floor, 0.0, 0.0, 0.0);
        Node::add_child(&root, &moving);
        Node::add_child(&root, &floor);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.y > 0.99, "normal.y = {}", normal.y);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.y > 0.0,
            "delta_velocity.y = {}",
            delta_velocity.y
        );
    }

    #[test]
    fn test_detect_contacts_swept_capsule_against_static_capsule() {
        let root = Node::new();
        let moving = Node::new();
        let post = Node::new();
        let moving_collider = box_family_collider(Vec3::new(0.0, 1.0, 0.0), 0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(3.0, 0.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        rc_mut!(&post).collider = Some(box_family_collider(Vec3::new(0.0, 1.0, 0.0), 0.25, 0.0));
        place_at(&moving, -1.5, 0.0, 0.0);
        place_at(&post, 0.0, 0.0, 0.0);
        Node::add_child(&root, &moving);
        Node::add_child(&root, &post);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.x < -0.99, "normal.x = {}", normal.x);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.x < 0.0,
            "delta_velocity.x = {}",
            delta_velocity.x
        );
    }

    #[test]
    fn test_detect_contacts_swept_capsule_side_against_static_capsule() {
        let root = Node::new();
        let moving = Node::new();
        let post = Node::new();
        let moving_collider = box_family_collider(Vec3::new(0.0, 4.0, 0.0), 0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, 0.0, 3.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        rc_mut!(&post).collider = Some(box_family_collider(Vec3::new(0.0, 1.0, 0.0), 0.25, 0.0));
        place_rotated_z_at(&moving, 0.0, 0.0, -2.0, -90.0);
        place_rotated_z_at(&post, 0.0, 0.0, 0.0, -90.0);
        Node::add_child(&root, &moving);
        Node::add_child(&root, &post);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.z < -0.99, "normal.z = {}", normal.z);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.z < 0.0,
            "delta_velocity.z = {}",
            delta_velocity.z
        );
    }

    #[test]
    fn test_detect_contacts_swept_capsule_side_against_static_box() {
        let root = Node::new();
        let moving = Node::new();
        let wall = Node::new();
        let moving_collider = box_family_collider(Vec3::new(0.0, 4.0, 0.0), 0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, 0.0, 3.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        rc_mut!(&wall).collider = Some(box_family_collider(Vec3::new(1.0, 1.0, 1.0), 0.0, 0.0));
        place_rotated_z_at(&moving, 0.0, 0.0, -2.0, -90.0);
        place_at(&wall, 0.0, 0.0, 0.0);
        Node::add_child(&root, &moving);
        Node::add_child(&root, &wall);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.z < -0.99, "normal.z = {}", normal.z);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.z < 0.0,
            "delta_velocity.z = {}",
            delta_velocity.z
        );
    }

    #[test]
    fn test_detect_contacts_swept_box_against_static_box() {
        let root = Node::new();
        let moving = Node::new();
        let wall = Node::new();
        let moving_collider = box_family_collider(Vec3::new(0.4, 0.4, 0.4), 0.0, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(3.0, 0.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        rc_mut!(&wall).collider = Some(box_family_collider(Vec3::new(0.2, 4.0, 4.0), 0.0, 0.0));
        place_at(&moving, -1.5, 0.0, 0.0);
        place_at(&wall, 0.0, 0.0, 0.0);
        Node::add_child(&root, &moving);
        Node::add_child(&root, &wall);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.x < -0.99, "normal.x = {}", normal.x);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.x < 0.0,
            "delta_velocity.x = {}",
            delta_velocity.x
        );
    }

    #[test]
    fn test_detect_contacts_swept_box_against_mesh_floor() {
        let root = mesh_floor_root();
        let moving = Node::new();
        let moving_collider = box_family_collider(Vec3::new(0.5, 0.5, 0.5), 0.0, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, -3.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        place_at(&moving, 0.0, 1.5, 0.0);
        Node::add_child(&root, &moving);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.y > 0.99, "normal.y = {}", normal.y);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.y > 0.0,
            "delta_velocity.y = {}",
            delta_velocity.y
        );
    }

    #[test]
    fn test_detect_contacts_swept_sphere_against_mesh_floor() {
        let root = mesh_floor_root();
        let moving = Node::new();
        let moving_collider = sphere_collider(0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, -3.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        place_at(&moving, 0.0, 1.5, 0.0);
        Node::add_child(&root, &moving);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.y > 0.99, "normal.y = {}", normal.y);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.y > 0.0,
            "delta_velocity.y = {}",
            delta_velocity.y
        );
    }

    #[test]
    fn test_detect_contacts_swept_sphere_against_mesh_edge() {
        let root = mesh_floor_root();
        let moving = Node::new();
        let moving_collider = sphere_collider(0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, -3.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        place_at(&moving, 5.1, 1.5, 0.0);
        Node::add_child(&root, &moving);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.x > 0.0, "normal.x = {}", normal.x);
        assert!(normal.y > 0.0, "normal.y = {}", normal.y);
    }

    #[test]
    fn test_detect_contacts_swept_capsule_against_mesh_floor() {
        let root = mesh_floor_root();
        let moving = Node::new();
        let moving_collider = box_family_collider(Vec3::new(0.0, 1.0, 0.0), 0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, -3.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        place_at(&moving, 0.0, 1.5, 0.0);
        Node::add_child(&root, &moving);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.y > 0.99, "normal.y = {}", normal.y);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.y > 0.0,
            "delta_velocity.y = {}",
            delta_velocity.y
        );
    }

    #[test]
    fn test_detect_contacts_swept_capsule_side_against_mesh_wall() {
        let root = mesh_wall_root();
        let moving = Node::new();
        let moving_collider = box_family_collider(Vec3::new(0.0, 4.0, 0.0), 0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, 0.0, 3.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        place_rotated_z_at(&moving, 0.0, 0.0, -2.0, -90.0);
        Node::add_child(&root, &moving);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.z < -0.99, "normal.z = {}", normal.z);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.z < 0.0,
            "delta_velocity.z = {}",
            delta_velocity.z
        );
    }

    #[test]
    fn test_detect_contacts_swept_box_face_against_mesh_wall() {
        let root = mesh_wall_root();
        let moving = Node::new();
        let moving_collider = box_family_collider(Vec3::new(1.0, 1.0, 1.0), 0.0, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, 0.0, 3.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        place_at(&moving, 0.0, 0.0, -2.0);
        Node::add_child(&root, &moving);

        Scene::integrate_motion(&root);
        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.z < -0.99, "normal.z = {}", normal.z);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!(
            delta_velocity.z < 0.0,
            "delta_velocity.z = {}",
            delta_velocity.z
        );
    }

    #[test]
    fn test_contact_depth_split_evenly_for_equal_mass() {
        // Equal-mass overlap splits the penetration depth 50/50 between
        // the two contacts (mass-share resolution, cube-design.md § 12.3).
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).collider = Some(sphere_collider(0.5, 1.0));
        rc_mut!(&b).collider = Some(sphere_collider(0.5, 1.0));
        place_at(&a, 0.0, 0.0, 0.0);
        place_at(&b, 0.5, 0.0, 0.0);
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        // Penetration = (0.5 + 0.5) - 0.5 = 0.5, halved for equal mass.
        let depth_a = rc_ref!(&pairs[0].contact_a).depth;
        let depth_b = rc_ref!(&pairs[0].contact_b).depth;
        assert!((depth_a - 0.25).abs() < 1e-4, "depth_a = {depth_a}");
        assert!((depth_b - 0.25).abs() < 1e-4, "depth_b = {depth_b}");
    }

    #[test]
    fn test_contact_depth_full_on_movable_against_immovable() {
        // mass == 0 short-circuits the share: the immovable side takes
        // zero push-back, the movable side absorbs the full penetration.
        let root = Node::new();
        let movable = Node::new();
        let wall = Node::new();
        rc_mut!(&movable).collider = Some(sphere_collider(0.5, 1.0));
        rc_mut!(&wall).collider = Some(sphere_collider(0.5, 0.0));
        place_at(&movable, 0.0, 0.0, 0.0);
        place_at(&wall, 0.5, 0.0, 0.0);
        Node::add_child(&root, &movable);
        Node::add_child(&root, &wall);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        // node_a = movable (added first), node_b = wall.
        let depth_movable = rc_ref!(&pairs[0].contact_a).depth;
        let depth_wall = rc_ref!(&pairs[0].contact_b).depth;
        assert!(
            (depth_movable - 0.5).abs() < 1e-4,
            "movable absorbs full = {depth_movable}"
        );
        assert!(
            depth_wall.abs() < 1e-4,
            "immovable takes none = {depth_wall}"
        );
    }

    #[test]
    fn test_immovable_side_receives_no_motion_deltas_even_when_rolls() {
        let root = Node::new();
        let movable = Node::new();
        let wall = Node::new();
        let movable_collider = sphere_collider(0.5, 1.0);
        rc_mut!(&movable_collider).velocity = Vec3::new(0.0, 0.0, 1.0);
        rc_mut!(&movable).collider = Some(movable_collider);
        let wall_collider = sphere_collider(0.5, 0.0);
        rc_mut!(&wall_collider).rolls = true;
        rc_mut!(&wall).collider = Some(wall_collider);
        place_at(&movable, 0.0, 0.0, 0.0);
        place_at(&wall, 0.5, 0.0, 0.0);
        Node::add_child(&root, &movable);
        Node::add_child(&root, &wall);

        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact_wall = rc_ref!(&pairs[0].contact_b);
        assert_eq!(contact_wall.depth, 0.0);
        let delta_velocity = rc_ref!(&contact_wall.delta_velocity);
        let delta_angular_velocity = rc_ref!(&contact_wall.delta_angular_velocity);
        assert_eq!(delta_velocity.x, 0.0);
        assert_eq!(delta_velocity.y, 0.0);
        assert_eq!(delta_velocity.z, 0.0);
        assert_eq!(delta_angular_velocity.x, 0.0);
        assert_eq!(delta_angular_velocity.y, 0.0);
        assert_eq!(delta_angular_velocity.z, 0.0);
    }

    #[test]
    fn test_restitution_reflects_normal_velocity() {
        let root = Node::new();
        let movable = Node::new();
        let wall = Node::new();
        let movable_collider = sphere_collider(0.5, 1.0);
        rc_mut!(&movable_collider).velocity = Vec3::new(1.0, 0.0, 0.0);
        rc_mut!(&movable_collider).restitution = 1.0;
        rc_mut!(&movable).collider = Some(movable_collider);
        rc_mut!(&wall).collider = Some(sphere_collider(0.5, 0.0));
        place_at(&movable, 0.0, 0.0, 0.0);
        place_at(&wall, 0.5, 0.0, 0.0);
        Node::add_child(&root, &movable);
        Node::add_child(&root, &wall);

        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!((delta_velocity.x + 2.0).abs() < 1e-6);
        assert_eq!(delta_velocity.y, 0.0);
        assert_eq!(delta_velocity.z, 0.0);
    }

    #[test]
    fn test_friction_damps_tangential_velocity() {
        let root = Node::new();
        let movable = Node::new();
        let wall = Node::new();
        let movable_collider = sphere_collider(0.5, 1.0);
        rc_mut!(&movable_collider).velocity = Vec3::new(1.0, 1.0, 0.0);
        rc_mut!(&movable_collider).friction = 1.0;
        rc_mut!(&movable).collider = Some(movable_collider);
        let wall_collider = sphere_collider(0.5, 0.0);
        rc_mut!(&wall_collider).friction = 1.0;
        rc_mut!(&wall).collider = Some(wall_collider);
        place_at(&movable, 0.0, 0.0, 0.0);
        place_at(&wall, 0.5, 0.0, 0.0);
        Node::add_child(&root, &movable);
        Node::add_child(&root, &wall);

        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        assert!((delta_velocity.x + 1.0).abs() < 1e-6);
        assert!((delta_velocity.y + 1.0).abs() < 1e-6);
        assert_eq!(delta_velocity.z, 0.0);
    }

    #[test]
    fn test_rolling_side_receives_angular_velocity_delta() {
        let root = Node::new();
        let movable = Node::new();
        let wall = Node::new();
        let movable_collider = sphere_collider(0.5, 1.0);
        rc_mut!(&movable_collider).velocity = Vec3::new(1.0, 1.0, 0.0);
        rc_mut!(&movable_collider).rolls = true;
        rc_mut!(&movable).collider = Some(movable_collider);
        rc_mut!(&wall).collider = Some(sphere_collider(0.5, 0.0));
        place_at(&movable, 0.0, 0.0, 0.0);
        place_at(&wall, 0.5, 0.0, 0.0);
        Node::add_child(&root, &movable);
        Node::add_child(&root, &wall);

        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let delta_angular_velocity = rc_ref!(&contact.delta_angular_velocity);
        assert_eq!(delta_angular_velocity.x, 0.0);
        assert_eq!(delta_angular_velocity.y, 0.0);
        assert!((delta_angular_velocity.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_narrow_phase_rotated_wall_no_phantom_contact() {
        // A thin wall (size=(0.4, 4, 4)) rotated 45° about Y and a sphere
        // 1.0 along the wall's world face normal: the true face gap is
        // 1.0 - 0.2 - 0.5 = 0.3, but the wall's world AABB spans ±1.56
        // on X/Z and swallows the sphere center — the pre-fix AABB
        // narrow phase reported a phantom contact here.
        let root = Node::new();
        let wall = Node::new();
        rc_mut!(&wall).transform = Mat4::from_axis_angle(
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            45.0,
        );
        rc_mut!(&wall).collider = Some(box_family_collider(Vec3::new(0.4, 4.0, 4.0), 0.0, 0.0));
        let ball = Node::new();
        // The wall's local +X face normal maps to (1, 0, -1)/√2 in world.
        let s = 1.0 / std::f32::consts::SQRT_2;
        place_at(&ball, s, 0.0, -s);
        rc_mut!(&ball).collider = Some(sphere_collider(0.5, 1.0));
        Node::add_child(&root, &wall);
        Node::add_child(&root, &ball);
        let pairs = Scene::detect_contacts(&root);
        assert!(pairs.is_empty(), "phantom contact reported");
    }

    #[test]
    fn test_narrow_phase_capsule_rests_on_box_top() {
        // Capsule (size=(0, 1, 0), radius=0.3) standing 0.05 into a
        // static box floor (size=(4, 1, 4), top at y=0.5): bottom cap
        // reach = 1.25 - 0.5 - 0.3 = 0.45 → depth 0.05, normal +Y
        // toward the capsule, full depth on the movable side.
        let root = Node::new();
        let capsule = Node::new();
        place_at(&capsule, 0.0, 1.25, 0.0);
        rc_mut!(&capsule).collider = Some(box_family_collider(Vec3::new(0.0, 1.0, 0.0), 0.3, 1.0));
        let floor = Node::new();
        rc_mut!(&floor).collider = Some(box_family_collider(Vec3::new(4.0, 1.0, 4.0), 0.0, 0.0));
        Node::add_child(&root, &capsule);
        Node::add_child(&root, &floor);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let normal = rc_ref!(&contact.normal);
        assert!(normal.y > 0.99, "normal should point +Y");
        assert!(
            (contact.depth - 0.05).abs() < 1e-4,
            "depth = {}",
            contact.depth
        );
    }

    #[test]
    fn test_narrow_phase_capsule_edge_miss_beyond_box_rim() {
        // Capsule center at (2.2, 1.25, 0), floor rim corner at
        // (2.0, 0.5, 0): segment-to-corner distance √(0.2² + 0.25²)
        // ≈ 0.32 > radius 0.3 → no contact. The pre-fix AABB path
        // overlapped on every axis and reported one.
        let root = Node::new();
        let capsule = Node::new();
        place_at(&capsule, 2.2, 1.25, 0.0);
        rc_mut!(&capsule).collider = Some(box_family_collider(Vec3::new(0.0, 1.0, 0.0), 0.3, 1.0));
        let floor = Node::new();
        rc_mut!(&floor).collider = Some(box_family_collider(Vec3::new(4.0, 1.0, 4.0), 0.0, 0.0));
        Node::add_child(&root, &capsule);
        Node::add_child(&root, &floor);
        let pairs = Scene::detect_contacts(&root);
        assert!(pairs.is_empty(), "rim phantom contact reported");
    }

    #[test]
    fn test_detect_contacts_skips_two_static() {
        // mass==0 on both sides means neither moves; the broad/narrow
        // pipeline drops the pair to avoid emitting no-op deltas.
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).collider = Some(sphere_collider(0.5, 0.0));
        rc_mut!(&b).collider = Some(sphere_collider(0.5, 0.0));
        place_at(&a, 0.0, 0.0, 0.0);
        place_at(&b, 0.5, 0.0, 0.0);
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);
        let pairs = Scene::detect_contacts(&root);
        assert!(pairs.is_empty());
    }

    #[test]
    fn test_static_trigger_pair_still_notifies_without_response() {
        let root = Node::new();
        let sensor = Node::new();
        let wall = Node::new();
        rc_mut!(&sensor).collider = Some(trigger_sphere_collider(0.5, 0.0));
        rc_mut!(&wall).collider = Some(sphere_collider(0.5, 0.0));
        place_at(&sensor, 0.0, 0.0, 0.0);
        place_at(&wall, 0.5, 0.0, 0.0);
        Node::add_child(&root, &sensor);
        Node::add_child(&root, &wall);

        let pairs = Scene::detect_contacts(&root);

        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_eq!(contact.depth, 0.0);
        let delta_velocity = rc_ref!(&contact.delta_velocity);
        let delta_angular_velocity = rc_ref!(&contact.delta_angular_velocity);
        assert_eq!(delta_velocity.x, 0.0);
        assert_eq!(delta_velocity.y, 0.0);
        assert_eq!(delta_velocity.z, 0.0);
        assert_eq!(delta_angular_velocity.x, 0.0);
        assert_eq!(delta_angular_velocity.y, 0.0);
        assert_eq!(delta_angular_velocity.z, 0.0);
    }

    #[test]
    fn test_mesh_collider_mass_is_immovable_for_static_pair_filtering() {
        let (root, terrain) = sparse_triangle_mesh_root();
        let terrain_collider = rc_ref!(&terrain).collider.clone().unwrap();
        rc_mut!(&terrain_collider).mass = 1.0;

        let wall = Node::new();
        rc_mut!(&wall).collider = Some(sphere_collider(0.5, 0.0));
        place_at(&wall, 0.2, 0.4, 0.2);
        Node::add_child(&root, &wall);

        let pairs = Scene::detect_contacts(&root);

        assert!(pairs.is_empty());
    }

    // Two-triangle floor quad at y=0 under a mesh collider, as the
    // fixture for the mesh raycast tests below.
    fn mesh_floor_root() -> RcNode {
        use crate::cube::collider::Collider;
        use crate::cube::mesh::Mesh;
        use crate::cube::primitive::Primitive;

        let floor_mesh = Mesh::new();
        {
            let m = rc_mut!(&floor_mesh);
            let geom = Primitive::new();
            {
                let g = rc_mut!(&geom);
                g.positions = vec![
                    -5.0, 0.0, -5.0, 5.0, 0.0, -5.0, -5.0, 0.0, 5.0, 5.0, 0.0, 5.0,
                ];
                g.indices = vec![0, 1, 2, 1, 3, 2];
            }
            m.primitives = vec![Some(geom)];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1];
        }
        let root = Node::new();
        let floor = Node::new();
        rc_mut!(&floor).collider = Some(Collider::new(
            Vec3::zero(),
            0.0,
            Some(floor_mesh),
            false,
            false,
            0.0,
            0.0,
            0.5,
            Vec3::zero(),
            Vec3::zero(),
        ));
        Node::add_child(&root, &floor);
        root
    }

    fn mesh_wall_root() -> RcNode {
        use crate::cube::collider::Collider;
        use crate::cube::mesh::Mesh;
        use crate::cube::primitive::Primitive;

        let wall_mesh = Mesh::new();
        {
            let m = rc_mut!(&wall_mesh);
            let geom = Primitive::new();
            {
                let g = rc_mut!(&geom);
                g.positions = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
                g.indices = vec![0, 1, 2];
            }
            m.primitives = vec![Some(geom)];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1];
        }
        let root = Node::new();
        let wall = Node::new();
        rc_mut!(&wall).collider = Some(Collider::new(
            Vec3::zero(),
            0.0,
            Some(wall_mesh),
            false,
            false,
            0.0,
            0.0,
            0.5,
            Vec3::zero(),
            Vec3::zero(),
        ));
        Node::add_child(&root, &wall);
        root
    }

    fn sparse_triangle_mesh_root() -> (RcNode, RcNode) {
        use crate::cube::collider::Collider;
        use crate::cube::mesh::Mesh;
        use crate::cube::primitive::Primitive;

        let mesh = Mesh::new();
        {
            let m = rc_mut!(&mesh);
            let geom = Primitive::new();
            {
                let g = rc_mut!(&geom);
                g.positions = vec![0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 10.0];
                g.indices = vec![0, 1, 2];
            }
            m.primitives = vec![Some(geom)];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1];
        }
        let root = Node::new();
        let terrain = Node::new();
        rc_mut!(&terrain).collider = Some(Collider::new(
            Vec3::zero(),
            0.0,
            Some(mesh),
            false,
            false,
            0.0,
            0.0,
            0.5,
            Vec3::zero(),
            Vec3::zero(),
        ));
        Node::add_child(&root, &terrain);
        (root, terrain)
    }

    #[test]
    fn test_raycast_mesh_floor_reports_triangle_hit() {
        let root = mesh_floor_root();
        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 0.3,
                y: 5.0,
                z: 0.3,
            },
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            f32::INFINITY,
            false,
            None,
        )
        .expect("downward ray must hit the mesh floor");
        assert!(
            (hit.distance - 5.0).abs() < 1e-3,
            "distance = {}",
            hit.distance
        );
        // ray_vs_triangle faces the normal toward the ray origin (+Y here).
        assert!(hit.normal.y > 0.99);
        assert!(hit.point.y.abs() < 1e-3);
    }

    #[test]
    fn test_raycast_mesh_respects_max_distance() {
        let root = mesh_floor_root();
        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 0.3,
                y: 5.0,
                z: 0.3,
            },
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            3.0,
            false,
            None,
        );
        assert!(hit.is_none(), "floor at distance 5 must not hit within 3");
    }

    #[test]
    fn test_raycast_returns_nearest_hit() {
        let root = Node::new();
        let near = Node::new();
        let far = Node::new();
        rc_mut!(&near).collider = Some(sphere_collider(0.5, 1.0));
        rc_mut!(&far).collider = Some(sphere_collider(0.5, 1.0));
        place_at(&near, 0.0, 0.0, 0.0);
        place_at(&far, 0.0, 0.0, -5.0);
        Node::add_child(&root, &near);
        Node::add_child(&root, &far);
        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            f32::INFINITY,
            false,
            None,
        )
        .unwrap();
        assert!((hit.distance - 4.5).abs() < 1e-3);
        assert!(std::rc::Rc::ptr_eq(&hit.node, &near));
    }

    #[test]
    fn test_raycast_normalizes_direction_for_distance_and_max_distance() {
        let root = Node::new();
        let target = Node::new();
        rc_mut!(&target).collider = Some(sphere_collider(0.5, 1.0));
        Node::add_child(&root, &target);

        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            f32::INFINITY,
            false,
            None,
        )
        .unwrap();
        assert!((hit.distance - 4.5).abs() < 1e-3);

        let capped = Scene::raycast(
            &root,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            3.0,
            false,
            None,
        );
        assert!(capped.is_none());
    }

    #[test]
    fn test_raycast_clamps_negative_collider_radius_to_zero() {
        let root = Node::new();
        let point = Node::new();
        rc_mut!(&point).collider = Some(sphere_collider(-0.5, 1.0));
        Node::add_child(&root, &point);

        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            f32::INFINITY,
            false,
            None,
        )
        .unwrap();

        assert!((hit.distance - 5.0).abs() < 1e-3);
    }

    #[test]
    fn test_raycast_misses_capsule_aabb_corner() {
        let root = Node::new();
        let capsule = Node::new();
        rc_mut!(&capsule).collider = Some(box_family_collider(Vec3::new(0.0, 2.0, 0.0), 0.2, 1.0));
        Node::add_child(&root, &capsule);

        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 0.2,
                y: 1.2,
                z: 1.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            f32::INFINITY,
            false,
            None,
        );

        assert!(hit.is_none());
    }

    #[test]
    fn test_raycast_all_misses_capsule_aabb_corner() {
        let root = Node::new();
        let capsule = Node::new();
        rc_mut!(&capsule).collider = Some(box_family_collider(Vec3::new(0.0, 2.0, 0.0), 0.2, 1.0));
        Node::add_child(&root, &capsule);

        let hits = Scene::raycast_all(
            &root,
            Vec3 {
                x: 0.2,
                y: 1.2,
                z: 1.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            f32::INFINITY,
            false,
            None,
        );

        assert!(hits.is_empty());
    }

    #[test]
    fn test_raycast_misses_rotated_box_aabb_corner() {
        let root = Node::new();
        let wall = Node::new();
        rc_mut!(&wall).collider = Some(box_family_collider(Vec3::new(0.2, 2.0, 2.0), 0.0, 1.0));
        let rot = Mat4::from_axis_angle(
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            45.0,
        );
        rc_mut!(&wall).transform = rot;
        Node::add_child(&root, &wall);

        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 0.75,
                y: 2.0,
                z: 0.75,
            },
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            f32::INFINITY,
            false,
            None,
        );

        assert!(hit.is_none());
    }

    #[test]
    fn test_raycast_misses_rounded_box_corner_outside_radius() {
        let root = Node::new();
        let wall = Node::new();
        rc_mut!(&wall).collider = Some(box_family_collider(Vec3::new(1.0, 1.0, 1.0), 0.25, 1.0));
        Node::add_child(&root, &wall);

        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 2.0,
                y: 0.74,
                z: 0.74,
            },
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            f32::INFINITY,
            false,
            None,
        );

        assert!(hit.is_none());
    }

    #[test]
    fn test_raycast_hits_rounded_box_face() {
        let root = Node::new();
        let wall = Node::new();
        rc_mut!(&wall).collider = Some(box_family_collider(Vec3::new(1.0, 1.0, 1.0), 0.25, 1.0));
        Node::add_child(&root, &wall);

        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            f32::INFINITY,
            false,
            None,
        )
        .unwrap();

        assert!((hit.distance - 1.25).abs() < 1e-3);
        assert!(hit.normal.x > 0.99, "normal.x = {}", hit.normal.x);
    }

    #[test]
    fn test_raycast_skips_triggers_by_default() {
        let root = Node::new();
        let n = Node::new();
        let coll = sphere_collider(0.5, 1.0);
        rc_mut!(&coll).trigger = true;
        rc_mut!(&n).collider = Some(coll);
        Node::add_child(&root, &n);
        let hit = Scene::raycast(
            &root,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            f32::INFINITY,
            false,
            None,
        );
        assert!(hit.is_none());
        let hit_with_triggers = Scene::raycast(
            &root,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            f32::INFINITY,
            true,
            None,
        );
        assert!(hit_with_triggers.is_some());
    }

    #[test]
    fn test_raycast_all_sorted_by_distance() {
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        let c = Node::new();
        rc_mut!(&a).collider = Some(sphere_collider(0.3, 1.0));
        rc_mut!(&b).collider = Some(sphere_collider(0.3, 1.0));
        rc_mut!(&c).collider = Some(sphere_collider(0.3, 1.0));
        place_at(&a, 0.0, 0.0, -3.0);
        place_at(&b, 0.0, 0.0, -1.0);
        place_at(&c, 0.0, 0.0, -2.0);
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);
        Node::add_child(&root, &c);
        let hits = Scene::raycast_all(
            &root,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            f32::INFINITY,
            false,
            None,
        );
        assert_eq!(hits.len(), 3);
        for i in 1..hits.len() {
            assert!(hits[i].distance >= hits[i - 1].distance);
        }
    }

    #[test]
    fn test_overlap_sphere_filters_by_tag() {
        let root = Node::new();
        let enemy = Node::new();
        let friend = Node::new();
        rc_mut!(&enemy).collider = Some(sphere_collider(0.5, 1.0));
        rc_mut!(&friend).collider = Some(sphere_collider(0.5, 1.0));
        rc_mut!(&enemy).tags = vec!["enemy".to_string()];
        rc_mut!(&friend).tags = vec!["friend".to_string()];
        Node::add_child(&root, &enemy);
        Node::add_child(&root, &friend);
        let only_enemy = Scene::overlap_sphere(
            &root,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            false,
            Some(&["enemy".to_string()]),
        );
        assert_eq!(only_enemy.len(), 1);
        assert!(std::rc::Rc::ptr_eq(&only_enemy[0], &enemy));
    }

    #[test]
    fn test_overlap_sphere_clamps_negative_collider_radius_to_zero() {
        let root = Node::new();
        let point = Node::new();
        rc_mut!(&point).collider = Some(sphere_collider(-0.5, 1.0));
        place_at(&point, 0.75, 0.0, 0.0);
        Node::add_child(&root, &point);

        let nodes = Scene::overlap_sphere(
            &root,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            false,
            None,
        );

        assert!(nodes.iter().any(|n| std::rc::Rc::ptr_eq(n, &point)));
    }

    #[test]
    fn test_overlap_sphere_misses_capsule_aabb_corner() {
        let root = Node::new();
        let capsule = Node::new();
        rc_mut!(&capsule).collider = Some(box_family_collider(Vec3::new(0.0, 2.0, 0.0), 0.2, 1.0));
        Node::add_child(&root, &capsule);

        let nodes = Scene::overlap_sphere(
            &root,
            Vec3 {
                x: 0.2,
                y: 1.2,
                z: 0.2,
            },
            0.05,
            false,
            None,
        );

        assert!(nodes.is_empty());
    }

    #[test]
    fn test_overlap_box_finds_overlapping_sphere() {
        let root = Node::new();
        let inside = Node::new();
        let outside = Node::new();
        rc_mut!(&inside).collider = Some(sphere_collider(0.5, 1.0));
        rc_mut!(&outside).collider = Some(sphere_collider(0.5, 1.0));
        place_at(&outside, 10.0, 0.0, 0.0);
        Node::add_child(&root, &inside);
        Node::add_child(&root, &outside);
        let identity_rc = Mat4::identity();
        let identity = *rc_ref!(&identity_rc);
        let nodes = Scene::overlap_box(
            &root,
            &identity,
            Vec3 {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
            false,
            None,
        );
        assert_eq!(nodes.len(), 1);
        assert!(std::rc::Rc::ptr_eq(&nodes[0], &inside));
    }

    #[test]
    fn test_overlap_box_uses_current_aabb_not_swept_motion() {
        let root = Node::new();
        let mover = Node::new();
        let coll = box_family_collider(Vec3::new(1.0, 1.0, 1.0), 0.0, 1.0);
        rc_mut!(&coll).velocity = Vec3::new(10.0, 0.0, 0.0);
        rc_mut!(&mover).collider = Some(coll);
        place_at(&mover, 10.0, 0.0, 0.0);
        Node::add_child(&root, &mover);
        let identity_rc = Mat4::identity();
        let identity = *rc_ref!(&identity_rc);

        let nodes = Scene::overlap_box(
            &root,
            &identity,
            Vec3 {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
            false,
            None,
        );

        assert!(nodes.is_empty());
    }

    #[test]
    fn test_overlap_box_misses_capsule_aabb_corner() {
        let root = Node::new();
        let capsule = Node::new();
        rc_mut!(&capsule).collider = Some(box_family_collider(Vec3::new(0.0, 2.0, 0.0), 0.2, 1.0));
        Node::add_child(&root, &capsule);
        let query = Mat4::from_translation(&Vec3 {
            x: 0.2,
            y: 1.2,
            z: 0.2,
        });
        let query = *rc_ref!(&query);

        let nodes = Scene::overlap_box(
            &root,
            &query,
            Vec3 {
                x: 0.05,
                y: 0.05,
                z: 0.05,
            },
            false,
            None,
        );

        assert!(nodes.is_empty());
    }

    #[test]
    fn test_overlap_sphere_mesh_uses_triangle_geometry() {
        let (root, terrain) = sparse_triangle_mesh_root();
        let nodes = Scene::overlap_sphere(
            &root,
            Vec3 {
                x: 9.0,
                y: 0.0,
                z: 9.0,
            },
            0.1,
            false,
            None,
        );
        assert!(!nodes.iter().any(|n| std::rc::Rc::ptr_eq(n, &terrain)));
    }

    #[test]
    fn test_overlap_box_mesh_uses_triangle_geometry() {
        let (root, terrain) = sparse_triangle_mesh_root();
        let query = Mat4::from_translation(&Vec3 {
            x: 9.0,
            y: 0.0,
            z: 9.0,
        });
        let query = *rc_ref!(&query);
        let nodes = Scene::overlap_box(
            &root,
            &query,
            Vec3 {
                x: 0.2,
                y: 0.2,
                z: 0.2,
            },
            false,
            None,
        );
        assert!(!nodes.iter().any(|n| std::rc::Rc::ptr_eq(n, &terrain)));
    }

    #[test]
    fn test_integrate_motion_skips_inactive_subtree() {
        let root = Node::new();
        let n = Node::new();
        let coll = sphere_collider(0.5, 1.0);
        rc_mut!(&coll).velocity = Vec3::new(1.0, 0.0, 0.0);
        rc_mut!(&n).collider = Some(coll);
        rc_mut!(&root).active = false;
        Node::add_child(&root, &n);
        Scene::integrate_motion(&root);
        let pos_rc = rc_ref!(&n).transform.clone();
        let pos = rc_ref!(&pos_rc).pos();
        assert_eq!(rc_ref!(&pos).x, 0.0, "inactive subtree should not move");
    }
}
