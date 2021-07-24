use crate::canvas::Canvas;
use crate::rectarea::RectArea;
use crate::utility::{parse_hex_string, simplify_string};

pub type Tile = (u8, u8);

pub struct Tilemap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Vec<Tile>>,

    self_rect: RectArea,
    clip_rect: RectArea,
}

impl Tilemap {
    pub fn new(width: u32, height: u32) -> Tilemap {
        Tilemap {
            width: width,
            height: height,
            data: vec![vec![(0, 0); width as usize]; height as usize],

            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
        }
    }

    pub fn set(&mut self, x: i32, y: i32, data: &[&str]) {
        let width = data[0].len() as u32 / 4;
        let height = data.len() as u32;

        if width == 0 || height == 0 {
            return;
        }

        let mut tilemap = Tilemap::new(width, height);

        for i in 0..height {
            let data = simplify_string(data[i as usize]);

            for j in 0..width {
                let index = j as usize * 4;

                if let Some(value) = parse_hex_string(&data[index..index + 4]) {
                    tilemap.data[i as usize][j as usize] =
                        (((value >> 16) & 0xff) as u8, (value & 0xff) as u8);
                } else {
                    panic!("invalid tilemap data");
                }
            }
        }

        self.blt(x, y, &tilemap, 0, 0, width as i32, height as i32, None);
    }
}

impl Canvas<Tile> for Tilemap {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn data<'a>(&'a self) -> &'a Vec<Vec<Tile>> {
        &self.data
    }

    fn data_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<Tile>> {
        &mut self.data
    }

    fn _self_rect(&self) -> RectArea {
        self.self_rect
    }

    fn _clip_rect(&self) -> RectArea {
        self.clip_rect
    }

    fn _clip_rect_mut(&mut self) -> &mut RectArea {
        &mut self.clip_rect
    }

    fn _palette_value(&self, val: Tile) -> Tile {
        val
    }
}
