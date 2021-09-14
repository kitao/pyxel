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
    Image { pyxel_image }
}

#[pymethods]
impl Image {
    #[new]
    pub fn new(width: &PyAny, height: &PyAny) -> PyResult<Image> {
        Ok(wrap_pyxel_image(PyxelImage::new(
            as_u32!(width),
            as_u32!(height),
        )))
    }

    #[staticmethod]
    pub fn from_image(filename: &str) -> PyResult<Image> {
        Ok(wrap_pyxel_image(PyxelImage::from_image(filename)))
    }

    #[getter]
    pub fn width(&self) -> PyResult<u32> {
        Ok(self.pyxel_image.lock().width())
    }

    #[getter]
    pub fn height(&self) -> PyResult<u32> {
        Ok(self.pyxel_image.lock().height())
    }

    pub fn set(&self, x: &PyAny, y: &PyAny, data: Vec<&str>) -> PyResult<()> {
        self.pyxel_image.lock().set(as_i32!(x), as_i32!(y), &data);

        Ok(())
    }

    pub fn load(&self, x: &PyAny, y: &PyAny, filename: &str) -> PyResult<()> {
        self.pyxel_image
            .lock()
            .load(as_i32!(x), as_i32!(y), filename, &instance().colors);

        Ok(())
    }

    pub fn save(&self, filename: &str, scale: &PyAny) -> PyResult<()> {
        self.pyxel_image
            .lock()
            .save(filename, &instance().colors, as_u32!(scale));

        Ok(())
    }

    pub fn clip(
        &self,
        x: Option<&PyAny>,
        y: Option<&PyAny>,
        w: Option<&PyAny>,
        h: Option<&PyAny>,
    ) -> PyResult<()> {
        if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
            self.pyxel_image
                .lock()
                .clip(as_i32!(x), as_i32!(y), as_u32!(w), as_u32!(h));
        } else if let (None, None, None, None) = (x, y, w, h) {
            self.pyxel_image.lock().clip0();
        } else {
            type_error!("clip() takes 0 or 4 arguments");
        }

