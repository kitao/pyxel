use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;

use crate::instance;

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<PyObject> {
    let value = match name {
        "width" => instance().width().to_object(py),
        "height" => instance().height().to_object(py),
        "frame_count" => instance().frame_count().to_object(py),
        "mouse_x" => instance().mouse_x().to_object(py),
        "mouse_y" => instance().mouse_y().to_object(py),
        "mouse_wheel" => instance().mouse_wheel().to_object(py),
        "text_input" => instance().text_input().to_object(py),
        "drop_files" => instance().drop_files().to_object(py),

        _ => {
            return Err(PyAttributeError::new_err(format!(
                "module 'pyxel' has no attribute '{}'",
                name
            )))
        }
    };

    Ok(value)
}

pub fn add_module_variables(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;

    Ok(())
}
