use crate::cube::mesh::RcMesh;
use crate::cube::vec3::RcVec3;

// Unified collider: rounded-box family (size + radius) or static mesh
// terrain, plus behavior flags, physical coefficients, and per-frame
// motion state. Detection and contact resolution live in scene.rs
// (cube-design.md § 11 and § 16).

pub struct Collider {
    pub size: RcVec3,
    pub radius: f32,
    pub mesh: Option<RcMesh>,
    pub trigger: bool,
    pub rolls: bool,
    pub mass: f32,
    pub restitution: f32,
    pub friction: f32,
    pub velocity: RcVec3,
    pub angular_velocity: RcVec3,
}

define_rc_type!(RcCollider, Collider);

// The constructor mirrors the public collider fields and avoids a temporary
// options struct on the Rust-Python boundary.
#[allow(clippy::too_many_arguments)]
impl Collider {
    pub fn new(
        size: RcVec3,
        radius: f32,
        mesh: Option<RcMesh>,
        trigger: bool,
        rolls: bool,
        mass: f32,
        restitution: f32,
        friction: f32,
        velocity: RcVec3,
        angular_velocity: RcVec3,
    ) -> RcCollider {
        new_rc_type!(Collider {
            size,
            radius,
            mesh,
            trigger,
            rolls,
            mass,
            restitution,
            friction,
            velocity,
            angular_velocity,
        })
    }
}
