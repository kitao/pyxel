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
mod blipbuf;
mod canvas;
mod channel;
mod graphics;
mod image;
mod input;
mod math;
mod music;
mod oscillator;
mod profiler;
mod rectarea;
mod resource;
mod screencast;
mod settings;
mod sound;
mod system;
mod tilemap;

pub use pyxel_platform::keys::*;

pub use crate::audio::{channel, music, play, play1, play_pos, playm, sound, stop, stop0};
pub use crate::channel::{Channel, Note, SharedChannel, Speed, Volume};
pub use crate::graphics::{
    blt, bltm, camera, camera0, circ, circb, clip, clip0, cls, colors, cursor, elli, ellib, fill,
    font, image, image_no, line, pal, pal0, pget, pset, rect, rectb, screen, text, tilemap, tri,
    trib,
};
pub use crate::image::{Color, Image, Rgb8, SharedImage};
pub use crate::input::{
    btn, btnp, btnr, btnv, drop_files, input_text, mouse, mouse_wheel, mouse_x, mouse_y,
    set_mouse_pos,
};
pub use crate::math::{atan2, ceil, cos, floor, noise, nseed, rndf, rndi, rseed, sgn, sin, sqrt};
pub use crate::music::{Music, SharedMusic};
pub use crate::oscillator::{Effect, Tone};
pub use crate::resource::{load, reset_capture, save, screencast, screenshot};
pub use crate::settings::*;
pub use crate::sound::{SharedSound, Sound};
pub use crate::system::{
    flip, frame_count, fullscreen, height, icon, init, is_fullscreen, quit, run, show, title,
    width, PyxelCallback,
};
pub use crate::tilemap::{SharedTilemap, Tile, Tilemap};
