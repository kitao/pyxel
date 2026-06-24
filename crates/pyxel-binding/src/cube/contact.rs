use pyo3::prelude::*;

use super::quat::Quat;
use super::vec3::Vec3;

define_wrapper!(Contact, pyxel::cube::Contact);

// Contact is an engine-built payload passed to on_collide; it is not
// user-constructible and its fields are read-only.
#[pymethods]
impl Contact {
    #[getter]
    fn point(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().point.clone())
    }

    #[getter]
    fn normal(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().normal.clone())
    }

    #[getter]
    fn depth(&self) -> f32 {
        self.inner_ref().depth
    }

    #[getter]
    fn delta_rotation(&self) -> Quat {
        Quat::wrap(self.inner_ref().delta_rotation.clone())
    }

    #[getter]
    fn delta_velocity(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().delta_velocity.clone())
    }

    #[getter]
    fn delta_angular_velocity(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().delta_angular_velocity.clone())
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
