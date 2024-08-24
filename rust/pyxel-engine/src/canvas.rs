use std::cmp::max;
use std::f64::consts::PI;
use std::mem::swap;

use crate::rect_area::RectArea;
use crate::utils::{f64_to_i32, f64_to_u32};

pub trait ToIndex {
    fn to_index(&self) -> usize;
}

pub struct Canvas<T: Copy + PartialEq + Default + ToIndex> {
    pub self_rect: RectArea,
    pub clip_rect: RectArea,
    pub camera_x: i32,
    pub camera_y: i32,
    pub alpha: f32,
    pub data: Vec<T>,
    should_write: fn(&Canvas<T>, i32, i32) -> bool,
}

impl<T: Copy + PartialEq + Default + ToIndex> Canvas<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
            camera_x: 0,
            camera_y: 0,
            alpha: 1.0,
            data: vec![T::default(); (width * height) as usize],
            should_write: Self::should_write_always,
        }
    }

    pub const fn width(&self) -> u32 {
        self.self_rect.width()
    }

    pub const fn height(&self) -> u32 {
        self.self_rect.height()
    }

    pub fn data_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    pub fn clip(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let x = f64_to_i32(x);
        let y = f64_to_i32(y);
        let width = f64_to_u32(width);
        let height = f64_to_u32(height);
        self.clip_rect = self
            .self_rect
            .intersects(RectArea::new(x, y, width, height));
    }

    pub fn clip0(&mut self) {
        self.clip_rect = self.self_rect;
    }

    pub fn camera(&mut self, x: f64, y: f64) {
        self.camera_x = f64_to_i32(x);
        self.camera_y = f64_to_i32(y);
    }

    pub fn camera0(&mut self) {
        self.camera_x = 0;
        self.camera_y = 0;
    }

    pub fn dither(&mut self, alpha: f32) {
        self.alpha = alpha;
        if alpha <= 0.0 {
            self.should_write = Self::should_write_never;
        } else if alpha >= 1.0 {
            self.should_write = Self::should_write_always;
        } else {
            self.should_write = Self::should_write_normal;
        }
    }

    pub fn cls(&mut self, value: T) {
        let width = self.width();
        let height = self.height();
        let alpha = self.alpha;
        self.dither(1.0);
        for y in 0..height {
            for x in 0..width {
                self.write_data(x as usize, y as usize, value);
            }
        }
        self.dither(alpha);
    }

    pub fn pget(&mut self, x: f64, y: f64) -> T {
        let x = f64_to_i32(x);
        let y = f64_to_i32(y);
        if self.clip_rect.contains(x, y) {
            self.read_data(x as usize, y as usize)
        } else {
            T::default()
        }
    }

    pub fn pset(&mut self, x: f64, y: f64, value: T) {
        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        self.write_data_with_clipping(x, y, value);
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, value: T) {
        let x1 = f64_to_i32(x1) - self.camera_x;
        let y1 = f64_to_i32(y1) - self.camera_y;
        let x2 = f64_to_i32(x2) - self.camera_x;
        let y2 = f64_to_i32(y2) - self.camera_y;

        if x1 == x2 && y1 == y2 {
            self.write_data_with_clipping(x1, y1, value);
        } else if (x1 - x2).abs() > (y1 - y2).abs() {
            let (start_x, start_y, end_x, end_y) = if x1 < x2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };
            let length = end_x - start_x + 1;
            let alpha = (end_y - start_y) as f64 / (end_x - start_x) as f64;
            for xi in 0..length {
                self.write_data_with_clipping(
                    start_x + xi,
                    start_y + f64_to_i32(alpha * xi as f64),
                    value,
                );
            }
        } else {
            let (start_x, start_y, end_x, end_y) = if y1 < y2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };
            let length = end_y - start_y + 1;
            let alpha = (end_x - start_x) as f64 / (end_y - start_y) as f64;
            for yi in 0..length {
                self.write_data_with_clipping(
                    start_x + f64_to_i32(alpha * yi as f64),
                    start_y + yi,
                    value,
                );
            }
        }
    }

    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        let width = f64_to_u32(width);
        let height = f64_to_u32(height);
        let rect = RectArea::new(x, y, width, height).intersects(self.clip_rect);
        if rect.is_empty() {
            return;
        }
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();
        for y in top..=bottom {
            for x in left..=right {
                self.write_data(x as usize, y as usize, value);
            }
        }
    }

    pub fn rectb(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        let width = f64_to_u32(width);
        let height = f64_to_u32(height);
        let rect = RectArea::new(x, y, width, height);
        if rect.intersects(self.clip_rect).is_empty() {
            return;
        }
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();
        for x in left..=right {
            self.write_data_with_clipping(x, top, value);
            self.write_data_with_clipping(x, bottom, value);
        }
        for y in top..=bottom {
            self.write_data_with_clipping(left, y, value);
            self.write_data_with_clipping(right, y, value);
        }
    }

    pub fn circ(&mut self, x: f64, y: f64, radius: f64, value: T) {
        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        let radius = f64_to_u32(radius);
        for xi in 0..=radius as i32 {
            let (x1, y1, x2, y2) = Self::ellipse_area(0.0, 0.0, radius as f64, radius as f64, xi);
            for yi in y1..=y2 {
                self.write_data_with_clipping(x + x1, y + yi, value);
                self.write_data_with_clipping(x + x2, y + yi, value);
                self.write_data_with_clipping(x + yi, y + x1, value);
                self.write_data_with_clipping(x + yi, y + x2, value);
            }
        }
    }

    pub fn circb(&mut self, x: f64, y: f64, radius: f64, value: T) {
        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        let radius = f64_to_u32(radius);
        for xi in 0..=radius as i32 {
            let (x1, y1, x2, y2) = Self::ellipse_area(0.0, 0.0, radius as f64, radius as f64, xi);
            self.write_data_with_clipping(x + x1, y + y1, value);
            self.write_data_with_clipping(x + x2, y + y1, value);
            self.write_data_with_clipping(x + x1, y + y2, value);
            self.write_data_with_clipping(x + x2, y + y2, value);

            self.write_data_with_clipping(x + y1, y + x1, value);
            self.write_data_with_clipping(x + y1, y + x2, value);
            self.write_data_with_clipping(x + y2, y + x1, value);
            self.write_data_with_clipping(x + y2, y + x2, value);
        }
    }

    pub fn elli(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        let width = f64_to_u32(width);
        let height = f64_to_u32(height);
        let (ra, rb, cx, cy) = Self::ellipse_params(x, y, width, height);
        for xi in x..=(x + width as i32 / 2) {
            let (x1, y1, x2, y2) = Self::ellipse_area(cx, cy, ra, rb, xi);
            for yi in y1..=y2 {
                self.write_data_with_clipping(x1, yi, value);
                self.write_data_with_clipping(x2, yi, value);
            }
        }
        for yi in y..=(y + height as i32 / 2) {
            let (y1, x1, y2, x2) = Self::ellipse_area(cy, cx, rb, ra, yi);
            for xi in x1..=x2 {
                self.write_data_with_clipping(xi, y1, value);
                self.write_data_with_clipping(xi, y2, value);
            }
        }
    }

    pub fn ellib(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        let width = f64_to_u32(width);
        let height = f64_to_u32(height);
        let (ra, rb, cx, cy) = Self::ellipse_params(x, y, width, height);
        for xi in x..=(x + width as i32 / 2) {
            let (x1, y1, x2, y2) = Self::ellipse_area(cx, cy, ra, rb, xi);
            self.write_data_with_clipping(x1, y1, value);
            self.write_data_with_clipping(x2, y1, value);
            self.write_data_with_clipping(x1, y2, value);
            self.write_data_with_clipping(x2, y2, value);
        }
        for yi in y..=(y + height as i32 / 2) {
            let (y1, x1, y2, x2) = Self::ellipse_area(cy, cx, rb, ra, yi);
            self.write_data_with_clipping(x1, y1, value);
            self.write_data_with_clipping(x2, y1, value);
            self.write_data_with_clipping(x1, y2, value);
            self.write_data_with_clipping(x2, y2, value);
        }
    }

    pub fn tri(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, value: T) {
        let mut x1 = f64_to_i32(x1) - self.camera_x;
        let mut y1 = f64_to_i32(y1) - self.camera_y;
        let mut x2 = f64_to_i32(x2) - self.camera_x;
        let mut y2 = f64_to_i32(y2) - self.camera_y;
        let mut x3 = f64_to_i32(x3) - self.camera_x;
        let mut y3 = f64_to_i32(y3) - self.camera_y;
        if y1 > y2 {
            swap(&mut y1, &mut y2);
            swap(&mut x1, &mut x2);
        }
        if y1 > y3 {
            swap(&mut y1, &mut y3);
            swap(&mut x1, &mut x3);
        }
        if y2 > y3 {
            swap(&mut y2, &mut y3);
            swap(&mut x2, &mut x3);
        }

        let alpha12 = if y2 == y1 {
            0.0
        } else {
            (x2 - x1) as f64 / (y2 - y1) as f64
        };
        let alpha13 = if y3 == y1 {
            0.0
        } else {
            (x3 - x1) as f64 / (y3 - y1) as f64
        };
        let alpha23 = if y3 == y2 {
            0.0
        } else {
            (x3 - x2) as f64 / (y3 - y2) as f64
        };
        let x_inter = f64_to_i32(x1 as f64 + alpha13 * (y2 - y1) as f64);

        for y in y1..=y2 {
            let (x_slider, x_end) = if x_inter < x2 {
                (
                    f64_to_i32(x_inter as f64 + alpha13 * (y - y2) as f64),
                    f64_to_i32(x2 as f64 + alpha12 * (y - y2) as f64),
                )
            } else {
                (
                    f64_to_i32(x2 as f64 + alpha12 * (y - y2) as f64),
                    f64_to_i32(x_inter as f64 + alpha13 * (y - y2) as f64),
                )
            };
            for x in x_slider..=x_end {
                self.write_data_with_clipping(x, y, value);
            }
        }
        for y in (y2 + 1)..=y3 {
            let (x_slider, x_end) = if x_inter < x2 {
                (
                    f64_to_i32(x_inter as f64 + alpha13 * (y - y2) as f64),
                    f64_to_i32(x2 as f64 + alpha23 * (y - y2) as f64),
                )
            } else {
                (
                    f64_to_i32(x2 as f64 + alpha23 * (y - y2) as f64),
                    f64_to_i32(x_inter as f64 + alpha13 * (y - y2) as f64),
                )
            };
            for x in x_slider..=x_end {
                self.write_data_with_clipping(x, y, value);
            }
        }
    }

    pub fn trib(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, value: T) {
        self.line(x1, y1, x2, y2, value);
        self.line(x1, y1, x3, y3, value);
        self.line(x2, y2, x3, y3, value);
    }

    pub fn fill(&mut self, x: f64, y: f64, value: T) {
        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        if !self.clip_rect.contains(x, y) {
            return;
        }
        let dst_value = self.read_data(x as usize, y as usize);
        if value != dst_value {
            self.fill_rec(x, y, value, dst_value);
        }
    }

    pub fn blt(
        &mut self,
        x: f64,
        y: f64,
        canvas: &Self,
        canvas_x: f64,
        canvas_y: f64,
        width: f64,
        height: f64,
        transparent: Option<T>,
        palette: Option<&[T]>,
    ) {
        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        let canvas_x = f64_to_i32(canvas_x);
        let canvas_y = f64_to_i32(canvas_y);
        let width = f64_to_i32(width);
        let height = f64_to_i32(height);

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
            self.clip_rect,
            canvas_x,
            canvas_y,
            canvas.self_rect,
            width,
            height,
        );
        if width == 0 || height == 0 {
            return;
        }

        for yi in 0..height {
            for xi in 0..width {
                let value_x = src_x + sign_x * xi + offset_x;
                let value_y = src_y + sign_y * yi + offset_y;
                let value = canvas.read_data(value_x as usize, value_y as usize);
                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }
                let value = palette.map_or(value, |palette| palette[value.to_index()]);
                self.write_data((dst_x + xi) as usize, (dst_y + yi) as usize, value);
            }
        }
    }

    pub fn blt_transform(
        &mut self,
        x: f64,
        y: f64,
        canvas: &Self,
        canvas_x: f64,
        canvas_y: f64,
        width: f64,
        height: f64,
        transparent: Option<T>,
        palette: Option<&[T]>,
        rotate: f64,
        scale: f64,
        use_canvas_clip: bool,
    ) {
        if scale < f64::EPSILON {
            return;
        }

        let x = f64_to_i32(x) - self.camera_x;
        let y = f64_to_i32(y) - self.camera_y;
        let canvas_x = f64_to_i32(canvas_x);
        let canvas_y = f64_to_i32(canvas_y);
        let sign_x = if width < 0.0 { -1.0 } else { 1.0 };
        let sign_y = if height < 0.0 { -1.0 } else { 1.0 };
        let width = f64_to_i32(width).abs();
        let height = f64_to_i32(height).abs();

        let canvas_area = RectArea::new(canvas_x, canvas_y, width as u32, height as u32)
            .intersects(if use_canvas_clip {
                canvas.clip_rect
            } else {
                canvas.self_rect
            });
        if canvas_area.is_empty() {
            return;
        }

        let half_width = (width - 1) as f64 / 2.0;
        let half_height = (height - 1) as f64 / 2.0;
        let src_cx = canvas_x as f64 + half_width;
        let src_cy = canvas_y as f64 + half_height;
        let dst_cx = x as f64 + half_width;
        let dst_cy = y as f64 + half_height;

        let rotate = rotate * PI / 180.0;
        let sin = f64::sin(rotate);
        let cos = f64::cos(rotate);
        let offset_x = (half_width * cos.abs() + half_height * sin.abs() + 1.0) * scale;
        let offset_y = (half_width * sin.abs() + half_height * cos.abs() + 1.0) * scale;
        let x1 = f64_to_i32(dst_cx - offset_x).max(self.clip_rect.left());
        let x2 = f64_to_i32(dst_cx + offset_x).min(self.clip_rect.right());
        let y1 = f64_to_i32(dst_cy - offset_y).max(self.clip_rect.top());
        let y2 = f64_to_i32(dst_cy + offset_y).min(self.clip_rect.bottom());

        for yi in y1..=y2 {
            for xi in x1..=x2 {
                let offset_x = (xi as f64 - dst_cx) * sign_x;
                let offset_y = (yi as f64 - dst_cy) * sign_y;
                let value_x = f64_to_i32(src_cx + (offset_x * cos - offset_y * sin) / scale);
                let value_y = f64_to_i32(src_cy + (offset_x * sin + offset_y * cos) / scale);
                if !canvas_area.contains(value_x, value_y) {
                    continue;
                }
                let value = canvas.read_data(value_x as usize, value_y as usize);
                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }
                let value = palette.map_or(value, |palette| palette[value.to_index()]);
                self.write_data(xi as usize, yi as usize, value);
            }
        }
    }

    pub fn read_data(&self, x: usize, y: usize) -> T {
        let width = self.width() as usize;
        self.data[width * y + x]
    }

    pub fn write_data(&mut self, x: usize, y: usize, value: T) {
        if (self.should_write)(self, x as i32, y as i32) {
            let width = self.width() as usize;
            self.data[width * y + x] = value;
        }
    }

    fn write_data_with_clipping(&mut self, x: i32, y: i32, value: T) {
        if (self.should_write)(self, x, y) && self.clip_rect.contains(x, y) {
            let width = self.width() as usize;
            self.data[width * y as usize + x as usize] = value;
        }
    }

    fn ellipse_params(x: i32, y: i32, width: u32, height: u32) -> (f64, f64, f64, f64) {
        let ra = (width - 1) as f64 / 2.0;
        let rb = (height - 1) as f64 / 2.0;
        let cx = x as f64 + ra;
        let cy = y as f64 + rb;
        (ra, rb, cx, cy)
    }

    fn ellipse_area(cx: f64, cy: f64, ra: f64, rb: f64, x: i32) -> (i32, i32, i32, i32) {
        let dx = x as f64 - cx;
        let dy = if ra > 0.0 {
            rb * (1.0 - dx * dx / (ra * ra)).sqrt()
        } else {
            rb
        };
        let x1 = f64_to_i32(cx - dx - 0.01);
        let y1 = f64_to_i32(cy - dy - 0.01);
        let x2 = f64_to_i32(cx + dx + 0.01);
        let y2 = f64_to_i32(cy + dy + 0.01);
        (x1, y1, x2, y2)
    }

    fn fill_rec(&mut self, x: i32, y: i32, value: T, dst_value: T) {
        if self.read_data(x as usize, y as usize) != dst_value {
            return;
        }
        let mut xi = x;
        while xi >= self.clip_rect.left() {
            if self.read_data(xi as usize, y as usize) != dst_value {
                break;
            }
            self.write_data(xi as usize, y as usize, value);
            if y > self.clip_rect.top() {
                self.fill_rec(xi, y - 1, value, dst_value);
            }
            if y < self.clip_rect.bottom() {
                self.fill_rec(xi, y + 1, value, dst_value);
            }
            xi -= 1;
        }
        let mut xi = x + 1;
        while xi <= self.clip_rect.right() {
            if self.read_data(xi as usize, y as usize) != dst_value {
                break;
            }
            self.write_data(xi as usize, y as usize, value);
            if y > self.clip_rect.top() {
                self.fill_rec(xi, y - 1, value, dst_value);
            }
            if y < self.clip_rect.bottom() {
                self.fill_rec(xi, y + 1, value, dst_value);
            }
            xi += 1;
        }
    }

    fn should_write_always(&self, _x: i32, _y: i32) -> bool {
        true
    }

    fn should_write_never(&self, _x: i32, _y: i32) -> bool {
        false
    }

    fn should_write_normal(&self, x: i32, y: i32) -> bool {
        const DITHERING_MATRIX: [[f32; 4]; 4] = [
            [1.0 / 16.0, 9.0 / 16.0, 3.0 / 16.0, 11.0 / 16.0],
            [13.0 / 16.0, 5.0 / 16.0, 15.0 / 16.0, 7.0 / 16.0],
            [3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0],
            [15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0],
        ];
        self.alpha > DITHERING_MATRIX[y.rem_euclid(4) as usize][x.rem_euclid(4) as usize]
    }
}

