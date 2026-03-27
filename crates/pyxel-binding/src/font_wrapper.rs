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

    #[allow(clippy::mut_from_ref)]
    fn inner_mut(&self) -> &mut pyxel::Font {
        unsafe { &mut *self.inner }
    }
}

#[pymethods]
impl Font {
    // Constructor

    #[new]
    #[pyo3(signature = (filename, font_size=None))]
    fn new(filename: &str, font_size: Option<f32>) -> PyResult<Self> {
        pyxel::Font::new(filename, font_size)
            .map(Self::wrap)
            .map_err(pyo3::exceptions::PyException::new_err)
    }

    // Text measurement

    fn text_width(&self, s: &str) -> i32 {
        self.inner_mut().text_width(s)
    }
}

pub fn add_font_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Font>()?;
    Ok(())
}
