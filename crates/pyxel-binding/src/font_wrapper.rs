use pyo3::prelude::*;

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub struct Font {
    pub(crate) inner: *mut pyxel::Font,
}

unsafe impl Send for Font {}
unsafe impl Sync for Font {}

impl Font {
    pub fn wrap(inner: *mut pyxel::Font) -> Self {
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
        unsafe { &mut *self.inner }.text_width(s)
    }
}

pub fn add_font_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Font>()?;
    Ok(())
}
