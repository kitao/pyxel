use pyo3::prelude::*;
use pyxel::{Color, Image as PyxelImage, SharedImage as PyxelSharedImage};

use crate::tilemap_wrapper::Tilemap;

#[pyclass]
#[derive(Clone)]
pub struct Image {
    pub pyxel_image: PyxelSharedImage,
}

pub const fn wrap_pyxel_image(pyxel_image: PyxelSharedImage) -> Image {
    Image { pyxel_image }
}

#[pymethods]
impl Image {
    #[new]
    pub fn new(width: u32, height: u32) -> Self {
        wrap_pyxel_image(PyxelImage::new(width, height))
    }

    #[staticmethod]
    pub fn from_image(filename: &str) -> Self {
        wrap_pyxel_image(PyxelImage::from_image(filename))
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.pyxel_image.lock().width()
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.pyxel_image.lock().height()
    }

    pub fn set(&self, x: i32, y: i32, data: Vec<&str>) {
        self.pyxel_image.lock().set(x, y, &data);
    }

    pub fn load(&self, x: i32, y: i32, filename: &str) {
        self.pyxel_image.lock().load(x, y, filename);
    }

    pub fn save(&self, filename: &str, scale: u32) {
        self.pyxel_image.lock().save(filename, scale);
    }

    pub fn clip(
        &self,
        x: Option<f64>,
        y: Option<f64>,
        w: Option<f64>,
        h: Option<f64>,
    ) -> PyResult<()> {
        if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
            self.pyxel_image.lock().clip(x, y, w, h);
        } else if (x, y, w, h) == (None, None, None, None) {
            self.pyxel_image.lock().clip0();
        } else {
            type_error!("clip() takes 0 or 4 arguments");
        }
        Ok(())
    }

    pub fn camera(&self, x: Option<f64>, y: Option<f64>) -> PyResult<()> {
        if let (Some(x), Some(y)) = (x, y) {
            self.pyxel_image.lock().camera(x, y);
        } else if (x, y) == (None, None) {
            self.pyxel_image.lock().camera0();
        } else {
            type_error!("camera() takes 0 or 2 arguments");
        }
        Ok(())
    }

    pub fn cls(&self, col: Color) {
        self.pyxel_image.lock().cls(col);
    }

    pub fn pget(&self, x: f64, y: f64) -> Color {
        self.pyxel_image.lock().pget(x, y)
    }

    pub fn pset(&self, x: f64, y: f64, col: Color) {
        self.pyxel_image.lock().pset(x, y, col);
    }

    pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64, col: Color) {
        self.pyxel_image.lock().line(x1, y1, x2, y2, col);
    }

    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64, col: Color) {
        self.pyxel_image.lock().rect(x, y, w, h, col);
    }

    pub fn rectb(&self, x: f64, y: f64, w: f64, h: f64, col: Color) {
        self.pyxel_image.lock().rectb(x, y, w, h, col);
    }

    pub fn circ(&self, x: f64, y: f64, r: f64, col: Color) {
        self.pyxel_image.lock().circ(x, y, r, col);
    }

    pub fn circb(&self, x: f64, y: f64, r: f64, col: Color) {
        self.pyxel_image.lock().circb(x, y, r, col);
    }

    pub fn elli(&self, x: f64, y: f64, w: f64, h: f64, col: Color) {
        self.pyxel_image.lock().elli(x, y, w, h, col);
    }

    pub fn ellib(&self, x: f64, y: f64, w: f64, h: f64, col: Color) {
        self.pyxel_image.lock().ellib(x, y, w, h, col);
    }

    pub fn tri(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: Color) {
        self.pyxel_image.lock().tri(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn trib(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: Color) {
        self.pyxel_image.lock().trib(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn fill(&self, x: f64, y: f64, col: Color) {
        self.pyxel_image.lock().fill(x, y, col);
    }

    pub fn blt(
        &self,
        x: f64,
        y: f64,
        img: &PyAny,
        u: f64,
        v: f64,
        w: f64,
        h: f64,
        colkey: Option<Color>,
    ) -> PyResult<()> {
        type_switch! {
            img,
            u32, {
                self.pyxel_image.lock().blt(x, y, pyxel::image(img), u, v, w, h, colkey);
            },
            Image, {
                self.pyxel_image.lock().blt(x, y, img.pyxel_image, u, v, w, h, colkey);
            }
        }
        Ok(())
    }

    pub fn bltm(
        &self,
        x: f64,
        y: f64,
        tm: &PyAny,
        u: f64,
        v: f64,
        w: f64,
        h: f64,
        colkey: Option<Color>,
    ) -> PyResult<()> {
        type_switch! {
            tm,
            u32, {
                self.pyxel_image.lock().bltm(x, y, pyxel::tilemap(tm), u, v, w, h, colkey);
            },
            Tilemap, {
                self.pyxel_image.lock().bltm(x, y, tm.pyxel_tilemap, u, v, w, h, colkey);
            }
        }
        Ok(())
    }

    pub fn text(&self, x: f64, y: f64, s: &str, col: Color) {
        self.pyxel_image.lock().text(x, y, s, col);
    }
}

pub fn add_image_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Image>()?;
    Ok(())
}
