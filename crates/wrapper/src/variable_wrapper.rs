use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;

use pyxel::Rgb8;

use crate::image_wrapper::wrap_pyxel_image;
use crate::instance;

#[pyclass]
struct Colors;

#[pyproto]
impl PySequenceProtocol for Colors {
    fn __len__(&self) -> usize {
        instance().colors.len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Rgb8> {
        Ok(instance().colors[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, value: Rgb8) {
        instance().colors[idx as usize] = value;
    }
}

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<PyObject> {
    let value = match name {
        //
        // System
        //
        "width" => instance().width().to_object(py),
        "height" => instance().height().to_object(py),
        "frame_count" => instance().frame_count().to_object(py),

        //
        // Input
        //
        "mouse_x" => instance().mouse_x().to_object(py),
        "mouse_y" => instance().mouse_y().to_object(py),
        "mouse_wheel" => instance().mouse_wheel().to_object(py),
        "text_input" => instance().text_input().to_object(py),
        "drop_files" => instance().drop_files().to_object(py),

        //
        // Graphics
        //
        "colors" => Py::new(py, Colors)?.into_py(py),
        "screen" => wrap_pyxel_image(instance().screen.clone()).into_py(py),
        "cursor" => wrap_pyxel_image(instance().cursor.clone()).into_py(py),
        "font" => wrap_pyxel_image(instance().font.clone()).into_py(py),

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
    m.add_class::<Colors>()?;
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;

    Ok(())
}
