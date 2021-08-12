use pyo3::prelude::*;

pub fn add_module_constants(m: &PyModule) -> PyResult<()> {
    m.add("TEST", 1234)?;

    Ok(())
}
