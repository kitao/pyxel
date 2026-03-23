use std::ffi::CString;
use std::sync::Once;

use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::image_wrapper::Image;

static IMAGE_ONCE: Once = Once::new();
static SET_IMAGE_ONCE: Once = Once::new();
static REFIMG_ONCE: Once = Once::new();
static SET_REFIMG_ONCE: Once = Once::new();

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub struct Tilemap {
    pub(crate) inner: *mut pyxel::Tilemap,
}

unsafe impl Send for Tilemap {}
unsafe impl Sync for Tilemap {}

impl Tilemap {
    pub fn wrap(inner: *mut pyxel::Tilemap) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Tilemap {
    #[new]
    fn new(width: u32, height: u32, img: Bound<'_, PyAny>) -> PyResult<Self> {
        let imgsrc = cast_pyany! {
            img,
            (u32, { pyxel::ImageSource::Index(img) }),
            (Image, { pyxel::ImageSource::Image(img.inner) })
        };
        Ok(Tilemap::wrap(pyxel::Tilemap::new(width, height, imgsrc)))
    }

    #[staticmethod]
    fn from_tmx(filename: &str, layer: u32) -> PyResult<Self> {
        pyxel::Tilemap::from_tmx(filename, layer)
            .map(Tilemap::wrap)
            .map_err(PyException::new_err)
    }

    #[getter]
    fn width(&self) -> u32 {
        unsafe { &*self.inner }.width()
    }

    #[getter]
    fn height(&self) -> u32 {
        unsafe { &*self.inner }.height()
    }

    #[getter]
    fn imgsrc(&self, py: Python) -> Py<PyAny> {
        let tilemap = unsafe { &*self.inner };
        match &tilemap.imgsrc {
            pyxel::ImageSource::Index(index) => value_to_pyobj!(py, index),
            pyxel::ImageSource::Image(image) => class_to_pyobj!(py, Image::wrap(*image)),
        }
    }

    #[setter]
    fn set_imgsrc(&self, img: Bound<'_, PyAny>) -> PyResult<()> {
        let imgsrc = cast_pyany! {
            img,
            (u32, { pyxel::ImageSource::Index(img) }),
            (Image, { pyxel::ImageSource::Image(img.inner) })
        };
        unsafe { &mut *self.inner }.imgsrc = imgsrc;
        Ok(())
    }

    fn data_ptr(&self, py: Python) -> PyResult<Py<PyAny>> {
        let inner = unsafe { &mut *self.inner };
        let python_code = CString::new(format!(
            "import ctypes; c_uint16_array = (ctypes.c_uint16 * {}).from_address({:p})",
            inner.width() * inner.height() * 2,
            inner.data_ptr()
        ))
        .unwrap();
        let locals = PyDict::new(py);
        py.run(python_code.as_c_str(), None, Some(&locals))?;
        let array = locals
            .get_item("c_uint16_array")?
            .ok_or_else(|| PyException::new_err("Failed to create data pointer"))?;
        Ok(array.unbind())
    }

    fn set(&self, x: i32, y: i32, data: Vec<String>) {
        let data_refs: Vec<_> = data.iter().map(String::as_str).collect();
        unsafe { &mut *self.inner }.set(x, y, &data_refs);
    }

    fn load(&self, x: i32, y: i32, filename: &str, layer: u32) -> PyResult<()> {
        unsafe { &mut *self.inner }
            .load(x, y, filename, layer)
            .map_err(PyException::new_err)
    }

    #[pyo3(signature = (x=None, y=None, w=None, h=None))]
    fn clip(&self, x: Option<f32>, y: Option<f32>, w: Option<f32>, h: Option<f32>) -> PyResult<()> {
        if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
            unsafe { &mut *self.inner }.set_clip_rect(x, y, w, h);
        } else if (x, y, w, h) == (None, None, None, None) {
            unsafe { &mut *self.inner }.reset_clip_rect();
        } else {
            python_type_error!("clip() takes 0 or 4 arguments");
        }
        Ok(())
    }

    #[pyo3(signature = (x=None, y=None))]
    fn camera(&self, x: Option<f32>, y: Option<f32>) -> PyResult<()> {
        if let (Some(x), Some(y)) = (x, y) {
            unsafe { &mut *self.inner }.set_draw_offset(x, y);
        } else if (x, y) == (None, None) {
            unsafe { &mut *self.inner }.reset_draw_offset();
        } else {
            python_type_error!("camera() takes 0 or 2 arguments");
        }
        Ok(())
    }

    fn cls(&self, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.clear(tile);
    }

