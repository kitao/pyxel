use pyo3::prelude::*;

use crate::instance;

#[pyfunction]
pub fn load(
    filename: &str,
    image: Option<bool>,
    tilemap: Option<bool>,
    sound: Option<bool>,
    music: Option<bool>,
) {
    let image = image.unwrap_or(false);
    let tilemap = tilemap.unwrap_or(false);
    let sound = sound.unwrap_or(false);
    let music = music.unwrap_or(false);

    instance().load(filename, image, tilemap, sound, music);
}

#[pyfunction]
pub fn save(
    filename: &str,
    image: Option<bool>,
    tilemap: Option<bool>,
    sound: Option<bool>,
    music: Option<bool>,
) {
    let image = image.unwrap_or(false);
    let tilemap = tilemap.unwrap_or(false);
    let sound = sound.unwrap_or(false);
    let music = music.unwrap_or(false);

    instance().save(filename, image, tilemap, sound, music);
}

pub fn add_resource_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;

    Ok(())
}
