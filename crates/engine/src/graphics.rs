use crate::image::Image;
use crate::palette::Rgb24;
use crate::settings::{
    DISPLAY_COLORS, IMAGEBANK_COUNT, IMAGEBANK_SIZE, TILEMAP_COUNT, TILEMAP_SIZE,
};
use crate::tilemap::Tilemap;

pub struct Graphics {
    screen: Image,
    images: Vec<Image>,
    tilemaps: Vec<Tilemap>,
}

impl Graphics {
    pub fn new(width: u32, height: u32, colors: Option<&[Rgb24]>) -> Graphics {
        let mut screen = Image::new(width, height);
        let mut images = Vec::new();
        let mut tilemaps = Vec::new();

        screen
            .palette_mut()
            .set_display_colors(colors.unwrap_or(&DISPLAY_COLORS));

        for _ in 0..IMAGEBANK_COUNT {
            images.push(Image::new(IMAGEBANK_SIZE, IMAGEBANK_SIZE));
        }

        for _ in 0..TILEMAP_COUNT {
            tilemaps.push(Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE));
        }

        let graphics = Graphics {
            screen: screen,
            images: images,
            tilemaps: tilemaps,
        };

        graphics
    }

    #[inline]
    pub fn screen(&self) -> &Image {
        &self.screen
    }

    #[inline]
    pub fn screen_mut(&mut self) -> &mut Image {
        &mut self.screen
    }

    #[inline]
    pub fn image(&self, no: usize) -> &Image {
        if no < self.images.len() {
            &self.images[no]
        } else {
            &self.screen
        }
    }

    #[inline]
    pub fn image_mut(&mut self, no: usize) -> &mut Image {
        if no < self.images.len() {
            &mut self.images[no]
        } else {
            &mut self.screen
        }
    }

    #[inline]
    pub fn tilemap(&self, no: usize) -> &Tilemap {
        &self.tilemaps[no]
    }

    #[inline]
    pub fn tilemap_mut(&mut self, no: usize) -> &mut Tilemap {
        &mut self.tilemaps[no]
    }
}
