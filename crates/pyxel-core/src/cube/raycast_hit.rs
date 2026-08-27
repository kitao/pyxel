use crate::cube::node::RcNode;
use crate::cube::vec3::{RcVec3, Vec3};

// Result payload returned by Scene.raycast / raycast_all. The binding fills
// every field before exposing this engine-built value.

pub struct RaycastHit {
    pub node: Option<RcNode>,
    pub point: RcVec3,
    pub normal: RcVec3,
    pub distance: f32,
}

define_rc_type!(RcRaycastHit, RaycastHit);

impl RaycastHit {
    pub fn new() -> RcRaycastHit {
        new_rc_type!(RaycastHit {
            node: None,
            point: Vec3::zero(),
            normal: Vec3::zero(),
            distance: 0.0,
        })
    }
}
