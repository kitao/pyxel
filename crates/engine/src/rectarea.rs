use std::cmp::{max, min};

pub struct RectArea {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl RectArea {
    #[inline]
    pub fn new_with_pos(x1: i32, y1: i32, x2: i32, y2: i32) -> RectArea {
        let mut rect = RectArea {
            left: 0,
            top: 0,
            width: 0,
            height: 0,
        };

        if x1 < x2 {
            rect.left = x1;
            rect.width = x2 - x1 + 1;
        } else {
            rect.left = x2;
            rect.width = x1 - x2 + 1;
        }

        if y1 < y2 {
            rect.top = y1;
            rect.height = y2 - y1 + 1;
        } else {
            rect.top = y2;
            rect.height = y1 - y2 + 1;
        }

        rect
    }

    #[inline]
    pub fn new_with_size(left: i32, top: i32, width: i32, height: i32) -> RectArea {
        assert!(width >= 0 && height >= 0);

        RectArea {
            left: left,
            top: top,
            width: width,
            height: height,
        }
    }

    #[inline]
    pub fn get_left(&self) -> i32 {
        self.left
    }

    #[inline]
    pub fn get_top(&self) -> i32 {
        self.top
    }

    #[inline]
    pub fn get_right(&self) -> i32 {
        self.left + self.width - 1
    }

    #[inline]
    pub fn get_bottom(&self) -> i32 {
        self.top + self.height - 1
    }

    #[inline]
    pub fn get_width(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn get_height(&self) -> i32 {
        self.height
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.width <= 0 || self.height <= 0
    }

    #[inline]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left && x < self.left + self.width && y >= self.top && y < self.top + self.height
    }

    #[inline]
    pub fn intersects(&self, rect: &RectArea) -> RectArea {
        let left = max(self.left, rect.left);
        let top = max(self.top, rect.top);
        let right = min(self.get_right(), rect.get_right());
        let bottom = min(self.get_bottom(), rect.get_bottom());
        let width = right - left + 1;
        let height = bottom - top + 1;

        if width > 0 && height > 0 {
            RectArea {
                left: left,
                top: top,
                width: width,
                height: height,
            }
        } else {
            RectArea {
                left: 0,
                top: 0,
                width: 0,
                height: 0,
            }
        }
    }
}
