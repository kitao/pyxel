use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::{PyAttributeError, PyValueError};
use pyo3::prelude::*;
use pyxel::Rgb8;

use crate::image_wrapper::wrap_pyxel_image;
use crate::instance;

#[pyclass]
struct Colors;

impl Colors {
    #[allow(dead_code)]
    fn list(&self) -> &[Rgb8] {
        &instance().colors
    }

    #[allow(dead_code)]
    fn list_mut(&mut self) -> &mut [Rgb8] {
        &mut instance().colors
    }
}

#[pymethods]
impl Colors {
    pub fn assign(&mut self, list: Vec<Rgb8>) -> PyResult<()> {
        if self.list().len() == list.len() {
            self.list_mut()[..].clone_from_slice(&list[..]);
            Ok(())
        } else {
            Err(PyValueError::new_err("arrays must all be same length"))
        }
    }
}

#[pyproto]
impl PySequenceProtocol for Colors {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(Self::list, self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Rgb8> {
        define_list_get_operator!(Self::list, self, index)
    }

    fn __setitem__(&mut self, index: isize, value: Rgb8) -> PyResult<()> {
        define_list_set_operator!(Self::list_mut, self, index, value)
    }
}

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<PyObject> {
    let value = match name {
        // System
        "width" => instance().width().to_object(py),
        "height" => instance().height().to_object(py),
        "frame_count" => instance().frame_count().to_object(py),

        // Input
        "mouse_x" => instance().mouse_x().to_object(py),
        "mouse_y" => instance().mouse_y().to_object(py),
        "mouse_wheel" => instance().mouse_wheel().to_object(py),
        "input_keys" => instance().input_keys().to_object(py),
        "input_text" => instance().input_text().to_object(py),
        "drop_files" => instance().drop_files().to_object(py),

        // Graphics
        "colors" => Py::new(py, Colors)?.into_py(py),
        "screen" => wrap_pyxel_image(instance().screen.clone()).into_py(py),
        "cursor" => wrap_pyxel_image(instance().cursor.clone()).into_py(py),
        "font" => wrap_pyxel_image(instance().font.clone()).into_py(py),

        // others
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
