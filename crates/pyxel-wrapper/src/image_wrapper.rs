use std::sync::{Arc, Mutex};

use pyo3::prelude::*;
use pyxel::Image as PyxelImage;

#[pyclass]
struct Image {
    image: Arc<Mutex<PyxelImage>>,
}

#[pymethods]
impl Image {
    #[new]
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            image: Arc::new(Mutex::new(PyxelImage::new(width, height))),
        }
    }
}
