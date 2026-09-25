use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::cube::camera::RcCamera;
use crate::cube::collider::{Collider, RcCollider};
use crate::cube::collision::{
    capsule_vs_capsule, capsule_vs_rounded_obb, capsule_vs_sphere, capsule_vs_triangle,
    classify_shape, closest_points_segment_aabb, closest_points_segment_segment,
    closest_points_segment_triangle, collider_aabb, local_box_vs_triangle, ray_vs_aabb,
    ray_vs_sphere, ray_vs_sphere_filtered, ray_vs_triangle, rounded_obb_vs_rounded_obb,
    sphere_vs_rounded_obb, sphere_vs_sphere, sphere_vs_triangle, Aabb, ColliderShape, ContactGeom,
    ObbPairDistance,
};
use crate::cube::contact::{Contact, RcContact};
use crate::cube::mat4::Mat4;
use crate::cube::mesh::RcMesh;
use crate::cube::node::{Node, RcNode};
use crate::cube::raster::{ClipRect, Mat4x4};
use crate::cube::vec3::Vec3;
use crate::image::RcImage;

// Per-frame rasterizer context shared across a Node::draw traversal

// One transformed vertex in the prim scratch cache: world position and
// screen projection (None = at or behind the camera plane).
pub type ProjectedVertex = (Vec3, Option<(f32, f32, f32)>);

pub struct DrawContext {
    pub target: RcImage,
    pub vp: Mat4x4,
    // Camera-plane clip row for near clipping (raster::camera_clip_row)
    pub clip_row: [f32; 4],
    pub vp_x: f32,
    pub vp_y: f32,
    pub vp_w: f32,
    pub vp_h: f32,
    pub clip: ClipRect,
    pub camera: RcCamera,

    // The effective camera caches the depth allocation between frames.
    pub depth: Vec<f32>,
    pub depth_w: u32,
    pub depth_h: u32,
    // Shared vertices are projected once per prim call without reallocating.
    pub vertex_cache: Vec<ProjectedVertex>,

    // State modifiers reset before each Node.on_draw call
    pub dither_alpha: f32,
    pub depth_test: bool,
    pub depth_write: bool,
    // View-direction depth bias that leaves screen position unchanged
    pub depth_offset: f32,
    pub decal_distance: Option<f32>,
    pub shaded: bool,
}

thread_local! {
    // Current draw context, set by Node::draw for the duration of the
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
        ctx.decal_distance = None;
        ctx.shaded = true;
    });
}

// Taking the context makes nested access return None rather than aliasing mutable state.
pub fn with_draw_context<R>(f: impl FnOnce(&mut DrawContext) -> R) -> Option<R> {
    CURRENT_DRAW_CONTEXT.with(|cell| {
        let mut ctx = cell.take()?;
        let result = f(&mut ctx);
        cell.set(Some(ctx));
        Some(result)
    })
}

// Pipeline data types passed across the binding boundary

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

// Contact solver state

#[derive(Clone)]
struct ColliderEntry {
    node: RcNode,
    world: Mat4,
    inverse: Mat4,
    aabb: Aabb,
    collider: RcCollider,
    velocity: Vec3,
    immovable: bool,
    trigger: bool,
    margin: f32,
}

#[derive(Clone)]
struct ContactConstraint {
    pair: usize,
    a: usize,
    b: usize,
    normal: Vec3,
    depth: f32,
    shares: (f32, f32),
    push: f32,
    swept: bool,
}

struct ContactBody {
    inverse_mass: f32,
    inverse_inertia: Vec3,
    axes: [Vec3; 3],
    velocity: Vec3,
    spin: Vec3,
}

impl ContactBody {
    fn new(entry: &ColliderEntry, mass_unit: f32) -> Self {
        let collider = rc_ref!(&entry.collider);
        // A common mass unit keeps equivalent scenes numerically equivalent.
        // Ratios below float precision already behave as an immovable partner.
        let physical_mass = collider.contact_mass();
        let mass = if physical_mass > 0.0 {
            (physical_mass / mass_unit).max(1e-12)
        } else {
            0.0
        };
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        let radius = collider.radius.max(0.0);
        let size = *rc_ref!(&collider.size);
        let inertia = match classify_shape(size, radius) {
            ColliderShape::Sphere { r } => {
                let i = 0.4 * mass * r * r;
                Vec3 { x: i, y: i, z: i }
            }
            ColliderShape::Capsule { half_h, r } => {
                let length = 2.0 * half_h;
                let cylinder = mass * length / (length + 4.0 * r / 3.0);
                let caps = mass - cylinder;
                let axial = cylinder * r * r * 0.5 + caps * r * r * 0.4;
                let radial = cylinder * (3.0 * r * r + length * length) / 12.0
                    + caps * (0.4 * r * r + half_h * half_h + 0.75 * half_h * r);
                Vec3 {
                    x: radial,
                    y: axial,
                    z: radial,
                }
            }
            ColliderShape::RoundedBox { half, r } => {
                let x = (half.x + r) * 2.0;
                let y = (half.y + r) * 2.0;
                let z = (half.z + r) * 2.0;
                Vec3 {
                    x: mass * (y * y + z * z) / 12.0,
                    y: mass * (x * x + z * z) / 12.0,
                    z: mass * (x * x + y * y) / 12.0,
                }
            }
        };
        let inverse = |i: f32| {
            if collider.rolls && mass > 0.0 && i > 0.0 {
                1.0 / i
            } else {
                0.0
            }
        };

        let rotation = Node::world_rotation_value(&entry.node);
        let axes = std::array::from_fn(|i| Vec3 {
            x: rotation.data[0][i],
            y: rotation.data[1][i],
            z: rotation.data[2][i],
        });
        let spin = if collider.mesh.is_some() {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            rotation.mul_dir_value(&vec_mul(
                *rc_ref!(&collider.angular_velocity),
                1.0_f32.to_radians(),
            ))
        };

        Self {
            inverse_mass,
            inverse_inertia: Vec3 {
                x: inverse(inertia.x),
                y: inverse(inertia.y),
                z: inverse(inertia.z),
            },
            axes,
            velocity: entry.velocity,
            spin,
        }
    }

    fn apply_inverse_inertia(&self, torque: Vec3) -> Vec3 {
        let mut result = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        for (axis, inverse) in self.axes.iter().zip([
            self.inverse_inertia.x,
            self.inverse_inertia.y,
            self.inverse_inertia.z,
        ]) {
            result = vec_add(result, vec_mul(*axis, vec_dot(*axis, torque) * inverse));
        }
        result
    }

    fn point_velocity(&self, arm: Vec3) -> Vec3 {
        vec_add(self.velocity, vec_cross(self.spin, arm))
    }

    fn apply_impulse(&mut self, arm: Vec3, impulse: Vec3) {
        self.velocity = vec_add(self.velocity, vec_mul(impulse, self.inverse_mass));
        self.spin = vec_add(
            self.spin,
            self.apply_inverse_inertia(vec_cross(arm, impulse)),
        );
    }
}

#[derive(Default)]
pub(crate) struct ContactCache {
    contacts: Vec<CachedContact>,
    bodies: Vec<CachedBody>,
}

struct CachedBody {
    node: crate::cube::node::WeakNode,
    world: Mat4,
    velocity: Vec3,
    spin: Vec3,
    output_velocity: Vec3,
    acceleration: Vec3,
    previous_acceleration: Vec3,
    material: [f32; 8],
    quiet_frames: u8,
    tolerance: f32,
    integrated: bool,
}

impl CachedBody {
    fn matches_pose(&self, world: &Mat4, displacement: Vec3) -> bool {
        let position = vec_sub(world.pos_value(), displacement);
        vec_len_sq(vec_sub(position, self.world.pos_value())) < self.tolerance * self.tolerance
            && (0..3).all(|row| {
                (0..3).all(|col| (world.data[row][col] - self.world.data[row][col]).abs() < 0.0001)
            })
    }

    fn matches_integrated_pose(&self, entry: &ColliderEntry) -> bool {
        let mut world = entry.world;
        let mut displacement = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        if self.integrated {
            displacement = entry.velocity;
            let collider = rc_ref!(&entry.collider);
            if collider.mesh.is_none() {
                let angular = *rc_ref!(&collider.angular_velocity);
                let length = vec_len(angular);
                if length > 1e-6 {
                    let undo =
                        Mat4::from_axis_angle_value(&vec_mul(angular, 1.0 / length), -length);
                    world = world.mul_mat_value(&undo);
                }
            }
        }
        self.matches_pose(&world, displacement)
    }

    fn matches_motion(&self, velocity: Vec3, spin: Vec3) -> bool {
        // Sleeping removes the previous residual velocity, so repeated caller
        // acceleration is applied to zero on the next update.
        let expected_velocity = if self.quiet_frames >= 20 {
            self.acceleration
        } else {
            self.velocity
        };
        vec_len_sq(vec_sub(velocity, expected_velocity)) < self.tolerance * self.tolerance
            && vec_len_sq(vec_sub(spin, self.spin)) < 1e-8
    }
}

struct CachedContact {
    nodes: [crate::cube::node::WeakNode; 2],
    anchors: [Vec3; 2],
    normal: Vec3,
    impulse: Vec3,
    mass_unit: f32,
}

struct ContactImpulse {
    pair: usize,
    group: usize,
    a: usize,
    b: usize,
    normal: Vec3,
    ra: Vec3,
    rb: Vec3,
    bounce: f32,
    friction: f32,
    normal_impulse: f32,
    tangent_impulse: Vec3,
    tangents: [Vec3; 2],
    angular_a: [Vec3; 3],
    angular_b: [Vec3; 3],
    normal_mass: f32,
    tangent_mass: [f32; 3],
    velocity_change: [f32; 3],
}

thread_local! {
    static COLLIDER_ENTRY_SCRATCH: RefCell<Vec<ColliderEntry>> = const { RefCell::new(Vec::new()) };
}

// Pipeline and spatial-query operations on a subtree
pub struct Scene;

// Spatial-query kernels keep shapes, transforms, velocities, and tolerances
// explicit so hot-path calls stay stateless and avoid parameter objects.

impl Scene {
    // Collect destroyed nodes leaf-first for the binding layer's on_destroy
    // notification and detachment at the end of Node.update.
    pub fn collect_destroyed_post_order(scene_root: &RcNode) -> Vec<RcNode> {
        let mut out: Vec<RcNode> = Vec::new();
        Self::collect_destroyed_recursive(scene_root, &mut out);
        out
    }

    fn collect_destroyed_recursive(node: &RcNode, out: &mut Vec<RcNode>) {
        let node_ref = rc_ref!(node);
        for child in &node_ref.children {
            Self::collect_destroyed_recursive(child, out);
        }
        if node_ref.destroyed && !node_ref.destroy_processed {
            out.push(node.clone());
        }
    }

    // Consume before calling Python, which can reenter update or raise an exception.
    pub fn take_destroy_notification(node: &RcNode) -> bool {
        let mut node = rc_mut!(node);
        if !node.destroyed || node.destroy_notified {
            return false;
        }
        node.destroy_notified = true;
        true
    }

    // Detachment completes cleanup without clearing the public destruction state.
    pub fn detach_destroyed(node: &RcNode) {
        Node::detach(node);
        rc_mut!(node).destroy_processed = true;
    }

    // Motion integration: walks the active subtree and applies each
    // analytic collider's velocity / angular_velocity to its node transform.
    pub fn integrate_motion(scene_root: &RcNode, frame_seconds: f32) {
        let parent = Node::parent(scene_root);
        if parent
            .as_ref()
            .is_some_and(|parent| !Node::effective_active(parent))
        {
            return;
        }
        let parent_world = parent.as_ref().map(Node::world_transform_value);
        let mut cache = rc_mut!(scene_root).contact_cache.take();
        let mut previous: HashMap<_, _> = cache
            .iter_mut()
            .flat_map(|cache| &mut cache.bodies)
            .map(|body| {
                body.integrated = true;
                (body.node.as_ptr(), body)
            })
            .collect();
        // Geometry uses centimeters; gravity is specified in meters per second squared.
        Self::integrate_motion_recursive(
            scene_root,
            parent_world.as_ref(),
            &mut previous,
            frame_seconds,
        );
        rc_mut!(scene_root).contact_cache = cache;
    }

    fn integrate_motion_recursive(
        node: &RcNode,
        parent_world: Option<&Mat4>,
        previous: &mut HashMap<*const RefCell<Node>, &mut CachedBody>,
        frame_seconds: f32,
    ) {
        if !rc_ref!(node).active {
            return;
        }

        let coll_opt = rc_ref!(node).collider.clone();
        if let Some(coll_rc) = coll_opt {
            let mut coll = rc_mut!(&coll_rc);
            if coll.mesh.is_none() && coll.mass > 0.0 {
                let linear = (1.0 - coll.linear_damp.max(0.0) * frame_seconds).max(0.0);
                let angular = (1.0 - coll.angular_damp.max(0.0) * frame_seconds).max(0.0);
                let mut velocity = vec_mul(*rc_ref!(&coll.velocity), linear);
                let spin = vec_mul(*rc_ref!(&coll.angular_velocity), angular);
                let gravity_step = 100.0 * frame_seconds * frame_seconds;
                let direction = *rc_ref!(&coll.gravity_direction);
                let length = vec_len(direction);
                if length > 0.0 {
                    velocity = vec_add(
                        velocity,
                        vec_mul(direction, coll.gravity * gravity_step / length),
                    );
                }
                // Vec3 values can be shared with callers and constants; replace their handles.
                coll.velocity = Vec3::new(velocity.x, velocity.y, velocity.z);
                coll.angular_velocity = Vec3::new(spin.x, spin.y, spin.z);
            }
            let world_velocity = Self::effective_linear_velocity(parent_world, &coll);
            let angular_velocity = if coll.mesh.is_none() {
                *rc_ref!(&coll.angular_velocity)
            } else {
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            };

            let angular_len_sq = angular_velocity.x * angular_velocity.x
                + angular_velocity.y * angular_velocity.y
                + angular_velocity.z * angular_velocity.z;
            let resting = previous
                .get_mut(&std::rc::Rc::as_ptr(node))
                .is_some_and(|old| {
                    if old.quiet_frames < 20 || old.material != contact_material(&coll) {
                        return false;
                    }
                    let local = *rc_ref!(&rc_ref!(node).transform);
                    let world = parent_world.map_or(local, |parent| parent.mul_mat_value(&local));
                    let spin = Node::world_rotation_value(node)
                        .mul_dir_value(&vec_mul(angular_velocity, 1.0_f32.to_radians()));
                    let unchanged = old.matches_pose(
                        &world,
                        Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    ) && old.matches_motion(world_velocity, spin);
                    old.integrated = !unchanged;
                    unchanged
                });
            // Do not sink a sleeping stack under the same per-frame gravity.
            // Correcting that sink along slightly tilted contact normals would
            // otherwise accumulate sideways drift even with zero final speed.
            if !resting
                && (world_velocity.x != 0.0
                    || world_velocity.y != 0.0
                    || world_velocity.z != 0.0
                    || angular_len_sq > 1e-12)
            {
                let transform_rc = rc_ref!(node).transform.clone();
                let mut transform = *rc_ref!(&transform_rc);
                let local_displacement = parent_world.map_or(world_velocity, |parent| {
                    parent.inverse_value().mul_dir_value(&world_velocity)
                });

                // Apply the parent-local displacement without assuming the
                // transform's homogeneous row is canonical.
                let homogeneous = transform.data[3];
                for (col, &component) in homogeneous.iter().enumerate() {
                    transform.data[0][col] += local_displacement.x * component;
                    transform.data[1][col] += local_displacement.y * component;
                    transform.data[2][col] += local_displacement.z * component;
                }

                if angular_len_sq > 1e-12 {
                    let len = angular_len_sq.sqrt();
                    let axis = Vec3 {
                        x: angular_velocity.x / len,
                        y: angular_velocity.y / len,
                        z: angular_velocity.z / len,
                    };
                    // Spin is a local-frame rotation, so it multiplies on
                    // the right without moving the translation column.
                    let rotation = Mat4::from_axis_angle_value(&axis, len);
                    transform = transform.mul_mat_value(&rotation);
                }

                rc_mut!(node).transform = Mat4::from_rows(transform.data);
            }
        }

        let transform_rc = rc_ref!(node).transform.clone();
        let local = *rc_ref!(&transform_rc);
        let world = parent_world.map_or(local, |parent| parent.mul_mat_value(&local));
        let node_ref = rc_ref!(node);
        for child in &node_ref.children {
            Self::integrate_motion_recursive(child, Some(&world), previous, frame_seconds);
        }
    }

    fn effective_linear_velocity(parent_world: Option<&Mat4>, coll: &Collider) -> Vec3 {
        let parent_is_invertible = parent_world.is_none_or(|parent| {
            let determinant = parent.determinant();
            determinant.is_finite() && determinant.abs() >= 1e-12
        });
        if coll.mesh.is_some() || !parent_is_invertible {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            *rc_ref!(&coll.velocity)
        }
    }

