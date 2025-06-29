use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;

#[pyfunction]
#[pyo3(signature = (filename, ignore_images=None, ignore_tilemaps=None, ignore_sounds=None, ignore_musics=None))]
fn load(
    filename: &str,
    ignore_images: Option<bool>,
    ignore_tilemaps: Option<bool>,
    ignore_sounds: Option<bool>,
    ignore_musics: Option<bool>,
) {
    pyxel().load(
        filename,
        ignore_images,
        ignore_tilemaps,
        ignore_sounds,
        ignore_musics,
    );
}

#[pyfunction]
#[pyo3(signature = (filename, ignore_images=None, ignore_tilemaps=None, ignore_sounds=None, ignore_musics=None))]
fn save(
    filename: &str,
    ignore_images: Option<bool>,
    ignore_tilemaps: Option<bool>,
    ignore_sounds: Option<bool>,
    ignore_musics: Option<bool>,
) {
    pyxel().save(
        filename,
        ignore_images,
        ignore_tilemaps,
        ignore_sounds,
        ignore_musics,
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
