use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;

#[pyfunction]
#[pyo3(
    text_signature = "(filename, *, colors, images, tilemaps, channels, sounds, musics, waveforms)"
)]
fn load(
    filename: &str,
    colors: Option<bool>,
    images: Option<bool>,
    tilemaps: Option<bool>,
    channels: Option<bool>,
    sounds: Option<bool>,
    musics: Option<bool>,
    waveforms: Option<bool>,
) {
    pyxel().load(
        filename, colors, images, tilemaps, channels, sounds, musics, waveforms,
    );
}

#[pyfunction]
#[pyo3(
    text_signature = "(filename, *, colors, images, tilemaps, channels, sounds, musics, waveforms)"
)]
fn save(
    filename: &str,
    colors: Option<bool>,
    images: Option<bool>,
    tilemaps: Option<bool>,
    channels: Option<bool>,
    sounds: Option<bool>,
    musics: Option<bool>,
    waveforms: Option<bool>,
) {
    pyxel().save(
        filename, colors, images, tilemaps, channels, sounds, musics, waveforms,
    );
}

#[pyfunction]
fn screenshot(scale: Option<u32>) {
    pyxel().screenshot(scale);
}

#[pyfunction]
fn screencast(scale: Option<u32>) {
    pyxel().screencast(scale);
}

#[pyfunction]
fn reset_screencast() {
    pyxel().reset_screencast();
}

pub fn add_resource_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    m.add_function(wrap_pyfunction!(screenshot, m)?)?;
    m.add_function(wrap_pyfunction!(screencast, m)?)?;
    m.add_function(wrap_pyfunction!(reset_screencast, m)?)?;
    Ok(())
}
