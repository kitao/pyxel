use pyo3::prelude::*;

use pyxel::SharedTilemap as PyxelSharedTilemap;
use pyxel::Tilemap as PyxelTilemap;

use crate::image_wrapper::{wrap_pyxel_image, Image};
use crate::instance;

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
    pub fn new(width: &PyAny, height: &PyAny, image: &PyAny) -> PyResult<Tilemap> {
        let image = type_switch! {
            image,
            Image,
            {
                image.pyxel_image.clone()
            },
            u32,
            {
                instance().image(image).clone()
            }
        };

        Ok(wrap_pyxel_tilemap(PyxelTilemap::new(
            as_u32!(width),
            as_u32!(height),
            image,
        )))
    }

    #[getter]
    pub fn image(&self) -> PyResult<Image> {
        Ok(wrap_pyxel_image(self.pyxel_tilemap.lock().image.clone()))
    }

    #[setter]
    pub fn set_image(&self, image: &PyAny) -> PyResult<()> {
        type_switch! {
            image,
            Image, {
                self.pyxel_tilemap.lock().image = image.pyxel_image.clone();
            },
            u32, {
                self.pyxel_tilemap.lock().image = instance().image(image).clone();
            }
        }

        Ok(())
    }

    pub fn set(&mut self, x: &PyAny, y: &PyAny, data_str: Vec<&str>) -> PyResult<()> {
        self.pyxel_tilemap
            .lock()
            .set(as_i32!(x), as_i32!(y), &data_str);

        Ok(())
    }
}

pub fn add_tilemap_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Tilemap>()?;

    Ok(())
}
