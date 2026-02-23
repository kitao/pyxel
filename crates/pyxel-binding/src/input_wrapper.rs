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
pub fn warp_mouse(x: f32, y: f32) {
    pyxel().set_mouse_position(x, y);
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
