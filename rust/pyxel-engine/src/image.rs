use std::array;
use std::collections::HashMap;
use std::path::Path;

use image::imageops;

use crate::canvas::{Canvas, CopyArea, ToIndex};
use crate::font::SharedFont;
use crate::pyxel::{COLORS, FONT_IMAGE, IMAGES};
use crate::rect_area::RectArea;
use crate::settings::{
    FONT_HEIGHT, FONT_WIDTH, MAX_COLORS, MAX_FONT_CODE, MIN_FONT_CODE, NUM_FONT_ROWS, TILE_SIZE,
};
use crate::tilemap::{ImageSource, SharedTilemap};
use crate::utils;

pub type Rgb24 = u32;
pub type Color = u8;

impl ToIndex for Color {
    fn to_index(&self) -> usize {
        *self as usize
    }
}

pub struct Image {
    pub(crate) canvas: Canvas<Color>,
    pub(crate) palette: [Color; MAX_COLORS as usize],
}

pub type SharedImage = shared_type!(Image);

impl Image {
    pub fn new(width: u32, height: u32) -> SharedImage {
        new_shared_type!(Self {
            canvas: Canvas::new(width, height),
            palette: array::from_fn(|i| i as Color),
        })
    }

    pub fn from_image(filename: &str, include_colors: Option<bool>) -> SharedImage {
        let include_colors = include_colors.unwrap_or(false);
        let mut colors = COLORS.lock();
        if include_colors {
            colors.clear();
        }
        let file = image::open(Path::new(&filename));
        if file.is_err() {
            println!("Failed to open file '{filename}'");
            return Self::new(1, 1);
        }
        let file_image = file.unwrap().to_rgb8();
        let (width, height) = file_image.dimensions();
        let image = Self::new(width, height);
        {
            let mut image = image.lock();
            let mut color_table = HashMap::<(u8, u8, u8), Color>::new();
            for y in 0..height {
                for x in 0..width {
                    let p = file_image.get_pixel(x, y);
                    let src_rgb = (p[0], p[1], p[2]);
                    if let Some(color) = color_table.get(&src_rgb) {
                        image.canvas.write_data(x as usize, y as usize, *color);
                    } else {
                        let mut closest_color: Color = 0;
                        if include_colors {
                            colors.push(
                                (src_rgb.0 as u32) << 16
                                    | (src_rgb.1 as u32) << 8
                                    | src_rgb.2 as u32,
                            );
                            closest_color = colors.len() as Color - 1;
                        } else {
                            let mut closest_dist: f64 = f64::MAX;
                            for (i, pal_color) in colors.iter().enumerate() {
                                let pal_rgb = (
                                    (pal_color >> 16) as u8,
                                    (pal_color >> 8) as u8,
                                    *pal_color as u8,
                                );
                                let dist = Self::color_dist(src_rgb, pal_rgb);
                                if dist < closest_dist {
                                    closest_color = i as Color;
                                    closest_dist = dist;
                                }
                            }
                        }
                        color_table.insert(src_rgb, closest_color);
                        image
                            .canvas
                            .write_data(x as usize, y as usize, closest_color);
                    }
                }
            }
        }
        image
    }

    pub const fn width(&self) -> u32 {
        self.canvas.width()
    }

    pub const fn height(&self) -> u32 {
        self.canvas.height()
    }

