#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::multiple_crate_versions,
    clippy::needless_pass_by_value,
    clippy::new_without_default,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::wrong_self_convention
)]

#[macro_use]
mod utils;
mod audio_wrapper;
mod channel_wrapper;
mod constant_wrapper;
mod font_wrapper;
mod graphics_wrapper;
mod image_wrapper;
mod input_wrapper;
mod math_wrapper;
mod music_wrapper;
mod pyxel_singleton;
mod resource_wrapper;
mod sound_wrapper;
mod system_wrapper;
mod tilemap_wrapper;
mod tone_wrapper;
mod variable_wrapper;

use pyo3::prelude::*;

#[pymodule]
fn pyxel_binding(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    font_wrapper::add_font_class(&m)?;
    image_wrapper::add_image_class(&m)?;
    tilemap_wrapper::add_tilemap_class(&m)?;
    channel_wrapper::add_channel_class(&m)?;
    tone_wrapper::add_tone_class(&m)?;
    sound_wrapper::add_sound_class(&m)?;
    music_wrapper::add_music_class(&m)?;

    constant_wrapper::add_module_constants(&m)?;
    variable_wrapper::add_module_variables(&m)?;

    system_wrapper::add_system_functions(&m)?;
    resource_wrapper::add_resource_functions(&m)?;
    input_wrapper::add_input_functions(&m)?;
    graphics_wrapper::add_graphics_functions(&m)?;
    audio_wrapper::add_audio_functions(&m)?;
    math_wrapper::add_math_functions(&m)?;

    Ok(())
}
