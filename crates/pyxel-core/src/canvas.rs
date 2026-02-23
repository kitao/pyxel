use std::cmp::max;
use std::f32::consts::PI;
use std::mem::swap;

use crate::rect_area::RectArea;
use crate::utils::{f32_to_i32, f32_to_u32};

const DITHERING_MATRIX: [[f32; 4]; 4] = [
    [1.0 / 16.0, 9.0 / 16.0, 3.0 / 16.0, 11.0 / 16.0],
    [13.0 / 16.0, 5.0 / 16.0, 15.0 / 16.0, 7.0 / 16.0],
    [3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0],
    [15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0],
];

pub trait ToIndex {
    fn to_index(&self) -> usize;
}

#[derive(Clone)]
pub struct Canvas<T: Copy + PartialEq + Default + ToIndex> {
    pub self_rect: RectArea,
    pub clip_rect: RectArea,
    pub camera_x: i32,
    pub camera_y: i32,
    pub alpha: f32,
    pub data: Vec<T>,
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

    pub fn clip(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let x = f32_to_i32(x);
        let y = f32_to_i32(y);
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
        self.clip_rect = self
            .self_rect
            .intersects(RectArea::new(x, y, width, height));
    }

    pub fn clip0(&mut self) {
        self.clip_rect = self.self_rect;
    }

    pub fn camera(&mut self, x: f32, y: f32) {
        self.camera_x = f32_to_i32(x);
        self.camera_y = f32_to_i32(y);
    }

    pub fn camera0(&mut self) {
        self.camera_x = 0;
        self.camera_y = 0;
    }

