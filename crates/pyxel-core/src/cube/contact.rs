use crate::cube::mat4::{Mat4, RcMat4};
use crate::cube::vec3::{RcVec3, Vec3};

// Collision payload passed to on_collide(other, contact). Carries the
// contact geometry (point / normal / depth) and engine-resolved motion
// deltas the user applies to push the body back into a non-penetrating
// state (cube-design.md § 12).

pub struct Contact {
    pub point: RcVec3,
    pub normal: RcVec3,
    pub depth: f32,
    pub delta_rotation: RcMat4,
    pub delta_velocity: RcVec3,
    pub delta_angular_velocity: RcVec3,
}

define_rc_type!(RcContact, Contact);

impl Contact {
    pub fn new() -> RcContact {
        new_rc_type!(Contact {
            point: Vec3::zero(),
            normal: Vec3::zero(),
            depth: 0.0,
            delta_rotation: Mat4::identity(),
            delta_velocity: Vec3::zero(),
            delta_angular_velocity: Vec3::zero(),
        })
    }
}
