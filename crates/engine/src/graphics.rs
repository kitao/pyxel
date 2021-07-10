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
        let images = (0..IMAGE_COUNT)
            .map(|_| Image::new(IMAGE_SIZE, IMAGE_SIZE))
            .collect();
        let tilemaps = (0..TILEMAP_COUNT)
            .map(|_| Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE))
            .collect();

        screen
            .palette_mut()
            .set_display_colors(colors.unwrap_or(&DISPLAY_COLORS));

        Graphics {
            screen: screen,
            images: images,
            tilemaps: tilemaps,
        }
    }

    pub fn screen(&self) -> &Image {
        &self.screen
    }

    pub fn screen_mut(&mut self) -> &mut Image {
        &mut self.screen
    }

    pub fn image(&self, image_no: u32) -> &Image {
        if image_no < self.images.len() as u32 {
            &self.images[image_no as usize]
        } else {
            &self.screen
        }
    }

    pub fn image_mut(&mut self, image_no: u32) -> &mut Image {
        if image_no < self.images.len() as u32 {
            &mut self.images[image_no as usize]
        } else {
            &mut self.screen
        }
    }

    pub fn tilemap(&self, tilemap_no: u32) -> &Tilemap {
        &self.tilemaps[tilemap_no as usize]
    }

    pub fn tilemap_mut(&mut self, tilemap_no: u32) -> &mut Tilemap {
        &mut self.tilemaps[tilemap_no as usize]
    }
}
