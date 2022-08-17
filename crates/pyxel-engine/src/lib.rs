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

use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::audio::Audio;
pub use crate::channel::{Channel, SharedChannel};
use crate::graphics::Graphics;
pub use crate::image::{Image, SharedImage};
use crate::input::Input;
pub use crate::key::*;
pub use crate::math::Math;
pub use crate::music::{Music, SharedMusic};
use crate::platform::{DisplayScale, Platform};
use crate::resource::Resource;
pub use crate::settings::*;
pub use crate::sound::{SharedSound, Sound};
use crate::system::System;
pub use crate::tilemap::{SharedTilemap, Tilemap};
pub use crate::types::*;

pub static COLORS: Mutex<[Rgb8; NUM_COLORS as usize]> = Mutex::new(DEFAULT_COLORS);
pub static SCREEN: Lazy<SharedImage> = Lazy::new(|| Image::new(1, 1));
pub static CURSOR: Lazy<SharedImage> = Lazy::new(|| Graphics::new_cursor_image());
pub static FONT: Lazy<SharedImage> = Lazy::new(|| Graphics::new_font_image());

pub struct Pyxel;

pub trait PyxelCallback {
    fn update(&mut self, pyxel: &mut Pyxel);
    fn draw(&mut self, pyxel: &mut Pyxel);
}

impl Pyxel {
    pub fn new(
        width: u32,
        height: u32,
        title: Option<&str>,
        fps: Option<u32>,
        quit_key: Option<Key>,
        display_scale: Option<u32>,
        capture_scale: Option<u32>,
        capture_sec: Option<u32>,
    ) -> Self {
        let title = title.unwrap_or(DEFAULT_TITLE);
        let fps = fps.unwrap_or(DEFAULT_FPS);
        let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);
        let display_scale = display_scale.map_or(DisplayScale::Ratio(DISPLAY_RATIO), |scale| {
            DisplayScale::Scale(scale)
        });
        let capture_scale = capture_scale.unwrap_or(DEFAULT_CAPTURE_SCALE);
        let capture_sec = capture_sec.unwrap_or(DEFAULT_CAPTURE_SEC);

        SCREEN.lock().resize(width, height);
        Platform::init(title, width, height, display_scale);
        System::init(fps, quit_key);
        Resource::init(fps, capture_scale, capture_sec);
        Input::init();
        Graphics::init();
        Audio::init();
        Math::init();

        let mut pyxel = Self {};
        pyxel.icon(&ICON_DATA, ICON_SCALE);
        pyxel
    }
}
