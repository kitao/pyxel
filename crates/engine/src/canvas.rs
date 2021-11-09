use std::cmp::max;
use std::mem::swap;
use std::ptr::eq;

use crate::rectarea::RectArea;
use crate::utils::{as_i32, as_u32};

pub trait ToIndex {
    fn to_index(&self) -> usize;
}

pub struct Canvas<T: Copy + PartialEq + Default + ToIndex> {
    pub self_rect: RectArea,
    pub clip_rect: RectArea,
    pub data: Vec<Vec<T>>,
}

impl<T: Copy + PartialEq + Default + ToIndex> Canvas<T> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
            data: vec![vec![T::default(); width as usize]; height as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.self_rect.width()
    }

    pub fn height(&self) -> u32 {
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

    pub fn cls(&mut self, value: T) {
        let width = self.width();
        let height = self.height();
        for y in 0..height {
            for x in 0..width {
                self.data[y as usize][x as usize] = value;
            }
        }
    }

    pub fn pget(&mut self, x: f64, y: f64) -> T {
        let x = as_i32(x);
        let y = as_i32(y);
        if self.self_rect.contains(x, y) {
            self.data[y as usize][x as usize]
        } else {
            T::default()
        }
    }

    pub fn pset(&mut self, x: f64, y: f64, value: T) {
        let x = as_i32(x);
        let y = as_i32(y);
        if self.clip_rect.contains(x, y) {
            self.data[y as usize][x as usize] = value;
        }
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, value: T) {
        let x1 = as_i32(x1);
        let y1 = as_i32(y1);
        let x2 = as_i32(x2);
        let y2 = as_i32(y2);

        if x1 == x2 && y1 == y2 {
            self.pset(x1 as f64, y1 as f64, value);
        } else if (x1 - x2).abs() > (y1 - y2).abs() {
            let (start_x, start_y, end_x, end_y) = if x1 < x2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };
            let length = end_x - start_x + 1;
            let alpha = (end_y - start_y) as f64 / (end_x - start_x) as f64;
            for xi in 0..length {
                self.pset(
                    (start_x + xi) as f64,
                    start_y as f64 + alpha * xi as f64,
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
                self.pset(
                    start_x as f64 + alpha * yi as f64,
                    (start_y + yi) as f64,
                    value,
                );
            }
        }
    }

    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = as_i32(x);
        let y = as_i32(y);
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
                self.data[y as usize][x as usize] = value;
            }
        }
    }

    pub fn rectb(&mut self, x: f64, y: f64, width: f64, height: f64, value: T) {
        let x = as_i32(x);
        let y = as_i32(y);
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
            self.pset(x as f64, top as f64, value);
            self.pset(x as f64, bottom as f64, value);
        }
        for y in top..=bottom {
            self.pset(left as f64, y as f64, value);
            self.pset(right as f64, y as f64, value);
        }
    }

    pub fn circ(&mut self, x: f64, y: f64, radius: f64, value: T) {
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

    pub fn circb(&mut self, x: f64, y: f64, radius: f64, value: T) {
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

    pub fn tri(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, value: T) {
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
                self.pset(x as f64, y as f64, value);
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
                self.pset(x as f64, y as f64, value);
            }
        }
    }

    pub fn trib(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, value: T) {
        self.line(x1, y1, x2, y2, value);
        self.line(x1, y1, x3, y3, value);
        self.line(x2, y2, x3, y3, value);
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
        if eq(self as *const Self, canvas as *const Self) {
            let copy_width = as_u32(width.abs());
            let copy_height = as_u32(height.abs());
            let mut canvas = Self::new(copy_width, copy_height);
            canvas.blt(
                0.0,
                0.0,
                self,
                canvas_x,
                canvas_y,
                copy_width as f64,
                copy_height as f64,
                None,
                None,
            );
            self.blt(x, y, &canvas, 0.0, 0.0, width, height, transparent, palette);
            return;
        }

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

        for i in 0..height {
            for j in 0..width {
                let value_x = src_x + sign_x * j + offset_x;
                let value_y = src_y + sign_y * i + offset_y;
                let value = canvas.data[value_y as usize][value_x as usize];
                if let Some(transparent) = transparent {
                    if value == transparent {
                        continue;
                    }
                }
                let value = if let Some(palette) = palette {
                    palette[value.to_index()]
                } else {
                    value
                };
                self.data[(dst_y + i) as usize][(dst_x + j) as usize] = value;
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
