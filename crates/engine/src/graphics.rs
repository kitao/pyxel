use crate::image::Image;
use crate::palette::Rgb24;
use crate::settings::{DISPLAY_COLORS, IMAGE_COUNT, IMAGE_SIZE, TILEMAP_COUNT, TILEMAP_SIZE};
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

        for _ in 0..IMAGE_COUNT {
            images.push(Image::new(IMAGE_SIZE, IMAGE_SIZE));
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

    pub fn screen(&self) -> &Image {
        &self.screen
    }

    pub fn screen_mut(&mut self) -> &mut Image {
        &mut self.screen
    }

    pub fn image(&self, no: u32) -> &Image {
        if no < self.images.len() as u32 {
            &self.images[no as usize]
        } else {
            &self.screen
        }
    }

    pub fn image_mut(&mut self, no: u32) -> &mut Image {
        if no < self.images.len() as u32 {
            &mut self.images[no as usize]
        } else {
            &mut self.screen
        }
    }

    pub fn tilemap(&self, no: u32) -> &Tilemap {
        &self.tilemaps[no as usize]
    }

    pub fn tilemap_mut(&mut self, no: u32) -> &mut Tilemap {
        &mut self.tilemaps[no as usize]
    }
}
