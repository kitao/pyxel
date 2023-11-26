use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;

use crate::channel_wrapper::Channel;
use crate::image_wrapper::Image;
use crate::music_wrapper::Music;
use crate::pyxel_singleton::pyxel;
use crate::sound_wrapper::Sound;
use crate::tilemap_wrapper::Tilemap;
use crate::waveform_wrapper::Waveform;

wrap_as_python_list!(
    Colors,
    pyxel::Rgb24,
    u32,
    (|_| pyxel().colors.lock().len()),
    (|_, index| pyxel().colors.lock()[index]),
    (|_, index, value| pyxel().colors.lock()[index] = value),
    (|_, index, value| pyxel().colors.lock().insert(index, value)),
    (|_, index| pyxel().colors.lock().remove(index))
);

macro_rules! wrap_shared_vec_as_python_list {
    ($wrapper_name:ident, $value_type:ident, $field_name:ident) => {
        wrap_as_python_list!(
            $wrapper_name,
            $value_type,
            u32,
            (|_| pyxel().$field_name.lock().len()),
            (|_, index: usize| $value_type::wrap(pyxel().$field_name.lock()[index].clone())),
            (|_, index, value: $value_type| pyxel().$field_name.lock()[index] = value.inner),
            (|_, index, value: $value_type| pyxel().$field_name.lock().insert(index, value.inner)),
            (|_, index| pyxel().$field_name.lock().remove(index))
        );
    };
}

wrap_shared_vec_as_python_list!(Images, Image, images);
wrap_shared_vec_as_python_list!(Tilemaps, Tilemap, tilemaps);
wrap_shared_vec_as_python_list!(Channels, Channel, channels);
wrap_shared_vec_as_python_list!(Sounds, Sound, sounds);
wrap_shared_vec_as_python_list!(Musics, Music, musics);
wrap_shared_vec_as_python_list!(Waveforms, Waveform, waveforms);

#[pyfunction]
fn __getattr__(py: Python, name: &str) -> PyResult<PyObject> {
    let value = match name {
        // System
        "width" => pyxel().width.to_object(py),
        "height" => pyxel().height.to_object(py),
        "frame_count" => pyxel().frame_count.to_object(py),

        // Input
        "mouse_x" => pyxel().mouse_x.to_object(py),
        "mouse_y" => pyxel().mouse_y.to_object(py),
        "mouse_wheel" => pyxel().mouse_wheel.to_object(py),
        "input_text" => pyxel().input_text.to_object(py),
        "dropped_files" => pyxel().dropped_files.to_object(py),

        // Graphics
        "colors" => Py::new(py, Colors::wrap(0))?.into_py(py),
        "images" => Py::new(py, Images::wrap(0))?.into_py(py),
        "tilemaps" => Py::new(py, Tilemaps::wrap(0))?.into_py(py),
        "screen" => Image::wrap(pyxel().screen.clone()).into_py(py),
        "cursor" => Image::wrap(pyxel().cursor.clone()).into_py(py),
        "font" => Image::wrap(pyxel().font.clone()).into_py(py),

        // Audio
        "channels" => Py::new(py, Channels::wrap(0))?.into_py(py),
        "sounds" => Py::new(py, Sounds::wrap(0))?.into_py(py),
        "musics" => Py::new(py, Musics::wrap(0))?.into_py(py),
        "waveforms" => Py::new(py, Waveforms::wrap(0))?.into_py(py),

        // Others
        _ => {
            return Err(PyAttributeError::new_err(format!(
                "module 'pyxel' has no attribute '{name}'"
            )))
        }
    };
    Ok(value)
}

pub fn add_module_variables(m: &PyModule) -> PyResult<()> {
    m.add_class::<Colors>()?;
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;
    Ok(())
}
