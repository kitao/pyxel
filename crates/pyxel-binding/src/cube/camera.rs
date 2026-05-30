use pyo3::prelude::*;

use super::mat4::Mat4;

define_wrapper!(Camera, pyxel::cube::Camera);

#[pymethods]
impl Camera {
    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::cube::Camera::new())
    }

    // Attributes

    #[getter]
    fn transform(&self) -> Mat4 {
        Mat4::wrap(self.inner_ref().transform.clone())
    }

    #[setter]
    fn set_transform(&self, mat: PyRef<'_, Mat4>) {
        self.inner_mut().transform = mat.inner.clone();
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

    #[getter]
    fn ortho_size(&self) -> Option<f32> {
        self.inner_ref().ortho_size
    }

    #[setter]
    fn set_ortho_size(&self, v: Option<f32>) {
        self.inner_mut().ortho_size = v;
    }

    #[getter]
    fn clear_color(&self) -> Option<i32> {
        self.inner_ref().clear_color
    }

    #[setter]
    fn set_clear_color(&self, v: Option<i32>) {
        self.inner_mut().clear_color = v;
    }

    // Dunder

    fn __repr__(&self) -> String {
        let c = self.inner_ref();
        let ortho = c
            .ortho_size
            .map_or_else(|| "None".to_string(), |v| format!("{v}"));
        format!(
            "Camera(fov={}, near={}, far={}, ortho_size={})",
            c.fov, c.near, c.far, ortho
        )
    }
}

pub fn add_camera_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Camera>()?;
    Ok(())
}
