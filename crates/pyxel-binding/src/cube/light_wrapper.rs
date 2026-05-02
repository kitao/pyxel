use pyo3::prelude::*;

use crate::cube::math_wrapper::Vec3;

define_wrapper!(Light, pyxel::cube::Light);

#[pymethods]
impl Light {
    #[new]
    fn new(dir: &Vec3) -> Self {
        Self::wrap(pyxel::cube::Light::new(dir.inner))
    }

    #[getter]
    fn dir(&self) -> Vec3 {
        Vec3 {
            inner: self.inner_ref().dir,
        }
    }

    #[setter]
    fn set_dir(&self, v: &Vec3) {
        self.inner_mut().dir = v.inner.normalize();
    }
}

pub fn add_cube_light_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Light>()?;
    Ok(())
}
