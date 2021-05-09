#[macro_use]
mod system;
mod audio;
mod canvas;
mod graphics;
mod image;
mod input;
mod palette;
mod rectarea;
mod resource;
mod settings;
mod tilemap;

use audio::Audio;
use canvas::Canvas;
use graphics::Graphics;
use input::Input;
use palette::{Color, Rgb24};
use resource::Resource;
use settings::*;
use system::System;

pub struct Pyxel {
    system: System,
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
        let system = System::new(width, height, caption, fps);
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
    pub fn width(&self) -> u32 {
        self.system.width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.system.height()
    }

    #[inline]
    pub fn run(&mut self, callback: &mut dyn PyxelCallback) {
        run!(self.system, self.graphics, callback, self);
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
    pub fn cls(&mut self, color: Color) {
        self.graphics.screen_mut().clear(color);
    }

    #[inline]
    pub fn pget(&mut self, x: i32, y: i32) -> Color {
        self.graphics.screen_mut().get_color(x, y)
    }

    #[inline]
    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        self.graphics.screen_mut().draw_point(x, y, color);
    }

    //
    // Audio
    //
}
