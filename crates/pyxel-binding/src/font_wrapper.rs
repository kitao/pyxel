use pyo3::prelude::*;

define_wrapper!(Font, pyxel::Font);

#[pymethods]
impl Font {
    #[new]
    #[pyo3(signature = (filename, font_size=None))]
    fn new(filename: &str, font_size: Option<f32>) -> PyResult<Self> {
        pyxel::Font::new(filename, font_size)
            .map(Self::wrap)
            .map_err(pyo3::exceptions::PyException::new_err)
    }

    fn text_width(&self, s: &str) -> i32 {
        self.inner_mut().text_width(s)
    }
}

pub fn add_font_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Font>()?;
    Ok(())
}
