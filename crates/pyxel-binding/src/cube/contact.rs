use pyo3::prelude::*;

use super::vec3::Vec3;

define_wrapper!(Contact, pyxel::cube::Contact);

#[pymethods]
impl Contact {
    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::cube::Contact::new())
    }

    #[getter]
    fn point(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().point.clone())
    }

    #[setter]
    fn set_point(&self, v: PyRef<'_, Vec3>) {
        self.inner_mut().point = v.inner.clone();
    }

    #[getter]
    fn normal(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().normal.clone())
    }

    #[setter]
    fn set_normal(&self, v: PyRef<'_, Vec3>) {
        self.inner_mut().normal = v.inner.clone();
    }

    fn __repr__(&self) -> String {
        let c = self.inner_ref();
        let p = rc_ref!(&c.point);
        let n = rc_ref!(&c.normal);
        format!(
            "Contact(point=Vec3({}, {}, {}), normal=Vec3({}, {}, {}))",
            p.x, p.y, p.z, n.x, n.y, n.z
        )
    }
}

pub fn add_contact_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Contact>()?;
    Ok(())
}
