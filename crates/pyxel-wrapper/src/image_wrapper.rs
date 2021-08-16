use parking_lot::Mutex;
use pyo3::prelude::*;
use pyxel::Canvas;
use pyxel::Image as PyxelImage;
use std::sync::Arc;

#[pyclass]
pub struct Image {
    image: Arc<Mutex<PyxelImage>>,
}

#[pymethods]
impl Image {
    #[new]
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            image: PyxelImage::with_arc_mutex(width, height),
        }
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.image.lock().width()
    }
}

pub fn add_image_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Image>()?;

    Ok(())
}
