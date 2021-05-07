use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::color_palette::{ColorPalette, Rgb24};
use crate::image_buffer::ImageBuffer;
use crate::tilemap_buffer::TilemapBuffer;

const IMAGE_BUFFER_COUNT: usize = 4;
const IMAGE_BUFFER_SIZE: u32 = 256;

const TILEMAP_BUFFER_COUNT: usize = 8;
const TILEMAP_BUFFER_SIZE: u32 = 256;

const DEFAULT_COLOR_COUNT: usize = 16;
const DEFAULT_DISPLAY_COLORS: [Rgb24; DEFAULT_COLOR_COUNT] = [
    0x000000, 0x2b335f, 0x7e2072, 0x19959c, 0x8b4852, 0x395c98, 0xa9c1ff, 0xeeeeee, 0xd4186c,
    0xd38441, 0xe9c35b, 0x70c6a9, 0x7696de, 0xa3a3a3, 0xFF9798, 0xedc7b0,
];

pub enum DefaultColor {
    Black,
    Navy,
    Purple,
    Green,
    Brown,
    DarkBlue,
    LightBlue,
    White,
    Red,
    Orange,
    Yellow,
    Lime,
    Cyan,
    Gray,
    Pink,
    Peach,
}

static mut INSTANCE: Option<Graphics> = None;

#[inline]
pub fn graphics() -> &'static mut Graphics {
    unsafe { INSTANCE.as_mut().expect("System is not initialized") }
}

pub fn init_graphics(width: u32, height: u32) {
    unsafe {
        INSTANCE = Some(Graphics::new(width, height));
    }
}

pub struct Graphics {
    palette: Rc<RefCell<ColorPalette>>,
    screen: ImageBuffer,
    image_buffers: Vec<ImageBuffer>,
    tilemap_buffers: Vec<TilemapBuffer>,
}

impl Graphics {
    pub fn new(screen_width: u32, screen_height: u32) -> Graphics {
        let palette = Rc::new(RefCell::new(ColorPalette::new()));
        let screen = ImageBuffer::new(screen_width, screen_height, Rc::clone(&palette));
        let mut image_buffers = Vec::new();
        let mut tilemap_buffers = Vec::new();

        for _ in 0..IMAGE_BUFFER_COUNT {
            image_buffers.push(ImageBuffer::new(
                IMAGE_BUFFER_SIZE,
                IMAGE_BUFFER_SIZE,
                Rc::clone(&palette),
            ));
        }

        for _ in 0..TILEMAP_BUFFER_COUNT {
            tilemap_buffers.push(TilemapBuffer::new(TILEMAP_BUFFER_SIZE, TILEMAP_BUFFER_SIZE));
        }

        let graphics = Graphics {
            palette: palette,
            screen: screen,
            image_buffers: image_buffers,
            tilemap_buffers: tilemap_buffers,
        };

        graphics
            .palette()
            .set_display_colors(&DEFAULT_DISPLAY_COLORS);

        graphics
    }

    pub fn screen(&mut self) -> &mut ImageBuffer {
        &mut self.screen
    }

    pub fn image(&mut self, no: usize) -> &mut ImageBuffer {
        if no < self.image_buffers.len() {
            &mut self.image_buffers[no]
        } else {
            &mut self.screen
        }
    }

    pub fn tilemap(&mut self, no: usize) -> &mut TilemapBuffer {
        &mut self.tilemap_buffers[no]
    }

    pub fn palette(&self) -> RefMut<ColorPalette> {
        self.palette.borrow_mut()
    }
}

/*
Image* ScreenImage() const { return image_bank_[IMAGE_BANK_FOR_SCREEN]; }
*/
