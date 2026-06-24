use std::cell::RefCell;

use pyo3::prelude::*;

use super::node::Node;
use super::vec3::Vec3;

// Hand-rolled (rather than `define_wrapper!`) because `node` must keep
// the same Py<Node> instance the scene tree holds. RaycastHit is an
// engine-built payload returned by raycast / raycast_all; it is not
// user-constructible and its fields are read-only. Engine-built hits
// route through `wrap_with_py_node` so `hit.node is scene_node` holds;
// `wrap` leaves `py_node` unset and the getter falls back to wrapping
// the core RcNode.

#[pyclass(unsendable, from_py_object)]
pub struct RaycastHit {
    pub(crate) inner: pyxel::cube::RcRaycastHit,
    py_node: RefCell<Option<Py<Node>>>,
}

impl Clone for RaycastHit {
    fn clone(&self) -> Self {
        Python::try_attach(|py| Self {
            inner: self.inner.clone(),
            py_node: RefCell::new(self.py_node.borrow().as_ref().map(|p| p.clone_ref(py))),
        })
        .expect("RaycastHit clone requires the Python GIL to be attached")
    }
}

impl RaycastHit {
    pub fn wrap(inner: pyxel::cube::RcRaycastHit) -> Self {
        Self {
            inner,
            py_node: RefCell::new(None),
        }
    }

    pub fn wrap_with_py_node(inner: pyxel::cube::RcRaycastHit, py_node: Py<Node>) -> Self {
        Self {
            inner,
            py_node: RefCell::new(Some(py_node)),
        }
    }

    pub(crate) fn inner_ref(&self) -> &pyxel::cube::RaycastHit {
        rc_ref!(self.inner)
    }
}

#[pymethods]
impl RaycastHit {
    #[getter]
    fn node(&self, py: Python<'_>) -> PyResult<Py<Node>> {
        if let Some(p) = self.py_node.borrow().as_ref() {
            return Ok(p.clone_ref(py));
        }
        match &self.inner_ref().node {
            Some(n) => Py::new(py, Node::wrap(n.clone())),
            None => Err(pyo3::exceptions::PyValueError::new_err(
                "RaycastHit.node is not set",
            )),
        }
    }

    #[getter]
    fn point(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().point.clone())
    }

    #[getter]
    fn normal(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().normal.clone())
    }

    #[getter]
    fn distance(&self) -> f32 {
        self.inner_ref().distance
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
