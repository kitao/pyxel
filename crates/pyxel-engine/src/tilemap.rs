use parking_lot::Mutex;
use std::sync::Arc;

use crate::canvas::Canvas;
use crate::rectarea::RectArea;
use crate::types::Tile;
use crate::utility::{parse_hex_string, simplify_string};

pub struct Tilemap {
    width: u32,
    height: u32,
    self_rect: RectArea,
    clip_rect: RectArea,
    data: Vec<Vec<Tile>>,
}

impl Tilemap {
    pub fn new(width: u32, height: u32) -> Tilemap {
        Tilemap {
            width: width,
            height: height,
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
            data: vec![vec![(0, 0); width as usize]; height as usize],
        }
    }

    pub fn with_arc_mutex(width: u32, height: u32) -> Arc<Mutex<Tilemap>> {
        Arc::new(Mutex::new(Tilemap::new(width, height)))
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = data_str[0].len() as u32 / 4;
        let height = data_str.len() as u32;
        let mut dst_tilemap = Tilemap::new(width, height);

        for i in 0..height {
            let src_data = simplify_string(data_str[i as usize]);

            for j in 0..width {
                let index = j as usize * 4;

                if let Some(value) = parse_hex_string(&src_data[index..index + 4]) {
                    dst_tilemap.set_value(
                        j as i32,
                        i as i32,
                        (((value >> 16) & 0xff) as u8, (value & 0xff) as u8),
                    );
                } else {
                    panic!("invalid tilemap data");
                }
            }
        }

        self.blt(
            x,
            y,
            &dst_tilemap,
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
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn value(&self, x: i32, y: i32) -> Tile {
        self.data[y as usize][x as usize]
    }

    fn set_value(&mut self, x: i32, y: i32, tile: Tile) {
        self.data[y as usize][x as usize] = tile;
    }

    fn self_rect(&self) -> RectArea {
        self.self_rect
    }

    fn clip_rect(&self) -> RectArea {
        self.clip_rect
    }

    fn clip(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.clip_rect = self
            .self_rect
            .intersects(RectArea::new(x, y, width, height));
    }

    fn clip0(&mut self) {
        self.clip_rect = self.self_rect;
    }
}
