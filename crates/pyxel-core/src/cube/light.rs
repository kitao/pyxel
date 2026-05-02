use super::math::Vec3;

pub struct Light {
    pub dir: Vec3,
}

define_rc_type!(RcLight, Light);

impl Light {
    pub fn new(dir: Vec3) -> RcLight {
        new_rc_type!(Self {
            dir: dir.normalize(),
        })
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            dir: Vec3::new(1.0, -1.0, -1.0).normalize(),
        }
    }
}