        Ok(())
    }

    pub fn cls(&self, col: Color) -> PyResult<()> {
        self.pyxel_image.lock().cls(col);

        Ok(())
    }

    pub fn pget(&self, x: &PyAny, y: &PyAny) -> PyResult<Color> {
        Ok(self.pyxel_image.lock().pget(as_i32!(x), as_i32!(y)))
    }

    pub fn pset(&self, x: &PyAny, y: &PyAny, col: Color) -> PyResult<()> {
        self.pyxel_image.lock().pset(as_i32!(x), as_i32!(y), col);

        Ok(())
    }

    pub fn line(&self, x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, col: Color) -> PyResult<()> {
        self.pyxel_image
            .lock()
            .line(as_i32!(x1), as_i32!(y1), as_i32!(x2), as_i32!(y2), col);

        Ok(())
    }

    pub fn rect(&self, x: &PyAny, y: &PyAny, w: &PyAny, h: &PyAny, col: Color) -> PyResult<()> {
        self.pyxel_image
            .lock()
            .rect(as_i32!(x), as_i32!(y), as_u32!(w), as_u32!(h), col);

        Ok(())
    }

    pub fn rectb(&self, x: &PyAny, y: &PyAny, w: &PyAny, h: &PyAny, col: Color) -> PyResult<()> {
        self.pyxel_image
            .lock()
            .rectb(as_i32!(x), as_i32!(y), as_u32!(w), as_u32!(h), col);

        Ok(())
    }

    pub fn circ(&self, x: &PyAny, y: &PyAny, r: &PyAny, col: Color) -> PyResult<()> {
        self.pyxel_image
            .lock()
            .circ(as_i32!(x), as_i32!(y), as_u32!(r), col);

        Ok(())
    }

    pub fn circb(&self, x: &PyAny, y: &PyAny, r: &PyAny, col: Color) -> PyResult<()> {
        self.pyxel_image
            .lock()
            .circb(as_i32!(x), as_i32!(y), as_u32!(r), col);

        Ok(())
    }

    pub fn tri(
        &self,
        x1: &PyAny,
        y1: &PyAny,
        x2: &PyAny,
        y2: &PyAny,
        x3: &PyAny,
        y3: &PyAny,
        col: Color,
    ) -> PyResult<()> {
        self.pyxel_image.lock().tri(
            as_i32!(x1),
            as_i32!(y1),
            as_i32!(x2),
            as_i32!(y2),
            as_i32!(x3),
            as_i32!(y3),
            col,
        );

        Ok(())
    }

    pub fn trib(
        &self,
        x1: &PyAny,
        y1: &PyAny,
        x2: &PyAny,
        y2: &PyAny,
        x3: &PyAny,
        y3: &PyAny,
        col: Color,
    ) -> PyResult<()> {
        self.pyxel_image.lock().trib(
            as_i32!(x1),
            as_i32!(y1),
            as_i32!(x2),
            as_i32!(y2),
            as_i32!(x3),
            as_i32!(y3),
            col,
        );

        Ok(())
    }

    pub fn fill(&self, x: &PyAny, y: &PyAny, col: Color) -> PyResult<()> {
        self.pyxel_image.lock().fill(as_i32!(x), as_i32!(y), col);

        Ok(())
    }

    pub fn blt(
        &self,
        x: &PyAny,
        y: &PyAny,
        img: &PyAny,
        u: &PyAny,
        v: &PyAny,
        w: &PyAny,
        h: &PyAny,
        colkey: Option<Color>,
    ) -> PyResult<()> {
        type_switch! {
            img,
            u32,
            {
                self.pyxel_image.lock().blt(
                    as_i32!(x),
                    as_i32!(y),
                    instance().image(img),
                    as_i32!(u),
                    as_i32!(v),
                    as_i32!(w),
                    as_i32!(h),
                    colkey,
                );
            },
            Image,
            {
                self.pyxel_image.lock().blt(
                    as_i32!(x),
                    as_i32!(y),
                    img.pyxel_image,
                    as_i32!(u),
                    as_i32!(v),
                    as_i32!(w),
                    as_i32!(h),
                    colkey,
                );
            }
        }

        Ok(())
    }

    pub fn blt_self(
        &self,
        x: &PyAny,
        y: &PyAny,
        u: &PyAny,
        v: &PyAny,
        w: &PyAny,
        h: &PyAny,
        colkey: Option<Color>,
    ) -> PyResult<()> {
        self.pyxel_image.lock().blt_self(
            as_i32!(x),
            as_i32!(y),
            as_i32!(u),
            as_i32!(v),
            as_i32!(w),
            as_i32!(h),
            colkey,
        );

        Ok(())
    }

    pub fn bltm(
        &self,
        x: &PyAny,
        y: &PyAny,
        tm: &PyAny,
        u: &PyAny,
        v: &PyAny,
        w: &PyAny,
        h: &PyAny,
        colkey: Option<Color>,
    ) -> PyResult<()> {
        type_switch! {
            tm,
            u32,
            {
                self.pyxel_image.lock().bltm(
                    as_i32!(x),
                    as_i32!(y),
                    instance().tilemap(tm),
                    as_i32!(u),
                    as_i32!(v),
                    as_i32!(w),
                    as_i32!(h),
                    colkey,
                );
            },
            Tilemap,
            {
                self.pyxel_image.lock().bltm(
                    as_i32!(x),
                    as_i32!(y),
                    tm.pyxel_tilemap,
                    as_i32!(u),
                    as_i32!(v),
                    as_i32!(w),
                    as_i32!(h),
                    colkey,
                );
            }
        }

        Ok(())
    }

    pub fn text(
        &self,
        x: &PyAny,
        y: &PyAny,
        s: &str,
        col: Color,
        font: Option<Image>,
    ) -> PyResult<()> {
        if let Some(font) = font {
            self.pyxel_image
                .lock()
                .text(as_i32!(x), as_i32!(y), s, col, font.pyxel_image);
        } else {
            self.pyxel_image
                .lock()
                .text(as_i32!(x), as_i32!(y), s, col, instance().font.clone());
        }

        Ok(())
    }
}

pub fn add_image_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Image>()?;

    Ok(())
}
