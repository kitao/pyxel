use pyo3::prelude::*;
use pyo3::types::PyTuple;

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
pub fn clip(x: Option<i32>, y: Option<i32>, width: Option<u32>, height: Option<u32>) {
    if let Some(x) = x {
        if let Some(y) = y {
            if let Some(width) = width {
                if let Some(height) = height {
                    instance().clip(x, y, width, height);
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
fn pget(x: i32, y: i32) -> Color {
    instance().pget(x, y)
}

#[pyfunction]
fn pset(x: i32, y: i32, color: Color) {
    instance().pset(x, y, color);
}

#[pyfunction]
fn line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
    instance().line(x1, y1, x2, y2, color);
}

#[pyfunction]
fn rect(x: i32, y: i32, width: u32, height: u32, color: Color) {
    instance().rect(x, y, width, height, color);
}

#[pyfunction]
fn rectb(x: i32, y: i32, width: u32, height: u32, color: Color) {
    instance().rectb(x, y, width, height, color);
}

#[pyfunction]
fn circ(x: i32, y: i32, radius: u32, color: Color) {
    instance().circ(x, y, radius, color);
}

#[pyfunction]
fn circb(x: i32, y: i32, radius: u32, color: Color) {
    instance().circb(x, y, radius, color);
}

#[pyfunction]
fn tri(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
    instance().tri(x1, y1, x2, y2, x3, y3, color);
}

#[pyfunction]
fn trib(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
    instance().trib(x1, y1, x2, y2, x3, y3, color);
}

#[pyfunction]
fn blt(
    x: i32,
    y: i32,
    image_no: u32,
    image_x: i32,
    image_y: i32,
    width: i32,
    height: i32,
    color_key: Option<Color>,
) {
    instance().blt(x, y, image_no, image_x, image_y, width, height, color_key);
}

#[pyfunction]
fn bltm(
    x: i32,
    y: i32,
    tilemap_no: u32,
    tilemap_x: i32,
    tilemap_y: i32,
    width: i32,
    height: i32,
    tile_key: Option<&PyTuple>,
) {
    let tile_key = if let Some(tile_key) = tile_key {
        Some((
            tile_key.get_item(0).extract().unwrap(),
            tile_key.get_item(1).extract().unwrap(),
        ))
    } else {
        None
    };

    instance().bltm(
        x, y, tilemap_no, tilemap_x, tilemap_y, width, height, tile_key,
    );
}

#[pyfunction]
fn text(x: i32, y: i32, string: &str, color: Color) {
    instance().text(x, y, string, color);
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
