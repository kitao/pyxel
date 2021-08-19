use pyo3::prelude::*;

use pyxel::{Key, KeyValue};

use crate::instance;

#[pyfunction]
pub fn mouse(is_visible: bool) {
    instance().mouse(is_visible);
}

#[pyfunction]
pub fn btn(key: Key) -> bool {
    instance().btn(key)
}

#[pyfunction]
pub fn btnp(key: Key, hold_frame_count: Option<u32>, repeat_frame_count: Option<u32>) -> bool {
    instance().btnp(key, hold_frame_count, repeat_frame_count)
}

#[pyfunction]
pub fn btnr(key: Key) -> bool {
    instance().btnr(key)
}

#[pyfunction]
pub fn btnv(key: Key) -> KeyValue {
    instance().btnv(key)
}

pub fn add_input_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mouse, m)?)?;
    m.add_function(wrap_pyfunction!(btn, m)?)?;
    m.add_function(wrap_pyfunction!(btnp, m)?)?;
    m.add_function(wrap_pyfunction!(btnr, m)?)?;
    m.add_function(wrap_pyfunction!(btnv, m)?)?;

    Ok(())
}
