use pyo3::exceptions::PyTypeError;
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
fn clip(
    x: Option<&PyAny>,
    y: Option<&PyAny>,
    width: Option<&PyAny>,
    height: Option<&PyAny>,
) -> PyResult<()> {
    if let (Some(x), Some(y), Some(width), Some(height)) = (x, y, width, height) {
        instance().clip(as_i32!(x), as_i32!(y), as_u32!(width), as_u32!(height));
    } else if let (None, None, None, None) = (x, y, width, height) {
        instance().clip0();
    } else {
        return Err(PyTypeError::new_err("clip() takes 0 or 4 arguments"));
    }

    Ok(())
}

#[pyfunction]
fn pal(src_color: Option<Color>, dst_color: Option<Color>) -> PyResult<()> {
    if let (Some(src_color), Some(dst_color)) = (src_color, dst_color) {
        instance().pal(src_color, dst_color);
    } else if let (None, None) = (src_color, dst_color) {
        instance().pal0();
    } else {
        return Err(PyTypeError::new_err("pal() takes 0 or 2 arguments"));
    }

    Ok(())
}

#[pyfunction]
fn cls(color: Color) {
    instance().cls(color);
}

#[pyfunction]
fn pget(x: &PyAny, y: &PyAny) -> Color {
    instance().pget(as_i32!(x), as_i32!(y))
}

#[pyfunction]
fn pset(x: &PyAny, y: &PyAny, color: Color) {
    instance().pset(as_i32!(x), as_i32!(y), color);
}

#[pyfunction]
fn line(x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, color: Color) {
    instance().line(as_i32!(x1), as_i32!(y1), as_i32!(x2), as_i32!(y2), color);
}

#[pyfunction]
fn rect(x: &PyAny, y: &PyAny, width: &PyAny, height: &PyAny, color: Color) {
    instance().rect(
        as_i32!(x),
        as_i32!(y),
        as_u32!(width),
        as_u32!(height),
        color,
    );
}

#[pyfunction]
fn rectb(x: &PyAny, y: &PyAny, width: &PyAny, height: &PyAny, color: Color) {
    instance().rectb(
        as_i32!(x),
        as_i32!(y),
        as_u32!(width),
        as_u32!(height),
        color,
    );
}

#[pyfunction]
fn circ(x: &PyAny, y: &PyAny, radius: &PyAny, color: Color) {
    instance().circ(as_i32!(x), as_i32!(y), as_u32!(radius), color);
}

#[pyfunction]
fn circb(x: &PyAny, y: &PyAny, radius: &PyAny, color: Color) {
    instance().circb(as_i32!(x), as_i32!(y), as_u32!(radius), color);
}

#[pyfunction]
fn tri(x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, x3: &PyAny, y3: &PyAny, color: Color) {
    instance().tri(
        as_i32!(x1),
        as_i32!(y1),
        as_i32!(x2),
        as_i32!(y2),
        as_i32!(x3),
        as_i32!(y3),
        color,
    );
}

#[pyfunction]
fn trib(x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, x3: &PyAny, y3: &PyAny, color: Color) {
    instance().trib(
        as_i32!(x1),
        as_i32!(y1),
        as_i32!(x2),
        as_i32!(y2),
        as_i32!(x3),
        as_i32!(y3),
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
        as_i32!(x),
        as_i32!(y),
        image_no,
        as_i32!(image_x),
        as_i32!(image_y),
        as_i32!(width),
        as_i32!(height),
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
        as_i32!(x),
        as_i32!(y),
        tilemap_no,
        as_i32!(tilemap_x),
        as_i32!(tilemap_y),
        as_i32!(width),
        as_i32!(height),
        transparent,
    );
}

#[pyfunction]
fn text(x: &PyAny, y: &PyAny, string: &str, color: Color) {
    instance().text(as_i32!(x), as_i32!(y), string, color);
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
