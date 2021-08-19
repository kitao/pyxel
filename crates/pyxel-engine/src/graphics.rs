use array_macro::array;
use parking_lot::Mutex;
use std::sync::Arc;

use crate::canvas::Canvas;
use crate::image::Image;
use crate::settings::{
    COLOR_COUNT, CURSOR_DATA, CURSOR_HEIGHT, CURSOR_WIDTH, FONT_COLOR, FONT_DATA, FONT_HEIGHT,
    FONT_ROW_COUNT, FONT_WIDTH, IMAGE_COUNT, IMAGE_SIZE, TILEMAP_COUNT, TILEMAP_SIZE,
};
use crate::tilemap::Tilemap;
use crate::types::{Color, Tile};
use crate::Pyxel;

pub struct Graphics {
    images: [Arc<Mutex<Image>>; IMAGE_COUNT as usize],
    tilemaps: [Arc<Mutex<Tilemap>>; TILEMAP_COUNT as usize],
}

impl Graphics {
    pub fn new() -> Graphics {
        let images =
            array![_ => Image::with_arc_mutex(IMAGE_SIZE, IMAGE_SIZE); IMAGE_COUNT as usize];
        let tilemaps = array![_ => Tilemap::with_arc_mutex(TILEMAP_SIZE, TILEMAP_SIZE); TILEMAP_COUNT as usize];

        Graphics {
            images: images,
            tilemaps: tilemaps,
        }
    }

    pub fn new_cursor_image() -> Arc<Mutex<Image>> {
        let image = Image::with_arc_mutex(CURSOR_WIDTH, CURSOR_HEIGHT);
        image.lock().set(0, 0, &CURSOR_DATA);

        image
    }

    pub fn new_font_image() -> Arc<Mutex<Image>> {
        let width = FONT_WIDTH * FONT_ROW_COUNT;
        let height = FONT_HEIGHT * ((FONT_DATA.len() as u32 + FONT_ROW_COUNT - 1) / FONT_ROW_COUNT);
        let mut image = Image::new(width, height);

        for (i, data) in FONT_DATA.iter().enumerate() {
            let row = i as u32 / FONT_ROW_COUNT;
            let col = i as u32 % FONT_ROW_COUNT;

            for j in 0..FONT_HEIGHT {
                let mut data = *data;

                for k in 0..FONT_WIDTH {
                    let color = if (data & 0x800000) != 0 {
                        FONT_COLOR
                    } else {
                        0
                    };

                    image._set_value(
                        (FONT_WIDTH * col + k) as i32,
                        (FONT_HEIGHT * row + j) as i32,
                        color,
                    );

                    data <<= 1;
                }
            }
        }

        Arc::new(Mutex::new(image))
    }
}

impl Pyxel {
    pub fn image(&self, image_no: u32) -> Arc<Mutex<Image>> {
        self.graphics.images[image_no as usize].clone()
    }

    pub fn tilemap(&self, image_no: u32) -> Arc<Mutex<Tilemap>> {
        self.graphics.tilemaps[image_no as usize].clone()
    }

    pub fn clip(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.screen.lock().clip(x, y, width, height);
    }

    pub fn clip0(&mut self) {
        self.screen.lock().clip0();
    }

    pub fn pal(&mut self, src_color: Color, dst_color: Color) {
        self.palette[src_color as usize] = dst_color;
    }

    pub fn pal0(&mut self) {
        for i in 0..COLOR_COUNT {
            self.palette[i as usize] = i as Color;
        }
    }

    pub fn cls(&mut self, color: Color) {
        let color = self.palette[color as usize];

        self.screen.lock().cls(color);
    }

    pub fn pget(&mut self, x: i32, y: i32) -> Color {
        self.screen.lock().pget(x, y)
    }

    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        let color = self.palette[color as usize];

        self.screen.lock().pset(x, y, color);
    }

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        let color = self.palette[color as usize];

        self.screen.lock().line(x1, y1, x2, y2, color);
    }

    pub fn rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        let color = self.palette[color as usize];

        self.screen.lock().rect(x, y, width, height, color);
    }

    pub fn rectb(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        let color = self.palette[color as usize];

        self.screen.lock().rectb(x, y, width, height, color);
    }

    pub fn circ(&mut self, x: i32, y: i32, radius: u32, color: Color) {
        let color = self.palette[color as usize];

        self.screen.lock().circ(x, y, radius, color);
    }

    pub fn circb(&mut self, x: i32, y: i32, radius: u32, color: Color) {
        let color = self.palette[color as usize];

        self.screen.lock().circb(x, y, radius, color);
    }

    pub fn tri(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        let color = self.palette[color as usize];

        self.screen.lock().tri(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn trib(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        let color = self.palette[color as usize];

        self.screen.lock().trib(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn blt(
        &mut self,
        x: i32,
        y: i32,
        image_no: u32,
        image_x: i32,
        image_y: i32,
        width: i32,
        height: i32,
        color_key: Option<Color>,
    ) {
        self.screen.lock().blt(
            x,
            y,
            &self.graphics.images[image_no as usize].lock(),
            image_x,
            image_y,
            width,
            height,
            color_key,
            Some(&self.palette),
        );
    }

    pub fn bltm(
        &mut self,
        x: i32,
        y: i32,
        tilemap_no: u32,
        tilemap_x: i32,
        tilemap_y: i32,
        width: i32,
        height: i32,
        tile_key: Option<Tile>,
    ) {
        self.screen.lock().bltm(
            x,
            y,
            &self.graphics.tilemaps[tilemap_no as usize].lock(),
            tilemap_x,
            tilemap_y,
            width,
            height,
            tile_key,
        );
    }

    pub fn text(&mut self, x: i32, y: i32, string: &str, color: Color) {
        let color = self.palette[color as usize];

        self.screen
            .lock()
            .text(x, y, string, color, &self.font.lock());
    }
}
