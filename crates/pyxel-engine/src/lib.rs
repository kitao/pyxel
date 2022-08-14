#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_const_for_fn,
    clippy::pedantic,
    clippy::suboptimal_flops,
    clippy::too_many_arguments
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
mod platform_sdl2;
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
pub use crate::channel::{Channel, SharedChannel};
use crate::graphics::Graphics;
pub use crate::image::{Image, SharedImage};
use crate::input::Input;
pub use crate::key::*;
pub use crate::math::Math;
pub use crate::music::{Music, SharedMusic};
use crate::platform::Platform;
use crate::platform_sdl2::PlatformSdl2;
use crate::resource::Resource;
pub use crate::settings::*;
pub use crate::sound::{SharedSound, Sound};
use crate::system::System;
pub use crate::tilemap::{SharedTilemap, Tilemap};
pub use crate::types::*;

type TargetPlatform = PlatformSdl2;

pub struct Pyxel {
    platform: TargetPlatform,
    system: System,
    resource: Resource,
    input: Input,
    graphics: Graphics,
    audio: Audio,
    math: Math,
    pub colors: [Rgb8; NUM_COLORS as usize],
    pub screen: SharedImage,
    pub cursor: SharedImage,
    pub font: SharedImage,
}

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
        let capture_scale = capture_scale.unwrap_or(DEFAULT_CAPTURE_SCALE);
        let capture_sec = capture_sec.unwrap_or(DEFAULT_CAPTURE_SEC);

        let mut platform =
            TargetPlatform::new(title, width, height, |screen_width, screen_height| {
                display_scale.unwrap_or(f64::max(
                    f64::min(
                        screen_width as f64 / width as f64,
                        screen_height as f64 / height as f64,
                    ) * DISPLAY_RATIO,
                    1.0,
                ) as u32)
            });
        let system = System::new(fps, quit_key);
        let resource = Resource::new(fps, capture_scale, capture_sec);
        let input = Input::new();
        let graphics = Graphics::new();
        let audio = Audio::new(&mut platform);
        let math = Math::new(&mut platform);

        let colors = DEFAULT_COLORS;
        let screen = Image::new(width, height);
        let cursor = Graphics::new_cursor_image();
        let font = Graphics::new_font_image();

        let mut pyxel = Self {
            platform,
            system,
            resource,
            input,
            graphics,
            audio,
            math,
            colors,
            screen,
            cursor,
            font,
        };
        pyxel.icon(&ICON_DATA, ICON_SCALE);
        pyxel
    }
}
