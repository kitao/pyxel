use std::collections::HashMap;
use std::path::Path;

use array_macro::array;
use image::imageops::{self, FilterType};
use image::{Rgb, RgbImage};

use crate::canvas::{Canvas, CopyArea, ToIndex};
use crate::rectarea::RectArea;
use crate::resource::ResourceItem;
use crate::settings::{
    FONT_HEIGHT, FONT_WIDTH, MAX_FONT_CODE, MIN_FONT_CODE, NUM_COLORS, NUM_FONT_ROWS,
    RESOURCE_ARCHIVE_DIRNAME, TILE_SIZE,
};
use crate::tilemap::SharedTilemap;
use crate::types::{Color, Rgb8};
use crate::utils::{add_file_extension, as_i32, as_u32, parse_hex_string, simplify_string};
use crate::Pyxel;

impl ToIndex for Color {
    fn to_index(&self) -> usize {
        *self as usize
    }
}

pub struct Image {
    pub(crate) canvas: Canvas<Color>,
    pub(crate) palette: [Color; NUM_COLORS as usize],
}

pub type SharedImage = shared_type!(Image);

impl Image {
    pub fn new(width: u32, height: u32) -> SharedImage {
        new_shared_type!(Self {
            canvas: Canvas::new(width, height),
            palette: array![i => i as Color; NUM_COLORS as usize],
        })
    }

    pub fn from_image(filename: &str, colors: &[Rgb8]) -> SharedImage {
        let image_file = image::open(&Path::new(&filename))
            .unwrap_or_else(|_| panic!("Unable to open file '{}'", filename))
            .to_rgb8();
        let (width, height) = image_file.dimensions();
        let image = Self::new(width, height);
        {
            let mut image = image.lock();
            let mut color_table = HashMap::<(u8, u8, u8), Color>::new();
            for y in 0..height {
                for x in 0..width {
                    let p = image_file.get_pixel(x, y);
                    let src_rgb = (p[0], p[1], p[2]);
                    if let Some(color) = color_table.get(&src_rgb) {
                        image.canvas.data[y as usize][x as usize] = *color;
                    } else {
                        let mut closest_color: Color = 0;
                        let mut closest_dist: f64 = f64::MAX;
                        for i in 0..NUM_COLORS {
                            let pal_color = colors[i as usize];
                            let pal_rgb = (
                                ((pal_color >> 16) & 0xff) as u8,
                                ((pal_color >> 8) & 0xff) as u8,
                                (pal_color & 0xff) as u8,
                            );
                            let dist = Self::color_dist(src_rgb, pal_rgb);
                            if dist < closest_dist {
                                closest_color = i as Color;
                                closest_dist = dist;
                            }
                        }
                        color_table.insert(src_rgb, closest_color);
                        image.canvas.data[y as usize][x as usize] = closest_color;
                    }
                }
            }
        }
        image
    }

    fn color_dist(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
        let (r1, g1, b1) = rgb1;
        let (r2, g2, b2) = rgb2;
        let dx = (r1 as f64 - r2 as f64) * 0.30;
        let dy = (g1 as f64 - g2 as f64) * 0.59;
        let dz = (b1 as f64 - b2 as f64) * 0.11;
        dx * dx + dy * dy + dz * dz
    }

    pub fn width(&self) -> u32 {
        self.canvas.width()
    }

