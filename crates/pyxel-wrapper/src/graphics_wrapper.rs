use pyo3::prelude::*;
use pyxel::Color;

use crate::instance;

#[pyfunction]
fn cls(color: Color) {
    instance().cls(color);
}

pub fn add_graphics_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cls, m)?)?;

    Ok(())
}
