use std::f32::consts::PI;
use std::mem::swap;

use crate::rect_area::RectArea;
use crate::utils::{f32_to_i32, f32_to_u32};

// Bias curve intersections outward before rounding to pixel coordinates.
const ELLIPSE_ROUNDING_BIAS: f32 = 0.01;

// Dithering uses eight thresholds repeated over a 4x4 footprint.
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
        Self::try_new(width, height).expect("width and height are too large")
    }

    pub(crate) fn try_new(width: u32, height: u32) -> Option<Self> {
        let len = width.checked_mul(height)? as usize;
        Some(Self {
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
            camera_x: 0,
            camera_y: 0,
            alpha: 1.0,
            data: vec![T::default(); len],
        })
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
        let x = i64::from(f32_to_i32(x)) - i64::from(self.camera_x);
        let y = i64::from(f32_to_i32(y)) - i64::from(self.camera_y);
        self.write_data_with_clipping_i64(x, y, value);
    }

    // Drawing primitives

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, value: T) {
        let x1 = i64::from(f32_to_i32(x1)) - i64::from(self.camera_x);
        let y1 = i64::from(f32_to_i32(y1)) - i64::from(self.camera_y);
        let x2 = i64::from(f32_to_i32(x2)) - i64::from(self.camera_x);
        let y2 = i64::from(f32_to_i32(y2)) - i64::from(self.camera_y);

        // Preserve ordinary f32 raster rounding; large offsets need pixel-sized steps.
        if [x1, y1, x2, y2].iter().any(|value| value.abs() > 1 << 23) {
            let left = i64::from(self.clip_rect.left());
            let right = i64::from(self.clip_rect.right());
            let top = i64::from(self.clip_rect.top());
            let bottom = i64::from(self.clip_rect.bottom());

            if (x1 - x2).abs() > (y1 - y2).abs() {
                let (start_x, start_y, end_x, end_y) = if x1 < x2 {
                    (x1, y1, x2, y2)
                } else {
                    (x2, y2, x1, y1)
                };
                let slope = (end_y - start_y) as f64 / (end_x - start_x) as f64;

                for x in start_x.max(left)..=end_x.min(right) {
                    let y = start_y + (slope * (x - start_x) as f64).round() as i64;
                    if (top..=bottom).contains(&y) {
                        self.write_data(x as usize, y as usize, value);
                    }
                }
            } else if y1 != y2 {
                let (start_x, start_y, end_x, end_y) = if y1 < y2 {
                    (x1, y1, x2, y2)
                } else {
                    (x2, y2, x1, y1)
                };
                let slope = (end_x - start_x) as f64 / (end_y - start_y) as f64;

                for y in start_y.max(top)..=end_y.min(bottom) {
                    let x = start_x + (slope * (y - start_y) as f64).round() as i64;
                    if (left..=right).contains(&x) {
                        self.write_data(x as usize, y as usize, value);
                    }
                }
            } else if (left..=right).contains(&x1) && (top..=bottom).contains(&y1) {
                self.write_data(x1 as usize, y1 as usize, value);
            }
            return;
        }

        let (x1, y1, x2, y2) = (x1 as i32, y1 as i32, x2 as i32, y2 as i32);
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

            for xi in self.clip_rect.left().saturating_sub(start_x).max(0)
                ..=self
                    .clip_rect
                    .right()
                    .saturating_sub(start_x)
                    .min(length - 1)
            {
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

            for yi in self.clip_rect.top().saturating_sub(start_y).max(0)
                ..=self
                    .clip_rect
                    .bottom()
                    .saturating_sub(start_y)
                    .min(length - 1)
            {
                self.write_data_with_clipping(
                    start_x + f32_to_i32(slope * yi as f32),
                    start_y + yi,
                    value,
                );
            }
        }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        let x = i64::from(f32_to_i32(x)) - i64::from(self.camera_x);
        let y = i64::from(f32_to_i32(y)) - i64::from(self.camera_y);
        let width = i64::from(f32_to_u32(width));
        let height = i64::from(f32_to_u32(height));

        let left = x.max(i64::from(self.clip_rect.left()));
        let top = y.max(i64::from(self.clip_rect.top()));
        let right = (x + width - 1).min(i64::from(self.clip_rect.right()));
        let bottom = (y + height - 1).min(i64::from(self.clip_rect.bottom()));
        if width == 0 || height == 0 || left > right || top > bottom {
            return;
        }

        let (left, top, right, bottom) =
            (left as usize, top as usize, right as usize, bottom as usize);
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
        let x = i64::from(f32_to_i32(x)) - i64::from(self.camera_x);
        let y = i64::from(f32_to_i32(y)) - i64::from(self.camera_y);
        let width = i64::from(f32_to_u32(width));
        let height = i64::from(f32_to_u32(height));
        if width == 0 || height == 0 {
            return;
        }

        let right = x + width - 1;
        let bottom = y + height - 1;
        self.fill_row_with_dither_i64(x, right, y, value);
        self.fill_row_with_dither_i64(x, right, bottom, value);
        self.fill_column_with_dither_i64(y, bottom, x, value);
        self.fill_column_with_dither_i64(y, bottom, right, value);
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, value: T) {
        self.draw_circle_impl::<false>(x, y, radius, value);
    }

    pub fn draw_circle_border(&mut self, x: f32, y: f32, radius: f32, value: T) {
        self.draw_circle_impl::<true>(x, y, radius, value);
    }

    pub fn draw_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        self.draw_ellipse_impl::<false>(x, y, width, height, value);
    }

    pub fn draw_ellipse_border(&mut self, x: f32, y: f32, width: f32, height: f32, value: T) {
        self.draw_ellipse_impl::<true>(x, y, width, height, value);
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
        let x1 = i64::from(f32_to_i32(x1)) - i64::from(self.camera_x);
        let y1 = i64::from(f32_to_i32(y1)) - i64::from(self.camera_y);
        let x2 = i64::from(f32_to_i32(x2)) - i64::from(self.camera_x);
        let y2 = i64::from(f32_to_i32(y2)) - i64::from(self.camera_y);
        let x3 = i64::from(f32_to_i32(x3)) - i64::from(self.camera_x);
        let y3 = i64::from(f32_to_i32(y3)) - i64::from(self.camera_y);
        if [x1, y1, x2, y2, x3, y3]
            .iter()
            .any(|value| value.abs() > 1 << 23)
        {
            self.draw_triangle_wide([(x1, y1), (x2, y2), (x3, y3)], value);
            return;
        }

        let (mut x1, mut y1, mut x2, mut y2, mut x3, mut y3) = (
            x1 as i32, y1 as i32, x2 as i32, y2 as i32, x3 as i32, y3 as i32,
        );
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
        let slope13 = (x3 - x1) as f32 / (y3 - y1) as f32;
        let slope23 = if y3 == y2 {
            0.0
        } else {
            (x3 - x2) as f32 / (y3 - y2) as f32
        };
        let x_split = f32_to_i32(x1 as f32 + slope13 * (y2 - y1) as f32);

        for y in y1.max(self.clip_rect.top())..=y2.min(self.clip_rect.bottom()) {
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

        for y in (y2 + 1).max(self.clip_rect.top())..=y3.min(self.clip_rect.bottom()) {
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

    pub fn flood_fill(&mut self, x: f32, y: f32, value: T) {
        let Some(x) = f32_to_i32(x).checked_sub(self.camera_x) else {
            return;
        };
        let Some(y) = f32_to_i32(y).checked_sub(self.camera_y) else {
            return;
        };
        if !self.clip_rect.contains(x, y) || self.alpha <= 0.0 || self.alpha.is_nan() {
            return;
        }

        let dst_value = self.read_data(x as usize, y as usize);
        if value == dst_value {
            return;
        }

        let mut visit_stack = Vec::with_capacity(64);
        let mut filled_spans = Vec::new();
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

            // Solid spans mark visited pixels even where dithering will leave holes.
            let w = self.width() as usize;
            self.data[w * y as usize + left as usize..=w * y as usize + right as usize].fill(value);
            if self.alpha < 1.0 {
                filled_spans.push((left, right, y));
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

        for (left, right, y) in filled_spans {
            for x in left..=right {
                if !self.should_write(x, y) {
                    let index = self.width() as usize * y as usize + x as usize;
                    self.data[index] = dst_value;
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
        let Some(x) = f32_to_i32(x).checked_sub(self.camera_x) else {
            return;
        };
        let Some(y) = f32_to_i32(y).checked_sub(self.camera_y) else {
            return;
        };
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
                if let Some(value) = Self::apply_pixel(value, transparent, palette) {
                    self.write_data((dst_x + xi) as usize, (dst_y + yi) as usize, value);
                }
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
    ) {
        let Some(proj) = TransformProjection::new(
            x,
            y,
            canvas_x,
            canvas_y,
            width,
            height,
            self.camera_x,
            self.camera_y,
            rotate,
            scale,
            self.clip_rect,
        ) else {
            return;
        };

        let canvas_area = RectArea::new(proj.src_x, proj.src_y, proj.width, proj.height)
            .intersection(canvas.self_rect);
        if canvas_area.is_empty() {
            return;
        }
        let (step_sx, step_sy) = proj.src_step_per_x();

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
                    for yi in proj.y1..=proj.y2 {
                        let (mut sx, mut sy) = proj.src_base(proj.x1, yi);
                        let di_row = dst_w * yi as usize;
                        for xi in proj.x1..=proj.x2 {
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
        for yi in proj.y1..=proj.y2 {
            let (mut sx, mut sy) = proj.src_base(proj.x1, yi);
            for xi in proj.x1..=proj.x2 {
                let vx = f32_to_i32(sx);
                let vy = f32_to_i32(sy);
                sx += step_sx;
                sy += step_sy;
                if !canvas_area.contains(vx, vy) {
                    continue;
                }

                let value = canvas.read_data(vx as usize, vy as usize);
                if let Some(value) = Self::apply_pixel(value, transparent, palette) {
                    self.write_data(xi as usize, yi as usize, value);
                }
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
        let x2 = proj
            .dst_x
            .saturating_add(proj.w - 1)
            .min(self.clip_rect.right());
        let y1 = proj.dst_y.max(self.clip_rect.top());
        let y2 = proj
            .dst_y
            .saturating_add(proj.h - 1)
            .min(self.clip_rect.bottom());
        let (wx_step, wy_step, wz_step) = proj.world_step_per_x();

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
                            if let Some(value) = Self::apply_pixel(value, transparent, palette) {
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

    fn draw_circle_impl<const BORDER: bool>(&mut self, x: f32, y: f32, radius: f32, value: T) {
        let x = i64::from(f32_to_i32(x)) - i64::from(self.camera_x);
        let y = i64::from(f32_to_i32(y)) - i64::from(self.camera_y);
        let radius = i64::from(f32_to_u32(radius));
        if self.clip_rect.is_empty()
            || x + radius < i64::from(self.clip_rect.left())
            || x - radius > i64::from(self.clip_rect.right())
            || y + radius < i64::from(self.clip_rect.top())
            || y - radius > i64::from(self.clip_rect.bottom())
        {
            return;
        }

        let x_range =
            Self::circle_axis_range(x, self.clip_rect.left(), self.clip_rect.right(), radius);
        let y_range =
            Self::circle_axis_range(y, self.clip_rect.top(), self.clip_rect.bottom(), radius);
        let wide = radius >= 1 << 22;

        // Every symmetric point or span has a coordinate at center +/- xi.
        for (start, end) in Self::merge_ranges([x_range, y_range]) {
            for xi in start..=end {
                let (x1, y1, x2, y2) =
                    Self::ellipse_area_i64(0.0, 0.0, radius as f64, radius as f64, xi, wide);
                if BORDER {
                    for (px, py) in [
                        (x1, y1),
                        (x2, y1),
                        (x1, y2),
                        (x2, y2),
                        (y1, x1),
                        (y1, x2),
                        (y2, x1),
                        (y2, x2),
                    ] {
                        self.write_data_with_clipping_i64(x + px, y + py, value);
                    }
                } else {
                    self.fill_column_with_dither_i64(y + y1, y + y2, x + x1, value);
                    self.fill_column_with_dither_i64(y + y1, y + y2, x + x2, value);
                    self.fill_row_with_dither_i64(x + y1, x + y2, y + x1, value);
                    self.fill_row_with_dither_i64(x + y1, x + y2, y + x2, value);
                }
            }
        }
    }

    fn draw_ellipse_impl<const BORDER: bool>(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        value: T,
    ) {
        let x = i64::from(f32_to_i32(x)) - i64::from(self.camera_x);
        let y = i64::from(f32_to_i32(y)) - i64::from(self.camera_y);
        let width = i64::from(f32_to_u32(width));
        let height = i64::from(f32_to_u32(height));
        let right = x + width - 1;
        let bottom = y + height - 1;
        if width == 0
            || height == 0
            || self.clip_rect.is_empty()
            || right < i64::from(self.clip_rect.left())
            || x > i64::from(self.clip_rect.right())
            || bottom < i64::from(self.clip_rect.top())
            || y > i64::from(self.clip_rect.bottom())
        {
            return;
        }

        // Half-pixel centers must remain representable in the ordinary f32 path.
        let wide = [x, y, right, bottom]
            .iter()
            .any(|value| value.abs() >= 1 << 22);
        let (ra, rb, cx, cy) = if wide {
            let ra = (width - 1) as f64 / 2.0;
            let rb = (height - 1) as f64 / 2.0;
            (ra, rb, x as f64 + ra, y as f64 + rb)
        } else {
            let (ra, rb, cx, cy) =
                Self::ellipse_params(x as i32, y as i32, width as u32, height as u32);
            (f64::from(ra), f64::from(rb), f64::from(cx), f64::from(cy))
        };

        for (start, end) in Self::ellipse_axis_ranges(
            x,
            x + width / 2,
            x + right,
            self.clip_rect.left(),
            self.clip_rect.right(),
        ) {
            for xi in start..=end {
                let (x1, y1, x2, y2) = Self::ellipse_area_i64(cx, cy, ra, rb, xi, wide);
                if BORDER {
                    for (px, py) in [(x1, y1), (x2, y1), (x1, y2), (x2, y2)] {
                        self.write_data_with_clipping_i64(px, py, value);
                    }
                } else {
                    self.fill_column_with_dither_i64(y1, y2, x1, value);
                    self.fill_column_with_dither_i64(y1, y2, x2, value);
                }
            }
        }

        for (start, end) in Self::ellipse_axis_ranges(
            y,
            y + height / 2,
            y + bottom,
            self.clip_rect.top(),
            self.clip_rect.bottom(),
        ) {
            for yi in start..=end {
                let (y1, x1, y2, x2) = Self::ellipse_area_i64(cy, cx, rb, ra, yi, wide);
                if BORDER {
                    for (px, py) in [(x1, y1), (x1, y2), (x2, y1), (x2, y2)] {
                        self.write_data_with_clipping_i64(px, py, value);
                    }
                } else {
                    self.fill_row_with_dither_i64(x1, x2, y1, value);
                    self.fill_row_with_dither_i64(x1, x2, y2, value);
                }
            }
        }
    }

    fn circle_axis_range(center: i64, low: i32, high: i32, radius: i64) -> (i64, i64) {
        let low = i64::from(low) - center;
        let high = i64::from(high) - center;
        let start = if low <= 0 && high >= 0 {
            0
        } else {
            low.abs().min(high.abs())
        };
        (start, low.abs().max(high.abs()).min(radius))
    }

    fn ellipse_axis_ranges(
        start: i64,
        end: i64,
        mirror: i64,
        low: i32,
        high: i32,
    ) -> [(i64, i64); 2] {
        let low = i64::from(low);
        let high = i64::from(high);
        Self::merge_ranges([
            (start.max(low), end.min(high)),
            (start.max(mirror - high), end.min(mirror - low)),
        ])
    }

    fn merge_ranges(mut ranges: [(i64, i64); 2]) -> [(i64, i64); 2] {
        if ranges[0].0 > ranges[0].1 {
            return [ranges[1], (1, 0)];
        }
        if ranges[1].0 > ranges[1].1 {
            return [ranges[0], (1, 0)];
        }

        if ranges[0].0 > ranges[1].0 {
            ranges.swap(0, 1);
        }
        if ranges[0].1 + 1 >= ranges[1].0 {
            ranges[0].1 = ranges[0].1.max(ranges[1].1);
            ranges[1] = (1, 0);
        }
        ranges
    }

    fn ellipse_area_i64(
        cx: f64,
        cy: f64,
        ra: f64,
        rb: f64,
        x: i64,
        wide: bool,
    ) -> (i64, i64, i64, i64) {
        if !wide {
            let (x1, y1, x2, y2) =
                Self::ellipse_area(cx as f32, cy as f32, ra as f32, rb as f32, x as i32);
            return (i64::from(x1), i64::from(y1), i64::from(x2), i64::from(y2));
        }

        let dx = x as f64 - cx;
        let dy = if ra > 0.0 {
            rb * (1.0 - dx * dx / (ra * ra)).sqrt()
        } else {
            rb
        };
        let bias = f64::from(ELLIPSE_ROUNDING_BIAS);
        (
            (cx - dx - bias).round() as i64,
            (cy - dy - bias).round() as i64,
            (cx + dx + bias).round() as i64,
            (cy + dy + bias).round() as i64,
        )
    }

    pub(crate) fn write_data_with_clipping_i64(&mut self, x: i64, y: i64, value: T) {
        if (i64::from(self.clip_rect.left())..=i64::from(self.clip_rect.right())).contains(&x)
            && (i64::from(self.clip_rect.top())..=i64::from(self.clip_rect.bottom())).contains(&y)
        {
            self.write_data(x as usize, y as usize, value);
        }
    }

    fn fill_column_with_dither_i64(&mut self, y1: i64, y2: i64, x: i64, value: T) {
        let top = y1.max(i64::from(self.clip_rect.top()));
        let bottom = y2.min(i64::from(self.clip_rect.bottom()));
        if top <= bottom
            && (i64::from(self.clip_rect.left())..=i64::from(self.clip_rect.right())).contains(&x)
        {
            self.fill_column_with_dither(top as i32, bottom as i32, x as i32, value);
        }
    }

    fn draw_triangle_wide(&mut self, mut points: [(i64, i64); 3], value: T) {
        points.sort_by_key(|point| point.1);
        let [(x1, y1), (x2, y2), (x3, y3)] = points;
        if y1 == y3 {
            self.fill_row_with_dither_i64(x1.min(x2).min(x3), x1.max(x2).max(x3), y1, value);
            return;
        }

        let slope12 = if y2 == y1 {
            0.0
        } else {
            (x2 - x1) as f64 / (y2 - y1) as f64
        };
        let slope13 = (x3 - x1) as f64 / (y3 - y1) as f64;
        let slope23 = if y3 == y2 {
            0.0
        } else {
            (x3 - x2) as f64 / (y3 - y2) as f64
        };
        let x_split = (x1 as f64 + slope13 * (y2 - y1) as f64).round() as i64;
        let top = i64::from(self.clip_rect.top());
        let bottom = i64::from(self.clip_rect.bottom());

        for (start, end, slope) in [(y1, y2, slope12), (y2 + 1, y3, slope23)] {
            for y in start.max(top)..=end.min(bottom) {
                let x_a = (x_split as f64 + slope13 * (y - y2) as f64).round() as i64;
                let x_b = (x2 as f64 + slope * (y - y2) as f64).round() as i64;
                self.fill_row_with_dither_i64(x_a.min(x_b), x_a.max(x_b), y, value);
            }
        }
    }

    fn fill_row_with_dither_i64(&mut self, x1: i64, x2: i64, y: i64, value: T) {
        let left = x1.max(i64::from(self.clip_rect.left()));
        let right = x2.min(i64::from(self.clip_rect.right()));
        if left <= right
            && (i64::from(self.clip_rect.top())..=i64::from(self.clip_rect.bottom())).contains(&y)
        {
            self.fill_row_with_dither(left as i32, right as i32, y as i32, value);
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
        let flip_x = width < 0;
        let flip_y = height < 0;
        let width = i64::from(width).abs();
        let height = i64::from(height).abs();
        let (dst_x, dst_y, src_x, src_y) = (
            i64::from(dst_x),
            i64::from(dst_y),
            i64::from(src_x),
            i64::from(src_y),
        );

        let src_left_cut = i64::from(src_rect.left()) - src_x;
        let src_top_cut = i64::from(src_rect.top()) - src_y;
        let src_right_cut = src_x + width - 1 - i64::from(src_rect.right());
        let src_bottom_cut = src_y + height - 1 - i64::from(src_rect.bottom());

        // A flipped blit reads the source backwards, so a source overhang
        // trims the opposite edge of the copy window
        let left_cut = (i64::from(dst_rect.left()) - dst_x)
            .max(if flip_x { src_right_cut } else { src_left_cut })
            .max(0);
        let top_cut = (i64::from(dst_rect.top()) - dst_y)
            .max(if flip_y { src_bottom_cut } else { src_top_cut })
            .max(0);
        let right_cut = (dst_x + width - 1 - i64::from(dst_rect.right()))
            .max(if flip_x { src_left_cut } else { src_right_cut })
            .max(0);
        let bottom_cut = (dst_y + height - 1 - i64::from(dst_rect.bottom()))
            .max(if flip_y { src_top_cut } else { src_bottom_cut })
            .max(0);

        let width = (width - left_cut - right_cut).max(0);
        let height = (height - top_cut - bottom_cut).max(0);
        let (sign_x, offset_x) = if flip_x { (-1, width - 1) } else { (1, 0) };
        let (sign_y, offset_y) = if flip_y { (-1, height - 1) } else { (1, 0) };

        Self {
            dst_x: (dst_x + left_cut) as i32,
            dst_y: (dst_y + top_cut) as i32,
            src_x: (src_x + if flip_x { right_cut } else { left_cut }) as i32,
            src_y: (src_y + if flip_y { bottom_cut } else { top_cut }) as i32,
            sign_x,
            sign_y,
            offset_x: offset_x as i32,
            offset_y: offset_y as i32,
            width: width as i32,
            height: height as i32,
        }
    }
}

pub(crate) struct TransformProjection {
    src_cx: f32,
    src_cy: f32,
    dst_cx: f32,
    dst_cy: f32,
    sign_x: f32,
    sign_y: f32,
    cos_s: f32,
    sin_s: f32,

    pub(crate) src_x: i32,
    pub(crate) src_y: i32,
    pub(crate) width: u32,
    pub(crate) height: u32,

    pub(crate) x1: i32,
    pub(crate) x2: i32,
    pub(crate) y1: i32,
    pub(crate) y2: i32,
}

impl TransformProjection {
    pub(crate) fn new(
        x: f32,
        y: f32,
        src_x: f32,
        src_y: f32,
        width: f32,
        height: f32,
        offset_x: i32,
        offset_y: i32,
        rotate: f32,
        scale: f32,
        clip_rect: RectArea,
    ) -> Option<Self> {
        if scale < f32::EPSILON {
            return None;
        }

        let x = i64::from(f32_to_i32(x)) - i64::from(offset_x);
        let y = i64::from(f32_to_i32(y)) - i64::from(offset_y);
        let src_x = f32_to_i32(src_x);
        let src_y = f32_to_i32(src_y);
        let sign_x = if width < 0.0 { -1.0 } else { 1.0 };
        let sign_y = if height < 0.0 { -1.0 } else { 1.0 };
        let width = f32_to_i32(width).unsigned_abs();
        let height = f32_to_i32(height).unsigned_abs();
        if width == 0 || height == 0 {
            return None;
        }

        let half_width = (width - 1) as f32 / 2.0;
        let half_height = (height - 1) as f32 / 2.0;
        let src_cx = src_x as f32 + half_width;
        let src_cy = src_y as f32 + half_height;
        let dst_cx = x as f32 + half_width;
        let dst_cy = y as f32 + half_height;

        let rotate = rotate * PI / 180.0;
        // Positive rotation is clockwise in screen space.
        let sin = -f32::sin(rotate);
        let cos = f32::cos(rotate);
        let bound_x = (half_width * cos.abs() + half_height * sin.abs() + 1.0) * scale;
        let bound_y = (half_width * sin.abs() + half_height * cos.abs() + 1.0) * scale;

        Some(Self {
            src_cx,
            src_cy,
            dst_cx,
            dst_cy,
            sign_x,
            sign_y,
            cos_s: cos / scale,
            sin_s: sin / scale,

            src_x,
            src_y,
            width,
            height,

            x1: f32_to_i32(dst_cx - bound_x).max(clip_rect.left()),
            x2: f32_to_i32(dst_cx + bound_x).min(clip_rect.right()),
            y1: f32_to_i32(dst_cy - bound_y).max(clip_rect.top()),
            y2: f32_to_i32(dst_cy + bound_y).min(clip_rect.bottom()),
        })
    }

    // Per-pixel step values (constant for all xi, yi)
    pub(crate) fn src_step_per_x(&self) -> (f32, f32) {
        (self.sign_x * self.cos_s, self.sign_x * self.sin_s)
    }

    // Base source-space position for a given (xi, yi)
    pub(crate) fn src_base(&self, xi: i32, yi: i32) -> (f32, f32) {
        let ox = (xi as f32 - self.dst_cx) * self.sign_x;
        let oy = (yi as f32 - self.dst_cy) * self.sign_y;
        (
            self.src_cx + ox * self.cos_s - oy * self.sin_s,
            self.src_cy + ox * self.sin_s + oy * self.cos_s,
        )
    }
}

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

            dst_x: f32_to_i32(x).checked_sub(offset_x)?,
            dst_y: f32_to_i32(y).checked_sub(offset_y)?,
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

    #[test]
    fn test_copy_area_handles_minimum_signed_width() {
        let rect = RectArea::new(0, 0, 8, 8);
        let area = CopyArea::new(0, 0, rect, i32::MIN + 1, 0, rect, i32::MIN, 1);
        assert_eq!((area.dst_x, area.src_x, area.width), (0, 0, 1));
        assert_eq!((area.sign_x, area.offset_x), (-1, 0));
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

    // Large coordinates, camera offsets, and clipping

    #[test]
    fn test_large_line_half_ties_are_relative_to_start() {
        let cases = [
            (
                (
                    -2_000_000_000.0,
                    -1_000_000_000.0,
                    2_000_000_000.0,
                    1_000_000_000.0,
                ),
                (0.0, 1.0),
                vec![(1, 0), (2, 0), (3, 1), (4, 1), (5, 2), (6, 2), (7, 3)],
            ),
            (
                (
                    -2_000_000_000.0,
                    1_000_000_000.0,
                    2_000_000_000.0,
                    -1_000_000_000.0,
                ),
                (0.0, -1.0),
                vec![(1, 0), (2, 0), (0, 1)],
            ),
            (
                (
                    -1_000_000_000.0,
                    -2_000_000_000.0,
                    1_000_000_000.0,
                    2_000_000_000.0,
                ),
                (1.0, 0.0),
                vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6), (3, 7)],
            ),
            (
                (
                    1_000_000_000.0,
                    -2_000_000_000.0,
                    -1_000_000_000.0,
                    2_000_000_000.0,
                ),
                (-1.0, 0.0),
                vec![(1, 0), (0, 1), (0, 2)],
            ),
        ];

        for ((x1, y1, x2, y2), (camera_x, camera_y), expected) in cases {
            let mut canvas = Canvas::<Color>::new(8, 8);
            canvas.set_camera(camera_x, camera_y);
            canvas.draw_line(x1, y1, x2, y2, 3);

            let pixels: Vec<_> = canvas
                .data
                .iter()
                .enumerate()
                .filter(|(_, value)| **value != 0)
                .map(|(index, _)| (index % 8, index / 8))
                .collect();
            assert_eq!(pixels, expected);
        }
    }

    #[test]
    fn test_circle_half_pixel_precision_boundary_uses_wide_sampling() {
        for border in [false, true] {
            let mut canvas = Canvas::<Color>::new(1, 2);
            if border {
                canvas.draw_circle_border(-2069.0, -4_194_303.0, 4_194_304.0, 3);
            } else {
                canvas.draw_circle(-2069.0, -4_194_303.0, 4_194_304.0, 3);
            }
            assert_eq!(canvas.data, vec![3, 0]);
        }
    }

    #[test]
    fn test_camera_overflow_does_not_wrap_into_canvas() {
        let mut canvas = Canvas::<Color>::new(8, 8);
        canvas.set_camera(2_147_483_648.0, 0.0);
        canvas.set_value(-2_147_483_648.0, 0.0, 3);
        canvas.draw_rect(-2_147_483_648.0, 0.0, 2.0, 2.0, 3);
        canvas.draw_rect_border(-2_147_483_648.0, 0.0, 2.0, 2.0, 3);
        canvas.flood_fill(-2_147_483_648.0, 0.0, 3);

        let mut source = Canvas::<Color>::new(2, 2);
        source.clear(3);
        canvas.blit(
            -2_147_483_648.0,
            0.0,
            &source,
            0.0,
            0.0,
            2.0,
            2.0,
            None,
            None,
        );
        assert_eq!(canvas.data, vec![0; 64]);
    }

    #[test]
    fn test_rectangle_large_camera_retains_intersecting_extent() {
        let mut canvas = Canvas::<Color>::new(8, 8);
        canvas.set_camera(2_000_000_000.0, 0.0);
        canvas.draw_rect(-2_000_000_000.0, 0.0, 4_294_967_296.0, 8.0, 3);
        assert_eq!(canvas.data, vec![3; 64]);
    }

    #[test]
    fn test_projection_boundaries_preserve_signed_extent() {
        let rect = RectArea::new(0, 0, 8, 8);
        let transform = TransformProjection::new(
            2_147_483_648.0,
            0.0,
            0.0,
            0.0,
            -2_147_483_648.0,
            8.0,
            i32::MIN,
            0,
            0.0,
            1.0,
            rect,
        )
        .unwrap();
        assert_eq!(transform.width, 2_147_483_648);
        assert!(transform.x1 > transform.x2);

        assert!(PerspectiveProjection::new(
            -2_147_483_648.0,
            0.0,
            8.0,
            8.0,
            i32::MAX,
            0,
            (0.0, 0.0, 1.0),
            (0.0, 0.0, 0.0),
            None
        )
        .is_none());
    }

    #[test]
    fn test_curves_preserve_pixels_with_clip_camera_and_dither() {
        for border in [false, true] {
            let mut circle = Canvas::<Color>::new(8, 8);
            let mut ellipse = Canvas::<Color>::new(8, 8);
            for canvas in [&mut circle, &mut ellipse] {
                canvas.set_camera(2.0, 1.0);
                canvas.set_clip_rect(1.0, 1.0, 5.0, 5.0);
                canvas.set_dithering(0.5);
            }

            if border {
                circle.draw_circle_border(5.0, 4.0, 2.0, 3);
                ellipse.draw_ellipse_border(3.0, 2.0, 5.0, 5.0, 3);
            } else {
                circle.draw_circle(5.0, 4.0, 2.0, 3);
                ellipse.draw_ellipse(3.0, 2.0, 5.0, 5.0, 3);
            }

            let pixels = if border {
                vec![(3, 1), (1, 3), (5, 3), (3, 5)]
            } else {
                vec![
                    (3, 1),
                    (2, 2),
                    (4, 2),
                    (1, 3),
                    (3, 3),
                    (5, 3),
                    (2, 4),
                    (4, 4),
                    (3, 5),
                ]
            };
            let mut expected = vec![0; 64];
            for (x, y) in pixels {
                expected[y * 8 + x] = 3;
            }
            assert_eq!(circle.data, expected);
            assert_eq!(ellipse.data, expected);
        }
    }

    #[test]
    fn test_large_curves_clip_to_small_canvas() {
        for border in [false, true] {
            let mut circle = Canvas::<Color>::new(8, 8);
            let mut ellipse = Canvas::<Color>::new(8, 8);
            if border {
                circle.draw_circle_border(0.0, 0.0, 2_000_000_000.0, 3);
                ellipse.draw_ellipse_border(
                    -2_000_000_000.0,
                    -2_000_000_000.0,
                    4_000_000_000.0,
                    4_000_000_000.0,
                    3,
                );
            } else {
                circle.draw_circle(0.0, 0.0, 2_000_000_000.0, 3);
                ellipse.draw_ellipse(
                    -2_000_000_000.0,
                    -2_000_000_000.0,
                    4_000_000_000.0,
                    4_000_000_000.0,
                    3,
                );
            }

            let expected = vec![if border { 0 } else { 3 }; 64];
            assert_eq!(circle.data, expected);
            assert_eq!(ellipse.data, expected);
        }
    }

    #[test]
    fn test_curves_reject_disjoint_large_camera_bounds() {
        let mut canvas = Canvas::<Color>::new(8, 8);
        canvas.set_camera(-2_000_000_000.0, -2_000_000_000.0);
        canvas.draw_circle(2_000_000_000.0, 2_000_000_000.0, 2.0, 3);
        canvas.draw_circle_border(2_000_000_000.0, 2_000_000_000.0, 2.0, 3);
        canvas.draw_ellipse(2_000_000_000.0, 2_000_000_000.0, 5.0, 5.0, 3);
        canvas.draw_ellipse_border(2_000_000_000.0, 2_000_000_000.0, 5.0, 5.0, 3);
        assert_eq!(canvas.data, vec![0; 64]);
    }

    #[test]
    fn test_triangle_crosses_large_coordinates() {
        for extent in [1_000_000_000.0, 2_000_000_000.0] {
            let mut canvas = Canvas::<Color>::new(8, 8);
            canvas.draw_triangle(-extent, -extent, extent, -extent, 0.0, extent, 3);
            assert_eq!(canvas.data, vec![3; 64]);
        }
    }

    #[test]
    fn test_triangle_large_camera_preserves_clipped_edge() {
        let mut canvas = Canvas::<Color>::new(8, 8);
        canvas.set_camera(2_000_000_000.0, 0.0);
        canvas.draw_triangle(
            -2_000_000_000.0,
            -2_000_000_000.0,
            2_000_000_000.0,
            -2_000_000_000.0,
            2_000_000_000.0,
            2_000_000_000.0,
            3,
        );

        let mut expected = vec![0; 64];
        for row in 0..8 {
            expected[row * 8] = 3;
        }
        assert_eq!(canvas.data, expected);
    }

    #[test]
    fn test_diagonal_line_crosses_large_coordinates() {
        for extent in [1_000_000_000.0, 2_000_000_000.0] {
            let mut canvas = Canvas::<Color>::new(8, 8);
            canvas.draw_line(-extent, -extent, extent, extent, 3);

            let pixels: Vec<_> = canvas
                .data
                .iter()
                .enumerate()
                .filter(|(_, value)| **value != 0)
                .map(|(index, _)| (index % 8, index / 8))
                .collect();
            assert_eq!(
                pixels,
                (0..8)
                    .map(|coordinate| (coordinate, coordinate))
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_diagonal_line_large_camera_preserves_visible_endpoint() {
        let mut canvas = Canvas::<Color>::new(8, 8);
        canvas.set_camera(2_000_000_000.0, 2_000_000_000.0);
        canvas.draw_line(
            -2_000_000_000.0,
            -2_000_000_000.0,
            2_000_000_000.0,
            2_000_000_000.0,
            3,
        );

        let mut expected = vec![0; 64];
        expected[0] = 3;
        assert_eq!(canvas.data, expected);
    }

    #[test]
    fn test_diagonal_line_large_coordinates_obeys_clip_camera_and_dither() {
        let mut canvas = Canvas::<Color>::new(8, 8);
        canvas.set_camera(2.0, 2.0);
        canvas.set_clip_rect(1.0, 1.0, 5.0, 5.0);
        canvas.set_dithering(0.5);
        canvas.draw_line(
            -2_000_000_000.0,
            -2_000_000_000.0,
            2_000_000_000.0,
            2_000_000_000.0,
            3,
        );

        let pixels: Vec<_> = canvas
            .data
            .iter()
            .enumerate()
            .filter(|(_, value)| **value != 0)
            .map(|(index, _)| (index % 8, index / 8))
            .collect();
        assert_eq!(pixels, vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_diagonal_line_preserves_pixels_with_clip_camera_and_dither() {
        for (x1, y1, x2, y2, clip, expected) in [
            (
                -3.0,
                0.0,
                9.0,
                6.0,
                (1, 0, 5, 4),
                vec![(1, 1), (4, 2), (5, 3)],
            ),
            (
                -1.0,
                -2.0,
                5.0,
                10.0,
                (0, 1, 4, 5),
                vec![(1, 1), (2, 4), (3, 5)],
            ),
        ] {
            let mut canvas = Canvas::<Color>::new(8, 8);
            canvas.set_camera(1.0, 2.0);
            canvas.set_clip_rect(clip.0 as f32, clip.1 as f32, clip.2 as f32, clip.3 as f32);
            canvas.set_dithering(0.5);
            canvas.draw_line(x1, y1, x2, y2, 1);

            let pixels: Vec<_> = canvas
                .data
                .iter()
                .enumerate()
                .filter_map(|(i, &value)| (value != 0).then_some((i % 8, i / 8)))
                .collect();
            assert_eq!(pixels, expected);
        }
    }

    #[test]
    fn test_triangle_preserves_pixels_with_clip_camera_and_dither() {
        let mut canvas = Canvas::<Color>::new(8, 8);
        canvas.set_camera(1.0, 2.0);
        canvas.set_clip_rect(1.0, 1.0, 4.0, 4.0);
        canvas.set_dithering(0.5);
        canvas.draw_triangle(-1.0, 0.0, 7.0, 0.0, -1.0, 8.0, 1);

        let pixels: Vec<_> = canvas
            .data
            .iter()
            .enumerate()
            .filter_map(|(i, &value)| (value != 0).then_some((i % 8, i / 8)))
            .collect();
        assert_eq!(pixels, [(1, 1), (3, 1), (2, 2), (1, 3)]);
    }
}
