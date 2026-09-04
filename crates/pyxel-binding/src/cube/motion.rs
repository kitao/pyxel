use pyo3::prelude::*;

// Hand-rolled because engine-built motions retain their source Mesh;
// node.rs uses it to reject applying a motion to a different Mesh.

#[pyclass(module = "pyxel.cube", unsendable, from_py_object)]
#[derive(Clone)]
pub struct Motion {
    pub(crate) inner: pyxel::cube::RcMotion,
    source_mesh: pyxel::cube::RcMesh,
}

impl Motion {
    pub(crate) fn wrap_with_source(
        inner: pyxel::cube::RcMotion,
        source_mesh: pyxel::cube::RcMesh,
    ) -> Self {
        Self { inner, source_mesh }
    }

    pub(crate) fn inner_ref(&self) -> std::cell::Ref<'_, pyxel::cube::Motion> {
        rc_ref!(self.inner)
    }

    pub(crate) fn source_mesh(&self) -> &pyxel::cube::RcMesh {
        &self.source_mesh
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

    fn __repr__(&self) -> String {
        let motion = self.inner_ref();
        format!("Motion(name={:?}, length={})", motion.name, motion.length)
    }
}

pub fn add_motion_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Motion>()?;
    Ok(())
}
