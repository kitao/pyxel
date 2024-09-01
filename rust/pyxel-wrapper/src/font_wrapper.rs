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
    pub fn new(filename: &str) -> Self {
        Self::wrap(pyxel::Font::new(filename))
    }

    pub fn text_width(&self, s: &str) -> i32 {
        self.inner.lock().text_width(s)
    }
}

pub fn add_font_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Font>()?;
    Ok(())
}
