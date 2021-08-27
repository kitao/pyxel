use pyo3::prelude::*;

use pyxel::Image as PyxelImage;
use pyxel::SharedImage as PyxelSharedImage;
use pyxel::{Canvas, Color};

use crate::instance;
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
    pub fn new(width: &PyAny, height: &PyAny) -> Image {
        wrap_pyxel_image(PyxelImage::new(as_u32!(width), as_u32!(height)))
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.pyxel_image.lock().width()
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.pyxel_image.lock().height()
    }

    pub fn set(&self, x: &PyAny, y: &PyAny, data_str: Vec<&str>) {
        self.pyxel_image
            .lock()
            .set(as_i32!(x), as_i32!(y), &data_str);
    }

    pub fn load(&self, x: &PyAny, y: &PyAny, filename: &str) {
        self.pyxel_image
            .lock()
            .load(as_i32!(x), as_i32!(y), filename, &instance().colors);
    }

    pub fn save(&self, filename: &str, scale: &PyAny) {
        self.pyxel_image
            .lock()
            .save(filename, &instance().colors, as_u32!(scale));
    }

    pub fn clip(
        &self,
        x: Option<&PyAny>,
        y: Option<&PyAny>,
        width: Option<&PyAny>,
        height: Option<&PyAny>,
    ) {
        if let Some(x) = x {
            if let Some(y) = y {
                if let Some(width) = width {
                    if let Some(height) = height {
                        self.pyxel_image.lock().clip(
                            as_i32!(x),
                            as_i32!(y),
                            as_u32!(width),
                            as_u32!(height),
                        );
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

    pub fn pget(&self, x: &PyAny, y: &PyAny) -> Color {
        self.pyxel_image.lock().pget(as_i32!(x), as_i32!(y))
    }

    pub fn pset(&self, x: &PyAny, y: &PyAny, color: Color) {
        self.pyxel_image.lock().pset(as_i32!(x), as_i32!(y), color);
    }

    pub fn line(&self, x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, color: Color) {
        self.pyxel_image
            .lock()
            .line(as_i32!(x1), as_i32!(y1), as_i32!(x2), as_i32!(y2), color);
    }

    pub fn rect(&self, x: &PyAny, y: &PyAny, width: &PyAny, height: &PyAny, color: Color) {
        self.pyxel_image.lock().rect(
            as_i32!(x),
            as_i32!(y),
            as_u32!(width),
            as_u32!(height),
            color,
        );
    }

    pub fn rectb(&self, x: &PyAny, y: &PyAny, width: &PyAny, height: &PyAny, color: Color) {
        self.pyxel_image.lock().rectb(
            as_i32!(x),
            as_i32!(y),
            as_u32!(width),
            as_u32!(height),
            color,
        );
    }

    pub fn circ(&self, x: &PyAny, y: &PyAny, radius: &PyAny, color: Color) {
        self.pyxel_image
            .lock()
            .circ(as_i32!(x), as_i32!(y), as_u32!(radius), color);
    }

    pub fn circb(&self, x: &PyAny, y: &PyAny, radius: &PyAny, color: Color) {
        self.pyxel_image
            .lock()
            .circb(as_i32!(x), as_i32!(y), as_u32!(radius), color);
    }

    pub fn tri(
        &self,
        x1: &PyAny,
        y1: &PyAny,
        x2: &PyAny,
        y2: &PyAny,
        x3: &PyAny,
        y3: &PyAny,
        color: Color,
    ) {
        self.pyxel_image.lock().tri(
            as_i32!(x1),
            as_i32!(y1),
            as_i32!(x2),
            as_i32!(y2),
            as_i32!(x3),
            as_i32!(y3),
            color,
        );
    }

    pub fn trib(
        &self,
        x1: &PyAny,
        y1: &PyAny,
        x2: &PyAny,
        y2: &PyAny,
        x3: &PyAny,
        y3: &PyAny,
        color: Color,
    ) {
        self.pyxel_image.lock().trib(
            as_i32!(x1),
            as_i32!(y1),
            as_i32!(x2),
            as_i32!(y2),
            as_i32!(x3),
            as_i32!(y3),
            color,
        );
    }

    pub fn fill(&self, x: &PyAny, y: &PyAny, color: Color) {
        self.pyxel_image.lock().fill(as_i32!(x), as_i32!(y), color);
    }

    pub fn blt(
        &self,
        x: &PyAny,
        y: &PyAny,
        image: Image,
        image_x: &PyAny,
        image_y: &PyAny,
        width: &PyAny,
        height: &PyAny,
        transparent: Option<Color>,
    ) {
        self.pyxel_image.lock().blt(
            as_i32!(x),
            as_i32!(y),
            &image.pyxel_image.lock(),
            as_i32!(image_x),
            as_i32!(image_y),
            as_i32!(width),
            as_i32!(height),
            transparent,
        );
    }

    pub fn bltm(
        &self,
        x: &PyAny,
        y: &PyAny,
        tilemap: Tilemap,
        tilemap_x: &PyAny,
        tilemap_y: &PyAny,
        width: &PyAny,
        height: &PyAny,
        transparent: Option<Color>,
    ) {
        self.pyxel_image.lock().bltm(
            as_i32!(x),
            as_i32!(y),
            &tilemap.pyxel_tilemap.lock(),
            as_i32!(tilemap_x),
            as_i32!(tilemap_y),
            as_i32!(width),
            as_i32!(height),
            transparent,
        );
    }

    pub fn text(&self, x: &PyAny, y: &PyAny, string: &str, color: Color, font: Image) {
        self.pyxel_image.lock().text(
            as_i32!(x),
            as_i32!(y),
            string,
            color,
            &font.pyxel_image.lock(),
        );
    }
}

pub fn add_image_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Image>()?;

    Ok(())
}
