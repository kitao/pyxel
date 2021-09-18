use array_macro::array;

use crate::canvas::Canvas;
use crate::image::{Image, SharedImage};
use crate::settings::{
    CURSOR_DATA, CURSOR_HEIGHT, CURSOR_WIDTH, FONT_DATA, FONT_HEIGHT, FONT_ROW_COUNT, FONT_WIDTH,
    IMAGE_COUNT, IMAGE_SIZE, TILEMAP_COUNT, TILEMAP_SIZE,
};
use crate::tilemap::{SharedTilemap, Tilemap};
use crate::types::Color;
use crate::Pyxel;

pub struct Graphics {
    images: [SharedImage; IMAGE_COUNT as usize],
    tilemaps: [SharedTilemap; TILEMAP_COUNT as usize],
}

impl Graphics {
    pub fn new() -> Self {
        let images = array![_ => Image::new(IMAGE_SIZE, IMAGE_SIZE); IMAGE_COUNT as usize];
        let tilemaps = array![_ => Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE, images[0].clone()); TILEMAP_COUNT as usize];
        Self { images, tilemaps }
    }

    pub fn new_cursor_image() -> SharedImage {
        let image = Image::new(CURSOR_WIDTH, CURSOR_HEIGHT);
        image.lock().set(0, 0, &CURSOR_DATA);
        image
    }

    pub fn new_font_image() -> SharedImage {
        let width = FONT_WIDTH * FONT_ROW_COUNT;
        let height = FONT_HEIGHT * ((FONT_DATA.len() as u32 + FONT_ROW_COUNT - 1) / FONT_ROW_COUNT);
        let image = Image::new(width, height);
        {
            let mut image = image.lock();

            for (i, data) in FONT_DATA.iter().enumerate() {
                let row = i as u32 / FONT_ROW_COUNT;
                let col = i as u32 % FONT_ROW_COUNT;
                let mut data = *data;

                for j in 0..FONT_HEIGHT {
                    for k in 0..FONT_WIDTH {
                        image._set_value(
                            (FONT_WIDTH * col + k) as i32,
                            (FONT_HEIGHT * row + j) as i32,
                            if (data & 0x800000) == 0 { 0 } else { 1 },
                        );
                        data <<= 1;
                    }
                }
            }
        }
        image
    }
}

impl Pyxel {
    pub fn image(&self, image_no: u32) -> SharedImage {
        self.graphics.images[image_no as usize].clone()
    }

    pub fn tilemap(&self, image_no: u32) -> SharedTilemap {
        self.graphics.tilemaps[image_no as usize].clone()
    }

    pub fn clip(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.screen.lock().clip(x, y, width, height);
    }

    pub fn clip0(&mut self) {
        self.screen.lock().clip0();
    }

    pub fn pal(&mut self, src_color: Color, dst_color: Color) {
        self.screen.lock().pal(src_color, dst_color);
    }

    pub fn pal0(&mut self) {
        self.screen.lock().pal0();
    }

    pub fn cls(&mut self, color: Color) {
        self.screen.lock().cls(color);
    }

    pub fn pget(&mut self, x: f64, y: f64) -> Color {
        self.screen.lock().pget(x, y)
    }

    pub fn pset(&mut self, x: f64, y: f64, color: Color) {
        self.screen.lock().pset(x, y, color);
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        self.screen.lock().line(x1, y1, x2, y2, color);
    }

    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().rect(x, y, width, height, color);
    }

    pub fn rectb(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().rectb(x, y, width, height, color);
    }

    pub fn circ(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        self.screen.lock().circ(x, y, radius, color);
    }

    pub fn circb(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        self.screen.lock().circb(x, y, radius, color);
    }

    pub fn tri(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.screen.lock().tri(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn trib(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.screen.lock().trib(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn blt(
        &mut self,
        x: f64,
        y: f64,
        image_no: u32,
        image_x: f64,
        image_y: f64,
        width: f64,
        height: f64,
        color_key: Option<Color>,
    ) {
        self.screen.lock().blt(
            x,
            y,
            self.graphics.images[image_no as usize].clone(),
            image_x,
            image_y,
            width,
            height,
            color_key,
        );
    }

    pub fn bltm(
        &mut self,
        x: f64,
        y: f64,
        tilemap_no: u32,
        tilemap_x: f64,
        tilemap_y: f64,
        width: f64,
        height: f64,
        color_key: Option<Color>,
    ) {
        self.screen.lock().bltm(
            x,
            y,
            self.graphics.tilemaps[tilemap_no as usize].clone(),
            tilemap_x,
            tilemap_y,
            width,
            height,
            color_key,
        );
    }

    pub fn text(&mut self, x: f64, y: f64, string: &str, color: Color) {
        self.screen
            .lock()
            .text(x, y, string, color, self.font.clone());
    }
}
