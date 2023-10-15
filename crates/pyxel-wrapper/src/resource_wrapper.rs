use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;

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
    pyxel().load(filename, image, tilemap, sound, music);
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
    pyxel().save(filename, image, tilemap, sound, music);
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
