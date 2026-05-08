use pyo3::prelude::*;

define_wrapper!(Collider, pyxel::cube::Collider);

#[pymethods]
impl Collider {
    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::cube::Collider::new())
    }

    fn __repr__(&self) -> String {
        "Collider()".to_string()
    }
}

pub fn add_collider_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Collider>()?;
    Ok(())
}
