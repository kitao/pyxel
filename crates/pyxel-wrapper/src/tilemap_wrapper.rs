use pyo3::prelude::*;

#[pyclass]
struct Tilemap;

pub fn add_tilemap_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Tilemap>()?;

    Ok(())
}
