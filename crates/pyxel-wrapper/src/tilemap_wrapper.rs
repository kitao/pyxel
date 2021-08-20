use parking_lot::Mutex;
use pyo3::prelude::*;
use std::sync::Arc;

use pyxel::Tilemap as PyxelTilemap;

use crate::image_wrapper::Image;

#[pyclass]
#[derive(Clone)]
pub struct Tilemap {
    pub pyxel_tilemap: Arc<Mutex<PyxelTilemap>>,
}

pub fn wrap_pyxel_tilemap(pyxel_tilemap: Arc<Mutex<PyxelTilemap>>) -> Tilemap {
    Tilemap {
        pyxel_tilemap: pyxel_tilemap,
    }
}

#[pymethods]
impl Tilemap {
    #[new]
    pub fn new(width: u32, height: u32, image: Image) -> Tilemap {
        wrap_pyxel_tilemap(PyxelTilemap::with_arc_mutex(
            width,
            height,
            image.pyxel_image,
        ))
    }
}

pub fn add_tilemap_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Tilemap>()?;

    Ok(())
}
