mod color_palette;
mod graphics;
mod graphics_buffer;
mod image_buffer;
mod rectarea;
mod system;
mod tilemap_buffer;

use color_palette::Color;
use graphics::{graphics, init_graphics};
use graphics_buffer::GraphicsBuffer;
use system::{init_system, system};

//
// System
//
#[inline]
pub fn init(width: u32, height: u32) {
    init_system("hoge", width, height);
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
