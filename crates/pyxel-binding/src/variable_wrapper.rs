use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;

use crate::channel_wrapper::Channel;
use crate::image_wrapper::Image;
use crate::music_wrapper::Music;
use crate::sound_wrapper::Sound;
use crate::tilemap_wrapper::Tilemap;
use crate::tone_wrapper::Tone;

// Python sequence wrappers for the global resource lists

wrap_as_python_primitive_sequence!(
    Colors,
    u32, // Global sequence handle has no per-instance state
    (|_| pyxel::colors().len()),
    pyxel::Rgb24,
    (|_, index| pyxel::colors()[index]),
    pyxel::Rgb24,
    (|_, index, value| pyxel::colors()[index] = value),
    (|_: &u32| -> &mut Vec<pyxel::Rgb24> { pyxel::colors() }),
    Vec<pyxel::Rgb24>,
    (|_, list| *pyxel::colors() = list),
    (|_| pyxel::colors().clone())
);

macro_rules! wrap_ptr_vec_as_python_object_sequence {
    ($wrapper_name:ident, $value_type:ident, $rc_type:path, $global_fn:path) => {
        wrap_as_python_object_sequence!(
            $wrapper_name,
            u32, // Global sequence handle has no per-instance state
            (|_| $global_fn().len()),
            $value_type,
            (|_, index: usize| $value_type::wrap($global_fn()[index].clone())),
            $value_type,
            (|_, index, value: $value_type| $global_fn()[index] = value.inner),
            $rc_type,
            (|_: &u32| -> &mut Vec<$rc_type> { $global_fn() }),
            (|v: $value_type| v.inner),
            (|p| $value_type::wrap(p)),
            Vec<$value_type>,
            (|_, list: Vec<$value_type>| *$global_fn() =
                list.into_iter().map(|value| value.inner).collect()),
            (|_| $global_fn()
                .iter()
                .map(|rc| $value_type::wrap(rc.clone()))
                .collect::<Vec<$value_type>>())
        );
    };
}

wrap_ptr_vec_as_python_object_sequence!(Images, Image, pyxel::RcImage, pyxel::images);
wrap_ptr_vec_as_python_object_sequence!(Tilemaps, Tilemap, pyxel::RcTilemap, pyxel::tilemaps);
wrap_ptr_vec_as_python_object_sequence!(Channels, Channel, pyxel::RcChannel, pyxel::channels);
wrap_ptr_vec_as_python_object_sequence!(Tones, Tone, pyxel::RcTone, pyxel::tones);
wrap_ptr_vec_as_python_object_sequence!(Sounds, Sound, pyxel::RcSound, pyxel::sounds);
wrap_ptr_vec_as_python_object_sequence!(Musics, Music, pyxel::RcMusic, pyxel::musics);

// Module attribute dispatch

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<Py<PyAny>> {
    let value = match name {
        // System
        "width" => value_to_py_any!(py, *pyxel::width()),
        "height" => value_to_py_any!(py, *pyxel::height()),
        "frame_count" => value_to_py_any!(py, *pyxel::frame_count()),

        // Input
        "mouse_x" => value_to_py_any!(py, *pyxel::mouse_x()),
        "mouse_y" => value_to_py_any!(py, *pyxel::mouse_y()),
        "mouse_wheel" => value_to_py_any!(py, *pyxel::mouse_wheel()),
        "input_keys" => value_to_py_any!(py, pyxel::input_keys().clone()),
        "input_text" => value_to_py_any!(py, pyxel::input_text().clone()),
        "dropped_files" => value_to_py_any!(py, pyxel::dropped_files().clone()),

        // Graphics
        "colors" => instance_to_py_any!(py, Colors::wrap(0)),
        "images" => instance_to_py_any!(py, Images::wrap(0)),
        "tilemaps" => instance_to_py_any!(py, Tilemaps::wrap(0)),
        "screen" => instance_to_py_any!(py, Image::wrap(pyxel::screen().clone())),
        "cursor" => instance_to_py_any!(py, Image::wrap(pyxel::cursor_image().clone())),
        "font" => instance_to_py_any!(py, Image::wrap(pyxel::font_image().clone())),

        // Audio
        "channels" => instance_to_py_any!(py, Channels::wrap(0)),
        "tones" => instance_to_py_any!(py, Tones::wrap(0)),
        "sounds" => instance_to_py_any!(py, Sounds::wrap(0)),
        "musics" => instance_to_py_any!(py, Musics::wrap(0)),

        // Others
        _ => {
            return Err(PyAttributeError::new_err(format!(
                "module 'pyxel' has no attribute '{name}'"
            )))
        }
    };
    Ok(value)
}

// Module registration

pub fn add_module_variables(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Colors>()?;
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;
    Ok(())
}
