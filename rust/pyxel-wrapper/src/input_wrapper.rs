use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;

#[pyfunction]
fn btn(key: pyxel::Key) -> bool {
    pyxel().btn(key)
}

#[pyfunction]
#[pyo3(signature = (key, hold=None, repeat=None))]
fn btnp(key: pyxel::Key, hold: Option<u32>, repeat: Option<u32>) -> bool {
    pyxel().btnp(key, hold, repeat)
}

#[pyfunction]
fn btnr(key: pyxel::Key) -> bool {
    pyxel().btnr(key)
}

#[pyfunction]
fn btnv(key: pyxel::Key) -> pyxel::KeyValue {
    pyxel().btnv(key)
}

#[pyfunction]
fn mouse(visible: bool) {
    pyxel().mouse(visible);
}

#[pyfunction]
pub fn warp_mouse(x: f64, y: f64) {
    pyxel().warp_mouse(x, y);
}

pub fn add_input_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(btn, m)?)?;
    m.add_function(wrap_pyfunction!(btnp, m)?)?;
    m.add_function(wrap_pyfunction!(btnr, m)?)?;
    m.add_function(wrap_pyfunction!(btnv, m)?)?;
    m.add_function(wrap_pyfunction!(mouse, m)?)?;
    m.add_function(wrap_pyfunction!(warp_mouse, m)?)?;
    Ok(())
}
