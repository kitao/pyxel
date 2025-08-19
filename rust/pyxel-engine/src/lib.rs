#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::fn_params_excessive_bools,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::unused_self
)]

#[macro_use]
mod utils;
mod audio;
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

use pyxel_platform::key;

pub use crate::channel::{Channel, ChannelDetune, ChannelGain, SharedChannel};
pub use crate::font::{Font, SharedFont};
pub use crate::image::{Color, Image, Rgb24, SharedImage};
pub use crate::key::*;
pub use crate::music::{Music, SharedMusic, SharedSeq};
pub use crate::pyxel::{init, Pyxel};
pub use crate::settings::*;
pub use crate::sound::{
    SharedSound, Sound, SoundEffect, SoundNote, SoundSpeed, SoundTone, SoundVolume,
};
pub use crate::system::PyxelCallback;
pub use crate::tilemap::{ImageSource, ImageTileCoord, SharedTilemap, Tile, Tilemap};
pub use crate::tone::{SharedTone, Tone, ToneGain, ToneMode, ToneSample};
