use std::cmp::max;
use std::mem::swap;

use crate::rectarea::RectArea;
use crate::utils::{as_i32, as_u32};

pub trait ToIndex {
    fn to_index(&self) -> usize;
}

pub struct Canvas<T: Copy + PartialEq + Default + ToIndex> {
    pub self_rect: RectArea,
    pub clip_rect: RectArea,
    pub camera_x: i32,
    pub camera_y: i32,
    pub data: Vec<Vec<T>>,
}

impl<T: Copy + PartialEq + Default + ToIndex> Canvas<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
            camera_x: 0,
            camera_y: 0,
            data: vec![vec![T::default(); width as usize]; height as usize],
        }
    }

    pub const fn width(&self) -> u32 {
        self.self_rect.width()
    }

    pub const fn height(&self) -> u32 {
        self.self_rect.height()
    }

    pub fn clip(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let x = as_i32(x);
        let y = as_i32(y);
        let width = as_u32(width);
        let height = as_u32(height);
        self.clip_rect = self
            .self_rect
            .intersects(RectArea::new(x, y, width, height));
    }

    pub fn clip0(&mut self) {
        self.clip_rect = self.self_rect;
    }

    pub fn camera(&mut self, x: f64, y: f64) {
        self.camera_x = as_i32(x);
        self.camera_y = as_i32(y);
    }

    pub fn camera0(&mut self) {
        self.camera_x = 0;
        self.camera_y = 0;
    }

    pub fn cls(&mut self, value: T) {
        let width = self.width();
        let height = self.height();
        for y in 0..height {
            for x in 0..width {
                self.write_data(x as i32, y as i32, value);
            }
        }
    }

    pub fn pget(&mut self, x: f64, y: f64) -> T {
        let x = as_i32(x);
        let y = as_i32(y);
        if self.self_rect.contains(x, y) {
            self.read_data(x, y)
        } else {
            T::default()
        }
    }

    pub fn pset(&mut self, x: f64, y: f64, value: T) {
        let x = as_i32(x) - self.camera_x;
        let y = as_i32(y) - self.camera_y;
        self.write_clipped_data(x, y, value);
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, value: T) {
        let x1 = as_i32(x1) - self.camera_x;
        let y1 = as_i32(y1) - self.camera_y;
        let x2 = as_i32(x2) - self.camera_x;
        let y2 = as_i32(y2) - self.camera_y;

        if x1 == x2 && y1 == y2 {
            self.write_clipped_data(x1, y1, value);
        } else if (x1 - x2).abs() > (y1 - y2).abs() {
            let (start_x, start_y, end_x, end_y) = if x1 < x2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };
            let length = end_x - start_x + 1;
            let alpha = (end_y - start_y) as f64 / (end_x - start_x) as f64;
            for xi in 0..length {
                self.write_clipped_data(start_x + xi, start_y + as_i32(alpha * xi as f64), value);
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
                self.write_clipped_data(start_x + as_i32(alpha * yi as f64), start_y + yi, value);
            }
        }
    }

    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = as_i32(x) - self.camera_x;
        let y = as_i32(y) - self.camera_y;
        let width = as_u32(width);
        let height = as_u32(height);
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
                self.write_data(x, y, value);
            }
        }
    }

    pub fn rectb(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = as_i32(x) - self.camera_x;
        let y = as_i32(y) - self.camera_y;
        let width = as_u32(width);
        let height = as_u32(height);
        let rect = RectArea::new(x, y, width, height);
        if rect.intersects(self.clip_rect).is_empty() {
            return;
        }
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();
        for x in left..=right {
            self.write_clipped_data(x, top, value);
            self.write_clipped_data(x, bottom, value);
        }
        for y in top..=bottom {
            self.write_clipped_data(left, y, value);
            self.write_clipped_data(right, y, value);
        }
    }

    pub fn circ(&mut self, x: f64, y: f64, radius: f64, value: T) {
        let x = as_i32(x) - self.camera_x;
        let y = as_i32(y) - self.camera_y;
        let radius = as_u32(radius);
        for xi in 0..=radius as i32 {
            let (x1, y1, x2, y2) = Self::ellipse_area(0.0, 0.0, radius as f64, radius as f64, xi);
            for yi in y1..=y2 {
                self.write_clipped_data(x + x1, y + yi, value);
                self.write_clipped_data(x + x2, y + yi, value);
                self.write_clipped_data(x + yi, y + x1, value);
                self.write_clipped_data(x + yi, y + x2, value);
            }
        }
    }

    pub fn circb(&mut self, x: f64, y: f64, radius: f64, value: T) {
        let x = as_i32(x) - self.camera_x;
        let y = as_i32(y) - self.camera_y;
        let radius = as_u32(radius);
        for xi in 0..=radius as i32 {
            let (x1, y1, x2, y2) = Self::ellipse_area(0.0, 0.0, radius as f64, radius as f64, xi);
            self.write_clipped_data(x + x1, y + y1, value);
            self.write_clipped_data(x + x2, y + y1, value);
            self.write_clipped_data(x + x1, y + y2, value);
            self.write_clipped_data(x + x2, y + y2, value);

            self.write_clipped_data(x + y1, y + x1, value);
            self.write_clipped_data(x + y1, y + x2, value);
            self.write_clipped_data(x + y2, y + x1, value);
            self.write_clipped_data(x + y2, y + x2, value);
        }
    }

    pub fn elli(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = as_i32(x) - self.camera_x;
        let y = as_i32(y) - self.camera_y;
        let width = as_u32(width);
        let height = as_u32(height);
        let (ra, rb, cx, cy) = Self::ellipse_params(x, y, width, height);
        for xi in x..(x + width as i32 / 2) + 1 {
            let (x1, y1, x2, y2) = Self::ellipse_area(cx, cy, ra, rb, xi);
            for yi in y1..=y2 {
                self.write_clipped_data(x1, yi, value);
                self.write_clipped_data(x2, yi, value);
            }
        }
        for yi in y..(y + height as i32 / 2) + 1 {
            let (y1, x1, y2, x2) = Self::ellipse_area(cy, cx, rb, ra, yi);
            for xi in x1..=x2 {
                self.write_clipped_data(xi, y1, value);
                self.write_clipped_data(xi, y2, value);
            }
        }
    }

    pub fn ellib(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = as_i32(x) - self.camera_x;
        let y = as_i32(y) - self.camera_y;
        let width = as_u32(width);
        let height = as_u32(height);
        let (ra, rb, cx, cy) = Self::ellipse_params(x, y, width, height);
        for xi in x..(x + width as i32 / 2) + 1 {
            let (x1, y1, x2, y2) = Self::ellipse_area(cx, cy, ra, rb, xi);
            self.write_clipped_data(x1, y1, value);
            self.write_clipped_data(x2, y1, value);
            self.write_clipped_data(x1, y2, value);
            self.write_clipped_data(x2, y2, value);
        }
        for yi in y..(y + height as i32 / 2) + 1 {
            let (y1, x1, y2, x2) = Self::ellipse_area(cy, cx, rb, ra, yi);
            self.write_clipped_data(x1, y1, value);
            self.write_clipped_data(x2, y1, value);
            self.write_clipped_data(x1, y2, value);
            self.write_clipped_data(x2, y2, value);
        }
    }

    pub fn tri(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, value: T) {
        let mut x1 = as_i32(x1) - self.camera_x;
        let mut y1 = as_i32(y1) - self.camera_y;
        let mut x2 = as_i32(x2) - self.camera_x;
        let mut y2 = as_i32(y2) - self.camera_y;
        let mut x3 = as_i32(x3) - self.camera_x;
        let mut y3 = as_i32(y3) - self.camera_y;
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
        let x_inter = as_i32(x1 as f64 + alpha13 * (y2 - y1) as f64);

        for y in y1..=y2 {
            let (x_slider, x_end) = if x_inter < x2 {
                (
                    as_i32(x_inter as f64 + alpha13 * (y - y2) as f64),
                    as_i32(x2 as f64 + alpha12 * (y - y2) as f64),
                )
            } else {
                (
                    as_i32(x2 as f64 + alpha12 * (y - y2) as f64),
                    as_i32(x_inter as f64 + alpha13 * (y - y2) as f64),
                )
            };
            for x in x_slider..=x_end {
                self.write_clipped_data(x, y, value);
            }
        }
        for y in (y2 + 1)..=y3 {
            let (x_slider, x_end) = if x_inter < x2 {
                (
                    as_i32(x_inter as f64 + alpha13 * (y - y2) as f64),
                    as_i32(x2 as f64 + alpha23 * (y - y2) as f64),
                )
            } else {
                (
                    as_i32(x2 as f64 + alpha23 * (y - y2) as f64),
                    as_i32(x_inter as f64 + alpha13 * (y - y2) as f64),
                )
            };
            for x in x_slider..=x_end {
                self.write_clipped_data(x, y, value);
            }
        }
    }

    pub fn trib(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, value: T) {
        self.line(x1, y1, x2, y2, value);
        self.line(x1, y1, x3, y3, value);
        self.line(x2, y2, x3, y3, value);
    }

    pub fn fill(&mut self, x: f64, y: f64, value: T) {
        let x = as_i32(x) - self.camera_x;
        let y = as_i32(y) - self.camera_y;
        if !self.clip_rect.contains(x, y) {
            return;
        }
        let dst_value = self.read_data(x, y);
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
        let x = as_i32(x) - self.camera_x;
        let y = as_i32(y) - self.camera_y;
        let canvas_x = as_i32(canvas_x);
        let canvas_y = as_i32(canvas_y);
        let width = as_i32(width);
        let height = as_i32(height);

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
                let value = canvas.read_data(value_x, value_y);
                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }
                let value = palette.map_or(value, |palette| palette[value.to_index()]);
                self.write_data(dst_x + xi, dst_y + yi, value);
            }
        }
    }

    fn read_data(&self, x: i32, y: i32) -> T {
        self.data[y as usize][x as usize]
    }

    fn write_data(&mut self, x: i32, y: i32, value: T) {
        self.data[y as usize][x as usize] = value;
    }

    fn write_clipped_data(&mut self, x: i32, y: i32, value: T) {
        if self.clip_rect.contains(x, y) {
            self.data[y as usize][x as usize] = value;
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
        let x1 = as_i32(cx - dx - 0.01);
        let y1 = as_i32(cy - dy - 0.01);
        let x2 = as_i32(cx + dx + 0.01);
        let y2 = as_i32(cy + dy + 0.01);
        (x1, y1, x2, y2)
    }

    fn fill_rec(&mut self, x: i32, y: i32, value: T, dst_value: T) {
        if self.read_data(x, y) != dst_value {
            return;
        }
        let mut xi = x;
        while xi >= self.clip_rect.left() {
            if self.read_data(xi, y) != dst_value {
                break;
            }
            self.write_data(xi, y, value);
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
            if self.read_data(xi, y) != dst_value {
                break;
            }
            self.write_data(xi, y, value);
            if y > self.clip_rect.top() {
                self.fill_rec(xi, y - 1, value, dst_value);
            }
            if y < self.clip_rect.bottom() {
                self.fill_rec(xi, y + 1, value, dst_value);
            }
            xi += 1;
        }
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