    pub fn dither(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    pub fn cls(&mut self, value: T) {
        self.data.fill(value);
    }

    pub fn pget(&mut self, x: f32, y: f32) -> T {
        let x = f32_to_i32(x);
        let y = f32_to_i32(y);
        if self.clip_rect.contains(x, y) {
            self.read_data(x as usize, y as usize)
        } else {
            T::default()
        }
    }

    pub fn pset(&mut self, x: f32, y: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        self.write_data_with_clipping(x, y, value);
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, value: T) {
        let x1 = f32_to_i32(x1) - self.camera_x;
        let y1 = f32_to_i32(y1) - self.camera_y;
        let x2 = f32_to_i32(x2) - self.camera_x;
        let y2 = f32_to_i32(y2) - self.camera_y;

        if x1 == x2 && y1 == y2 {
            self.write_data_with_clipping(x1, y1, value);
        } else if (x1 - x2).abs() > (y1 - y2).abs() {
            let (start_x, start_y, end_x, end_y) = if x1 < x2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };
            let length = end_x - start_x + 1;
            let alpha = (end_y - start_y) as f32 / (end_x - start_x) as f32;
            for xi in 0..length {
                self.write_data_with_clipping(
                    start_x + xi,
                    start_y + f32_to_i32(alpha * xi as f32),
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
            let alpha = (end_x - start_x) as f32 / (end_y - start_y) as f32;
            for yi in 0..length {
                self.write_data_with_clipping(
                    start_x + f32_to_i32(alpha * yi as f32),
                    start_y + yi,
                    value,
                );
            }
        }
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
        let rect = RectArea::new(x, y, width, height).intersects(self.clip_rect);
        if rect.is_empty() {
            return;
        }

        let left = rect.left() as usize;
        let top = rect.top() as usize;
        let right = rect.right() as usize;
        let bottom = rect.bottom() as usize;
        let w = self.width() as usize;

        if self.alpha >= 1.0 {
            for y in top..=bottom {
                self.data[w * y + left..=w * y + right].fill(value);
            }
        } else {
            for y in top..=bottom {
                for x in left..=right {
                    self.write_data(x, y, value);
                }
            }
        }
    }

    pub fn rectb(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
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

    pub fn circ(&mut self, x: f32, y: f32, radius: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let radius = f32_to_u32(radius);

        for xi in 0..=radius as i32 {
            let (x1, y1, x2, y2) = Self::ellipse_area(0.0, 0.0, radius as f32, radius as f32, xi);
            for yi in y1..=y2 {
                self.write_data_with_clipping(x + x1, y + yi, value);
                self.write_data_with_clipping(x + x2, y + yi, value);
                self.write_data_with_clipping(x + yi, y + x1, value);
                self.write_data_with_clipping(x + yi, y + x2, value);
            }
        }
    }

    pub fn circb(&mut self, x: f32, y: f32, radius: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let radius = f32_to_u32(radius);

        for xi in 0..=radius as i32 {
            let (x1, y1, x2, y2) = Self::ellipse_area(0.0, 0.0, radius as f32, radius as f32, xi);
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

    pub fn elli(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
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

    pub fn ellib(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
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

    pub fn tri(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, value: T) {
        let mut x1 = f32_to_i32(x1) - self.camera_x;
        let mut y1 = f32_to_i32(y1) - self.camera_y;
        let mut x2 = f32_to_i32(x2) - self.camera_x;
        let mut y2 = f32_to_i32(y2) - self.camera_y;
        let mut x3 = f32_to_i32(x3) - self.camera_x;
        let mut y3 = f32_to_i32(y3) - self.camera_y;

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
            (x2 - x1) as f32 / (y2 - y1) as f32
        };
        let alpha13 = if y3 == y1 {
            0.0
        } else {
            (x3 - x1) as f32 / (y3 - y1) as f32
        };
        let alpha23 = if y3 == y2 {
            0.0
        } else {
            (x3 - x2) as f32 / (y3 - y2) as f32
        };
        let x_inter = f32_to_i32(x1 as f32 + alpha13 * (y2 - y1) as f32);

        for y in y1..=y2 {
            let (x_slider, x_end) = if x_inter < x2 {
                (
                    f32_to_i32(x_inter as f32 + alpha13 * (y - y2) as f32),
                    f32_to_i32(x2 as f32 + alpha12 * (y - y2) as f32),
                )
            } else {
                (
                    f32_to_i32(x2 as f32 + alpha12 * (y - y2) as f32),
                    f32_to_i32(x_inter as f32 + alpha13 * (y - y2) as f32),
                )
            };
            for x in x_slider..=x_end {
                self.write_data_with_clipping(x, y, value);
            }
        }

        for y in (y2 + 1)..=y3 {
            let (x_slider, x_end) = if x_inter < x2 {
                (
                    f32_to_i32(x_inter as f32 + alpha13 * (y - y2) as f32),
                    f32_to_i32(x2 as f32 + alpha23 * (y - y2) as f32),
                )
            } else {
                (
                    f32_to_i32(x2 as f32 + alpha23 * (y - y2) as f32),
                    f32_to_i32(x_inter as f32 + alpha13 * (y - y2) as f32),
                )
            };
            for x in x_slider..=x_end {
                self.write_data_with_clipping(x, y, value);
            }
        }
    }

    pub fn trib(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, value: T) {
        self.line(x1, y1, x2, y2, value);
        self.line(x1, y1, x3, y3, value);
        self.line(x2, y2, x3, y3, value);
    }

    pub fn fill(&mut self, x: f32, y: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        if !self.clip_rect.contains(x, y) {
            return;
        }

        let dst_value = self.read_data(x as usize, y as usize);
        if value == dst_value {
            return;
        }

        let mut visit_stack = Vec::new();
        visit_stack.push((x, y));
        while let Some((x, y)) = visit_stack.pop() {
            if !self.clip_rect.contains(x, y) || self.read_data(x as usize, y as usize) != dst_value
            {
                continue;
            }

            let mut left = x;
            let mut right = x;
            while left > self.clip_rect.left()
                && self.read_data((left - 1) as usize, y as usize) == dst_value
            {
                left -= 1;
            }
            while right < self.clip_rect.right()
                && self.read_data((right + 1) as usize, y as usize) == dst_value
            {
                right += 1;
            }

            for xi in left..=right {
                self.write_data(xi as usize, y as usize, value);
            }

            for scan_y in [y - 1, y + 1] {
                if scan_y >= self.clip_rect.top() && scan_y <= self.clip_rect.bottom() {
                    let mut scan_x = left;
                    let mut in_segment = false;
                    while scan_x <= right {
                        let is_target =
                            self.read_data(scan_x as usize, scan_y as usize) == dst_value;
                        if is_target && !in_segment {
                            visit_stack.push((scan_x, scan_y));
                            in_segment = true;
                        } else if !is_target {
                            in_segment = false;
                        }
                        scan_x += 1;
                    }
                }
            }
        }
    }

    pub fn blt(
        &mut self,
        x: f32,
        y: f32,
        canvas: &Self,
        canvas_x: f32,
        canvas_y: f32,
        width: f32,
        height: f32,
        transparent: Option<T>,
        palette: Option<&[T]>,
    ) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let canvas_x = f32_to_i32(canvas_x);
        let canvas_y = f32_to_i32(canvas_y);
        let width = f32_to_i32(width);
        let height = f32_to_i32(height);

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

        // Fast path: no flip, no transparency, no palette, full alpha
        if sign_x == 1
            && sign_y == 1
            && transparent.is_none()
            && palette.is_none()
            && self.alpha >= 1.0
        {
            let dst_w = self.width() as usize;
            let src_w = canvas.width() as usize;
            let width = width as usize;
            for yi in 0..height as usize {
                let dst_start = dst_w * (dst_y as usize + yi) + dst_x as usize;
                let src_start = src_w * (src_y as usize + yi) + src_x as usize;
                self.data[dst_start..dst_start + width]
                    .copy_from_slice(&canvas.data[src_start..src_start + width]);
            }
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
        x: f32,
        y: f32,
        canvas: &Self,
        canvas_x: f32,
        canvas_y: f32,
        width: f32,
        height: f32,
        transparent: Option<T>,
        palette: Option<&[T]>,
        rotate: f32,
        scale: f32,
        use_canvas_clip: bool,
    ) {
        if scale < f32::EPSILON {
            return;
        }

        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let canvas_x = f32_to_i32(canvas_x);
        let canvas_y = f32_to_i32(canvas_y);
        let sign_x = if width < 0.0 { -1.0 } else { 1.0 };
        let sign_y = if height < 0.0 { -1.0 } else { 1.0 };
        let width = f32_to_i32(width).abs();
        let height = f32_to_i32(height).abs();

        let canvas_area = RectArea::new(canvas_x, canvas_y, width as u32, height as u32)
            .intersects(if use_canvas_clip {
                canvas.clip_rect
            } else {
                canvas.self_rect
            });
        if canvas_area.is_empty() {
            return;
        }

        let half_width = (width - 1) as f32 / 2.0;
        let half_height = (height - 1) as f32 / 2.0;
        let src_cx = canvas_x as f32 + half_width;
        let src_cy = canvas_y as f32 + half_height;
        let dst_cx = x as f32 + half_width;
        let dst_cy = y as f32 + half_height;

        let rotate = rotate * PI / 180.0;
        let sin = -f32::sin(rotate); // Clockwise
        let cos = f32::cos(rotate);
        let offset_x = (half_width * cos.abs() + half_height * sin.abs() + 1.0) * scale;
        let offset_y = (half_width * sin.abs() + half_height * cos.abs() + 1.0) * scale;
        let x1 = f32_to_i32(dst_cx - offset_x).max(self.clip_rect.left());
        let x2 = f32_to_i32(dst_cx + offset_x).min(self.clip_rect.right());
        let y1 = f32_to_i32(dst_cy - offset_y).max(self.clip_rect.top());
        let y2 = f32_to_i32(dst_cy + offset_y).min(self.clip_rect.bottom());

        for yi in y1..=y2 {
            for xi in x1..=x2 {
                let offset_x = (xi as f32 - dst_cx) * sign_x;
                let offset_y = (yi as f32 - dst_cy) * sign_y;
                let value_x = f32_to_i32(src_cx + (offset_x * cos - offset_y * sin) / scale);
                let value_y = f32_to_i32(src_cy + (offset_x * sin + offset_y * cos) / scale);
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
        if self.should_write(x as i32, y as i32) {
            let width = self.width() as usize;
            self.data[width * y + x] = value;
        }
    }

    pub(crate) fn write_data_with_clipping(&mut self, x: i32, y: i32, value: T) {
        if self.clip_rect.contains(x, y) && self.should_write(x, y) {
            let width = self.width() as usize;
            self.data[width * y as usize + x as usize] = value;
        }
    }

    fn ellipse_params(x: i32, y: i32, width: u32, height: u32) -> (f32, f32, f32, f32) {
        let ra = (width - 1) as f32 / 2.0;
        let rb = (height - 1) as f32 / 2.0;
        let cx = x as f32 + ra;
        let cy = y as f32 + rb;
        (ra, rb, cx, cy)
    }

    fn ellipse_area(cx: f32, cy: f32, ra: f32, rb: f32, x: i32) -> (i32, i32, i32, i32) {
        let dx = x as f32 - cx;
        let dy = if ra > 0.0 {
            rb * (1.0 - dx * dx / (ra * ra)).sqrt()
        } else {
            rb
        };

        let x1 = f32_to_i32(cx - dx - 0.01);
        let y1 = f32_to_i32(cy - dy - 0.01);
        let x2 = f32_to_i32(cx + dx + 0.01);
        let y2 = f32_to_i32(cy + dy + 0.01);

        (x1, y1, x2, y2)
    }

    fn should_write(&self, x: i32, y: i32) -> bool {
        if self.alpha >= 1.0 {
            return true;
        }
        if self.alpha <= 0.0 {
            return false;
        }
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

#[cfg(test)]
mod tests {
    use super::Canvas;

    #[test]
    fn fill_doesnt_overflow_stack() {
        let mut canvas = Canvas::<u8>::new(256, 256);
        canvas.fill(0.0, 0.0, 8);
        // this assertion won't even be reached if the above line overflows the stack
        assert_eq!(canvas.read_data(128, 128), 8);
    }
}
