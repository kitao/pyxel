use std::ffi::CString;

use pyo3::exceptions::{PyException, PyValueError};
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

    fn inner_ref(&self) -> &pyxel::Image {
        unsafe { &*self.inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn inner_mut(&self) -> &mut pyxel::Image {
        unsafe { &mut *self.inner }
    }
}

#[pymethods]
impl Image {
    // Constructors

    #[new]
    fn new(width: u32, height: u32) -> Self {
        Self::wrap(pyxel::Image::new(width, height))
    }

    #[staticmethod]
    #[pyo3(signature = (filename, include_colors=None, incl_colors=None))]
    fn from_image(
        filename: &str,
        include_colors: Option<bool>,
        incl_colors: Option<bool>,
    ) -> PyResult<Self> {
        let include_colors = include_colors.or(incl_colors);
        pyxel::Image::from_image(filename, include_colors)
            .map(Self::wrap)
            .map_err(PyException::new_err)
    }

    // Properties

    #[getter]
    fn width(&self) -> u32 {
        self.inner_ref().width()
    }

    #[getter]
    fn height(&self) -> u32 {
        self.inner_ref().height()
    }

    fn data_ptr(&self, py: Python) -> PyResult<Py<PyAny>> {
        let inner = self.inner_mut();
        let python_code = CString::new(format!(
            "import ctypes; c_uint8_array = (ctypes.c_uint8 * {}).from_address({:p})",
            inner.width() * inner.height(),
            inner.data_ptr()
        ))
        .unwrap();
        let locals = PyDict::new(py);
        py.run(python_code.as_c_str(), None, Some(&locals))?;
        let array = locals
            .get_item("c_uint8_array")?
            .ok_or_else(|| PyException::new_err("Failed to create data pointer"))?;
        Ok(array.unbind())
    }

    // Data operations

    fn set(&self, x: i32, y: i32, data: Vec<String>) {
        let data_refs: Vec<_> = data.iter().map(String::as_str).collect();
        self.inner_mut().set(x, y, &data_refs);
    }

    #[pyo3(signature = (x, y, filename, include_colors=None, incl_colors=None))]
    fn load(
        &self,
        x: i32,
        y: i32,
        filename: &str,
        include_colors: Option<bool>,
        incl_colors: Option<bool>,
    ) -> PyResult<()> {
        let include_colors = include_colors.or(incl_colors);
        self.inner_mut()
            .load(x, y, filename, include_colors)
            .map_err(PyException::new_err)
    }

    fn save(&self, filename: &str, scale: u32) -> PyResult<()> {
        self.inner_mut()
            .save(filename, scale)
            .map_err(PyException::new_err)
    }

    // Canvas operations

    #[pyo3(signature = (x=None, y=None, w=None, h=None))]
    fn clip(&self, x: Option<f32>, y: Option<f32>, w: Option<f32>, h: Option<f32>) -> PyResult<()> {
        if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
            self.inner_mut().set_clip_rect(x, y, w, h);
        } else if (x, y, w, h) == (None, None, None, None) {
            self.inner_mut().reset_clip_rect();
        } else {
            python_type_error!("clip() takes 0 or 4 arguments");
        }
        Ok(())
    }

    #[pyo3(signature = (x=None, y=None))]
    fn camera(&self, x: Option<f32>, y: Option<f32>) -> PyResult<()> {
        if let (Some(x), Some(y)) = (x, y) {
            self.inner_mut().set_draw_offset(x, y);
        } else if (x, y) == (None, None) {
            self.inner_mut().reset_draw_offset();
        } else {
            python_type_error!("camera() takes 0 or 2 arguments");
        }
        Ok(())
    }

    #[pyo3(signature = (col1=None, col2=None))]
    fn pal(&self, col1: Option<pyxel::Color>, col2: Option<pyxel::Color>) -> PyResult<()> {
        if let (Some(col1), Some(col2)) = (col1, col2) {
            self.inner_mut().map_color(col1, col2);
        } else if (col1, col2) == (None, None) {
            self.inner_mut().reset_color_map();
        } else {
            python_type_error!("pal() takes 0 or 2 arguments");
        }
        Ok(())
    }

    fn dither(&self, alpha: f32) {
        self.inner_mut().set_dithering(alpha);
    }

    fn cls(&self, col: pyxel::Color) {
        self.inner_mut().clear(col);
    }

    fn pget(&self, x: f32, y: f32) -> pyxel::Color {
        self.inner_mut().get_pixel(x, y)
    }

    fn pset(&self, x: f32, y: f32, col: pyxel::Color) {
        self.inner_mut().set_pixel(x, y, col);
    }

    fn line(&self, x1: f32, y1: f32, x2: f32, y2: f32, col: pyxel::Color) {
        self.inner_mut().draw_line(x1, y1, x2, y2, col);
    }

    fn rect(&self, x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
        self.inner_mut().draw_rect(x, y, w, h, col);
    }

    fn rectb(&self, x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
        self.inner_mut().draw_rect_border(x, y, w, h, col);
    }

    fn circ(&self, x: f32, y: f32, r: f32, col: pyxel::Color) {
        self.inner_mut().draw_circle(x, y, r, col);
    }

    fn circb(&self, x: f32, y: f32, r: f32, col: pyxel::Color) {
        self.inner_mut().draw_circle_border(x, y, r, col);
    }

    fn elli(&self, x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
        self.inner_mut().draw_ellipse(x, y, w, h, col);
    }

    fn ellib(&self, x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
        self.inner_mut().draw_ellipse_border(x, y, w, h, col);
    }

    fn tri(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: pyxel::Color) {
        self.inner_mut().draw_triangle(x1, y1, x2, y2, x3, y3, col);
    }

    fn trib(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: pyxel::Color) {
        self.inner_mut()
            .draw_triangle_border(x1, y1, x2, y2, x3, y3, col);
    }

    fn fill(&self, x: f32, y: f32, col: pyxel::Color) {
        self.inner_mut().flood_fill(x, y, col);
    }

    // Blit operations

    #[pyo3(signature = (x, y, img, u, v, w, h, colkey=None, rotate=None, scale=None))]
    fn blt(
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
                let image = pyxel::images().get(img as usize).copied()
                    .ok_or_else(|| PyValueError::new_err("Invalid image index"))?;
                unsafe { self.inner_mut().draw_image(x, y, image, u, v, w, h, colkey, rotate, scale) };
            }),

            (Image, {
                unsafe { self.inner_mut().draw_image(x, y, img.inner, u, v, w, h, colkey, rotate, scale) };
            })
        }
        Ok(())
    }

    #[pyo3(signature = (x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None))]
    fn bltm(
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
                let tilemap = pyxel::tilemaps().get(tm as usize).copied()
                    .ok_or_else(|| PyValueError::new_err("Invalid tilemap index"))?;
                unsafe { self.inner_mut().draw_tilemap(x, y, tilemap, u, v, w, h, colkey, rotate, scale) };
            }),

            (Tilemap, {
                unsafe { self.inner_mut().draw_tilemap(x, y, tm.inner, u, v, w, h, colkey, rotate, scale) };
            })
        }
        Ok(())
    }

    #[pyo3(signature = (x, y, w, h, img, pos, rot, fov=None, colkey=None))]
    fn blt3d(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        img: Bound<'_, PyAny>,
        pos: (f32, f32, f32),
        rot: (f32, f32, f32),
        fov: Option<f32>,
        colkey: Option<pyxel::Color>,
    ) -> PyResult<()> {
        cast_pyany! {
            img,

            (u32, {
                let image = pyxel::images().get(img as usize).copied()
                    .ok_or_else(|| PyValueError::new_err("Invalid image index"))?;
                unsafe { self.inner_mut().draw_image_3d(x, y, w, h, image, pos, rot, fov, colkey) };
            }),

            (Image, {
                unsafe { self.inner_mut().draw_image_3d(x, y, w, h, img.inner, pos, rot, fov, colkey) };
            })
        }
        Ok(())
    }

    #[pyo3(signature = (x, y, w, h, tm, pos, rot, fov=None, colkey=None))]
    fn bltm3d(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        tm: Bound<'_, PyAny>,
        pos: (f32, f32, f32),
        rot: (f32, f32, f32),
        fov: Option<f32>,
        colkey: Option<pyxel::Color>,
    ) -> PyResult<()> {
        cast_pyany! {
            tm,

            (u32, {
                let tilemap = pyxel::tilemaps().get(tm as usize).copied()
                    .ok_or_else(|| PyValueError::new_err("Invalid tilemap index"))?;
                unsafe { self.inner_mut().draw_tilemap_3d(x, y, w, h, tilemap, pos, rot, fov, colkey) };
            }),

            (Tilemap, {
                unsafe { self.inner_mut().draw_tilemap_3d(x, y, w, h, tm.inner, pos, rot, fov, colkey) };
            })
        }
        Ok(())
    }

    // Text

    #[pyo3(signature = (x, y, s, col, font=None))]
    fn text(&self, x: f32, y: f32, s: &str, col: pyxel::Color, font: Option<Font>) {
        let font = font.map(|f| f.inner);
        self.inner_mut().draw_text(x, y, s, col, font);
    }
}

pub fn add_image_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Image>()?;
    Ok(())
}
