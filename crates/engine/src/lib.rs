pub mod color_palette;
pub mod graphics;
pub mod graphics_buffer;
pub mod image_buffer;
pub mod rectarea;
pub mod system;
pub mod tilemap_buffer;

use graphics::Graphics;
use system::System;

pub struct Pyxel {
    system: System,
    graphics: Graphics,
}

static mut PYXEL: Option<Pyxel> = None;

#[inline]
fn pyxel() -> &'static mut Pyxel {
    unsafe { PYXEL.as_mut().expect("Pyxel is not initialized") }
}

//
//  System
//
pub fn init(width: u32, height: u32) {
    let system = System::new("hoge", width, height);
    let graphics = Graphics::new(width, height);

    let pyxel = Pyxel {
        system: system,
        graphics: graphics,
    };

    unsafe {
        PYXEL = Some(pyxel);
    }
}

pub fn run() {
    pyxel().system.run();
}

//
//  Graphics
//
