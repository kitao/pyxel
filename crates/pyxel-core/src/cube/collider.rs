use crate::cube::mesh::RcMesh;
use crate::cube::vec3::RcVec3;

// A mesh collider is static terrain; only the size/radius family uses motion state.
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
    pub gravity: f32,
    pub gravity_direction: RcVec3,
    pub linear_damp: f32,
    pub angular_damp: f32,
}

define_rc_type!(RcCollider, Collider);

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
        gravity: f32,
        gravity_direction: RcVec3,
        linear_damp: f32,
        angular_damp: f32,
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
            gravity,
            gravity_direction,
            linear_damp,
            angular_damp,
        })
    }

    pub(crate) fn contact_mass(&self) -> f32 {
        if self.mesh.is_none() && self.mass.is_finite() && self.mass > 0.0 {
            self.mass
        } else {
            0.0
        }
    }
}
