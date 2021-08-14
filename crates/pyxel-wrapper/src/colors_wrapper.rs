use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;
use pyxel::Rgb8;

use crate::instance;

#[pyclass]
pub struct Colors;

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
