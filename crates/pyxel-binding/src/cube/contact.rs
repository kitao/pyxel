use pyo3::prelude::*;

use super::quat::Quat;
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

    #[getter]
    fn depth(&self) -> f32 {
        self.inner_ref().depth
    }

    #[setter]
    fn set_depth(&self, v: f32) {
        self.inner_mut().depth = v;
    }

    #[getter]
    fn delta_rotation(&self) -> Quat {
        Quat::wrap(self.inner_ref().delta_rotation.clone())
    }

    #[setter]
    fn set_delta_rotation(&self, v: PyRef<'_, Quat>) {
        self.inner_mut().delta_rotation = v.inner.clone();
    }

    #[getter]
    fn delta_velocity(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().delta_velocity.clone())
    }

    #[setter]
    fn set_delta_velocity(&self, v: PyRef<'_, Vec3>) {
        self.inner_mut().delta_velocity = v.inner.clone();
    }

    #[getter]
    fn delta_angular_velocity(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().delta_angular_velocity.clone())
    }

    #[setter]
    fn set_delta_angular_velocity(&self, v: PyRef<'_, Vec3>) {
        self.inner_mut().delta_angular_velocity = v.inner.clone();
    }

    fn __repr__(&self) -> String {
        let c = self.inner_ref();
        let p = rc_ref!(&c.point);
        let n = rc_ref!(&c.normal);
        format!(
            "Contact(point=Vec3({}, {}, {}), normal=Vec3({}, {}, {}), depth={})",
            p.x, p.y, p.z, n.x, n.y, n.z, c.depth,
        )
    }
}

pub fn add_contact_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Contact>()?;
    Ok(())
}
