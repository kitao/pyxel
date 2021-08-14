use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;
use pyxel::Color;

use crate::instance;

#[pyclass]
pub struct Palette;

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
