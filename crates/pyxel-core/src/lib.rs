#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::fn_params_excessive_bools,
    clippy::match_same_arms,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::range_plus_one,
    clippy::redundant_closure,
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
mod event;
mod graphics;
mod image;
mod input;
mod key;
mod math;
mod music;
mod oscillator;
mod platform;
mod profiler;
mod rectarea;
mod resource;
mod screencast;
mod settings;
mod sound;
mod system;
mod tilemap;
mod types;

use crate::audio::Audio;
pub use crate::audio::{channel, music, play, play1, play_pos, playm, sound, stop, stop0};
pub use crate::channel::{Channel, SharedChannel};
use crate::graphics::Graphics;
pub use crate::graphics::{
    blt, bltm, camera, camera0, circ, circb, clip, clip0, cls, colors, cursor, elli, ellib, fill,
    font, image, image_no, line, pal, pal0, pget, pset, rect, rectb, screen, text, tilemap, tri,
    trib,
};
pub use crate::image::{Image, SharedImage};
use crate::input::Input;
pub use crate::input::{
    btn, btnp, btnr, btnv, drop_files, input_keys, input_text, mouse, mouse_wheel, mouse_x,
    mouse_y, set_btn, set_btnv, set_mouse_pos,
};
pub use crate::key::*;
use crate::math::Math;
pub use crate::math::{atan2, ceil, cos, floor, noise, nseed, rndf, rndi, rseed, sgn, sin, sqrt};
pub use crate::music::{Music, SharedMusic};
use crate::platform::{DisplayScale, Platform};
use crate::resource::Resource;
pub use crate::resource::{load, reset_capture, save, screencast, screenshot};
pub use crate::settings::*;
pub use crate::sound::{SharedSound, Sound};
use crate::system::System;
pub use crate::system::{
    frame_count, fullscreen, height, icon, is_fullscreen, quit, run, show, title, width,
    PyxelCallback,
};
pub use crate::tilemap::{SharedTilemap, Tilemap};
pub use crate::types::*;

pub fn init(
    width: u32,
    height: u32,
    title: Option<&str>,
    fps: Option<u32>,
    quit_key: Option<Key>,
    display_scale: Option<u32>,
    capture_scale: Option<u32>,
    capture_sec: Option<u32>,
) {
    let title = title.unwrap_or(DEFAULT_TITLE);
    let fps = fps.unwrap_or(DEFAULT_FPS);
    let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);
    let display_scale = display_scale.map_or(DisplayScale::Ratio(DISPLAY_RATIO), |scale| {
        DisplayScale::Scale(scale)
    });
    let capture_scale = capture_scale.unwrap_or(DEFAULT_CAPTURE_SCALE);
    let capture_sec = capture_sec.unwrap_or(DEFAULT_CAPTURE_SEC);
    Platform::init(title, width, height, display_scale);
    System::init(fps, quit_key);
    crate::icon(&ICON_DATA, ICON_SCALE);
    Resource::init(fps, capture_scale, capture_sec);
    Input::init();
    Graphics::init();
    Audio::init();
    Math::init();
}
