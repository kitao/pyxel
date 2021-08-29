use pyo3::prelude::*;

use crate::instance;

#[pyfunction]
fn load(
    filename: &str,
    image: Option<bool>,
    tilemap: Option<bool>,
    sound: Option<bool>,
    music: Option<bool>,
) -> PyResult<()> {
    let image = image.unwrap_or(false);
    let tilemap = tilemap.unwrap_or(false);
    let sound = sound.unwrap_or(false);
    let music = music.unwrap_or(false);

    instance().load(filename, image, tilemap, sound, music);

    Ok(())
}

#[pyfunction]
fn save(
    filename: &str,
    image: Option<bool>,
    tilemap: Option<bool>,
    sound: Option<bool>,
    music: Option<bool>,
) -> PyResult<()> {
    let image = image.unwrap_or(false);
    let tilemap = tilemap.unwrap_or(false);
    let sound = sound.unwrap_or(false);
    let music = music.unwrap_or(false);

    instance().save(filename, image, tilemap, sound, music);

    Ok(())
}

#[pyfunction]
fn screenshot() -> PyResult<()> {
    instance().screenshot();

    Ok(())
}

#[pyfunction]
fn reset_capture() -> PyResult<()> {
    instance().reset_capture();

    Ok(())
}

#[pyfunction]
fn screencast() -> PyResult<()> {
    instance().screencast();

    Ok(())
}

pub fn add_resource_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    m.add_function(wrap_pyfunction!(screenshot, m)?)?;
    m.add_function(wrap_pyfunction!(reset_capture, m)?)?;
    m.add_function(wrap_pyfunction!(screencast, m)?)?;

    Ok(())
}
