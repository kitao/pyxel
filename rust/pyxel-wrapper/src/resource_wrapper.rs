use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;

#[pyfunction]
#[pyo3(signature = (filename, skip_images=None, skip_tilemaps=None, skip_sounds=None, skip_musics=None))]
fn load(
    filename: &str,
    skip_images: Option<bool>,
    skip_tilemaps: Option<bool>,
    skip_sounds: Option<bool>,
    skip_musics: Option<bool>,
) {
    pyxel().load(
        filename,
        skip_images,
        skip_tilemaps,
        skip_sounds,
        skip_musics,
    );
}

#[pyfunction]
#[pyo3(signature = (filename, skip_images=None, skip_tilemaps=None, skip_sounds=None, skip_musics=None))]
fn save(
    filename: &str,
    skip_images: Option<bool>,
    skip_tilemaps: Option<bool>,
    skip_sounds: Option<bool>,
    skip_musics: Option<bool>,
) {
    pyxel().save(
        filename,
        skip_images,
        skip_tilemaps,
        skip_sounds,
        skip_musics,
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

#[pyfunction]
fn user_data_dir(vendor_name: &str, app_name: &str) -> String {
    pyxel().user_data_dir(vendor_name, app_name)
}

pub fn add_resource_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    m.add_function(wrap_pyfunction!(screenshot, m)?)?;
    m.add_function(wrap_pyfunction!(screencast, m)?)?;
    m.add_function(wrap_pyfunction!(reset_screencast, m)?)?;
    m.add_function(wrap_pyfunction!(user_data_dir, m)?)?;
    Ok(())
}
