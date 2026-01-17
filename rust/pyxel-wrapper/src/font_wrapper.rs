use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Font {
    pub(crate) inner: pyxel::SharedFont,
}

impl Font {
    pub fn wrap(inner: pyxel::SharedFont) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Font {
    #[new]
    #[pyo3(signature = (filename, font_size=None))]
    pub fn new(filename: &str, font_size: Option<f32>) -> PyResult<Self> {
        pyxel::Font::new(filename, font_size)
            .map(Self::wrap)
            .map_err(pyo3::exceptions::PyException::new_err)
    }

    pub fn text_width(&self, s: &str) -> i32 {
        self.inner.lock().text_width(s)
    }
}

pub fn add_font_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Font>()?;
    Ok(())
}
