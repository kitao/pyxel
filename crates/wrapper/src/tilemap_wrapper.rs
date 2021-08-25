use pyo3::prelude::*;

use pyxel::SharedTilemap as PyxelSharedTilemap;
use pyxel::Tilemap as PyxelTilemap;

use crate::image_wrapper::{wrap_pyxel_image, Image};

#[pyclass]
#[derive(Clone)]
pub struct Tilemap {
    pub pyxel_tilemap: PyxelSharedTilemap,
}

pub fn wrap_pyxel_tilemap(pyxel_tilemap: PyxelSharedTilemap) -> Tilemap {
    Tilemap {
        pyxel_tilemap: pyxel_tilemap,
    }
}

#[pymethods]
impl Tilemap {
    #[new]
    pub fn new(width: u32, height: u32, image: Image) -> Tilemap {
        wrap_pyxel_tilemap(PyxelTilemap::new(width, height, image.pyxel_image))
    }

    #[getter]
    pub fn image(&self) -> Image {
        wrap_pyxel_image(self.pyxel_tilemap.lock().image.clone())
    }

    #[setter]
    pub fn set_image(&self, image: Image) {
        self.pyxel_tilemap.lock().image = image.pyxel_image.clone();
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: Vec<&str>) {
        self.pyxel_tilemap.lock().set(x, y, &data_str);
    }
}

pub fn add_tilemap_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Tilemap>()?;

    Ok(())
}
