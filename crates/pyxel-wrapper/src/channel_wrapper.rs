use pyo3::prelude::*;

#[pyclass]
struct Channel;

pub fn add_channel_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Channel>()?;

    Ok(())
}
