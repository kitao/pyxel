use std::sync::Once;

use pyo3::prelude::*;

use crate::image_wrapper::Image;
use crate::pyxel_singleton::pyxel;

static IMAGE_ONCE: Once = Once::new();
static SET_IMAGE_ONCE: Once = Once::new();
static REFIMG_ONCE: Once = Once::new();
static SET_REFIMG_ONCE: Once = Once::new();

#[pyclass]
#[derive(Clone)]
pub struct Tilemap {
    pub(crate) inner: pyxel::SharedTilemap,
}

impl Tilemap {
    pub fn wrap(inner: pyxel::SharedTilemap) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Tilemap {
    #[new]
    pub fn new(width: u32, height: u32, img: Bound<'_, PyAny>) -> PyResult<Self> {
        let imgsrc = cast_pyany! {
            img,
            (u32, { pyxel::ImageSource::Index(img) }),
            (Image, { pyxel::ImageSource::Image(img.inner) })
        };
        Ok(Tilemap::wrap(pyxel::Tilemap::new(width, height, imgsrc)))
    }

    #[staticmethod]
    pub fn from_tmx(filename: &str, layer: u32) -> Self {
        Self::wrap(pyxel::Tilemap::from_tmx(filename, layer))
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.inner.lock().width()
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.inner.lock().height()
    }

    #[getter]
    pub fn imgsrc(&self, py: Python) -> PyObject {
        let tilemap = self.inner.lock();
        match &tilemap.imgsrc {
            pyxel::ImageSource::Index(index) => index.into_py(py),
            pyxel::ImageSource::Image(image) => Image::wrap(image.clone()).into_py(py),
        }
    }

    #[setter]
    pub fn set_imgsrc(&self, img: Bound<'_, PyAny>) -> PyResult<()> {
        let imgsrc = cast_pyany! {
            img,
            (u32, { pyxel::ImageSource::Index(img) }),
            (Image, { pyxel::ImageSource::Image(img.inner) })
        };
        self.inner.lock().imgsrc = imgsrc;
        Ok(())
    }

    pub fn data_ptr(&self, py: Python) -> PyObject {
        let mut inner = self.inner.lock();
        let python_code = format!(
            "import ctypes; c_uint8_array = (ctypes.c_uint8 * {}).from_address({:p})",
            inner.width() * inner.height(),
            inner.data_ptr()
        );
        let locals = pyo3::types::PyDict::new_bound(py);
        py.run_bound(&python_code, None, Some(&locals)).unwrap();
        locals.get_item("c_uint8_array").unwrap().to_object(py)
    }

    pub fn set(&mut self, x: i32, y: i32, data: Vec<String>) {
        let data_refs: Vec<_> = data.iter().map(String::as_str).collect();
        self.inner.lock().set(x, y, &data_refs);
    }

    pub fn load(&self, x: i32, y: i32, filename: &str, layer: u32) {
        self.inner.lock().load(x, y, filename, layer);
    }

    #[pyo3(signature = (x=None, y=None, w=None, h=None))]
    pub fn clip(
        &self,
        x: Option<f64>,
        y: Option<f64>,
        w: Option<f64>,
        h: Option<f64>,
    ) -> PyResult<()> {
        if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
            self.inner.lock().clip(x, y, w, h);
        } else if (x, y, w, h) == (None, None, None, None) {
            self.inner.lock().clip0();
        } else {
            python_type_error!("clip() takes 0 or 4 arguments");
        }
        Ok(())
    }

    #[pyo3(signature = (x=None, y=None))]
    pub fn camera(&self, x: Option<f64>, y: Option<f64>) -> PyResult<()> {
        if let (Some(x), Some(y)) = (x, y) {
            self.inner.lock().camera(x, y);
        } else if (x, y) == (None, None) {
            self.inner.lock().camera0();
        } else {
            python_type_error!("camera() takes 0 or 2 arguments");
        }
        Ok(())
    }

    pub fn cls(&self, tile: pyxel::Tile) {
        self.inner.lock().cls(tile);
    }

    pub fn pget(&self, x: f64, y: f64) -> pyxel::Tile {
        self.inner.lock().pget(x, y)
    }

