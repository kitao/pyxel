use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use pyxel::Pyxel;

#[pyfunction]
fn test(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn pyxelcore(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;

    Ok(())
}
