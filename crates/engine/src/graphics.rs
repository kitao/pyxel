use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn new_cursor_image() -> Rc<RefCell<Image>> {
        // TODO
        Image::new(10, 10)
    }

    pub fn new_font_image() -> Rc<RefCell<Image>> {
        // TOTO
        Image::new(10, 10)
    }
}

impl Pyxel {
    pub fn clip(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.screen.borrow_mut().clip(x, y, w, h);
    }

    pub fn clip_(&mut self) {
        self.screen.borrow_mut().clip_();
    }

    pub fn pal(&mut self, col1: Color, col2: Color) {
        self.screen.borrow_mut().pal(col1, col2);
    }

    pub fn pal_(&mut self) {
        self.screen.borrow_mut().pal_();
    }

    pub fn cls(&mut self, col: Color) {
        self.screen.borrow_mut().cls(col);
    }

    pub fn pget(&mut self, x: i32, y: i32) -> Color {
        self.screen.borrow_mut().pget(x, y)
    }

    pub fn pset(&mut self, x: i32, y: i32, col: Color) {
        self.screen.borrow_mut().pset(x, y, col);
    }

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, col: Color) {
        self.screen.borrow_mut().line(x1, y1, x2, y2, col);
    }

    pub fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, col: Color) {
        self.screen.borrow_mut().rect(x, y, w, h, col);
    }

    pub fn rectb(&mut self, x: i32, y: i32, w: u32, h: u32, col: Color) {
        self.screen.borrow_mut().rectb(x, y, w, h, col);
    }

    pub fn circ(&mut self, x: i32, y: i32, r: u32, col: Color) {
        self.screen.borrow_mut().circ(x, y, r, col);
    }

    pub fn circb(&mut self, x: i32, y: i32, r: u32, col: Color) {
        self.screen.borrow_mut().circb(x, y, r, col);
    }

    pub fn tri(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: Color) {
        self.screen.borrow_mut().tri(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn trib(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: Color) {
        self.screen.borrow_mut().trib(x1, y1, x2, y2, x3, y3, col);
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
        self.screen.borrow_mut().blt(
            x,
            y,
            &self.images[img as usize].borrow(),
            u,
            v,
            w,
            h,
            colkey,
        );
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
        self.screen.borrow_mut().bltm(
            x,
            y,
            &self.tilemaps[tm as usize].borrow(),
            u,
            v,
            w,
            h,
            tilekey,
        );
    }

    pub fn text(&mut self, x: i32, y: i32, s: &str, col: Color) {
        self.screen
            .borrow_mut()
            .text(x, y, s, col, &self.font.borrow());
    }
}
