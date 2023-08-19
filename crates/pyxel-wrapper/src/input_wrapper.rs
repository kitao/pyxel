use pyo3::prelude::*;
use pyxel_engine::{Key, KeyValue};

#[pyfunction]
fn btn(key: Key) -> bool {
    pyxel_engine::btn(key)
}

#[pyfunction]
#[pyo3(text_signature = "(key, *, hold, repeat)")]
fn btnp(key: Key, hold: Option<u32>, repeat: Option<u32>) -> bool {
    pyxel_engine::btnp(key, hold, repeat)
}

#[pyfunction]
fn btnr(key: Key) -> bool {
    pyxel_engine::btnr(key)
}

#[pyfunction]
fn btnv(key: Key) -> KeyValue {
    pyxel_engine::btnv(key)
}

#[pyfunction]
fn mouse(visible: bool) {
    pyxel_engine::mouse(visible);
}

#[pyfunction]
pub fn set_mouse_pos(x: f64, y: f64) {
    pyxel_engine::set_mouse_pos(x, y);
}

pub fn add_input_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(btn, m)?)?;
    m.add_function(wrap_pyfunction!(btnp, m)?)?;
    m.add_function(wrap_pyfunction!(btnr, m)?)?;
    m.add_function(wrap_pyfunction!(btnv, m)?)?;
    m.add_function(wrap_pyfunction!(mouse, m)?)?;
    m.add_function(wrap_pyfunction!(set_mouse_pos, m)?)?;
    Ok(())
}
