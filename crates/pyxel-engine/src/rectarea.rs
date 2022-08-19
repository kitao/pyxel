use std::cmp::{max, min};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RectArea {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    width: u32,
    height: u32,
}

impl RectArea {
    pub const fn new(left: i32, top: i32, width: u32, height: u32) -> Self {
        Self {
            left,
            top,
            right: left + width as i32 - 1,
            bottom: top + height as i32 - 1,
            width,
            height,
        }
    }

    pub const fn left(&self) -> i32 {
        self.left
    }

    pub const fn top(&self) -> i32 {
        self.top
    }

    pub const fn right(&self) -> i32 {
        self.right
    }

    pub const fn bottom(&self) -> i32 {
        self.bottom
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub const fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left
            && x < self.left + self.width as i32
            && y >= self.top
            && y < self.top + self.height as i32
    }

    pub fn intersects(&self, rect: Self) -> Self {
        let left = max(self.left, rect.left);
        let top = max(self.top, rect.top);
        let right = min(self.right, rect.right);
        let bottom = min(self.bottom, rect.bottom);
        let width = right - left + 1;
        let height = bottom - top + 1;
        if width > 0 && height > 0 {
            Self::new(left, top, width as u32, height as u32)
        } else {
            Self::new(0, 0, 0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let rect1 = RectArea::new(1, 2, 3, 4);
        assert_eq!(rect1.left(), 1);
        assert_eq!(rect1.top(), 2);
        assert_eq!(rect1.right(), 3);
        assert_eq!(rect1.bottom(), 5);
        assert_eq!(rect1.width(), 3);
        assert_eq!(rect1.height(), 4);

        let rect2 = RectArea::new(10, 20, 0, 40);
        assert_eq!(rect2.left(), 10);
        assert_eq!(rect2.top(), 20);
        assert_eq!(rect2.right(), 9);
        assert_eq!(rect2.bottom(), 59);
        assert_eq!(rect2.width(), 0);
        assert_eq!(rect2.height(), 40);

        let rect3 = RectArea::new(100, 200, 300, 0);
        assert_eq!(rect3.left(), 100);
        assert_eq!(rect3.top(), 200);
        assert_eq!(rect3.right(), 399);
        assert_eq!(rect3.bottom(), 199);
        assert_eq!(rect3.width(), 300);
        assert_eq!(rect3.height(), 0);
    }

    #[test]
    fn is_empty() {
        let rect1 = RectArea::new(1, 2, 3, 4);
        assert!(!rect1.is_empty());

        let rect2 = RectArea::new(1, 2, 0, 4);
        assert!(rect2.is_empty());

        let rect3 = RectArea::new(1, 2, 3, 0);
        assert!(rect3.is_empty());
    }

    #[test]
    fn contains() {
        let rect1 = RectArea::new(1, 2, 3, 3);
        assert!(rect1.contains(1, 2));
        assert!(rect1.contains(3, 4));
        assert!(!rect1.contains(0, 2));
        assert!(!rect1.contains(1, 1));
        assert!(!rect1.contains(4, 4));
        assert!(!rect1.contains(3, 5));

        let rect2 = RectArea::new(1, 2, 0, 4);
        assert!(!rect2.contains(1, 2));
        assert!(!rect2.contains(1, 4));

        let rect3 = RectArea::new(1, 2, 3, 0);
        assert!(!rect3.contains(1, 2));
        assert!(!rect3.contains(3, 2));
    }

    #[test]
    fn intersects() {
        let rect1 = RectArea::new(10, 20, 30, 40);
        let rect2 = RectArea::new(11, 22, 300, 400);
        let rect3 = RectArea::new(5, 6, 10, 20);
        let rect4 = RectArea::new(1, 2, 3, 4);
        let rect5 = RectArea::new(0, 0, 0, 0);
        assert_eq!(rect1.intersects(rect2), RectArea::new(11, 22, 29, 38));
        assert_eq!(rect1.intersects(rect3), RectArea::new(10, 20, 5, 6));
        assert!(rect1.intersects(rect4).is_empty());
        assert!(rect1.intersects(rect5).is_empty());
    }
}
