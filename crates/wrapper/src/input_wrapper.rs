use pyo3::prelude::*;

use pyxel::{Key, KeyValue};

use crate::instance;

#[pyfunction]
fn mouse(visible: bool) -> PyResult<()> {
    instance().mouse(visible);

    Ok(())
}

#[pyfunction]
fn btn(key: Key) -> PyResult<bool> {
    Ok(instance().btn(key))
}

#[pyfunction]
#[pyo3(text_signature = "(key, *, hold, repeat)")]
fn btnp(key: Key, hold: Option<u32>, repeat: Option<u32>) -> PyResult<bool> {
    Ok(instance().btnp(key, hold, repeat))
}

#[pyfunction]
fn btnr(key: Key) -> PyResult<bool> {
    Ok(instance().btnr(key))
}

#[pyfunction]
fn btnv(key: Key) -> PyResult<KeyValue> {
    Ok(instance().btnv(key))
}

#[pyfunction]
pub fn set_btnp(key: Key) -> PyResult<()> {
    instance().set_btnp(key);

    Ok(())
}

#[pyfunction]
pub fn set_btnr(key: Key) -> PyResult<()> {
    instance().set_btnr(key);

    Ok(())
}

#[pyfunction]
pub fn set_btnv(key: Key, val: KeyValue) -> PyResult<()> {
    instance().set_btnv(key, val);

    Ok(())
}

pub fn add_input_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mouse, m)?)?;
    m.add_function(wrap_pyfunction!(btn, m)?)?;
    m.add_function(wrap_pyfunction!(btnp, m)?)?;
    m.add_function(wrap_pyfunction!(btnr, m)?)?;
    m.add_function(wrap_pyfunction!(btnv, m)?)?;
    m.add_function(wrap_pyfunction!(set_btnp, m)?)?;
    m.add_function(wrap_pyfunction!(set_btnr, m)?)?;
    m.add_function(wrap_pyfunction!(set_btnv, m)?)?;

    Ok(())
}
