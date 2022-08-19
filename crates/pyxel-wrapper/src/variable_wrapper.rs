use std::ptr;

use pyo3::exceptions::{PyAttributeError, PyValueError};
use pyo3::prelude::*;
use pyxel::Rgb8;

use crate::image_wrapper::wrap_pyxel_image;

#[pyclass]
struct Colors;

impl Colors {
    fn list(&self) -> &[Rgb8] {
        unsafe { &*ptr::addr_of!(*pyxel::colors().lock()) }
    }

    fn list_mut(&mut self) -> &mut [Rgb8] {
        unsafe { &mut *ptr::addr_of_mut!(*pyxel::colors().lock()) }
    }
}

#[pymethods]
impl Colors {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Rgb8> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: Rgb8) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }

    pub fn from_list(&mut self, lst: Vec<Rgb8>) -> PyResult<()> {
        if self.list().len() == lst.len() {
            self.list_mut()[..].clone_from_slice(&lst[..]);
            Ok(())
        } else {
            Err(PyValueError::new_err("list must be same length as array"))
        }
    }

    pub fn to_list(&self) -> PyResult<Vec<Rgb8>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<PyObject> {
    let value = match name {
        // System
        "width" => pyxel::width().to_object(py),
        "height" => pyxel::height().to_object(py),
        "frame_count" => pyxel::frame_count().to_object(py),
        "is_fullscreen" => pyxel::is_fullscreen().to_object(py),

        // Input
        "mouse_x" => pyxel::mouse_x().to_object(py),
        "mouse_y" => pyxel::mouse_y().to_object(py),
        "mouse_wheel" => pyxel::mouse_wheel().to_object(py),
        "input_keys" => pyxel::input_keys().to_object(py),
        "input_text" => pyxel::input_text().to_object(py),
        "drop_files" => pyxel::drop_files().to_object(py),

        // Graphics
        "colors" => Py::new(py, Colors)?.into_py(py),
        "screen" => wrap_pyxel_image(pyxel::screen()).into_py(py),
        "cursor" => wrap_pyxel_image(pyxel::cursor()).into_py(py),
        "font" => wrap_pyxel_image(pyxel::font()).into_py(py),

        // Others
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