    // Collision detection: AABB refresh, broad phase, narrow phase, and
    // response resolution. Returns the contact pairs the binding layer
    // feeds to on_collide.
    pub fn detect_contacts(scene_root: &RcNode) -> Vec<ContactPair> {
        let previous = rc_mut!(scene_root).contact_cache.take();
        Self::with_collider_entries(scene_root, true, |entries| {
            let n = entries.len();
            let mut pairs: Vec<ContactPair> = Vec::new();
            let mut constraints = Vec::new();

            for i in 0..n {
                for j in (i + 1)..n {
                    let entry_a = &entries[i];
                    let entry_b = &entries[j];
                    let margin = entry_a.margin.max(entry_b.margin);
                    let mut bounds = entry_a.aabb;
                    bounds.min = vec_sub(
                        bounds.min,
                        Vec3 {
                            x: margin,
                            y: margin,
                            z: margin,
                        },
                    );
                    bounds.max = vec_add(
                        bounds.max,
                        Vec3 {
                            x: margin,
                            y: margin,
                            z: margin,
                        },
                    );
                    if !bounds.overlaps(&entry_b.aabb) {
                        continue;
                    }
                    if entry_a.immovable
                        && entry_b.immovable
                        && !entry_a.trigger
                        && !entry_b.trigger
                    {
                        continue;
                    }

                    let mesh_contact = rc_ref!(&entry_a.collider).mesh.is_some()
                        || rc_ref!(&entry_b.collider).mesh.is_some();
                    let iterations = if mesh_contact && !entry_a.trigger && !entry_b.trigger {
                        8
                    } else {
                        1
                    };
                    let mut world_a = entry_a.world;
                    let mut world_b = entry_b.world;
                    let mut velocity_a = entry_a.velocity;
                    let mut velocity_b = entry_b.velocity;
                    let first_constraint = constraints.len();
                    let zero = Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    };

                    for iteration in 0..iterations {
                        // Keep sweeping the motion along resolved surfaces: a
                        // wall contact must not hide a floor crossed in this frame.
                        let Some((geom, swept)) = Self::narrow_phase(
                            &world_a,
                            &entry_a.collider,
                            velocity_a,
                            &world_b,
                            &entry_b.collider,
                            velocity_b,
                        )
                        .or_else(|| {
                            if margin == 0.0 || entry_a.trigger || entry_b.trigger {
                                return None;
                            }
                            // Keep near-touching support contacts coherent. A
                            // separated point may close its gap, but cannot pull.
                            Self::narrow_phase_with_margin(
                                &world_a,
                                &entry_a.collider,
                                zero,
                                &world_b,
                                &entry_b.collider,
                                zero,
                                margin,
                            )
                            .map(|(mut geom, swept)| {
                                geom.depth -= margin;
                                (geom, swept)
                            })
                        }) else {
                            break;
                        };
                        if iteration > 0 && geom.depth <= 1e-5 {
                            break;
                        }

                        let normal = geom.normal;
                        let offset = (0..3)
                            .map(|axis| {
                                component(normal, axis)
                                    * (world_a.data[axis][3]
                                        - entry_a.world.data[axis][3]
                                        - world_b.data[axis][3]
                                        + entry_b.world.data[axis][3])
                            })
                            .sum::<f32>();
                        let depth = geom.depth + offset;
                        // Opposite faces reached by successive mesh corrections
                        // can be alternative exits from a thin solid. Do not
                        // turn those exits into an impossible pair of constraints.
                        if constraints[first_constraint..].iter().any(
                            |previous: &ContactConstraint| {
                                previous.normal.dot(&normal) < -0.9999
                                    && previous.depth + depth > 1e-5
                            },
                        ) {
                            break;
                        }

                        let pair = Self::build_contact_pair(
                            &entry_a.node,
                            &entry_a.collider,
                            &entry_b.node,
                            &entry_b.collider,
                            ContactGeom {
                                depth: geom.depth.max(0.0),
                                ..geom
                            },
                        );
                        if !entry_a.trigger && !entry_b.trigger {
                            let a = rc_ref!(&entry_a.collider);
                            let b = rc_ref!(&entry_b.collider);
                            constraints.push(ContactConstraint {
                                pair: pairs.len(),
                                a: i,
                                b: j,
                                normal,
                                depth,
                                shares: Self::contact_shares(a.contact_mass(), b.contact_mass()),
                                push: 0.0,
                                swept,
                            });
                        }
                        if iterations > 1 {
                            for (world, velocity, contact) in [
                                (&mut world_a, &mut velocity_a, &pair.contact_a),
                                (&mut world_b, &mut velocity_b, &pair.contact_b),
                            ] {
                                Scene::apply_contact_to_snapshot(world, velocity, contact);
                                // Continue only the tangential sweep. Physical
                                // friction and restitution are resolved later.
                                let contact = rc_ref!(contact);
                                let normal = rc_ref!(&contact.normal);
                                let inward = velocity.dot(&normal).min(0.0);
                                velocity.x -= normal.x * inward;
                                velocity.y -= normal.y * inward;
                                velocity.z -= normal.z * inward;
                            }
                        }
                        if pairs.is_empty() {
                            pairs.reserve(n);
                        }
                        pairs.push(pair);
                        if geom.depth <= 1e-5 {
                            break;
                        }
                    }
                }
            }

            Self::resolve_contact_positions(entries, &pairs, &mut constraints);
            let next_cache = Self::resolve_contact_velocities(
                entries,
                &pairs,
                &constraints,
                previous.as_deref().unwrap_or(&ContactCache::default()),
            );
            if !next_cache.contacts.is_empty() {
                let mut cache = previous.unwrap_or_default();
                *cache = next_cache;
                rc_mut!(scene_root).contact_cache = Some(cache);
            }
            pairs
        })
    }

    // Coupled normal responses make a stack's floor and inter-block contacts agree
    // about support. Callers still receive and apply the same contact callbacks.
    fn resolve_contact_positions(
        entries: &[ColliderEntry],
        pairs: &[ContactPair],
        constraints: &mut [ContactConstraint],
    ) {
        if constraints.len() < 2 {
            return;
        }
        let zero = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut offsets = vec![zero; entries.len()];
        for iteration in 0..32 {
            let mut largest_change = 0.0_f32;
            for step in 0..constraints.len() {
                let index = if iteration % 2 == 0 {
                    step
                } else {
                    constraints.len() - 1 - step
                };
                let constraint = &mut constraints[index];
                let normal = constraint.normal;
                let separation =
                    normal.dot(&offsets[constraint.a]) - normal.dot(&offsets[constraint.b]);
                // Keep overlap above the coordinate rounding error so touching
                // bodies remain in the next frame's contact graph. A fixed
                // world-unit slop disappears at larger coordinate magnitudes.
                let slop = if constraint.shares.0 > 0.0 && constraint.shares.1 > 0.0 {
                    let scale = (0..3).fold(1.0_f32, |scale, axis| {
                        scale
                            .max(entries[constraint.a].world.data[axis][3].abs())
                            .max(entries[constraint.b].world.data[axis][3].abs())
                    });
                    (4.0 * f32::EPSILON * scale).max(1e-5).max(
                        entries[constraint.a]
                            .margin
                            .max(entries[constraint.b].margin)
                            * 0.1,
                    )
                } else {
                    0.0
                };
                let push = (constraint.push + constraint.depth - slop - separation).max(0.0);
                let push_change = push - constraint.push;
                constraint.push = push;
                largest_change = largest_change.max(push_change.abs());
                for (index, share) in [
                    (constraint.a, constraint.shares.0),
                    (constraint.b, -constraint.shares.1),
                ] {
                    offsets[index].x += normal.x * push_change * share;
                    offsets[index].y += normal.y * push_change * share;
                    offsets[index].z += normal.z * push_change * share;
                }
            }
            if largest_change < 1e-5 {
                break;
            }
        }

        for constraint in constraints {
            let pair = &pairs[constraint.pair];
            for (contact, share) in [
                (&pair.contact_a, constraint.shares.0),
                (&pair.contact_b, constraint.shares.1),
            ] {
                let mut contact = rc_mut!(contact);
                contact.depth = constraint.push * share;
            }
        }
    }

    fn resolve_contact_velocities(
        entries: &[ColliderEntry],
        pairs: &[ContactPair],
        constraints: &[ContactConstraint],
        cache: &ContactCache,
    ) -> ContactCache {
        if constraints.is_empty() {
            return ContactCache::default();
        }

        // Without rotation or restitution, contact points all have the same
        // velocity. Solve translation directly: no contact patches, inertia,
        // warm-start history, or sleeping-body bookkeeping are needed.
        let is_linear = |contact: &ContactConstraint| {
            [contact.a, contact.b].into_iter().all(|i| {
                let collider = rc_ref!(&entries[i].collider);
                collider.restitution == 0.0
                    && (collider.mesh.is_some()
                        || ((!collider.rolls || entries[i].immovable)
                            && vec_len_sq(*rc_ref!(&collider.angular_velocity)) == 0.0))
            })
        };
        if constraints.iter().all(is_linear) {
            Self::resolve_linear_velocities(entries, pairs, constraints);
            return ContactCache::default();
        }

        // A rolling body elsewhere in the scene must not make every character
        // pay for rigid-body response. Only moving bodies connect contact groups;
        // sharing the same immovable floor does not connect them.
        let mut groups: Vec<_> = (0..entries.len()).collect();
        for contact in constraints {
            if !entries[contact.a].immovable && !entries[contact.b].immovable {
                let a = contact_group(&groups, contact.a);
                let b = contact_group(&groups, contact.b);
                groups[b] = a;
            }
        }
        for i in 0..groups.len() {
            groups[i] = contact_group(&groups, i);
        }
        let mut rigid = vec![false; entries.len()];
        for contact in constraints.iter().filter(|contact| !is_linear(contact)) {
            for i in [contact.a, contact.b] {
                if !entries[i].immovable {
                    rigid[groups[i]] = true;
                }
            }
        }

        let mut linear_contacts = Vec::new();
        let mut rigid_contacts = Vec::new();
        let mut rigid_entries = Vec::new();
        let mut indices = vec![usize::MAX; entries.len()];
        for contact in constraints {
            if !rigid[groups[contact.a]] && !rigid[groups[contact.b]] {
                linear_contacts.push(contact.clone());
                continue;
            }
            let mut contact = contact.clone();
            for i in [&mut contact.a, &mut contact.b] {
                if indices[*i] == usize::MAX {
                    indices[*i] = rigid_entries.len();
                    rigid_entries.push(entries[*i].clone());
                }
                *i = indices[*i];
            }
            rigid_contacts.push(contact);
        }

        if !linear_contacts.is_empty() {
            Self::resolve_linear_velocities(entries, pairs, &linear_contacts);
        }
        Self::resolve_rigid_velocities(&rigid_entries, pairs, &rigid_contacts, cache)
    }

    fn resolve_rigid_velocities(
        entries: &[ColliderEntry],
        pairs: &[ContactPair],
        constraints: &[ContactConstraint],
        cache: &ContactCache,
    ) -> ContactCache {
        let previous = &cache.contacts;
        let zero = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut groups: Vec<_> = (0..entries.len()).collect();
        for contact in constraints {
            if !entries[contact.a].immovable && !entries[contact.b].immovable {
                let a = contact_group(&groups, contact.a);
                let b = contact_group(&groups, contact.b);
                groups[b] = a;
            }
        }
        for i in 0..groups.len() {
            groups[i] = contact_group(&groups, i);
        }

        // Unrelated contact groups must not change each other's mass ratios or
        // rounding, even when their bodies have very different mass scales.
        let mut mass_units = vec![f32::MIN_POSITIVE; entries.len()];
        for (i, entry) in entries.iter().enumerate() {
            mass_units[groups[i]] =
                mass_units[groups[i]].max(rc_ref!(&entry.collider).contact_mass());
        }
        let contact_mass_unit =
            |a: usize, b: usize| mass_units[groups[if entries[a].immovable { b } else { a }]];
        let mut bodies: Vec<_> = entries
            .iter()
            .enumerate()
            .map(|(i, entry)| ContactBody::new(entry, mass_units[groups[i]]))
            .collect();
        let initial_velocities: Vec<_> = bodies
            .iter()
            .map(|body| (body.velocity, body.spin))
            .collect();
        let materials: Vec<_> = entries
            .iter()
            .map(|entry| contact_material(&rc_ref!(&entry.collider)))
            .collect();
        let old_body_map: HashMap<_, _> = cache
            .bodies
            .iter()
            .map(|old| (old.node.as_ptr(), old))
            .collect();
        let old_bodies: Vec<_> = entries
            .iter()
            .map(|entry| old_body_map.get(&std::rc::Rc::as_ptr(&entry.node)).copied())
            .collect();

        let mut old_pairs = HashMap::new();
        let mut start = 0;
        // Contact points for one collider pair are adjacent in the cache.
        while start < previous.len() {
            let key = (
                previous[start].nodes[0].as_ptr(),
                previous[start].nodes[1].as_ptr(),
            );
            let mut end = start + 1;
            while end < previous.len()
                && (
                    previous[end].nodes[0].as_ptr(),
                    previous[end].nodes[1].as_ptr(),
                ) == key
            {
                end += 1;
            }
            old_pairs.insert(key, start..end);
            start = end;
        }

        let mut quiet = vec![20_u8; entries.len()];
        let mut supported = vec![false; entries.len()];
        let mut valid = vec![false; entries.len()];
        let mut poses_valid = vec![false; entries.len()];
        for (i, entry) in entries.iter().enumerate() {
            poses_valid[i] = old_bodies[i].is_some_and(|old| {
                old.material == materials[i] && old.matches_integrated_pose(entry)
            });
            // Awake bodies settle according to their solved motion. Only an
            // already sleeping body must preserve its input motion to stay asleep.
            valid[i] = poses_valid[i]
                && old_bodies[i].is_some_and(|old| {
                    old.quiet_frames < 20 || old.matches_motion(bodies[i].velocity, bodies[i].spin)
                });
            if bodies[i].inverse_mass > 0.0 {
                quiet[groups[i]] = quiet[groups[i]].min(if valid[i] && entry.margin > 0.0 {
                    old_bodies[i].unwrap().quiet_frames
                } else {
                    0
                });
            }
        }
        for contact in constraints {
            for (body, other) in [(contact.a, contact.b), (contact.b, contact.a)] {
                if bodies[body].inverse_mass == 0.0 {
                    continue;
                }
                if bodies[other].inverse_mass == 0.0 {
                    let still = vec_len_sq(bodies[other].velocity) == 0.0
                        && vec_len_sq(bodies[other].spin) == 0.0;
                    supported[groups[body]] |= still;
                    if !still || !valid[other] {
                        quiet[groups[body]] = 0;
                    }
                }
                if materials[body][1] + materials[other][1] == 0.0 {
                    quiet[groups[body]] = 0;
                }
            }
        }

        let mut impulses = Vec::new();
        let mut reused = vec![false; previous.len()];
        for constraint in constraints {
            let a = &entries[constraint.a];
            let b = &entries[constraint.b];
            let pair = &pairs[constraint.pair];
            let mass_unit = contact_mass_unit(constraint.a, constraint.b);
            // Sweeps use the contact surfaces at impact. End-of-frame centers
            // can be far beyond those surfaces and invent large torque arms.
            let impact = constraint.swept.then(|| {
                let closing = -vec_dot(vec_sub(a.velocity, b.velocity), constraint.normal);
                let remaining = if closing > 0.0 {
                    (constraint.depth / closing).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                [a, b].map(|entry| {
                    let mut entry = entry.clone();
                    let offset = vec_mul(entry.velocity, -remaining);
                    for axis in 0..3 {
                        entry.world.data[axis][3] += component(offset, axis);
                    }
                    entry.inverse = entry.world.inverse_value();
                    entry.aabb.min = vec_add(entry.aabb.min, offset);
                    entry.aabb.max = vec_add(entry.aabb.max, offset);
                    entry
                })
            });
            let (a, b) = impact
                .as_ref()
                .map_or((a, b), |entries| (&entries[0], &entries[1]));
            let point = *rc_ref!(&rc_ref!(&pair.contact_a).point);
            let depth = if constraint.swept {
                0.0
            } else {
                constraint.depth
            };
            let points = Self::contact_patch(a, b, constraint.normal, point, depth);
            let key = (std::rc::Rc::as_ptr(&a.node), std::rc::Rc::as_ptr(&b.node));
            let old_range = old_pairs.get(&key).cloned().unwrap_or(0..0);

            let restitution = rc_ref!(&a.collider)
                .restitution
                .max(rc_ref!(&b.collider).restitution);
            let support_speed = if restitution > 0.0
                && poses_valid[constraint.a]
                && poses_valid[constraint.b]
                && old_range.clone().any(|i| {
                    vec_dot(previous[i].normal, constraint.normal) > 0.99
                        && vec_dot(previous[i].impulse, previous[i].normal) > 0.0
                }) {
                let old_a = old_bodies[constraint.a].unwrap();
                let old_b = old_bodies[constraint.b].unwrap();
                let input = vec_sub(bodies[constraint.a].velocity, bodies[constraint.b].velocity);
                let output = vec_sub(old_a.output_velocity, old_b.output_velocity);
                let acceleration = vec_sub(input, output);
                let previous_acceleration = vec_sub(old_a.acceleration, old_b.acceleration);
                let speed = -vec_dot(acceleration, constraint.normal);
                let previous_speed = -vec_dot(previous_acceleration, constraint.normal);
                let tolerance = 32.0
                    * f32::EPSILON
                    * (vec_len(bodies[constraint.a].velocity)
                        + vec_len(bodies[constraint.b].velocity)
                        + vec_len(old_a.output_velocity)
                        + vec_len(old_b.output_velocity)
                        + vec_len(old_a.acceleration)
                        + vec_len(old_b.acceleration))
                    + 1e-6;
                let change = speed - previous_speed;
                let previous_input = vec_sub(old_a.velocity, old_b.velocity);
                let previous_output = vec_sub(previous_input, previous_acceleration);
                let output_change = vec_dot(vec_sub(output, previous_output), constraint.normal);
                // With constant force and velocity damping, the acceleration
                // change opposes, and cannot exceed, the prior velocity change.
                let repeated = change.abs() <= tolerance
                    || (change * output_change >= 0.0
                        && change.abs() <= output_change.abs() + tolerance);
                if repeated {
                    // One explicit velocity change must not become the next
                    // frame's estimate of the ordinary supporting force.
                    let older_speed = -vec_dot(
                        vec_sub(old_a.previous_acceleration, old_b.previous_acceleration),
                        constraint.normal,
                    );
                    speed.min(previous_speed).min(older_speed).max(0.0)
                } else {
                    0.0
                }
            } else {
                0.0
            };
            let friction = ((rc_ref!(&a.collider).friction + rc_ref!(&b.collider).friction) * 0.5)
                .clamp(0.0, 1.0);
            for point in points {
                let anchors = [
                    a.inverse.mul_vec_value(&point),
                    b.inverse.mul_vec_value(&point),
                ];
                let size = *rc_ref!(&rc_ref!(&a.collider).size);
                let other_size = *rc_ref!(&rc_ref!(&b.collider).size);
                let scale = [
                    size.x.abs(),
                    size.y.abs(),
                    size.z.abs(),
                    rc_ref!(&a.collider).radius,
                    other_size.x.abs(),
                    other_size.y.abs(),
                    other_size.z.abs(),
                    rc_ref!(&b.collider).radius,
                ]
                .into_iter()
                .filter(|v| *v > 0.0)
                .fold(f32::INFINITY, f32::min);
                let tolerance = (scale * 0.02).max(0.001);
                let warm = old_range
                    .clone()
                    .filter(|index| {
                        !reused[*index]
                            && vec_dot(previous[*index].normal, constraint.normal) > 0.99
                    })
                    .map(|index| (index, &previous[index]))
                    .filter_map(|(index, old)| {
                        let distance = vec_len_sq(vec_sub(old.anchors[0], anchors[0]))
                            + vec_len_sq(vec_sub(old.anchors[1], anchors[1]));
                        (distance < 2.0 * tolerance * tolerance).then_some((
                            distance,
                            index,
                            old.impulse,
                            old.mass_unit,
                        ))
                    })
                    .min_by(|a, b| a.0.total_cmp(&b.0))
                    .map_or(zero, |(_, index, impulse, old_mass_unit)| {
                        let ratio = old_mass_unit / mass_unit;
                        if !(1e-6..=1e6).contains(&ratio) {
                            return zero;
                        }
                        reused[index] = true;
                        vec_mul(impulse, ratio)
                    });
                let normal_impulse = vec_dot(warm, constraint.normal).max(0.0);
                let tangent_impulse = vec_sub(warm, vec_mul(constraint.normal, normal_impulse));

                let ra = vec_sub(point, a.world.pos_value());
                let rb = vec_sub(point, b.world.pos_value());
                let relative = vec_sub(
                    bodies[constraint.a].point_velocity(ra),
                    bodies[constraint.b].point_velocity(rb),
                );
                let speed = vec_dot(relative, constraint.normal);
                let normal = constraint.normal;
                let axis = if normal.x.abs() < 0.577 {
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    }
                } else {
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    }
                };
                let t0 = vec_cross(normal, axis);
                let t0 = vec_mul(t0, 1.0 / vec_len(t0));
                let t1 = vec_cross(normal, t0);
                let directions = [normal, t0, t1];
                let ba = &bodies[constraint.a];
                let bb = &bodies[constraint.b];
                let angular_a = directions.map(|d| ba.apply_inverse_inertia(vec_cross(ra, d)));
                let angular_b = directions.map(|d| bb.apply_inverse_inertia(vec_cross(rb, d)));
                let response = |i: usize| {
                    vec_add(
                        vec_mul(directions[i], ba.inverse_mass + bb.inverse_mass),
                        vec_add(vec_cross(angular_a[i], ra), vec_cross(angular_b[i], rb)),
                    )
                };
                let normal_k = vec_dot(normal, response(0));
                let k00 = vec_dot(t0, response(1));
                let k01 = vec_dot(t0, response(2));
                let k11 = vec_dot(t1, response(2));
                let determinant = k00 * k11 - k01 * k01;
                let tangent_mass = if determinant > 0.0 {
                    [k11 / determinant, -k01 / determinant, k00 / determinant]
                } else {
                    [0.0; 3]
                };

                let speed_tolerance = if support_speed > 0.0 {
                    32.0 * f32::EPSILON
                        * (vec_len(ba.velocity)
                            + vec_len(bb.velocity)
                            + vec_len(ba.spin) * vec_len(ra)
                            + vec_len(bb.spin) * vec_len(rb)
                            + support_speed)
                        // Contact arms subtract world positions. Include that
                        // rounding error before their normal speeds cancel.
                        + 4.0 * f32::EPSILON
                            * (vec_len(ba.spin) * (vec_len(point) + vec_len(a.world.pos_value()))
                                + vec_len(bb.spin) * (vec_len(point) + vec_len(b.world.pos_value())))
                        + 1e-6
                } else {
                    0.0
                };
                impulses.push(ContactImpulse {
                    pair: constraint.pair,
                    group: groups[if entries[constraint.a].immovable {
                        constraint.b
                    } else {
                        constraint.a
                    }],
                    a: constraint.a,
                    b: constraint.b,
                    normal: constraint.normal,
                    ra,
                    rb,
                    bounce: if constraint.depth < 0.0 {
                        constraint.depth
                    } else if support_speed > 0.0 && -speed <= support_speed + speed_tolerance {
                        // Repeated normal acceleration is support, including
                        // while rolling. A changed approach still rebounds.
                        0.0
                    } else {
                        -speed.min(0.0) * restitution
                    },
                    friction,
                    normal_impulse,
                    tangent_impulse,
                    tangents: [t0, t1],
                    angular_a,
                    angular_b,
                    normal_mass: if normal_k > 0.0 { 1.0 / normal_k } else { 0.0 },
                    tangent_mass,
                    velocity_change: std::array::from_fn(|i| vec_len(response(i))),
                });
            }
            // Contact points can change while the same faces stay at rest.
            // Wake on a new pair or normal, not on a missed warm-start anchor.
            let persistent = old_range
                .clone()
                .any(|i| vec_dot(previous[i].normal, constraint.normal) > 0.99);
            if !persistent {
                quiet[groups[constraint.a]] = 0;
                quiet[groups[constraint.b]] = 0;
            }
            for contact in [&pair.contact_a, &pair.contact_b] {
                let mut contact = rc_mut!(contact);
                contact.delta_velocity = Vec3::zero();
                contact.delta_angular_velocity = Vec3::zero();
            }
        }

        // Disconnected groups converge independently; one slow stack must not
        // keep solving unrelated contacts. Static partners never receive impulses.
        let mut active = vec![false; entries.len()];
        for contact in &impulses {
            active[contact.group] = quiet[contact.group] < 20 || !supported[contact.group];
        }
        let mut changes = vec![0.0_f32; entries.len()];
        // Periodically reconstruct motion from accumulated impulses to bound
        // rounding drift without repeating the full warm start on every sweep.
        for iteration in 0..512 {
            if iteration % 8 == 0 {
                for (i, body) in bodies.iter_mut().enumerate() {
                    if iteration == 0 || active[groups[i]] {
                        (body.velocity, body.spin) = initial_velocities[i];
                    }
                }
                for contact in &impulses {
                    if iteration > 0 && !active[contact.group] {
                        continue;
                    }
                    let impulse = vec_add(
                        vec_mul(contact.normal, contact.normal_impulse),
                        contact.tangent_impulse,
                    );
                    bodies[contact.a].apply_impulse(contact.ra, impulse);
                    bodies[contact.b].apply_impulse(contact.rb, vec_mul(impulse, -1.0));
                }
            }
            changes.fill(0.0);
            for step in 0..impulses.len() {
                let index = if iteration % 2 == 0 {
                    step
                } else {
                    impulses.len() - 1 - step
                };
                let contact = &mut impulses[index];
                let group = contact.group;
                if !active[group] {
                    continue;
                }
                let relative = vec_sub(
                    bodies[contact.a].point_velocity(contact.ra),
                    bodies[contact.b].point_velocity(contact.rb),
                );
                let next = (contact.normal_impulse
                    + (contact.bounce - vec_dot(relative, contact.normal)) * contact.normal_mass)
                    .max(0.0);
                let delta = next - contact.normal_impulse;
                contact.normal_impulse = next;
                let impulse = vec_mul(contact.normal, delta);
                bodies[contact.a].velocity = vec_add(
                    bodies[contact.a].velocity,
                    vec_mul(impulse, bodies[contact.a].inverse_mass),
                );
                bodies[contact.b].velocity = vec_sub(
                    bodies[contact.b].velocity,
                    vec_mul(impulse, bodies[contact.b].inverse_mass),
                );
                bodies[contact.a].spin =
                    vec_add(bodies[contact.a].spin, vec_mul(contact.angular_a[0], delta));
                bodies[contact.b].spin =
                    vec_sub(bodies[contact.b].spin, vec_mul(contact.angular_b[0], delta));
                changes[group] = changes[group].max(delta.abs() * contact.velocity_change[0]);

                let relative = vec_sub(
                    bodies[contact.a].point_velocity(contact.ra),
                    bodies[contact.b].point_velocity(contact.rb),
                );
                let x = vec_dot(relative, contact.tangents[0]);
                let y = vec_dot(relative, contact.tangents[1]);
                let [m00, m01, m11] = contact.tangent_mass;
                let delta = vec_add(
                    vec_mul(contact.tangents[0], -m00 * x - m01 * y),
                    vec_mul(contact.tangents[1], -m01 * x - m11 * y),
                );
                let mut next = vec_add(contact.tangent_impulse, delta);
                let limit = contact.friction * contact.normal_impulse;
                let length = vec_len(next);
                if length > limit {
                    next = vec_mul(next, limit / length);
                }
                let delta = vec_sub(next, contact.tangent_impulse);
                contact.tangent_impulse = next;
                bodies[contact.a].velocity = vec_add(
                    bodies[contact.a].velocity,
                    vec_mul(delta, bodies[contact.a].inverse_mass),
                );
                bodies[contact.b].velocity = vec_sub(
                    bodies[contact.b].velocity,
                    vec_mul(delta, bodies[contact.b].inverse_mass),
                );
                let x = vec_dot(delta, contact.tangents[0]);
                let y = vec_dot(delta, contact.tangents[1]);
                bodies[contact.a].spin = vec_add(
                    bodies[contact.a].spin,
                    vec_add(
                        vec_mul(contact.angular_a[1], x),
                        vec_mul(contact.angular_a[2], y),
                    ),
                );
                bodies[contact.b].spin = vec_sub(
                    bodies[contact.b].spin,
                    vec_add(
                        vec_mul(contact.angular_b[1], x),
                        vec_mul(contact.angular_b[2], y),
                    ),
                );
                changes[group] = changes[group].max(
                    x.abs() * contact.velocity_change[1] + y.abs() * contact.velocity_change[2],
                );
            }
            for (active, change) in active.iter_mut().zip(&changes) {
                *active &= *change >= 1e-6;
            }
            if !active.iter().any(|active| *active) {
                break;
            }
        }

        for (i, body) in bodies.iter().enumerate() {
            // Sleeping groups skipped the solve; cached impulses are not a new
            // estimate of their motion.
            if body.inverse_mass == 0.0 || (quiet[groups[i]] >= 20 && supported[groups[i]]) {
                continue;
            }
            let tolerance = entries[i].margin;
            let radius = vec_len(vec_sub(entries[i].aabb.max, entries[i].aabb.min)) * 0.5;
            if vec_len(body.velocity) + radius * vec_len(body.spin) > tolerance {
                quiet[groups[i]] = 0;
            }
        }

        let mut next_cache = Vec::with_capacity(impulses.len());
        for impulse in impulses {
            let pair = &pairs[impulse.pair];
            let momentum = vec_add(
                vec_mul(impulse.normal, impulse.normal_impulse),
                impulse.tangent_impulse,
            );
            next_cache.push(CachedContact {
                nodes: [
                    std::rc::Rc::downgrade(&entries[impulse.a].node),
                    std::rc::Rc::downgrade(&entries[impulse.b].node),
                ],
                anchors: [
                    entries[impulse.a].inverse.mul_dir_value(&impulse.ra),
                    entries[impulse.b].inverse.mul_dir_value(&impulse.rb),
                ],
                normal: impulse.normal,
                impulse: momentum,
                mass_unit: contact_mass_unit(impulse.a, impulse.b),
            });
            for (index, arm, sign, contact) in [
                (impulse.a, impulse.ra, 1.0, &pair.contact_a),
                (impulse.b, impulse.rb, -1.0, &pair.contact_b),
            ] {
                let body = &bodies[index];
                let momentum = vec_mul(momentum, sign);
                let dv = vec_mul(momentum, body.inverse_mass);
                let spin = body.apply_inverse_inertia(vec_cross(arm, momentum));
                let dw = vec_mul(
                    Self::world_to_node_local_direction(&entries[index].node, spin),
                    1.0_f32.to_degrees(),
                );
                let contact = rc_ref!(contact);
                let mut velocity = rc_mut!(&contact.delta_velocity);
                *velocity = vec_add(*velocity, dv);
                let mut angular = rc_mut!(&contact.delta_angular_velocity);
                *angular = vec_add(*angular, dw);
            }
        }

        let mut resting = vec![false; entries.len()];
        let mut next_bodies = Vec::with_capacity(entries.len());
        for (i, entry) in entries.iter().enumerate() {
            let quiet_frames = if supported[groups[i]] && entry.margin > 0.0 {
                quiet[groups[i]].saturating_add(1).min(20)
            } else {
                0
            };
            resting[i] = quiet_frames >= 20;
            next_bodies.push(CachedBody {
                node: std::rc::Rc::downgrade(&entry.node),
                world: entry.world,
                velocity: initial_velocities[i].0,
                spin: if resting[i] {
                    zero
                } else {
                    initial_velocities[i].1
                },
                output_velocity: if resting[i] { zero } else { bodies[i].velocity },
                acceleration: old_bodies[i].map_or(zero, |old| {
                    vec_sub(initial_velocities[i].0, old.output_velocity)
                }),
                previous_acceleration: old_bodies[i].map_or(zero, |old| old.acceleration),
                material: materials[i],
                quiet_frames,
                tolerance: entry.margin.max(0.001) * 0.1,
                integrated: true,
            });
        }

        // Resting is internal to the solver. A changed velocity, transform,
        // material, support, or a new impact wakes the connected moving bodies.
        // Callers still receive and apply their usual correction values.
        for constraint in constraints {
            let pair = &pairs[constraint.pair];
            for (i, contact) in [
                (constraint.a, &pair.contact_a),
                (constraint.b, &pair.contact_b),
            ] {
                let mut contact = rc_mut!(contact);
                if next_bodies[i].quiet_frames >= 20
                    && old_bodies[i].is_some_and(|old| !old.integrated)
                {
                    contact.depth = 0.0;
                }
                let offset = vec_mul(*rc_ref!(&contact.normal), contact.depth);
                for axis in 0..3 {
                    next_bodies[i].world.data[axis][3] += component(offset, axis);
                }
                if !resting[i] {
                    continue;
                }
                resting[i] = false;
                let mut velocity = rc_mut!(&contact.delta_velocity);
                *velocity = vec_sub(*velocity, bodies[i].velocity);
                let correction = vec_mul(
                    Self::world_to_node_local_direction(&entries[i].node, bodies[i].spin),
                    1.0_f32.to_degrees(),
                );
                let mut spin = rc_mut!(&contact.delta_angular_velocity);
                *spin = vec_sub(*spin, correction);
            }
        }

        ContactCache {
            contacts: next_cache,
            bodies: next_bodies,
        }
    }

    fn resolve_linear_velocities(
        entries: &[ColliderEntry],
        pairs: &[ContactPair],
        constraints: &[ContactConstraint],
    ) {
        let zero = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let response = |constraint: &ContactConstraint, relative: Vec3, old: Vec3| {
            let normal = constraint.normal;
            let normal_speed = (vec_dot(old, normal) + constraint.depth.min(0.0)
                - vec_dot(relative, normal))
            .max(0.0);
            let tangent = vec_sub(old, relative);
            let tangent = vec_sub(tangent, vec_mul(normal, vec_dot(tangent, normal)));
            let friction = ((rc_ref!(&entries[constraint.a].collider).friction
                + rc_ref!(&entries[constraint.b].collider).friction)
                * 0.5)
                .clamp(0.0, 1.0);
            let limit = friction * normal_speed;
            let length = vec_len(tangent);
            let tangent = if length > limit {
                vec_mul(tangent, limit / length)
            } else {
                tangent
            };
            vec_add(vec_mul(normal, normal_speed), tangent)
        };
        let store = |constraint: &ContactConstraint, correction: Vec3| {
            let pair = &pairs[constraint.pair];
            *rc_mut!(&rc_ref!(&pair.contact_a).delta_velocity) =
                vec_mul(correction, constraint.shares.0);
            *rc_mut!(&rc_ref!(&pair.contact_b).delta_velocity) =
                vec_mul(correction, -constraint.shares.1);
        };
        if let [constraint] = constraints {
            let relative = vec_sub(
                entries[constraint.a].velocity,
                entries[constraint.b].velocity,
            );
            store(constraint, response(constraint, relative, zero));
            return;
        }

        let mut velocities: Vec<_> = entries.iter().map(|entry| entry.velocity).collect();
        let mut corrections = vec![zero; constraints.len()];
        for iteration in 0..128 {
            let mut largest_change = 0.0_f32;
            for step in 0..constraints.len() {
                let i = if iteration % 2 == 0 {
                    step
                } else {
                    constraints.len() - 1 - step
                };
                let constraint = &constraints[i];
                let relative = vec_sub(velocities[constraint.a], velocities[constraint.b]);
                let next = response(constraint, relative, corrections[i]);
                let change = vec_sub(next, corrections[i]);
                corrections[i] = next;
                velocities[constraint.a] = vec_add(
                    velocities[constraint.a],
                    vec_mul(change, constraint.shares.0),
                );
                velocities[constraint.b] = vec_sub(
                    velocities[constraint.b],
                    vec_mul(change, constraint.shares.1),
                );
                largest_change = largest_change.max(vec_len_sq(change));
            }
            if largest_change < 1e-12 {
                break;
            }
        }

        for (constraint, correction) in constraints.iter().zip(corrections) {
            store(constraint, correction);
        }
    }

    fn contact_patch(
        a: &ColliderEntry,
        b: &ColliderEntry,
        normal: Vec3,
        point: Vec3,
        depth: f32,
    ) -> Vec<Vec3> {
        let scale = a
            .aabb
            .max
            .x
            .abs()
            .max(a.aabb.max.y.abs())
            .max(a.aabb.max.z.abs())
            .max(1.0);
        let tolerance = (scale * f32::EPSILON * 8.0)
            .max(0.001)
            .max(a.margin.max(b.margin));
        if let Some(points) = Self::box_contact_patch(a, b, normal, tolerance)
            .or_else(|| Self::mesh_contact_patch(a, b, normal, point, tolerance))
            .or_else(|| Self::mesh_contact_patch(b, a, normal, point, tolerance))
            .or_else(|| Self::capsule_mesh_contact_patch(a, b, normal, point, tolerance))
            .or_else(|| {
                Self::capsule_mesh_contact_patch(b, a, vec_mul(normal, -1.0), point, tolerance)
            })
        {
            if !points.is_empty() {
                return reduce_contact_patch(points, normal, tolerance);
            }
        }

        let mut points = Vec::new();
        for (entry, other, sign) in [(a, b, -1.0), (b, a, 1.0)] {
            let collider = rc_ref!(&entry.collider);
            if collider.mesh.is_some() {
                continue;
            }
            let outward = vec_mul(normal, sign);
            let radius = collider.radius.max(0.0);
            let size = *rc_ref!(&collider.size);
            let mut vertices = Vec::new();
            match classify_shape(size, radius) {
                ColliderShape::Sphere { r } => {
                    vertices.push(vec_add(entry.world.pos_value(), vec_mul(outward, r)));
                }
                ColliderShape::Capsule { half_h, r } => {
                    for y in [-half_h, half_h] {
                        vertices.push(vec_add(
                            entry.world.mul_vec_value(&Vec3 { x: 0.0, y, z: 0.0 }),
                            vec_mul(outward, r),
                        ));
                    }
                    // A long capsule can contact the middle of a shorter box:
                    // neither capsule endpoint nor a box corner lies on that
                    // contact line. Clip its ends to the box instead of using
                    // one arbitrary point and tipping a centered impact.
                    let direction = vec_sub(vertices[1], vertices[0]);
                    let other_collider = rc_ref!(&other.collider);
                    if other_collider.mesh.is_none()
                        && vec_dot(direction, normal).abs() <= tolerance
                    {
                        if let ColliderShape::RoundedBox { half, r } =
                            classify_shape(*rc_ref!(&other_collider.size), other_collider.radius)
                        {
                            let ends = [vertices[0], vertices[1]].map(|vertex| {
                                vec_sub(
                                    vertex,
                                    vec_mul(normal, vec_dot(vec_sub(vertex, point), normal)),
                                )
                            });
                            for (start, end) in [(ends[0], ends[1]), (ends[1], ends[0])] {
                                if !Self::contains_contact_point(other, start, tolerance) {
                                    if let Some((_, p, _)) = ray_vs_rounded_box(
                                        other.inverse.mul_vec_value(&start),
                                        other.inverse.mul_dir_value(&vec_sub(end, start)),
                                        half,
                                        r + tolerance,
                                        1.0,
                                    ) {
                                        vertices.push(other.world.mul_vec_value(&p));
                                    }
                                }
                            }
                        }
                    }
                }
                ColliderShape::RoundedBox { half, r } => {
                    for x in [-half.x, half.x] {
                        for y in [-half.y, half.y] {
                            for z in [-half.z, half.z] {
                                vertices.push(vec_add(
                                    entry.world.mul_vec_value(&Vec3 { x, y, z }),
                                    vec_mul(outward, r),
                                ));
                            }
                        }
                    }
                }
            }
            let support = vertices
                .iter()
                .map(|p| vec_dot(*p, outward))
                .fold(f32::NEG_INFINITY, f32::max);
            let tolerance = tolerance.max(
                1e-4 * size
                    .x
                    .abs()
                    .max(size.y.abs())
                    .max(size.z.abs())
                    .max(radius)
                    .max(1.0),
            );
            let reach = depth.abs() + tolerance;
            for vertex in vertices {
                if support - vec_dot(vertex, outward) > reach {
                    continue;
                }
                let projected = vec_sub(
                    vertex,
                    vec_mul(normal, vec_dot(vec_sub(vertex, point), normal)),
                );
                let candidate = if rc_ref!(&other.collider).mesh.is_some() {
                    let origin = vec_sub(vertex, vec_mul(outward, reach));
                    Self::ray_vs_mesh_collider(
                        origin,
                        outward,
                        &other.world,
                        &other.collider,
                        reach * 2.0,
                    )
                    .filter(|(_, _, n)| vec_dot(*n, normal).abs() > 0.99)
                    .map(|(_, p, _)| p)
                } else if Self::contains_contact_point(other, projected, tolerance) {
                    Some(projected)
                } else {
                    None
                };
                if let Some(candidate) = candidate {
                    if points
                        .iter()
                        .all(|p| vec_len_sq(vec_sub(*p, candidate)) > tolerance * tolerance)
                    {
                        points.push(candidate);
                    }
                }
            }
        }

        if points.is_empty() {
            points.push(point);
        }
        points
    }

    fn box_contact_patch(
        a: &ColliderEntry,
        b: &ColliderEntry,
        normal: Vec3,
        tolerance: f32,
    ) -> Option<Vec<Vec3>> {
        let ca = rc_ref!(&a.collider);
        let cb = rc_ref!(&b.collider);
        if ca.mesh.is_some() || cb.mesh.is_some() {
            return None;
        }
        let ColliderShape::RoundedBox { half: ha, r: ra } =
            classify_shape(*rc_ref!(&ca.size), ca.radius)
        else {
            return None;
        };
        let ColliderShape::RoundedBox { half: hb, r: rb } =
            classify_shape(*rc_ref!(&cb.size), cb.radius)
        else {
            return None;
        };

        let na = a.inverse.mul_dir_value(&normal);
        let nb = b.inverse.mul_dir_value(&normal);
        let axis_a = dominant_axis(na);
        let axis_b = dominant_axis(nb);
        let (reference, incident, half, radius, incident_half, incident_radius, outward) =
            if component(na, axis_a).abs() >= component(nb, axis_b).abs() {
                (a, b, ha, ra, hb, rb, vec_mul(normal, -1.0))
            } else {
                (b, a, hb, rb, ha, ra, normal)
            };
        let reference_inv = reference.inverse;
        let local_normal = reference_inv.mul_dir_value(&outward);
        let axis = dominant_axis(local_normal);
        // A cross-edge separating axis has no reference face.
        if component(local_normal, axis).abs() < 0.99 {
            return None;
        }

        let sign = component(local_normal, axis).signum();
        let direction = incident
            .world
            .inverse_value()
            .mul_dir_value(&vec_mul(outward, -1.0));
        let mut polygon: Vec<_> = box_face(incident_half, incident_radius, direction)
            .into_iter()
            .map(|p| reference_inv.mul_vec_value(&incident.world.mul_vec_value(&p)))
            .collect();
        for other in 0..3 {
            if other == axis {
                continue;
            }
            for side in [-1.0, 1.0] {
                polygon =
                    clip_contact_polygon(polygon, other, side, component(half, other) + radius);
            }
        }
        polygon = clip_contact_polygon(
            polygon,
            axis,
            sign,
            component(half, axis) + radius + tolerance,
        );

        for point in &mut polygon {
            let separation = sign * component(*point, axis) - component(half, axis) - radius;
            let coordinate = component(*point, axis) - sign * separation * 0.5;
            set_axis(point, axis, coordinate);
            *point = reference.world.mul_vec_value(point);
        }

        Some(polygon)
    }

    fn mesh_contact_patch(
        body: &ColliderEntry,
        terrain: &ColliderEntry,
        normal: Vec3,
        point: Vec3,
        tolerance: f32,
    ) -> Option<Vec<Vec3>> {
        let collider = rc_ref!(&body.collider);
        if collider.mesh.is_some() {
            return None;
        }
        let ColliderShape::RoundedBox { half, r } =
            classify_shape(*rc_ref!(&collider.size), collider.radius)
        else {
            return None;
        };
        let mesh = rc_ref!(&terrain.collider).mesh.clone()?;
        let inverse = body.inverse;
        let query = transform_aabb_to_local(&terrain.inverse, &body.aabb);
        let mesh = rc_ref!(&mesh);
        let mut points = Vec::new();
        mesh.with_collision_bvh(|bvh| {
            bvh.query_aabb(&query, |indices| {
                let vertices = indices
                    .map(|index| terrain.world.mul_vec_value(&bvh.positions[index as usize]));
                let face = vec_cross(
                    vec_sub(vertices[1], vertices[0]),
                    vec_sub(vertices[2], vertices[0]),
                );
                let Some(face_normal) = normalize_axis(face) else {
                    return;
                };
                if vec_dot(face_normal, normal).abs() < 0.999
                    || vec_dot(vec_sub(vertices[0], point), normal).abs() > tolerance
                {
                    return;
                }
                let mut polygon: Vec<_> = vertices
                    .into_iter()
                    .map(|p| inverse.mul_vec_value(&p))
                    .collect();
                for axis in 0..3 {
                    for sign in [-1.0, 1.0] {
                        polygon = clip_contact_polygon(
                            polygon,
                            axis,
                            sign,
                            component(half, axis) + r + tolerance,
                        );
                    }
                }
                for p in polygon {
                    let p = body.world.mul_vec_value(&p);
                    if points
                        .iter()
                        .all(|q| vec_len_sq(vec_sub(*q, p)) > tolerance * tolerance)
                    {
                        points.push(p);
                    }
                }
            });
        });
        Some(points)
    }

    fn capsule_mesh_contact_patch(
        body: &ColliderEntry,
        terrain: &ColliderEntry,
        normal: Vec3,
        point: Vec3,
        tolerance: f32,
    ) -> Option<Vec<Vec3>> {
        let collider = rc_ref!(&body.collider);
        if collider.mesh.is_some() {
            return None;
        }
        let ColliderShape::Capsule { half_h, r } =
            classify_shape(*rc_ref!(&collider.size), collider.radius)
        else {
            return None;
        };
        let mesh = rc_ref!(&terrain.collider).mesh.clone()?;
        let ends = [-half_h, half_h].map(|y| {
            vec_sub(
                body.world.mul_vec_value(&Vec3 { x: 0.0, y, z: 0.0 }),
                vec_mul(normal, r),
            )
        });
        let direction = vec_sub(ends[1], ends[0]);
        if vec_dot(direction, normal).abs() > tolerance {
            return None;
        }
        let ends = ends.map(|p| vec_sub(p, vec_mul(normal, vec_dot(vec_sub(p, point), normal))));
        let direction = vec_sub(ends[1], ends[0]);
        let query = transform_aabb_to_local(&terrain.inverse, &body.aabb);
        let mut interval: Option<(f32, f32)> = None;
        // The capsule can bridge several triangles with both end caps outside
        // the platform. Keep the extreme supported points of its contact line.
        rc_ref!(&mesh).with_collision_bvh(|bvh| {
            bvh.query_aabb(&query, |indices| {
                let vertices =
                    indices.map(|i| terrain.world.mul_vec_value(&bvh.positions[i as usize]));
                let Some(face_normal) = normalize_axis(vec_cross(
                    vec_sub(vertices[1], vertices[0]),
                    vec_sub(vertices[2], vertices[0]),
                )) else {
                    return;
                };
                if vec_dot(face_normal, normal).abs() < 0.999
                    || vec_dot(vec_sub(vertices[0], point), normal).abs() > tolerance
                {
                    return;
                }
                let mut lo = 0.0_f32;
                let mut hi = 1.0_f32;
                for i in 0..3 {
                    let edge = vec_sub(vertices[(i + 1) % 3], vertices[i]);
                    let distances = ends
                        .map(|p| vec_dot(vec_cross(edge, vec_sub(p, vertices[i])), face_normal));
                    let epsilon = tolerance * vec_len(edge);
                    let [a, b] = distances;
                    if a < -epsilon && b < -epsilon {
                        return;
                    }
                    if a < -epsilon {
                        lo = lo.max(a / (a - b));
                    }
                    if b < -epsilon {
                        hi = hi.min(a / (a - b));
                    }
                    if lo > hi {
                        return;
                    }
                }
                interval = Some(interval.map_or((lo, hi), |(a, b)| (a.min(lo), b.max(hi))));
            });
        });

        interval.map(|(lo, hi)| {
            vec![
                vec_add(ends[0], vec_mul(direction, lo)),
                vec_add(ends[0], vec_mul(direction, hi)),
            ]
        })
    }

    fn contains_contact_point(entry: &ColliderEntry, point: Vec3, tolerance: f32) -> bool {
        let collider = rc_ref!(&entry.collider);
        let p = entry.inverse.mul_vec_value(&point);
        let shape = classify_shape(*rc_ref!(&collider.size), collider.radius.max(0.0));
        let (distance, radius) = match shape {
            ColliderShape::Sphere { r } => (vec_len(p), r),
            ColliderShape::Capsule { half_h, r } => (
                vec_len(Vec3 {
                    x: p.x,
                    y: p.y - p.y.clamp(-half_h, half_h),
                    z: p.z,
                }),
                r,
            ),
            ColliderShape::RoundedBox { half, r } => (
                vec_len(Vec3 {
                    x: (p.x.abs() - half.x).max(0.0),
                    y: (p.y.abs() - half.y).max(0.0),
                    z: (p.z.abs() - half.z).max(0.0),
                }),
                r,
            ),
        };
        distance <= radius + tolerance
    }

    // Update a detached snapshot; callers still apply the returned corrections
    // to their nodes themselves, in notification order.
    fn apply_contact_to_snapshot(world: &mut Mat4, velocity: &mut Vec3, contact: &RcContact) {
        let contact = rc_ref!(contact);
        let normal = rc_ref!(&contact.normal);
        let mut push = Mat4::identity_value();
        push.data[0][3] = normal.x * contact.depth;
        push.data[1][3] = normal.y * contact.depth;
        push.data[2][3] = normal.z * contact.depth;
        *world = push.mul_mat_value(world);
        let delta = rc_ref!(&contact.delta_velocity);
        velocity.x += delta.x;
        velocity.y += delta.y;
        velocity.z += delta.z;
    }

    fn with_collider_entries<R>(
        scene_root: &RcNode,
        swept: bool,
        f: impl FnOnce(&[ColliderEntry]) -> R,
    ) -> R {
        COLLIDER_ENTRY_SCRATCH.with(|scratch| {
            let mut entries = scratch.borrow_mut();
            entries.clear();

            Self::for_each_collider_entry(
                scene_root,
                swept,
                &mut |node, world, aabb, collider, velocity| {
                    let collider_ref = rc_ref!(collider);
                    entries.push(ColliderEntry {
                        node: node.clone(),
                        world: *world,
                        inverse: world.inverse_value(),
                        aabb: *aabb,
                        collider: collider.clone(),
                        velocity,
                        immovable: collider_ref.contact_mass() == 0.0,
                        trigger: collider_ref.trigger,
                        margin: if collider_ref.rolls
                            && !collider_ref.trigger
                            && collider_ref.mesh.is_none()
                        {
                            let size = *rc_ref!(&collider_ref.size);
                            let radius = collider_ref.radius.max(0.0);
                            let scale = [size.x.abs(), size.y.abs(), size.z.abs()]
                                .into_iter()
                                .map(|s| s + radius * 2.0)
                                .filter(|s| *s > 0.0)
                                .fold(f32::INFINITY, f32::min);
                            if scale.is_finite() {
                                scale * 0.001
                            } else {
                                0.0
                            }
                        } else {
                            0.0
                        },
                    });
                },
            );

            let result = f(&entries);
            entries.clear();
            result
        })
    }

    #[cfg(test)]
    fn collider_entry_scratch_capacity() -> usize {
        COLLIDER_ENTRY_SCRATCH.with(|scratch| scratch.borrow().capacity())
    }

    fn for_each_collider_entry(
        node: &RcNode,
        swept: bool,
        f: &mut impl FnMut(&RcNode, &Mat4, &Aabb, &RcCollider, Vec3),
    ) {
        let parent = Node::parent(node);
        if parent
            .as_ref()
            .is_some_and(|parent| !Node::effective_active(parent))
        {
            return;
        }
        let parent_world = parent.as_ref().map(Node::world_transform_value);
        Self::for_each_collider_entry_recursive(node, parent_world.as_ref(), swept, f);
    }

    fn for_each_collider_entry_recursive(
        node: &RcNode,
        parent_world: Option<&Mat4>,
        swept: bool,
        f: &mut impl FnMut(&RcNode, &Mat4, &Aabb, &RcCollider, Vec3),
    ) {
        let node_ref = rc_ref!(node);
        if !node_ref.active {
            return;
        }

        let local = *rc_ref!(&node_ref.transform);
        let world = parent_world.map_or(local, |parent| parent.mul_mat_value(&local));
        if let Some(coll_rc) = &node_ref.collider {
            let coll = rc_ref!(coll_rc);
            let velocity = Self::effective_linear_velocity(parent_world, &coll);
            let mut aabb = collider_aabb(&coll, &world);
            if swept {
                aabb = Self::swept_aabb(aabb, velocity);
            }
            f(node, &world, &aabb, coll_rc, velocity);
        }

        for child in &node_ref.children {
            Self::for_each_collider_entry_recursive(child, Some(&world), swept, f);
        }
    }

    // Integration has already advanced positions; sweeps recover the prior position from velocity.
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
        vel_a: Vec3,
        world_b: &Mat4,
        coll_b: &RcCollider,
        vel_b: Vec3,
    ) -> Option<(ContactGeom, bool)> {
        Self::narrow_phase_with_margin(world_a, coll_a, vel_a, world_b, coll_b, vel_b, 0.0)
    }

    fn narrow_phase_with_margin(
        world_a: &Mat4,
        coll_a: &RcCollider,
        vel_a: Vec3,
        world_b: &Mat4,
        coll_b: &RcCollider,
        vel_b: Vec3,
        margin: f32,
    ) -> Option<(ContactGeom, bool)> {
        use ColliderShape as S;

        let (size_a, mut r_a, mesh_a) = {
            let a = rc_ref!(coll_a);
            let size = *rc_ref!(&a.size);
            (size, a.radius.max(0.0), a.mesh.clone())
        };
        let (size_b, r_b, mesh_b) = {
            let b = rc_ref!(coll_b);
            let size = *rc_ref!(&b.size);
            (size, b.radius.max(0.0), b.mesh.clone())
        };

        let r_b = r_b + if mesh_a.is_some() { margin } else { 0.0 };
        if mesh_a.is_none() {
            r_a += margin;
        }

        // Normalize swapped solvers back to the b → a normal contract.
        let flip = |g: ContactGeom| ContactGeom {
            point: g.point,
            normal: Vec3 {
                x: -g.normal.x,
                y: -g.normal.y,
                z: -g.normal.z,
            },
            depth: g.depth,
        };

        // Mesh-vs-mesh is unsupported: both sides are static terrain
        // with no resolution payload.
        match (mesh_a, mesh_b) {
            (Some(_), Some(_)) => return None,
            (Some(mesh), None) => {
                return Self::narrow_phase_mesh_vs_shape(
                    world_a, &mesh, world_b, size_b, r_b, vel_b,
                )
                .map(|(geom, swept)| (flip(geom), swept));
            }
            (None, Some(mesh)) => {
                return Self::narrow_phase_mesh_vs_shape(
                    world_b, &mesh, world_a, size_a, r_a, vel_a,
                );
            }
            (None, None) => {}
        }

        let c_a = world_a.pos_value();
        let c_b = world_b.pos_value();
        let shapes = (classify_shape(size_a, r_a), classify_shape(size_b, r_b));
        let overlap = match shapes {
            (S::Sphere { r: ra }, S::Sphere { r: rb }) => sphere_vs_sphere(c_a, ra, c_b, rb),
            (S::Sphere { r: ra }, S::RoundedBox { half, r }) => {
                sphere_vs_rounded_obb(c_a, ra, world_b, half, r)
            }
            (S::RoundedBox { half, r }, S::Sphere { r: rb }) => {
                sphere_vs_rounded_obb(c_b, rb, world_a, half, r).map(flip)
            }
            (S::Sphere { r: ra }, S::Capsule { half_h, r }) => {
                capsule_vs_sphere(world_b, half_h, r, c_a, ra)
            }
            (S::Capsule { half_h, r }, S::Sphere { r: rb }) => {
                capsule_vs_sphere(world_a, half_h, r, c_b, rb).map(flip)
            }
            (S::Capsule { half_h: ha, r: ra }, S::Capsule { half_h: hb, r: rb }) => {
                capsule_vs_capsule(world_a, ha, ra, world_b, hb, rb)
            }
            (S::Capsule { half_h, r }, S::RoundedBox { half, r: br }) => {
                capsule_vs_rounded_obb(world_a, half_h, r, world_b, half, br)
            }
            (S::RoundedBox { half, r: br }, S::Capsule { half_h, r }) => {
                capsule_vs_rounded_obb(world_b, half_h, r, world_a, half, br).map(flip)
            }
            (S::RoundedBox { half: ha, r: ra }, S::RoundedBox { half: hb, r: rb }) => {
                rounded_obb_vs_rounded_obb(world_a, ha, ra, world_b, hb, rb)
            }
        };
        // A body can finish on the far side while still overlapping. Preserve
        // resting contacts, but use the entry face for a newly crossed solid.
        let relative = vec_sub(vel_a, vel_b);
        if overlap.is_some_and(|geom| {
            vec_dot(relative, geom.normal) < 0.0 || vec_len_sq(relative) < 1e-12
        }) {
            return overlap.map(|geom| (geom, false));
        }
        match shapes {
            (S::Sphere { r: ra }, S::Sphere { r: rb }) => {
                Self::swept_sphere_vs_sphere(c_a, ra, vel_a, c_b, rb, vel_b)
            }
            (S::Sphere { r: ra }, S::RoundedBox { half, r }) => {
                Self::swept_sphere_vs_rounded_obb(c_a, ra, vel_a, world_b, half, r, vel_b)
            }
            (S::RoundedBox { half, r }, S::Sphere { r: rb }) => {
                Self::swept_sphere_vs_rounded_obb(c_b, rb, vel_b, world_a, half, r, vel_a).map(flip)
            }
            (S::Sphere { r: ra }, S::Capsule { half_h, r }) => {
                Self::swept_sphere_vs_capsule(c_a, ra, vel_a, world_b, half_h, r, vel_b)
            }
            (S::Capsule { half_h, r }, S::Sphere { r: rb }) => {
                Self::swept_sphere_vs_capsule(c_b, rb, vel_b, world_a, half_h, r, vel_a).map(flip)
            }
            (S::Capsule { half_h: ha, r: ra }, S::Capsule { half_h: hb, r: rb }) => {
                Self::swept_capsule_vs_capsule(world_a, ha, ra, vel_a, world_b, hb, rb, vel_b)
            }
            (S::Capsule { half_h, r }, S::RoundedBox { half, r: br }) => {
                Self::swept_capsule_vs_rounded_obb(
                    world_a, half_h, r, vel_a, world_b, half, br, vel_b,
                )
            }
            (S::RoundedBox { half, r: br }, S::Capsule { half_h, r }) => {
                Self::swept_capsule_vs_rounded_obb(
                    world_b, half_h, r, vel_b, world_a, half, br, vel_a,
                )
                .map(flip)
            }
            (S::RoundedBox { half: ha, r: ra }, S::RoundedBox { half: hb, r: rb }) => {
                Self::swept_rounded_obb_vs_rounded_obb(
                    world_a, ha, ra, vel_a, world_b, hb, rb, vel_b,
                )
            }
        }
        .map(|geom| (geom, true))
        .or_else(|| overlap.map(|geom| (geom, false)))
    }

    // Continuous collision helpers

    fn swept_sphere_vs_sphere(
        c_a: Vec3,
        r_a: f32,
        vel_a: Vec3,
        c_b: Vec3,
        r_b: f32,
        vel_b: Vec3,
    ) -> Option<ContactGeom> {
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
            let distance = vec_len(d0);
            let normal = normalize_axis(d0)?;
            let closing = -vec_dot(rel, normal);
            return (closing > 1e-6).then(|| ContactGeom {
                point: vec_add(vec_sub(c_b, vel_b), vec_mul(normal, r_b)),
                normal,
                depth: (r_sum - distance).max(0.0) + closing,
            });
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
            depth: (-vec_dot(rel, normal) * (1.0 - toi)).max(0.0),
        })
    }

    fn swept_sphere_vs_rounded_obb(
        c_sphere: Vec3,
        r_sphere: f32,
        sphere_vel: Vec3,
        box_world: &Mat4,
        half: Vec3,
        box_r: f32,
        box_vel: Vec3,
    ) -> Option<ContactGeom> {
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

        ray_vs_aabb(previous_local, rel_local, &expanded, 1.0)?;
        // The expanded AABB is only a broad filter; its corners extend beyond
        // the box's rounded boundary. A touching start still blocks approach.
        let offset = Vec3 {
            x: (previous_local.x.abs() - half.x).max(0.0),
            y: (previous_local.y.abs() - half.y).max(0.0),
            z: (previous_local.z.abs() - half.z).max(0.0),
        };
        if vec_len_sq(offset) <= reach * reach {
            let core_point = Vec3 {
                x: previous_local.x.clamp(-half.x, half.x),
                y: previous_local.y.clamp(-half.y, half.y),
                z: previous_local.z.clamp(-half.z, half.z),
            };
            let normal_local = normalize_axis(vec_sub(previous_local, core_point))?;
            let normal = box_world.mul_dir_value(&normal_local);
            let closing = -vec_dot(rel_vel, normal);
            return (closing > 1e-6).then(|| ContactGeom {
                point: vec_sub(
                    box_world.mul_vec_value(&vec_add(core_point, vec_mul(normal_local, box_r))),
                    box_vel,
                ),
                normal,
                depth: (reach - vec_len(offset)).max(0.0) + closing,
            });
        }
        let (toi, _, normal_local) =
            ray_vs_rounded_box(previous_local, rel_local, half, reach, 1.0)?;
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
            depth: (-vec_dot(rel_vel, normal) * (1.0 - toi)).max(0.0),
        })
    }

    fn swept_sphere_vs_capsule(
        c_sphere: Vec3,
        r_sphere: f32,
        sphere_vel: Vec3,
        cap_world: &Mat4,
        half_h: f32,
        cap_r: f32,
        cap_vel: Vec3,
    ) -> Option<ContactGeom> {
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
        let (_, on_axis) = closest_points_segment_segment(previous, previous, top, bot);
        let offset = vec_sub(previous, on_axis);
        if vec_len_sq(offset) <= reach * reach {
            let normal = normalize_axis(offset)?;
            let closing = -vec_dot(rel_vel, normal);
            return (closing > 1e-6).then(|| ContactGeom {
                point: vec_sub(vec_add(on_axis, vec_mul(normal, cap_r)), cap_vel),
                normal,
                depth: (reach - vec_len(offset)).max(0.0) + closing,
            });
        }
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

        best.map(|(toi, mut geom)| {
            geom.depth = (-vec_dot(rel_vel, geom.normal) * (1.0 - toi)).max(0.0);
            geom.point = vec_sub(geom.point, vec_mul(cap_vel, 1.0 - toi));
            geom
        })
    }

    fn swept_capsule_vs_capsule(
        world_a: &Mat4,
        half_h_a: f32,
        r_a: f32,
        vel_a: Vec3,
        world_b: &Mat4,
        half_h_b: f32,
        r_b: f32,
        vel_b: Vec3,
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
        .filter(|(_, geom)| vec_dot(rel_vel, geom.normal) < -1e-6)
        .map(|(toi, mut geom)| {
            geom.depth += (-vec_dot(rel_vel, geom.normal) * (1.0 - toi)).max(0.0);
            geom.point = vec_sub(geom.point, vec_mul(vel_b, 1.0 - toi));
            geom
        })
    }

    fn swept_capsule_vs_rounded_obb(
        cap_world: &Mat4,
        half_h: f32,
        cap_r: f32,
        cap_vel: Vec3,
        box_world: &Mat4,
        half: Vec3,
        box_r: f32,
        box_vel: Vec3,
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
        .filter(|(_, geom)| vec_dot(rel_vel, geom.normal) < -1e-6)
        .map(|(toi, mut geom)| {
            geom.depth += (-vec_dot(rel_vel, geom.normal) * (1.0 - toi)).max(0.0);
            geom.point = vec_sub(geom.point, vec_mul(box_vel, 1.0 - toi));
            geom
        })
    }

    fn swept_rounded_obb_vs_rounded_obb(
        world_a: &Mat4,
        half_a: Vec3,
        r_a: f32,
        vel_a: Vec3,
        world_b: &Mat4,
        half_b: Vec3,
        r_b: f32,
        vel_b: Vec3,
    ) -> Option<ContactGeom> {
        let rel_vel = Vec3 {
            x: vel_a.x - vel_b.x,
            y: vel_a.y - vel_b.y,
            z: vel_a.z - vel_b.z,
        };
        swept_obb_vs_obb(world_a, half_a, r_a, rel_vel, world_b, half_b, r_b).map(|mut geom| {
            let closing = -vec_dot(rel_vel, geom.normal);
            if closing > 0.0 {
                geom.point = vec_sub(geom.point, vec_mul(vel_b, (geom.depth / closing).min(1.0)));
            }
            geom
        })
    }

    // Keep ordinary overlap contacts stable. An overlap facing along the motion
    // may be behind a crossed triangle; prefer the swept impact in that case.
    fn narrow_phase_mesh_vs_shape(
        world_mesh: &Mat4,
        mesh: &RcMesh,
        world_shape: &Mat4,
        size_shape: Vec3,
        r_shape: f32,
        shape_vel: Vec3,
    ) -> Option<(ContactGeom, bool)> {
        let overlap =
            narrow_phase_mesh_vs_dynamic(world_mesh, mesh, world_shape, size_shape, r_shape);
        if overlap.is_some_and(|geom| vec_dot(shape_vel, geom.normal) <= 0.0) {
            return overlap.map(|geom| (geom, false));
        }
        Self::swept_sphere_vs_mesh(
            world_mesh,
            mesh,
            world_shape,
            size_shape,
            r_shape,
            shape_vel,
        )
        .or_else(|| {
            Self::swept_capsule_vs_mesh(
                world_mesh,
                mesh,
                world_shape,
                size_shape,
                r_shape,
                shape_vel,
            )
        })
        .or_else(|| {
            Self::swept_rounded_obb_vs_mesh(
                world_mesh,
                mesh,
                world_shape,
                size_shape,
                r_shape,
                shape_vel,
            )
        })
        .map(|geom| (geom, true))
        .or_else(|| overlap.map(|geom| (geom, false)))
    }

    fn swept_sphere_vs_mesh(
        world_mesh: &Mat4,
        mesh: &RcMesh,
        world_sphere: &Mat4,
        size_sphere: Vec3,
        r_sphere: f32,
        sphere_vel: Vec3,
    ) -> Option<ContactGeom> {
        let ColliderShape::Sphere { r } = classify_shape(size_sphere, r_sphere) else {
            return None;
        };
        let rel_vel = sphere_vel;
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

        let tolerance = 4.0 * f32::EPSILON * (vec_len(previous) + r);
        let mesh_inv = world_mesh.inverse_value();
        let query_local = transform_aabb_to_local(&mesh_inv, &swept_world);
        let m = rc_ref!(mesh);
        let mut best: Option<(f32, ContactGeom)> = None;

        m.with_collision_bvh(|bvh| {
            bvh.query_aabb(&query_local, |tri| {
                let v0 = world_mesh.mul_vec_value(&bvh.positions[tri[0] as usize]);
                let v1 = world_mesh.mul_vec_value(&bvh.positions[tri[1] as usize]);
                let v2 = world_mesh.mul_vec_value(&bvh.positions[tri[2] as usize]);

                let Some((toi, geom)) = swept_sphere_vs_triangle(previous, rel_vel, r, v0, v1, v2)
                else {
                    return;
                };
                if !is_blocking_sweep(toi, &geom, rel_vel, tolerance) {
                    return;
                }
                match best {
                    None => best = Some((toi, geom)),
                    Some((prev_toi, _)) if toi < prev_toi => best = Some((toi, geom)),
                    _ => {}
                }
            });
        });

        best.map(|(toi, mut geom)| {
            geom.depth += (-vec_dot(rel_vel, geom.normal) * (1.0 - toi)).max(0.0);
            geom
        })
    }

    fn swept_capsule_vs_mesh(
        world_mesh: &Mat4,
        mesh: &RcMesh,
        world_capsule: &Mat4,
        size_capsule: Vec3,
        r_capsule: f32,
        capsule_vel: Vec3,
    ) -> Option<ContactGeom> {
        let ColliderShape::Capsule { half_h, r } = classify_shape(size_capsule, r_capsule) else {
            return None;
        };
        let rel_vel = capsule_vel;
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

        let tolerance = 4.0 * f32::EPSILON * (vec_len(prev_top).max(vec_len(prev_bot)) + r);
        let mesh_inv = world_mesh.inverse_value();
        let query_local = transform_aabb_to_local(&mesh_inv, &swept_world);
        let m = rc_ref!(mesh);
        let mut best: Option<(f32, ContactGeom)> = None;

        m.with_collision_bvh(|bvh| {
            bvh.query_aabb(&query_local, |tri| {
                let v0 = world_mesh.mul_vec_value(&bvh.positions[tri[0] as usize]);
                let v1 = world_mesh.mul_vec_value(&bvh.positions[tri[1] as usize]);
                let v2 = world_mesh.mul_vec_value(&bvh.positions[tri[2] as usize]);

                let Some((toi, geom)) =
                    swept_segment_vs_triangle(prev_top, prev_bot, rel_vel, r, v0, v1, v2)
                else {
                    return;
                };
                if !is_blocking_sweep(toi, &geom, rel_vel, tolerance) {
                    return;
                }
                match best {
                    None => best = Some((toi, geom)),
                    Some((prev_toi, _)) if toi < prev_toi => best = Some((toi, geom)),
                    _ => {}
                }
            });
        });

        best.map(|(toi, mut geom)| {
            geom.depth = (-vec_dot(rel_vel, geom.normal) * (1.0 - toi)).max(0.0);
            geom
        })
    }

    fn swept_rounded_obb_vs_mesh(
        world_mesh: &Mat4,
        mesh: &RcMesh,
        world_box: &Mat4,
        size_box: Vec3,
        r_box: f32,
        box_vel: Vec3,
    ) -> Option<ContactGeom> {
        let ColliderShape::RoundedBox { half, r } = classify_shape(size_box, r_box) else {
            return None;
        };
        let rel_vel = box_vel;
        let rel_len_sq = rel_vel.x * rel_vel.x + rel_vel.y * rel_vel.y + rel_vel.z * rel_vel.z;
        if rel_len_sq < 1e-12 {
            return None;
        }

        let corners = rounded_obb_corners(world_box, half);
        let swept_world = swept_points_aabb(&corners, rel_vel, r);
        let tolerance = 4.0
            * f32::EPSILON
            * (vec_len(world_box.pos_value()) + vec_len(half) + r + vec_len(rel_vel));
        let mesh_inv = world_mesh.inverse_value();
        let query_local = transform_aabb_to_local(&mesh_inv, &swept_world);
        let m = rc_ref!(mesh);
        let mut best: Option<(f32, ContactGeom)> = None;

        m.with_collision_bvh(|bvh| {
            bvh.query_aabb(&query_local, |tri| {
                let v0 = world_mesh.mul_vec_value(&bvh.positions[tri[0] as usize]);
                let v1 = world_mesh.mul_vec_value(&bvh.positions[tri[1] as usize]);
                let v2 = world_mesh.mul_vec_value(&bvh.positions[tri[2] as usize]);

                let Some((toi, geom)) =
                    swept_obb_vs_triangle(world_box, half, r, rel_vel, v0, v1, v2)
                else {
                    return;
                };
                if !is_blocking_sweep(toi, &geom, rel_vel, tolerance) {
                    return;
                }
                match best {
                    None => best = Some((toi, geom)),
                    Some((prev_toi, _)) if toi < prev_toi => best = Some((toi, geom)),
                    _ => {}
                }
            });
        });

        best.map(|(toi, mut geom)| {
            geom.depth = (-vec_dot(rel_vel, geom.normal) * (1.0 - toi)).max(0.0);
            geom
        })
    }

    // Contact response

    fn build_contact_pair(
        node_a: &RcNode,
        coll_a: &RcCollider,
        node_b: &RcNode,
        coll_b: &RcCollider,
        geom: ContactGeom,
    ) -> ContactPair {
        let a = rc_ref!(coll_a);
        let b = rc_ref!(coll_b);
        let mass_a = a.contact_mass();
        let mass_b = b.contact_mass();
        let trigger_a = a.trigger;
        let trigger_b = b.trigger;

        let (share_a, share_b) = Self::contact_shares(mass_a, mass_b);

        let zero = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let trigger_pair = trigger_a || trigger_b;
        let depth_a = if trigger_pair {
            0.0
        } else {
            geom.depth * share_a
        };
        let depth_b = if trigger_pair {
            0.0
        } else {
            geom.depth * share_b
        };
        let contact_a = Contact::from_values(geom.point, geom.normal, depth_a, zero, zero);
        let contact_b = Contact::from_values(
            geom.point,
            Vec3 {
                x: -geom.normal.x,
                y: -geom.normal.y,
                z: -geom.normal.z,
            },
            depth_b,
            zero,
            zero,
        );
        ContactPair {
            node_a: node_a.clone(),
            node_b: node_b.clone(),
            contact_a,
            contact_b,
        }
    }

    fn contact_shares(mass_a: f32, mass_b: f32) -> (f32, f32) {
        if mass_a == 0.0 && mass_b == 0.0 {
            (0.0, 0.0)
        } else if mass_a == 0.0 {
            (0.0, 1.0)
        } else if mass_b == 0.0 {
            (1.0, 0.0)
        } else {
            let total = mass_a + mass_b;
            if total.is_finite() {
                (mass_b / total, mass_a / total)
            } else {
                let scale = mass_a.max(mass_b);
                let scaled_a = mass_a / scale;
                let scaled_b = mass_b / scale;
                let scaled_total = scaled_a + scaled_b;
                (scaled_b / scaled_total, scaled_a / scaled_total)
            }
        }
    }

    fn world_to_node_local_direction(node: &RcNode, direction: Vec3) -> Vec3 {
        let rotation = Node::world_rotation_value(node);
        Vec3 {
            x: rotation.data[0][0] * direction.x
                + rotation.data[1][0] * direction.y
                + rotation.data[2][0] * direction.z,
            y: rotation.data[0][1] * direction.x
                + rotation.data[1][1] * direction.y
                + rotation.data[2][1] * direction.z,
            z: rotation.data[0][2] * direction.x
                + rotation.data[1][2] * direction.y
                + rotation.data[2][2] * direction.z,
        }
    }

    // Spatial queries

    pub fn raycast(
        scene_root: &RcNode,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        hit_triggers: bool,
        tags_filter: Option<&[String]>,
    ) -> Option<RaycastHitInfo> {
        let direction = Self::normalize_ray_direction(direction)?;
        let mut best: Option<RaycastHitInfo> = None;

        Self::for_each_collider_entry(scene_root, false, &mut |node, world, aabb, collider, _| {
            let Some(hit) = Self::ray_test_one(
                node,
                collider,
                world,
                aabb,
                origin,
                direction,
                max_distance,
                hit_triggers,
                tags_filter,
            ) else {
                return;
            };
            if best.as_ref().is_none_or(|b| hit.distance < b.distance) {
                best = Some(hit);
            }
        });

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
        let mut hits: Vec<RaycastHitInfo> = Vec::new();

        Self::for_each_collider_entry(scene_root, false, &mut |node, world, aabb, collider, _| {
            if let Some(hit) = Self::ray_test_one(
                node,
                collider,
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
        });

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
        coll_rc: &RcCollider,
        world: &Mat4,
        aabb: &Aabb,
        origin: Vec3,
        direction: Vec3,
        max_distance: f32,
        hit_triggers: bool,
        tags_filter: Option<&[String]>,
    ) -> Option<RaycastHitInfo> {
        let coll = rc_ref!(coll_rc);
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
            Self::ray_vs_mesh_collider(origin, direction, world, coll_rc, max_distance)
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
        let radius = radius.max(0.0);
        let probe = Aabb::from_sphere(center, radius);
        let mut out: Vec<RcNode> = Vec::new();

        // Test broad-phase candidates against the sphere
        Self::for_each_collider_entry(scene_root, false, &mut |node, world, aabb, coll_rc, _| {
            let coll = rc_ref!(coll_rc);
            if !hit_triggers && coll.trigger {
                return;
            }
            let size = *rc_ref!(&coll.size);
            let r_other = coll.radius.max(0.0);
            let has_mesh = coll.mesh.is_some();
            if !Self::tags_match(node, tags_filter) {
                return;
            }
            if !probe.overlaps(aabb) {
                return;
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
        });

        out
    }

    pub fn overlap_box(
        scene_root: &RcNode,
        transform: &Mat4,
        size: Vec3,
        hit_triggers: bool,
        tags_filter: Option<&[String]>,
    ) -> Vec<RcNode> {
        let probe = Aabb::from_rounded_box(transform, size, 0.0);
        let probe_half = Vec3 {
            x: size.x.abs() * 0.5,
            y: size.y.abs() * 0.5,
            z: size.z.abs() * 0.5,
        };
        let mut out: Vec<RcNode> = Vec::new();

        // Test broad-phase candidates against the box
        Self::for_each_collider_entry(scene_root, false, &mut |node, world, aabb, coll_rc, _| {
            let coll = rc_ref!(coll_rc);
            if !hit_triggers && coll.trigger {
                return;
            }
            let other_size = *rc_ref!(&coll.size);
            let r_other = coll.radius.max(0.0);
            let has_mesh = coll.mesh.is_some();
            if !Self::tags_match(node, tags_filter) {
                return;
            }
            if !probe.overlaps(aabb) {
                return;
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
        });

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
        let node_tags = &rc_ref!(node).tags;
        tags.iter().any(|t| node_tags.contains(t))
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
            if let Some((t, point, normal)) = ray_vs_sphere_filtered(
                local_origin,
                local_direction,
                center,
                r,
                max_distance,
                |point| {
                    if center.y > 0.0 {
                        point.y >= half_h
                    } else {
                        point.y <= -half_h
                    }
                },
            ) {
                let hit = (t, world.mul_vec_value(&point), world.mul_dir_value(&normal));
                if best.as_ref().is_none_or(|(best_t, _, _)| t < *best_t) {
                    best = Some(hit);
                }
            }
        }

        // Side wall: infinite-cylinder solve in local XZ, hits clamped to the cap span.
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

// Ray-vs-collider intersection helpers

fn ray_vs_rounded_box(
    origin: Vec3,
    direction: Vec3,
    half: Vec3,
    radius: f32,
    max_distance: f32,
) -> Option<(f32, Vec3, Vec3)> {
    let r = radius.max(0.0);
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

    if r <= 0.0 {
        return best;
    }

    // Only each edge capsule's outward quarter belongs to the box surface;
    // its endpoint hemispheres supply the rounded corners.
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

                if let Some(hit) =
                    ray_vs_segment_capsule(origin, direction, a, b, r, max_distance, |point| {
                        component(point, other0) * sign0 >= component(half, other0)
                            && component(point, other1) * sign1 >= component(half, other1)
                    })
                {
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
    accepts: impl Fn(Vec3) -> bool,
) -> Option<(f32, Vec3, Vec3)> {
    let axis = vec_sub(b, a);
    let mut best: Option<(f32, Vec3, Vec3)> = None;
    for (center, sign) in [(a, -1.0), (b, 1.0)] {
        if let Some(hit) =
            ray_vs_sphere_filtered(origin, direction, center, radius, max_distance, |point| {
                vec_dot(vec_sub(point, center), axis) * sign >= 0.0 && accepts(point)
            })
        {
            set_nearer_hit(&mut best, hit);
        }
    }

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
        if !accepts(point) {
            continue;
        }
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

// Contact material and patch helpers

fn contact_material(collider: &Collider) -> [f32; 8] {
    let size = *rc_ref!(&collider.size);
    [
        collider.mass,
        collider.friction,
        collider.restitution,
        collider.radius,
        size.x,
        size.y,
        size.z,
        f32::from(collider.rolls),
    ]
}

fn dominant_axis(v: Vec3) -> usize {
    let mut axis = 0;
    for i in 1..3 {
        if component(v, i).abs() > component(v, axis).abs() {
            axis = i;
        }
    }
    axis
}

fn box_face(half: Vec3, radius: f32, direction: Vec3) -> Vec<Vec3> {
    let axis = dominant_axis(direction);
    let u = (axis + 1) % 3;
    let v = (axis + 2) % 3;
    let mut points = Vec::with_capacity(4);
    for (a, b) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
        let mut p = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        set_axis(
            &mut p,
            axis,
            component(direction, axis).signum() * (component(half, axis) + radius),
        );
        set_axis(&mut p, u, a * component(half, u));
        set_axis(&mut p, v, b * component(half, v));
        points.push(p);
    }
    points
}

fn clip_contact_polygon(polygon: Vec<Vec3>, axis: usize, sign: f32, limit: f32) -> Vec<Vec3> {
    let Some(mut previous) = polygon.last().copied() else {
        return polygon;
    };
    let mut output = Vec::with_capacity(polygon.len() + 1);
    let mut previous_distance = sign * component(previous, axis) - limit;
    for current in polygon {
        let distance = sign * component(current, axis) - limit;
        if (distance <= 0.0) != (previous_distance <= 0.0) {
            let fraction = previous_distance / (previous_distance - distance);
            output.push(vec_add(
                previous,
                vec_mul(vec_sub(current, previous), fraction),
            ));
        }
        if distance <= 0.0 {
            output.push(current);
        }
        previous = current;
        previous_distance = distance;
    }
    output
}

fn contact_group(groups: &[usize], mut index: usize) -> usize {
    while groups[index] != index {
        index = groups[index];
    }
    index
}

// Triangulation adds interior and collinear points without changing support.
// Reduce the surface boundary first, so those points cannot replace its corners.
fn reduce_contact_patch(mut points: Vec<Vec3>, normal: Vec3, tolerance: f32) -> Vec<Vec3> {
    if points.len() <= 4 {
        return points;
    }
    let axis = dominant_axis(normal);
    let u = (axis + 1) % 3;
    let v = (axis + 2) % 3;
    points.sort_by(|a, b| {
        component(*a, u)
            .total_cmp(&component(*b, u))
            .then_with(|| component(*a, v).total_cmp(&component(*b, v)))
    });

    let mut hull: Vec<Vec3> = Vec::with_capacity(points.len() * 2);
    for reverse in [false, true] {
        let first = hull.len();
        for i in 0..points.len() {
            let point = points[if reverse { points.len() - 1 - i } else { i }];
            while hull.len() >= first + 2 {
                let a = hull[hull.len() - 2];
                let b = hull[hull.len() - 1];
                let edge = vec_sub(b, a);
                let next = vec_sub(point, b);
                let area = component(edge, u) * component(next, v)
                    - component(edge, v) * component(next, u);
                if area > tolerance * vec_len(vec_sub(point, a)) {
                    break;
                }
                hull.pop();
            }
            hull.push(point);
        }
        hull.pop();
    }
    points = hull;
    if points.len() <= 4 {
        return points;
    }

    let center = vec_mul(
        points.iter().copied().fold(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            vec_add,
        ),
        1.0 / points.len() as f32,
    );
    let (first, _) = points
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            vec_len_sq(vec_sub(**a, center)).total_cmp(&vec_len_sq(vec_sub(**b, center)))
        })
        .unwrap();
    let mut result = vec![points.remove(first)];
    while result.len() < 4 && !points.is_empty() {
        let (index, _) = points
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let nearest = result
                    .iter()
                    .map(|other| vec_len_sq(vec_sub(*point, *other)))
                    .fold(f32::INFINITY, f32::min);
                (i, nearest)
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap();
        result.push(points.remove(index));
    }
    result
}

// Mesh narrow-phase helpers

// Mesh-vs-dynamic contact in world space, with the normal pointing toward the dynamic body.
fn narrow_phase_mesh_vs_dynamic(
    world_mesh: &Mat4,
    mesh: &RcMesh,
    world_dyn: &Mat4,
    size_dyn: Vec3,
    r_dyn: f32,
) -> Option<ContactGeom> {
    let shape_dyn = classify_shape(size_dyn, r_dyn);
    let mesh_inv = world_mesh.inverse_value();
    // Map the dynamic body's AABB into the mesh-local BVH as a broad filter.
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
            let v0 = world_mesh.mul_vec_value(&v0_local);
            let v1 = world_mesh.mul_vec_value(&v1_local);
            let v2 = world_mesh.mul_vec_value(&v2_local);

            let hit = match shape_dyn {
                ColliderShape::Sphere { r } => sphere_vs_triangle(dyn_center, r, v0, v1, v2),
                ColliderShape::Capsule { half_h, r } => {
                    capsule_vs_triangle(world_dyn, half_h, r, v0, v1, v2)
                }

                ColliderShape::RoundedBox { half, r } => {
                    // Solve in the body-local frame where the box is
                    // axis-aligned, then map the contact back to world
                    // (rotation + translation only, so depth carries).
                    let l0 = dyn_inv.mul_vec_value(&v0);
                    let l1 = dyn_inv.mul_vec_value(&v1);
                    let l2 = dyn_inv.mul_vec_value(&v2);
                    local_box_vs_triangle(half, r, l0, l1, l2).map(|g| ContactGeom {
                        point: world_dyn.mul_vec_value(&g.point),
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
        let local = inv.mul_vec_value(c);
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

// Swept segment / OBB intersection helpers

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

    // Conservatively advance to the segment pair's first contact
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
            depth: (reach - vec_len(vec_sub(on_a, on_b))).max(0.0),
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

    // Conservatively advance to the segment/box first contact
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
            depth: (reach - vec_len(vec_sub(on_seg, on_box))).max(0.0),
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

    // Conservatively advance to the segment/triangle first contact
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

    // Intersect the swept interval across all OBB axes
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

        if axis_entry > entry || (entry_normal.is_none() && axis_entry == entry) {
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
    if r_sum > 0.0 {
        let pair = ObbPairDistance::new(world_a, half_a, world_b, half_b);
        let points_at = |t: f32| pair.closest_points(vec_mul(velocity, t - 1.0));
        let (on_a, on_b) = points_at(entry);
        if vec_len_sq(vec_sub(on_a, on_b)) <= r_sum * r_sum {
            // Initial contact blocks approach, but never an exit or a slide.
            if entry == 0.0 && vec_dot(velocity, vec_sub(on_a, on_b)) >= 0.0 {
                return None;
            }
        } else {
            // Distance between translating convex cores is convex in time.
            // Its derivative has the sign of velocity · (on_a - on_b).
            // Find the minimum inside the conservative SAT interval first;
            // a rounded contact can occur after its false SAT entry.
            let mut lo = entry;
            let mut hi = exit;
            for _ in 0..32 {
                let mid = (lo + hi) * 0.5;
                if mid == lo || mid == hi {
                    break;
                }
                let (on_a, on_b) = points_at(mid);
                let delta = vec_sub(on_a, on_b);
                if vec_len_sq(delta) <= r_sum * r_sum {
                    hi = mid;
                    break; // A real contact already brackets the first hit.
                }
                if vec_dot(velocity, delta) < 0.0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            let (lo_a, lo_b) = points_at(lo);
            let (hi_a, hi_b) = points_at(hi);
            let lo_dist2 = vec_len_sq(vec_sub(lo_a, lo_b));
            let hi_dist2 = vec_len_sq(vec_sub(hi_a, hi_b));
            let minimum = if lo_dist2 < hi_dist2 { lo } else { hi };
            if lo_dist2.min(hi_dist2) > r_sum * r_sum {
                return None;
            }

            // Bisect the decreasing part to the first contact. Stop when
            // f32 time has no representable interior point, or after 32
            // halvings of a frame interval, without enlarging the radius.
            lo = entry;
            hi = minimum;
            for _ in 0..32 {
                let mid = (lo + hi) * 0.5;
                if mid == lo || mid == hi {
                    break;
                }
                let (on_a, on_b) = points_at(mid);
                if vec_len_sq(vec_sub(on_a, on_b)) <= r_sum * r_sum {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            entry = hi;
        }
        let (on_a, on_b) = points_at(entry);
        let normal = normalized_or(vec_sub(on_a, on_b), vec_mul(velocity, -1.0))?;
        let closing = -vec_dot(velocity, normal);
        if closing <= 1e-6 {
            return None;
        }
        return Some(ContactGeom {
            point: vec_add(on_b, vec_mul(normal, r_b.max(0.0))),
            normal,
            depth: (r_sum - vec_len(vec_sub(on_a, on_b))).max(0.0) + closing * (1.0 - entry),
        });
    }
    let normal = entry_normal?;
    let support_b = obb_projection_radius(&bx, &half_b_arr, normal) + r_b.max(0.0);
    Some(ContactGeom {
        point: vec_add(cb, vec_mul(normal, support_b)),
        normal,
        depth: (-vec_dot(velocity, normal) * (1.0 - entry)).max(0.0),
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

    // Intersect the swept interval across box/triangle SAT axes
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

// SAT axis and projection helpers

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

// File-private raw Vec3 math

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

// Swept-sphere intersection helpers

// A zero-time hit already supports the body. Reconstructing its start point
// and normal after sliding introduces coordinate rounding; that tiny correction
// must not mask another triangle farther along the remaining motion.
fn is_blocking_sweep(toi: f32, geom: &ContactGeom, velocity: Vec3, tolerance: f32) -> bool {
    let closing = -vec_dot(velocity, geom.normal);
    closing > 0.0 && (toi > 0.0 || geom.depth + closing > tolerance)
}

fn swept_sphere_vs_triangle(
    previous_center: Vec3,
    velocity: Vec3,
    radius: f32,
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
) -> Option<(f32, ContactGeom)> {
    let tolerance = 4.0 * f32::EPSILON * (vec_len(previous_center) + radius);
    if let Some(mut contact) = sphere_vs_triangle(previous_center, radius + tolerance, v0, v1, v2) {
        if vec_dot(velocity, contact.normal) >= 0.0 {
            return None;
        }
        contact.depth = (contact.depth - tolerance).max(0.0);
        return Some((0.0, contact));
    }
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
    // A long mesh edge can dwarf the moving body. Keep the projection and
    // quadratic in f64 so a tangential motion cannot become a false impact.
    let coordinates = |v: Vec3| [f64::from(v.x), f64::from(v.y), f64::from(v.z)];
    let start = coordinates(previous_center);
    let motion = coordinates(velocity);
    let origin = coordinates(a);
    let end = coordinates(b);
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let axis = std::array::from_fn(|i| end[i] - origin[i]);
    let len_sq = dot(axis, axis);
    if len_sq < 1e-12 {
        return swept_sphere_vs_point(previous_center, velocity, radius, a);
    }

    let len = len_sq.sqrt();
    let unit: [f64; 3] = std::array::from_fn(|i| axis[i] / len);
    let relative = std::array::from_fn(|i| start[i] - origin[i]);
    let s0 = dot(relative, unit);
    let sv = dot(motion, unit);
    let q0 = std::array::from_fn(|i| relative[i] - unit[i] * s0);
    let qv = std::array::from_fn(|i| motion[i] - unit[i] * sv);
    let qa = dot(qv, qv);
    if qa < 1e-12 {
        return None;
    }

    let qb = 2.0 * dot(q0, qv);
    let qc = dot(q0, q0) - f64::from(radius).powi(2);
    if qc <= 0.0 {
        return None;
    }
    let disc = qb * qb - 4.0 * qa * qc;
    if disc < 0.0 {
        return None;
    }

    for toi in [
        (-qb - disc.sqrt()) / (2.0 * qa),
        (-qb + disc.sqrt()) / (2.0 * qa),
    ] {
        if !(0.0..=1.0).contains(&toi) {
            continue;
        }
        let s = s0 + sv * toi;
        if s < -1e-5 || s > len + 1e-5 {
            continue;
        }
        let point: [f64; 3] = std::array::from_fn(|i| origin[i] + unit[i] * s.clamp(0.0, len));
        let normal = std::array::from_fn(|i| start[i] + motion[i] * toi - point[i]);
        let normal_len = dot(normal, normal).sqrt();
        if normal_len < 1e-12 {
            continue;
        }
        return Some((
            toi as f32,
            ContactGeom {
                point: Vec3 {
                    x: point[0] as f32,
                    y: point[1] as f32,
                    z: point[2] as f32,
                },
                normal: Vec3 {
                    x: (normal[0] / normal_len) as f32,
                    y: (normal[1] / normal_len) as f32,
                    z: (normal[2] / normal_len) as f32,
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
    let (toi, mut geom) = swept_sphere_vs_segment(previous_center, velocity, reach, a, b)?;
    geom.point = vec_add(geom.point, vec_mul(geom.normal, capsule_radius));
    Some((toi, geom))
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
    use std::collections::HashSet;

    use super::*;

    fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    // 1e-4 exceeds the f32 rounding of these unit-scale values and stays below any expected difference
    fn assert_vec3_close(actual: Vec3, expected: Vec3) {
        assert!(
            (actual.x - expected.x).abs() < 1e-4
                && (actual.y - expected.y).abs() < 1e-4
                && (actual.z - expected.z).abs() < 1e-4,
            "actual={actual:?} expected={expected:?}"
        );
    }

    #[test]
    fn test_contact_reduction_keeps_corners_when_triangles_split_edges() {
        let corners = [
            vec3(-8.0, 0.0, -3.0),
            vec3(-8.0, 0.0, 3.0),
            vec3(8.0, 0.0, 3.0),
            vec3(8.0, 0.0, -3.0),
        ];
        for axis in 0..3 {
            let rotate = |p: Vec3| match axis {
                0 => p,
                1 => vec3(p.y, p.z, p.x),
                _ => vec3(p.z, p.x, p.y),
            };
            let expected = corners.map(rotate);
            let mut points = expected.to_vec();
            points.extend(
                [
                    vec3(0.0, 0.0, -3.0),
                    vec3(0.0, 0.0, 3.0),
                    vec3(0.0, 0.0, 0.0),
                ]
                .map(rotate),
            );
            for reverse in [false, true] {
                if reverse {
                    points.reverse();
                }
                for _ in 0..points.len() {
                    points.rotate_left(1);
                    let patch =
                        reduce_contact_patch(points.clone(), rotate(vec3(0.0, 1.0, 0.0)), 0.006);
                    assert_eq!(patch.len(), 4);
                    assert!(expected.iter().all(|p| patch.contains(p)));
                }
            }
        }
    }

    #[test]
    fn test_mesh_triangulation_preserves_a_resting_stack() {
        for (nx, nz, shift) in [(1, 1, 0.0), (6, 3, 0.0), (6, 3, 1.0)] {
            let root = mesh_floor_root();
            let floor = rc_ref!(&root).children[0].clone();
            let collider = rc_ref!(&floor).collider.clone().unwrap();
            let mesh = rc_ref!(&collider).mesh.clone().unwrap();
            let primitive = rc_ref!(&mesh).primitives[0].clone().unwrap();
            {
                let mut primitive = rc_mut!(&primitive);
                primitive.positions.clear();
                primitive.indices.clear();
                for x in 0..nx {
                    for z in 0..nz {
                        let x0 = -115.0 + 230.0 * x as f32 / nx as f32;
                        let x1 = -115.0 + 230.0 * (x + 1) as f32 / nx as f32;
                        let z0 = -80.0 + 230.0 * z as f32 / nz as f32;
                        let z1 = -80.0 + 230.0 * (z + 1) as f32 / nz as f32;
                        let i = primitive.positions.len() as i32 / 3;
                        primitive
                            .positions
                            .extend([x0, 0.0, z0, x0, 0.0, z1, x1, 0.0, z1, x1, 0.0, z0]);
                        primitive.indices.extend([i, i + 1, i + 2, i, i + 2, i + 3]);
                    }
                }
            }

            let mut bodies = Vec::new();
            let mut starts = Vec::new();
            for (size, radius, mass, pos) in [
                ((16.0, 28.0, 6.0), 0.0, 0.5, (43.0, 14.0, 60.0)),
                ((16.0, 28.0, 6.0), 0.0, 0.5, (81.0, 14.0, 60.0)),
                ((56.0, 8.0, 20.0), 0.0, 2.0, (62.0, 32.0, 60.0)),
                ((0.0, 0.0, 0.0), 8.0, 1.0, (62.0, 44.0, 60.0)),
            ] {
                let body = Node::new();
                let start = vec3(pos.0 + shift, pos.1, pos.2);
                place_at(&body, start.x, start.y, start.z);
                let collider = box_family_collider(Vec3::new(size.0, size.1, size.2), radius, mass);
                rc_mut!(&collider).rolls = true;
                rc_mut!(&collider).friction = 0.2;
                rc_mut!(&body).collider = Some(collider);
                Node::add_child(&root, &body);
                bodies.push(body);
                starts.push(start);
            }

            for _ in 0..1800 {
                advance_rigid_bodies(&root, &bodies);
            }

            for (body, start) in bodies.iter().zip(starts) {
                let pos = Node::world_transform_value(body).pos_value();
                assert!(
                    vec_len(vec_sub(pos, start)) < 0.03,
                    "nx={nx}, nz={nz}, shift={shift}, start={start:?}, pos={pos:?}"
                );
                let collider = rc_ref!(body).collider.clone().unwrap();
                assert!(vec_len(*rc_ref!(&rc_ref!(&collider).velocity)) < 1e-5);
            }
        }
    }

    #[test]
    fn test_swept_rounded_obb_rejects_corner_gap_and_finds_later_contact() {
        let target = Mat4::identity_value();
        let half = vec3(1.0, 1.0, 1.0);
        let sweep = |offset| {
            let mut end = target;
            end.data[0][3] = 5.0;
            end.data[1][3] = offset;
            end.data[2][3] = offset;
            swept_obb_vs_obb(&end, half, 1.0, vec3(10.0, 0.0, 0.0), &target, half, 1.0)
        };
        assert!(sweep(3.5).is_none());
        let contact = sweep(3.0).unwrap();
        assert!((contact.depth - (1.0 + 7.0 * 0.5_f32.sqrt())).abs() < 1e-4);
        assert_vec3_close(contact.normal, vec3(-0.5_f32.sqrt(), 0.5, 0.5));
        assert_vec3_close(contact.point, vec3(-1.0 - 0.5_f32.sqrt(), 1.5, 1.5));

        // The start can already be inside the expanded SAT projections
        // yet outside the actual rounding, before a later real entry.
        let mut end = target;
        end.data[0][3] = 5.0;
        end.data[1][3] = 3.0;
        end.data[2][3] = 3.0;
        let later =
            swept_obb_vs_obb(&end, half, 1.0, vec3(8.5, 0.0, 0.0), &target, half, 1.0).unwrap();
        assert_vec3_close(later.normal, contact.normal);

        // Endpoint touching is included by the swept test.
        end.data[0][3] = -4.0;
        end.data[1][3] = 0.0;
        end.data[2][3] = 0.0;
        let endpoint =
            swept_obb_vs_obb(&end, half, 1.0, vec3(1.0, 0.0, 0.0), &target, half, 1.0).unwrap();
        assert_vec3_close(endpoint.normal, vec3(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_swept_rounded_obb_grazing_and_narrow_crossing() {
        let target = Mat4::identity_value();
        let half = vec3(1.0, 1.0, 1.0);
        // The line through (3.5, 4, 0) in direction (4, -3, 0) is
        // tangent to the radius-2.5 circle around the core corner (2,2).
        // Both frame endpoints are outside even for the narrow crossing.
        for (shift, hit) in [(0.001, false), (0.0, true), (-0.0001, true)] {
            let mut end = target;
            end.data[0][3] = 7.5 + 0.6 * shift;
            end.data[1][3] = 1.0 + 0.8 * shift;
            let contact =
                swept_obb_vs_obb(&end, half, 1.25, vec3(8.0, -6.0, 0.0), &target, half, 1.25);
            assert_eq!(contact.is_some(), hit, "shift={shift}");
            if shift == 0.0 {
                // Tangent time is ill-conditioned: f32 distance rounding
                // produces an angular error of order sqrt(EPSILON). Check
                // angular alignment rather than crossing-contact tolerance.
                let alignment = vec_dot(contact.unwrap().normal, vec3(0.6, 0.8, 0.0));
                assert!(alignment >= 1.0 - 4.0 * f32::EPSILON);
            }
        }

        // A separating move from an initially overlapping pair is not
        // reported as a new swept entry; sharp boxes use the SAT path.
        let mut end = target;
        end.data[0][3] = 5.0;
        assert!(
            swept_obb_vs_obb(&end, half, 0.1, vec3(4.0, 0.0, 0.0), &target, half, 0.1).is_none()
        );
        assert!(
            swept_obb_vs_obb(&end, half, 0.0, vec3(10.0, 0.0, 0.0), &target, half, 0.0).is_some()
        );
    }

    #[test]
    fn test_with_draw_context_outside_scope_returns_none() {
        let result = with_draw_context(|_| 42);
        assert!(result.is_none());
    }

    #[test]
    fn test_reset_draw_state_outside_scope_is_noop() {
        reset_draw_state();
        assert!(with_draw_context(|_| ()).is_none());
    }

    #[test]
    fn test_closest_points_segment_triangle_crossing_face() {
        let zero = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v0 = Vec3 {
            x: -10.0,
            y: 0.0,
            z: -10.0,
        };
        let v1 = Vec3 {
            x: 10.0,
            y: 0.0,
            z: -10.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        };
        let top = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let bottom = Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };

        for (a, b) in [(top, bottom), (bottom, top), (zero, zero)] {
            assert_eq!(
                closest_points_segment_triangle(a, b, v0, v1, v2),
                (zero, zero)
            );
        }

        let (toi, contact) = swept_segment_vs_triangle(
            top,
            bottom,
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            0.1,
            v0,
            v1,
            v2,
        )
        .expect("an already crossing segment contacts at the start of its sweep");
        assert_eq!(toi, 0.0);
        assert_eq!(contact.point, zero);
        assert_eq!(contact.depth, 0.0);
    }

    #[test]
    fn test_sphere_above_mesh_floor_generates_contact() {
        use crate::cube::collider::Collider;

        let root = mesh_floor_root();
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
            0.0,
            crate::cube::Vec3::zero(),
            0.0,
            0.0,
        ));
        Node::add_child(&root, &ball_node);

        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
    }

    #[test]
    fn test_mesh_corner_resolves_floor_and_wall_for_each_shape() {
        for (size, radius, start, expected) in [
            ((0.0, 0.0, 0.0), 0.5, (0.4, 0.3), (0.5, 0.5)),
            ((0.0, 1.0, 0.0), 0.5, (0.4, 0.9), (0.5, 1.0)),
            ((1.0, 1.0, 1.0), 0.0, (0.4, 0.3), (0.5, 0.5)),
            ((1.0, 1.0, 1.0), 0.1, (0.5, 0.4), (0.6, 0.6)),
        ] {
            for reverse in [false, true] {
                let root = mesh_corner_root();
                let body = Node::new();
                place_at(&body, start.0, start.1, 0.0);
                let collider = box_family_collider(Vec3::new(size.0, size.1, size.2), radius, 1.0);
                rc_mut!(&collider).friction = 0.0;
                rc_mut!(&collider).velocity = Vec3::new(-0.2, -0.3, 0.4);
                rc_mut!(&body).collider = Some(collider);
                Node::add_child(&root, &body);
                if reverse {
                    rc_mut!(&root).children.reverse();
                }

                let pairs = Scene::detect_contacts(&root);
                assert_eq!(pairs.len(), 2, "size={size:?}, radius={radius}");
                let mut position = (start.0, start.1, 0.0);
                let mut velocity = (-0.2, -0.3, 0.4);
                for pair in pairs {
                    let contact = if std::rc::Rc::ptr_eq(&pair.node_a, &body) {
                        rc_ref!(&pair.contact_a)
                    } else {
                        rc_ref!(&pair.contact_b)
                    };
                    let normal = rc_ref!(&contact.normal);
                    position.0 += normal.x * contact.depth;
                    position.1 += normal.y * contact.depth;
                    position.2 += normal.z * contact.depth;
                    let delta = rc_ref!(&contact.delta_velocity);
                    velocity.0 += delta.x;
                    velocity.1 += delta.y;
                    velocity.2 += delta.z;
                }
                assert!((position.0 - expected.0).abs() < 1e-5, "{position:?}");
                assert!((position.1 - expected.1).abs() < 1e-5, "{position:?}");
                assert!(position.2.abs() < 1e-5, "{position:?}");
                assert!(
                    velocity.0.abs() < 1e-5 && velocity.1.abs() < 1e-5,
                    "{velocity:?}"
                );
                assert!((velocity.2 - 0.4).abs() < 1e-5, "{velocity:?}");
                // Detection computes payloads without moving the node itself.
                assert_eq!(Node::world_transform_value(&body).pos_value().x, start.0);
                assert_eq!(Node::world_transform_value(&body).pos_value().y, start.1);
            }
        }
    }

    #[test]
    fn test_stack_support_remains_stable_in_both_traversal_orders() {
        for mesh_floor in [false, true] {
            for reverse in [false, true] {
                let root = if mesh_floor {
                    mesh_floor_root()
                } else {
                    Node::new()
                };
                if !mesh_floor {
                    let floor = Node::new();
                    place_at(&floor, 0.0, -0.5, 0.0);
                    rc_mut!(&floor).collider =
                        Some(box_family_collider(Vec3::new(10.0, 1.0, 10.0), 0.0, 0.0));
                    Node::add_child(&root, &floor);
                }

                let mut blocks = Vec::new();
                for level in 0..4 {
                    let block = Node::new();
                    place_at(&block, 0.0, 0.5 + level as f32, 0.0);
                    let collider = box_family_collider(Vec3::one(), 0.0, 1.0);
                    rc_mut!(&collider).friction = 0.0;
                    rc_mut!(&block).collider = Some(collider);
                    Node::add_child(&root, &block);
                    blocks.push(block);
                }
                if reverse {
                    rc_mut!(&root).children.reverse();
                }

                for frame in 0..180 {
                    for block in &blocks {
                        let collider = rc_ref!(block).collider.clone().unwrap();
                        let velocity = rc_ref!(&collider).velocity.clone();
                        rc_mut!(&velocity).y -= 0.01;
                    }
                    Scene::integrate_motion(&root, 1.0 / 30.0);
                    let pairs = Scene::detect_contacts(&root);
                    // Solver passes must not become duplicate collision callbacks.
                    assert!(pairs.len() <= 4);
                    for pair in pairs {
                        for (node, contact) in [
                            (&pair.node_a, &pair.contact_a),
                            (&pair.node_b, &pair.contact_b),
                        ] {
                            let collider = rc_ref!(node).collider.clone().unwrap();
                            let transform = rc_ref!(node).transform.clone();
                            let velocity = rc_ref!(&collider).velocity.clone();
                            Scene::apply_contact_to_snapshot(
                                &mut rc_mut!(&transform),
                                &mut rc_mut!(&velocity),
                                contact,
                            );
                        }
                    }
                    if frame < 60 {
                        continue;
                    }
                    for (level, block) in blocks.iter().enumerate() {
                        let pos = Node::world_transform_value(block).pos_value();
                        let collider = rc_ref!(block).collider.clone().unwrap();
                        let velocity = *rc_ref!(&rc_ref!(&collider).velocity);
                        assert!((pos.y - (0.5 + level as f32)).abs() < 0.003,
                            "mesh={mesh_floor}, reverse={reverse}, frame={frame}, level={level}, y={}", pos.y);
                        assert!(velocity.y.abs() < 0.003, "{velocity:?}");
                    }
                }
            }
        }
    }

    fn advance_rigid_bodies(root: &RcNode, bodies: &[RcNode]) {
        advance_damped_rigid_bodies(root, bodies, 1.0, 1.0);
    }

    fn advance_damped_rigid_bodies(
        root: &RcNode,
        bodies: &[RcNode],
        damping: f32,
        angular_damping: f32,
    ) {
        for body in bodies {
            let collider = rc_ref!(body).collider.clone().unwrap();
            let velocity = rc_ref!(&collider).velocity.clone();
            let mut velocity = rc_mut!(&velocity);
            velocity.y -= 0.16;
            *velocity = vec_mul(*velocity, damping);
            let spin = rc_ref!(&collider).angular_velocity.clone();
            let mut spin = rc_mut!(&spin);
            *spin = vec_mul(*spin, angular_damping);
        }
        Scene::integrate_motion(root, 1.0 / 30.0);
        for pair in Scene::detect_contacts(root) {
            for (node, contact) in [
                (&pair.node_a, &pair.contact_a),
                (&pair.node_b, &pair.contact_b),
            ] {
                let collider = rc_ref!(node).collider.clone().unwrap();
                let transform = rc_ref!(node).transform.clone();
                let velocity = rc_ref!(&collider).velocity.clone();
                Scene::apply_contact_to_snapshot(
                    &mut rc_mut!(&transform),
                    &mut rc_mut!(&velocity),
                    contact,
                );
                let spin = rc_ref!(&collider).angular_velocity.clone();
                let value = vec_add(
                    *rc_ref!(&spin),
                    *rc_ref!(&rc_ref!(contact).delta_angular_velocity),
                );
                *rc_mut!(&spin) = value;
            }
        }
    }

    #[test]
    fn test_capsule_resting_across_a_short_mesh_platform_does_not_tip() {
        let root = mesh_floor_root();
        let floor = rc_ref!(&root).children[0].clone();
        let collider = rc_ref!(&floor).collider.clone().unwrap();
        let mesh = rc_ref!(&collider).mesh.clone().unwrap();
        let primitive = rc_ref!(&mesh).primitives[0].clone().unwrap();
        for position in &mut rc_mut!(&primitive).positions {
            *position *= 0.2;
        }

        let body = Node::new();
        place_rotated_z_at(&body, 0.0, 1.0, 0.0, 90.0);
        let collider = box_family_collider(Vec3::new(0.0, 4.0, 0.0), 1.0, 1.0);
        rc_mut!(&collider).rolls = true;
        rc_mut!(&body).collider = Some(collider.clone());
        Node::add_child(&root, &body);

        for _ in 0..600 {
            advance_rigid_bodies(&root, std::slice::from_ref(&body));
            let pos = Node::world_transform_value(&body).pos_value();
            assert!(
                vec_len(vec_sub(pos, vec3(0.0, 1.0, 0.0))) < 0.003,
                "{pos:?}"
            );
            assert!(vec_len(*rc_ref!(&rc_ref!(&collider).angular_velocity)) < 0.001);
        }
    }

    #[test]
    fn test_restitution_bounces_impacts_but_not_continuing_support() {
        let root = Node::new();
        let floor = Node::new();
        place_at(&floor, 0.0, -4.0, 0.0);
        let collider = box_family_collider(Vec3::new(100.0, 8.0, 100.0), 0.0, 0.0);
        rc_mut!(&collider).friction = 0.0;
        rc_mut!(&floor).collider = Some(collider);
        Node::add_child(&root, &floor);

        let ball = Node::new();
        place_at(&ball, 0.0, 40.0, 0.0);
        let collider = sphere_collider(10.0, 1.0);
        rc_mut!(&collider).rolls = true;
        rc_mut!(&collider).friction = 0.0;
        rc_mut!(&collider).restitution = 0.65;
        rc_mut!(&ball).collider = Some(collider.clone());
        Node::add_child(&root, &ball);

        let mut bounced = false;
        for _ in 0..600 {
            advance_rigid_bodies(&root, std::slice::from_ref(&ball));
            bounced |= rc_ref!(&rc_ref!(&collider).velocity).y > 0.1;
        }
        assert!(bounced);
        assert!((Node::world_transform_value(&ball).pos_value().y - 10.0).abs() < 0.001);
        assert!(vec_len(*rc_ref!(&rc_ref!(&collider).velocity)) < 1e-5);
    }

    #[test]
    fn test_bouncy_bodies_roll_without_repeated_gravity_bounces() {
        for (damping, angular_damping) in [(1.0, 1.0), (0.995, 0.98)] {
            for capsule in [false, true] {
                for slope in [-10.0, 0.0, 10.0] {
                    let root = mesh_floor_root();
                    let floor = rc_ref!(&root).children[0].clone();
                    let terrain = rc_ref!(&floor).collider.clone().unwrap();
                    let mesh = rc_ref!(&terrain).mesh.clone().unwrap();
                    let primitive = rc_ref!(&mesh).primitives[0].clone().unwrap();
                    for position in &mut rc_mut!(&primitive).positions {
                        *position *= 2000.0;
                    }
                    let rotation = *rc_ref!(&Mat4::from_euler(&vec3(0.0, 0.0, slope)));
                    rc_mut!(&floor).transform = Mat4::from_rows(rotation.data);
                    let normal = rotation.mul_dir_value(&vec3(0.0, 1.0, 0.0));
                    let tangent = rotation.mul_dir_value(&vec3(1.0, 0.0, 0.0));

                    let radius = 10.0;
                    let body = Node::new();
                    let size = if capsule {
                        Vec3::new(0.0, 18.0, 0.0)
                    } else {
                        Vec3::zero()
                    };
                    let collider = box_family_collider(size, radius, 1.0);
                    rc_mut!(&collider).rolls = true;
                    rc_mut!(&collider).restitution = 0.65;
                    rc_mut!(&collider).velocity = Vec3::new(tangent.x, tangent.y, tangent.z);
                    let spin = (-1.0 / radius).to_degrees();
                    rc_mut!(&collider).angular_velocity = if capsule {
                        Vec3::new(0.0, spin, 0.0)
                    } else {
                        Vec3::new(0.0, 0.0, spin)
                    };
                    rc_mut!(&body).collider = Some(collider.clone());
                    let mut pose = if capsule {
                        *rc_ref!(&Mat4::from_euler(&vec3(90.0, 0.0, 0.0)))
                    } else {
                        Mat4::identity_value()
                    };
                    for axis in 0..3 {
                        pose.data[axis][3] = component(normal, axis) * radius;
                    }
                    rc_mut!(&body).transform = Mat4::from_rows(pose.data);
                    Node::add_child(&root, &body);

                    for frame in 0..600 {
                        advance_damped_rigid_bodies(
                            &root,
                            std::slice::from_ref(&body),
                            damping,
                            angular_damping,
                        );
                        if frame < 10 {
                            continue;
                        }
                        let velocity = *rc_ref!(&rc_ref!(&collider).velocity);
                        let normal_speed = vec_dot(velocity, normal);
                        assert!(normal_speed.abs() < 0.001, "damping={damping}, capsule={capsule}, slope={slope}, frame={frame}, normal_speed={normal_speed}, pos={:?}, spin={:?}", Node::world_transform_value(&body).pos_value(), *rc_ref!(&rc_ref!(&collider).angular_velocity));
                        let distance =
                            vec_dot(Node::world_transform_value(&body).pos_value(), normal);
                        assert!(
                            (distance - radius).abs() < 0.003,
                            "damping={damping}, capsule={capsule}, slope={slope}, frame={frame}, distance={distance}"
                        );
                    }

                    // A new imposed approach is an impact, even without a frame
                    // of separation from the preceding rolling contact.
                    rc_mut!(&collider).velocity =
                        Vec3::new(-2.0 * normal.x, -2.0 * normal.y, -2.0 * normal.z);
                    advance_rigid_bodies(&root, std::slice::from_ref(&body));
                    assert!(vec_dot(*rc_ref!(&rc_ref!(&collider).velocity), normal) > 1.0);

                    // A new drop after relocating above the same floor must also bounce.
                    let mut pose = Node::world_transform_value(&body);
                    for axis in 0..3 {
                        pose.data[axis][3] += component(normal, axis) * 30.0;
                    }
                    rc_mut!(&body).transform = Mat4::from_rows(pose.data);
                    rc_mut!(&collider).velocity = Vec3::zero();
                    let mut bounced = false;
                    for _ in 0..100 {
                        advance_rigid_bodies(&root, std::slice::from_ref(&body));
                        let distance =
                            vec_dot(Node::world_transform_value(&body).pos_value(), normal);
                        if distance < radius + 0.01
                            && vec_dot(*rc_ref!(&rc_ref!(&collider).velocity), normal) > 1.0
                        {
                            bounced = true;
                        }
                    }
                    assert!(bounced, "capsule={capsule}, slope={slope}");

                    Node::remove_child(&root, &floor);
                    for _ in 0..60 {
                        advance_rigid_bodies(&root, std::slice::from_ref(&body));
                    }
                    assert!(
                        vec_dot(Node::world_transform_value(&body).pos_value(), normal) < -50.0
                    );
                }
            }
        }
    }

    #[test]
    fn test_fast_sphere_stops_when_starting_in_mesh_contact() {
        for (x, z) in [(0.0, 0.0), (5.0, 0.0), (5.0, -5.0)] {
            for y in [1.0, 0.99] {
                for restitution in [0.0, 0.65] {
                    let root = mesh_floor_root();
                    let ball = Node::new();
                    place_at(&ball, x, y, z);
                    let collider = sphere_collider(1.0, 1.0);
                    rc_mut!(&collider).rolls = true;
                    rc_mut!(&collider).restitution = restitution;
                    rc_mut!(&collider).velocity = Vec3::new(0.0, -10.0, 0.0);
                    rc_mut!(&ball).collider = Some(collider.clone());
                    Node::add_child(&root, &ball);

                    advance_rigid_bodies(&root, std::slice::from_ref(&ball));
                    let pos = Node::world_transform_value(&ball).pos_value();
                    let velocity = *rc_ref!(&rc_ref!(&collider).velocity);
                    assert!(
                        (pos.y - 1.0).abs() < 1e-4,
                        "start=({x},{y},{z}), pos={pos:?}"
                    );
                    assert!(
                        (velocity.y - 10.16 * restitution).abs() < 1e-4,
                        "start=({x},{y},{z}), velocity={velocity:?}"
                    );
                    assert!(vec_len(*rc_ref!(&rc_ref!(&collider).angular_velocity)) < 1e-4);
                }
            }
        }
    }

    #[test]
    fn test_new_approach_after_a_large_impact_still_bounces() {
        let root = Node::new();
        let floor = Node::new();
        place_at(&floor, 0.0, -4.0, 0.0);
        rc_mut!(&floor).collider =
            Some(box_family_collider(Vec3::new(100.0, 8.0, 100.0), 0.0, 0.0));
        Node::add_child(&root, &floor);

        let ball = Node::new();
        place_at(&ball, 0.0, 10.0, 0.0);
        let collider = sphere_collider(10.0, 1.0);
        rc_mut!(&collider).rolls = true;
        rc_mut!(&collider).restitution = 0.65;
        rc_mut!(&collider).velocity = Vec3::new(0.0, -10.0, 0.0);
        rc_mut!(&ball).collider = Some(collider.clone());
        Node::add_child(&root, &ball);

        advance_rigid_bodies(&root, std::slice::from_ref(&ball));
        assert!(rc_ref!(&rc_ref!(&collider).velocity).y > 6.0);
        rc_mut!(&collider).velocity = Vec3::new(0.0, -2.0, 0.0);
        advance_rigid_bodies(&root, std::slice::from_ref(&ball));
        assert!(rc_ref!(&rc_ref!(&collider).velocity).y > 1.0);

        // An existing gravity history must not turn a large explicit approach
        // into a supporting force that absorbs the following smaller impact.
        for _ in 0..120 {
            advance_rigid_bodies(&root, std::slice::from_ref(&ball));
        }
        rc_mut!(&collider).velocity = Vec3::new(0.0, -10.0, 0.0);
        advance_rigid_bodies(&root, std::slice::from_ref(&ball));
        assert!(rc_ref!(&rc_ref!(&collider).velocity).y > 6.0);
        rc_mut!(&collider).velocity = Vec3::new(0.0, -6.0, 0.0);
        advance_rigid_bodies(&root, std::slice::from_ref(&ball));
        assert!(rc_ref!(&rc_ref!(&collider).velocity).y > 3.9);
    }

    #[test]
    fn test_rounded_cross_stack_stays_at_rest_in_both_traversal_orders() {
        for reverse in [false, true] {
            let root = Node::new();
            let floor = Node::new();
            place_at(&floor, 0.0, -6.0, 0.0);
            rc_mut!(&floor).collider =
                Some(box_family_collider(Vec3::new(300.0, 12.0, 260.0), 0.0, 0.0));
            Node::add_child(&root, &floor);

            let mut bodies = Vec::new();
            for level in 0..6 {
                for offset in [-20.0, 0.0, 20.0] {
                    let body = Node::new();
                    let y = 10.0 + level as f32 * 20.0;
                    if level % 2 == 0 {
                        place_at(&body, 0.0, y, offset);
                    } else {
                        let translation = *rc_ref!(&Mat4::from_translation(&vec3(offset, y, 0.0)));
                        let rotation = *rc_ref!(&Mat4::from_euler(&vec3(0.0, 90.0, 0.0)));
                        rc_mut!(&body).transform = translation.mul_mat(&rotation);
                    }
                    let collider = box_family_collider(Vec3::new(58.0, 18.0, 18.0), 1.0, 1.0);
                    rc_mut!(&collider).rolls = true;
                    rc_mut!(&body).collider = Some(collider);
                    bodies.push(body);
                }
            }
            if reverse {
                bodies.reverse();
            }
            for body in &bodies {
                Node::add_child(&root, body);
            }
            let starts: Vec<_> = bodies
                .iter()
                .map(|body| Node::world_transform_value(body).pos_value())
                .collect();

            for _ in 0..600 {
                advance_rigid_bodies(&root, &bodies);
            }
            for (body, start) in bodies.iter().zip(starts) {
                // Settling stays within one twentieth of a beam's thickness.
                let pos = Node::world_transform_value(body).pos_value();
                assert!(
                    vec_len(vec_sub(pos, start)) < 1.0,
                    "reverse={reverse}, {pos:?}"
                );
            }
            let resting: Vec<_> = bodies.iter().map(Node::world_transform_value).collect();
            for _ in 0..2400 {
                advance_rigid_bodies(&root, &bodies);
            }
            for (body, pose) in bodies.iter().zip(resting) {
                assert_eq!(
                    Node::world_transform_value(body).data,
                    pose.data,
                    "reverse={reverse}"
                );
            }
        }
    }

    #[test]
    fn test_resting_stack_wakes_on_motion_impact_and_lost_support() {
        let root = Node::new();
        let floor = Node::new();
        place_at(&floor, 0.0, -4.0, 0.0);
        rc_mut!(&floor).collider =
            Some(box_family_collider(Vec3::new(200.0, 8.0, 200.0), 0.0, 0.0));
        Node::add_child(&root, &floor);

        let mut bodies = Vec::new();
        let mut starts = Vec::new();
        for (size, radius, mass, pos) in [
            ((16.0, 28.0, 6.0), 0.0, 0.5, (-19.0, 14.0, 0.0)),
            ((16.0, 28.0, 6.0), 0.0, 0.5, (19.0, 14.0, 0.0)),
            ((56.0, 8.0, 20.0), 0.0, 2.0, (0.0, 32.0, 0.0)),
            ((0.0, 0.0, 0.0), 8.0, 1.0, (0.0, 44.0, 0.0)),
        ] {
            let body = Node::new();
            place_at(&body, pos.0, pos.1, pos.2);
            let collider = box_family_collider(Vec3::new(size.0, size.1, size.2), radius, mass);
            rc_mut!(&collider).rolls = true;
            rc_mut!(&collider).friction = 0.2;
            rc_mut!(&body).collider = Some(collider);
            Node::add_child(&root, &body);
            bodies.push(body);
            starts.push(vec3(pos.0, pos.1, pos.2));
        }

        for _ in 0..600 {
            advance_rigid_bodies(&root, &bodies);
        }
        for (body, start) in bodies.iter().zip(&starts) {
            let pos = Node::world_transform_value(body).pos_value();
            assert!(
                vec_len(vec_sub(pos, *start)) < 0.03,
                "start={start:?}, pos={pos:?}"
            );
            let collider = rc_ref!(body).collider.clone().unwrap();
            assert!(vec_len(*rc_ref!(&rc_ref!(&collider).velocity)) < 1e-5);
        }

        let resting: Vec<_> = bodies.iter().map(Node::world_transform_value).collect();
        for _ in 0..2400 {
            advance_rigid_bodies(&root, &bodies);
        }
        for (body, pose) in bodies.iter().zip(resting) {
            assert_eq!(Node::world_transform_value(body).data, pose.data);
        }

        let ball = &bodies[3];
        let collider = rc_ref!(ball).collider.clone().unwrap();
        rc_mut!(&collider).velocity = Vec3::new(1.0, 1.0, 0.0);
        advance_rigid_bodies(&root, &bodies);
        let pos = Node::world_transform_value(ball).pos_value();
        assert!(pos.x > 0.9 && pos.y > 44.5, "{pos:?}");

        let shot = Node::new();
        place_at(&shot, -42.0, 15.0, 0.0);
        let collider = sphere_collider(8.0, 3.0);
        rc_mut!(&collider).rolls = true;
        rc_mut!(&collider).velocity = Vec3::new(8.0, 0.0, 0.0);
        rc_mut!(&shot).collider = Some(collider);
        Node::add_child(&root, &shot);
        bodies.push(shot);
        for _ in 0..10 {
            advance_rigid_bodies(&root, &bodies);
        }
        assert!(
            vec_len(vec_sub(
                Node::world_transform_value(&bodies[0]).pos_value(),
                starts[0]
            )) > 1.0
        );

        Node::remove_child(&root, &floor);
        for _ in 0..60 {
            advance_rigid_bodies(&root, &bodies);
        }
        assert!(bodies
            .iter()
            .all(|body| Node::world_transform_value(body).pos_value().y < -50.0));
    }

    #[test]
    fn test_rolling_capsules_support_a_loaded_plank_at_rest() {
        for reverse in [false, true] {
            let root = mesh_floor_root();
            let floor = rc_ref!(&root).children[0].clone();
            let floor_collider = rc_ref!(&floor).collider.clone().unwrap();
            rc_mut!(&floor_collider).friction = 0.0;
            let mesh = rc_ref!(&floor_collider).mesh.clone().unwrap();
            let primitive = rc_ref!(&mesh).primitives[0].clone().unwrap();
            rc_mut!(&primitive).positions = vec![
                -96.0, 52.0, -112.0, -96.0, 52.0, -70.0, 96.0, 52.0, -70.0, 96.0, 52.0, -112.0,
            ];
            rc_mut!(&primitive).indices = vec![0, 1, 2, 0, 2, 3];

            let mut bodies = Vec::new();
            for (size, radius, mass, pos, angle) in [
                ((0.0, 16.0, 0.0), 7.0, 2.0, (49.0, 59.0, -97.0), 90.0),
                ((0.0, 16.0, 0.0), 7.0, 2.0, (49.0, 59.0, -81.0), 90.0),
                ((56.0, 8.0, 20.0), 0.0, 2.0, (49.0, 70.0, -89.0), 0.0),
                ((0.0, 0.0, 0.0), 8.0, 1.0, (39.0, 82.0, -89.0), 0.0),
                ((0.0, 0.0, 0.0), 8.0, 1.0, (59.0, 82.0, -89.0), 0.0),
            ] {
                let body = Node::new();
                place_rotated_z_at(&body, pos.0, pos.1, pos.2, angle);
                let collider = box_family_collider(Vec3::new(size.0, size.1, size.2), radius, mass);
                rc_mut!(&collider).rolls = true;
                rc_mut!(&collider).friction = 0.2;
                rc_mut!(&body).collider = Some(collider);
                Node::add_child(&root, &body);
                bodies.push((body, vec3(pos.0, pos.1, pos.2)));
            }
            if reverse {
                rc_mut!(&root).children.reverse();
            }

            for frame in 0..600 {
                for (body, _) in &bodies {
                    let collider = rc_ref!(body).collider.clone().unwrap();
                    let velocity = rc_ref!(&collider).velocity.clone();
                    let spin = rc_ref!(&collider).angular_velocity.clone();
                    rc_mut!(&velocity).y -= 0.16;
                    let damped_velocity = vec_mul(*rc_ref!(&velocity), 0.995);
                    let damped_spin = vec_mul(*rc_ref!(&spin), 0.98);
                    *rc_mut!(&velocity) = damped_velocity;
                    *rc_mut!(&spin) = damped_spin;
                }
                Scene::integrate_motion(&root, 1.0 / 30.0);
                for pair in Scene::detect_contacts(&root) {
                    for (node, contact) in [
                        (&pair.node_a, &pair.contact_a),
                        (&pair.node_b, &pair.contact_b),
                    ] {
                        let collider = rc_ref!(node).collider.clone().unwrap();
                        let transform = rc_ref!(node).transform.clone();
                        let velocity = rc_ref!(&collider).velocity.clone();
                        Scene::apply_contact_to_snapshot(
                            &mut rc_mut!(&transform),
                            &mut rc_mut!(&velocity),
                            contact,
                        );
                        let spin = rc_ref!(&collider).angular_velocity.clone();
                        let updated = vec_add(
                            *rc_ref!(&spin),
                            *rc_ref!(&rc_ref!(contact).delta_angular_velocity),
                        );
                        *rc_mut!(&spin) = updated;
                    }
                }
                if frame >= 60 {
                    for (body, start) in &bodies {
                        let pos = Node::world_transform_value(body).pos_value();
                        assert!(
                            vec_len(vec_sub(pos, *start)) < 0.01,
                            "reverse={reverse}, frame={frame}, start={start:?}, pos={pos:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_mesh_opposite_faces_do_not_amplify_position_corrections() {
        for reverse in [false, true] {
            let root = mesh_floor_root();
            let floor = rc_ref!(&root).children[0].clone();
            let collider = rc_ref!(&floor).collider.clone().unwrap();
            let mesh = rc_ref!(&collider).mesh.clone().unwrap();
            let primitive = rc_ref!(&mesh).primitives[0].clone().unwrap();
            {
                let mut primitive = rc_mut!(&primitive);
                // A body wider than this opening cannot satisfy both exits.
                primitive.positions.extend_from_slice(&[
                    -5.0, 0.8, -5.0, 5.0, 0.8, -5.0, -5.0, 0.8, 5.0, 5.0, 0.8, 5.0,
                ]);
                primitive.indices.extend_from_slice(&[4, 5, 6, 5, 7, 6]);
            }

            let body = Node::new();
            place_at(&body, 0.0, 0.3, 0.0);
            rc_mut!(&body).collider = Some(box_family_collider(Vec3::new(1.0, 1.0, 1.0), 0.0, 1.0));
            Node::add_child(&root, &body);
            if reverse {
                rc_mut!(&root).children.reverse();
            }

            let pairs = Scene::detect_contacts(&root);
            assert_eq!(pairs.len(), 1);
            let contact = if std::rc::Rc::ptr_eq(&pairs[0].node_a, &body) {
                rc_ref!(&pairs[0].contact_a)
            } else {
                rc_ref!(&pairs[0].contact_b)
            };
            assert!((contact.depth - 0.2).abs() < 1e-5);
            assert!(rc_ref!(&contact.normal).y > 0.99);
        }
    }

    #[test]
    fn test_mesh_corner_trigger_reports_once_without_correction() {
        let root = mesh_corner_root();
        let body = Node::new();
        place_at(&body, 0.4, 0.3, 0.0);
        rc_mut!(&body).collider = Some(trigger_sphere_collider(0.5, 1.0));
        Node::add_child(&root, &body);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        assert_eq!(rc_ref!(&pairs[0].contact_b).depth, 0.0);
    }

    fn mesh_corner_root() -> RcNode {
        let root = mesh_floor_root();
        let floor = rc_ref!(&root).children[0].clone();
        let collider = rc_ref!(&floor).collider.clone().unwrap();
        rc_mut!(&collider).friction = 0.0;
        let mesh = rc_ref!(&collider).mesh.clone().unwrap();
        let primitive = rc_ref!(&mesh).primitives[0].clone().unwrap();
        let mut primitive = rc_mut!(&primitive);
        primitive
            .positions
            .extend_from_slice(&[0.0, 0.0, -5.0, 0.0, 5.0, -5.0, 0.0, 0.0, 5.0, 0.0, 5.0, 5.0]);
        primitive.indices.extend_from_slice(&[4, 5, 6, 5, 7, 6]);
        root
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

    #[test]
    fn test_detach_destroyed_consumes_root_notification() {
        let scene = Node::new();
        Node::destroy(&scene);
        Scene::detach_destroyed(&scene);
        assert!(rc_ref!(&scene).destroyed);
        assert!(Scene::collect_destroyed_post_order(&scene).is_empty());
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
            0.0,
            crate::cube::Vec3::zero(),
            0.0,
            0.0,
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
            0.0,
            crate::cube::Vec3::zero(),
            0.0,
            0.0,
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
            0.0,
            crate::cube::Vec3::zero(),
            0.0,
            0.0,
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
        rc_mut!(node).transform = rc_ref!(&translation).mul_mat(&rc_ref!(&rotation));
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
        assert_vec3_close(*normal, vec3(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_detect_contacts_preserves_collider_entry_capacity() {
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).collider = Some(sphere_collider(0.5, 1.0));
        rc_mut!(&b).collider = Some(sphere_collider(0.5, 1.0));
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);

        Scene::detect_contacts(&root);
        let first_capacity = Scene::collider_entry_scratch_capacity();
        Scene::detect_contacts(&root);
        assert!(first_capacity >= 2);
        assert_eq!(Scene::collider_entry_scratch_capacity(), first_capacity);
    }

    #[test]
    fn test_detect_contacts_does_not_allocate_result_without_contacts() {
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).collider = Some(sphere_collider(0.5, 1.0));
        rc_mut!(&b).collider = Some(sphere_collider(0.5, 1.0));
        place_at(&b, 10.0, 0.0, 0.0);
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);

        let pairs = Scene::detect_contacts(&root);
        assert!(pairs.is_empty());
        assert_eq!(pairs.capacity(), 0);
    }

    #[test]
    fn test_swept_analytic_bodies_stop_on_the_entry_side() {
        // Sphere, capsule, and box all have a unit half-width on the motion
        // axis. Check actual applied responses, including partial crossings,
        // a moving zero-mass obstacle, and an equally massive dynamic body.
        let shapes = [
            (vec3(0.0, 0.0, 0.0), 1.0),
            (vec3(0.0, 4.0, 0.0), 1.0),
            (vec3(2.0, 2.0, 2.0), 0.0),
        ];
        for (sa, ra) in shapes {
            for (sb, rb) in shapes {
                for mass_b in [0.0, 1.0] {
                    for speed_b in [0.0, 7.0] {
                        for closing_speed in [11.0, 30.0] {
                            for restitution in [0.0, 1.0] {
                                let root = Node::new();
                                let a = Node::new();
                                let b = Node::new();
                                let ca = box_family_collider(Vec3::new(sa.x, sa.y, sa.z), ra, 1.0);
                                let cb =
                                    box_family_collider(Vec3::new(sb.x, sb.y, sb.z), rb, mass_b);
                                for (node, collider, x, speed) in [
                                    (&a, &ca, -10.0, closing_speed + speed_b),
                                    (&b, &cb, 0.0, speed_b),
                                ] {
                                    place_at(node, x, 0.0, 0.0);
                                    rc_mut!(collider).rolls = true;
                                    rc_mut!(collider).friction = 0.0;
                                    rc_mut!(collider).restitution = restitution;
                                    rc_mut!(collider).velocity = Vec3::new(speed, 0.0, 0.0);
                                    rc_mut!(node).collider = Some(collider.clone());
                                    Node::add_child(&root, node);
                                }

                                Scene::integrate_motion(&root, 1.0 / 30.0);
                                let contacts = Scene::detect_contacts(&root);
                                assert_eq!(contacts.len(), 1);
                                let pair = &contacts[0];
                                let impact_x = speed_b * (8.0 / closing_speed) - 1.0;
                                assert!(
                                    (rc_ref!(&rc_ref!(&pair.contact_a).point).x - impact_x).abs()
                                        < 1e-4,
                                    "point={:?}, expected_x={impact_x}, sa={sa:?} sb={sb:?}",
                                    *rc_ref!(&rc_ref!(&pair.contact_a).point)
                                );
                                for (node, collider, contact) in
                                    [(&a, &ca, &pair.contact_a), (&b, &cb, &pair.contact_b)]
                                {
                                    let transform = rc_ref!(node).transform.clone();
                                    let velocity = rc_ref!(collider).velocity.clone();
                                    Scene::apply_contact_to_snapshot(
                                        &mut rc_mut!(&transform),
                                        &mut rc_mut!(&velocity),
                                        contact,
                                    );
                                    assert!(
                                        vec_len(*rc_ref!(&rc_ref!(contact).delta_angular_velocity))
                                            < 0.001,
                                        "sa={sa:?} sb={sb:?} mass_b={mass_b} speed_b={speed_b} closing={closing_speed} e={restitution} spin={:?}",
                                        *rc_ref!(&rc_ref!(contact).delta_angular_velocity)
                                    );
                                }

                                let separation = Node::world_transform_value(&b).pos_value().x
                                    - Node::world_transform_value(&a).pos_value().x;
                                assert!(
                                    (separation - 2.0).abs() < 1e-4,
                                    "separation={separation}, sa={sa:?}, sb={sb:?}"
                                );
                                let va = *rc_ref!(&rc_ref!(&ca).velocity);
                                let vb = *rc_ref!(&rc_ref!(&cb).velocity);
                                let impulse = closing_speed * (1.0 + restitution) / (1.0 + mass_b);
                                assert_vec3_close(
                                    va,
                                    vec3(speed_b + closing_speed - impulse, 0.0, 0.0),
                                );
                                assert_vec3_close(vb, vec3(speed_b + impulse * mass_b, 0.0, 0.0));
                                // In the obstacle's initial rest frame, e=1 conserves
                                // energy and e=0 cannot add energy.
                                let energy =
                                    (va.x - speed_b).powi(2) + mass_b * (vb.x - speed_b).powi(2);
                                let initial_energy = closing_speed * closing_speed;
                                assert!(energy <= initial_energy + 0.001);
                                if restitution == 1.0 {
                                    assert!((energy - initial_energy).abs() < 0.001);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_swept_analytic_bodies_approach_from_initial_touch() {
        let shapes = [
            (vec3(0.0, 0.0, 0.0), 1.0),
            (vec3(0.0, 4.0, 0.0), 1.0),
            (vec3(2.0, 2.0, 2.0), 0.0),
            (vec3(1.0, 1.0, 1.0), 0.5),
        ];
        for (sa, ra) in shapes {
            for (sb, rb) in shapes {
                for mass_b in [0.0, 1.0] {
                    for speed_b in [0.0, 7.0] {
                        let root = Node::new();
                        let a = Node::new();
                        let b = Node::new();
                        let ca = box_family_collider(Vec3::new(sa.x, sa.y, sa.z), ra, 1.0);
                        let cb = box_family_collider(Vec3::new(sb.x, sb.y, sb.z), rb, mass_b);
                        for (node, collider, x, speed) in
                            [(&a, &ca, -2.0, speed_b + 6.0), (&b, &cb, 0.0, speed_b)]
                        {
                            place_at(node, x, 0.0, 0.0);
                            rc_mut!(collider).rolls = true;
                            rc_mut!(collider).friction = 0.0;
                            rc_mut!(collider).restitution = 1.0;
                            rc_mut!(collider).velocity = Vec3::new(speed, 0.0, 0.0);
                            rc_mut!(node).collider = Some(collider.clone());
                            Node::add_child(&root, node);
                        }

                        Scene::integrate_motion(&root, 1.0 / 30.0);
                        let contacts = Scene::detect_contacts(&root);
                        assert_eq!(
                            contacts.len(),
                            1,
                            "sa={sa:?} sb={sb:?} mass_b={mass_b} speed_b={speed_b}"
                        );
                        let pair = &contacts[0];
                        assert!((rc_ref!(&rc_ref!(&pair.contact_a).point).x + 1.0).abs() < 1e-4);
                        for (node, collider, contact) in
                            [(&a, &ca, &pair.contact_a), (&b, &cb, &pair.contact_b)]
                        {
                            let transform = rc_ref!(node).transform.clone();
                            let velocity = rc_ref!(collider).velocity.clone();
                            Scene::apply_contact_to_snapshot(
                                &mut rc_mut!(&transform),
                                &mut rc_mut!(&velocity),
                                contact,
                            );
                            assert!(
                                vec_len(*rc_ref!(&rc_ref!(contact).delta_angular_velocity)) < 0.001
                            );
                        }

                        let separation = Node::world_transform_value(&b).pos_value().x
                            - Node::world_transform_value(&a).pos_value().x;
                        assert!(
                            (separation - 2.0).abs() < 1e-4,
                            "sa={sa:?} sb={sb:?} separation={separation}"
                        );
                        let impulse = 12.0 / (1.0 + mass_b);
                        assert_vec3_close(
                            *rc_ref!(&rc_ref!(&ca).velocity),
                            vec3(speed_b + 6.0 - impulse, 0.0, 0.0),
                        );
                        assert_vec3_close(
                            *rc_ref!(&rc_ref!(&cb).velocity),
                            vec3(speed_b + impulse * mass_b, 0.0, 0.0),
                        );

                        // The same touching boundary must allow sliding and exit.
                        for motion in [vec3(-6.0, 0.0, 0.0), vec3(0.0, 0.0, 0.5)] {
                            let va = vec_add(motion, vec3(speed_b, 0.0, 0.0));
                            let vb = vec3(speed_b, 0.0, 0.0);
                            place_at(&a, -2.0 + va.x, va.y, va.z);
                            place_at(&b, vb.x, vb.y, vb.z);
                            assert!(
                                Scene::narrow_phase(
                                    &Node::world_transform_value(&a),
                                    &ca,
                                    va,
                                    &Node::world_transform_value(&b),
                                    &cb,
                                    vb,
                                )
                                .is_none(),
                                "sa={sa:?} sb={sb:?} motion={motion:?}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_sweep_keeps_the_entry_face_when_ending_inside_a_wide_box() {
        let root = Node::new();
        let ball = Node::new();
        let wall = Node::new();
        let collider = sphere_collider(1.0, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(100.0, 0.0, 0.0);
        rc_mut!(&ball).collider = Some(collider);
        rc_mut!(&wall).collider = Some(box_family_collider(Vec3::new(100.0, 2.0, 100.0), 0.0, 0.0));
        place_at(&ball, -100.0, 0.0, 0.0);
        Node::add_child(&root, &ball);
        Node::add_child(&root, &wall);

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(-1.0, 0.0, 0.0));
        assert!((contact.depth - 51.0).abs() < 1e-4);
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(-100.0, 0.0, 0.0));
    }

    #[test]
    fn test_swept_friction_uses_the_sphere_radius_at_impact() {
        let root = Node::new();
        let ball = Node::new();
        let wall = Node::new();
        let ca = sphere_collider(1.0, 1.0);
        let cb = box_family_collider(Vec3::new(2.0, 100.0, 100.0), 0.0, 0.0);
        rc_mut!(&ca).rolls = true;
        rc_mut!(&ca).friction = 0.5;
        rc_mut!(&cb).friction = 0.5;
        rc_mut!(&ca).velocity = Vec3::new(30.0, 10.0, 0.0);
        rc_mut!(&ball).collider = Some(ca.clone());
        rc_mut!(&wall).collider = Some(cb);
        place_at(&ball, -10.0, 0.0, 0.0);
        Node::add_child(&root, &ball);
        Node::add_child(&root, &wall);

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        let velocity = vec_add(
            *rc_ref!(&rc_ref!(&ca).velocity),
            *rc_ref!(&contact.delta_velocity),
        );
        let spin = vec_mul(
            *rc_ref!(&contact.delta_angular_velocity),
            1.0_f32.to_radians(),
        );
        // I = 2/5 m r². An inelastic normal impact and sufficient friction
        // leave 5/7 of the tangential speed, with matching surface rotation.
        assert_vec3_close(velocity, vec3(0.0, 50.0 / 7.0, 0.0));
        assert_vec3_close(spin, vec3(0.0, 0.0, -50.0 / 7.0));
        let position = vec_add(
            Node::world_transform_value(&ball).pos_value(),
            vec_mul(*rc_ref!(&contact.normal), contact.depth),
        );
        assert_vec3_close(position, vec3(-2.0, 10.0, 0.0));
        let energy = 0.5 * vec_len_sq(velocity) + 0.2 * vec_len_sq(spin);
        assert!((energy - 250.0 / 7.0).abs() < 1e-4);
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(-1.0, 0.0, 0.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(-3.0, 0.0, 0.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(-1.0, 0.0, 0.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(-3.0, 0.0, 0.0));
    }

    #[test]
    fn test_swept_sphere_follows_box_corner_radius() {
        let world = Mat4::identity_value();
        let half = vec3(1.0, 1.0, 1.0);
        let velocity = vec3(0.0, 0.0, 6.0);
        let zero = vec3(0.0, 0.0, 0.0);
        // Both paths cross the expanded AABB. Only the .3/.3 offset is within
        // the summed .5 radius; its corner normal has X/Y components .3/.5.
        for (sphere_radius, box_radius) in [(0.5, 0.0), (0.25, 0.25)] {
            assert!(Scene::swept_sphere_vs_rounded_obb(
                vec3(1.4, 1.4, 3.0),
                sphere_radius,
                velocity,
                &world,
                half,
                box_radius,
                zero,
            )
            .is_none());
            let hit = Scene::swept_sphere_vs_rounded_obb(
                vec3(1.3, 1.3, 3.0),
                sphere_radius,
                velocity,
                &world,
                half,
                box_radius,
                zero,
            )
            .unwrap();
            assert_vec3_close(hit.normal, vec3(0.6, 0.6, -0.28_f32.sqrt()));
        }
    }

    #[test]
    fn test_swept_sphere_distinguishes_box_overlap_from_aabb_overlap() {
        let world = Mat4::identity_value();
        let half = vec3(1.0, 1.0, 1.0);
        let zero = vec3(0.0, 0.0, 0.0);
        // Start inside the true rounded boundary and move out: no new impact.
        assert!(Scene::swept_sphere_vs_rounded_obb(
            vec3(1.3, 1.3, 3.0),
            0.5,
            vec3(0.0, 0.0, 3.0),
            &world,
            half,
            0.0,
            zero,
        )
        .is_none());
        // Start inside the expanded AABB but outside the rounded boundary,
        // then cross the box. The earlier broad-filter entry time is zero.
        let hit = Scene::swept_sphere_vs_rounded_obb(
            vec3(-2.0, 1.4, 0.0),
            0.5,
            vec3(-3.4, 0.0, 0.0),
            &world,
            half,
            0.0,
            zero,
        )
        .unwrap();
        assert_vec3_close(hit.normal, vec3(0.6, 0.8, 0.0));
        assert_vec3_close(hit.point, vec3(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_swept_sphere_contact_tracks_moving_rotated_box() {
        let rotation = *rc_ref!(&Mat4::from_euler(&vec3(0.0, 0.0, 90.0)));
        let translation = *rc_ref!(&Mat4::from_translation(&vec3(4.0, 5.0, 6.0)));
        let world = translation.mul_mat_value(&rotation);
        let hit = Scene::swept_sphere_vs_rounded_obb(
            vec3(2.7, 6.3, 9.0),
            0.25,
            vec3(2.0, 0.0, 6.0),
            &world,
            vec3(1.0, 1.0, 1.0),
            0.25,
            vec3(2.0, 0.0, 0.0),
        )
        .unwrap();
        // In box space the path crosses the .5-radius corner at offsets .3/.3.
        // Its world contact includes the box's translation up to the impact time.
        let toi = (2.0 - 0.07_f32.sqrt()) / 6.0;
        assert_vec3_close(hit.normal, vec3(-0.6, 0.6, -0.28_f32.sqrt()));
        assert_vec3_close(
            hit.point,
            vec3(0.85 + 2.0 * toi, 6.15, 5.0 - 0.25 * 0.28_f32.sqrt()),
        );
        assert!((hit.depth - 6.0 * 0.28_f32.sqrt() * (1.0 - toi)).abs() < 1e-4);
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(-1.0, 0.0, 0.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(-3.0, 0.0, 0.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(0.0, 1.0, 0.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(0.0, 3.0, 0.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(-1.0, 0.0, 0.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(-3.0, 0.0, 0.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(0.0, 0.0, -1.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(0.0, 0.0, -3.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(0.0, 0.0, -1.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(0.0, 0.0, -3.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_a);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(-1.0, 0.0, 0.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(-3.0, 0.0, 0.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(0.0, 1.0, 0.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(0.0, 3.0, 0.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(0.0, 1.0, 0.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(0.0, 3.0, 0.0));
    }

    #[test]
    fn test_long_mesh_edge_does_not_deflect_parallel_motion() {
        let rotation = *rc_ref!(&Mat4::from_euler(&vec3(0.0, 0.0, 10.0)));
        let a = rotation.mul_vec_value(&vec3(-10000.0, 0.0, -10000.0));
        let b = rotation.mul_vec_value(&vec3(10000.0, 0.0, 10000.0));
        let velocity = rotation.mul_dir_value(&vec3(-0.96, 0.0, 0.0));
        for clearance in [0.00001, 0.001, 0.01] {
            let start = rotation.mul_vec_value(&vec3(0.2, 10.0 + clearance, 0.0));
            assert!(swept_sphere_vs_segment(start, velocity, 10.0, a, b).is_none());
        }
    }

    #[test]
    fn test_touching_mesh_vertex_does_not_hide_a_crossed_floor() {
        let root = mesh_floor_root();
        let floor = rc_ref!(&root).children[0].clone();
        let collider = rc_ref!(&floor).collider.clone().unwrap();
        let mesh = rc_ref!(&collider).mesh.clone().unwrap();
        let primitive = rc_ref!(&mesh).primitives[0].clone().unwrap();
        {
            let mut p = rc_mut!(&primitive);
            // A raised triangle corner, followed by a lower, broad floor.
            p.positions = vec![
                -128.0, 48.0, -128.0, 128.0, 48.0, -128.0, -128.0, 48.0, 128.0, 128.0, 48.0, 128.0,
                0.0, 56.0, -33.0, 16.0, 56.0, -33.0, 0.0, 56.0, -17.0,
            ];
            p.indices.extend([4, 5, 6]);
        }

        let body = Node::new();
        let collider = sphere_collider(8.0, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(1.9806976, -9.674616, 0.2771951);
        rc_mut!(&body).collider = Some(collider);
        place_at(&body, -5.4078255, 56.25569, -38.889828);
        Node::add_child(&root, &body);

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let mut position = Node::world_transform_value(&body).pos_value();
        for pair in Scene::detect_contacts(&root) {
            let contact = rc_ref!(&pair.contact_b);
            position = vec_add(position, vec_mul(*rc_ref!(&contact.normal), contact.depth));
        }
        assert!(position.y >= 56.0 - 1e-4, "{position:?}");
    }

    #[test]
    fn test_mesh_wall_contact_does_not_hide_a_crossed_floor() {
        for (size, radius, height) in [
            ((0.0, 0.0, 0.0), 0.5, 0.5),
            ((0.0, 1.0, 0.0), 0.5, 1.0),
            ((1.0, 1.0, 1.0), 0.0, 0.5),
            ((1.0, 1.0, 1.0), 0.1, 0.6),
        ] {
            for reverse in [false, true] {
                let root = mesh_corner_root();
                let body = Node::new();
                let width = size.0 * 0.5 + radius;
                let start_y = height + 0.01;
                let collider = box_family_collider(Vec3::new(size.0, size.1, size.2), radius, 1.0);
                rc_mut!(&collider).velocity = Vec3::new(-0.001, -start_y - 0.1, 0.2);
                rc_mut!(&body).collider = Some(collider);
                place_at(&body, width, start_y, 0.0);
                Node::add_child(&root, &body);
                if reverse {
                    rc_mut!(&root).children.reverse();
                }

                Scene::integrate_motion(&root, 1.0 / 30.0);
                let mut position = Node::world_transform_value(&body).pos_value();
                for pair in Scene::detect_contacts(&root) {
                    let contact = if std::rc::Rc::ptr_eq(&pair.node_a, &body) {
                        rc_ref!(&pair.contact_a)
                    } else {
                        rc_ref!(&pair.contact_b)
                    };
                    position = vec_add(position, vec_mul(*rc_ref!(&contact.normal), contact.depth));
                }
                assert!(
                    (position.x - width).abs() < 1e-4,
                    "size={size:?}: {position:?}"
                );
                assert!(
                    (position.y - height).abs() < 1e-4,
                    "size={size:?}: {position:?}"
                );
                assert!(
                    (position.z - 0.2).abs() < 1e-4,
                    "size={size:?}: {position:?}"
                );
            }
        }
    }

    #[test]
    fn test_mesh_landing_resolves_crossed_surface_for_each_shape() {
        for (size, radius, height) in [
            ((0.0, 0.0, 0.0), 0.5, 0.5),
            ((0.0, 1.0, 0.0), 0.5, 1.0),
            ((1.0, 1.0, 1.0), 0.0, 0.5),
            ((1.0, 1.0, 1.0), 0.1, 0.6),
        ] {
            for end_y in [-0.01, -2.0] {
                let root = mesh_floor_root();
                let body = Node::new();
                let start_y = height + 0.1;
                let collider = box_family_collider(Vec3::new(size.0, size.1, size.2), radius, 1.0);
                rc_mut!(&collider).velocity = Vec3::new(0.0, end_y - start_y, 0.0);
                rc_mut!(&body).collider = Some(collider);
                place_at(&body, 0.0, start_y, 0.0);
                Node::add_child(&root, &body);
                Scene::integrate_motion(&root, 1.0 / 30.0);

                let pairs = Scene::detect_contacts(&root);
                assert_eq!(pairs.len(), 1, "size={size:?}, end_y={end_y}");
                let contact = rc_ref!(&pairs[0].contact_b);
                assert_vec3_close(*rc_ref!(&contact.normal), vec3(0.0, 1.0, 0.0));
                assert!((end_y + contact.depth - height).abs() < 1e-4);
                assert!((end_y - start_y + rc_ref!(&contact.delta_velocity).y).abs() < 1e-4);
            }
        }
    }

    #[test]
    fn test_mesh_velocity_does_not_cancel_dynamic_sweep() {
        let root = mesh_floor_root();
        let floor = rc_ref!(&root).children[0].clone();
        let floor_collider = rc_ref!(&floor).collider.clone().unwrap();
        rc_mut!(&floor_collider).velocity = Vec3::new(0.0, -3.0, 0.0);
        let moving = Node::new();
        let moving_collider = sphere_collider(0.25, 1.0);
        rc_mut!(&moving_collider).velocity = Vec3::new(0.0, -3.0, 0.0);
        rc_mut!(&moving).collider = Some(moving_collider);
        place_at(&moving, 0.0, 1.5, 0.0);
        Node::add_child(&root, &moving);

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(0.0, 1.0, 0.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(0.0, 3.0, 0.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(0.0, 0.0, -1.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(0.0, 0.0, -3.0));
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        assert_vec3_close(*rc_ref!(&contact.normal), vec3(0.0, 0.0, -1.0));
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(0.0, 0.0, -3.0));
    }

    #[test]
    fn test_contact_depth_split_evenly_for_equal_mass() {
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
        assert_eq!(depth_a, 0.25);
        assert_eq!(depth_b, 0.25);
    }

    #[test]
    fn test_contact_depth_uses_direct_finite_mass_ratio() {
        for (mass_a, mass_b) in [(0.85, 2.2), (2.2, 0.85)] {
            let root = Node::new();
            let a = Node::new();
            let b = Node::new();
            rc_mut!(&a).collider = Some(sphere_collider(0.5, mass_a));
            rc_mut!(&b).collider = Some(sphere_collider(0.5, mass_b));
            place_at(&a, 0.0, 0.0, 0.0);
            place_at(&b, 0.5, 0.0, 0.0);
            Node::add_child(&root, &a);
            Node::add_child(&root, &b);

            let pairs = Scene::detect_contacts(&root);
            assert_eq!(pairs.len(), 1);
            let total = mass_a + mass_b;
            assert_eq!(rc_ref!(&pairs[0].contact_a).depth, 0.5 * (mass_b / total));
            assert_eq!(rc_ref!(&pairs[0].contact_b).depth, 0.5 * (mass_a / total));
        }
    }

    #[test]
    fn test_contact_mass_ratio_does_not_overflow() {
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).collider = Some(sphere_collider(0.5, f32::MAX));
        rc_mut!(&b).collider = Some(sphere_collider(0.5, f32::MAX));
        place_at(&a, 0.0, 0.0, 0.0);
        place_at(&b, 0.5, 0.0, 0.0);
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);

        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let depth_a = rc_ref!(&pairs[0].contact_a).depth;
        let depth_b = rc_ref!(&pairs[0].contact_b).depth;
        assert_eq!(depth_a, 0.25);
        assert_eq!(depth_b, 0.25);
    }

    #[test]
    fn test_invalid_internal_mass_is_immovable() {
        for invalid_mass in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let root = Node::new();
            let invalid = Node::new();
            let movable = Node::new();
            rc_mut!(&invalid).collider = Some(sphere_collider(0.5, invalid_mass));
            rc_mut!(&movable).collider = Some(sphere_collider(0.5, 1.0));
            place_at(&invalid, 0.0, 0.0, 0.0);
            place_at(&movable, 0.5, 0.0, 0.0);
            Node::add_child(&root, &invalid);
            Node::add_child(&root, &movable);

            let pairs = Scene::detect_contacts(&root);
            assert_eq!(pairs.len(), 1, "invalid_mass = {invalid_mass}");
            let depth_invalid = rc_ref!(&pairs[0].contact_a).depth;
            let depth_movable = rc_ref!(&pairs[0].contact_b).depth;
            assert_eq!(depth_invalid, 0.0, "invalid_mass = {invalid_mass}");
            assert_eq!(depth_movable, 0.5, "invalid_mass = {invalid_mass}");
        }
    }

    #[test]
    fn test_invalid_internal_mass_is_static_for_pair_filtering() {
        for invalid_mass in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let root = Node::new();
            let invalid = Node::new();
            let static_node = Node::new();
            rc_mut!(&invalid).collider = Some(sphere_collider(0.5, invalid_mass));
            rc_mut!(&static_node).collider = Some(sphere_collider(0.5, 0.0));
            place_at(&invalid, 0.0, 0.0, 0.0);
            place_at(&static_node, 0.5, 0.0, 0.0);
            Node::add_child(&root, &invalid);
            Node::add_child(&root, &static_node);

            let pairs = Scene::detect_contacts(&root);
            assert!(pairs.is_empty(), "invalid_mass = {invalid_mass}");
        }
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
        assert_eq!(depth_movable, 0.5);
        assert_eq!(depth_wall, 0.0);
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
    fn test_coupled_contacts_conserve_momentum_and_ignore_triggers() {
        for restitution in [0.0, 0.5, 1.0] {
            for reverse in [false, true] {
                let root = Node::new();
                let masses = [1.0, 2.0, 3.0];
                let speeds = [0.2, 0.0, -0.1];
                let mut bodies = Vec::new();
                for index in 0..3 {
                    let body = Node::new();
                    place_at(&body, (index as f32 - 1.0) * 0.9, 0.0, 0.0);
                    let collider = sphere_collider(0.5, masses[index]);
                    rc_mut!(&collider).velocity = Vec3::new(speeds[index], 0.0, 0.0);
                    rc_mut!(&collider).friction = 0.0;
                    rc_mut!(&collider).restitution = restitution;
                    rc_mut!(&body).collider = Some(collider);
                    Node::add_child(&root, &body);
                    bodies.push(body);
                }
                let trigger = Node::new();
                rc_mut!(&trigger).collider = Some(trigger_sphere_collider(2.0, 100.0));
                Node::add_child(&root, &trigger);
                if reverse {
                    rc_mut!(&root).children.reverse();
                }

                let pairs = Scene::detect_contacts(&root);
                assert_eq!(pairs.len(), 5); // Two solid pairs and three trigger pairs.
                let mut resolved = speeds;
                for pair in pairs {
                    let is_trigger = std::rc::Rc::ptr_eq(&pair.node_a, &trigger)
                        || std::rc::Rc::ptr_eq(&pair.node_b, &trigger);
                    for (node, contact) in [
                        (&pair.node_a, &pair.contact_a),
                        (&pair.node_b, &pair.contact_b),
                    ] {
                        let contact = rc_ref!(contact);
                        let delta = rc_ref!(&contact.delta_velocity);
                        if is_trigger {
                            assert_eq!(contact.depth, 0.0);
                            assert_eq!((delta.x, delta.y, delta.z), (0.0, 0.0, 0.0));
                        }
                        if let Some(index) = bodies
                            .iter()
                            .position(|body| std::rc::Rc::ptr_eq(body, node))
                        {
                            resolved[index] += delta.x;
                        }
                    }
                }
                let momentum: f32 = masses
                    .iter()
                    .zip(speeds)
                    .map(|(mass, speed)| mass * speed)
                    .sum();
                let resolved_momentum: f32 = masses
                    .iter()
                    .zip(resolved)
                    .map(|(mass, speed)| mass * speed)
                    .sum();
                assert!((momentum - resolved_momentum).abs() < 1e-6);
                let center_speed = momentum / masses.iter().sum::<f32>();
                for index in 0..3 {
                    // Collinear contacts reverse relative speed by restitution.
                    let expected = center_speed - restitution * (speeds[index] - center_speed);
                    assert!((resolved[index] - expected).abs() < 1e-5, "{resolved:?}");
                    let collider = rc_ref!(&bodies[index]).collider.clone().unwrap();
                    // Detection supplies corrections; it must not apply them itself.
                    assert_eq!(rc_ref!(&rc_ref!(&collider).velocity).x, speeds[index]);
                }
            }
        }
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
        assert_eq!(delta_velocity.x, -2.0);
        assert_eq!(delta_velocity.y, 0.0);
        assert_eq!(delta_velocity.z, 0.0);
    }

    #[test]
    fn test_linear_mesh_contacts_keep_friction_without_physics_history() {
        for (size, radius) in [
            ((0.0, 0.0, 0.0), 1.0),
            ((0.0, 1.0, 0.0), 0.5),
            ((1.0, 1.0, 1.0), 0.5),
            ((2.0, 2.0, 2.0), 0.0),
        ] {
            let root = mesh_floor_root();
            let floor = rc_ref!(&root).children[0].clone();
            let floor_collider = rc_ref!(&floor).collider.clone().unwrap();
            rc_mut!(&floor_collider).friction = 0.3;

            let body = Node::new();
            let collider = box_family_collider(Vec3::new(size.0, size.1, size.2), radius, 2.0);
            rc_mut!(&collider).friction = 0.5;
            rc_mut!(&collider).velocity = Vec3::new(2.0, -1.0, 0.0);
            rc_mut!(&body).collider = Some(collider);
            place_at(&body, 0.0, 0.9, 0.0);
            Node::add_child(&root, &body);

            for _ in 0..3 {
                let pairs = Scene::detect_contacts(&root);
                assert_eq!(pairs.len(), 1);
                let contact = rc_ref!(&pairs[0].contact_b);
                assert!((contact.depth - 0.1).abs() < 1e-5);
                assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(-0.4, 1.0, 0.0));
                assert_vec3_close(
                    *rc_ref!(&contact.delta_angular_velocity),
                    vec3(0.0, 0.0, 0.0),
                );
                assert!(rc_ref!(&root).contact_cache.is_none());
            }
        }
    }

    #[test]
    fn test_rigid_contact_history_excludes_unconnected_characters() {
        let root = mesh_floor_root();
        let mut bodies = Vec::new();
        for (x, rolls) in [(-3.0, false), (0.0, true), (3.0, false)] {
            let body = Node::new();
            let collider = sphere_collider(1.0, 1.0);
            rc_mut!(&collider).rolls = rolls;
            rc_mut!(&collider).velocity = Vec3::new(0.0, -0.1, 0.0);
            rc_mut!(&body).collider = Some(collider);
            place_at(&body, x, 0.95, 0.0);
            Node::add_child(&root, &body);
            bodies.push(body);
        }

        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 3);
        for pair in &pairs {
            assert_vec3_close(
                *rc_ref!(&rc_ref!(&pair.contact_b).delta_velocity),
                vec3(0.0, 0.1, 0.0),
            );
        }
        assert_eq!(
            rc_ref!(&root).contact_cache.as_ref().unwrap().bodies.len(),
            2
        );

        // The character hitting a rolling body must join its coupled response;
        // another character merely standing on the same floor still must not.
        place_at(&bodies[0], -1.8, 0.95, 0.0);
        Scene::detect_contacts(&root);
        let root = rc_ref!(&root);
        let cache = root.contact_cache.as_ref().unwrap();
        assert_eq!(cache.bodies.len(), 3);
        assert!(cache
            .bodies
            .iter()
            .all(|cached| cached.node.as_ptr() != std::rc::Rc::as_ptr(&bodies[2])));
    }

    #[test]
    fn test_disconnected_contacts_keep_their_mass_ratios() {
        let root = Node::new();
        for (x, mass, speed) in [
            (0.0, 1.0, 3.0),
            (0.9, 2.0, 0.0),
            (1000.0, 1e25, 3.0),
            (1000.9, 2e25, 0.0),
            (2000.0, 1e30, 0.0),
        ] {
            let body = Node::new();
            let collider = sphere_collider(0.5, mass);
            rc_mut!(&collider).rolls = true;
            rc_mut!(&collider).restitution = 1.0;
            rc_mut!(&collider).velocity = Vec3::new(speed, 0.0, 0.0);
            rc_mut!(&body).collider = Some(collider);
            place_at(&body, x, 0.0, 0.0);
            Node::add_child(&root, &body);
        }

        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 2);
        // For masses 1 and 2, an elastic head-on impact at speed 3 ends at
        // speeds -1 and 2 at either mass scale. Neither a separate collision
        // nor a body in free flight can alter the colliding bodies' mass ratio.
        for pair in pairs {
            assert_vec3_close(
                *rc_ref!(&rc_ref!(&pair.contact_a).delta_velocity),
                vec3(-4.0, 0.0, 0.0),
            );
            assert_vec3_close(
                *rc_ref!(&rc_ref!(&pair.contact_b).delta_velocity),
                vec3(2.0, 0.0, 0.0),
            );
        }
    }

    #[test]
    fn test_prescribed_spin_still_affects_nonrolling_contact_friction() {
        let root = mesh_floor_root();
        let body = Node::new();
        let collider = sphere_collider(1.0, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(0.0, -0.1, 0.0);
        rc_mut!(&collider).angular_velocity = Vec3::new(0.0, 0.0, 90.0);
        rc_mut!(&body).collider = Some(collider);
        place_at(&body, 0.0, 0.95, 0.0);
        Node::add_child(&root, &body);

        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(-0.05, 0.1, 0.0));
        assert_vec3_close(
            *rc_ref!(&contact.delta_angular_velocity),
            vec3(0.0, 0.0, 0.0),
        );
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
        assert_eq!(delta_velocity.x, -1.0);
        assert_eq!(delta_velocity.y, -1.0);
        assert_eq!(delta_velocity.z, 0.0);
    }

    #[test]
    fn test_rigid_impacts_conserve_energy_and_turn_only_off_center() {
        for mass in [1e-25, 1.0, 1e25] {
            for height in [-0.8, 0.0, 0.8] {
                let root = Node::new();
                let ball = Node::new();
                let block = Node::new();
                place_at(&ball, -2.4, height, 0.0);
                let ball_collider = sphere_collider(0.5, mass);
                rc_mut!(&ball_collider).velocity = Vec3::new(3.0, 0.0, 0.0);
                rc_mut!(&ball_collider).rolls = true;
                rc_mut!(&ball_collider).friction = 0.0;
                rc_mut!(&ball_collider).restitution = 1.0;
                rc_mut!(&ball).collider = Some(ball_collider);
                let block_collider = box_family_collider(Vec3::new(4.0, 2.0, 2.0), 0.0, mass * 2.0);
                rc_mut!(&block_collider).rolls = true;
                rc_mut!(&block_collider).friction = 0.0;
                rc_mut!(&block).collider = Some(block_collider);
                Node::add_child(&root, &ball);
                Node::add_child(&root, &block);

                let pairs = Scene::detect_contacts(&root);
                assert_eq!(pairs.len(), 1);
                let contact_a = rc_ref!(&pairs[0].contact_a);
                let contact_b = rc_ref!(&pairs[0].contact_b);
                let va = vec_add(vec3(3.0, 0.0, 0.0), *rc_ref!(&contact_a.delta_velocity));
                let vb = *rc_ref!(&contact_b.delta_velocity);
                let wa = vec_mul(
                    *rc_ref!(&contact_a.delta_angular_velocity),
                    1.0_f32.to_radians(),
                );
                let wb = vec_mul(
                    *rc_ref!(&contact_b.delta_angular_velocity),
                    1.0_f32.to_radians(),
                );
                assert_vec3_close(vec_add(va, vec_mul(vb, 2.0)), vec3(3.0, 0.0, 0.0));
                let energy = 0.5 * vec_len_sq(va)
                    + vec_len_sq(vb)
                    + 0.5 * 0.1 * vec_len_sq(wa)
                    + 0.5 * (4.0 / 3.0 * wb.x * wb.x + 10.0 / 3.0 * (wb.y * wb.y + wb.z * wb.z));
                assert!(
                    (energy - 4.5).abs() < 1e-4,
                    "height={height}, energy={energy}"
                );
                assert!(vec_len(wa) < 1e-5);
                if height == 0.0 {
                    assert!(vec_len(wb) < 1e-5);
                } else {
                    assert!(wb.z * height < -0.1, "height={height}, spin={wb:?}");
                }
            }
        }
    }

    #[test]
    fn test_frictionless_sliding_does_not_create_spin() {
        let root = mesh_floor_root();
        let floor = rc_ref!(&root).children[0].clone();
        let floor_collider = rc_ref!(&floor).collider.clone().unwrap();
        rc_mut!(&floor_collider).friction = 0.0;

        let ball = Node::new();
        place_at(&ball, 0.0, 0.49, 0.0);
        let collider = sphere_collider(0.5, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(1.0, -0.1, 0.0);
        rc_mut!(&collider).rolls = true;
        rc_mut!(&collider).friction = 0.0;
        rc_mut!(&ball).collider = Some(collider);
        Node::add_child(&root, &ball);

        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        assert_vec3_close(*rc_ref!(&contact.delta_velocity), vec3(0.0, 0.1, 0.0));
        assert!(vec_len(*rc_ref!(&contact.delta_angular_velocity)) < 1e-5);
    }

    #[test]
    fn test_friction_converts_sliding_to_rolling_without_adding_energy() {
        for radius in [0.5, 9.0, 18.0] {
            for rotated in [false, true] {
                let root = Node::new();
                let ball = Node::new();
                place_at(&ball, 0.0, radius - 0.001, 0.0);
                if rotated {
                    let transform = rc_ref!(&ball).transform.clone();
                    rc_mut!(&ball).transform = rc_ref!(&transform)
                        .mul_mat(&rc_ref!(&Mat4::from_euler(&vec3(25.0, 70.0, 35.0))));
                }
                let collider = sphere_collider(radius, 1.0);
                rc_mut!(&collider).velocity = Vec3::new(1.0, -1.0, 0.0);
                rc_mut!(&collider).friction = 1.0;
                rc_mut!(&collider).rolls = true;
                rc_mut!(&ball).collider = Some(collider);
                let floor = Node::new();
                place_at(&floor, 0.0, -1.0, 0.0);
                let floor_collider = box_family_collider(Vec3::new(100.0, 2.0, 100.0), 0.0, 0.0);
                rc_mut!(&floor_collider).friction = 1.0;
                rc_mut!(&floor).collider = Some(floor_collider);
                Node::add_child(&root, &ball);
                Node::add_child(&root, &floor);

                let pairs = Scene::detect_contacts(&root);
                let contact = rc_ref!(&pairs[0].contact_a);
                let velocity = vec_add(vec3(1.0, -1.0, 0.0), *rc_ref!(&contact.delta_velocity));
                let spin = Node::world_rotation_value(&ball).mul_dir_value(&vec_mul(
                    *rc_ref!(&contact.delta_angular_velocity),
                    1.0_f32.to_radians(),
                ));
                // A solid sphere settles to v=5/7 and omega=v/r after friction
                // converts some of its initial translational energy into rotation.
                assert!((velocity.x - 5.0 / 7.0).abs() < 0.002, "{velocity:?}");
                assert!(
                    vec_len(vec_add(velocity, vec_cross(spin, vec3(0.0, -radius, 0.0)))) < 0.003
                );
                let energy = 0.5 * vec_len_sq(velocity) + 0.2 * radius * radius * vec_len_sq(spin);
                assert!(energy < 0.5, "{energy}");
            }
        }
    }

    #[test]
    fn test_narrow_phase_rotated_wall_no_phantom_contact() {
        // A thin wall (size=(0.4, 4, 4)) rotated 45° about Y and a sphere
        // 1.0 along the wall's world face normal: the true face gap is
        // 1.0 - 0.2 - 0.5 = 0.3, so exact narrow phase finds no contact.
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
        assert_vec3_close(*normal, vec3(0.0, 1.0, 0.0));
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
        // ≈ 0.32 > radius 0.3 → no contact.
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
        // Neither collider responds to collision, so omit this non-trigger pair.
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

    #[test]
    fn test_mesh_velocity_does_not_drive_contact_response() {
        let root = mesh_floor_root();
        let floor = rc_ref!(&root).children[0].clone();
        let floor_collider = rc_ref!(&floor).collider.clone().unwrap();
        rc_mut!(&floor_collider).velocity = Vec3::new(0.0, 1.0, 0.0);
        let ball = Node::new();
        rc_mut!(&ball).collider = Some(sphere_collider(0.5, 1.0));
        place_at(&ball, 0.0, 0.4, 0.0);
        Node::add_child(&root, &ball);

        let pairs = Scene::detect_contacts(&root);
        assert_eq!(pairs.len(), 1);
        let contact = rc_ref!(&pairs[0].contact_b);
        let delta = rc_ref!(&contact.delta_velocity);
        assert_eq!((delta.x, delta.y, delta.z), (0.0, 0.0, 0.0));
    }

    // Two-triangle floor fixture for mesh contact and raycast tests
    fn mesh_floor_root() -> RcNode {
        use crate::cube::collider::Collider;
        use crate::cube::mesh::Mesh;
        use crate::cube::primitive::Primitive;

        let floor_mesh = Mesh::new();
        {
            let mut m = rc_mut!(&floor_mesh);
            let geom = Primitive::new();
            {
                let mut g = rc_mut!(&geom);
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
            0.0,
            crate::cube::Vec3::zero(),
            0.0,
            0.0,
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
            let mut m = rc_mut!(&wall_mesh);
            let geom = Primitive::new();
            {
                let mut g = rc_mut!(&geom);
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
            0.0,
            crate::cube::Vec3::zero(),
            0.0,
            0.0,
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
            let mut m = rc_mut!(&mesh);
            let geom = Primitive::new();
            {
                let mut g = rc_mut!(&geom);
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
            0.0,
            crate::cube::Vec3::zero(),
            0.0,
            0.0,
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
        assert_vec3_close(hit.normal, vec3(0.0, 1.0, 0.0));
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
        Node::add_child(&root, &far);
        Node::add_child(&root, &near);

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
        assert_vec3_close(hit.normal, vec3(1.0, 0.0, 0.0));
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
        assert!(std::rc::Rc::ptr_eq(&hits[0].node, &b));
        assert!(std::rc::Rc::ptr_eq(&hits[1].node, &c));
        assert!(std::rc::Rc::ptr_eq(&hits[2].node, &a));
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
        rc_mut!(&enemy).tags = HashSet::from(["enemy".to_string()]);
        rc_mut!(&friend).tags = HashSet::from(["friend".to_string()]);
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

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pos_rc = rc_ref!(&n).transform.clone();
        let pos = rc_ref!(&pos_rc).pos();
        assert_eq!(rc_ref!(&pos).x, 0.0, "inactive subtree should not move");
    }

    #[test]
    fn test_integrate_motion_ignores_mesh_motion_state() {
        let (root, terrain) = sparse_triangle_mesh_root();
        let translation = Mat4::from_translation(&Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let rotation = Mat4::from_axis_angle(
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            30.0,
        );
        let initial = rc_ref!(&translation).mul_mat_value(&rc_ref!(&rotation));
        rc_mut!(&terrain).transform = Mat4::from_rows(initial.data);
        let collider = rc_ref!(&terrain).collider.clone().unwrap();
        rc_mut!(&collider).velocity = Vec3::new(1.0, 2.0, 3.0);
        rc_mut!(&collider).angular_velocity = Vec3::new(0.0, 45.0, 0.0);

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let actual_rc = rc_ref!(&terrain).transform.clone();
        let actual = rc_ref!(&actual_rc);
        assert_eq!(actual.data, initial.data);
    }

    #[test]
    fn test_integrate_motion_preserves_world_translation_and_local_rotation() {
        let node = Node::new();
        rc_mut!(&node).transform = Mat4::from_translation(&Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let collider = sphere_collider(0.5, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(1.0, 0.0, 0.0);
        rc_mut!(&collider).angular_velocity = Vec3::new(0.0, 90.0, 0.0);
        rc_mut!(&node).collider = Some(collider);

        Scene::integrate_motion(&node, 1.0 / 30.0);

        let translation = Mat4::from_translation(&Vec3 {
            x: 2.0,
            y: 2.0,
            z: 3.0,
        });
        let rotation = Mat4::from_axis_angle(
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            90.0,
        );
        let expected = rc_ref!(&translation).mul_mat_value(&rc_ref!(&rotation));
        let actual_rc = rc_ref!(&node).transform.clone();
        let actual = rc_ref!(&actual_rc);
        for row in 0..4 {
            for col in 0..4 {
                assert!((actual.data[row][col] - expected.data[row][col]).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn test_integrate_motion_uses_world_velocity_under_rotated_parent() {
        let root = Node::new();
        let parent = Node::new();
        let child = Node::new();
        rc_mut!(&parent).transform = Mat4::from_axis_angle(
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            90.0,
        );
        let collider = sphere_collider(0.5, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(1.0, 0.0, 0.0);
        rc_mut!(&child).collider = Some(collider);
        Node::add_child(&root, &parent);
        Node::add_child(&parent, &child);

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let world = Node::world_transform_value(&child);
        let pos = world.pos_value();
        assert!((pos.x - 1.0).abs() < 1e-6, "pos.x = {}", pos.x);
        assert!(pos.y.abs() < 1e-6, "pos.y = {}", pos.y);
        assert!(pos.z.abs() < 1e-6, "pos.z = {}", pos.z);
    }

    #[test]
    fn test_integrate_motion_uses_world_velocity_under_scaled_parent() {
        let root = Node::new();
        let parent = Node::new();
        let child = Node::new();
        rc_mut!(&parent).transform = Mat4::from_scale(&Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        });
        let collider = sphere_collider(0.5, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(1.0, 1.0, 1.0);
        rc_mut!(&child).collider = Some(collider);
        Node::add_child(&root, &parent);
        Node::add_child(&parent, &child);

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pos = Node::world_transform_value(&child).pos_value();
        assert!((pos.x - 1.0).abs() < 1e-6, "pos.x = {}", pos.x);
        assert!((pos.y - 1.0).abs() < 1e-6, "pos.y = {}", pos.y);
        assert!((pos.z - 1.0).abs() < 1e-6, "pos.z = {}", pos.z);
    }

    #[test]
    fn test_integrate_motion_on_subtree_uses_external_parent_transform() {
        let parent = Node::new();
        let child = Node::new();
        rc_mut!(&parent).transform = Mat4::from_axis_angle(
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            90.0,
        );
        let collider = sphere_collider(0.5, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(1.0, 0.0, 0.0);
        rc_mut!(&child).collider = Some(collider);
        Node::add_child(&parent, &child);

        Scene::integrate_motion(&child, 1.0 / 30.0);
        let pos = Node::world_transform_value(&child).pos_value();
        assert!((pos.x - 1.0).abs() < 1e-6, "pos.x = {}", pos.x);
        assert!(pos.y.abs() < 1e-6, "pos.y = {}", pos.y);
        assert!(pos.z.abs() < 1e-6, "pos.z = {}", pos.z);
    }

    #[test]
    fn test_integrate_motion_keeps_local_spin_under_singular_parent() {
        let root = Node::new();
        let parent = Node::new();
        let child = Node::new();
        rc_mut!(&parent).transform = Mat4::from_scale(&Vec3 {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        });
        let collider = sphere_collider(0.5, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(1.0, 0.0, 0.0);
        rc_mut!(&collider).angular_velocity = Vec3::new(0.0, 90.0, 0.0);
        rc_mut!(&child).collider = Some(collider);
        Node::add_child(&root, &parent);
        Node::add_child(&parent, &child);

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let local_rc = rc_ref!(&child).transform.clone();
        let local = rc_ref!(&local_rc);
        assert_eq!(
            local.pos_value(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        );
        assert!(local.data.iter().flatten().all(|value| value.is_finite()));
        assert!(local.data[0][0].abs() < 1e-6);
    }

    #[test]
    fn test_integrate_motion_moves_massless_analytic_collider() {
        let node = Node::new();
        let collider = sphere_collider(0.5, 0.0);
        rc_mut!(&collider).velocity = Vec3::new(1.0, 2.0, 3.0);
        rc_mut!(&node).collider = Some(collider);

        Scene::integrate_motion(&node, 1.0 / 30.0);
        let pos = Node::world_transform_value(&node).pos_value();
        assert_eq!(
            pos,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );
    }

    #[test]
    fn test_singular_parent_velocity_does_not_create_swept_contact() {
        let root = Node::new();
        let parent = Node::new();
        let child = Node::new();
        rc_mut!(&parent).transform = Mat4::from_scale(&Vec3 {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        });
        let collider = sphere_collider(0.25, 1.0);
        rc_mut!(&collider).velocity = Vec3::new(3.0, 0.0, 0.0);
        rc_mut!(&child).collider = Some(collider);
        let wall = Node::new();
        rc_mut!(&wall).collider = Some(sphere_collider(0.25, 0.0));
        place_at(&wall, -1.5, 0.0, 0.0);
        Node::add_child(&root, &parent);
        Node::add_child(&parent, &child);
        Node::add_child(&root, &wall);

        Scene::integrate_motion(&root, 1.0 / 30.0);
        let pairs = Scene::detect_contacts(&root);
        assert!(pairs.is_empty());
    }
}
