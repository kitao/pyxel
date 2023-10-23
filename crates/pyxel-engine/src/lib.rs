#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::fn_params_excessive_bools,
    clippy::items_after_statements,
    clippy::match_same_arms,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::range_plus_one,
    clippy::redundant_closure,
    clippy::significant_drop_tightening,
    clippy::suboptimal_flops,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::wildcard_imports,
    clippy::zero_ptr
)]

#[macro_use]
mod utils;
mod audio;
mod blip_buf;
mod canvas;
mod channel;
mod graphics;
mod image;
mod input;
mod math;
mod music;
mod oscillator;
mod profiler;
mod pyxel;
mod rect_area;
mod resource;
mod screencast;
mod settings;
mod sound;
mod system;
mod tilemap;
mod watch_info;

use pyxel_platform::keys;

pub use crate::channel::{Channel, Note, SharedChannel, Speed, Volume};
pub use crate::graphics::SharedColors;
pub use crate::image::{Color, Image, Rgb24, SharedImage};
pub use crate::keys::*;
pub use crate::music::{Music, SharedMusic, SharedSoundNums};
pub use crate::oscillator::{Effect, Tone};
pub use crate::pyxel::{init, Pyxel};
pub use crate::settings::*;
pub use crate::sound::{SharedSound, Sound};
pub use crate::system::PyxelCallback;
pub use crate::tilemap::{SharedTilemap, Tile, Tilemap};
