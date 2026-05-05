use pyo3::prelude::*;

use super::vec3::Vec3;

define_wrapper!(Light, pyxel::cube::Light);

#[pymethods]
impl Light {
    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::cube::Light::new())
    }

    // Attributes

    #[getter]
    fn ambient(&self) -> f32 {
        self.inner_ref().ambient
    }

    #[setter]
    fn set_ambient(&self, v: f32) {
        self.inner_mut().ambient = v;
    }

    #[getter]
    fn direction(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().direction.clone())
    }

    #[setter]
    fn set_direction(&self, v: PyRef<'_, Vec3>) {
        self.inner_mut().direction = v.inner.clone();
    }

    #[getter]
    fn intensity(&self) -> f32 {
        self.inner_ref().intensity
    }

    #[setter]
    fn set_intensity(&self, v: f32) {
        self.inner_mut().intensity = v;
    }

    // Dunder

    fn __repr__(&self) -> String {
        let l = self.inner_ref();
        let dir = rc_ref!(&l.direction);
        format!(
            "Light(ambient={}, direction=Vec3({}, {}, {}), intensity={})",
            l.ambient, dir.x, dir.y, dir.z, l.intensity
        )
    }
}

pub fn add_light_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Light>()?;
    Ok(())
}