    fn pget(&self, x: f32, y: f32) -> pyxel::Tile {
        unsafe { &mut *self.inner }.get_tile(x, y)
    }

    fn pset(&self, x: f32, y: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.set_tile(x, y, tile);
    }

    fn line(&self, x1: f32, y1: f32, x2: f32, y2: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.draw_line(x1, y1, x2, y2, tile);
    }

    fn rect(&self, x: f32, y: f32, w: f32, h: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.draw_rect(x, y, w, h, tile);
    }

    fn rectb(&self, x: f32, y: f32, w: f32, h: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.draw_rect_border(x, y, w, h, tile);
    }

    fn circ(&self, x: f32, y: f32, r: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.draw_circle(x, y, r, tile);
    }

    fn circb(&self, x: f32, y: f32, r: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.draw_circle_border(x, y, r, tile);
    }

    fn elli(&self, x: f32, y: f32, w: f32, h: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.draw_ellipse(x, y, w, h, tile);
    }

    fn ellib(&self, x: f32, y: f32, w: f32, h: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.draw_ellipse_border(x, y, w, h, tile);
    }

    fn tri(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.draw_triangle(x1, y1, x2, y2, x3, y3, tile);
    }

    fn trib(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.draw_triangle_border(x1, y1, x2, y2, x3, y3, tile);
    }

    fn fill(&self, x: f32, y: f32, tile: pyxel::Tile) {
        unsafe { &mut *self.inner }.flood_fill(x, y, tile);
    }

    fn collide(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        dx: f32,
        dy: f32,
        walls: Vec<pyxel::Tile>,
    ) -> (f32, f32) {
        unsafe { &mut *self.inner }.collide(x, y, w, h, dx, dy, &walls)
    }

    #[pyo3(signature = (x, y, tm, u, v, w, h, tilekey=None, rotate=None, scale=None))]
    fn blt(
        &self,
        x: f32,
        y: f32,
        tm: Bound<'_, PyAny>,
        u: f32,
        v: f32,
        w: f32,
        h: f32,
        tilekey: Option<pyxel::Tile>,
        rotate: Option<f32>,
        scale: Option<f32>,
    ) -> PyResult<()> {
        cast_pyany! {
            tm,
            (u32, {
                let tilemap = pyxel::tilemaps().get(tm as usize).copied()
                    .ok_or_else(|| PyValueError::new_err("Invalid tilemap index"))?;
                unsafe { (&mut *self.inner).draw_tilemap(x, y, tilemap, u, v, w, h, tilekey, rotate, scale) };
            }),
            (Tilemap, {
                unsafe { (&mut *self.inner).draw_tilemap(x, y, tm.inner, u, v, w, h, tilekey, rotate, scale) };
            })
        }
        Ok(())
    }

    #[getter]
    fn image(&self) -> PyResult<Image> {
        IMAGE_ONCE.call_once(|| {
            println!("Tilemap.image is deprecated. Use Tilemap.imgsrc instead.");
        });

        let tilemap = unsafe { &*self.inner };
        match &tilemap.imgsrc {
            pyxel::ImageSource::Index(index) => pyxel::images()
                .get(*index as usize)
                .copied()
                .map(Image::wrap)
                .ok_or_else(|| PyValueError::new_err("Invalid image index")),
            pyxel::ImageSource::Image(image) => Ok(Image::wrap(*image)),
        }
    }

    #[setter]
    fn set_image(&self, image: Image) {
        SET_IMAGE_ONCE.call_once(|| {
            println!("Tilemap.image is deprecated. Use Tilemap.imgsrc instead.");
        });

        unsafe { &mut *self.inner }.imgsrc = pyxel::ImageSource::Image(image.inner);
    }

    #[getter]
    fn refimg(&self) -> Option<u32> {
        REFIMG_ONCE.call_once(|| {
            println!("Tilemap.refimg is deprecated. Use Tilemap.imgsrc instead.");
        });

        let tilemap = unsafe { &*self.inner };
        match &tilemap.imgsrc {
            pyxel::ImageSource::Index(index) => Some(*index),
            pyxel::ImageSource::Image(_image) => None,
        }
    }

    #[setter]
    fn set_refimg(&self, img: u32) {
        SET_REFIMG_ONCE.call_once(|| {
            println!("Tilemap.refimg is deprecated. Use Tilemap.imgsrc instead.");
        });

        unsafe { &mut *self.inner }.imgsrc = pyxel::ImageSource::Index(img);
    }
}

pub fn add_tilemap_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tilemap>()?;
    Ok(())
}
