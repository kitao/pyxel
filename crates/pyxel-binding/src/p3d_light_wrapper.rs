use pyo3::prelude::*;
use pyxel::p3d;

use crate::p3d_math_wrapper::Vec3;

#[pyclass(name = "Light")]
pub struct Light {
    pub inner: p3d::Light,
}

#[pymethods]
impl Light {
    #[new]
    fn new(dir: &Vec3) -> Self {
        Self {
            inner: p3d::Light::new(dir.inner),
        }
    }

    #[getter]
    fn dir(&self) -> Vec3 {
        Vec3 {
            inner: self.inner.dir,
        }
    }

    #[setter]
    fn set_dir(&mut self, v: &Vec3) {
        self.inner.dir = v.inner.normalize();
    }
}

pub fn add_p3d_light_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Light>()?;
    Ok(())
}
