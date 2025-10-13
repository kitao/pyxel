use std::sync::Once;

use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;

static EXCL_OPTION_ONCE: Once = Once::new();

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
) {
    let exclude_images = if excl_images.is_some() {
        EXCL_OPTION_ONCE.call_once(|| {
            println!("excl_* options are deprecated. Use exclude_* instead.");
        });
        excl_images
    } else {
        exclude_images
    };
    let exclude_tilemaps = if excl_tilemaps.is_some() {
        EXCL_OPTION_ONCE.call_once(|| {
            println!("excl_* options are deprecated. Use exclude_* instead.");
        });
        excl_tilemaps
    } else {
        exclude_tilemaps
    };
    let exclude_sounds = if excl_sounds.is_some() {
        EXCL_OPTION_ONCE.call_once(|| {
            println!("excl_* options are deprecated. Use exclude_* instead.");
        });
        excl_sounds
    } else {
        exclude_sounds
    };
    let exclude_musics = if excl_musics.is_some() {
        EXCL_OPTION_ONCE.call_once(|| {
            println!("excl_* options are deprecated. Use exclude_* instead.");
        });
        excl_musics
    } else {
        exclude_musics
    };

    pyxel().load(
        filename,
        exclude_images,
        exclude_tilemaps,
        exclude_sounds,
        exclude_musics,
    );
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
) {
    let exclude_images = if excl_images.is_some() {
        EXCL_OPTION_ONCE.call_once(|| {
            println!("excl_* options are deprecated. Use exclude_* instead.");
        });
        excl_images
    } else {
        exclude_images
    };
    let exclude_tilemaps = if excl_tilemaps.is_some() {
        EXCL_OPTION_ONCE.call_once(|| {
            println!("excl_* options are deprecated. Use exclude_* instead.");
        });
        excl_tilemaps
    } else {
        exclude_tilemaps
    };
    let exclude_sounds = if excl_sounds.is_some() {
        EXCL_OPTION_ONCE.call_once(|| {
            println!("excl_* options are deprecated. Use exclude_* instead.");
        });
        excl_sounds
    } else {
        exclude_sounds
    };
    let exclude_musics = if excl_musics.is_some() {
        EXCL_OPTION_ONCE.call_once(|| {
            println!("excl_* options are deprecated. Use exclude_* instead.");
        });
        excl_musics
    } else {
        exclude_musics
    };

    pyxel().save(
        filename,
        exclude_images,
        exclude_tilemaps,
        exclude_sounds,
        exclude_musics,
    );
}

#[pyfunction]
fn load_pal(filename: &str) {
    pyxel().load_pal(filename);
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
    m.add_function(wrap_pyfunction!(load_pal, m)?)?;
    m.add_function(wrap_pyfunction!(screenshot, m)?)?;
    m.add_function(wrap_pyfunction!(screencast, m)?)?;
    m.add_function(wrap_pyfunction!(reset_screencast, m)?)?;
    m.add_function(wrap_pyfunction!(user_data_dir, m)?)?;
    Ok(())
}