pub struct CopyArea {
    pub dst_x: i32,
    pub dst_y: i32,
    pub src_x: i32,
    pub src_y: i32,
    pub sign_x: i32,
    pub sign_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: i32,
    pub height: i32,
}

impl CopyArea {
    pub fn new(
        dst_x: i32,
        dst_y: i32,
        dst_rect: RectArea,
        src_x: i32,
        src_y: i32,
        src_rect: RectArea,
        width: i32,
        height: i32,
    ) -> Self {
        let flip_x = width < 0;
        let flip_y = height < 0;
        let width = width.abs();
        let height = height.abs();

        let left_cut = max(max(src_rect.left() - src_x, dst_rect.left() - dst_x), 0);
        let top_cut = max(max(src_rect.top() - src_y, dst_rect.top() - dst_y), 0);
        let right_cut = max(
            max(
                src_x + width - 1 - src_rect.right(),
                dst_x + width - 1 - dst_rect.right(),
            ),
            0,
        );
        let bottom_cut = max(
            max(
                src_y + height - 1 - src_rect.bottom(),
                dst_y + height - 1 - dst_rect.bottom(),
            ),
            0,
        );

        let width = max(width - left_cut - right_cut, 0);
        let height = max(height - top_cut - bottom_cut, 0);
        let (sign_x, offset_x) = if flip_x { (-1, width - 1) } else { (1, 0) };
        let (sign_y, offset_y) = if flip_y { (-1, height - 1) } else { (1, 0) };

        Self {
            dst_x: dst_x + left_cut,
            dst_y: dst_y + top_cut,
            src_x: src_x + if flip_x { right_cut } else { left_cut },
            src_y: src_y + if flip_y { bottom_cut } else { top_cut },
            sign_x,
            sign_y,
            offset_x,
            offset_y,
            width,
            height,
        }
    }
}
