use std::cmp::max;
use std::mem::swap;

use crate::rectarea::RectArea;
use crate::types::ToIndex;

pub trait Canvas<T: Copy + PartialEq + Default + ToIndex> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn self_rect(&self) -> RectArea;
    fn clip_rect(&self) -> RectArea;
    fn value(&self, x: i32, y: i32) -> T;
    fn set_value(&mut self, x: i32, y: i32, value: T);
    fn clip(&mut self, x: i32, y: i32, width: u32, height: u32);
    fn clip0(&mut self);

    fn cls(&mut self, value: T) {
        let width = self.width();
        let height = self.height();

        for i in 0..height {
            for j in 0..width {
                self.set_value(j as i32, i as i32, value);
            }
        }
    }

    fn pget(&mut self, x: i32, y: i32) -> T {
        if self.self_rect().contains(x, y) {
            self.value(x, y)
        } else {
            T::default()
        }
    }

    fn pset(&mut self, x: i32, y: i32, value: T) {
        if self.self_rect().contains(x, y) {
            self.set_value(x, y, value)
        }
    }

    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, value: T) {
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
        let rect = RectArea::new(x, y, width, height).intersects(self.clip_rect());

        if rect.is_empty() {
            return;
        }

        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();

        for i in top..=bottom {
            for j in left..=right {
                self.set_value(j, i, value);
            }
        }
    }

    fn rectb(&mut self, x: i32, y: i32, width: u32, height: u32, value: T) {
        let rect = RectArea::new(x, y, width, height).intersects(self.clip_rect());

        if rect.is_empty() {
            return;
        }

        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();

        for i in left..=right {
            self.set_value(i, top, value);
            self.set_value(i, bottom, value);
        }

        for i in top..=bottom {
            self.set_value(left, i, value);
            self.set_value(right, i, value);
        }
    }

    fn circ(&mut self, x: i32, y: i32, radius: u32, value: T) {
        if radius == 0 {
            self.pset(x, y, value);
            return;
        }

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
            self.pset(x, y, value);
            return;
        }

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
        if !self.clip_rect().contains(x, y) {
            return;
        }

        let target_value = self.value(x, y);

        if value != target_value {
            self._fill_rec(x, y, value, target_value);
        }
    }

    fn _fill_rec(&mut self, x: i32, y: i32, value: T, target_value: T) {
        let rect = self.clip_rect();
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();

        for i in (x..=left).rev() {
            if self.value(i, y) != target_value {
                break;
            }

            self.set_value(i, y, value);

            if y > top && self.value(i, y - 1) == target_value {
                self._fill_rec(i, y - 1, value, target_value);
            }

            if y > bottom && self.value(i, y + 1) == target_value {
                self._fill_rec(i, y + 1, value, target_value);
            }
        }

        for i in x + 1..=right {
            if self.value(i, y) != target_value {
                break;
            }

            self.set_value(i, y, value);

            if y > top && self.value(i, y - 1) == target_value {
                self._fill_rec(i, y - 1, value, target_value);
            }

            if y > bottom && self.value(i, y + 1) == target_value {
                self._fill_rec(i, y + 1, value, target_value);
            }
        }
    }

    fn blt(
        &mut self,
        x: i32,
        y: i32,
        canvas: &Self,
        canvas_x: i32,
        canvas_y: i32,
        width: i32,
        height: i32,
        transparent: Option<T>,
        palette: Option<&[T]>,
    ) {
        let src_rect = canvas.self_rect();
        let dst_rect = self.clip_rect();
        let flip_x = width < 0;
        let flip_y = height < 0;
        let width = width.abs();
        let height = height.abs();

        let left_margin = max(max(src_rect.left() - canvas_x, dst_rect.left() - x), 0);
        let right_margin = max(
            max(
                canvas_x + width - 1 - src_rect.right(),
                x + width - 1 - dst_rect.right(),
            ),
            0,
        );
        let top_margin = max(max(src_rect.top() - canvas_y, dst_rect.top() - y), 0);
        let bottom_margin = max(
            max(
                canvas_y + height - 1 - src_rect.bottom(),
                y + height - 1 - dst_rect.bottom(),
            ),
            0,
        );

        let src_x = canvas_x + if flip_x { right_margin } else { left_margin };
        let src_y = canvas_y + if flip_y { bottom_margin } else { top_margin };
        let dst_x = x + left_margin;
        let dst_y = y + top_margin;
        let width = max(width - left_margin - right_margin, 0);
        let height = max(height - top_margin - bottom_margin, 0);

        if width == 0 || height == 0 {
            return;
        }

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

        for i in 0..height {
            for j in 0..width {
                let value =
                    canvas.value(src_x + sign_x * j + offset_x, src_y + sign_y * i + offset_y);

                let value = if let Some(palette) = palette {
                    palette[value.to_index()]
                } else {
                    value
                };

                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }

                self.set_value(dst_x + j, dst_y + i, value);
            }
        }
    }
}
