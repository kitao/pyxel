use pyo3::prelude::*;
use pyxel::{Canvas, Color};

use crate::image_wrapper::{wrap_pyxel_image, Image};
use crate::instance;
use crate::tilemap_wrapper::{wrap_pyxel_tilemap, Tilemap};

#[pyfunction]
fn image(img: u32) -> PyResult<Image> {
    Ok(wrap_pyxel_image(instance().image(img)))
}

#[pyfunction]
fn tilemap(tm: u32) -> PyResult<Tilemap> {
    Ok(wrap_pyxel_tilemap(instance().tilemap(tm)))
}

#[pyfunction]
fn clip(
    x: Option<&PyAny>,
    y: Option<&PyAny>,
    w: Option<&PyAny>,
    h: Option<&PyAny>,
) -> PyResult<()> {
    if let (Some(x), Some(y), Some(w), Some(h)) = (x, y, w, h) {
        instance().clip(as_i32!(x), as_i32!(y), as_u32!(w), as_u32!(h));
    } else if let (None, None, None, None) = (x, y, w, h) {
        instance().clip0();
    } else {
        type_error!("clip() takes 0 or 4 arguments");
    }

    Ok(())
}

#[pyfunction]
fn pal(col1: Option<Color>, col2: Option<Color>) -> PyResult<()> {
    if let (Some(col1), Some(col2)) = (col1, col2) {
        instance().pal(col1, col2);
    } else if let (None, None) = (col1, col2) {
        instance().pal0();
    } else {
        type_error!("pal() takes 0 or 2 arguments");
    }

    Ok(())
}

#[pyfunction]
fn cls(col: Color) -> PyResult<()> {
    instance().cls(col);

    Ok(())
}

#[pyfunction]
fn pget(x: &PyAny, y: &PyAny) -> PyResult<Color> {
    Ok(instance().pget(as_i32!(x), as_i32!(y)))
}

#[pyfunction]
fn pset(x: &PyAny, y: &PyAny, col: Color) -> PyResult<()> {
    instance().pset(as_i32!(x), as_i32!(y), col);

    Ok(())
}

#[pyfunction]
fn line(x1: &PyAny, y1: &PyAny, x2: &PyAny, y2: &PyAny, col: Color) -> PyResult<()> {
    instance().line(as_i32!(x1), as_i32!(y1), as_i32!(x2), as_i32!(y2), col);

    Ok(())
}

#[pyfunction]
fn rect(x: &PyAny, y: &PyAny, w: &PyAny, h: &PyAny, col: Color) -> PyResult<()> {
    instance().rect(as_i32!(x), as_i32!(y), as_u32!(w), as_u32!(h), col);

    Ok(())
}

#[pyfunction]
fn rectb(x: &PyAny, y: &PyAny, w: &PyAny, h: &PyAny, col: Color) -> PyResult<()> {
    instance().rectb(as_i32!(x), as_i32!(y), as_u32!(w), as_u32!(h), col);

    Ok(())
}

#[pyfunction]
fn circ(x: &PyAny, y: &PyAny, r: &PyAny, col: Color) -> PyResult<()> {
    instance().circ(as_i32!(x), as_i32!(y), as_u32!(r), col);

    Ok(())
}

#[pyfunction]
fn circb(x: &PyAny, y: &PyAny, r: &PyAny, col: Color) -> PyResult<()> {
    instance().circb(as_i32!(x), as_i32!(y), as_u32!(r), col);

    Ok(())
}

#[pyfunction]
fn tri(
    x1: &PyAny,
    y1: &PyAny,
    x2: &PyAny,
    y2: &PyAny,
    x3: &PyAny,
    y3: &PyAny,
    col: Color,
) -> PyResult<()> {
    instance().tri(
        as_i32!(x1),
        as_i32!(y1),
        as_i32!(x2),
        as_i32!(y2),
        as_i32!(x3),
        as_i32!(y3),
        col,
    );

    Ok(())
}

#[pyfunction]
fn trib(
    x1: &PyAny,
    y1: &PyAny,
    x2: &PyAny,
    y2: &PyAny,
    x3: &PyAny,
    y3: &PyAny,
    col: Color,
) -> PyResult<()> {
    instance().trib(
        as_i32!(x1),
        as_i32!(y1),
        as_i32!(x2),
        as_i32!(y2),
        as_i32!(x3),
        as_i32!(y3),
        col,
    );

    Ok(())
}

#[pyfunction]
fn blt(
    x: &PyAny,
    y: &PyAny,
    img: &PyAny,
    u: &PyAny,
    v: &PyAny,
    w: &PyAny,
    h: &PyAny,
    color_key: Option<Color>,
) -> PyResult<()> {
    type_switch! {
        img,
        u32,
        {
            instance().blt(
                as_i32!(x),
                as_i32!(y),
                img,
                as_i32!(u),
                as_i32!(v),
                as_i32!(w),
                as_i32!(h),
                color_key,
            );
        },
        Image,
        {
            instance().screen.lock().blt(
                as_i32!(x),
                as_i32!(y),
                img.pyxel_image,
                as_i32!(u),
                as_i32!(v),
                as_i32!(w),
                as_i32!(h),
                color_key,
            );
        }
    }

    Ok(())
}

#[pyfunction]
fn bltm(
    x: &PyAny,
    y: &PyAny,
    tm: &PyAny,
    u: &PyAny,
    v: &PyAny,
    w: &PyAny,
    h: &PyAny,
    colkey: Option<Color>,
) -> PyResult<()> {
    type_switch! {
        tm,
        u32,
        {
            instance().bltm(
                as_i32!(x),
                as_i32!(y),
                tm,
                as_i32!(u),
                as_i32!(v),
                as_i32!(w),
                as_i32!(h),
                colkey,
            );
        },
        Tilemap,
        {
            instance().screen.lock().bltm(
                as_i32!(x),
                as_i32!(y),
                tm.pyxel_tilemap,
                as_i32!(u),
                as_i32!(v),
                as_i32!(w),
                as_i32!(h),
                colkey,
            );
        }
    }

    Ok(())
}

#[pyfunction]
fn text(x: &PyAny, y: &PyAny, s: &str, col: Color) -> PyResult<()> {
    instance().text(as_i32!(x), as_i32!(y), s, col);

    Ok(())
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