    pub fn height(&self) -> u32 {
        self.canvas.height()
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = simplify_string(data_str[0]).len() as u32;
        let height = data_str.len() as u32;
        let image = Self::new(width, height);
        {
            let mut image = image.lock();
            for y in 0..height {
                let src_data = simplify_string(data_str[y as usize]);
                for x in 0..width {
                    let color = parse_hex_string(&src_data[x as usize..x as usize + 1]).unwrap();
                    image.canvas.data[y as usize][x as usize] = color as Color;
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

    pub fn save(&self, filename: &str, colors: &[Rgb8], scale: u32) {
        let width = self.width();
        let height = self.height();
        let mut image = RgbImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let rgb = colors[self.canvas.data[y as usize][x as usize] as usize];
                let r = ((rgb >> 16) & 0xff) as u8;
                let g = ((rgb >> 8) & 0xff) as u8;
                let b = (rgb & 0xff) as u8;
                image.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        let image = imageops::resize(&image, width * scale, height * scale, FilterType::Nearest);
        let filename = add_file_extension(filename, ".png");
        image
            .save(&filename)
            .unwrap_or_else(|_| panic!("Unable to open file '{}'", filename));
    }

    pub fn clip(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas.clip(x, y, width, height);
    }

    pub fn clip0(&mut self) {
        self.canvas.clip0();
    }

    pub fn camera(&mut self, x: f64, y: f64) {
        self.canvas.camera(x, y);
    }

    pub fn camera0(&mut self) {
        self.canvas.camera0();
    }

    pub fn pal(&mut self, src_color: Color, dst_color: Color) {
        self.palette[src_color as usize] = dst_color;
    }

    pub fn pal0(&mut self) {
        for i in 0..NUM_COLORS {
            self.palette[i as usize] = i as Color;
        }
    }

    pub fn cls(&mut self, color: Color) {
        self.canvas.cls(self.palette[color as usize]);
    }

    pub fn pget(&mut self, x: f64, y: f64) -> Color {
        self.canvas.pget(x, y)
    }

    pub fn pset(&mut self, x: f64, y: f64, color: Color) {
        self.canvas.pset(x, y, self.palette[color as usize]);
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        self.canvas
            .line(x1, y1, x2, y2, self.palette[color as usize]);
    }

    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.canvas
            .rect(x, y, width, height, self.palette[color as usize]);
    }

    pub fn rectb(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.canvas
            .rectb(x, y, width, height, self.palette[color as usize]);
    }

    pub fn circ(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        self.canvas.circ(x, y, radius, self.palette[color as usize]);
    }

    pub fn circb(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        self.canvas
            .circb(x, y, radius, self.palette[color as usize]);
    }

    pub fn tri(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.canvas
            .tri(x1, y1, x2, y2, x3, y3, self.palette[color as usize]);
    }

    pub fn trib(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.canvas
            .trib(x1, y1, x2, y2, x3, y3, self.palette[color as usize]);
    }

    pub fn blt(
        &mut self,
        x: f64,
        y: f64,
        image: shared_type!(Self),
        image_x: f64,
        image_y: f64,
        width: f64,
        height: f64,
        transparent: Option<Color>,
    ) {
        if let Some(image) = image.try_lock() {
            self.canvas.blt(
                x,
                y,
                &image.canvas,
                image_x,
                image_y,
                width,
                height,
                transparent,
                Some(&self.palette),
            );
        } else {
            let copy_width = as_u32(width.abs());
            let copy_height = as_u32(height.abs());
            let mut canvas = Canvas::new(copy_width, copy_height);
            canvas.blt(
                0.0,
                0.0,
                &self.canvas,
                image_x,
                image_y,
                copy_width as f64,
                copy_height as f64,
                None,
                None,
            );
            self.canvas.blt(
                x,
                y,
                &canvas,
                0.0,
                0.0,
                width,
                height,
                transparent,
                Some(&self.palette),
            );
        }
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
        let x = as_i32(x) - self.canvas.camera_x;
        let y = as_i32(y) - self.canvas.camera_y;
        let tilemap_x = as_i32(tilemap_x);
        let tilemap_y = as_i32(tilemap_y);
        let width = as_i32(width);
        let height = as_i32(height);

        let tilemap = tilemap.lock();
        let tilemap_rect = RectArea::new(
            tilemap.canvas.self_rect.left() * TILE_SIZE as i32,
            tilemap.canvas.self_rect.top() * TILE_SIZE as i32,
            tilemap.canvas.self_rect.width() * TILE_SIZE,
            tilemap.canvas.self_rect.height() * TILE_SIZE,
        );

        let CopyArea {
            dst_x,
            dst_y,
            src_x,
            src_y,
            sign_x,
            sign_y,
            offset_x,
            offset_y,
            width,
            height,
        } = CopyArea::new(
            x,
            y,
            self.canvas.clip_rect,
            tilemap_x,
            tilemap_y,
            tilemap_rect,
            width,
            height,
        );
        if width == 0 || height == 0 {
            return;
        }

        let image = tilemap.image.lock();
        for yi in 0..height {
            for xi in 0..width {
                let tilemap_x = src_x + sign_x * xi + offset_x;
                let tilemap_y = src_y + sign_y * yi + offset_y;

                let tile_x = tilemap_x / TILE_SIZE as i32;
                let tile_y = tilemap_y / TILE_SIZE as i32;
                let tile = tilemap.canvas.data[tile_y as usize][tile_x as usize];

                let value_x = tile.0 as i32 * TILE_SIZE as i32 + tilemap_x % TILE_SIZE as i32;
                let value_y = tile.1 as i32 * TILE_SIZE as i32 + tilemap_y % TILE_SIZE as i32;
                let value = image.canvas.data[value_y as usize][value_x as usize];

                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }
                let value = self.palette[value.to_index()];
                self.canvas.data[(dst_y + yi) as usize][(dst_x + xi) as usize] = value;
            }
        }
    }

    pub fn text(&mut self, x: f64, y: f64, string: &str, color: Color, font: SharedImage) {
        let mut x = as_i32(x);
        let mut y = as_i32(y);
        let color = self.palette[color as usize];
        let palette1 = self.palette[1];
        self.pal(1, color);
        let start_x = x;
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
            let src_x = (code % NUM_FONT_ROWS as i32) * FONT_WIDTH as i32;
            let src_y = (code / NUM_FONT_ROWS as i32) * FONT_HEIGHT as i32;
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

impl ResourceItem for Image {
    fn resource_name(item_no: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "image" + &item_no.to_string()
    }

    fn is_modified(&self) -> bool {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.canvas.data[y as usize][x as usize] != 0 {
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
        for y in 0..self.height() {
            for x in 0..self.width() {
                output += &format!("{:1x}", self.canvas.data[y as usize][x as usize]);
            }
            output += "\n";
        }
        output
    }

    fn deserialize(&mut self, _pyxel: &Pyxel, _version: u32, input: &str) {
        for (i, line) in input.lines().enumerate() {
            string_loop!(j, color, line, 1, {
                self.canvas.data[i][j] = parse_hex_string(&color).unwrap() as Color;
            });
        }
    }
}