    pub fn pset(&self, x: f64, y: f64, tile: pyxel::Tile) {
        self.inner.lock().pset(x, y, tile);
    }

    pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64, tile: pyxel::Tile) {
        self.inner.lock().line(x1, y1, x2, y2, tile);
    }

    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64, tile: pyxel::Tile) {
        self.inner.lock().rect(x, y, w, h, tile);
    }

    pub fn rectb(&self, x: f64, y: f64, w: f64, h: f64, tile: pyxel::Tile) {
        self.inner.lock().rectb(x, y, w, h, tile);
    }

    pub fn circ(&self, x: f64, y: f64, r: f64, tile: pyxel::Tile) {
        self.inner.lock().circ(x, y, r, tile);
    }

    pub fn circb(&self, x: f64, y: f64, r: f64, tile: pyxel::Tile) {
        self.inner.lock().circb(x, y, r, tile);
    }

    pub fn elli(&self, x: f64, y: f64, w: f64, h: f64, tile: pyxel::Tile) {
        self.inner.lock().elli(x, y, w, h, tile);
    }

    pub fn ellib(&self, x: f64, y: f64, w: f64, h: f64, tile: pyxel::Tile) {
        self.inner.lock().ellib(x, y, w, h, tile);
    }

    pub fn tri(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, tile: pyxel::Tile) {
        self.inner.lock().tri(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn trib(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, tile: pyxel::Tile) {
        self.inner.lock().trib(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn fill(&self, x: f64, y: f64, tile: pyxel::Tile) {
        self.inner.lock().fill(x, y, tile);
    }

    #[pyo3(signature = (x, y, tm, u, v, w, h, tilekey=None, rotate=None, scale=None))]
    pub fn blt(
        &self,
        x: f64,
        y: f64,
        tm: Bound<'_, PyAny>,
        u: f64,
        v: f64,
        w: f64,
        h: f64,
        tilekey: Option<pyxel::Tile>,
        rotate: Option<f64>,
        scale: Option<f64>,
    ) -> PyResult<()> {
        cast_pyany! {
            tm,
            (u32, {
                let tilemap = pyxel().tilemaps.lock()[tm as usize].clone();
                self.inner.lock().blt(x, y, tilemap, u, v, w, h, tilekey, rotate, scale);
            }),
            (Tilemap, {
                self.inner.lock().blt(x, y, tm.inner, u, v, w, h, tilekey, rotate, scale);
            })
        }
        Ok(())
    }

    #[getter]
    pub fn image(&self) -> Image {
        IMAGE_ONCE.call_once(|| {
            println!("Tilemap.image is deprecated, use Tilemap.imgsrc instead.");
        });
        let tilemap = self.inner.lock();
        match &tilemap.imgsrc {
            pyxel::ImageSource::Index(index) => {
                let images = pyxel().images.lock();
                Image::wrap(images[*index as usize].clone())
            }
            pyxel::ImageSource::Image(image) => Image::wrap(image.clone()),
        }
    }

    #[setter]
    pub fn set_image(&self, image: Image) {
        SET_IMAGE_ONCE.call_once(|| {
            println!("Tilemap.image is deprecated, use Tilemap.imgsrc instead.");
        });
        self.inner.lock().imgsrc = pyxel::ImageSource::Image(image.inner);
    }

    #[getter]
    pub fn refimg(&self) -> Option<u32> {
        REFIMG_ONCE.call_once(|| {
            println!("Tilemap.refimg is deprecated, use Tilemap.imgsrc instead.");
        });
        let tilemap = self.inner.lock();
        match &tilemap.imgsrc {
            pyxel::ImageSource::Index(index) => Some(*index),
            pyxel::ImageSource::Image(_image) => None,
        }
    }

    #[setter]
    pub fn set_refimg(&self, img: u32) {
        SET_REFIMG_ONCE.call_once(|| {
            println!("Tilemap.refimg is deprecated, use Tilemap.imgsrc instead.");
        });
        self.inner.lock().imgsrc = pyxel::ImageSource::Index(img);
    }
}

pub fn add_tilemap_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tilemap>()?;
    Ok(())
}
