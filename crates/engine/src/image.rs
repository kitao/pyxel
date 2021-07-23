use std::collections::HashMap;
use std::path::Path;

use crate::canvas::Canvas;
use crate::graphics::Graphics;
use crate::palette::{Color, Palette};
use crate::rectarea::RectArea;
use crate::settings::MAX_COLOR_COUNT;
use crate::tilemap::{Tile, Tilemap};
use crate::utility::{parse_hex_string, simplify_string};

pub struct Image {
    width: u32,
    height: u32,
    palette: Palette,
    data: Vec<Vec<Color>>,
    self_rect: RectArea,
    clip_rect: RectArea,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width: width,
            height: height,
            palette: Palette::new(),
            data: vec![vec![0; width as usize]; height as usize],
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
        }
    }

    pub fn palette(&self) -> &Palette {
        &self.palette
    }

    pub fn palette_mut(&mut self) -> &mut Palette {
        &mut self.palette
    }

    pub fn set(&mut self, x: i32, y: i32, data: &[&str]) {
        let width = data[0].len() as u32;
        let height = data.len() as u32;

        if width == 0 || height == 0 {
            return;
        }

        let mut image = Image::new(width, height);

        for i in 0..height {
            let data = simplify_string(data[i as usize]);

            for j in 0..width {
                if let Some(value) = parse_hex_string(&data[j as usize..j as usize + 1]) {
                    image.data[i as usize][j as usize] = value as Color;
                } else {
                    panic!("invalid image data");
                }
            }
        }

        self.copy(x, y, &image, 0, 0, width as i32, height as i32, None);
    }

    pub fn draw_tilemap(
        &mut self,
        x: i32,
        y: i32,
        src: &Tilemap,
        u: i32,
        v: i32,
        width: i32,
        height: i32,
        tile_key: Option<Tile>,
    ) {
        //
    }

    pub fn draw_text(&mut self, graphics: &Graphics, x: i32, y: i32, text: &str, color: Color) {
        //
    }

    pub fn load_image(&mut self, x: i32, y: i32, filename: &str) {
        let src_image = image::open(&Path::new(&filename)).unwrap().to_rgb8();
        let (width, height) = src_image.dimensions();
        let mut dst_image = Image::new(width, height);
        let dst_data = dst_image.data_mut();
        let mut color_table = HashMap::<(u8, u8, u8), Color>::new();
        let max_used_color = Image::max_used_color(&self.palette);

        for i in 0..height {
            for j in 0..width {
                let p = src_image.get_pixel(j, i);
                let src_rgb = (p[0], p[1], p[2]);

                if let Some(color) = color_table.get(&src_rgb) {
                    dst_data[i as usize][j as usize] = *color;
                } else {
                    let mut closest_color: Color = 0;
                    let mut closest_dist: f64 = f64::MAX;

                    for k in 0..=max_used_color {
                        let pal_color = self.palette.display_color(k);
                        let pal_rgb = (
                            ((pal_color >> 16) & 0xff) as u8,
                            ((pal_color >> 8) & 0xff) as u8,
                            (pal_color & 0xff) as u8,
                        );
                        let dist = Image::color_dist(src_rgb, pal_rgb);

                        if dist < closest_dist {
                            closest_color = k;
                            closest_dist = dist;
                        }
                    }

                    color_table.insert(src_rgb, closest_color);
                    dst_data[i as usize][j as usize] = closest_color;
                }
            }
        }

        self.copy(x, y, &dst_image, 0, 0, width as i32, height as i32, None);
    }

    pub fn save_image(&self, filename: &str, scale: u32) {
        //
    }

    fn max_used_color(palette: &Palette) -> Color {
        let mut max_used_color: Color = Color::MAX;

        'outer_loop: for i in (1..MAX_COLOR_COUNT).rev() {
            max_used_color = i as Color;

            let color = palette.display_color(i as Color);

            for j in (0..i).rev() {
                if palette.display_color(j as Color) == color {
                    continue 'outer_loop;
                }
            }

            break;
        }

        max_used_color
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

impl Canvas<Color> for Image {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn data<'a>(&'a self) -> &'a Vec<Vec<Color>> {
        &self.data
    }

    fn data_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<Color>> {
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

    fn _render_value(&self, original_value: Color) -> Color {
        self.palette.render_color(original_value)
    }
}
