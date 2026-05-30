use std::cell::Cell;

use crate::cube::camera::RcCamera;
use crate::cube::collider::RcCollider;
use crate::cube::collision::{
    aabb_vs_aabb, aabb_vs_triangle, collider_aabb, ray_vs_aabb, ray_vs_sphere, ray_vs_triangle,
    sphere_vs_aabb, sphere_vs_sphere, sphere_vs_triangle, Aabb, ContactGeom,
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
// of the receiver Node's cache for the duration of the traversal), looked
// up by Node draw commands through `with_draw_context`, torn down on draw
// end (depth buffer moved back into the receiver Node).

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
    // one draw. The receiver Node caches the allocation between frames;
    // ctx takes it in at draw entry and returns it on exit.
    pub depth: Vec<f32>,
    pub depth_w: u32,
    pub depth_h: u32,
    // Per-on_draw state modifiers, mutated via Node.dither / depth_test /
    // depth_write / shaded setters; reset to defaults before each Node's
    // on_draw via reset_draw_state(). Rasterizers consult ctx for these
    // fields directly.
    pub dither_alpha: f32,
    pub depth_test: bool,
    pub depth_write: bool,
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
    CURRENT_DRAW_CONTEXT.with(|c| c.take())
}

// Reset the per-on_draw state modifiers on the active draw context to
// their defaults. Called by the binding's traverse_draw before invoking
// each Node.on_draw so state never leaks across siblings or children.
pub fn reset_draw_state() {
    with_draw_context(|ctx| {
        ctx.dither_alpha = 1.0;
        ctx.depth_test = true;
        ctx.depth_write = true;
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
// block covers the deterministic core stages.

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
        Self::collect_collider_entries(scene_root, &mut entries);
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
                let mass_a = rc_ref!(&coll_a_rc).mass;
                let mass_b = rc_ref!(&coll_b_rc).mass;
                if mass_a == 0.0 && mass_b == 0.0 {
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

    fn collect_collider_entries(node: &RcNode, out: &mut Vec<(RcNode, Mat4, Aabb)>) {
        if !Node::effective_active(node) {
            return;
        }
        let coll_opt = rc_ref!(node).collider.clone();
        if let Some(coll_rc) = coll_opt {
            let world_rc = Node::world_transform(node);
            let world = *rc_ref!(&world_rc);
            let aabb = collider_aabb(rc_ref!(&coll_rc), &world);
            out.push((node.clone(), world, aabb));
        }
        let children = rc_ref!(node).children.clone();
        for c in &children {
            Self::collect_collider_entries(c, out);
        }
    }

    fn narrow_phase(
        world_a: &Mat4,
        coll_a: &RcCollider,
        world_b: &Mat4,
        coll_b: &RcCollider,
    ) -> Option<ContactGeom> {
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
            let geom = narrow_phase_mesh_vs_dynamic(world_a, &mesh_rc, world_b, size_b, r_b)?;
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
            return narrow_phase_mesh_vs_dynamic(world_b, &mesh_rc, world_a, size_a, r_a);
        }
        let is_sphere_a = size_a.x.abs() < 1e-9 && size_a.y.abs() < 1e-9 && size_a.z.abs() < 1e-9;
        let is_sphere_b = size_b.x.abs() < 1e-9 && size_b.y.abs() < 1e-9 && size_b.z.abs() < 1e-9;
        let c_a_rc = world_a.pos();
        let c_a = *rc_ref!(&c_a_rc);
        let c_b_rc = world_b.pos();
        let c_b = *rc_ref!(&c_b_rc);

        if is_sphere_a && is_sphere_b {
            sphere_vs_sphere(c_a, r_a, c_b, r_b)
        } else if is_sphere_a {
            let aabb_b = Aabb::from_rounded_box(world_b, size_b, r_b);
            sphere_vs_aabb(c_a, r_a, &aabb_b)
        } else if is_sphere_b {
            let aabb_a = Aabb::from_rounded_box(world_a, size_a, r_a);
            // Reverse the result so the normal points from b -> a.
            let r = sphere_vs_aabb(c_b, r_b, &aabb_a)?;
            Some(ContactGeom {
                point: r.point,
                normal: Vec3 {
                    x: -r.normal.x,
                    y: -r.normal.y,
                    z: -r.normal.z,
                },
                depth: r.depth,
            })
        } else {
            let aabb_a = Aabb::from_rounded_box(world_a, size_a, r_a);
            let aabb_b = Aabb::from_rounded_box(world_b, size_b, r_b);
            aabb_vs_aabb(&aabb_a, &aabb_b)
        }
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
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
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
        let mut entries: Vec<(RcNode, Mat4, Aabb)> = Vec::new();
        Self::collect_collider_entries(scene_root, &mut entries);
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
        let mut entries: Vec<(RcNode, Mat4, Aabb)> = Vec::new();
        Self::collect_collider_entries(scene_root, &mut entries);
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
        let radius = coll.radius;
        let has_mesh = coll.mesh.is_some();
        if !Self::tags_match(node, tags_filter) {
            return None;
        }
        if !has_mesh && ray_vs_aabb(origin, direction, aabb, max_distance).is_none() {
            return None;
        }
        let is_sphere =
            size.x.abs() < 1e-9 && size.y.abs() < 1e-9 && size.z.abs() < 1e-9 && !has_mesh;
        let hit = if has_mesh {
            Self::ray_vs_mesh_collider(origin, direction, world, &coll_rc, max_distance)
        } else if is_sphere {
            let center_rc = world.pos();
            let center = *rc_ref!(&center_rc);
            ray_vs_sphere(origin, direction, center, radius, max_distance)
        } else {
            ray_vs_aabb(origin, direction, aabb, max_distance)
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
        Self::collect_collider_entries(scene_root, &mut entries);
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
            let r_other = coll.radius;
            let has_mesh = coll.mesh.is_some();
            if !Self::tags_match(node, tags_filter) {
                continue;
            }
            if !probe.overlaps(aabb) {
                continue;
            }
            let is_sphere =
                size.x.abs() < 1e-9 && size.y.abs() < 1e-9 && size.z.abs() < 1e-9 && !has_mesh;
            let hit = if has_mesh {
                true
            } else if is_sphere {
                let c_rc = world.pos();
                let c = *rc_ref!(&c_rc);
                sphere_vs_sphere(center, radius, c, r_other).is_some()
            } else {
                sphere_vs_aabb(center, radius, aabb).is_some()
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
        Self::collect_collider_entries(scene_root, &mut entries);
        let probe = Aabb::from_rounded_box(transform, size, 0.0);
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
            let r_other = coll.radius;
            let has_mesh = coll.mesh.is_some();
            if !Self::tags_match(node, tags_filter) {
                continue;
            }
            if !probe.overlaps(aabb) {
                continue;
            }
            let is_sphere = other_size.x.abs() < 1e-9
                && other_size.y.abs() < 1e-9
                && other_size.z.abs() < 1e-9
                && !has_mesh;
            let hit = if has_mesh {
                true
            } else if is_sphere {
                let c_rc = world.pos();
                let c = *rc_ref!(&c_rc);
                sphere_vs_aabb(c, r_other, &probe).is_some()
            } else {
                aabb_vs_aabb(&probe, aabb).is_some()
            };
            if hit {
                out.push(node.clone());
            }
        }
        out
    }

    fn tags_match(node: &RcNode, filter: Option<&[String]>) -> bool {
        let Some(tags) = filter else {
            return true;
        };
        let node_tags = rc_ref!(node).tags.clone();
        tags.iter().any(|t| node_tags.iter().any(|nt| nt == t))
    }

    fn ray_vs_mesh_collider(
        origin: Vec3,
        direction: Vec3,
        world: &Mat4,
        coll: &RcCollider,
        max_distance: f32,
    ) -> Option<(f32, Vec3, Vec3)> {
        let mesh_rc = rc_ref!(coll).mesh.clone()?;
        let mesh = rc_ref!(&mesh_rc);
        let world_per_part = mesh.compose_world_transforms(world);
        let mut best: Option<(f32, Vec3, Vec3)> = None;
        for (idx, part_world) in world_per_part.iter().enumerate() {
            let Some(geom_rc) = &mesh.geometries[idx] else {
                continue;
            };
            let g = rc_ref!(geom_rc);
            let positions = &g.positions;
            let indices_clone = g.indices.clone();
            let triangle_count = match &indices_clone {
                Some(idx) => idx.len() / 3,
                None => positions.len() / 9,
            };
            for tri in 0..triangle_count {
                let (i0, i1, i2) = if let Some(ids) = &indices_clone {
                    (
                        ids[tri * 3] as usize,
                        ids[tri * 3 + 1] as usize,
                        ids[tri * 3 + 2] as usize,
                    )
                } else {
                    (tri * 3, tri * 3 + 1, tri * 3 + 2)
                };
                let v0 = Vec3 {
                    x: positions[i0 * 3],
                    y: positions[i0 * 3 + 1],
                    z: positions[i0 * 3 + 2],
                };
                let v1 = Vec3 {
                    x: positions[i1 * 3],
                    y: positions[i1 * 3 + 1],
                    z: positions[i1 * 3 + 2],
                };
                let v2 = Vec3 {
                    x: positions[i2 * 3],
                    y: positions[i2 * 3 + 1],
                    z: positions[i2 * 3 + 2],
                };
                let v0_rc = part_world.mul_vec(&v0);
                let v0_w = *rc_ref!(&v0_rc);
                let v1_rc = part_world.mul_vec(&v1);
                let v1_w = *rc_ref!(&v1_rc);
                let v2_rc = part_world.mul_vec(&v2);
                let v2_w = *rc_ref!(&v2_rc);
                let cur_max = best.as_ref().map_or(max_distance, |(t, _, _)| *t);
                if let Some(hit) = ray_vs_triangle(origin, direction, v0_w, v1_w, v2_w, cur_max) {
                    best = Some(hit);
                }
            }
        }
        best
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
    let is_sphere = size_dyn.x.abs() < 1e-9 && size_dyn.y.abs() < 1e-9 && size_dyn.z.abs() < 1e-9;
    let mesh_inv_rc = world_mesh.inverse();
    let mesh_inv = *rc_ref!(&mesh_inv_rc);
    // Dynamic body's AABB in world space, then mapped into the mesh-
    // local frame where the BVH lives.
    let dyn_aabb_world = if is_sphere {
        let c_rc = world_dyn.pos();
        let c = *rc_ref!(&c_rc);
        Aabb::from_sphere(c, r_dyn)
    } else {
        Aabb::from_rounded_box(world_dyn, size_dyn, r_dyn)
    };
    let query_local = transform_aabb_to_local(&mesh_inv, &dyn_aabb_world);
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
            let hit = if is_sphere {
                let c_rc = world_dyn.pos();
                let c = *rc_ref!(&c_rc);
                sphere_vs_triangle(c, r_dyn, v0, v1, v2)
            } else {
                let aabb = Aabb::from_rounded_box(world_dyn, size_dyn, r_dyn);
                aabb_vs_triangle(&aabb, v0, v1, v2)
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
    let r = m.mul_vec(&v);
    *rc_ref!(&r)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_depth_default() {
        let n = Node::new();
        let n = rc_ref!(&n);
        assert_eq!(n.depth_w, 0);
        assert_eq!(n.depth_h, 0);
        assert!(n.depth.is_empty());
    }

    #[test]
    fn test_node_ensure_depth() {
        let n = Node::new();
        let n_mut = rc_mut!(&n);
        n_mut.ensure_depth(64, 48);
        assert_eq!(n_mut.depth_w, 64);
        assert_eq!(n_mut.depth_h, 48);
        assert_eq!(n_mut.depth.len(), 64 * 48);
    }

    #[test]
    fn test_node_ensure_depth_resize() {
        let n = Node::new();
        let n_mut = rc_mut!(&n);
        n_mut.ensure_depth(64, 48);
        n_mut.ensure_depth(128, 96);
        assert_eq!(n_mut.depth.len(), 128 * 96);
    }

    #[test]
    fn test_node_clear_depth() {
        let n = Node::new();
        let n_mut = rc_mut!(&n);
        n_mut.ensure_depth(8, 8);
        n_mut.depth[0] = 0.5;
        n_mut.clear_depth();
        assert_eq!(n_mut.depth[0], f32::INFINITY);
    }

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
        use crate::cube::geometry::Geometry;
        use crate::cube::mesh::Mesh;

        let floor_mesh = Mesh::new();
        {
            let m = rc_mut!(&floor_mesh);
            let geom = Geometry::new();
            {
                let g = rc_mut!(&geom);
                g.positions = vec![
                    -5.0, 0.0, -5.0, 5.0, 0.0, -5.0, -5.0, 0.0, 5.0, 5.0, 0.0, 5.0,
                ];
                g.indices = Some(vec![0, 1, 2, 1, 3, 2]);
            }
            m.geometries = vec![Some(geom)];
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

    fn place_at(node: &RcNode, x: f32, y: f32, z: f32) {
        rc_mut!(node).transform = Mat4::from_translation(&Vec3 { x, y, z });
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
