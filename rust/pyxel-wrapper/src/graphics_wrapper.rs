use std::sync::Once;

use pyo3::prelude::*;

use crate::font_wrapper::Font;
use crate::image_wrapper::Image;
use crate::pyxel_singleton::pyxel;
use crate::tilemap_wrapper::Tilemap;

static IMAGE_ONCE: Once = Once::new();
static TILEMAP_ONCE: Once = Once::new();

#[pyfunction]
#[pyo3(signature = (x=None, y=None, w=None, h=None))]
fn clip(x: Option<f64>, y: Option<f64>, w: Option<f64>, h: Option<f64>) -> PyResult<()> {
    if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
        pyxel().clip(x, y, w, h);
    } else if (x, y, w, h) == (None, None, None, None) {
        pyxel().clip0();
    } else {
        python_type_error!("clip() takes 0 or 4 arguments");
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (x=None, y=None))]
fn camera(x: Option<f64>, y: Option<f64>) -> PyResult<()> {
    if let (Some(x), Some(y)) = (x, y) {
        pyxel().camera(x, y);
    } else if (x, y) == (None, None) {
        pyxel().camera0();
    } else {
        python_type_error!("camera() takes 0 or 2 arguments");
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (col1=None, col2=None))]
fn pal(col1: Option<pyxel::Color>, col2: Option<pyxel::Color>) -> PyResult<()> {
    if let (Some(col1), Some(col2)) = (col1, col2) {
        pyxel().pal(col1, col2);
    } else if (col1, col2) == (None, None) {
        pyxel().pal0();
    } else {
        python_type_error!("pal() takes 0 or 2 arguments");
    }
    Ok(())
}

#[pyfunction]
fn dither(alpha: f32) {
    pyxel().dither(alpha);
}

#[pyfunction]
fn cls(col: pyxel::Color) {
    pyxel().cls(col);
}

#[pyfunction]
fn pget(x: f64, y: f64) -> pyxel::Color {
    pyxel().pget(x, y)
}

#[pyfunction]
fn pset(x: f64, y: f64, col: pyxel::Color) {
    pyxel().pset(x, y, col);
}

#[pyfunction]
fn line(x1: f64, y1: f64, x2: f64, y2: f64, col: pyxel::Color) {
    pyxel().line(x1, y1, x2, y2, col);
}

#[pyfunction]
fn rect(x: f64, y: f64, w: f64, h: f64, col: pyxel::Color) {
    pyxel().rect(x, y, w, h, col);
}

#[pyfunction]
fn rectb(x: f64, y: f64, w: f64, h: f64, col: pyxel::Color) {
    pyxel().rectb(x, y, w, h, col);
}

#[pyfunction]
fn circ(x: f64, y: f64, r: f64, col: pyxel::Color) {
    pyxel().circ(x, y, r, col);
}

#[pyfunction]
fn circb(x: f64, y: f64, r: f64, col: pyxel::Color) {
    pyxel().circb(x, y, r, col);
}

#[pyfunction]
fn elli(x: f64, y: f64, w: f64, h: f64, col: pyxel::Color) {
    pyxel().elli(x, y, w, h, col);
}

#[pyfunction]
fn ellib(x: f64, y: f64, w: f64, h: f64, col: pyxel::Color) {
    pyxel().ellib(x, y, w, h, col);
}

#[pyfunction]
fn tri(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: pyxel::Color) {
    pyxel().tri(x1, y1, x2, y2, x3, y3, col);
}

#[pyfunction]
fn trib(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: pyxel::Color) {
    pyxel().trib(x1, y1, x2, y2, x3, y3, col);
}

#[pyfunction]
fn fill(x: f64, y: f64, col: pyxel::Color) {
    pyxel().fill(x, y, col);
}

#[pyfunction]
#[pyo3(signature = (x, y ,img, u, v, w, h, colkey=None, rotate=None, scale=None))]
fn blt(
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
        (u32, { pyxel().blt(x, y, img, u, v, w, h, colkey, rotate, scale); }),
        (Image, { pyxel().screen.lock().blt(x, y, img.inner, u, v, w, h, colkey, rotate, scale); })
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None))]
fn bltm(
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
        (u32, { pyxel().bltm(x, y, tm, u, v, w, h, colkey, rotate, scale); }),
        (Tilemap, { pyxel().screen.lock().bltm(x, y, tm.inner, u, v, w, h, colkey, rotate, scale); })
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (x, y, s, col, font=None))]
fn text(x: f64, y: f64, s: &str, col: pyxel::Color, font: Option<Font>) {
    let font = if let Some(font) = font {
        Some(font.inner.clone())
    } else {
        None
    };
    pyxel().text(x, y, s, col, font);
}

#[pyfunction]
fn image(img: u32) -> Image {
    IMAGE_ONCE.call_once(|| {
        println!("pyxel.image(img) is deprecated, use pyxel.images[img] instead.");
    });
    Image {
        inner: pyxel().images.lock()[img as usize].clone(),
    }
}

#[pyfunction]
fn tilemap(tm: u32) -> Tilemap {
    TILEMAP_ONCE.call_once(|| {
        println!("pyxel.tilemap(tm) is deprecated, use pyxel.tilemaps[tm] instead.");
    });
    Tilemap::wrap(pyxel().tilemaps.lock()[tm as usize].clone())
}

pub fn add_graphics_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(clip, m)?)?;
    m.add_function(wrap_pyfunction!(camera, m)?)?;
    m.add_function(wrap_pyfunction!(pal, m)?)?;
    m.add_function(wrap_pyfunction!(dither, m)?)?;
    m.add_function(wrap_pyfunction!(cls, m)?)?;
    m.add_function(wrap_pyfunction!(pget, m)?)?;
    m.add_function(wrap_pyfunction!(pset, m)?)?;
    m.add_function(wrap_pyfunction!(line, m)?)?;
    m.add_function(wrap_pyfunction!(rect, m)?)?;
    m.add_function(wrap_pyfunction!(rectb, m)?)?;
    m.add_function(wrap_pyfunction!(circ, m)?)?;
    m.add_function(wrap_pyfunction!(circb, m)?)?;
    m.add_function(wrap_pyfunction!(elli, m)?)?;
    m.add_function(wrap_pyfunction!(ellib, m)?)?;
    m.add_function(wrap_pyfunction!(tri, m)?)?;
    m.add_function(wrap_pyfunction!(trib, m)?)?;
    m.add_function(wrap_pyfunction!(fill, m)?)?;
    m.add_function(wrap_pyfunction!(blt, m)?)?;
    m.add_function(wrap_pyfunction!(bltm, m)?)?;
    m.add_function(wrap_pyfunction!(text, m)?)?;

    // Deprecated functions
    m.add_function(wrap_pyfunction!(image, m)?)?;
    m.add_function(wrap_pyfunction!(tilemap, m)?)?;

    Ok(())
}
