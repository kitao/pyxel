use pyo3::prelude::*;
use pyxel::{Key, KeyValue};

use crate::instance;

#[pyfunction]
fn btn(key: Key) -> bool {
    instance().btn(key)
}

#[pyfunction]
#[pyo3(text_signature = "(key, *, hold, repeat)")]
fn btnp(key: Key, hold: Option<u32>, repeat: Option<u32>) -> bool {
    instance().btnp(key, hold, repeat)
}

#[pyfunction]
fn btnr(key: Key) -> bool {
    instance().btnr(key)
}

#[pyfunction]
fn btnv(key: Key) -> KeyValue {
    instance().btnv(key)
}

#[pyfunction]
fn mouse(visible: bool) {
    instance().mouse(visible);
}

#[pyfunction]
pub fn setbtn(key: Key, state: bool) {
    instance().setbtn(key, state);
}

#[pyfunction]
pub fn setbtnv(key: Key, val: f64) {
    instance().setbtnv(key, val);
}

#[pyfunction]
pub fn setmpos(x: f64, y: f64) {
    instance().setmpos(x, y);
}

pub fn add_input_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(btn, m)?)?;
    m.add_function(wrap_pyfunction!(btnp, m)?)?;
    m.add_function(wrap_pyfunction!(btnr, m)?)?;
    m.add_function(wrap_pyfunction!(btnv, m)?)?;
    m.add_function(wrap_pyfunction!(mouse, m)?)?;
    m.add_function(wrap_pyfunction!(setbtn, m)?)?;
    m.add_function(wrap_pyfunction!(setbtnv, m)?)?;
    m.add_function(wrap_pyfunction!(setmpos, m)?)?;
    Ok(())
}
