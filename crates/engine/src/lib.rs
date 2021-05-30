#[macro_use]
mod system;
mod audio;
mod canvas;
mod channel;
mod event;
mod graphics;
mod image;
mod input;
mod key;
mod music;
mod palette;
mod platform;
mod rectarea;
mod resource;
mod sdl2;
mod settings;
mod sound;
mod tilemap;

use std::sync::{Arc, Mutex};

use crate::audio::Audio;
use crate::canvas::Canvas;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::palette::{Color, Rgb24};
use crate::resource::Resource;
use crate::sdl2::Sdl2;
use crate::system::System;

pub use crate::key::*;
pub use crate::settings::*;

pub struct Pyxel {
    system: System<Sdl2>,
    resource: Resource,
    input: Input,
    graphics: Graphics,
    audio: Arc<Mutex<Audio>>,
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
        scale: Option<u32>,
        fps: Option<u32>,
        colors: Option<&[Rgb24]>,
    ) -> Pyxel {
        let mut system = System::new(width, height, title, scale, fps);
        let resource = Resource::new();
        let input = Input::new();
        let graphics = Graphics::new(width, height, colors);
        let audio = Audio::new(system.platform_mut());

        Pyxel {
            system: system,
            resource: resource,
            input: input,
            graphics: graphics,
            audio: audio,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.graphics.screen().width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.graphics.screen().height()
    }

    #[inline]
    pub fn frame_count(&self) -> u32 {
        self.system.frame_count()
    }

    #[inline]
    pub fn title(&mut self, title: &str) {
        self.system.set_window_title(title);
    }

    #[inline]
    pub fn run(&mut self, callback: &mut dyn PyxelCallback) {
        run!(self, callback);
    }

    //
    // Resource
    //

    //
    // Input
    //

    //
    // Graphics
    //
    #[inline]
    pub fn cls(&mut self, col: Color) {
        self.graphics.screen_mut().clear(col);
    }

    #[inline]
    pub fn pget(&mut self, x: i32, y: i32) -> Color {
        self.graphics.screen_mut().point(x, y)
    }

    #[inline]
    pub fn pset(&mut self, x: i32, y: i32, col: Color) {
        self.graphics.screen_mut().draw_point(x, y, col);
    }

    #[inline]
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, col: Color) {
        self.graphics
            .screen_mut()
            .draw_rect(x, y, w as f64 as u32, h as f64 as u32, col);
    }

    #[inline]
    pub fn rectb(&mut self, x: i32, y: i32, w: i32, h: i32, col: Color) {
        self.graphics
            .screen_mut()
            .draw_rect_border(x, y, w as f64 as u32, h as f64 as u32, col);
    }

    //
    // Audio
    //
}
