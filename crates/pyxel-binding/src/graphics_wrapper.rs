use std::sync::Once;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::font_wrapper::Font;
use crate::image_wrapper::Image;
use crate::pyxel_singleton::pyxel;
use crate::tilemap_wrapper::Tilemap;

static IMAGE_ONCE: Once = Once::new();
static TILEMAP_ONCE: Once = Once::new();

#[pyfunction]
#[pyo3(signature = (x=None, y=None, w=None, h=None))]
fn clip(x: Option<f32>, y: Option<f32>, w: Option<f32>, h: Option<f32>) -> PyResult<()> {
    if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
        pyxel().set_clip_rect(x, y, w, h);
    } else if (x, y, w, h) == (None, None, None, None) {
        pyxel().reset_clip_rect();
    } else {
        python_type_error!("clip() takes 0 or 4 arguments");
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (x=None, y=None))]
fn camera(x: Option<f32>, y: Option<f32>) -> PyResult<()> {
    if let (Some(x), Some(y)) = (x, y) {
        pyxel().set_draw_offset(x, y);
    } else if (x, y) == (None, None) {
        pyxel().reset_draw_offset();
    } else {
        python_type_error!("camera() takes 0 or 2 arguments");
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (col1=None, col2=None))]
fn pal(col1: Option<pyxel::Color>, col2: Option<pyxel::Color>) -> PyResult<()> {
    if let (Some(col1), Some(col2)) = (col1, col2) {
        pyxel().map_color(col1, col2);
    } else if (col1, col2) == (None, None) {
        pyxel().reset_color_map();
    } else {
        python_type_error!("pal() takes 0 or 2 arguments");
    }
    Ok(())
}

#[pyfunction]
fn dither(alpha: f32) {
    pyxel().set_dithering(alpha);
}

#[pyfunction]
fn cls(col: pyxel::Color) {
    pyxel().clear(col);
}

#[pyfunction]
fn pget(x: f32, y: f32) -> pyxel::Color {
    pyxel().get_pixel(x, y)
}

#[pyfunction]
fn pset(x: f32, y: f32, col: pyxel::Color) {
    pyxel().set_pixel(x, y, col);
}

#[pyfunction]
fn line(x1: f32, y1: f32, x2: f32, y2: f32, col: pyxel::Color) {
    pyxel().draw_line(x1, y1, x2, y2, col);
}

#[pyfunction]
fn rect(x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
    pyxel().draw_rect(x, y, w, h, col);
}

#[pyfunction]
fn rectb(x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
    pyxel().draw_rect_border(x, y, w, h, col);
}

#[pyfunction]
fn circ(x: f32, y: f32, r: f32, col: pyxel::Color) {
    pyxel().draw_circle(x, y, r, col);
}

#[pyfunction]
fn circb(x: f32, y: f32, r: f32, col: pyxel::Color) {
    pyxel().draw_circle_border(x, y, r, col);
}

#[pyfunction]
fn elli(x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
    pyxel().draw_ellipse(x, y, w, h, col);
}

#[pyfunction]
fn ellib(x: f32, y: f32, w: f32, h: f32, col: pyxel::Color) {
    pyxel().draw_ellipse_border(x, y, w, h, col);
}

#[pyfunction]
fn tri(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: pyxel::Color) {
    pyxel().draw_triangle(x1, y1, x2, y2, x3, y3, col);
}

#[pyfunction]
fn trib(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: pyxel::Color) {
    pyxel().draw_triangle_border(x1, y1, x2, y2, x3, y3, col);
}

#[pyfunction]
fn fill(x: f32, y: f32, col: pyxel::Color) {
    pyxel().flood_fill(x, y, col);
}

#[pyfunction]
#[pyo3(signature = (x, y ,img, u, v, w, h, colkey=None, rotate=None, scale=None))]
fn blt(
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
            if img as usize >= pyxel::images().len() {
                return Err(PyValueError::new_err("Invalid image index"));
            }
            pyxel().draw_image(x, y, img, u, v, w, h, colkey, rotate, scale);
        }),
        (Image, { unsafe { pyxel::screen().draw_image(x, y, img.inner, u, v, w, h, colkey, rotate, scale) }; })
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None))]
fn bltm(
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
            if tm as usize >= pyxel::tilemaps().len() {
                return Err(PyValueError::new_err("Invalid tilemap index"));
            }
            pyxel().draw_tilemap(x, y, tm, u, v, w, h, colkey, rotate, scale);
        }),
        (Tilemap, { unsafe { pyxel::screen().draw_tilemap(x, y, tm.inner, u, v, w, h, colkey, rotate, scale) }; })
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (x, y, s, col, font=None))]
fn text(x: f32, y: f32, s: &str, col: pyxel::Color, font: Option<Font>) {
    let font = font.map(|f| f.inner);
    pyxel().draw_text(x, y, s, col, font);
}

#[pyfunction]
fn image(img: u32) -> PyResult<Image> {
    IMAGE_ONCE.call_once(|| {
        println!("pyxel.image(img) is deprecated. Use pyxel.images[img] instead.");
    });

    pyxel::images()
        .get(img as usize)
        .copied()
        .map(Image::wrap)
        .ok_or_else(|| PyValueError::new_err("Invalid image index"))
}

#[pyfunction]
fn tilemap(tm: u32) -> PyResult<Tilemap> {
    TILEMAP_ONCE.call_once(|| {
        println!("pyxel.tilemap(tm) is deprecated. Use pyxel.tilemaps[tm] instead.");
    });

    pyxel::tilemaps()
        .get(tm as usize)
        .copied()
        .map(Tilemap::wrap)
        .ok_or_else(|| PyValueError::new_err("Invalid tilemap index"))
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
