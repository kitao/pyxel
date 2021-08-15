use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;
use pyxel::{Color, Rgb8};

use crate::instance;

#[pyclass]
struct Colors;

#[pymethods]
impl Colors {
    #[new]
    pub fn new() -> Colors {
        Colors
    }
}

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

pub fn add_colors_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Colors>()?;

    Ok(())
}

#[pyclass]
struct Palette;

#[pymethods]
impl Palette {
    #[new]
    pub fn new() -> Palette {
        Palette
    }
}

#[pyproto]
impl PySequenceProtocol for Palette {
    fn __len__(&self) -> usize {
        instance().palette.len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Color> {
        Ok(instance().palette[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, value: Color) {
        instance().palette[idx as usize] = value;
    }
}

pub fn add_palette_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Palette>()?;

    Ok(())
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
        "colors" => Py::new(py, Colors::new())?.into_py(py),
        "palette" => Py::new(py, Palette::new())?.into_py(py),
        "screen" => instance().mouse_x().to_object(py), // dummy
        "cursor" => instance().mouse_x().to_object(py), // dummy
        "font" => instance().mouse_x().to_object(py),   // dummy

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
    m.add_class::<Palette>()?;
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;

    Ok(())
}
