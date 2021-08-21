use pyo3::prelude::*;
use pyo3::types::PyTuple;

use pyxel::Image as PyxelImage;
use pyxel::SharedImage as PyxelSharedImage;
use pyxel::{Canvas, Color, Rgb8};

use crate::tilemap_wrapper::Tilemap;

#[pyclass]
#[derive(Clone)]
pub struct Image {
    pub pyxel_image: PyxelSharedImage,
}

pub fn wrap_pyxel_image(pyxel_image: PyxelSharedImage) -> Image {
    Image {
        pyxel_image: pyxel_image,
    }
}

#[pymethods]
impl Image {
    #[new]
    pub fn new(width: u32, height: u32) -> Image {
        wrap_pyxel_image(PyxelImage::new(width, height))
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.pyxel_image.lock().width()
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.pyxel_image.lock().height()
    }

    pub fn set(&self, x: i32, y: i32, data_str: Vec<&str>) {
        self.pyxel_image.lock().set(x, y, &data_str);
    }

    pub fn load(&self, x: i32, y: i32, filename: &str, colors: Vec<Rgb8>) {
        self.pyxel_image.lock().load(x, y, filename, &colors);
    }

    pub fn save(&self, filename: &str, colors: Vec<Rgb8>, scale: u32) {
        self.pyxel_image.lock().save(filename, &colors, scale);
    }

    pub fn clip(&self, x: Option<i32>, y: Option<i32>, width: Option<u32>, height: Option<u32>) {
        if let Some(x) = x {
            if let Some(y) = y {
                if let Some(width) = width {
                    if let Some(height) = height {
                        self.pyxel_image.lock().clip(x, y, width, height);
                        return;
                    }
                }
            }
        }

        if let None = x {
            if let None = y {
                if let None = width {
                    if let None = height {
                        self.pyxel_image.lock().clip0();
                        return;
                    }
                }
            }
        }

        panic!("invalid argument number for clip");
    }

    pub fn cls(&self, color: Color) {
        self.pyxel_image.lock().cls(color);
    }

    pub fn pget(&self, x: i32, y: i32) -> Color {
        self.pyxel_image.lock().pget(x, y)
    }

    pub fn pset(&self, x: i32, y: i32, color: Color) {
        self.pyxel_image.lock().pset(x, y, color);
    }

    pub fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        self.pyxel_image.lock().line(x1, y1, x2, y2, color);
    }

    pub fn rect(&self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.pyxel_image.lock().rect(x, y, width, height, color);
    }

    pub fn rectb(&self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.pyxel_image.lock().rectb(x, y, width, height, color);
    }

    pub fn circ(&self, x: i32, y: i32, radius: u32, color: Color) {
        self.pyxel_image.lock().circ(x, y, radius, color);
    }

    pub fn circb(&self, x: i32, y: i32, radius: u32, color: Color) {
        self.pyxel_image.lock().circb(x, y, radius, color);
    }

    pub fn tri(&self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        self.pyxel_image.lock().tri(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn trib(&self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        self.pyxel_image.lock().trib(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn fill(&self, x: i32, y: i32, color: Color) {
        self.pyxel_image.lock().fill(x, y, color);
    }

    pub fn bltm(
        &self,
        x: i32,
        y: i32,
        tilemap: Tilemap,
        tilemap_x: i32,
        tilemap_y: i32,
        width: i32,
        height: i32,
        transparent: Option<&PyTuple>,
    ) {
        let transparent = if let Some(transparent) = transparent {
            Some((
                transparent.get_item(0).extract().unwrap(),
                transparent.get_item(1).extract().unwrap(),
            ))
        } else {
            None
        };

        self.pyxel_image.lock().bltm(
            x,
            y,
            tilemap.pyxel_tilemap.clone(),
            tilemap_x,
            tilemap_y,
            width,
            height,
            transparent,
        );
    }

    pub fn text(&self, x: i32, y: i32, string: &str, color: Color, font: Image) {
        self.pyxel_image
            .lock()
            .text(x, y, string, color, font.pyxel_image.clone());
    }
}

pub fn add_image_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Image>()?;

    Ok(())
}
