use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;

use crate::channel_wrapper::Channel;
use crate::image_wrapper::Image;
use crate::music_wrapper::Music;
use crate::sound_wrapper::Sound;
use crate::tilemap_wrapper::Tilemap;
use crate::tone_wrapper::Tone;

wrap_as_python_sequence!(
    Colors,
    u32, // Dummy
    (|_| pyxel::colors().len()),
    pyxel::Rgb24,
    (|_, index| pyxel::colors()[index]),
    pyxel::Rgb24,
    (|_, index, value| pyxel::colors()[index] = value),
    Vec<pyxel::Rgb24>,
    (|_, list| *pyxel::colors() = list),
    (|_| pyxel::colors().clone())
);

macro_rules! wrap_ptr_vec_as_python_object_list {
    ($wrapper_name:ident, $value_type:ident, $global_fn:path) => {
        wrap_as_python_object_sequence!(
            $wrapper_name,
            u32, // Dummy
            (|_| $global_fn().len()),
            $value_type,
            (|_, index: usize| $value_type::wrap($global_fn()[index])),
            $value_type,
            (|_, index, value: $value_type| $global_fn()[index] = value.inner),
            Vec<$value_type>,
            (|_, list: Vec<$value_type>| *$global_fn() =
                list.iter().map(|value| value.inner).collect()),
            (|_| $global_fn()
                .iter()
                .map(|&ptr| $value_type::wrap(ptr))
                .collect::<Vec<$value_type>>())
        );
    };
}

wrap_ptr_vec_as_python_object_list!(Images, Image, pyxel::images);
wrap_ptr_vec_as_python_object_list!(Tilemaps, Tilemap, pyxel::tilemaps);
wrap_ptr_vec_as_python_object_list!(Channels, Channel, pyxel::channels);
wrap_ptr_vec_as_python_object_list!(Tones, Tone, pyxel::tones);
wrap_ptr_vec_as_python_object_list!(Sounds, Sound, pyxel::sounds);
wrap_ptr_vec_as_python_object_list!(Musics, Music, pyxel::musics);

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<Py<PyAny>> {
    let value = match name {
        // System
        "width" => value_to_pyobj!(py, *pyxel::width()),
        "height" => value_to_pyobj!(py, *pyxel::height()),
        "frame_count" => value_to_pyobj!(py, *pyxel::frame_count()),

        // Input
        "mouse_x" => value_to_pyobj!(py, *pyxel::mouse_x()),
        "mouse_y" => value_to_pyobj!(py, *pyxel::mouse_y()),
        "mouse_wheel" => value_to_pyobj!(py, *pyxel::mouse_wheel()),
        "input_keys" => value_to_pyobj!(py, pyxel::input_keys().clone()),
        "input_text" => value_to_pyobj!(py, pyxel::input_text().clone()),
        "dropped_files" => value_to_pyobj!(py, pyxel::dropped_files().clone()),

        // Graphics
        "colors" => class_to_pyobj!(py, Colors::wrap(0)),
        "images" => class_to_pyobj!(py, Images::wrap(0)),
        "tilemaps" => class_to_pyobj!(py, Tilemaps::wrap(0)),
        "screen" => class_to_pyobj!(py, Image::wrap(std::ptr::from_mut(pyxel::screen()))),
        "cursor" => class_to_pyobj!(py, Image::wrap(std::ptr::from_mut(pyxel::cursor_image()))),
        "font" => class_to_pyobj!(py, Image::wrap(std::ptr::from_mut(pyxel::font_image()))),

        // Audio
        "channels" => class_to_pyobj!(py, Channels::wrap(0)),
        "tones" => class_to_pyobj!(py, Tones::wrap(0)),
        "sounds" => class_to_pyobj!(py, Sounds::wrap(0)),
        "musics" => class_to_pyobj!(py, Musics::wrap(0)),

        // Others
        _ => {
            return Err(PyAttributeError::new_err(format!(
                "module 'pyxel' has no attribute '{name}'"
            )))
        }
    };
    Ok(value)
}

pub fn add_module_variables(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Colors>()?;
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;
    Ok(())
}
