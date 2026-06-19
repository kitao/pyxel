use pyo3::prelude::*;

#[pyclass(unsendable, from_py_object)]
#[derive(Clone)]
pub struct Motion {
    pub(crate) inner: pyxel::cube::RcMotion,
    source_mesh: Option<pyxel::cube::RcMesh>,
}

impl Motion {
    #[allow(dead_code)]
    pub fn wrap(inner: pyxel::cube::RcMotion) -> Self {
        Self {
            inner,
            source_mesh: None,
        }
    }

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
    #[getter]
    fn name(&self) -> String {
        self.inner_ref().name.clone()
    }

    #[getter]
    fn length(&self) -> f32 {
        self.inner_ref().length
    }

    fn __repr__(&self) -> String {
        format!("Motion(name={:?}, length={})", self.name(), self.length())
    }
}

pub fn add_motion_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Motion>()?;
    Ok(())
}
