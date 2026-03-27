use std::collections::HashMap;
use std::path::Path;
use std::{array, ptr};

use image::imageops;

use crate::canvas::{Canvas, CopyArea, PerspectiveProjection, ToIndex};
use crate::font::Font;
use crate::rect_area::RectArea;
use crate::settings::{
    FONT_HEIGHT, FONT_WIDTH, MAX_COLORS, MAX_FONT_CODE, MIN_FONT_CODE, NUM_FONT_ROWS, TILE_MASK,
    TILE_SHIFT, TILE_SIZE,
};
use crate::tilemap::{Tile, Tilemap};
use crate::{pyxel, utils};

pub type Rgb24 = u32;
pub type Color = u8;

/// Return `Some(&palette)` when a color map is active, `None` otherwise.
/// Uses field-level borrowing so the caller can still mutate `self.canvas`.
macro_rules! palette_opt {
    ($self:expr) => {
        if $self.palette_is_identity {
            None
        } else {
            Some(&$self.palette[..])
        }
    };
}

pub fn rgb_to_rgb8(rgb: Rgb24) -> (u8, u8, u8) {
    ((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
}

impl ToIndex for Color {
    fn to_index(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone)]
pub struct Image {
    pub(crate) canvas: Canvas<Color>,
    pub(crate) palette: [Color; MAX_COLORS as usize],
    pub(crate) palette_is_identity: bool,
}

impl Image {
    pub fn new(width: u32, height: u32) -> *mut Image {
        Box::into_raw(Box::new(Self {
            canvas: Canvas::new(width, height),
            palette: array::from_fn(|i| i as Color),
            palette_is_identity: true,
        }))
    }

    pub fn from_image(filename: &str, include_colors: Option<bool>) -> Result<*mut Image, String> {
        let include_colors = include_colors.unwrap_or(false);
        let colors = pyxel::colors();
        if include_colors {
            colors.clear();
        }
        let file_image = image::open(Path::new(&filename))
            .map_err(|_| format!("Failed to open file '{filename}'"))?
            .to_rgb8();
        let (width, height) = file_image.dimensions();
        let image = Self::new(width, height);

        {
            let image = unsafe { &mut *image };
            let mut color_table = HashMap::<(u8, u8, u8), Color>::with_capacity(256);

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
                                ((src_rgb.0 as Rgb24) << 16)
                                    | ((src_rgb.1 as Rgb24) << 8)
                                    | src_rgb.2 as Rgb24,
                            );
                            closest_color = colors.len() as Color - 1;
                        } else {
                            let mut closest_dist: f32 = f32::MAX;
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

        Ok(image)
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
            let image = unsafe { &mut *image };
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

        unsafe {
            self.draw_image(
                x as f32,
                y as f32,
                image,
                0.0,
                0.0,
                width as f32,
                height as f32,
                None,
                None,
                None,
            );
            drop(Box::from_raw(image));
        }
    }

    pub fn load(
        &mut self,
        x: i32,
        y: i32,
        filename: &str,
        include_colors: Option<bool>,
    ) -> Result<(), String> {
        let image = Self::from_image(filename, include_colors)?;
        let width = unsafe { &*image }.width();
        let height = unsafe { &*image }.height();

        unsafe {
            self.draw_image(
                x as f32,
                y as f32,
                image,
                0.0,
                0.0,
                width as f32,
                height as f32,
                None,
                None,
                None,
            );
            drop(Box::from_raw(image));
        }
        Ok(())
    }

    pub fn save(&self, filename: &str, scale: u32) -> Result<(), String> {
        let colors = pyxel::colors();
        let width = self.width();
        let height = self.height();
        let mut image = image::RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let rgb = colors[self.canvas.read_data(x as usize, y as usize) as usize];
                let (r, g, b) = rgb_to_rgb8(rgb);
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
            .map_err(|_| format!("Failed to save file '{filename}'"))?;
        Ok(())
    }

    pub fn set_clip_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.canvas.set_clip_rect(x, y, width, height);
    }

    pub fn reset_clip_rect(&mut self) {
        self.canvas.reset_clip_rect();
    }

    pub fn set_draw_offset(&mut self, x: f32, y: f32) {
        self.canvas.set_draw_offset(x, y);
    }

    pub fn reset_draw_offset(&mut self) {
        self.canvas.reset_draw_offset();
    }

    pub fn map_color(&mut self, src_color: Color, dst_color: Color) {
        self.palette[src_color as usize] = dst_color;
        self.palette_is_identity = false;
    }

    pub fn reset_color_map(&mut self) {
        self.palette = array::from_fn(|i| i as Color);
        self.palette_is_identity = true;
    }

    pub fn set_dithering(&mut self, alpha: f32) {
        self.canvas.set_dithering(alpha);
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.clear(self.palette[color as usize]);
    }

    pub fn get_pixel(&self, x: f32, y: f32) -> Color {
        self.canvas.get_value(x, y)
    }

    pub fn set_pixel(&mut self, x: f32, y: f32, color: Color) {
        self.canvas.set_value(x, y, self.palette[color as usize]);
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
        self.canvas
            .draw_line(x1, y1, x2, y2, self.palette[color as usize]);
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.canvas
            .draw_rect(x, y, width, height, self.palette[color as usize]);
    }

    pub fn draw_rect_border(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.canvas
            .draw_rect_border(x, y, width, height, self.palette[color as usize]);
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        self.canvas
            .draw_circle(x, y, radius, self.palette[color as usize]);
    }

    pub fn draw_circle_border(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        self.canvas
            .draw_circle_border(x, y, radius, self.palette[color as usize]);
    }

    pub fn draw_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.canvas
            .draw_ellipse(x, y, width, height, self.palette[color as usize]);
    }

    pub fn draw_ellipse_border(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.canvas
            .draw_ellipse_border(x, y, width, height, self.palette[color as usize]);
    }

    pub fn draw_triangle(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        color: Color,
    ) {
        self.canvas
            .draw_triangle(x1, y1, x2, y2, x3, y3, self.palette[color as usize]);
    }

    pub fn draw_triangle_border(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        color: Color,
    ) {
        self.canvas
            .draw_triangle_border(x1, y1, x2, y2, x3, y3, self.palette[color as usize]);
    }

    pub fn flood_fill(&mut self, x: f32, y: f32, color: Color) {
        self.canvas.flood_fill(x, y, self.palette[color as usize]);
    }

    /// # Safety
    /// `image` must be a valid, non-null pointer to an `Image`.
    pub unsafe fn draw_image(
        &mut self,
        x: f32,
        y: f32,
        image: *mut Image,
        image_x: f32,
        image_y: f32,
        width: f32,
        height: f32,
        transparent: Option<Color>,
        rotate: Option<f32>,
        scale: Option<f32>,
    ) {
        let rotate = rotate.unwrap_or(0.0);
        let scale = scale.unwrap_or(1.0);

        // When source and destination are the same image, copy to a
        // temporary canvas first to avoid read-write aliasing.
        let src_canvas = if ptr::eq(image.cast_const(), ptr::from_ref(self)) {
            Some(self.copy_region(image_x, image_y, width, height))
        } else {
            None
        };
        let (src, sx, sy) = match &src_canvas {
            Some(tmp) => (tmp, 0.0, 0.0),
            None => (&unsafe { &*image }.canvas, image_x, image_y),
        };

        let palette = palette_opt!(self);
        if rotate != 0.0 || scale != 1.0 {
            self.canvas.blit_with_transform(
                x,
                y,
                src,
                sx,
                sy,
                width,
                height,
                transparent,
                palette,
                rotate,
                scale,
                false,
            );
        } else {
            self.canvas
                .blit(x, y, src, sx, sy, width, height, transparent, palette);
        }
    }

    /// Copy a region of this image's canvas into a temporary canvas.
    fn copy_region(&self, x: f32, y: f32, width: f32, height: f32) -> Canvas<Color> {
        let w = utils::f32_to_u32(width.abs());
        let h = utils::f32_to_u32(height.abs());
        let mut canvas = Canvas::new(w, h);
        canvas.blit(0.0, 0.0, &self.canvas, x, y, w as f32, h as f32, None, None);
        canvas
    }

    /// # Safety
    /// `tilemap` must be a valid, non-null pointer to a `Tilemap`.
    pub unsafe fn draw_tilemap(
        &mut self,
        x: f32,
        y: f32,
        tilemap: *mut Tilemap,
        tilemap_x: f32,
        tilemap_y: f32,
        width: f32,
        height: f32,
        transparent: Option<Color>,
        rotate: Option<f32>,
        scale: Option<f32>,
    ) {
        let rotate = rotate.unwrap_or(0.0);
        let scale = scale.unwrap_or(1.0);
        if rotate != 0.0 || scale != 1.0 {
            self.draw_tilemap_with_transform(
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

        let x = utils::f32_to_i32(x) - self.canvas.draw_offset_x;
        let y = utils::f32_to_i32(y) - self.canvas.draw_offset_y;
        let tilemap_x = utils::f32_to_i32(tilemap_x);
        let tilemap_y = utils::f32_to_i32(tilemap_y);
        let width = utils::f32_to_i32(width);
        let height = utils::f32_to_i32(height);

        let tilemap = unsafe { &*tilemap };
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

        let image: &Image = unsafe { tilemap.imgsrc.resolve() };

        let tile_size = TILE_SIZE as i32;
        let img_w = image.width() as usize;
        let img_h = image.height() as usize;

        // Fast path: no flip, full alpha
        let palette = palette_opt!(self);
        if sign_x == 1 && sign_y == 1 && self.canvas.alpha >= 1.0 {
            let dst_w = self.canvas.width() as usize;

            for yi in 0..height {
                let tilemap_y = src_y + yi;
                let tile_y = (tilemap_y >> TILE_SHIFT) as usize;
                let pixel_y = (tilemap_y & TILE_MASK) as usize;
                let dst_row = dst_w * (dst_y + yi) as usize + dst_x as usize;

                let mut xi = 0;
                while xi < width {
                    let tilemap_x = src_x + xi;
                    let tile_x = (tilemap_x >> TILE_SHIFT) as usize;
                    let tile = tilemap.canvas.read_data(tile_x, tile_y);

                    let pixel_x = tilemap_x & TILE_MASK;
                    let chunk = (tile_size - pixel_x).min(width - xi) as usize;

                    let img_x = (tile.0 as i32 * tile_size + pixel_x) as usize;
                    let img_y = tile.1 as usize * TILE_SIZE as usize + pixel_y;

                    if img_y < img_h && img_x < img_w {
                        let valid = chunk.min(img_w - img_x);
                        let si = img_w * img_y + img_x;
                        let di = dst_row + xi as usize;
                        let src = &image.canvas.data[si..si + valid];
                        let dst = &mut self.canvas.data[di..di + valid];
                        match (transparent, palette) {
                            (None, None) => dst.copy_from_slice(src),
                            (Some(tkey), None) => {
                                for i in 0..valid {
                                    let val = src[i];
                                    if val != tkey {
                                        dst[i] = val;
                                    }
                                }
                            }
                            (None, Some(pal)) => {
                                for i in 0..valid {
                                    dst[i] = pal[src[i] as usize];
                                }
                            }
                            (Some(tkey), Some(pal)) => {
                                for i in 0..valid {
                                    let val = src[i];
                                    if val != tkey {
                                        dst[i] = pal[val as usize];
                                    }
                                }
                            }
                        }
                    }
                    xi += chunk as i32;
                }
            }
            return;
        }

        // General path: flip and/or dithering
        let img_w = img_w as i32;
        let img_h = img_h as i32;

        for yi in 0..height {
            let tilemap_y = src_y + sign_y * yi + offset_y;
            let tile_y = (tilemap_y >> TILE_SHIFT) as usize;
            let pixel_y = tilemap_y & TILE_MASK;
            let dst_yi = (dst_y + yi) as usize;

            let mut cached_tile_x = i32::MIN;
            let mut tile: Tile = (0, 0);

            for xi in 0..width {
                let tilemap_x = src_x + sign_x * xi + offset_x;
                let tile_x = tilemap_x >> TILE_SHIFT;

                if tile_x != cached_tile_x {
                    tile = tilemap.canvas.read_data(tile_x as usize, tile_y);
                    cached_tile_x = tile_x;
                }

                let value_x = tile.0 as i32 * tile_size + (tilemap_x & TILE_MASK);
                if value_x < 0 || value_x >= img_w {
                    continue;
                }
                let value_y = tile.1 as i32 * tile_size + pixel_y;
                if value_y < 0 || value_y >= img_h {
                    continue;
                }
                let value = image.canvas.read_data(value_x as usize, value_y as usize);

                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }
                let value = palette.map_or(value, |pal| pal[value.to_index()]);
                self.canvas.write_data((dst_x + xi) as usize, dst_yi, value);
            }
        }
    }

    fn draw_tilemap_with_transform(
        &mut self,
        x: f32,
        y: f32,
        tilemap: *mut Tilemap,
        tilemap_x: f32,
        tilemap_y: f32,
        width: f32,
        height: f32,
        transparent: Option<Color>,
        rotate: f32,
        scale: f32,
    ) {
        let tilemap_ref = unsafe { &*tilemap };
        let tilemap_pixel_w = tilemap_ref.width() as f32 * TILE_SIZE as f32;
        let tilemap_pixel_h = tilemap_ref.height() as f32 * TILE_SIZE as f32;

        // Render tilemap region into a temporary image
        let tmp = Self::new(
            utils::f32_to_u32(width.abs()),
            utils::f32_to_u32(height.abs()),
        );
        let tmp_ref = unsafe { &mut *tmp };
        unsafe {
            tmp_ref.draw_tilemap(
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
        }
        tmp_ref.set_clip_rect(-tilemap_x, -tilemap_y, tilemap_pixel_w, tilemap_pixel_h);

        let palette = palette_opt!(self);
        self.canvas.blit_with_transform(
            x,
            y,
            &tmp_ref.canvas,
            0.0,
            0.0,
            width,
            height,
            transparent,
            palette,
            rotate,
            scale,
            true,
        );
        unsafe { drop(Box::from_raw(tmp)) };
    }

    /// # Safety
    /// `image` must be a valid, non-null pointer to an `Image`.
    pub unsafe fn draw_image_3d(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        image: *mut Image,
        pos: (f32, f32, f32),
        rot: (f32, f32, f32),
        fov: Option<f32>,
        transparent: Option<Color>,
    ) {
        let src_canvas = if ptr::eq(image.cast_const(), ptr::from_ref(self)) {
            Some(self.canvas.clone())
        } else {
            None
        };
        let src = match &src_canvas {
            Some(tmp) => tmp,
            None => &unsafe { &*image }.canvas,
        };
        let palette = palette_opt!(self);
        self.canvas.blit_perspective(
            x,
            y,
            width,
            height,
            src,
            pos,
            rot,
            fov,
            transparent,
            palette,
        );
    }

    /// # Safety
    /// `tilemap` must be a valid, non-null pointer to a `Tilemap`.
    pub unsafe fn draw_tilemap_3d(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        tilemap: *mut Tilemap,
        pos: (f32, f32, f32),
        rot: (f32, f32, f32),
        fov: Option<f32>,
        transparent: Option<Color>,
    ) {
        let Some(proj) = PerspectiveProjection::new(
            x,
            y,
            width,
            height,
            self.canvas.draw_offset_x,
            self.canvas.draw_offset_y,
            pos,
            rot,
            fov,
        ) else {
            return;
        };

        let tilemap = unsafe { &*tilemap };
        let tile_size = TILE_SIZE as i32;
        let tm_w = tilemap.canvas.width() as i32;
        let tm_h = tilemap.canvas.height() as i32;

        let image: &Image = unsafe { tilemap.imgsrc.resolve() };
        let img_w = image.width() as i32;
        let img_h = image.height() as i32;

        let x1 = proj.dst_x.max(self.canvas.clip_rect.left());
        let x2 = (proj.dst_x + proj.w - 1).min(self.canvas.clip_rect.right());
        let y1 = proj.dst_y.max(self.canvas.clip_rect.top());
        let y2 = (proj.dst_y + proj.h - 1).min(self.canvas.clip_rect.bottom());

        let palette = palette_opt!(self);
        let (wx_step, wy_step, wz_step) = proj.x_steps();

        for yi in y1..=y2 {
            let (mut wx, mut wy, mut wz) = proj.world_base(x1, yi);

            for xi in x1..=x2 {
                if wz.abs() >= f32::EPSILON {
                    let t = -proj.cam_z / wz;
                    if t > 0.0 {
                        let src_xi = utils::f32_to_i32(proj.cam_x + t * wx);
                        let src_yi = utils::f32_to_i32(proj.cam_y + t * wy);

                        let tile_x = src_xi >> TILE_SHIFT;
                        let tile_y = src_yi >> TILE_SHIFT;
                        if tile_x >= 0 && tile_x < tm_w && tile_y >= 0 && tile_y < tm_h {
                            let tile = tilemap.canvas.read_data(tile_x as usize, tile_y as usize);
                            let px = tile.0 as i32 * tile_size + (src_xi & TILE_MASK);
                            let py = tile.1 as i32 * tile_size + (src_yi & TILE_MASK);
                            if px >= 0 && px < img_w && py >= 0 && py < img_h {
                                let value = image.canvas.read_data(px as usize, py as usize);
                                if transparent.is_none_or(|tkey| value != tkey) {
                                    let value = palette.map_or(value, |pal| pal[value.to_index()]);
                                    self.canvas.write_data(xi as usize, yi as usize, value);
                                }
                            }
                        }
                    }
                }
                wx += wx_step;
                wy += wy_step;
                wz += wz_step;
            }
        }
    }

    pub fn draw_text(
        &mut self,
        x: f32,
        y: f32,
        string: &str,
        color: Color,
        font: Option<*mut Font>,
    ) {
        if let Some(font) = font {
            let x = utils::f32_to_i32(x) - self.canvas.draw_offset_x;
            let y = utils::f32_to_i32(y) - self.canvas.draw_offset_y;
            let color = self.palette[color as usize];
            let font = unsafe { &mut *font };
            font.draw(&mut self.canvas, x, y, string, color);
            return;
        }

        let mut x = utils::f32_to_i32(x) - self.canvas.draw_offset_x;
        let mut y = utils::f32_to_i32(y) - self.canvas.draw_offset_y;
        let color = self.palette[color as usize];
        let font_image: *const Image = pyxel::font_image();
        let font_data = unsafe { &(*font_image).canvas.data };
        let font_w = unsafe { (*font_image).canvas.width() } as usize;

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
            let src_x = (code % NUM_FONT_ROWS as i32) as usize;
            let src_y = (code / NUM_FONT_ROWS as i32) as usize;
            let font_row = font_w * src_y * FONT_HEIGHT as usize + src_x * FONT_WIDTH as usize;

            // Fast path: character fully inside clip rect and no dithering
            if self.canvas.alpha >= 1.0
                && x >= self.canvas.clip_rect.left()
                && x + FONT_WIDTH as i32 - 1 <= self.canvas.clip_rect.right()
                && y >= self.canvas.clip_rect.top()
                && y + FONT_HEIGHT as i32 - 1 <= self.canvas.clip_rect.bottom()
            {
                let canvas_w = self.canvas.width() as usize;
                for fy in 0..FONT_HEIGHT as usize {
                    for fx in 0..FONT_WIDTH as usize {
                        if font_data[font_row + font_w * fy + fx] != 0 {
                            self.canvas.data
                                [canvas_w * (y + fy as i32) as usize + (x + fx as i32) as usize] =
                                color;
                        }
                    }
                }
            } else {
                for fy in 0..FONT_HEIGHT as usize {
                    for fx in 0..FONT_WIDTH as usize {
                        if font_data[font_row + font_w * fy + fx] != 0 {
                            self.canvas.write_data_with_clipping(
                                x + fx as i32,
                                y + fy as i32,
                                color,
                            );
                        }
                    }
                }
            }
            x += FONT_WIDTH as i32;
        }
    }

    fn color_dist(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f32 {
        let (r1, g1, b1) = rgb1;
        let (r2, g2, b2) = rgb2;
        // Weighted by perceived luminance contribution
        let dr = (r1 as f32 - r2 as f32) * 0.30;
        let dg = (g1 as f32 - g2 as f32) * 0.59;
        let db = (b1 as f32 - b2 as f32) * 0.11;
        dr * dr + dg * dg + db * db
    }
}
