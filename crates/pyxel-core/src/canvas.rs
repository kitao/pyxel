use std::f32::consts::PI;
use std::mem::swap;

use crate::rect_area::RectArea;
use crate::utils::{f32_to_i32, f32_to_u32};

const ELLIPSE_ROUNDING_BIAS: f32 = 0.01;
// Pyxel's historical dither uses eight thresholds repeated over a 4x4 footprint.
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
    // Constructors

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

    // Public accessors

    pub const fn width(&self) -> u32 {
        self.self_rect.width()
    }

    pub const fn height(&self) -> u32 {
        self.self_rect.height()
    }

    pub fn data_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    // Clip and offset

    pub fn set_clip_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let x = f32_to_i32(x);
        let y = f32_to_i32(y);
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
        self.clip_rect = self
            .self_rect
            .intersection(RectArea::new(x, y, width, height));
    }

    pub fn reset_clip_rect(&mut self) {
        self.clip_rect = self.self_rect;
    }

    pub fn set_camera(&mut self, x: f32, y: f32) {
        self.camera_x = f32_to_i32(x);
        self.camera_y = f32_to_i32(y);
    }

    pub fn reset_camera(&mut self) {
        self.camera_x = 0;
        self.camera_y = 0;
    }

    // Dithering

    pub fn set_dithering(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    // Public data operations

    pub fn clear(&mut self, value: T) {
        self.data.fill(value);
    }

    pub fn value(&self, x: f32, y: f32) -> T {
        let x = f32_to_i32(x);
        let y = f32_to_i32(y);
        if self.clip_rect.contains(x, y) {
            self.read_data(x as usize, y as usize)
        } else {
            T::default()
        }
    }

    pub fn set_value(&mut self, x: f32, y: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        self.write_data_with_clipping(x, y, value);
    }

    // Drawing primitives

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, value: T) {
        // Rasterize horizontal, vertical, and sloped line paths.
        let x1 = f32_to_i32(x1) - self.camera_x;
        let y1 = f32_to_i32(y1) - self.camera_y;
        let x2 = f32_to_i32(x2) - self.camera_x;
        let y2 = f32_to_i32(y2) - self.camera_y;

        if y1 == y2 {
            self.fill_row_with_dither(x1.min(x2), x1.max(x2), y1, value);
        } else if x1 == x2 {
            self.fill_column_with_dither(y1.min(y2), y1.max(y2), x1, value);
        } else if (x1 - x2).abs() > (y1 - y2).abs() {
            let (start_x, start_y, end_x, end_y) = if x1 < x2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };
            let length = end_x - start_x + 1;
            let slope = (end_y - start_y) as f32 / (end_x - start_x) as f32;
            for xi in 0..length {
                self.write_data_with_clipping(
                    start_x + xi,
                    start_y + f32_to_i32(slope * xi as f32),
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
            let slope = (end_x - start_x) as f32 / (end_y - start_y) as f32;
            for yi in 0..length {
                self.write_data_with_clipping(
                    start_x + f32_to_i32(slope * yi as f32),
                    start_y + yi,
                    value,
                );
            }
        }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
        let rect = RectArea::new(x, y, width, height).intersection(self.clip_rect);
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

    pub fn draw_rect_border(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
        let rect = RectArea::new(x, y, width, height);
        if rect.intersection(self.clip_rect).is_empty() {
            return;
        }

        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();
        self.fill_row_with_dither(left, right, top, value);
        self.fill_row_with_dither(left, right, bottom, value);
        self.fill_column_with_dither(top, bottom, left, value);
        self.fill_column_with_dither(top, bottom, right, value);
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let radius = f32_to_u32(radius);
        let r = radius as f32;

        for xi in 0..=radius as i32 {
            let (x1, y1, x2, y2) = Self::ellipse_area(0.0, 0.0, r, r, xi);
            self.fill_column_with_dither(y + y1, y + y2, x + x1, value);
            self.fill_column_with_dither(y + y1, y + y2, x + x2, value);
            self.fill_row_with_dither(x + y1, x + y2, y + x1, value);
            self.fill_row_with_dither(x + y1, x + y2, y + x2, value);
        }
    }

    pub fn draw_circle_border(&mut self, x: f32, y: f32, radius: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let radius = f32_to_u32(radius);
        let r = radius as f32;

        for xi in 0..=radius as i32 {
            let (x1, y1, x2, y2) = Self::ellipse_area(0.0, 0.0, r, r, xi);
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

    pub fn draw_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
        if width == 0 || height == 0 {
            return;
        }
        let (ra, rb, cx, cy) = Self::ellipse_params(x, y, width, height);

        for xi in x..=(x + width as i32 / 2) {
            let (x1, y1, x2, y2) = Self::ellipse_area(cx, cy, ra, rb, xi);
            self.fill_column_with_dither(y1, y2, x1, value);
            self.fill_column_with_dither(y1, y2, x2, value);
        }

        for yi in y..=(y + height as i32 / 2) {
            let (y1, x1, y2, x2) = Self::ellipse_area(cy, cx, rb, ra, yi);
            self.fill_row_with_dither(x1, x2, y1, value);
            self.fill_row_with_dither(x1, x2, y2, value);
        }
    }

    pub fn draw_ellipse_border(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let width = f32_to_u32(width);
        let height = f32_to_u32(height);
        if width == 0 || height == 0 {
            return;
        }
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

    pub fn draw_triangle(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        value: T,
    ) {
        // Sort vertices and split the triangle into scanline spans.
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

        // All vertices on one row: the split fill below would drop the x3 span
        if y1 == y3 {
            self.fill_row_with_dither(x1.min(x2).min(x3), x1.max(x2).max(x3), y1, value);
            return;
        }

        let slope12 = if y2 == y1 {
            0.0
        } else {
            (x2 - x1) as f32 / (y2 - y1) as f32
        };
        let slope13 = if y3 == y1 {
            0.0
        } else {
            (x3 - x1) as f32 / (y3 - y1) as f32
        };
        let slope23 = if y3 == y2 {
            0.0
        } else {
            (x3 - x2) as f32 / (y3 - y2) as f32
        };
        let x_split = f32_to_i32(x1 as f32 + slope13 * (y2 - y1) as f32);

        for y in y1..=y2 {
            let (x_start, x_end) = if x_split < x2 {
                (
                    f32_to_i32(x_split as f32 + slope13 * (y - y2) as f32),
                    f32_to_i32(x2 as f32 + slope12 * (y - y2) as f32),
                )
            } else {
                (
                    f32_to_i32(x2 as f32 + slope12 * (y - y2) as f32),
                    f32_to_i32(x_split as f32 + slope13 * (y - y2) as f32),
                )
            };
            self.fill_row_with_dither(x_start, x_end, y, value);
        }
        for y in (y2 + 1)..=y3 {
            let (x_start, x_end) = if x_split < x2 {
                (
                    f32_to_i32(x_split as f32 + slope13 * (y - y2) as f32),
                    f32_to_i32(x2 as f32 + slope23 * (y - y2) as f32),
                )
            } else {
                (
                    f32_to_i32(x2 as f32 + slope23 * (y - y2) as f32),
                    f32_to_i32(x_split as f32 + slope13 * (y - y2) as f32),
                )
            };
            self.fill_row_with_dither(x_start, x_end, y, value);
        }
    }

    pub fn draw_triangle_border(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        value: T,
    ) {
        self.draw_line(x1, y1, x2, y2, value);
        self.draw_line(x1, y1, x3, y3, value);
        self.draw_line(x2, y2, x3, y3, value);
    }

    // Flood fill

    pub fn flood_fill(&mut self, x: f32, y: f32, value: T) {
        // Grow horizontal spans and enqueue adjacent scanline segments.
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        if !self.clip_rect.contains(x, y) {
            return;
        }

        let dst_value = self.read_data(x as usize, y as usize);
        if value == dst_value {
            return;
        }

        let mut visit_stack = Vec::with_capacity(64);
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

            if self.alpha >= 1.0 {
                let w = self.width() as usize;
                self.data[w * y as usize + left as usize..=w * y as usize + right as usize]
                    .fill(value);
            } else {
                for xi in left..=right {
                    self.write_data(xi as usize, y as usize, value);
                }
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

    // Blit operations

    pub fn blit(
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

        // Clip source and destination before choosing the row-copy path.
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

        macro_rules! copy_row {
            (slice: $dst:expr, $src:expr) => {
                if transparent.is_none() && palette.is_none() {
                    $dst.copy_from_slice($src);
                } else {
                    for i in 0..$dst.len() {
                        if let Some(val) = Self::apply_pixel($src[i], transparent, palette) {
                            $dst[i] = val;
                        }
                    }
                }
            };
            (rev: $dst:expr, $src_row:expr, $start:expr) => {
                if transparent.is_none() && palette.is_none() {
                    for i in 0..$dst.len() {
                        $dst[i] = $src_row[$start - i];
                    }
                } else {
                    for i in 0..$dst.len() {
                        if let Some(val) =
                            Self::apply_pixel($src_row[$start - i], transparent, palette)
                        {
                            $dst[i] = val;
                        }
                    }
                }
            };
        }

        // Fast path: no flip, full alpha
        if sign_x == 1 && sign_y == 1 && self.alpha >= 1.0 {
            let dst_w = self.width() as usize;
            let src_w = canvas.width() as usize;
            let width = width as usize;
            for yi in 0..height as usize {
                let di = dst_w * (dst_y as usize + yi) + dst_x as usize;
                let si = src_w * (src_y as usize + yi) + src_x as usize;
                let dst = &mut self.data[di..di + width];
                let src = &canvas.data[si..si + width];
                copy_row!(slice: dst, src);
            }
            return;
        }

        // Flip-only path: no dithering, row-level operations
        if self.alpha >= 1.0 {
            let dst_w = self.width() as usize;
            let src_w = canvas.width() as usize;
            let width_usize = width as usize;

            for yi in 0..height {
                let sy = (src_y + sign_y * yi + offset_y) as usize;
                let di_base = dst_w * (dst_y + yi) as usize + dst_x as usize;
                let dst = &mut self.data[di_base..di_base + width_usize];

                if sign_x == 1 {
                    // Y-flip only: source row is contiguous
                    let si = src_w * sy + src_x as usize;
                    let src = &canvas.data[si..si + width_usize];
                    copy_row!(slice: dst, src);
                } else {
                    // X-flip: reverse pixel order within row
                    let sx_start = (src_x + offset_x) as usize;
                    let src_row = &canvas.data[src_w * sy..];
                    copy_row!(rev: dst, src_row, sx_start);
                }
            }
            return;
        }

        // General path: dithering
        for yi in 0..height {
            for xi in 0..width {
                let value_x = src_x + sign_x * xi + offset_x;
                let value_y = src_y + sign_y * yi + offset_y;
                let value = canvas.read_data(value_x as usize, value_y as usize);
                if transparent.is_some_and(|tkey| value == tkey) {
                    continue;
                }
                let value = palette.map_or(value, |palette| palette[value.to_index()]);
                self.write_data((dst_x + xi) as usize, (dst_y + yi) as usize, value);
            }
        }
    }

    pub fn blit_with_transform(
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

        // Build inverse transform bounds before scanning destination pixels.
        let x = f32_to_i32(x) - self.camera_x;
        let y = f32_to_i32(y) - self.camera_y;
        let canvas_x = f32_to_i32(canvas_x);
        let canvas_y = f32_to_i32(canvas_y);
        let sign_x = if width < 0.0 { -1.0 } else { 1.0 };
        let sign_y = if height < 0.0 { -1.0 } else { 1.0 };
        let width = f32_to_i32(width).abs();
        let height = f32_to_i32(height).abs();

        let canvas_area = RectArea::new(canvas_x, canvas_y, width as u32, height as u32)
            .intersection(if use_canvas_clip {
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
        // Positive rotation is clockwise in screen space.
        let sin = -f32::sin(rotate);
        let cos = f32::cos(rotate);
        let bound_x = (half_width * cos.abs() + half_height * sin.abs() + 1.0) * scale;
        let bound_y = (half_width * sin.abs() + half_height * cos.abs() + 1.0) * scale;
        let x1 = f32_to_i32(dst_cx - bound_x).max(self.clip_rect.left());
        let x2 = f32_to_i32(dst_cx + bound_x).min(self.clip_rect.right());
        let y1 = f32_to_i32(dst_cy - bound_y).max(self.clip_rect.top());
        let y2 = f32_to_i32(dst_cy + bound_y).min(self.clip_rect.bottom());

        // Pre-compute per-pixel stepping
        let cos_s = cos / scale;
        let sin_s = sin / scale;
        let step_sx = sign_x * cos_s;
        let step_sy = sign_x * sin_s;

        // Fast path: no dithering
        if self.alpha >= 1.0 {
            let dst_w = self.width() as usize;
            let src_w = canvas.width() as usize;
            let ca_l = canvas_area.left();
            let ca_r = canvas_area.right();
            let ca_t = canvas_area.top();
            let ca_b = canvas_area.bottom();
            macro_rules! scan {
                (|$val:ident, $di:ident| $body:expr) => {
                    for yi in y1..=y2 {
                        let oy = (yi as f32 - dst_cy) * sign_y;
                        let ox0 = (x1 as f32 - dst_cx) * sign_x;
                        let mut sx = src_cx + ox0 * cos_s - oy * sin_s;
                        let mut sy = src_cy + ox0 * sin_s + oy * cos_s;
                        let di_row = dst_w * yi as usize;
                        for xi in x1..=x2 {
                            let vx = f32_to_i32(sx);
                            let vy = f32_to_i32(sy);
                            sx += step_sx;
                            sy += step_sy;
                            if vx >= ca_l && vx <= ca_r && vy >= ca_t && vy <= ca_b {
                                let $val = canvas.data[src_w * vy as usize + vx as usize];
                                let $di = di_row + xi as usize;
                                $body
                            }
                        }
                    }
                };
            }
            match (transparent, palette) {
                (None, None) => scan!(|val, di| {
                    self.data[di] = val;
                }),
                _ => scan!(|val, di| {
                    if let Some(v) = Self::apply_pixel(val, transparent, palette) {
                        self.data[di] = v;
                    }
                }),
            }
            return;
        }

        // Dithering path
        for yi in y1..=y2 {
            let oy = (yi as f32 - dst_cy) * sign_y;
            let ox0 = (x1 as f32 - dst_cx) * sign_x;
            let mut sx = src_cx + ox0 * cos_s - oy * sin_s;
            let mut sy = src_cy + ox0 * sin_s + oy * cos_s;
            for xi in x1..=x2 {
                let vx = f32_to_i32(sx);
                let vy = f32_to_i32(sy);
                sx += step_sx;
                sy += step_sy;
                if !canvas_area.contains(vx, vy) {
                    continue;
                }
                let value = canvas.read_data(vx as usize, vy as usize);
                if transparent.is_some_and(|tkey| value == tkey) {
                    continue;
                }
                let value = palette.map_or(value, |p| p[value.to_index()]);
                self.write_data(xi as usize, yi as usize, value);
            }
        }
    }

    pub fn blit_perspective(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        canvas: &Self,
        pos: (f32, f32, f32),
        rot: (f32, f32, f32),
        fov: Option<f32>,
        transparent: Option<T>,
        palette: Option<&[T]>,
    ) {
        let Some(proj) = PerspectiveProjection::new(
            x,
            y,
            width,
            height,
            self.camera_x,
            self.camera_y,
            pos,
            rot,
            fov,
        ) else {
            return;
        };

        let src_w = canvas.width() as i32;
        let src_h = canvas.height() as i32;
        let x1 = proj.dst_x.max(self.clip_rect.left());
        let x2 = (proj.dst_x + proj.w - 1).min(self.clip_rect.right());
        let y1 = proj.dst_y.max(self.clip_rect.top());
        let y2 = (proj.dst_y + proj.h - 1).min(self.clip_rect.bottom());

        let (wx_step, wy_step, wz_step) = proj.world_step_per_x();

        // Project destination pixels back through the camera plane.
        // Fast path: no dithering
        if self.alpha >= 1.0 {
            let dst_w = self.width() as usize;
            let src_wu = src_w as usize;
            macro_rules! scan {
                (|$val:ident, $di:ident| $body:expr) => {
                    for yi in y1..=y2 {
                        let (mut wx, mut wy, mut wz) = proj.world_base(x1, yi);
                        let di_row = dst_w * yi as usize;
                        for xi in x1..=x2 {
                            if wz.abs() >= f32::EPSILON {
                                let t = -proj.cam_z / wz;
                                if t > 0.0 {
                                    let sxi = f32_to_i32(proj.cam_x + t * wx);
                                    let syi = f32_to_i32(proj.cam_y + t * wy);
                                    if sxi >= 0 && sxi < src_w && syi >= 0 && syi < src_h {
                                        let $val =
                                            canvas.data[src_wu * syi as usize + sxi as usize];
                                        let $di = di_row + xi as usize;
                                        $body
                                    }
                                }
                            }
                            wx += wx_step;
                            wy += wy_step;
                            wz += wz_step;
                        }
                    }
                };
            }
            match (transparent, palette) {
                (None, None) => scan!(|val, di| {
                    self.data[di] = val;
                }),
                _ => scan!(|val, di| {
                    if let Some(v) = Self::apply_pixel(val, transparent, palette) {
                        self.data[di] = v;
                    }
                }),
            }
            return;
        }

        // Dithering path
        for yi in y1..=y2 {
            let (mut wx, mut wy, mut wz) = proj.world_base(x1, yi);
            for xi in x1..=x2 {
                if wz.abs() >= f32::EPSILON {
                    let t = -proj.cam_z / wz;
                    if t > 0.0 {
                        let src_xi = f32_to_i32(proj.cam_x + t * wx);
                        let src_yi = f32_to_i32(proj.cam_y + t * wy);
                        if src_xi >= 0 && src_xi < src_w && src_yi >= 0 && src_yi < src_h {
                            let value = canvas.read_data(src_xi as usize, src_yi as usize);
                            if transparent.is_none_or(|tkey| value != tkey) {
                                let value = palette.map_or(value, |p| p[value.to_index()]);
                                self.write_data(xi as usize, yi as usize, value);
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

    // Internal helpers

    #[inline]
    fn apply_pixel(src: T, transparent: Option<T>, palette: Option<&[T]>) -> Option<T> {
        if transparent.is_some_and(|tkey| src == tkey) {
            return None;
        }
        Some(palette.map_or(src, |pal| pal[src.to_index()]))
    }

    #[inline]
    pub fn read_data(&self, x: usize, y: usize) -> T {
        let width = self.width() as usize;
        self.data[width * y + x]
    }

    #[inline]
    pub fn write_data(&mut self, x: usize, y: usize, value: T) {
        if self.should_write(x as i32, y as i32) {
            let width = self.width() as usize;
            self.data[width * y + x] = value;
        }
    }

    #[inline]
    pub(crate) fn write_data_with_clipping(&mut self, x: i32, y: i32, value: T) {
        if self.clip_rect.contains(x, y) && self.should_write(x, y) {
            let width = self.width() as usize;
            self.data[width * y as usize + x as usize] = value;
        }
    }

    fn fill_row(&mut self, x1: i32, x2: i32, y: i32, value: T) {
        if y < self.clip_rect.top() || y > self.clip_rect.bottom() {
            return;
        }
        let left = x1.max(self.clip_rect.left());
        let right = x2.min(self.clip_rect.right());
        if left > right {
            return;
        }
        let w = self.width() as usize;
        let y = y as usize;
        self.data[w * y + left as usize..=w * y + right as usize].fill(value);
    }

    fn fill_row_with_dither(&mut self, x1: i32, x2: i32, y: i32, value: T) {
        if self.alpha >= 1.0 {
            self.fill_row(x1, x2, y, value);
            return;
        }
        if y < self.clip_rect.top() || y > self.clip_rect.bottom() {
            return;
        }
        let left = x1.max(self.clip_rect.left());
        let right = x2.min(self.clip_rect.right());
        if left > right {
            return;
        }
        for x in left..=right {
            self.write_data(x as usize, y as usize, value);
        }
    }

    fn fill_column(&mut self, y1: i32, y2: i32, x: i32, value: T) {
        if x < self.clip_rect.left() || x > self.clip_rect.right() {
            return;
        }
        let top = y1.max(self.clip_rect.top());
        let bottom = y2.min(self.clip_rect.bottom());
        if top > bottom {
            return;
        }
        let w = self.width() as usize;
        let x = x as usize;
        for data in self.data[w * top as usize + x..=w * bottom as usize + x]
            .iter_mut()
            .step_by(w)
        {
            *data = value;
        }
    }

    fn fill_column_with_dither(&mut self, y1: i32, y2: i32, x: i32, value: T) {
        if self.alpha >= 1.0 {
            self.fill_column(y1, y2, x, value);
            return;
        }
        if x < self.clip_rect.left() || x > self.clip_rect.right() {
            return;
        }
        let top = y1.max(self.clip_rect.top());
        let bottom = y2.min(self.clip_rect.bottom());
        for y in top..=bottom {
            self.write_data(x as usize, y as usize, value);
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

        let x1 = f32_to_i32(cx - dx - ELLIPSE_ROUNDING_BIAS);
        let y1 = f32_to_i32(cy - dy - ELLIPSE_ROUNDING_BIAS);
        let x2 = f32_to_i32(cx + dx + ELLIPSE_ROUNDING_BIAS);
        let y2 = f32_to_i32(cy + dy + ELLIPSE_ROUNDING_BIAS);

        (x1, y1, x2, y2)
    }

    fn should_write(&self, x: i32, y: i32) -> bool {
        if self.alpha >= 1.0 {
            return true;
        }
        if self.alpha <= 0.0 {
            return false;
        }
        self.alpha > DITHERING_MATRIX[(y & 3) as usize][(x & 3) as usize]
    }
}

// Copy clipping

pub(crate) struct CopyArea {
    pub(crate) dst_x: i32,
    pub(crate) dst_y: i32,
    pub(crate) src_x: i32,
    pub(crate) src_y: i32,
    pub(crate) sign_x: i32,
    pub(crate) sign_y: i32,
    pub(crate) offset_x: i32,
    pub(crate) offset_y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl CopyArea {
    pub(crate) fn new(
        dst_x: i32,
        dst_y: i32,
        dst_rect: RectArea,
        src_x: i32,
        src_y: i32,
        src_rect: RectArea,
        width: i32,
        height: i32,
    ) -> Self {
        // Clip flipped copies against source and destination rectangles.
        let flip_x = width < 0;
        let flip_y = height < 0;
        let width = width.abs();
        let height = height.abs();

        let src_left_cut = src_rect.left() - src_x;
        let src_top_cut = src_rect.top() - src_y;
        let src_right_cut = src_x + width - 1 - src_rect.right();
        let src_bottom_cut = src_y + height - 1 - src_rect.bottom();

        // A flipped blit reads the source backwards, so a source overhang
        // trims the opposite edge of the copy window
        let left_cut = (dst_rect.left() - dst_x)
            .max(if flip_x { src_right_cut } else { src_left_cut })
            .max(0);
        let top_cut = (dst_rect.top() - dst_y)
            .max(if flip_y { src_bottom_cut } else { src_top_cut })
            .max(0);
        let right_cut = (dst_x + width - 1 - dst_rect.right())
            .max(if flip_x { src_left_cut } else { src_right_cut })
            .max(0);
        let bottom_cut = (dst_y + height - 1 - dst_rect.bottom())
            .max(if flip_y { src_top_cut } else { src_bottom_cut })
            .max(0);

        let width = (width - left_cut - right_cut).max(0);
        let height = (height - top_cut - bottom_cut).max(0);
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

// Perspective projection

pub(crate) struct PerspectiveProjection {
    pub(crate) cam_x: f32,
    pub(crate) cam_y: f32,
    pub(crate) cam_z: f32,
    r00: f32,
    r01: f32,
    r02: f32,
    r10: f32,
    r11: f32,
    r12: f32,
    r21: f32,
    r22: f32,
    sin_z: f32,
    cos_z: f32,
    tan_hfov: f32,
    aspect: f32,
    half_width: f32,
    half_height: f32,
    pub(crate) dst_x: i32,
    pub(crate) dst_y: i32,
    pub(crate) w: i32,
    pub(crate) h: i32,
}

impl PerspectiveProjection {
    pub(crate) fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        offset_x: i32,
        offset_y: i32,
        pos: (f32, f32, f32),
        rot: (f32, f32, f32),
        fov: Option<f32>,
    ) -> Option<Self> {
        // Build the camera transform used by perspective blits.
        let (cam_x, cam_y, cam_z) = pos;
        if cam_z.abs() < f32::EPSILON {
            return None;
        }

        let w = f32_to_i32(width);
        let h = f32_to_i32(height);
        if w <= 0 || h <= 0 {
            return None;
        }

        let tan_hfov = (fov.unwrap_or(60.0) * PI / 360.0).tan();
        if !tan_hfov.is_finite() || tan_hfov.abs() < f32::EPSILON {
            return None;
        }

        let (rot_x, rot_y, rot_z) = rot;
        let rot_x = rot_x * PI / 180.0;
        let rot_y = rot_y * PI / 180.0;
        let rot_z = rot_z * PI / 180.0;

        let (sx, cx) = (rot_x.sin(), rot_x.cos());
        let (sy, cy) = (rot_y.sin(), rot_y.cos());
        let (sz, cz) = (rot_z.sin(), rot_z.cos());

        // R_z(rot_y) * diag(1,-1,1) * R_x(rot_x)
        // rot_y=0 looks down -Z with screen-right=+X, screen-down=+Y (matches 2D)
        // rot_z is applied in view space before the world transform
        //
        //   r00 = cos(rot_y),   r01 = sin(rot_y)*cos(rot_x),   r02 = -sin(rot_y)*sin(rot_x)
        //   r10 = sin(rot_y),   r11 = -cos(rot_y)*cos(rot_x),  r12 = cos(rot_y)*sin(rot_x)
        //   r20 = 0,            r21 = sin(rot_x),               r22 = cos(rot_x)

        Some(Self {
            cam_x,
            cam_y,
            cam_z,
            r00: cy,
            r01: sy * cx,
            r02: -sy * sx,
            r10: sy,
            r11: -cy * cx,
            r12: cy * sx,
            r21: sx,
            r22: cx,
            sin_z: sz,
            cos_z: cz,
            tan_hfov,
            aspect: w as f32 / h as f32,
            half_width: w as f32 / 2.0,
            half_height: h as f32 / 2.0,
            dst_x: f32_to_i32(x) - offset_x,
            dst_y: f32_to_i32(y) - offset_y,
            w,
            h,
        })
    }

    // Per-pixel step values (constant for all xi, yi)
    pub(crate) fn world_step_per_x(&self) -> (f32, f32, f32) {
        let vx_step = self.tan_hfov * self.aspect / self.half_width;
        let vx2_step = vx_step * self.cos_z;
        let vy2_step = -vx_step * self.sin_z;
        let wx_step = self.r00 * vx2_step + self.r01 * vy2_step;
        let wy_step = self.r10 * vx2_step + self.r11 * vy2_step;
        let wz_step = self.r21 * vy2_step;
        (wx_step, wy_step, wz_step)
    }

    // Base world-space values for a given (xi, yi)
    pub(crate) fn world_base(&self, xi: i32, yi: i32) -> (f32, f32, f32) {
        let ndc_x = ((xi - self.dst_x) as f32 + 0.5 - self.half_width) / self.half_width;
        let ndc_y = ((yi - self.dst_y) as f32 + 0.5 - self.half_height) / self.half_height;
        let vx = ndc_x * self.tan_hfov * self.aspect;
        let vy = -ndc_y * self.tan_hfov;
        let vx2 = vx * self.cos_z + vy * self.sin_z;
        let vy2 = -vx * self.sin_z + vy * self.cos_z;
        (
            self.r00 * vx2 + self.r01 * vy2 - self.r02,
            self.r10 * vx2 + self.r11 * vy2 - self.r12,
            self.r21 * vy2 - self.r22,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::Color;

    // CopyArea clipping

    #[test]
    fn test_copy_area_unflipped_overhangs() {
        let rect = RectArea::new(0, 0, 16, 16);

        // Source overhang on the right shrinks the window in place
        let area = CopyArea::new(0, 0, rect, 12, 0, rect, 8, 1);
        assert_eq!((area.dst_x, area.src_x, area.width), (0, 12, 4));

        // Destination overhang on the left advances both windows
        let area = CopyArea::new(-3, 0, rect, 0, 0, rect, 8, 1);
        assert_eq!((area.dst_x, area.src_x, area.width), (0, 3, 5));
    }

    #[test]
    fn test_copy_area_flip_source_overhang() {
        // Source columns 12-19 overhang a 16-wide source by 4; with flip_x the
        // lost columns must disappear from the LEFT of the destination window
        let rect = RectArea::new(0, 0, 16, 16);
        let area = CopyArea::new(0, 0, rect, 12, 0, rect, -8, 1);
        assert_eq!((area.dst_x, area.src_x, area.width), (4, 12, 4));
        // Reads run right-to-left from the last kept source column (15)
        assert_eq!((area.sign_x, area.offset_x), (-1, 3));
    }

    #[test]
    fn test_copy_area_flip_dst_overhang() {
        // Destination overhang on the left trims the source's RIGHT edge
        let rect = RectArea::new(0, 0, 16, 16);
        let area = CopyArea::new(-3, 0, rect, 0, 0, rect, -8, 1);
        assert_eq!((area.dst_x, area.src_x, area.width), (0, 0, 5));
        assert_eq!((area.sign_x, area.offset_x), (-1, 4));
    }

    // Degenerate draw inputs

    #[test]
    fn test_draw_ellipse_zero_size_draws_nothing() {
        let mut canvas: Canvas<Color> = Canvas::new(16, 16);
        canvas.draw_ellipse(4.0, 4.0, 0.0, 5.0, 7);
        canvas.draw_ellipse_border(4.0, 4.0, 5.0, 0.0, 7);
        assert!(canvas.data.iter().all(|&value| value == 0));
    }

    #[test]
    fn test_draw_triangle_collinear_horizontal() {
        // All vertices on one row must fill the full extent including x3
        let mut canvas: Canvas<Color> = Canvas::new(24, 8);
        canvas.draw_triangle(0.0, 3.0, 10.0, 3.0, 20.0, 3.0, 7);
        for x in 0..=20 {
            assert_eq!(canvas.read_data(x, 3), 7, "x={x}");
        }
        assert_eq!(canvas.read_data(21, 3), 0);
    }
}
