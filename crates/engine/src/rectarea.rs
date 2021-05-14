use std::cmp::{max, min};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rectarea {
    left: i32,
    top: i32,
    width: u32,
    height: u32,
}

impl Rectarea {
    #[inline]
    pub fn with_pos(x1: i32, y1: i32, x2: i32, y2: i32) -> Rectarea {
        let left: i32;
        let top: i32;
        let width: i32;
        let height: i32;

        if x1 < x2 {
            left = x1;
            width = x2 - x1 + 1;
        } else {
            left = x2;
            width = x1 - x2 + 1;
        }

        if y1 < y2 {
            top = y1;
            height = y2 - y1 + 1;
        } else {
            top = y2;
            height = y1 - y2 + 1;
        }

        Rectarea {
            left: left,
            top: top,
            width: width as u32,
            height: height as u32,
        }
    }

    #[inline]
    pub fn with_size(left: i32, top: i32, width: u32, height: u32) -> Rectarea {
        Rectarea {
            left: left,
            top: top,
            width: width,
            height: height,
        }
    }

    #[inline]
    pub fn left(&self) -> i32 {
        self.left
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.top
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.left + self.width as i32 - 1
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.top + self.height as i32 - 1
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.width <= 0 || self.height <= 0
    }

    #[inline]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left
            && x < self.left + self.width as i32
            && y >= self.top
            && y < self.top + self.height as i32
    }

    #[inline]
    pub fn intersects(&self, rect: Rectarea) -> Rectarea {
        let left = max(self.left, rect.left);
        let top = max(self.top, rect.top);
        let right = min(self.right(), rect.right());
        let bottom = min(self.bottom(), rect.bottom());
        let width = right - left + 1;
        let height = bottom - top + 1;

        if width > 0 && height > 0 {
            Rectarea::with_size(left, top, width as u32, height as u32)
        } else {
            Rectarea::with_size(0, 0, 0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_pos() {
        let rect1 = Rectarea::with_pos(0, 0, 0, 0);
        assert_eq!(rect1.left(), 0);
        assert_eq!(rect1.top(), 0);
        assert_eq!(rect1.right(), 0);
        assert_eq!(rect1.bottom(), 0);
        assert_eq!(rect1.width(), 1);
        assert_eq!(rect1.height(), 1);

        let rect2 = Rectarea::with_pos(1, 2, 30, 40);
        assert_eq!(rect2.left(), 1);
        assert_eq!(rect2.top(), 2);
        assert_eq!(rect2.right(), 30);
        assert_eq!(rect2.bottom(), 40);
        assert_eq!(rect2.width(), 30);
        assert_eq!(rect2.height(), 39);

        let rect3 = Rectarea::with_pos(10, 20, 3, 4);
        assert_eq!(rect3.left(), 3);
        assert_eq!(rect3.top(), 4);
        assert_eq!(rect3.right(), 10);
        assert_eq!(rect3.bottom(), 20);
        assert_eq!(rect3.width(), 8);
        assert_eq!(rect3.height(), 17);
    }

    #[test]
    fn with_size() {
        let rect1 = Rectarea::with_size(1, 2, 3, 4);
        assert_eq!(rect1.left(), 1);
        assert_eq!(rect1.top(), 2);
        assert_eq!(rect1.right(), 3);
        assert_eq!(rect1.bottom(), 5);
        assert_eq!(rect1.width(), 3);
        assert_eq!(rect1.height(), 4);

        let rect2 = Rectarea::with_size(10, 20, 0, 40);
        assert_eq!(rect2.left(), 10);
        assert_eq!(rect2.top(), 20);
        assert_eq!(rect2.right(), 9);
        assert_eq!(rect2.bottom(), 59);
        assert_eq!(rect2.width(), 0);
        assert_eq!(rect2.height(), 40);

        let rect3 = Rectarea::with_size(100, 200, 300, 0);
        assert_eq!(rect3.left(), 100);
        assert_eq!(rect3.top(), 200);
        assert_eq!(rect3.right(), 399);
        assert_eq!(rect3.bottom(), 199);
        assert_eq!(rect3.width(), 300);
        assert_eq!(rect3.height(), 0);
    }

    #[test]
    fn is_empty() {
        let rect1 = Rectarea::with_size(1, 2, 3, 4);
        assert!(!rect1.is_empty());

        let rect2 = Rectarea::with_size(1, 2, 0, 4);
        assert!(rect2.is_empty());

        let rect3 = Rectarea::with_size(1, 2, 3, 0);
        assert!(rect3.is_empty());
    }

    #[test]
    fn contains() {
        let rect1 = Rectarea::with_pos(1, 2, 3, 4);
        assert!(rect1.contains(1, 2));
        assert!(rect1.contains(3, 4));
        assert!(!rect1.contains(0, 2));
        assert!(!rect1.contains(1, 1));
        assert!(!rect1.contains(4, 4));
        assert!(!rect1.contains(3, 5));

        let rect2 = Rectarea::with_size(1, 2, 0, 4);
        assert!(!rect2.contains(1, 2));
        assert!(!rect2.contains(1, 4));

        let rect3 = Rectarea::with_size(1, 2, 3, 0);
        assert!(!rect3.contains(1, 2));
        assert!(!rect3.contains(3, 2));
    }

    #[test]
    fn intersects() {
        let rect1 = Rectarea::with_size(10, 20, 30, 40);
        let rect2 = Rectarea::with_size(11, 22, 300, 400);
        let rect3 = Rectarea::with_size(5, 6, 10, 20);
        let rect4 = Rectarea::with_size(1, 2, 3, 4);
        let rect5 = Rectarea::with_size(0, 0, 0, 0);

        assert_eq!(rect1.intersects(rect2), Rectarea::with_pos(11, 22, 39, 59));
        assert_eq!(rect1.intersects(rect3), Rectarea::with_pos(10, 20, 14, 25));
        assert!(rect1.intersects(rect4).is_empty());
        assert!(rect1.intersects(rect5).is_empty());
    }
}
