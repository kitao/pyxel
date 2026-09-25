#![warn(clippy::pedantic)]
// Casts share one notation across integer storage and floating-point calculations.
// Range checks, rounding, clipping, and packed representations belong at call sites.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    // Exact comparisons select stored states and fixed numeric cases.
    clippy::float_cmp,
    // Resource-bank options are independent choices, as are channel states.
    clippy::fn_params_excessive_bools,
    // Preserve f32 arithmetic; midpoint promotes to f64 and changes rounding.
    clippy::manual_midpoint,
    // The coding policy excludes Rust documentation comments.
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    // Mark consequential discarded results, such as lock guards, individually.
    clippy::must_use_candidate,
    // Related axes, endpoints, and input/result pairs keep related names.
    clippy::similar_names,
    clippy::struct_excessive_bools,
    // Keep API operands explicit and closed formulas or dispatches together.
    clippy::too_many_arguments,
    clippy::too_many_lines,
    // Preserve RGB24, packed font data, and generated SDL constant spellings.
    clippy::unreadable_literal,
    // Public operations and platform counterparts retain a uniform method API.
    clippy::unused_self
)]

#[macro_use]
mod utils;
mod audio;
mod bgm_generator;
mod canvas;
mod channel;
pub mod cube;
mod font;
mod graphics;
mod image;
mod input;
mod math;
mod mml_command;
mod mml_parser;
mod music;
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

pub use crate::audio::AudioLock;
pub use crate::channel::{Channel, ChannelDetune, ChannelGain, RcChannel};
pub use crate::font::{Font, RcFont};
pub use crate::image::{Color, Image, RcImage, Rgb24};
pub use crate::key::*;
pub use crate::music::{Music, RcMusic};
#[cfg(target_os = "emscripten")]
pub use crate::pyxel::reset_statics;
pub use crate::pyxel::{
    channels, colors, cursor_image, dropped_files, font_image, frame_count, frame_seconds, height,
    images, init, input_keys, input_text, mouse_wheel, mouse_x, mouse_y, musics, pyxel,
    quit_callback, reset_callback, screen, sounds, tilemaps, tones, validate_init_params, width,
    AudioGlobalGuard, Pyxel,
};
pub use crate::settings::*;
pub use crate::sound::{
    RcSound, Sound, SoundEffect, SoundNote, SoundSpeed, SoundTone, SoundVolume,
};
pub use crate::system::PyxelCallback;
pub use crate::tilemap::{ImageSource, ImageTileCoord, RcTilemap, Tile, Tilemap};
pub use crate::tone::{RcTone, Tone, ToneGain, ToneMode, ToneSample};
