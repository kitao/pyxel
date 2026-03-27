use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;

#[pyfunction]
fn btn(key: pyxel::Key) -> bool {
    pyxel().is_button_down(key)
}

#[pyfunction]
#[pyo3(signature = (key, hold=None, repeat=None))]
fn btnp(key: pyxel::Key, hold: Option<u32>, repeat: Option<u32>) -> bool {
    pyxel().is_button_pressed(key, hold, repeat)
}

#[pyfunction]
fn btnr(key: pyxel::Key) -> bool {
    pyxel().is_button_released(key)
}

#[pyfunction]
fn btnv(key: pyxel::Key) -> pyxel::KeyValue {
    pyxel().button_value(key)
}

#[pyfunction]
fn mouse(visible: bool) {
    pyxel().set_mouse_visible(visible);
}

#[pyfunction]
fn set_btn(key: pyxel::Key, state: bool) {
    pyxel().set_button_state(key, state);
}

#[pyfunction]
fn set_btnv(key: pyxel::Key, val: pyxel::KeyValue) {
    pyxel().set_button_value(key, val);
}

#[pyfunction]
fn set_mouse_pos(x: f32, y: f32) {
    pyxel().set_mouse_position(x, y);
}

#[pyfunction]
fn set_input_text(text: &str) {
    pyxel().set_input_text(text);
}

#[pyfunction]
fn set_dropped_files(files: Vec<String>) {
    let refs: Vec<&str> = files.iter().map(String::as_str).collect();
    pyxel().set_dropped_files(&refs);
}

pub fn add_input_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(btn, m)?)?;
    m.add_function(wrap_pyfunction!(btnp, m)?)?;
    m.add_function(wrap_pyfunction!(btnr, m)?)?;
    m.add_function(wrap_pyfunction!(btnv, m)?)?;
    m.add_function(wrap_pyfunction!(mouse, m)?)?;
    m.add_function(wrap_pyfunction!(set_btn, m)?)?;
    m.add_function(wrap_pyfunction!(set_btnv, m)?)?;
    m.add_function(wrap_pyfunction!(set_mouse_pos, m)?)?;
    m.add_function(wrap_pyfunction!(set_input_text, m)?)?;
    m.add_function(wrap_pyfunction!(set_dropped_files, m)?)?;
    Ok(())
}
