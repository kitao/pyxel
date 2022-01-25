use pyo3::prelude::*;
use pyxel::Color;

use crate::image_wrapper::{wrap_pyxel_image, Image};
use crate::instance;
use crate::tilemap_wrapper::{wrap_pyxel_tilemap, Tilemap};

#[pyfunction]
fn image(img: u32) -> Image {
    wrap_pyxel_image(instance().image(img))
}

#[pyfunction]
fn tilemap(tm: u32) -> Tilemap {
    wrap_pyxel_tilemap(instance().tilemap(tm))
}

#[pyfunction]
fn clip(x: Option<f64>, y: Option<f64>, w: Option<f64>, h: Option<f64>) -> PyResult<()> {
    if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
        instance().clip(x, y, w, h);
    } else if (x, y, w, h) == (None, None, None, None) {
        instance().clip0();
    } else {
        type_error!("clip() takes 0 or 4 arguments");
    }
    Ok(())
}

#[pyfunction]
fn camera(x: Option<f64>, y: Option<f64>) -> PyResult<()> {
    if let (Some(x), Some(y)) = (x, y) {
        instance().camera(x, y);
    } else if (x, y) == (None, None) {
        instance().camera0();
    } else {
        type_error!("camera() takes 0 or 2 arguments");
    }
    Ok(())
}

#[pyfunction]
fn pal(col1: Option<Color>, col2: Option<Color>) -> PyResult<()> {
    if let (Some(col1), Some(col2)) = (col1, col2) {
        instance().pal(col1, col2);
    } else if (col1, col2) == (None, None) {
        instance().pal0();
    } else {
        type_error!("pal() takes 0 or 2 arguments");
    }
    Ok(())
}

#[pyfunction]
fn cls(col: Color) {
    instance().cls(col);
}

#[pyfunction]
fn pget(x: f64, y: f64) -> Color {
    instance().pget(x, y)
}

#[pyfunction]
fn pset(x: f64, y: f64, col: Color) {
    instance().pset(x, y, col);
}

#[pyfunction]
fn line(x1: f64, y1: f64, x2: f64, y2: f64, col: Color) {
    instance().line(x1, y1, x2, y2, col);
}

#[pyfunction]
fn rect(x: f64, y: f64, w: f64, h: f64, col: Color) {
    instance().rect(x, y, w, h, col);
}

#[pyfunction]
fn rectb(x: f64, y: f64, w: f64, h: f64, col: Color) {
    instance().rectb(x, y, w, h, col);
}

#[pyfunction]
fn circ(x: f64, y: f64, r: f64, col: Color) {
    instance().circ(x, y, r, col);
}

#[pyfunction]
fn circb(x: f64, y: f64, r: f64, col: Color) {
    instance().circb(x, y, r, col);
}

#[pyfunction]
fn elp(x: f64, y: f64, w: f64, h: f64, col: Color) {
    instance().elp(x, y, w, h, col);
}

#[pyfunction]
fn elpb(x: f64, y: f64, w: f64, h: f64, col: Color) {
    instance().elpb(x, y, w, h, col);
}

#[pyfunction]
fn tri(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: Color) {
    instance().tri(x1, y1, x2, y2, x3, y3, col);
}

#[pyfunction]
fn trib(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: Color) {
    instance().trib(x1, y1, x2, y2, x3, y3, col);
}

#[pyfunction]
fn fill(x: f64, y: f64, col: Color) {
    instance().fill(x, y, col);
}

#[pyfunction]
fn blt(
    x: f64,
    y: f64,
    img: &PyAny,
    u: f64,
    v: f64,
    w: f64,
    h: f64,
    colkey: Option<Color>,
) -> PyResult<()> {
    type_switch! {
        img,
        u32, {
            instance().blt(x, y, img, u, v, w, h, colkey);
        },
        Image, {
            instance().screen.lock().blt(x, y, img.pyxel_image, u, v, w, h, colkey);
        }
    }
    Ok(())
}

#[pyfunction]
fn bltm(
    x: f64,
    y: f64,
    tm: &PyAny,
    u: f64,
    v: f64,
    w: f64,
    h: f64,
    colkey: Option<Color>,
) -> PyResult<()> {
    type_switch! {
        tm,
        u32, {
            instance().bltm(x, y, tm, u, v, w, h, colkey);
        },
        Tilemap, {
            instance().screen.lock().bltm(x, y, tm.pyxel_tilemap, u, v, w, h, colkey);
        }
    }
    Ok(())
}

#[pyfunction]
fn text(x: f64, y: f64, s: &str, col: Color) {
    instance().text(x, y, s, col);
}

pub fn add_graphics_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(image, m)?)?;
    m.add_function(wrap_pyfunction!(tilemap, m)?)?;
    m.add_function(wrap_pyfunction!(clip, m)?)?;
    m.add_function(wrap_pyfunction!(camera, m)?)?;
    m.add_function(wrap_pyfunction!(pal, m)?)?;
    m.add_function(wrap_pyfunction!(cls, m)?)?;
    m.add_function(wrap_pyfunction!(pget, m)?)?;
    m.add_function(wrap_pyfunction!(pset, m)?)?;
    m.add_function(wrap_pyfunction!(line, m)?)?;
    m.add_function(wrap_pyfunction!(rect, m)?)?;
    m.add_function(wrap_pyfunction!(rectb, m)?)?;
    m.add_function(wrap_pyfunction!(circ, m)?)?;
    m.add_function(wrap_pyfunction!(circb, m)?)?;
    m.add_function(wrap_pyfunction!(ellip, m)?)?;
    m.add_function(wrap_pyfunction!(ellipb, m)?)?;
    m.add_function(wrap_pyfunction!(tri, m)?)?;
    m.add_function(wrap_pyfunction!(trib, m)?)?;
    m.add_function(wrap_pyfunction!(fill, m)?)?;
    m.add_function(wrap_pyfunction!(blt, m)?)?;
    m.add_function(wrap_pyfunction!(bltm, m)?)?;
    m.add_function(wrap_pyfunction!(text, m)?)?;
    Ok(())
}
