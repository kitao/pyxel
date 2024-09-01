use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::font_wrapper::Font;
use crate::pyxel_singleton::pyxel;
use crate::tilemap_wrapper::Tilemap;

#[pyclass]
#[derive(Clone)]
pub struct Image {
    pub(crate) inner: pyxel::SharedImage,
}

impl Image {
    pub fn wrap(inner: pyxel::SharedImage) -> Self {
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
    #[pyo3(signature = (filename, incl_colors=None))]
    pub fn from_image(filename: &str, incl_colors: Option<bool>) -> Self {
        Self::wrap(pyxel::Image::from_image(filename, incl_colors))
    }

    #[getter]
    pub fn width(&self) -> u32 {
        self.inner.lock().width()
    }

    #[getter]
    pub fn height(&self) -> u32 {
        self.inner.lock().height()
    }

    pub fn data_ptr(&self, py: Python) -> PyObject {
        let mut inner = self.inner.lock();
        let python_code = format!(
            "import ctypes; c_uint8_array = (ctypes.c_uint8 * {}).from_address({:p})",
            inner.width() * inner.height(),
            inner.data_ptr()
        );
        let locals = PyDict::new_bound(py);
        py.run_bound(&python_code, None, Some(&locals)).unwrap();
        locals.get_item("c_uint8_array").unwrap().to_object(py)
    }

    pub fn set(&self, x: i32, y: i32, data: Vec<String>) {
        let data_refs: Vec<_> = data.iter().map(String::as_str).collect();
        self.inner.lock().set(x, y, &data_refs);
    }

    #[pyo3(signature = (x, y, filename, incl_colors=None))]
    pub fn load(&self, x: i32, y: i32, filename: &str, incl_colors: Option<bool>) {
        self.inner.lock().load(x, y, filename, incl_colors);
    }

    pub fn save(&self, filename: &str, scale: u32) {
        self.inner.lock().save(filename, scale);
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

    #[pyo3(signature = (col1=None, col2=None))]
    fn pal(&self, col1: Option<pyxel::Color>, col2: Option<pyxel::Color>) -> PyResult<()> {
        if let (Some(col1), Some(col2)) = (col1, col2) {
            self.inner.lock().pal(col1, col2);
        } else if (col1, col2) == (None, None) {
            self.inner.lock().pal0();
        } else {
            python_type_error!("pal() takes 0 or 2 arguments");
        }
        Ok(())
    }

    fn dither(&self, alpha: f32) {
        self.inner.lock().dither(alpha);
    }

    pub fn cls(&self, col: pyxel::Color) {
        self.inner.lock().cls(col);
    }

    pub fn pget(&self, x: f64, y: f64) -> pyxel::Color {
        self.inner.lock().pget(x, y)
    }

    pub fn pset(&self, x: f64, y: f64, col: pyxel::Color) {
        self.inner.lock().pset(x, y, col);
    }

    pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64, col: pyxel::Color) {
        self.inner.lock().line(x1, y1, x2, y2, col);
    }

    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64, col: pyxel::Color) {
        self.inner.lock().rect(x, y, w, h, col);
    }

    pub fn rectb(&self, x: f64, y: f64, w: f64, h: f64, col: pyxel::Color) {
        self.inner.lock().rectb(x, y, w, h, col);
    }

    pub fn circ(&self, x: f64, y: f64, r: f64, col: pyxel::Color) {
        self.inner.lock().circ(x, y, r, col);
    }

    pub fn circb(&self, x: f64, y: f64, r: f64, col: pyxel::Color) {
        self.inner.lock().circb(x, y, r, col);
    }

    pub fn elli(&self, x: f64, y: f64, w: f64, h: f64, col: pyxel::Color) {
        self.inner.lock().elli(x, y, w, h, col);
    }

    pub fn ellib(&self, x: f64, y: f64, w: f64, h: f64, col: pyxel::Color) {
        self.inner.lock().ellib(x, y, w, h, col);
    }

    pub fn tri(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: pyxel::Color) {
        self.inner.lock().tri(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn trib(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: pyxel::Color) {
        self.inner.lock().trib(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn fill(&self, x: f64, y: f64, col: pyxel::Color) {
        self.inner.lock().fill(x, y, col);
    }

    #[pyo3(signature = (x, y, img, u, v, w, h, colkey=None, rotate=None, scale=None))]
    pub fn blt(
        &self,
        x: f64,
        y: f64,
        img: Bound<'_, PyAny>,
        u: f64,
        v: f64,
        w: f64,
        h: f64,
        colkey: Option<pyxel::Color>,
        rotate: Option<f64>,
        scale: Option<f64>,
    ) -> PyResult<()> {
        cast_pyany! {
            img,
            (u32, {
                let image = pyxel().images.lock()[img as usize].clone();
                self.inner.lock().blt(x, y, image, u, v, w, h, colkey, rotate, scale);
            }),
            (Image, { self.inner.lock().blt(x, y, img.inner, u, v, w, h, colkey, rotate, scale); })
        }
        Ok(())
    }

    #[pyo3(signature = (x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None))]
    pub fn bltm(
        &self,
        x: f64,
        y: f64,
        tm: Bound<'_, PyAny>,
        u: f64,
        v: f64,
        w: f64,
        h: f64,
        colkey: Option<pyxel::Color>,
        rotate: Option<f64>,
        scale: Option<f64>,
    ) -> PyResult<()> {
        cast_pyany! {
            tm,
            (u32, {
                let tilemap = pyxel().tilemaps.lock()[tm as usize].clone();
                self.inner.lock().bltm(x, y, tilemap, u, v, w, h, colkey, rotate, scale);
            }),
            (Tilemap, { self.inner.lock().bltm(x, y, tm.inner, u, v, w, h, colkey, rotate, scale); })
        }
        Ok(())
    }

    #[pyo3(signature = (x, y, s, col, font=None))]
    pub fn text(&self, x: f64, y: f64, s: &str, col: pyxel::Color, font: Option<Font>) {
        let font = if let Some(font) = font {
            Some(font.inner.clone())
        } else {
            None
        };
        self.inner.lock().text(x, y, s, col, font);
    }
}

pub fn add_image_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Image>()?;
    Ok(())
}
