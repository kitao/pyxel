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

use canvas::Canvas;
use graphics::{graphics, init_graphics};
use palette::{Color, Rgb24};
use system::{init_system, system};

//
// System
//
#[inline]
pub fn init(width: u32, height: u32, caption: Option<&str>, colors: Option<&[Rgb24]>) {
    init_system(width, height, caption);
    init_graphics(width, height, colors);
}

#[inline]
pub fn width() -> u32 {
    system().width()
}

#[inline]
pub fn height() -> u32 {
    system().height()
}

#[inline]
pub fn run() {
    system().run();
}

//
// Graphics
//
#[inline]
pub fn cls(color: Color) {
    graphics().screen().clear(color);
}

#[inline]
pub fn pget(x: i32, y: i32) -> Color {
    graphics().screen().get_color(x, y)
}

#[inline]
pub fn pset(x: i32, y: i32, color: Color) {
    graphics().screen().draw_point(x, y, color);
}
