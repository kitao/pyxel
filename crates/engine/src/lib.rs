mod audio;
mod canvas;
mod graphics;
mod image;
mod input;
mod palette;
mod rectarea;
mod resource;
mod settings;
mod system;
mod tilemap;

use audio::Audio;
use canvas::Canvas;
use graphics::Graphics;
use input::Input;
use palette::{Color, Rgb24};
use resource::Resource;
use system::System;

pub struct Pyxel {
    system: System,
    resource: Resource,
    input: Input,
    graphics: Graphics,
    audio: Audio,
}

impl Pyxel {
    //
    // System
    //
    #[inline]
    pub fn init(width: u32, height: u32, caption: Option<&str>, colors: Option<&[Rgb24]>) -> Pyxel {
        let system = System::new(width, height, caption);
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
    pub fn run(&mut self) {
        self.system.run(&self.graphics);
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
