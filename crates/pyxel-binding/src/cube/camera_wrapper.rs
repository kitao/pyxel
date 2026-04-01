use pyo3::prelude::*;
use pyxel::cube;

use crate::cube::math_wrapper::Vec3;

#[pyclass(name = "Camera")]
pub struct Camera {
    pub inner: cube::Camera,
}

#[pymethods]
impl Camera {
    #[new]
    #[pyo3(signature = (pos, target, fov=60.0, near=0.1, far=100.0))]
    fn new(pos: &Vec3, target: &Vec3, fov: f32, near: f32, far: f32) -> Self {
        let mut cam = cube::Camera::new(pos.inner, target.inner);
        cam.fov = fov;
        cam.near = near;
        cam.far = far;
        Self { inner: cam }
    }

    #[getter]
    fn pos(&self) -> Vec3 {
        Vec3 {
            inner: self.inner.pos,
        }
    }

    #[setter]
    fn set_pos(&mut self, v: &Vec3) {
        self.inner.pos = v.inner;
    }

    #[getter]
    fn target(&self) -> Vec3 {
        Vec3 {
            inner: self.inner.target,
        }
    }

    #[setter]
    fn set_target(&mut self, v: &Vec3) {
        self.inner.target = v.inner;
    }

    #[getter]
    fn fov(&self) -> f32 {
        self.inner.fov
    }

    #[setter]
    fn set_fov(&mut self, v: f32) {
        self.inner.fov = v;
    }

    #[getter]
    fn near(&self) -> f32 {
        self.inner.near
    }

    #[setter]
    fn set_near(&mut self, v: f32) {
        self.inner.near = v;
    }

    #[getter]
    fn far(&self) -> f32 {
        self.inner.far
    }

    #[setter]
    fn set_far(&mut self, v: f32) {
        self.inner.far = v;
    }
}

pub fn add_cube_camera_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Camera>()?;
    Ok(())
}
