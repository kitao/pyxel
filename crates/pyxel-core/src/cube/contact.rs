use crate::cube::vec3::{RcVec3, Vec3};

// Empty contact payload placeholder. Will be filled by the future
// collision pipeline (cube-design.md § 15). Both `point` and `normal`
// default to Vec3.ZERO; user code can construct one explicitly to
// stage scenarios for the upcoming `on_collide` hook.

pub struct Contact {
    pub point: RcVec3,
    pub normal: RcVec3,
}

define_rc_type!(RcContact, Contact);

impl Contact {
    pub fn new() -> RcContact {
        new_rc_type!(Contact {
            point: Vec3::zero(),
            normal: Vec3::zero(),
        })
    }
}
