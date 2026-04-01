use super::math::Vec3;

pub struct Light {
    pub dir: Vec3,
}

impl Light {
    pub fn new(dir: Vec3) -> Self {
        Self {
            dir: dir.normalize(),
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self::new(Vec3::new(1.0, -1.0, -1.0))
    }
}
