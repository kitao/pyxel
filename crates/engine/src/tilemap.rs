use parking_lot::Mutex;
use std::sync::Arc;

use crate::canvas::Canvas;
use crate::image::SharedImage;
use crate::rectarea::RectArea;
use crate::settings::RESOURCE_ARCHIVE_DIRNAME;
use crate::types::Tile;
use crate::utils::{parse_hex_string, pyxel_version, simplify_string};

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
        Arc::new(Mutex::new(Tilemap::without_arc_mutex(width, height, image)))
    }

    pub fn without_arc_mutex(width: u32, height: u32, image: SharedImage) -> Tilemap {
        Tilemap {
            width: width,
            height: height,
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
            data: vec![vec![(0, 0); width as usize]; height as usize],
            image: image,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = simplify_string(data_str[0]).len() as u32 / 4;
        let height = data_str.len() as u32;
        let mut tilemap = Tilemap::without_arc_mutex(width, height, self.image.clone());

        for i in 0..height {
            let src_data = simplify_string(data_str[i as usize]);

            for j in 0..width {
                let index = j as usize * 4;

                if let Some(value) = parse_hex_string(&src_data[index..index + 4]) {
                    tilemap._set_value(
                        j as i32,
                        i as i32,
                        (((value >> 8) & 0xff) as u8, (value & 0xff) as u8),
                    );
                } else {
                    panic!("invalid tilemap data");
                }
            }
        }

        self.blt(x, y, &tilemap, 0, 0, width as i32, height as i32, None);
    }

    pub fn resource_name(tilemap_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "tilemap" + &tilemap_no.to_string()
    }

    pub(crate) fn clear(&mut self) {
        self.cls((0, 0));
    }

    pub(crate) fn serialize(&self) -> String {
        /*
        Tilemap* tilemap = graphics_->GetTilemapBank(tilemap_index);
        int32_t** data = tilemap->Data();
        bool is_editted = false;

        for (int32_t i = 0; i < tilemap->Height(); i++) {
          for (int32_t j = 0; j < tilemap->Width(); j++) {
            if (data[i][j] != 0) {
              is_editted = true;
              break;
            }
          }

          if (is_editted) {
            break;
          }
        }

        if (!is_editted) {
          return "";
        }

        std::stringstream ss;

        ss << std::hex;

        for (int32_t i = 0; i < tilemap->Height(); i++) {
          for (int32_t j = 0; j < tilemap->Width(); j++) {
            ss << std::setw(3) << std::setfill('0') << data[i][j];
          }

          ss << std::endl;
        }

        ss << std::dec << tilemap->ImageIndex() << std::endl;

        return ss.str();
        */
        "TODO".to_string()
    }

    pub(crate) fn deserialize(&mut self, input: &str) {
        if pyxel_version() < 15000 {
            for (i, line) in input.lines().enumerate() {
                for j in 0..(line.len() / 3) {
                    let value = parse_hex_string(&line[j * 3..j * 3 + 3].to_string()).unwrap();
                    let x = value % 32;
                    let y = value / 32;

                    self._set_value(j as i32, i as i32, (x as u8, y as u8));
                }
            }
        } else {
            for (i, line) in input.lines().enumerate() {
                for j in 0..(line.len() / 4) {
                    let x = parse_hex_string(&line[j * 4..j * 4 + 2].to_string()).unwrap();
                    let y = parse_hex_string(&line[j * 4 + 2..j * 4 + 4].to_string()).unwrap();

                    self._set_value(j as i32, i as i32, (x as u8, y as u8));
                }
            }
        }
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
