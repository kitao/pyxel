use std::cmp::max;
use std::mem::swap;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::rectarea::RectArea;
use crate::types::ToIndex;
use crate::utils::{as_i32, as_u32};

pub trait Canvas<T: Copy + PartialEq + Default + ToIndex> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn _value(&self, x: i32, y: i32) -> T;
    fn _set_value(&mut self, x: i32, y: i32, value: T);
    fn _self_rect(&self) -> RectArea;
    fn _clip_rect(&self) -> RectArea;
    fn _set_clip_rect(&mut self, clip_rect: RectArea);
    fn _palette_value(&self, value: T) -> T;

    fn clip(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let x = as_i32(x);
        let y = as_i32(y);
        let width = as_u32(width);
        let height = as_u32(height);
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

    fn pget(&mut self, x: f64, y: f64) -> T {
        let x = as_i32(x);
        let y = as_i32(y);
        if self._self_rect().contains(x, y) {
            self._value(x, y)
        } else {
            T::default()
        }
    }

    fn pset(&mut self, x: f64, y: f64, value: T) {
        let x = as_i32(x);
        let y = as_i32(y);
        if self._clip_rect().contains(x, y) {
            self._set_value(x, y, self._palette_value(value));
        }
    }

    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, value: T) {
        let x1 = as_i32(x1);
        let y1 = as_i32(y1);
        let x2 = as_i32(x2);
        let y2 = as_i32(y2);
        if x1 == x2 && y1 == y2 {
            self.pset(x1 as f64, y1 as f64, value);
            return;
        }
        if (x1 - x2).abs() > (y1 - y2).abs() {
            let (start_x, start_y, end_x, end_y) = if x1 < x2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };
            let length = end_x - start_x + 1;
            let alpha = (end_y - start_y) as f64 / (end_x - start_x) as f64;
            for i in 0..length {
                self.pset(
                    (start_x + i) as f64,
                    start_y as f64 + alpha * i as f64,
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
            for i in 0..length {
                self.pset(
                    start_x as f64 + alpha * i as f64,
                    (start_y + i) as f64,
                    value,
                );
            }
        }
    }

    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = as_i32(x);
        let y = as_i32(y);
        let width = as_u32(width);
        let height = as_u32(height);
        let rect = RectArea::new(x, y, width, height).intersects(self._clip_rect());
        if rect.is_empty() {
            return;
        }
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();
        let value = self._palette_value(value);
        for i in top..=bottom {
            for j in left..=right {
                self._set_value(j, i, value);
            }
        }
    }

    fn rectb(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = as_i32(x);
        let y = as_i32(y);
        let width = as_u32(width);
        let height = as_u32(height);
        let rect = RectArea::new(x, y, width, height);
        if rect.intersects(self._clip_rect()).is_empty() {
            return;
        }
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();
        for i in left..=right {
            self.pset(i as f64, top as f64, value);
            self.pset(i as f64, bottom as f64, value);
        }
        for i in top..=bottom {
            self.pset(left as f64, i as f64, value);
            self.pset(right as f64, i as f64, value);
        }
    }

    fn circ(&mut self, x: f64, y: f64, radius: f64, value: T) {
        let x = as_i32(x);
        let y = as_i32(y);
        let radius = as_u32(radius);
        let sq_radius = radius * radius;
        for dx in 0..=radius as i32 {
            let dy = as_i32(((sq_radius as i32 - dx * dx) as f64).sqrt());
            if dx > dy {
                continue;
            }
            for i in -dy..=dy {
                self.pset((x - dx) as f64, (y + i) as f64, value);
                self.pset((x + dx) as f64, (y + i) as f64, value);
                self.pset((x + i) as f64, (y - dx) as f64, value);
                self.pset((x + i) as f64, (y + dx) as f64, value);
            }
        }
    }

    fn circb(&mut self, x: f64, y: f64, radius: f64, value: T) {
        let x = as_i32(x);
        let y = as_i32(y);
        let radius = as_u32(radius);
        let sq_radius = radius * radius;
        for dx in 0..=radius as i32 {
            let dy = as_i32(((sq_radius as i32 - dx * dx) as f64).sqrt());
            if dx > dy {
                continue;
            }
            self.pset((x - dx) as f64, (y - dy) as f64, value);
            self.pset((x + dx) as f64, (y - dy) as f64, value);
            self.pset((x - dx) as f64, (y + dy) as f64, value);
            self.pset((x + dx) as f64, (y + dy) as f64, value);
            self.pset((x - dy) as f64, (y - dx) as f64, value);
            self.pset((x + dy) as f64, (y - dx) as f64, value);
            self.pset((x - dy) as f64, (y + dx) as f64, value);
            self.pset((x + dy) as f64, (y + dx) as f64, value);
        }
    }

    fn tri(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, value: T) {
        let mut x1 = as_i32(x1);
        let mut y1 = as_i32(y1);
        let mut x2 = as_i32(x2);
        let mut y2 = as_i32(y2);
        let mut x3 = as_i32(x3);
        let mut y3 = as_i32(y3);
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
        for i in y1..=y2 {
            let (x_slider, x_end) = if x_inter < x2 {
                (
                    as_i32(x_inter as f64 + alpha13 * (i - y2) as f64),
                    as_i32(x2 as f64 + alpha12 * (i - y2) as f64),
                )
            } else {
                (
                    as_i32(x2 as f64 + alpha12 * (i - y2) as f64),
                    as_i32(x_inter as f64 + alpha13 * (i - y2) as f64),
                )
            };
            for j in x_slider..=x_end {
                self.pset(j as f64, i as f64, value);
            }
        }
        for i in (y2 + 1)..=y3 {
            let (x_slider, x_end) = if x_inter < x2 {
                (
                    as_i32(x_inter as f64 + alpha13 * (i - y2) as f64),
                    as_i32(x2 as f64 + alpha23 * (i - y2) as f64),
                )
            } else {
                (
                    as_i32(x2 as f64 + alpha23 * (i - y2) as f64),
                    as_i32(x_inter as f64 + alpha13 * (i - y2) as f64),
                )
            };
            for j in x_slider..=x_end {
                self.pset(j as f64, i as f64, value);
            }
        }
    }

    fn trib(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, value: T) {
        self.line(x1, y1, x2, y2, value);
        self.line(x1, y1, x3, y3, value);
        self.line(x2, y2, x3, y3, value);
    }

    fn fill(&mut self, x: f64, y: f64, value: T) {
        let x = as_i32(x);
        let y = as_i32(y);
        if !self._clip_rect().contains(x, y) {
            return;
        }
        let value = self._palette_value(value);
        let dst_value = self._value(x, y);
        if value != dst_value {
            self._fill_rec(x, y, value, dst_value);
        }
    }

    fn _fill_rec(&mut self, x: i32, y: i32, value: T, dst_value: T) {
        let rect = self._clip_rect();
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();
        for i in (x..=left).rev() {
            if self._value(i, y) != dst_value {
                break;
            }
            self._set_value(i, y, value);
            if y > top && self._value(i, y - 1) == dst_value {
                self._fill_rec(i, y - 1, value, dst_value);
            }
            if y > bottom && self._value(i, y + 1) == dst_value {
                self._fill_rec(i, y + 1, value, dst_value);
            }
        }
        for i in x + 1..=right {
            if self._value(i, y) != dst_value {
                break;
            }
            self._set_value(i, y, value);
            if y > top && self._value(i, y - 1) == dst_value {
                self._fill_rec(i, y - 1, value, dst_value);
            }
            if y > bottom && self._value(i, y + 1) == dst_value {
                self._fill_rec(i, y + 1, value, dst_value);
            }
        }
    }

    fn blt(
        &mut self,
        x: f64,
        y: f64,
        canvas: Arc<Mutex<Self>>,
        canvas_x: f64,
        canvas_y: f64,
        width: f64,
        height: f64,
        transparent: Option<T>,
    ) {
        let canvas = if let Some(canvas) = canvas.try_lock() {
            canvas
        } else {
            panic!("unable to lock canvas in blt");
        };
        let x = as_i32(x);
        let y = as_i32(y);
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
            self._clip_rect(),
            canvas_x,
            canvas_y,
            canvas._self_rect(),
            width,
            height,
        );
        if width == 0 || height == 0 {
            return;
        }
        for i in 0..height {
            for j in 0..width {
                let value =
                    canvas._value(src_x + sign_x * j + offset_x, src_y + sign_y * i + offset_y);
                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }
                self._set_value(dst_x + j, dst_y + i, self._palette_value(value));
            }
        }
    }

    fn blt_self(
        &mut self,
        x: f64,
        y: f64,
        canvas_x: f64,
        canvas_y: f64,
        width: f64,
        height: f64,
        transparent: Option<T>,
    ) {
        let x = as_i32(x);
        let y = as_i32(y);
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
            self._clip_rect(),
            canvas_x,
            canvas_y,
            self._self_rect(),
            width,
            height,
        );
        if width == 0 || height == 0 {
            return;
        }
        let canvas: Vec<Vec<T>> = (0..height)
            .map(|i| (0..width).map(|j| self._value(j, i)).collect())
            .collect();
        for i in 0..height {
            for j in 0..width {
                let value = canvas[(src_y + sign_y * i + offset_y) as usize]
                    [(src_x + sign_x * j + offset_x) as usize];
                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }
                self._set_value(dst_x + j, dst_y + i, self._palette_value(value));
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
    ) -> Self {
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
