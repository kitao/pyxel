use pyo3::prelude::*;

use pyxel::Color;

use crate::image_wrapper::{wrap_pyxel_image, Image};
use crate::instance;
use crate::tilemap_wrapper::{wrap_pyxel_tilemap, Tilemap};

#[pyfunction]
fn image(image_no: u32) -> Image {
    wrap_pyxel_image(instance().image(image_no))
}

#[pyfunction]
fn tilemap(tilemap_no: u32) -> Tilemap {
    wrap_pyxel_tilemap(instance().tilemap(tilemap_no))
}

#[pyfunction]
pub fn clip(x: Option<&PyAny>, y: Option<&PyAny>, width: Option<&PyAny>, height: Option<&PyAny>) {
    if let Some(x) = x {
        if let Some(y) = y {
            if let Some(width) = width {
                if let Some(height) = height {
                    instance().clip(
                        as_int!(i32, x),
                        as_int!(i32, y),
                        as_int!(u32, width),
                        as_int!(u32, height),
                    );
                    return;
                }
            }
        }
    }

    if let None = x {
        if let None = y {
            if let None = width {
                if let None = height {
                    instance().clip0();
                    return;
                }
            }
        }
    }

    panic!("invalid argument number for clip");
}

#[pyfunction]
pub fn pal(src_color: Option<Color>, dst_color: Option<Color>) {
    if let Some(src_color) = src_color {
        if let Some(dst_color) = dst_color {
            instance().pal(src_color, dst_color);
            return;
        }
    }

    if let None = src_color {
        if let None = dst_color {
            instance().pal0();
            return;
        }
    }

    panic!("invalid argument number for pal");
}

#[pyfunction]
fn cls(color: Color) {
    instance().cls(color);
}

#[pyfunction]
fn pget(x: &PyAny, y: &PyAny) -> Color {
    instance().pget(as_int!(i32, x), as_int!(i32, y))
}

#[pyfunction]
fn pset(x: &PyAny, y: &PyAny, color: Color) {
    instance().pset(as_int!(i32, x), as_int!(i32, y), color);
}

#[pyfunction]
fn line(x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, color: Color) {
    instance().line(
        as_int!(i32, x1),
        as_int!(i32, y1),
        as_int!(i32, x2),
        as_int!(i32, y2),
        color,
    );
}

#[pyfunction]
fn rect(x: &PyAny, y: &PyAny, width: &PyAny, height: &PyAny, color: Color) {
    instance().rect(
        as_int!(i32, x),
        as_int!(i32, y),
        as_int!(u32, width),
        as_int!(u32, height),
        color,
    );
}

#[pyfunction]
fn rectb(x: &PyAny, y: &PyAny, width: &PyAny, height: &PyAny, color: Color) {
    instance().rectb(
        as_int!(i32, x),
        as_int!(i32, y),
        as_int!(u32, width),
        as_int!(u32, height),
        color,
    );
}

#[pyfunction]
fn circ(x: &PyAny, y: &PyAny, radius: &PyAny, color: Color) {
    instance().circ(
        as_int!(i32, x),
        as_int!(i32, y),
        as_int!(u32, radius),
        color,
    );
}

#[pyfunction]
fn circb(x: &PyAny, y: &PyAny, radius: &PyAny, color: Color) {
    instance().circb(
        as_int!(i32, x),
        as_int!(i32, y),
        as_int!(u32, radius),
        color,
    );
}

#[pyfunction]
fn tri(x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, x3: &PyAny, y3: &PyAny, color: Color) {
    instance().tri(
        as_int!(i32, x1),
        as_int!(i32, y1),
        as_int!(i32, x2),
        as_int!(i32, y2),
        as_int!(i32, x3),
        as_int!(i32, y3),
        color,
    );
}

#[pyfunction]
fn trib(x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, x3: &PyAny, y3: &PyAny, color: Color) {
    instance().trib(
        as_int!(i32, x1),
        as_int!(i32, y1),
        as_int!(i32, x2),
        as_int!(i32, y2),
        as_int!(i32, x3),
        as_int!(i32, y3),
        color,
    );
}

#[pyfunction]
fn blt(
    x: &PyAny,
    y: &PyAny,
    image_no: u32,
    image_x: &PyAny,
    image_y: &PyAny,
    width: &PyAny,
    height: &PyAny,
    color_key: Option<Color>,
) {
    instance().blt(
        as_int!(i32, x),
        as_int!(i32, y),
        image_no,
        as_int!(i32, image_x),
        as_int!(i32, image_y),
        as_int!(i32, width),
        as_int!(i32, height),
        color_key,
    );
}

#[pyfunction]
fn bltm(
    x: &PyAny,
    y: &PyAny,
    tilemap_no: u32,
    tilemap_x: &PyAny,
    tilemap_y: &PyAny,
    width: &PyAny,
    height: &PyAny,
    transparent: Option<Color>,
) {
    instance().bltm(
        as_int!(i32, x),
        as_int!(i32, y),
        tilemap_no,
        as_int!(i32, tilemap_x),
        as_int!(i32, tilemap_y),
        as_int!(i32, width),
        as_int!(i32, height),
        transparent,
    );
}

#[pyfunction]
fn text(x: &PyAny, y: &PyAny, string: &str, color: Color) {
    instance().text(as_int!(i32, x), as_int!(i32, y), string, color);
}

pub fn add_graphics_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(image, m)?)?;
    m.add_function(wrap_pyfunction!(tilemap, m)?)?;
    m.add_function(wrap_pyfunction!(clip, m)?)?;
    m.add_function(wrap_pyfunction!(pal, m)?)?;
    m.add_function(wrap_pyfunction!(cls, m)?)?;
    m.add_function(wrap_pyfunction!(pget, m)?)?;
    m.add_function(wrap_pyfunction!(pset, m)?)?;
    m.add_function(wrap_pyfunction!(line, m)?)?;
    m.add_function(wrap_pyfunction!(rect, m)?)?;
    m.add_function(wrap_pyfunction!(rectb, m)?)?;
    m.add_function(wrap_pyfunction!(circ, m)?)?;
    m.add_function(wrap_pyfunction!(circb, m)?)?;
    m.add_function(wrap_pyfunction!(tri, m)?)?;
    m.add_function(wrap_pyfunction!(trib, m)?)?;
    m.add_function(wrap_pyfunction!(blt, m)?)?;
    m.add_function(wrap_pyfunction!(bltm, m)?)?;
    m.add_function(wrap_pyfunction!(text, m)?)?;

    Ok(())
}
