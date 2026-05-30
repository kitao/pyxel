use crate::cube::mat4::{Mat4, RcMat4};

// View information held independently from Scene so multiple cameras can
// be swapped per render call. Mutable.

pub struct Camera {
    pub transform: RcMat4,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub ortho_size: Option<f32>,
    pub clear_color: Option<i32>,
}

define_rc_type!(RcCamera, Camera);

impl Camera {
    pub fn new() -> RcCamera {
        new_rc_type!(Camera {
            transform: Mat4::identity(),
            fov: 60.0,
            near: 0.1,
            far: 1000.0,
            ortho_size: None,
            clear_color: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let c = Camera::new();
        let c = rc_ref!(&c);
        assert_eq!(c.fov, 60.0);
        assert_eq!(c.near, 0.1);
        assert_eq!(c.far, 1000.0);
        assert!(c.ortho_size.is_none());
        // transform defaults to identity
        let m = rc_ref!(&c.transform);
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(m.data[i][j], if i == j { 1.0 } else { 0.0 });
            }
        }
    }

    #[test]
    fn test_mutate() {
        let c = Camera::new();
        let c_mut = rc_mut!(&c);
        c_mut.fov = 90.0;
        c_mut.ortho_size = Some(10.0);
        assert_eq!(c_mut.fov, 90.0);
        assert_eq!(c_mut.ortho_size, Some(10.0));
    }

    #[test]
    fn test_clear_color_default_none() {
        let c = Camera::new();
        assert!(rc_ref!(&c).clear_color.is_none());
    }

    #[test]
    fn test_clear_color_set() {
        let c = Camera::new();
        rc_mut!(&c).clear_color = Some(7);
        assert_eq!(rc_ref!(&c).clear_color, Some(7));
    }
}
