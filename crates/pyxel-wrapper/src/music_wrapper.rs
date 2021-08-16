use pyo3::prelude::*;

#[pyclass]
struct Music;

pub fn add_music_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Music>()?;

    Ok(())
}
