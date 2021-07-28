use std::cell::RefCell;
use std::rc::Rc;

use crate::canvas::Canvas;
use crate::rectarea::RectArea;
use crate::utility::{parse_hex_string, set_data_value, simplify_string};

pub type Tile = (u8, u8);

pub struct Tilemap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Vec<Tile>>,

    self_rect: RectArea,
    clip_rect: RectArea,
}

pub type SharedTilemap = Rc<RefCell<Tilemap>>;

impl Tilemap {
    pub fn new(width: u32, height: u32) -> SharedTilemap {
        Rc::new(RefCell::new(Tilemap {
            width: width,
            height: height,
            data: vec![vec![(0, 0); width as usize]; height as usize],

            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
        }))
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = data_str[0].len() as u32 / 4;
        let height = data_str.len() as u32;
        let dst_tilemap = Tilemap::new(width, height);

        {
            let dst_data = &mut dst_tilemap.borrow_mut().data;

            for i in 0..height {
                let src_data = simplify_string(data_str[i as usize]);

                for j in 0..width {
                    let index = j as usize * 4;

                    if let Some(value) = parse_hex_string(&src_data[index..index + 4]) {
                        set_data_value(
                            dst_data,
                            j as i32,
                            i as i32,
                            (((value >> 16) & 0xff) as u8, (value & 0xff) as u8),
                        );
                    } else {
                        panic!("invalid tilemap data");
                    }
                }
            }
        }

        self.blt(
            x,
            y,
            &dst_tilemap.borrow(),
            0,
            0,
            width as i32,
            height as i32,
            None,
            None,
        );
    }
}

impl Canvas<Tile> for Tilemap {
    #[inline]
    fn _width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn _height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn _data<'a>(&'a self) -> &'a Vec<Vec<Tile>> {
        &self.data
    }

    #[inline]
    fn _data_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<Tile>> {
        &mut self.data
    }

    #[inline]
    fn _self_rect(&self) -> RectArea {
        self.self_rect
    }

    #[inline]
    fn _clip_rect(&self) -> RectArea {
        self.clip_rect
    }

    #[inline]
    fn _clip_rect_mut(&mut self) -> &mut RectArea {
        &mut self.clip_rect
    }
}
