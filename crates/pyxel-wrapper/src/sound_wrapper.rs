use pyo3::prelude::*;

#[pyclass]
struct Sound;

pub fn add_sound_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Sound>()?;

    Ok(())
}
