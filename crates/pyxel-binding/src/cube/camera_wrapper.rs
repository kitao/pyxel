use pyo3::prelude::*;

use crate::cube::math_wrapper::Vec3;

define_wrapper!(Camera, pyxel::cube::Camera);

#[pymethods]
impl Camera {
    #[new]
    #[pyo3(signature = (pos, target, fov=60.0, near=0.1, far=100.0))]
    fn new(pos: &Vec3, target: &Vec3, fov: f32, near: f32, far: f32) -> Self {
        let rc = pyxel::cube::Camera::new(pos.inner, target.inner);
        {
            let cam = rc_mut!(rc);
            cam.fov = fov;
            cam.near = near;
            cam.far = far;
        }
        Self::wrap(rc)
    }

    #[getter]
    fn pos(&self) -> Vec3 {
        Vec3 {
            inner: self.inner_ref().pos,
        }
    }

    #[setter]
    fn set_pos(&self, v: &Vec3) {
        self.inner_mut().pos = v.inner;
    }

    #[getter]
    fn target(&self) -> Vec3 {
        Vec3 {
            inner: self.inner_ref().target,
        }
    }

    #[setter]
    fn set_target(&self, v: &Vec3) {
        self.inner_mut().target = v.inner;
    }

    #[getter]
    fn fov(&self) -> f32 {
        self.inner_ref().fov
    }

    #[setter]
    fn set_fov(&self, v: f32) {
        self.inner_mut().fov = v;
    }

    #[getter]
    fn near(&self) -> f32 {
        self.inner_ref().near
    }

    #[setter]
    fn set_near(&self, v: f32) {
        self.inner_mut().near = v;
    }

    #[getter]
    fn far(&self) -> f32 {
        self.inner_ref().far
    }

    #[setter]
    fn set_far(&self, v: f32) {
        self.inner_mut().far = v;
    }
}

pub fn add_cube_camera_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Camera>()?;
    Ok(())
}
