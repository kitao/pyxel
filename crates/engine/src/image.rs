use std::collections::HashMap;
use std::path::Path;

use array_macro::array;
use image::imageops::{self, FilterType};
use image::{Rgb, RgbImage};

use crate::canvas::{Canvas, CopyArea};
use crate::rectarea::RectArea;
use crate::resource::ResourceItem;
use crate::settings::{
    COLOR_COUNT, FONT_HEIGHT, FONT_ROW_COUNT, FONT_WIDTH, MAX_FONT_CODE, MIN_FONT_CODE,
    RESOURCE_ARCHIVE_DIRNAME, TILE_SIZE,
};
use crate::tilemap::SharedTilemap;
use crate::types::{Color, Rgb8};
use crate::utils::as_i32;
use crate::utils::{parse_hex_string, simplify_string};
use crate::Pyxel;

pub struct Image {
    self_rect: RectArea,
    clip_rect: RectArea,
    palette: [Color; COLOR_COUNT as usize],
    data: Vec<Vec<Color>>,
}

pub type SharedImage = shared_type!(Image);

impl Image {
    pub fn new(width: u32, height: u32) -> SharedImage {
        new_shared_type!(Self {
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
            palette: array![i => i as Color; COLOR_COUNT as usize],
            data: vec![vec![0; width as usize]; height as usize],
        })
    }

    pub fn from_image(filename: &str, colors: &[Rgb8]) -> SharedImage {
        let image_file = image::open(&Path::new(&filename)).unwrap().to_rgb8();
        let (width, height) = image_file.dimensions();
        let image = Self::new(width, height);

        {
            let mut image = image.lock();
            let mut color_table = HashMap::<(u8, u8, u8), Color>::new();

            for i in 0..height {
                for j in 0..width {
                    let p = image_file.get_pixel(j, i);
                    let src_rgb = (p[0], p[1], p[2]);

                    if let Some(color) = color_table.get(&src_rgb) {
                        image._set_value(j as i32, i as i32, *color);
                    } else {
                        let mut closest_color: Color = 0;
                        let mut closest_dist: f64 = f64::MAX;

                        for k in 0..COLOR_COUNT {
                            let pal_color = colors[k as usize];

                            let pal_rgb = (
                                ((pal_color >> 16) & 0xff) as u8,
                                ((pal_color >> 8) & 0xff) as u8,
                                (pal_color & 0xff) as u8,
                            );

                            let dist = Self::color_dist(src_rgb, pal_rgb);

                            if dist < closest_dist {
                                closest_color = k as Color;
                                closest_dist = dist;
                            }
                        }

                        color_table.insert(src_rgb, closest_color);
                        image._set_value(j as i32, i as i32, closest_color);
                    }
                }
            }
        }

        image
    }

    pub fn _palette(&self) -> &[Color; COLOR_COUNT as usize] {
        &self.palette
    }

    pub fn _set_palette(&mut self, palette: &[Color; COLOR_COUNT as usize]) {
        self.palette = *palette;
    }

    pub fn pal(&mut self, src_color: Color, dst_color: Color) {
        self.palette[src_color as usize] = dst_color;
    }

    pub fn pal0(&mut self) {
        for i in 0..COLOR_COUNT {
            self.palette[i as usize] = i as Color;
        }
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = simplify_string(data_str[0]).len() as u32;
        let height = data_str.len() as u32;
        let image = Self::new(width, height);

        {
            let mut image = image.lock();

            for i in 0..height {
                let src_data = simplify_string(data_str[i as usize]);

                for j in 0..width {
                    let value = parse_hex_string(&src_data[j as usize..j as usize + 1]).unwrap();

                    image._set_value(j as i32, i as i32, value as Color);
                }
            }
        }

        self.blt(
            x as f64,
            y as f64,
            image,
            0.0,
            0.0,
            width as f64,
            height as f64,
            None,
        );
    }

    pub fn load(&mut self, x: i32, y: i32, filename: &str, colors: &[Rgb8]) {
        let image = Self::from_image(filename, colors);
        let width = image.lock().width();
        let height = image.lock().height();

        self.blt(
            x as f64,
            y as f64,
            image,
            0.0,
            0.0,
            width as f64,
            height as f64,
            None,
        );
    }

    fn color_dist(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
        let (r1, g1, b1) = rgb1;
        let (r2, g2, b2) = rgb2;
        let dx = (r1 as f64 - r2 as f64) * 0.30;
        let dy = (g1 as f64 - g2 as f64) * 0.59;
        let dz = (b1 as f64 - b2 as f64) * 0.11;

        dx * dx + dy * dy + dz * dz
    }

    pub fn save(&self, filename: &str, colors: &[Rgb8], scale: u32) {
        let width = self.width();
        let height = self.height();
        let mut image = RgbImage::new(width, height);

        for i in 0..height {
            for j in 0..width {
                let rgb = colors[self._value(j as i32, i as i32) as usize];
                let r = ((rgb >> 16) & 0xff) as u8;
                let g = ((rgb >> 8) & 0xff) as u8;
                let b = (rgb & 0xff) as u8;

                image.put_pixel(j, i, Rgb([r, g, b]));
            }
        }

        let image = imageops::resize(&image, width * scale, height * scale, FilterType::Nearest);
        let filename = if filename.to_lowercase().ends_with(".png") {
            filename.to_string()
        } else {
            filename.to_string() + ".png"
        };

        image.save(&filename).unwrap();
    }

