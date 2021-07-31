use crate::canvas::Canvas;
use crate::image::{Image, SharedImage};
use crate::settings::COLOR_COUNT;
use crate::types::{Color, Tile};
use crate::Pyxel;

pub struct Graphics {
    org_palette: [Color; COLOR_COUNT as usize],
    cur_palette: [Color; COLOR_COUNT as usize],
}

impl Graphics {
    pub fn new() -> Graphics {
        let mut org_palette: [Color; COLOR_COUNT as usize] = [0; COLOR_COUNT as usize];
        for i in 0..COLOR_COUNT {
            org_palette[i as usize] = i as Color;
        }

        let mut cur_palette: [Color; COLOR_COUNT as usize] = [0; COLOR_COUNT as usize];
        cur_palette.clone_from_slice(&org_palette);

        Graphics {
            org_palette: org_palette,
            cur_palette: cur_palette,
        }
    }

    pub fn new_cursor_image() -> SharedImage {
        Image::new(10, 10)
    }

    pub fn new_font_image() -> SharedImage {
        // TOTO
        Image::new(10, 10)
    }
}

impl Pyxel {
    pub fn clip(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.screen.borrow_mut().clip(x, y, width, height);
    }

    pub fn clip_(&mut self) {
        self.screen.borrow_mut().clip_();
    }

    pub fn pal(&mut self, src_color: Color, dst_color: Color) {
        self.graphics.cur_palette[src_color as usize] = dst_color;
    }

    pub fn pal_(&mut self) {
        self.graphics
            .cur_palette
            .clone_from_slice(&self.graphics.org_palette);
    }

    pub fn cls(&mut self, color: Color) {
        self.screen.borrow_mut().cls(color);
    }

    pub fn pget(&mut self, x: i32, y: i32) -> Color {
        self.screen.borrow_mut().pget(x, y)
    }

    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        self.screen.borrow_mut().pset(x, y, color);
    }

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        self.screen.borrow_mut().line(x1, y1, x2, y2, color);
    }

    pub fn rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.screen.borrow_mut().rect(x, y, width, height, color);
    }

    pub fn rectb(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.screen.borrow_mut().rectb(x, y, width, height, color);
    }

    pub fn circ(&mut self, x: i32, y: i32, radius: u32, color: Color) {
        self.screen.borrow_mut().circ(x, y, radius, color);
    }

    pub fn circb(&mut self, x: i32, y: i32, radius: u32, color: Color) {
        self.screen.borrow_mut().circb(x, y, radius, color);
    }

    pub fn tri(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        self.screen.borrow_mut().tri(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn trib(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        self.screen.borrow_mut().trib(x1, y1, x2, y2, x3, y3, color);
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
        self.screen.borrow_mut().blt(
            x,
            y,
            &self.images[image_no as usize].borrow(),
            image_x,
            image_y,
            width,
            height,
            color_key,
            Some(&self.graphics.cur_palette),
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
        self.screen.borrow_mut().bltm(
            x,
            y,
            &self.tilemaps[tilemap_no as usize].borrow(),
            tilemap_x,
            tilemap_y,
            width,
            height,
            tile_key,
        );
    }

    pub fn text(&mut self, x: i32, y: i32, string: &str, color: Color) {
        self.screen
            .borrow_mut()
            .text(x, y, string, color, &self.font.borrow());
    }
}
