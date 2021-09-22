use std::sync::Arc;

use parking_lot::Mutex;

use crate::canvas::Canvas;
use crate::image::SharedImage;
use crate::rectarea::RectArea;
use crate::resource::ResourceItem;
use crate::settings::{RESOURCE_ARCHIVE_DIRNAME, TILEMAP_SIZE};
use crate::types::Tile;
use crate::utils::{parse_hex_string, simplify_string};
use crate::Pyxel;

pub struct Tilemap {
    width: u32,
    height: u32,
    self_rect: RectArea,
    clip_rect: RectArea,
    data: Vec<Vec<Tile>>,
    pub image: SharedImage,
}

pub type SharedTilemap = Arc<Mutex<Tilemap>>;

impl Tilemap {
    pub fn new(width: u32, height: u32, image: SharedImage) -> SharedTilemap {
        Arc::new(Mutex::new(Self {
            width,
            height,
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
            data: vec![vec![(0, 0); width as usize]; height as usize],
            image,
        }))
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = simplify_string(data_str[0]).len() as u32 / 4;
        let height = data_str.len() as u32;
        let tilemap = Self::new(width, height, self.image.clone());

        {
            let mut tilemap = tilemap.lock();

            for i in 0..height {
                let src_data = simplify_string(data_str[i as usize]);

                for j in 0..width {
                    let index = j as usize * 4;
                    let value = parse_hex_string(&src_data[index..index + 4]).unwrap();

                    tilemap._set_value(
                        j as i32,
                        i as i32,
                        (((value >> 8) & 0xff) as u8, (value & 0xff) as u8),
                    );
                }
            }
        }

        self.blt(
            x as f64,
            y as f64,
            tilemap,
            0.0,
            0.0,
            width as f64,
            height as f64,
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

    fn _value(&self, x: i32, y: i32) -> Tile {
        self.data[y as usize][x as usize]
    }

    fn _set_value(&mut self, x: i32, y: i32, value: Tile) {
        self.data[y as usize][x as usize] = value;
    }

    fn _self_rect(&self) -> RectArea {
        self.self_rect
    }

    fn _clip_rect(&self) -> RectArea {
        self.clip_rect
    }

    fn _set_clip_rect(&mut self, clip_rect: RectArea) {
        self.clip_rect = clip_rect;
    }

    fn _palette_value(&self, value: Tile) -> Tile {
        value
    }
}

impl ResourceItem for Tilemap {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "tilemap" + &item_no.to_string()
    }

    fn is_modified(&self) -> bool {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self._value(j as i32, i as i32) != (0, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn clear(&mut self) {
        self.cls((0, 0));
    }

    fn serialize(&self, pyxel: &Pyxel) -> String {
        let mut output = String::new();

        for i in 0..self.height() {
            for j in 0..self.width() {
                let tile = self._value(j as i32, i as i32);

                output += &format!("{:02x}{:02x}", tile.0, tile.1);
            }

            output += "\n";
        }

        output += &format!("{}", pyxel.image_no(self.image.clone()).unwrap_or(0));

        output
    }

    fn deserialize(&mut self, pyxel: &Pyxel, version: u32, input: &str) {
        for (i, line) in input.lines().enumerate() {
            if i < TILEMAP_SIZE as usize {
                if version < 15000 {
                    string_loop!(j, value, line, 3, {
                        let value = parse_hex_string(&value).unwrap();
                        let x = value % 32;
                        let y = value / 32;

                        self._set_value(j as i32, i as i32, (x as u8, y as u8));
                    });
                } else {
                    string_loop!(j, value, line, 4, {
                        let x = parse_hex_string(&value[0..2].to_string()).unwrap();
                        let y = parse_hex_string(&value[2..4].to_string()).unwrap();

                        self._set_value(j as i32, i as i32, (x as u8, y as u8));
                    });
                }
            } else {
                self.image = pyxel.image(line.parse().unwrap());
            }
        }
    }
}
