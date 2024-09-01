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
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::too_many_arguments,
    clippy::unreadable_literal,
    clippy::unused_self
)]

#[macro_use]
mod utils;
mod audio;
mod blip_buf;
mod canvas;
mod channel;
mod font;
mod graphics;
mod image;
mod input;
mod math;
mod music;
mod old_resource_data;
mod oscillator;
mod profiler;
mod pyxel;
mod rect_area;
mod resource;
mod resource_data;
mod screencast;
mod settings;
mod sound;
mod system;
mod tiled_map_file;
mod tilemap;
mod tone;
mod watch_info;

use pyxel_platform::keys;

pub use crate::channel::{Channel, Detune, Note, SharedChannel, Speed, Volume};
pub use crate::font::{Font, SharedFont};
pub use crate::image::{Color, Image, Rgb24, SharedImage};
pub use crate::keys::*;
pub use crate::music::{Music, SharedMusic, SharedSeq};
pub use crate::oscillator::{Effect, Gain};
pub use crate::pyxel::{init, Pyxel};
pub use crate::settings::*;
pub use crate::sound::{SharedSound, Sound};
pub use crate::system::PyxelCallback;
pub use crate::tilemap::{ImageSource, SharedTilemap, Tile, TileCoord, Tilemap};
pub use crate::tone::{Amp4, Noise, SharedTone, Tone, Waveform};
