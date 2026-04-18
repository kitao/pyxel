use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;

fn resolve_exclude(new: Option<bool>, deprecated: Option<bool>) -> Option<bool> {
    if deprecated.is_some() {
        deprecation_warning!(
            EXCL_OPTION_ONCE,
            "excl_* options are deprecated. Use exclude_* instead."
        );
        deprecated
    } else {
        new
    }
}

// Load/Save

#[pyfunction]
#[pyo3(signature = (filename, exclude_images=None, exclude_tilemaps=None, exclude_sounds=None, exclude_musics=None, excl_images=None, excl_tilemaps=None, excl_sounds=None, excl_musics=None))]
fn load(
    filename: &str,
    exclude_images: Option<bool>,
    exclude_tilemaps: Option<bool>,
    exclude_sounds: Option<bool>,
    exclude_musics: Option<bool>,
    excl_images: Option<bool>,
    excl_tilemaps: Option<bool>,
    excl_sounds: Option<bool>,
    excl_musics: Option<bool>,
) -> PyResult<()> {
    pyxel()
        .load_resource(
            filename,
            resolve_exclude(exclude_images, excl_images),
            resolve_exclude(exclude_tilemaps, excl_tilemaps),
            resolve_exclude(exclude_sounds, excl_sounds),
            resolve_exclude(exclude_musics, excl_musics),
        )
        .map_err(PyException::new_err)
}

#[pyfunction]
#[pyo3(signature = (filename, exclude_images=None, exclude_tilemaps=None, exclude_sounds=None, exclude_musics=None, excl_images=None, excl_tilemaps=None, excl_sounds=None, excl_musics=None))]
fn save(
    filename: &str,
    exclude_images: Option<bool>,
    exclude_tilemaps: Option<bool>,
    exclude_sounds: Option<bool>,
    exclude_musics: Option<bool>,
    excl_images: Option<bool>,
    excl_tilemaps: Option<bool>,
    excl_sounds: Option<bool>,
    excl_musics: Option<bool>,
) -> PyResult<()> {
    pyxel()
        .save_resource(
            filename,
            resolve_exclude(exclude_images, excl_images),
            resolve_exclude(exclude_tilemaps, excl_tilemaps),
            resolve_exclude(exclude_sounds, excl_sounds),
            resolve_exclude(exclude_musics, excl_musics),
        )
        .map_err(PyException::new_err)
}

// Palette

#[pyfunction]
fn load_pal(filename: &str) -> PyResult<()> {
    pyxel().load_palette(filename).map_err(PyException::new_err)
}

#[pyfunction]
fn save_pal(filename: &str) -> PyResult<()> {
    pyxel().save_palette(filename).map_err(PyException::new_err)
}

// Screenshot/Screencast

#[pyfunction]
#[pyo3(signature = (filename=None, scale=None))]
fn screenshot(filename: Option<&str>, scale: Option<u32>) -> PyResult<()> {
    pyxel()
        .take_screenshot(filename, scale)
        .map_err(PyException::new_err)
}

#[pyfunction]
#[pyo3(signature = (filename=None, scale=None))]
fn screencast(filename: Option<&str>, scale: Option<u32>) -> PyResult<()> {
    pyxel()
        .save_screencast(filename, scale)
        .map_err(PyException::new_err)
}

#[pyfunction]
fn reset_screencast() {
    pyxel().reset_screencast();
}

// User data

#[pyfunction]
fn user_data_dir(vendor_name: &str, app_name: &str) -> PyResult<String> {
    pyxel()
        .user_data_dir(vendor_name, app_name)
        .map_err(PyException::new_err)
}

pub fn add_resource_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    m.add_function(wrap_pyfunction!(load_pal, m)?)?;
    m.add_function(wrap_pyfunction!(save_pal, m)?)?;
    m.add_function(wrap_pyfunction!(screenshot, m)?)?;
    m.add_function(wrap_pyfunction!(screencast, m)?)?;
    m.add_function(wrap_pyfunction!(reset_screencast, m)?)?;
    m.add_function(wrap_pyfunction!(user_data_dir, m)?)?;
    Ok(())
}
