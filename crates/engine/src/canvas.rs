use std::cmp::max;
use std::mem::swap;

use crate::rectarea::RectArea;

pub trait Canvas<T: Copy + PartialEq + Default> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn data(&self) -> &Vec<Vec<T>>;
    fn data_mut(&mut self) -> &mut Vec<Vec<T>>;
    fn self_rect(&self) -> RectArea;
    fn clip_rect(&self) -> RectArea;
    fn clip_rect_mut(&mut self) -> &mut RectArea;
    fn render_value(&self, original_value: T) -> T;

    fn clip_area(&mut self) -> (i32, i32, u32, u32) {
        (
            self.clip_rect().left(),
            self.clip_rect().top(),
            self.clip_rect().width(),
            self.clip_rect().height(),
        )
    }

    fn set_clip_area(&mut self, left: i32, top: i32, width: u32, height: u32) {
        let rect = RectArea::with_size(left, top, width, height).intersects(self.self_rect());

        *self.clip_rect_mut() = rect;
    }

    fn reset_clip_area(&mut self) {
        *self.clip_rect_mut() = self.self_rect();
    }

    fn value(&self, x: i32, y: i32) -> T {
        if self.self_rect().contains(x, y) {
            self.data()[y as usize][x as usize]
        } else {
            T::default()
        }
    }

    fn set_value(&mut self, x: i32, y: i32, value: T) {
        if self.self_rect().contains(x, y) {
            self.data_mut()[y as usize][x as usize] = value;
        }
    }

    fn clear(&mut self, value: T) {
        let value = self.render_value(value);

        for i in 0..self.height() {
            for j in 0..self.width() {
                self.data_mut()[i as usize][j as usize] = value;
            }
        }
    }

    fn draw_point(&mut self, x: i32, y: i32, value: T) {
        let value = self.render_value(value);

        if self.self_rect().contains(x, y) {
            self.data_mut()[y as usize][x as usize] = value;
        }
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, value: T) {
        let value = self.render_value(value);

        if x1 == x2 && y1 == y2 {
            self.set_value(x1, y1, value);
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
                self.set_value(
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
                self.set_value(
                    (start_x as f64 + alpha as f64 * i as f64 + 0.5) as i32,
                    start_y + i,
                    value,
                );
            }
        }
    }

    fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, value: T) {
        let value = self.render_value(value);
        let rect = RectArea::with_size(x, y, width, height).intersects(self.clip_rect());

        if rect.is_empty() {
            return;
        }

        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();

        for i in top..=bottom {
            for j in left..=right {
                self.data_mut()[i as usize][j as usize] = value;
            }
        }
    }

    fn draw_rect_border(&mut self, x: i32, y: i32, width: u32, height: u32, value: T) {
        let value = self.render_value(value);
        let rect = RectArea::with_size(x, y, width, height).intersects(self.clip_rect());

        if rect.is_empty() {
            return;
        }

        let left = rect.left();
        let top = rect.top();
        let right = rect.right();
        let bottom = rect.bottom();

        for i in left..=right {
            self.data_mut()[top as usize][i as usize] = value;
            self.data_mut()[bottom as usize][i as usize] = value;
        }

        for i in top..=bottom {
            self.data_mut()[i as usize][left as usize] = value;
            self.data_mut()[i as usize][right as usize] = value;
        }
    }

    fn draw_circle(&mut self, x: i32, y: i32, radius: u32, value: T) {
        let value = self.render_value(value);

        if radius == 0 {
            self.set_value(x, y, value);
            return;
        }

        let sq_radius = radius * radius;

        for dx in 0..=radius as i32 {
            let dy = ((sq_radius as i32 - dx * dx) as f64 + 0.5) as i32;

            if dx > dy {
                continue;
            }

            for i in -dy..=dy {
                self.set_value(x - dx, y + i, value);
                self.set_value(x + dx, y + i, value);
                self.set_value(x + i, y - dx, value);
                self.set_value(x + i, y + dx, value);
            }
        }
    }

    fn draw_circle_border(&mut self, x: i32, y: i32, radius: u32, value: T) {
        let value = self.render_value(value);

        if radius == 0 {
            self.set_value(x, y, value);
            return;
        }

        let sq_radius = radius * radius;

        /*
        for (int32_t dx = 0; dx <= radius; dx++) {
          int32_t dy = std::sqrt(sq_radius - dx * dx) + 0.5f;

          if (dx > dy) {
            continue;
          }

          SetPixel(x - dx, y - dy, draw_value);
          SetPixel(x + dx, y - dy, draw_value);
          SetPixel(x - dx, y + dy, draw_value);
          SetPixel(x + dx, y + dy, draw_value);

          SetPixel(x - dy, y - dx, draw_value);
          SetPixel(x + dy, y - dx, draw_value);
          SetPixel(x - dy, y + dx, draw_value);
          SetPixel(x + dy, y + dx, draw_value);
        }
        */
    }

    fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, value: T) {
        let value = self.render_value(value);

        // rank as y3 > y2 > y1
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

        // slide bottom-up from y1 to y3
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
        let mut y_slider = y1;

        while y_slider <= y2 {
            let mut x_slider;
            let x_end;

            if x_inter < x2 {
                x_slider = (x_inter as f64 + alpha13 * (y_slider - y2) as f64 + 0.5) as i32;
                x_end = (x2 as f64 + alpha12 * (y_slider - y2) as f64 + 0.5) as i32;
            } else {
                x_slider = (x2 as f64 + alpha12 * (y_slider - y2) as f64 + 0.5) as i32;
                x_end = (x_inter as f64 + alpha13 * (y_slider - y2) as f64 + 0.5) as i32;
            }

            while x_slider <= x_end {
                self.set_value(x_slider, y_slider, value);

                x_slider += 1;
            }

            y_slider += 1;
        }

        while y_slider <= y3 {
            let mut x_slider;
            let x_end;

            if x_inter < x2 {
                x_slider = (x_inter as f64 + alpha13 * (y_slider - y2) as f64 + 0.5) as i32;
                x_end = (x2 as f64 + alpha23 * (y_slider - y2) as f64 + 0.5) as i32;
            } else {
                x_slider = (x2 as f64 + alpha23 * (y_slider - y2) as f64 + 0.5) as i32;
                x_end = (x_inter as f64 + alpha13 * (y_slider - y2) as f64 + 0.5) as i32;
            }

            while x_slider <= x_end {
                self.set_value(x_slider, y_slider, value);

                x_slider += 1;
            }

            y_slider += 1;
        }
    }

    fn draw_triangle_border(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        value: T,
    ) {
        self.draw_line(x1, y1, x2, y2, value);
        self.draw_line(x1, y1, x3, y3, value);
        self.draw_line(x2, y2, x3, y3, value);
    }

    fn fill(&mut self, x: i32, y: i32, value: T) {
        let value = self.render_value(value);

        /*
        _col = col if type(col) is int else OverlayCanvas.COLOR_MARK

        self._fill_recursively(x, y, _col, dst)

        if type(col) is not int:
            self._replace_with_tiles(dst, x, y, col)
        */
    }

    fn fill_rec(&mut self) {
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

    fn copy(
        &mut self,
        x: i32,
        y: i32,
        src: &Self,
        u: i32,
        v: i32,
        width: i32,
        height: i32,
        value_key: Option<T>,
    ) {
        let src_rect = src.self_rect();
        let dst_rect = self.self_rect();

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

        if let Some(value_key) = value_key {
            for i in 0..height {
                for j in 0..width {
                    let value = src.value(u + sign_x * j + offset_x, v + sign_y * i + offset_y);

                    if value != value_key {
                        self.set_value(x + j, y + i, self.render_value(value));
                    }
                }
            }
        } else {
            for i in 0..height {
                for j in 0..width {
                    let value = self.render_value(
                        src.value(u + sign_x * j + offset_x, v + sign_y * i + offset_y),
                    );
                    self.set_value(x + j, y + i, value);
                }
            }
        }
    }
}
