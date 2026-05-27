use pyo3::prelude::*;

use super::node::Node;
use super::vec3::Vec3;

define_wrapper!(RaycastHit, pyxel::cube::RaycastHit);

#[pymethods]
impl RaycastHit {
    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::cube::RaycastHit::new())
    }

    #[getter]
    fn node(&self) -> PyResult<Node> {
        match &self.inner_ref().node {
            Some(n) => Ok(Node::wrap(n.clone())),
            None => Err(pyo3::exceptions::PyValueError::new_err(
                "RaycastHit.node is not set",
            )),
        }
    }

    #[setter]
    fn set_node(&self, v: PyRef<'_, Node>) {
        self.inner_mut().node = Some(v.inner.clone());
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
    fn distance(&self) -> f32 {
        self.inner_ref().distance
    }

    #[setter]
    fn set_distance(&self, v: f32) {
        self.inner_mut().distance = v;
    }

    fn __repr__(&self) -> String {
        let h = self.inner_ref();
        let p = rc_ref!(&h.point);
        let n = rc_ref!(&h.normal);
        format!(
            "RaycastHit(point=Vec3({}, {}, {}), normal=Vec3({}, {}, {}), distance={})",
            p.x, p.y, p.z, n.x, n.y, n.z, h.distance,
        )
    }
}

pub fn add_raycast_hit_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RaycastHit>()?;
    Ok(())
}
