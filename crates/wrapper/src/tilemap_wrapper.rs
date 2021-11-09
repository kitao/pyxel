use pyo3::prelude::*;
use pyxel::SharedTilemap as PyxelSharedTilemap;
use pyxel::Tile;
use pyxel::Tilemap as PyxelTilemap;

use crate::image_wrapper::{wrap_pyxel_image, Image};
use crate::instance;

#[pyclass]
#[derive(Clone)]
pub struct Tilemap {
    pub pyxel_tilemap: PyxelSharedTilemap,
}

pub fn wrap_pyxel_tilemap(pyxel_tilemap: PyxelSharedTilemap) -> Tilemap {
    Tilemap { pyxel_tilemap }
}

#[pymethods]
impl Tilemap {
    #[new]
    pub fn new(width: u32, height: u32, img: &PyAny) -> PyResult<Self> {
        let img = type_switch! {
            img,
            u32, {
                instance().image(img)
            },
            Image, {
                img.pyxel_image
            }
        };
        Ok(wrap_pyxel_tilemap(PyxelTilemap::new(width, height, img)))
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.pyxel_tilemap.lock().width()
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.pyxel_tilemap.lock().height()
    }

    #[getter]
    pub fn image(&self) -> Image {
        wrap_pyxel_image(self.pyxel_tilemap.lock().image.clone())
    }

    #[setter]
    pub fn set_image(&self, image: Image) {
        self.pyxel_tilemap.lock().image = image.pyxel_image;
    }

    #[getter]
    pub fn refimg(&self) -> Option<u32> {
        instance().image_no(self.pyxel_tilemap.lock().image.clone())
    }

    #[setter]
    pub fn set_refimg(&self, img: u32) {
        self.pyxel_tilemap.lock().image = instance().image(img);
    }

    pub fn set(&mut self, x: i32, y: i32, data: Vec<&str>) {
        self.pyxel_tilemap.lock().set(x, y, &data);
    }

    pub fn clip(
        &self,
        x: Option<f64>,
        y: Option<f64>,
        w: Option<f64>,
        h: Option<f64>,
    ) -> PyResult<()> {
        if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
            self.pyxel_tilemap.lock().clip(x, y, w, h);
        } else if let (None, None, None, None) = (x, y, w, h) {
            self.pyxel_tilemap.lock().clip0();
        } else {
            type_error!("clip() takes 0 or 4 arguments");
        }
        Ok(())
    }

    pub fn cls(&self, tile: Tile) {
        self.pyxel_tilemap.lock().cls(tile);
    }

    pub fn pget(&self, x: f64, y: f64) -> Tile {
        self.pyxel_tilemap.lock().pget(x, y)
    }

    pub fn pset(&self, x: f64, y: f64, tile: Tile) {
        self.pyxel_tilemap.lock().pset(x, y, tile);
    }

    pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64, tile: Tile) {
        self.pyxel_tilemap.lock().line(x1, y1, x2, y2, tile);
    }

    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64, tile: Tile) {
        self.pyxel_tilemap.lock().rect(x, y, w, h, tile);
    }

    pub fn rectb(&self, x: f64, y: f64, w: f64, h: f64, tile: Tile) {
        self.pyxel_tilemap.lock().rectb(x, y, w, h, tile);
    }

    pub fn circ(&self, x: f64, y: f64, r: f64, tile: Tile) {
        self.pyxel_tilemap.lock().circ(x, y, r, tile);
    }

    pub fn circb(&self, x: f64, y: f64, r: f64, tile: Tile) {
        self.pyxel_tilemap.lock().circb(x, y, r, tile);
    }

    pub fn tri(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, tile: Tile) {
        self.pyxel_tilemap.lock().tri(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn trib(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, tile: Tile) {
        self.pyxel_tilemap.lock().trib(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn blt(
        &self,
        x: f64,
        y: f64,
        tm: &PyAny,
        u: f64,
        v: f64,
        w: f64,
        h: f64,
        tilekey: Option<Tile>,
    ) -> PyResult<()> {
        type_switch! {
            tm,
            u32, {
                self.pyxel_tilemap.lock().blt(x, y, instance().tilemap(tm), u, v, w, h, tilekey);
            },
            Tilemap, {
                self.pyxel_tilemap.lock().blt(x, y, tm.pyxel_tilemap, u, v, w, h, tilekey);
            }
        }
        Ok(())
    }
}

pub fn add_tilemap_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Tilemap>()?;
    Ok(())
}