    pub fn bltm(
        &mut self,
        x: f64,
        y: f64,
        tilemap: SharedTilemap,
        tilemap_x: f64,
        tilemap_y: f64,
        width: f64,
        height: f64,
        transparent: Option<Color>,
    ) {
        let tilemap = if let Some(tilemap) = tilemap.try_lock() {
            tilemap
        } else {
            panic!("unable to lock tilemap in bltm");
        };

        let x = as_i32(x);
        let y = as_i32(y);
        let tilemap_x = as_i32(tilemap_x);
        let tilemap_y = as_i32(tilemap_y);
        let width = as_i32(width.round());
        let height = as_i32(height.round());

        let tile_size = if width < 0 {
            -(TILE_SIZE as i32)
        } else {
            TILE_SIZE as i32
        };

        let left = self.clip_rect.left() / TILE_SIZE as i32;
        let top = self.clip_rect.top() / TILE_SIZE as i32;
        let right = (self.clip_rect.right() + TILE_SIZE as i32 - 1) / TILE_SIZE as i32;
        let bottom = (self.clip_rect.bottom() + TILE_SIZE as i32 - 1) / TILE_SIZE as i32;

        let dst_rect = RectArea::new(
            left,
            top,
            (right - left + 1) as u32,
            (bottom - top + 1) as u32,
        );

        let CopyArea {
            dst_x: _,
            dst_y: _,
            src_x,
            src_y,
            sign_x,
            sign_y,
            offset_x,
            offset_y,
            width,
            height,
        } = CopyArea::new(
            x / TILE_SIZE as i32,
            y / TILE_SIZE as i32,
            dst_rect,
            tilemap_x,
            tilemap_y,
            tilemap._self_rect(),
            width,
            height,
        );

        if width == 0 || height == 0 {
            return;
        }

        for i in 0..height {
            for j in 0..width {
                let tile =
                    tilemap._value(src_x + sign_x * j + offset_x, src_y + sign_y * i + offset_y);

                self.blt(
                    (x + TILE_SIZE as i32 * j) as f64,
                    (y + TILE_SIZE as i32 * i) as f64,
                    tilemap.image.clone(),
                    (tile.0 as i32 * TILE_SIZE as i32) as f64,
                    (tile.1 as i32 * TILE_SIZE as i32) as f64,
                    tile_size as f64,
                    tile_size as f64,
                    transparent,
                );
            }
        }
    }

    pub fn text(&mut self, x: f64, y: f64, string: &str, color: Color, font: SharedImage) {
        let x = as_i32(x);
        let y = as_i32(y);
        let color = self._palette_value(color);
        let palette1 = self._palette()[1];

        self.pal(1, color);

        let start_x = x;
        let mut x = x;
        let mut y = y;

        for c in string.chars() {
            if c == '\n' {
                x = start_x;
                y += FONT_HEIGHT as i32;

                continue;
            }

            if c < MIN_FONT_CODE || c > MAX_FONT_CODE {
                continue;
            }

            let code = c as i32 - MIN_FONT_CODE as i32;
            let src_x = (code % FONT_ROW_COUNT as i32) * FONT_WIDTH as i32;
            let src_y = (code / FONT_ROW_COUNT as i32) * FONT_HEIGHT as i32;

            self.blt(
                x as f64,
                y as f64,
                font.clone(),
                src_x as f64,
                src_y as f64,
                FONT_WIDTH as f64,
                FONT_HEIGHT as f64,
                Some(0),
            );

            x += FONT_WIDTH as i32;
        }

        self.pal(1, palette1);
    }
}

impl Canvas<Color> for Image {
    fn width(&self) -> u32 {
        self.self_rect.width()
    }

    fn height(&self) -> u32 {
        self.self_rect.height()
    }

    fn _value(&self, x: i32, y: i32) -> Color {
        self.data[y as usize][x as usize]
    }

    fn _set_value(&mut self, x: i32, y: i32, value: Color) {
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

    fn _palette_value(&self, value: Color) -> Color {
        self.palette[value as usize]
    }
}

impl ResourceItem for Image {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "image" + &item_no.to_string()
    }

    fn is_modified(&self) -> bool {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self._value(j as i32, i as i32) != 0 {
                    return true;
                }
            }
        }

        false
    }

    fn clear(&mut self) {
        self.cls(0);
    }

    fn serialize(&self, _pyxel: &Pyxel) -> String {
        let mut output = String::new();

        for i in 0..self.height() {
            for j in 0..self.width() {
                output += &format!("{:1x}", self._value(j as i32, i as i32));
            }

            output += "\n";
        }

        output
    }

    fn deserialize(&mut self, _pyxel: &Pyxel, _version: u32, input: &str) {
        for (i, line) in input.lines().enumerate() {
            string_loop!(j, value, line, 1, {
                self._set_value(
                    j as i32,
                    i as i32,
                    parse_hex_string(&value).unwrap() as Color,
                );
            });
        }
    }
}