    pub fn data_ptr(&mut self) -> *mut Color {
        self.canvas.data_ptr()
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = utils::simplify_string(data_str[0]).len() as u32;
        let height = data_str.len() as u32;
        let image = Self::new(width, height);
        {
            let mut image = image.lock();
            for y in 0..height {
                let src_data = utils::simplify_string(data_str[y as usize]);
                for x in 0..width {
                    let color =
                        utils::parse_hex_string(&src_data[x as usize..=x as usize]).unwrap();
                    image
                        .canvas
                        .write_data(x as usize, y as usize, color as Color);
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
            None,
            None,
        );
    }

    pub fn load(&mut self, x: i32, y: i32, filename: &str, include_colors: Option<bool>) {
        let image = Self::from_image(filename, include_colors);
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
            None,
            None,
        );
    }

    pub fn save(&self, filename: &str, scale: u32) {
        let colors = COLORS.lock();
        let width = self.width();
        let height = self.height();
        let mut image = image::RgbImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let rgb = colors[self.canvas.read_data(x as usize, y as usize) as usize];
                let r = (rgb >> 16) as u8;
                let g = (rgb >> 8) as u8;
                let b = rgb as u8;
                image.put_pixel(x, y, image::Rgb([r, g, b]));
            }
        }
        let image = imageops::resize(
            &image,
            width * scale,
            height * scale,
            imageops::FilterType::Nearest,
        );
        let filename = utils::add_file_extension(filename, ".png");
        image
            .save(&filename)
            .unwrap_or_else(|_| panic!("Failed to open file '{filename}'"));
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
        for i in 0..self.palette.len() {
            self.palette[i] = i as Color;
        }
    }

    pub fn dither(&mut self, alpha: f32) {
        self.canvas.dither(alpha);
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

    pub fn elli(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.canvas
            .elli(x, y, width, height, self.palette[color as usize]);
    }

    pub fn ellib(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.canvas
            .ellib(x, y, width, height, self.palette[color as usize]);
    }

    pub fn tri(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.canvas
            .tri(x1, y1, x2, y2, x3, y3, self.palette[color as usize]);
    }

    pub fn trib(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.canvas
            .trib(x1, y1, x2, y2, x3, y3, self.palette[color as usize]);
    }

    pub fn fill(&mut self, x: f64, y: f64, color: Color) {
        self.canvas.fill(x, y, self.palette[color as usize]);
    }

    pub fn blt(
        &mut self,
        x: f64,
        y: f64,
        image: SharedImage,
        image_x: f64,
        image_y: f64,
        width: f64,
        height: f64,
        transparent: Option<Color>,
        rotate: Option<f64>,
        scale: Option<f64>,
    ) {
        let rotate = rotate.unwrap_or(0.0);
        let scale = scale.unwrap_or(1.0);
        if rotate != 0.0 || scale != 1.0 {
            self.blt_transform(
                x,
                y,
                image,
                image_x,
                image_y,
                width,
                height,
                transparent,
                rotate,
                scale,
            );
            return;
        }

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
            let copy_width = utils::f64_to_u32(width.abs());
            let copy_height = utils::f64_to_u32(height.abs());
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

    fn blt_transform(
        &mut self,
        x: f64,
        y: f64,
        image: SharedImage,
        image_x: f64,
        image_y: f64,
        width: f64,
        height: f64,
        transparent: Option<Color>,
        rotate: f64,
        scale: f64,
    ) {
        if let Some(image) = image.try_lock() {
            self.canvas.blt_transform(
                x,
                y,
                &image.canvas,
                image_x,
                image_y,
                width,
                height,
                transparent,
                Some(&self.palette),
                rotate,
                scale,
                false,
            );
        } else {
            let copy_width = utils::f64_to_u32(width.abs());
            let copy_height = utils::f64_to_u32(height.abs());
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
            self.canvas.blt_transform(
                x,
                y,
                &canvas,
                0.0,
                0.0,
                width,
                height,
                transparent,
                Some(&self.palette),
                rotate,
                scale,
                false,
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
        rotate: Option<f64>,
        scale: Option<f64>,
    ) {
        let rotate = rotate.unwrap_or(0.0);
        let scale = scale.unwrap_or(1.0);
        if rotate != 0.0 || scale != 1.0 {
            self.bltm_transform(
                x,
                y,
                tilemap,
                tilemap_x,
                tilemap_y,
                width,
                height,
                transparent,
                rotate,
                scale,
            );
            return;
        }

        let x = utils::f64_to_i32(x) - self.canvas.camera_x;
        let y = utils::f64_to_i32(y) - self.canvas.camera_y;
        let tilemap_x = utils::f64_to_i32(tilemap_x);
        let tilemap_y = utils::f64_to_i32(tilemap_y);
        let width = utils::f64_to_i32(width);
        let height = utils::f64_to_i32(height);

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

        let images = IMAGES.lock();
        let image = match &tilemap.imgsrc {
            ImageSource::Index(index) => images[*index as usize].lock(),
            ImageSource::Image(image) => image.lock(),
        };
        for yi in 0..height {
            for xi in 0..width {
                let tilemap_x = src_x + sign_x * xi + offset_x;
                let tilemap_y = src_y + sign_y * yi + offset_y;

                let tile_x = tilemap_x / TILE_SIZE as i32;
                let tile_y = tilemap_y / TILE_SIZE as i32;
                let tile = tilemap.canvas.read_data(tile_x as usize, tile_y as usize);

                let value_x = tile.0 as i32 * TILE_SIZE as i32 + tilemap_x % TILE_SIZE as i32;
                if value_x < 0 || value_x >= image.width() as i32 {
                    continue;
                }
                let value_y = tile.1 as i32 * TILE_SIZE as i32 + tilemap_y % TILE_SIZE as i32;
                if value_y < 0 || value_y >= image.height() as i32 {
                    continue;
                }
                let value = image.canvas.read_data(value_x as usize, value_y as usize);

                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }
                let value = self.palette[value.to_index()];
                self.canvas
                    .write_data((dst_x + xi) as usize, (dst_y + yi) as usize, value);
            }
        }
    }

    fn bltm_transform(
        &mut self,
        x: f64,
        y: f64,
        tilemap: SharedTilemap,
        tilemap_x: f64,
        tilemap_y: f64,
        width: f64,
        height: f64,
        transparent: Option<Color>,
        rotate: f64,
        scale: f64,
    ) {
        let copy_width = utils::f64_to_u32(width.abs());
        let copy_height = utils::f64_to_u32(height.abs());
        let tilemap_width = tilemap.lock().width() as f64;
        let tilemap_height = tilemap.lock().height() as f64;
        let image = Self::new(copy_width, copy_height);
        {
            let mut image = image.lock();
            image.bltm(
                0.0,
                0.0,
                tilemap,
                tilemap_x,
                tilemap_y,
                width.abs(),
                height.abs(),
                None,
                None,
                None,
            );
            image.clip(
                -tilemap_x,
                -tilemap_y,
                tilemap_width * TILE_SIZE as f64,
                tilemap_height * TILE_SIZE as f64,
            );
            self.canvas.blt_transform(
                x,
                y,
                &image.canvas,
                0.0,
                0.0,
                width,
                height,
                transparent,
                Some(&self.palette),
                rotate,
                scale,
                true,
            );
        }
    }

    pub fn text(&mut self, x: f64, y: f64, string: &str, color: Color, font: Option<SharedFont>) {
        let mut x = utils::f64_to_i32(x); // No need to reflect camera_x
        let mut y = utils::f64_to_i32(y); // No need to reflect camera_y
        let color = self.palette[color as usize];
        if let Some(font) = font {
            font.lock().draw(&mut self.canvas, x, y, string, color);
            return;
        }
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
                FONT_IMAGE.clone(),
                src_x as f64,
                src_y as f64,
                FONT_WIDTH as f64,
                FONT_HEIGHT as f64,
                Some(0),
                Some(0.0),
                Some(1.0),
            );
            x += FONT_WIDTH as i32;
        }
        self.pal(1, palette1);
    }

    fn color_dist(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
        let (r1, g1, b1) = rgb1;
        let (r2, g2, b2) = rgb2;
        let dx = (r1 as f64 - r2 as f64) * 0.30;
        let dy = (g1 as f64 - g2 as f64) * 0.59;
        let dz = (b1 as f64 - b2 as f64) * 0.11;
        dx * dx + dy * dy + dz * dz
    }
}
