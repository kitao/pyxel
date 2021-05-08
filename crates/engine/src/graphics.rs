use crate::image::Image;
use crate::palette::{Palette, Rgb24};
use crate::settings::{
    DISPLAY_COLORS, IMAGEBANK_COUNT, IMAGEBANK_SIZE, TILEMAP_COUNT, TILEMAP_SIZE,
};
use crate::tilemap::Tilemap;

static mut INSTANCE: Option<Graphics> = None;

#[inline]
pub fn graphics() -> &'static mut Graphics {
    unsafe { INSTANCE.as_mut().expect("Graphics is not initialized") }
}

pub fn init_graphics(width: u32, height: u32, colors: Option<&[Rgb24]>) {
    unsafe {
        assert!(INSTANCE.is_none(), "Graphics is already initialized");

        INSTANCE = Some(Graphics::new(width, height, colors));
    }
}

pub struct Graphics {
    screen: Image,
    images: Vec<Image>,
    tilemaps: Vec<Tilemap>,
}

impl Graphics {
    pub fn new(width: u32, height: u32, colors: Option<&[Rgb24]>) -> Graphics {
        let screen = Image::new(width, height);
        let mut images = Vec::new();
        let mut tilemaps = Vec::new();

        for _ in 0..IMAGEBANK_COUNT {
            images.push(Image::new(IMAGEBANK_SIZE, IMAGEBANK_SIZE));
        }

        for _ in 0..TILEMAP_COUNT {
            tilemaps.push(Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE));
        }

        let mut graphics = Graphics {
            screen: screen,
            images: images,
            tilemaps: tilemaps,
        };

        match colors {
            Some(cols) => graphics.palette().set_display_colors(&cols),
            None => graphics.palette().set_display_colors(&DISPLAY_COLORS),
        }

        graphics
    }

    #[inline]
    pub fn screen(&mut self) -> &mut Image {
        &mut self.screen
    }

    #[inline]
    pub fn palette(&mut self) -> &mut Palette {
        self.screen.palette_mut()
    }

    #[inline]
    pub fn image(&mut self, no: usize) -> &mut Image {
        if no < self.images.len() {
            &mut self.images[no]
        } else {
            &mut self.screen
        }
    }

    #[inline]
    pub fn tilemap(&mut self, no: usize) -> &mut Tilemap {
        &mut self.tilemaps[no]
    }
}
