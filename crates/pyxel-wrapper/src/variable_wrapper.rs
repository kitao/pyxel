use std::ptr;

use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;
use pyxel_engine::Rgb8;

use crate::image_wrapper::wrap_pyxel_image;

#[pyclass]
struct Colors;

impl Colors {
    fn list(&self) -> &[Rgb8] {
        unsafe { &*ptr::addr_of!(*pyxel_engine::colors().lock()) }
    }

    fn list_mut(&mut self) -> &mut [Rgb8] {
        unsafe { &mut *ptr::addr_of_mut!(*pyxel_engine::colors().lock()) }
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

    pub fn from_list(&mut self, lst: Vec<Rgb8>) {
        let copy_size = usize::min(self.list().len(), lst.len());
        self.list_mut()[..copy_size].clone_from_slice(&lst[..copy_size]);
    }

    pub fn to_list(&self) -> PyResult<Vec<Rgb8>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<PyObject> {
    let value = match name {
        // System
        "width" => pyxel_engine::width().to_object(py),
        "height" => pyxel_engine::height().to_object(py),
        "frame_count" => pyxel_engine::frame_count().to_object(py),
        "is_fullscreen" => pyxel_engine::is_fullscreen().to_object(py),

        // Input
        "mouse_x" => pyxel_engine::mouse_x().to_object(py),
        "mouse_y" => pyxel_engine::mouse_y().to_object(py),
        "mouse_wheel" => pyxel_engine::mouse_wheel().to_object(py),
        "input_text" => pyxel_engine::input_text().to_object(py),
        "drop_files" => pyxel_engine::drop_files().to_object(py),

        // Graphics
        "colors" => Py::new(py, Colors)?.into_py(py),
        "screen" => wrap_pyxel_image(pyxel_engine::screen()).into_py(py),
        "cursor" => wrap_pyxel_image(pyxel_engine::cursor()).into_py(py),
        "font" => wrap_pyxel_image(pyxel_engine::font()).into_py(py),

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
