use pyo3::prelude::*;
use pyxel::cube;

use crate::cube::math_wrapper::Vec3;

#[pyclass(name = "Light")]
pub struct Light {
    pub inner: cube::Light,
}

#[pymethods]
impl Light {
    #[new]
    fn new(dir: &Vec3) -> Self {
        Self {
            inner: cube::Light::new(dir.inner),
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

pub fn add_cube_light_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Light>()?;
    Ok(())
}
