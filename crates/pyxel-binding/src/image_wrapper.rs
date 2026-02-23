use std::ffi::CString;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::font_wrapper::Font;
use crate::tilemap_wrapper::Tilemap;

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub struct Image {
    pub(crate) inner: *mut pyxel::Image,
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl Image {
    pub fn wrap(inner: *mut pyxel::Image) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Image {
    #[new]
    pub fn new(width: u32, height: u32) -> Self {
        Self::wrap(pyxel::Image::new(width, height))
    }

    #[staticmethod]
    #[pyo3(signature = (filename, *, include_colors=None, incl_colors=None))]
    pub fn from_image(
        filename: &str,
        include_colors: Option<bool>,
        incl_colors: Option<bool>,
    ) -> PyResult<Self> {
        let include_colors = include_colors.or(incl_colors);
        pyxel::Image::from_image(filename, include_colors)
            .map(Self::wrap)
            .map_err(PyException::new_err)
    }

    #[getter]
    pub fn width(&self) -> u32 {
        unsafe { &*self.inner }.width()
    }

    #[getter]
    pub fn height(&self) -> u32 {
        unsafe { &*self.inner }.height()
    }

    pub fn data_ptr(&self, py: Python) -> Py<PyAny> {
        let inner = unsafe { &mut *self.inner };
        let python_code = CString::new(format!(
            "import ctypes; c_uint8_array = (ctypes.c_uint8 * {}).from_address({:p})",
            inner.width() * inner.height(),
            inner.data_ptr()
        ))
        .unwrap();
        let locals = PyDict::new(py);
        py.run(python_code.as_c_str(), None, Some(&locals)).unwrap();
        value_to_pyobj!(py, locals.get_item("c_uint8_array").unwrap())
    }

    pub fn set(&self, x: i32, y: i32, data: Vec<String>) {
        let data_refs: Vec<_> = data.iter().map(String::as_str).collect();
        unsafe { &mut *self.inner }.set(x, y, &data_refs);
    }

    #[pyo3(signature = (x, y, filename, *, include_colors=None, incl_colors=None))]
    pub fn load(
        &self,
        x: i32,
        y: i32,
        filename: &str,
        include_colors: Option<bool>,
        incl_colors: Option<bool>,
    ) -> PyResult<()> {
        let include_colors = include_colors.or(incl_colors);
        unsafe { &mut *self.inner }
            .load(x, y, filename, include_colors)
            .map_err(PyException::new_err)
    }

    pub fn save(&self, filename: &str, scale: u32) -> PyResult<()> {
        unsafe { &mut *self.inner }
            .save(filename, scale)
            .map_err(PyException::new_err)
    }

    #[pyo3(signature = (x=None, y=None, w=None, h=None))]
    pub fn clip(
        &self,
        x: Option<f32>,
        y: Option<f32>,
        w: Option<f32>,
        h: Option<f32>,
    ) -> PyResult<()> {
        if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
            unsafe { &mut *self.inner }.clip(x, y, w, h);
        } else if (x, y, w, h) == (None, None, None, None) {
            unsafe { &mut *self.inner }.clip0();
        } else {
            python_type_error!("clip() takes 0 or 4 arguments");
        }
        Ok(())
    }

    #[pyo3(signature = (x=None, y=None))]
    pub fn camera(&self, x: Option<f32>, y: Option<f32>) -> PyResult<()> {
        if let (Some(x), Some(y)) = (x, y) {
            unsafe { &mut *self.inner }.camera(x, y);
        } else if (x, y) == (None, None) {
            unsafe { &mut *self.inner }.camera0();
        } else {
            python_type_error!("camera() takes 0 or 2 arguments");
        }
        Ok(())
    }

    #[pyo3(signature = (col1=None, col2=None))]
    fn pal(&self, col1: Option<pyxel::Color>, col2: Option<pyxel::Color>) -> PyResult<()> {
        if let (Some(col1), Some(col2)) = (col1, col2) {
            unsafe { &mut *self.inner }.pal(col1, col2);
        } else if (col1, col2) == (None, None) {
            unsafe { &mut *self.inner }.pal0();
        } else {
            python_type_error!("pal() takes 0 or 2 arguments");
        }
        Ok(())
    }

    fn dither(&self, alpha: f32) {
        unsafe { &mut *self.inner }.dither(alpha);
    }

    pub fn cls(&self, col: pyxel::Color) {
        unsafe { &mut *self.inner }.cls(col);
    }

    pub fn pget(&self, x: f32, y: f32) -> pyxel::Color {
        unsafe { &mut *self.inner }.pget(x, y)
    }

    pub fn pset(&self, x: f32, y: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.pset(x, y, col);
    }

    pub fn line(&self, x1: f32, y1: f32, x2: f32, y2: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.line(x1, y1, x2, y2, col);
    }

    pub fn rect(&self, x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.rect(x, y, w, h, col);
    }

    pub fn rectb(&self, x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.rectb(x, y, w, h, col);
    }

    pub fn circ(&self, x: f32, y: f32, r: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.circ(x, y, r, col);
    }

    pub fn circb(&self, x: f32, y: f32, r: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.circb(x, y, r, col);
    }

    pub fn elli(&self, x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.elli(x, y, w, h, col);
    }

    pub fn ellib(&self, x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.ellib(x, y, w, h, col);
    }

    pub fn tri(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.tri(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn trib(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.trib(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn fill(&self, x: f32, y: f32, col: pyxel::Color) {
        unsafe { &mut *self.inner }.fill(x, y, col);
    }

    #[pyo3(signature = (x, y, img, u, v, w, h, colkey=None, rotate=None, scale=None))]
    pub fn blt(
        &self,
        x: f32,
        y: f32,
        img: Bound<'_, PyAny>,
        u: f32,
        v: f32,
        w: f32,
        h: f32,
        colkey: Option<pyxel::Color>,
        rotate: Option<f32>,
        scale: Option<f32>,
    ) -> PyResult<()> {
        cast_pyany! {
            img,
            (u32, {
                let image = pyxel::images()[img as usize];
                unsafe { (&mut *self.inner).blt(x, y, image, u, v, w, h, colkey, rotate, scale) };
            }),
            (Image, { unsafe { (&mut *self.inner).blt(x, y, img.inner, u, v, w, h, colkey, rotate, scale) }; })
        }
        Ok(())
    }

    #[pyo3(signature = (x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None))]
    pub fn bltm(
        &self,
        x: f32,
        y: f32,
        tm: Bound<'_, PyAny>,
        u: f32,
        v: f32,
        w: f32,
        h: f32,
        colkey: Option<pyxel::Color>,
        rotate: Option<f32>,
        scale: Option<f32>,
    ) -> PyResult<()> {
        cast_pyany! {
            tm,
            (u32, {
                let tilemap = pyxel::tilemaps()[tm as usize];
                unsafe { (&mut *self.inner).bltm(x, y, tilemap, u, v, w, h, colkey, rotate, scale) };
            }),
            (Tilemap, { unsafe { (&mut *self.inner).bltm(x, y, tm.inner, u, v, w, h, colkey, rotate, scale) }; })
        }
        Ok(())
    }

    #[pyo3(signature = (x, y, s, col, font=None))]
    pub fn text(&self, x: f32, y: f32, s: &str, col: pyxel::Color, font: Option<Font>) {
        let font = font.map(|f| f.inner);
        unsafe { &mut *self.inner }.text(x, y, s, col, font);
    }
}

pub fn add_image_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Image>()?;
    Ok(())
}
