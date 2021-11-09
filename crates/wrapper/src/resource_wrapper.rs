use pyo3::prelude::*;

use crate::instance;

#[pyfunction]
#[pyo3(text_signature = "(filename, *, image, tilemap, sound, music)")]
fn load(
    filename: &str,
    image: Option<bool>,
    tilemap: Option<bool>,
    sound: Option<bool>,
    music: Option<bool>,
) {
    let image = image.unwrap_or(true);
    let tilemap = tilemap.unwrap_or(true);
    let sound = sound.unwrap_or(true);
    let music = music.unwrap_or(true);
    instance().load(filename, image, tilemap, sound, music);
}

#[pyfunction]
#[pyo3(text_signature = "(filename, *, image, tilemap, sound, music)")]
fn save(
    filename: &str,
    image: Option<bool>,
    tilemap: Option<bool>,
    sound: Option<bool>,
    music: Option<bool>,
) {
    let image = image.unwrap_or(true);
    let tilemap = tilemap.unwrap_or(true);
    let sound = sound.unwrap_or(true);
    let music = music.unwrap_or(true);
    instance().save(filename, image, tilemap, sound, music);
}

#[pyfunction]
fn screenshot() {
    instance().screenshot();
}

#[pyfunction]
fn reset_capture() {
    instance().reset_capture();
}

#[pyfunction]
fn screencast() {
    instance().screencast();
}

pub fn add_resource_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    m.add_function(wrap_pyfunction!(screenshot, m)?)?;
    m.add_function(wrap_pyfunction!(reset_capture, m)?)?;
    m.add_function(wrap_pyfunction!(screencast, m)?)?;
    Ok(())
}
