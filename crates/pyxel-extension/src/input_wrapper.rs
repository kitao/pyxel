use pyo3::prelude::*;
use pyxel::{Key, KeyValue};

#[pyfunction]
fn btn(key: Key) -> bool {
    pyxel::btn(key)
}

#[pyfunction]
#[pyo3(text_signature = "(key, *, hold, repeat)")]
fn btnp(key: Key, hold: Option<u32>, repeat: Option<u32>) -> bool {
    pyxel::btnp(key, hold, repeat)
}

#[pyfunction]
fn btnr(key: Key) -> bool {
    pyxel::btnr(key)
}

#[pyfunction]
fn btnv(key: Key) -> KeyValue {
    pyxel::btnv(key)
}

#[pyfunction]
fn mouse(visible: bool) {
    pyxel::mouse(visible);
}

#[pyfunction]
pub fn set_btn(key: Key, state: bool) {
    pyxel::set_btn(key, state);
}

#[pyfunction]
pub fn set_btnv(key: Key, val: f64) {
    pyxel::set_btnv(key, val);
}

#[pyfunction]
pub fn set_mouse_pos(x: f64, y: f64) {
    pyxel::set_mouse_pos(x, y);
}

pub fn add_input_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(btn, m)?)?;
    m.add_function(wrap_pyfunction!(btnp, m)?)?;
    m.add_function(wrap_pyfunction!(btnr, m)?)?;
    m.add_function(wrap_pyfunction!(btnv, m)?)?;
    m.add_function(wrap_pyfunction!(mouse, m)?)?;
    m.add_function(wrap_pyfunction!(set_btn, m)?)?;
    m.add_function(wrap_pyfunction!(set_btnv, m)?)?;
    m.add_function(wrap_pyfunction!(set_mouse_pos, m)?)?;
    Ok(())
}
