use super::math::{Mat4, Vec3};

pub struct Camera {
    pub pos: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

define_rc_type!(RcCamera, Camera);

impl Camera {
    pub fn new(pos: Vec3, target: Vec3) -> RcCamera {
        new_rc_type!(Self {
            pos,
            target,
            up: Vec3::UP,
            fov: 60.0,
            near: 0.1,
            far: 100.0,
        })
    }

    #[must_use]
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.pos, self.target, self.up)
    }

    #[must_use]
    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        Mat4::perspective(self.fov, aspect, self.near, self.far)
    }
}
