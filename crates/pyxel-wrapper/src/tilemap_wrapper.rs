use parking_lot::Mutex;
use pyo3::prelude::*;
use std::sync::Arc;

use pyxel::Tilemap as PyxelTilemap;

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
    pub fn new(width: u32, height: u32) -> Tilemap {
        wrap_pyxel_tilemap(PyxelTilemap::with_arc_mutex(width, height))
    }
}

pub fn add_tilemap_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Tilemap>()?;

    Ok(())
}
