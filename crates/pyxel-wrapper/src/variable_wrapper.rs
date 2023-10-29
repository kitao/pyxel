use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;

use crate::image_wrapper::wrap_pyxel_image;
use crate::pyxel_singleton::pyxel;

wrap_as_python_list!(
    Colors,
    pyxel::Rgb24,
    u32,
    (|_| pyxel().colors.lock().len()),
    (|_, index| pyxel().colors.lock()[index]),
    (|_, index, value| pyxel().colors.lock()[index] = value),
    (|_, index, value| pyxel().colors.lock().insert(index, value)),
    (|_, index| pyxel().colors.lock().remove(index))
);

#[pyclass]
struct Images;

#[pyclass]
struct Tilemaps;

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<PyObject> {
    let value = match name {
        // System
        "width" => pyxel().width.to_object(py),
        "height" => pyxel().height.to_object(py),
        "frame_count" => pyxel().frame_count.to_object(py),

        // Input
        "mouse_x" => pyxel().mouse_x.to_object(py),
        "mouse_y" => pyxel().mouse_y.to_object(py),
        "mouse_wheel" => pyxel().mouse_wheel.to_object(py),
        "input_text" => pyxel().input_text.to_object(py),
        "dropped_files" => pyxel().dropped_files.to_object(py),

        // Graphics
        "colors" => Py::new(py, Colors { inner: 0 })?.into_py(py),
        "images" => Py::new(py, Images)?.into_py(py),
        "tilemaps" => Py::new(py, Tilemaps)?.into_py(py),
        "screen" => wrap_pyxel_image(pyxel().screen.clone()).into_py(py),
        "cursor" => wrap_pyxel_image(pyxel().cursor.clone()).into_py(py),
        "font" => wrap_pyxel_image(pyxel().font.clone()).into_py(py),

        // Audio

        // Others
        _ => {
            return Err(PyAttributeError::new_err(format!(
                "module 'pyxel' has no attribute '{name}'"
            )))
        }
    };
    Ok(value)
}

pub fn add_module_variables(m: &PyModule) -> PyResult<()> {
    m.add_class::<Colors>()?;
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;
    Ok(())
}
