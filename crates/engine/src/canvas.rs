use parking_lot::Mutex;
use std::cmp::max;
use std::mem::swap;
use std::sync::Arc;

use crate::rectarea::RectArea;
use crate::types::ToIndex;

pub trait Canvas<T: Copy + PartialEq + Default + ToIndex> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn _value(&self, x: i32, y: i32) -> T;
    fn _set_value(&mut self, x: i32, y: i32, value: T);
    fn _self_rect(&self) -> RectArea;
    fn _clip_rect(&self) -> RectArea;
    fn _set_clip_rect(&mut self, clip_rect: RectArea);
    fn _palette_value(&self, value: T) -> T;

    fn clip(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self._set_clip_rect(
            self._self_rect()
                .intersects(RectArea::new(x, y, width, height)),
        );
    }

    fn clip0(&mut self) {
        self._set_clip_rect(self._self_rect());
    }

    fn cls(&mut self, value: T) {
        let width = self.width();
        let height = self.height();
        let value = self._palette_value(value);

        for i in 0..height {
            for j in 0..width {
                self._set_value(j as i32, i as i32, value);
            }
        }
    }

    fn pget(&mut self, x: i32, y: i32) -> T {
        if self._self_rect().contains(x, y) {
            self._value(x, y)
        } else {
            T::default()
        }
    }

    fn pset(&mut self, x: i32, y: i32, value: T) {
        let value = self._palette_value(value);

        if self._self_rect().contains(x, y) {
            self._set_value(x, y, value)
        }
    }

    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, value: T) {
        let value = self._palette_value(value);

        if x1 == x2 && y1 == y2 {
            self.pset(x1, y1, value);
            return;
        }

        if (x1 - x2).abs() > (y1 - y2).abs() {
            let start_x: i32;
            let start_y: i32;
            let end_x: i32;
            let end_y: i32;

            if x1 < x2 {
                start_x = x1;
                start_y = y1;
                end_x = x2;
                end_y = y2;
            } else {
                start_x = x2;
                start_y = y2;
                end_x = x1;
                end_y = y1;
            }

            let length = end_x - start_x + 1;
            let alpha = (end_y - start_y) as f64 / (end_x - start_x) as f64;

            for i in 0..length {
                self.pset(
                    start_x + i,
                    (start_y as f64 + alpha * i as f64 + 0.5) as i32,
                    value,
                );
            }
        } else {
            let start_x: i32;
            let start_y: i32;
            let end_x: i32;
            let end_y: i32;

            if y1 < y2 {
                start_x = x1;
                start_y = y1;
                end_x = x2;
                end_y = y2;
            } else {
                start_x = x2;
                start_y = y2;
                end_x = x1;
                end_y = y1;
            }

            let length = end_y - start_y + 1;
            let alpha = (end_x - start_x) as f64 / (end_y - start_y) as f64;

            for i in 0..length {
                self.pset(
                    (start_x as f64 + alpha as f64 * i as f64 + 0.5) as i32,
                    start_y + i,
                    value,
                );
            }
        }
    }

    fn rect(&mut self, x: i32, y: i32, width: u32, height: u32, value: T) {
        let rect = RectArea::new(x, y, width, height).intersects(self._clip_rect());

        if rect.is_empty() {
            return;
        }

        let value = self._palette_value(value);
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();

        for i in top..=bottom {
            for j in left..=right {
                self._set_value(j, i, value);
            }
        }
    }

    fn rectb(&mut self, x: i32, y: i32, width: u32, height: u32, value: T) {
        let rect = RectArea::new(x, y, width, height).intersects(self._clip_rect());

        if rect.is_empty() {
            return;
        }

        let value = self._palette_value(value);
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();

        for i in left..=right {
            self._set_value(i, top, value);
            self._set_value(i, bottom, value);
        }

        for i in top..=bottom {
            self._set_value(left, i, value);
            self._set_value(right, i, value);
        }
    }

    fn circ(&mut self, x: i32, y: i32, radius: u32, value: T) {
        if radius == 0 {
            return;
        }

        let value = self._palette_value(value);
        let sq_radius = radius * radius;

        for dx in 0..=radius as i32 {
            let dy = (((sq_radius as i32 - dx * dx) as f64).sqrt() + 0.5) as i32;

            if dx > dy {
                continue;
            }

            for i in -dy..=dy {
                self.pset(x - dx, y + i, value);
                self.pset(x + dx, y + i, value);
                self.pset(x + i, y - dx, value);
                self.pset(x + i, y + dx, value);
            }
        }
    }

    fn circb(&mut self, x: i32, y: i32, radius: u32, value: T) {
        if radius == 0 {
            return;
        }

        let value = self._palette_value(value);
        let sq_radius = radius * radius;

        for dx in 0..=radius as i32 {
            let dy = (((sq_radius as i32 - dx * dx) as f64).sqrt() + 0.5) as i32;

            if dx > dy {
                continue;
            }

            self.pset(x - dx, y - dy, value);
            self.pset(x + dx, y - dy, value);
            self.pset(x - dx, y + dy, value);
            self.pset(x + dx, y + dy, value);

            self.pset(x - dy, y - dx, value);
            self.pset(x + dy, y - dx, value);
            self.pset(x - dy, y + dx, value);
            self.pset(x + dy, y + dx, value);
        }
    }

    fn tri(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, value: T) {
        let mut x1 = x1;
        let mut y1 = y1;
        let mut x2 = x2;
        let mut y2 = y2;
        let mut x3 = x3;
        let mut y3 = y3;

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
        let x_inter = (x1 as f64 + alpha13 * (y2 - y1) as f64 + 0.5) as i32;

        for i in y1..=y2 {
            let x_slider;
            let x_end;

            if x_inter < x2 {
                x_slider = (x_inter as f64 + alpha13 * (i - y2) as f64 + 0.5) as i32;
                x_end = (x2 as f64 + alpha12 * (i - y2) as f64 + 0.5) as i32;
            } else {
                x_slider = (x2 as f64 + alpha12 * (i - y2) as f64 + 0.5) as i32;
                x_end = (x_inter as f64 + alpha13 * (i - y2) as f64 + 0.5) as i32;
            }

            for j in x_slider..=x_end {
                self.pset(j, i, value);
            }
        }

        for i in (y2 + 1)..=y3 {
            let x_slider;
            let x_end;

            if x_inter < x2 {
                x_slider = (x_inter as f64 + alpha13 * (i - y2) as f64 + 0.5) as i32;
                x_end = (x2 as f64 + alpha23 * (i - y2) as f64 + 0.5) as i32;
            } else {
                x_slider = (x2 as f64 + alpha23 * (i - y2) as f64 + 0.5) as i32;
                x_end = (x_inter as f64 + alpha13 * (i - y2) as f64 + 0.5) as i32;
            }

            for j in x_slider..=x_end {
                self.pset(j, i, value);
            }
        }
    }

    fn trib(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, value: T) {
        self.line(x1, y1, x2, y2, value);
        self.line(x1, y1, x3, y3, value);
        self.line(x2, y2, x3, y3, value);
    }

    fn fill(&mut self, x: i32, y: i32, value: T) {
        if !self._clip_rect().contains(x, y) {
            return;
        }

        let value = self._palette_value(value);
        let target_value = self._value(x, y);

        if value != target_value {
            self._fill_rec(x, y, value, target_value);
        }
    }

    fn _fill_rec(&mut self, x: i32, y: i32, value: T, target_value: T) {
        let rect = self._clip_rect();
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();

        for i in (x..=left).rev() {
            if self._value(i, y) != target_value {
                break;
            }

            self._set_value(i, y, value);

            if y > top && self._value(i, y - 1) == target_value {
                self._fill_rec(i, y - 1, value, target_value);
            }

            if y > bottom && self._value(i, y + 1) == target_value {
                self._fill_rec(i, y + 1, value, target_value);
            }
        }

        for i in x + 1..=right {
            if self._value(i, y) != target_value {
                break;
            }

            self._set_value(i, y, value);

            if y > top && self._value(i, y - 1) == target_value {
                self._fill_rec(i, y - 1, value, target_value);
            }

            if y > bottom && self._value(i, y + 1) == target_value {
                self._fill_rec(i, y + 1, value, target_value);
            }
        }
    }

    fn blt(
        &mut self,
        x: i32,
        y: i32,
        canvas: Arc<Mutex<Self>>,
        canvas_x: i32,
        canvas_y: i32,
        width: i32,
        height: i32,
        transparent: Option<T>,
    ) {
        let canvas = canvas.lock();
        let copy_area = CopyArea::new(
            x,
            y,
            self._clip_rect(),
            canvas_x,
            canvas_y,
            canvas._self_rect(),
            width,
            height,
        );

        let dst_x = copy_area.dst_x;
        let dst_y = copy_area.dst_y;
        let src_x = copy_area.src_x;
        let src_y = copy_area.src_y;
        let sign_x = copy_area.sign_x;
        let sign_y = copy_area.sign_y;
        let offset_x = copy_area.offset_x;
        let offset_y = copy_area.offset_y;
        let width = copy_area.width;
        let height = copy_area.height;

        if width == 0 || height == 0 {
            return;
        }

        for i in 0..height {
            for j in 0..width {
                let value = self._palette_value(
                    canvas._value(src_x + sign_x * j + offset_x, src_y + sign_y * i + offset_y),
                );

                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }

                self._set_value(dst_x + j, dst_y + i, value);
            }
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
    ) -> CopyArea {
        let flip_x = width < 0;
        let flip_y = height < 0;
        let width = width.abs();
        let height = height.abs();

        let left_cut = max(max(src_rect.left() - src_x, dst_rect.left() - dst_x), 0);
        let right_cut = max(
            max(
                src_x + width - 1 - src_rect.right(),
                dst_x + width - 1 - dst_rect.right(),
            ),
            0,
        );
        let top_cut = max(max(src_rect.top() - src_y, dst_rect.top() - dst_y), 0);
        let bottom_cut = max(
            max(
                src_y + height - 1 - src_rect.bottom(),
                dst_y + height - 1 - dst_rect.bottom(),
            ),
            0,
        );

        let width = max(width - left_cut - right_cut, 0);
        let height = max(height - top_cut - bottom_cut, 0);

        let sign_x: i32;
        let sign_y: i32;
        let offset_x: i32;
        let offset_y: i32;

        if flip_x {
            sign_x = -1;
            offset_x = width - 1;
        } else {
            sign_x = 1;
            offset_x = 0;
        }

        if flip_y {
            sign_y = -1;
            offset_y = height - 1;
        } else {
            sign_y = 1;
            offset_y = 0;
        }

        CopyArea {
            dst_x: dst_x + left_cut,
            dst_y: dst_y + top_cut,
            src_x: src_x + if flip_x { right_cut } else { left_cut },
            src_y: src_y + if flip_y { bottom_cut } else { top_cut },
            sign_x: sign_x,
            sign_y: sign_y,
            offset_x: offset_x,
            offset_y: offset_y,
            width: width,
            height: height,
        }
    }
}
