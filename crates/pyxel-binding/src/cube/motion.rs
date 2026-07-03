use pyo3::prelude::*;

// Hand-rolled (rather than `define_wrapper!`) because the wrapper also
// carries `source_mesh`, the Mesh the motion came from. Motion is an
// engine-built payload exposed through `Mesh.motions`; it is not
// user-constructible. node.rs `validate_motion_source` compares
// `source_mesh` against a tree's originating Mesh to enforce the
// same-Mesh invariant for apply_motion / play_motion.

#[pyclass(unsendable, from_py_object)]
#[derive(Clone)]
pub struct Motion {
    pub(crate) inner: pyxel::cube::RcMotion,
    source_mesh: Option<pyxel::cube::RcMesh>,
}

impl Motion {
    pub(crate) fn wrap_with_source(
        inner: pyxel::cube::RcMotion,
        source_mesh: pyxel::cube::RcMesh,
    ) -> Self {
        Self {
            inner,
            source_mesh: Some(source_mesh),
        }
    }

    pub(crate) fn inner_ref(&self) -> &pyxel::cube::Motion {
        rc_ref!(self.inner)
    }

    pub(crate) fn source_mesh(&self) -> Option<pyxel::cube::RcMesh> {
        self.source_mesh.clone()
    }
}

#[pymethods]
impl Motion {
    // Attributes

    #[getter]
    fn name(&self) -> String {
        self.inner_ref().name.clone()
    }

    #[getter]
    fn length(&self) -> f32 {
        self.inner_ref().length
    }

    // Dunder

    fn __repr__(&self) -> String {
        format!("Motion(name={:?}, length={})", self.name(), self.length())
    }
}

// Module registration

pub fn add_motion_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Motion>()?;
    Ok(())
}
