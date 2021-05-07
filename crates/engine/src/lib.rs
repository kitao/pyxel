mod canvas;
mod graphics;
mod imagebank;
mod palette;
mod rectarea;
mod settings;
mod system;
mod tilemap;

use canvas::Canvas;
use graphics::{graphics, init_graphics};
use palette::Color;
use system::{init_system, system};

//
// System
//
#[inline]
pub fn init(width: u32, height: u32, caption: &str) {
    init_system(width, height, caption);
    init_graphics(width, height);
}

#[inline]
pub fn width() -> u32 {
    system().screen_width()
}

#[inline]
pub fn height() -> u32 {
    system().screen_height()
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
    graphics().screen().clear_buffer(color);
}

#[inline]
pub fn pget(x: i32, y: i32) -> Color {
    graphics().screen().get_color(x, y)
}

#[inline]
pub fn pset(x: i32, y: i32, color: Color) {
    graphics().screen().set_color(x, y, color);
}
