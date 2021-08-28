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
fn save_png() -> PyResult<()> {
    instance().save_png();

    Ok(())
}

#[pyfunction]
fn reset_gif() -> PyResult<()> {
    instance().reset_gif();

    Ok(())
}

#[pyfunction]
fn save_gif() -> PyResult<()> {
    instance().save_gif();

    Ok(())
}

pub fn add_resource_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    m.add_function(wrap_pyfunction!(save_png, m)?)?;
    m.add_function(wrap_pyfunction!(reset_gif, m)?)?;
    m.add_function(wrap_pyfunction!(save_gif, m)?)?;

    Ok(())
}
