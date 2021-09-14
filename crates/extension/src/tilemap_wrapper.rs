use pyo3::prelude::*;
use pyxel::SharedTilemap as PyxelSharedTilemap;
use pyxel::Tilemap as PyxelTilemap;
use pyxel::{Canvas, Tile};

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
    pub fn new(width: &PyAny, height: &PyAny, img: &PyAny) -> PyResult<Tilemap> {
        let img = type_switch! {
            img,
            Image,
            {
                img.pyxel_image
            },
            u32,
            {
                instance().image(img)
            }
        };

        Ok(wrap_pyxel_tilemap(PyxelTilemap::new(
            as_u32!(width),
            as_u32!(height),
            img,
        )))
    }

    #[getter]
    pub fn width(&self) -> PyResult<u32> {
        Ok(self.pyxel_tilemap.lock().width())
    }

    #[getter]
    pub fn height(&self) -> PyResult<u32> {
        Ok(self.pyxel_tilemap.lock().height())
    }

    #[getter]
    pub fn image(&self) -> PyResult<Image> {
        Ok(wrap_pyxel_image(self.pyxel_tilemap.lock().image.clone()))
    }

    #[setter]
    pub fn set_image(&self, img: &PyAny) -> PyResult<()> {
        type_switch! {
            img,
            Image, {
                self.pyxel_tilemap.lock().image = img.pyxel_image;
            },
            u32, {
                self.pyxel_tilemap.lock().image = instance().image(img);
            }
        }

        Ok(())
    }

    pub fn set(&mut self, x: &PyAny, y: &PyAny, data: Vec<&str>) -> PyResult<()> {
        self.pyxel_tilemap.lock().set(as_i32!(x), as_i32!(y), &data);

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
            self.pyxel_tilemap
                .lock()
                .clip(as_i32!(x), as_i32!(y), as_u32!(w), as_u32!(h));
        } else if let (None, None, None, None) = (x, y, w, h) {
            self.pyxel_tilemap.lock().clip0();
        } else {
            type_error!("clip() takes 0 or 4 arguments");
        }

        Ok(())
    }

    pub fn cls(&self, tile: Tile) -> PyResult<()> {
        self.pyxel_tilemap.lock().cls(tile);

        Ok(())
    }

    pub fn pget(&self, x: &PyAny, y: &PyAny) -> PyResult<Tile> {
        Ok(self.pyxel_tilemap.lock().pget(as_i32!(x), as_i32!(y)))
    }

    pub fn pset(&self, x: &PyAny, y: &PyAny, tile: Tile) -> PyResult<()> {
        self.pyxel_tilemap.lock().pset(as_i32!(x), as_i32!(y), tile);

        Ok(())
    }

    pub fn line(&self, x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, tile: Tile) -> PyResult<()> {
        self.pyxel_tilemap
            .lock()
            .line(as_i32!(x1), as_i32!(y1), as_i32!(x2), as_i32!(y2), tile);

        Ok(())
    }

    pub fn rect(&self, x: &PyAny, y: &PyAny, w: &PyAny, h: &PyAny, tile: Tile) -> PyResult<()> {
        self.pyxel_tilemap
            .lock()
            .rect(as_i32!(x), as_i32!(y), as_u32!(w), as_u32!(h), tile);

        Ok(())
    }

    pub fn rectb(&self, x: &PyAny, y: &PyAny, w: &PyAny, h: &PyAny, tile: Tile) -> PyResult<()> {
        self.pyxel_tilemap
            .lock()
            .rectb(as_i32!(x), as_i32!(y), as_u32!(w), as_u32!(h), tile);

        Ok(())
    }

    pub fn circ(&self, x: &PyAny, y: &PyAny, r: &PyAny, tile: Tile) -> PyResult<()> {
        self.pyxel_tilemap
            .lock()
            .circ(as_i32!(x), as_i32!(y), as_u32!(r), tile);

        Ok(())
    }

    pub fn circb(&self, x: &PyAny, y: &PyAny, r: &PyAny, tile: Tile) -> PyResult<()> {
        self.pyxel_tilemap
            .lock()
            .circb(as_i32!(x), as_i32!(y), as_u32!(r), tile);

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
        tile: Tile,
    ) -> PyResult<()> {
        self.pyxel_tilemap.lock().tri(
            as_i32!(x1),
            as_i32!(y1),
            as_i32!(x2),
            as_i32!(y2),
            as_i32!(x3),
            as_i32!(y3),
            tile,
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
        tile: Tile,
    ) -> PyResult<()> {
        self.pyxel_tilemap.lock().trib(
            as_i32!(x1),
            as_i32!(y1),
            as_i32!(x2),
            as_i32!(y2),
            as_i32!(x3),
            as_i32!(y3),
            tile,
        );

        Ok(())
    }

    pub fn fill(&self, x: &PyAny, y: &PyAny, tile: Tile) -> PyResult<()> {
        self.pyxel_tilemap.lock().fill(as_i32!(x), as_i32!(y), tile);

        Ok(())
    }

    pub fn blt(
        &self,
        x: &PyAny,
        y: &PyAny,
        tm: &PyAny,
        u: &PyAny,
        v: &PyAny,
        w: &PyAny,
        h: &PyAny,
        tilekey: Option<Tile>,
    ) -> PyResult<()> {
        type_switch! {
            tm,
            u32,
            {
                self.pyxel_tilemap.lock().blt(
                    as_i32!(x),
                    as_i32!(y),
                    instance().tilemap(tm),
                    as_i32!(u),
                    as_i32!(v),
                    as_i32!(w),
                    as_i32!(h),
                    tilekey,
                );
            },
            Tilemap,
            {
                self.pyxel_tilemap.lock().blt(
                    as_i32!(x),
                    as_i32!(y),
                    tm.pyxel_tilemap,
                    as_i32!(u),
                    as_i32!(v),
                    as_i32!(w),
                    as_i32!(h),
                    tilekey,
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
        tilekey: Option<Tile>,
    ) -> PyResult<()> {
        self.pyxel_tilemap.lock().blt_self(
            as_i32!(x),
            as_i32!(y),
            as_i32!(u),
            as_i32!(v),
            as_i32!(w),
            as_i32!(h),
            tilekey,
        );

        Ok(())
    }
}

pub fn add_tilemap_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Tilemap>()?;

    Ok(())
}
