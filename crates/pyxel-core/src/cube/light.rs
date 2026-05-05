use crate::cube::vec3::{RcVec3, Vec3};

// Flat-shading light parameters held independently so multiple lights can
// be swapped between frames or scenes. Mutable.

pub struct Light {
    pub ambient: f32,
    pub direction: RcVec3,
    pub intensity: f32,
}

define_rc_type!(RcLight, Light);

impl Light {
    pub fn new() -> RcLight {
        new_rc_type!(Light {
            ambient: 0.0,
            direction: Vec3::down(),
            intensity: 1.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let l = Light::new();
        let l = rc_ref!(&l);
        assert_eq!(l.ambient, 0.0);
        assert_eq!(l.intensity, 1.0);
        let dir = rc_ref!(&l.direction);
        assert_eq!(dir.x, 0.0);
        assert_eq!(dir.y, -1.0);
        assert_eq!(dir.z, 0.0);
    }

    #[test]
    fn test_mutate() {
        let l = Light::new();
        let l_mut = rc_mut!(&l);
        l_mut.ambient = 0.3;
        l_mut.intensity = 0.7;
        l_mut.direction = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(l_mut.ambient, 0.3);
        assert_eq!(l_mut.intensity, 0.7);
        assert_eq!(rc_ref!(&l_mut.direction).x, 1.0);
    }
}
