use std::cmp::max;
use std::mem::swap;

use crate::rectarea::RectArea;

pub trait Canvas<T: Copy + PartialEq + Default> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn data(&self) -> &Vec<Vec<T>>;
    fn data_mut(&mut self) -> &mut Vec<Vec<T>>;

    fn _self_rect(&self) -> RectArea;
    fn _clip_rect(&self) -> RectArea;
    fn _clip_rect_mut(&mut self) -> &mut RectArea;
    fn _palette_value(&self, val: T) -> T;

    fn clip(&mut self, x: i32, y: i32, w: u32, h: u32) {
        let rect = RectArea::new(x, y, w, h).intersects(self._self_rect());
        *self._clip_rect_mut() = rect;
    }

    fn clip_(&mut self) {
        *self._clip_rect_mut() = self._self_rect();
    }

    fn cls(&mut self, val: T) {
        let val = self._palette_value(val);
        let width = self.width();
        let height = self.height();
        let data = self.data_mut();

        for i in 0..height {
            for j in 0..width {
                set_data_value(data, j as i32, i as i32, val);
            }
        }
    }

    fn pget(&mut self, x: i32, y: i32) -> T {
        data_value_with_check(self.data(), self._self_rect(), x, y)
    }

    fn pset(&mut self, x: i32, y: i32, val: T) {
        let data: *mut Vec<Vec<T>> = self.data_mut();

        unsafe {
            set_data_value_with_check(&mut *data, self._self_rect(), x, y, val);
        }
    }

    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, val: T) {
        let val = self._palette_value(val);
        let rect = self._self_rect();
        let data = self.data_mut();

        if x1 == x2 && y1 == y2 {
            self.pset(x1, y1, val);
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
                set_data_value_with_check(
                    data,
                    rect,
                    start_x + i,
                    (start_y as f64 + alpha * i as f64 + 0.5) as i32,
                    val,
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
                set_data_value_with_check(
                    data,
                    rect,
                    (start_x as f64 + alpha as f64 * i as f64 + 0.5) as i32,
                    start_y + i,
                    val,
                );
            }
        }
    }

    fn rect(&mut self, x: i32, y: i32, width: u32, height: u32, val: T) {
        let rect = RectArea::new(x, y, width, height).intersects(self._clip_rect());

        if rect.is_empty() {
            return;
        }

        let val = self._palette_value(val);
        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();
        let data = self.data_mut();

        for i in top..=bottom {
            for j in left..=right {
                set_data_value(data, j, i, val);
            }
        }
    }

    fn rectb(&mut self, x: i32, y: i32, width: u32, height: u32, val: T) {
        let rect = RectArea::new(x, y, width, height).intersects(self._clip_rect());

        if rect.is_empty() {
            return;
        }

        let val = self._palette_value(val);
        let data = self.data_mut();

        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();

        for i in left..=right {
            set_data_value(data, i, top, val);
            set_data_value(data, i, bottom, val);
        }

        for i in top..=bottom {
            set_data_value(data, left, i, val);
            set_data_value(data, right, i, val);
        }
    }

    fn circ(&mut self, x: i32, y: i32, radius: u32, val: T) {
        if radius == 0 {
            self.pset(x, y, val);
            return;
        }

        let val = self._palette_value(val);
        let rect = self._self_rect();
        let data = self.data_mut();
        let sq_radius = radius * radius;

        for dx in 0..=radius as i32 {
            let dy = (((sq_radius as i32 - dx * dx) as f64).sqrt() + 0.5) as i32;

            if dx > dy {
                continue;
            }

            for i in -dy..=dy {
                set_data_value_with_check(data, rect, x - dx, y + i, val);
                set_data_value_with_check(data, rect, x + dx, y + i, val);
                set_data_value_with_check(data, rect, x + i, y - dx, val);
                set_data_value_with_check(data, rect, x + i, y + dx, val);
            }
        }
    }

    fn circb(&mut self, x: i32, y: i32, radius: u32, val: T) {
        if radius == 0 {
            self.pset(x, y, val);
            return;
        }

        let val = self._palette_value(val);
        let rect = self._self_rect();
        let data = self.data_mut();
        let sq_radius = radius * radius;

        for dx in 0..=radius as i32 {
            let dy = (((sq_radius as i32 - dx * dx) as f64).sqrt() + 0.5) as i32;

            if dx > dy {
                continue;
            }

            set_data_value_with_check(data, rect, x - dx, y - dy, val);
            set_data_value_with_check(data, rect, x + dx, y - dy, val);
            set_data_value_with_check(data, rect, x - dx, y + dy, val);
            set_data_value_with_check(data, rect, x + dx, y + dy, val);

            set_data_value_with_check(data, rect, x - dy, y - dx, val);
            set_data_value_with_check(data, rect, x + dy, y - dx, val);
            set_data_value_with_check(data, rect, x - dy, y + dx, val);
            set_data_value_with_check(data, rect, x + dy, y + dx, val);
        }
    }

    fn tri(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, val: T) {
        let val = self._palette_value(val);
        let rect = self._self_rect();
        let data = self.data_mut();

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
                set_data_value_with_check(data, rect, j, i, val);
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
                set_data_value_with_check(data, rect, j, i, val);
            }
        }
    }

    fn trib(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, val: T) {
        self.line(x1, y1, x2, y2, val);
        self.line(x1, y1, x3, y3, val);
        self.line(x2, y2, x3, y3, val);
    }

    fn fill(&mut self, x: i32, y: i32, val: T) {
        let val = self._palette_value(val);

        self._fill_rec(x, y, val);

        /*
        _col = col if type(col) is int else OverlayCanvas.COLOR_MARK

        self._fill_recursively(x, y, _col, dst)

        if type(col) is not int:
            self._replace_with_tiles(dst, x, y, col)
        */
    }

    fn _fill_rec(&mut self, x: i32, y: i32, val: T) {
        /*
        dst_col = dst[y][x]

        if dst_col == col:
            return

        for i in range(x, -1, -1):
            if dst[y][i] != dst_col:
                break

            dst[y][i] = col

            if y > 0 and dst[y - 1][i] == dst_col:
                self._fill_recursively(i, y - 1, col, dst)

            if y < 15 and dst[y + 1][i] == dst_col:
                self._fill_recursively(i, y + 1, col, dst)

        for i in range(x + 1, 16):
            if dst[y][i] != dst_col:
                return

            dst[y][i] = col

            if y > 0 and dst[y - 1][i] == dst_col:
                self._fill_recursively(i, y - 1, col, dst)

            if y < 15 and dst[y + 1][i] == dst_col:
                self._fill_recursively(i, y + 1, col, dst)
        */
    }

    fn blt(
        &mut self,
        x: i32,
        y: i32,
        src: &Self,
        u: i32,
        v: i32,
        width: i32,
        height: i32,
        valkey: Option<T>,
    ) {
        let src_rect = src._self_rect();
        let dst_rect = self._self_rect();

        let flip_x = width < 0;
        let flip_y = height < 0;
        let width = width.abs();
        let height = height.abs();

        let left_margin = max(max(src_rect.left() - u, dst_rect.left() - x), 0);
        let right_margin = max(
            max(
                u + width - 1 - src_rect.right(),
                x + width - 1 - dst_rect.right(),
            ),
            0,
        );
        let top_margin = max(max(src_rect.top() - v, dst_rect.top() - y), 0);
        let bottom_margin = max(
            max(
                v + height - 1 - src_rect.bottom(),
                y + height - 1 - dst_rect.bottom(),
            ),
            0,
        );

        let x = x + left_margin;
        let y = y + top_margin;
        let u = u + if flip_x { right_margin } else { left_margin };
        let v = v + if flip_y { bottom_margin } else { top_margin };
        let width = max(width - left_margin - right_margin, 0);
        let height = max(height - top_margin - bottom_margin, 0);

        if width == 0 && height == 0 {
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

        let src_data = src.data();
        let dst_data: *mut Vec<Vec<T>> = self.data_mut();

        if let Some(valkey) = valkey {
            for i in 0..height {
                for j in 0..width {
                    let val = data_value(
                        src_data,
                        u + sign_x * j + offset_x,
                        v + sign_y * i + offset_y,
                    );

                    if val != valkey {
                        unsafe {
                            set_data_value(&mut *dst_data, x + j, y + i, self._palette_value(val));
                        }
                    }
                }
            }
        } else {
            for i in 0..height {
                for j in 0..width {
                    let val = data_value(
                        src_data,
                        u + sign_x * j + offset_x,
                        v + sign_y * i + offset_y,
                    );
                    self.data_mut()[(y + i) as usize][(x + j) as usize] = self._palette_value(val);
                }
            }
        }
    }
}

fn data_value<T: Copy>(data: &Vec<Vec<T>>, x: i32, y: i32) -> T {
    data[y as usize][x as usize]
}

fn data_value_with_check<T: Copy + Default>(
    data: &Vec<Vec<T>>,
    rect: RectArea,
    x: i32,
    y: i32,
) -> T {
    if rect.contains(x, y) {
        data[y as usize][x as usize]
    } else {
        T::default()
    }
}

#[inline]
fn set_data_value<T: Copy>(data: &mut Vec<Vec<T>>, x: i32, y: i32, val: T) {
    data[y as usize][x as usize] = val;
}

#[inline]
fn set_data_value_with_check<T: Copy>(
    data: &mut Vec<Vec<T>>,
    rect: RectArea,
    x: i32,
    y: i32,
    val: T,
) {
    if rect.contains(x, y) {
        data[y as usize][x as usize] = val;
    }
}
