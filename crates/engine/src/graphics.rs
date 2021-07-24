use crate::canvas::Canvas;
use crate::image::Image;
use crate::tilemap::Tilemap;
use crate::types::{Color, Tile};

use super::Pyxel;

pub struct Graphics {}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {}
    }

    pub fn new_cursor_image() -> Image {
        // TODO
        Image::new(10, 10)
    }

    pub fn new_font_image() -> Image {
        // TOTO
        Image::new(10, 10)
    }
}

impl Pyxel {
    pub fn clip(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.screen.clip(x, y, w, h);
    }

    pub fn clip_(&mut self) {
        self.screen.clip_();
    }

    pub fn pal(&mut self, col1: Color, col2: Color) {
        self.screen.pal(col1, col2);
    }

    pub fn pal_(&mut self) {
        self.screen.pal_();
    }

    pub fn cls(&mut self, col: Color) {
        self.screen.cls(col);
    }

    pub fn pget(&mut self, x: i32, y: i32) -> Color {
        self.screen.pget(x, y)
    }

    pub fn pset(&mut self, x: i32, y: i32, col: Color) {
        self.screen.pset(x, y, col);
    }

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, col: Color) {
        self.screen.line(x1, y1, x2, y2, col);
    }

    pub fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, col: Color) {
        self.screen.rect(x, y, w, h, col);
    }

    pub fn rectb(&mut self, x: i32, y: i32, w: u32, h: u32, col: Color) {
        self.screen.rectb(x, y, w, h, col);
    }

    pub fn circ(&mut self, x: i32, y: i32, r: u32, col: Color) {
        self.screen.circ(x, y, r, col);
    }

    pub fn circb(&mut self, x: i32, y: i32, r: u32, col: Color) {
        self.screen.circb(x, y, r, col);
    }

    pub fn tri(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: Color) {
        self.screen.tri(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn trib(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: Color) {
        self.screen.trib(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn blt(
        &mut self,
        x: i32,
        y: i32,
        img: u32,
        u: i32,
        v: i32,
        w: i32,
        h: i32,
        colkey: Option<Color>,
    ) {
        self.screen
            .blt(x, y, &self.image[img as usize], u, v, w, h, colkey);
    }

    pub fn blt_(
        &mut self,
        x: i32,
        y: i32,
        img: &Image,
        u: i32,
        v: i32,
        w: i32,
        h: i32,
        colkey: Option<Color>,
    ) {
        self.screen.blt(x, y, img, u, v, w, h, colkey);
    }

    pub fn bltm(
        &mut self,
        x: i32,
        y: i32,
        tm: u32,
        u: i32,
        v: i32,
        w: i32,
        h: i32,
        tilekey: Option<Tile>,
    ) {
        self.screen
            .bltm(x, y, &self.tilemap[tm as usize], u, v, w, h, tilekey);
    }

    pub fn bltm_(
        &mut self,
        x: i32,
        y: i32,
        tm: &Tilemap,
        u: i32,
        v: i32,
        w: i32,
        h: i32,
        tilekey: Option<Tile>,
    ) {
        self.screen.bltm(x, y, tm, u, v, w, h, tilekey);
    }

    pub fn text(&mut self, x: i32, y: i32, s: &str, col: Color) {
        //self.screen.text(x, y, s, col);
    }
}
