#![warn(clippy::pedantic)]
#![allow(
    static_mut_refs,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_ptr_alignment,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::fn_params_excessive_bools,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::wildcard_imports
)]

#[macro_use]
mod utils;
mod audio;
mod bgm_generator;
mod canvas;
mod channel;
mod font;
mod graphics;
mod image;
mod input;
mod math;
mod mml_command;
mod mml_parser;
mod music;
mod old_mml_parser;
mod old_resource_data;
mod pcm_decoder;
pub(crate) mod platform;
mod profiler;
mod pyxel;
mod rect_area;
mod resource;
mod resource_data;
mod screencast;
mod settings;
mod sound;
mod system;
mod tilemap;
mod tmx_parser;
mod tone;
mod voice;
mod window_watcher;

use platform::key;

pub use crate::channel::{Channel, ChannelDetune, ChannelGain};
pub use crate::font::Font;
pub use crate::image::{Color, Image, Rgb24};
pub use crate::key::*;
pub use crate::music::Music;
pub use crate::pyxel::{
    channels, colors, cursor_image, dropped_files, font_image, frame_count, height, images, init,
    input_keys, input_text, mouse_wheel, mouse_x, mouse_y, musics, pyxel, reset_func,
    reset_statics, screen, sounds, tilemaps, tones, width, Pyxel,
};
pub use crate::settings::*;
pub use crate::sound::{Sound, SoundEffect, SoundNote, SoundSpeed, SoundTone, SoundVolume};
pub use crate::system::PyxelCallback;
pub use crate::tilemap::{ImageSource, ImageTileCoord, Tile, Tilemap};
pub use crate::tone::{Tone, ToneGain, ToneMode, ToneSample};
