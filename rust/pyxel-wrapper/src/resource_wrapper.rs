use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;

#[pyfunction]
#[pyo3(signature = (filename, excl_images=None, excl_tilemaps=None, excl_sounds=None, excl_musics=None, incl_colors=None, incl_channels=None, incl_tones=None))]
fn load(
    filename: &str,
    excl_images: Option<bool>,
    excl_tilemaps: Option<bool>,
    excl_sounds: Option<bool>,
    excl_musics: Option<bool>,
    incl_colors: Option<bool>,
    incl_channels: Option<bool>,
    incl_tones: Option<bool>,
) {
    pyxel().load(
        filename,
        excl_images,
        excl_tilemaps,
        excl_sounds,
        excl_musics,
        incl_colors,
        incl_channels,
        incl_tones,
    );
}

#[pyfunction]
#[pyo3(signature = (filename, excl_images=None, excl_tilemaps=None, excl_sounds=None, excl_musics=None, incl_colors=None, incl_channels=None, incl_tones=None))]
fn save(
    filename: &str,
    excl_images: Option<bool>,
    excl_tilemaps: Option<bool>,
    excl_sounds: Option<bool>,
    excl_musics: Option<bool>,
    incl_colors: Option<bool>,
    incl_channels: Option<bool>,
    incl_tones: Option<bool>,
) {
    pyxel().save(
        filename,
        excl_images,
        excl_tilemaps,
        excl_sounds,
        excl_musics,
        incl_colors,
        incl_channels,
        incl_tones,
    );
}

#[pyfunction]
#[pyo3(signature = (scale=None))]
fn screenshot(scale: Option<u32>) {
    pyxel().screenshot(scale);
}

#[pyfunction]
#[pyo3(signature = (scale=None))]
fn screencast(scale: Option<u32>) {
    pyxel().screencast(scale);
}

#[pyfunction]
fn reset_screencast() {
    pyxel().reset_screencast();
}

pub fn add_resource_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    m.add_function(wrap_pyfunction!(screenshot, m)?)?;
    m.add_function(wrap_pyfunction!(screencast, m)?)?;
    m.add_function(wrap_pyfunction!(reset_screencast, m)?)?;
    Ok(())
}
