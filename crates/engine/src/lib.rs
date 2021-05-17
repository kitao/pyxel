#[macro_use]
mod system;
mod audio;
mod canvas;
mod event;
mod graphics;
mod image;
mod input;
mod keycode;
mod music;
mod palette;
mod platform;
mod rectarea;
mod resource;
mod sdl2;
mod settings;
mod sound;
mod tilemap;

use crate::audio::Audio;
use crate::canvas::Canvas;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::palette::{Color, Rgb24};
use crate::resource::Resource;
use crate::sdl2::Sdl2;
use crate::system::System;

pub use crate::keycode::*;
pub use crate::settings::*;

type CurrentPlatform = Sdl2;

#[inline]
fn i32_to_u32(v: i32) -> u32 {
    if v < 0 {
        0
    } else {
        v as u32
    }
}

#[inline]
fn u32_to_i32(v: u32) -> i32 {
    if v > i32::MAX as u32 {
        i32::MAX
    } else {
        v as i32
    }
}

pub struct Pyxel {
    system: System<CurrentPlatform>,
    resource: Resource,
    input: Input,
    graphics: Graphics,
    audio: Audio,
}

pub trait PyxelCallback {
    fn update(&mut self, pyxel: &mut Pyxel);
    fn draw(&mut self, pyxel: &mut Pyxel);
}

impl Pyxel {
    //
    // System
    //
    #[inline]
    pub fn init(
        width: u32,
        height: u32,
        caption: Option<&str>,
        colors: Option<&[Rgb24]>,
        fps: Option<u32>,
    ) -> Pyxel {
        let platform = Sdl2::new(width, height);
        let system = System::new(platform, width, height, caption, fps);
        let graphics = Graphics::new(width, height, colors);

        Pyxel {
            system: system,
            resource: Resource {},
            input: Input {},
            graphics: graphics,
            audio: Audio {},
        }
    }

    #[inline]
    pub fn width(&self) -> i32 {
        u32_to_i32(self.system.width())
    }

    #[inline]
    pub fn height(&self) -> i32 {
        u32_to_i32(self.system.height())
    }

    #[inline]
    pub fn frame_count(&self) -> i32 {
        u32_to_i32(self.system.frame_count())
    }

    #[inline]
    pub fn run(&mut self, callback: &mut dyn PyxelCallback) {
        run!(self.system, &self.graphics, callback, self);
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
