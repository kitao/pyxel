use crate::cube::quat::{Quat, RcQuat};
use crate::cube::vec3::{RcVec3, Vec3};

// Collision payload passed to on_collide(other, contact). Carries the
// contact geometry (point / normal / depth) and engine-resolved motion
// deltas the user applies to push the body back into a non-penetrating
// state.

pub struct Contact {
    pub point: RcVec3,
    pub normal: RcVec3,
    pub depth: f32,
    pub delta_rotation: RcQuat,
    pub delta_velocity: RcVec3,
    pub delta_angular_velocity: RcVec3,
}

define_rc_type!(RcContact, Contact);

impl Contact {
    pub fn new() -> RcContact {
        let zero = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        Self::from_values(zero, zero, 0.0, zero, zero)
    }

    pub(crate) fn from_values(
        point: Vec3,
        normal: Vec3,
        depth: f32,
        delta_velocity: Vec3,
        delta_angular_velocity: Vec3,
    ) -> RcContact {
        new_rc_type!(Contact {
            point: Vec3::new(point.x, point.y, point.z),
            normal: Vec3::new(normal.x, normal.y, normal.z),
            depth,
            delta_rotation: Quat::identity(),
            delta_velocity: Vec3::new(delta_velocity.x, delta_velocity.y, delta_velocity.z),
            delta_angular_velocity: Vec3::new(
                delta_angular_velocity.x,
                delta_angular_velocity.y,
                delta_angular_velocity.z,
            ),
        })
    }
}
