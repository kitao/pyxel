#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            right: if width == 0 {
                left.saturating_sub(1)
            } else {
                left.saturating_add_unsigned(width - 1)
            },
            bottom: if height == 0 {
                top.saturating_sub(1)
            } else {
                top.saturating_add_unsigned(height - 1)
            },
            width,
            height,
        }
    }

    // Accessors

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

    // Queries

    pub const fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub const fn contains(&self, x: i32, y: i32) -> bool {
        !self.is_empty() && x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }

    pub fn intersection(&self, other: Self) -> Self {
        if self.is_empty() || other.is_empty() {
            return Self::new(0, 0, 0, 0);
        }
        let left = self.left.max(other.left);
        let top = self.top.max(other.top);
        // Intersect logical extents rather than saturated coordinate bounds.
        let right =
            (self.left as i64 + self.width as i64).min(other.left as i64 + other.width as i64);
        let bottom =
            (self.top as i64 + self.height as i64).min(other.top as i64 + other.height as i64);
        let width = right - left as i64;
        let height = bottom - top as i64;

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
    fn test_new() {
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
    fn test_negative_coordinates() {
        let rect = RectArea::new(-10, -5, 20, 10);
        assert_eq!(rect.left(), -10);
        assert_eq!(rect.top(), -5);
        assert_eq!(rect.right(), 9);
        assert_eq!(rect.bottom(), 4);
        assert_eq!(rect.width(), 20);
        assert_eq!(rect.height(), 10);
    }

    #[test]
    fn test_single_pixel() {
        let rect = RectArea::new(5, 5, 1, 1);
        assert_eq!(rect.left(), 5);
        assert_eq!(rect.right(), 5);
        assert_eq!(rect.top(), 5);
        assert_eq!(rect.bottom(), 5);
        assert!(!rect.is_empty());
        assert!(rect.contains(5, 5));
        assert!(!rect.contains(4, 5));
        assert!(!rect.contains(6, 5));
    }

    #[test]
    fn test_empty_at_min_coordinates() {
        for (width, height) in [(0, 1), (1, 0)] {
            let rect = RectArea::new(i32::MIN, i32::MIN, width, height);
            assert!(rect.is_empty());
            assert!(!rect.contains(i32::MIN, i32::MIN));
            assert!(rect.intersection(RectArea::new(0, 0, 2, 2)).is_empty());
        }
    }

    #[test]
    fn test_coordinate_bounds_preserve_extent() {
        let rect = RectArea::new(i32::MAX, i32::MAX, 2, u32::MAX);
        assert_eq!((rect.right(), rect.bottom()), (i32::MAX, i32::MAX));
        assert_eq!((rect.width(), rect.height()), (2, u32::MAX));
        assert!(rect.contains(i32::MAX, i32::MAX));
        assert_eq!(rect.intersection(rect), rect);

        let rect = RectArea::new(i32::MIN, i32::MIN, u32::MAX, u32::MAX);
        assert_eq!((rect.right(), rect.bottom()), (i32::MAX - 1, i32::MAX - 1));
        assert!(rect.contains(0, 0));
        assert!(!rect.contains(i32::MAX, i32::MAX));
        assert_eq!(rect.intersection(rect), rect);
        let small = RectArea::new(0, 0, 2, 2);
        assert_eq!(rect.intersection(small), small);
    }

    #[test]
    fn test_is_empty() {
        assert!(!RectArea::new(1, 2, 3, 4).is_empty());
        assert!(RectArea::new(1, 2, 0, 4).is_empty());
        assert!(RectArea::new(1, 2, 3, 0).is_empty());
        assert!(RectArea::new(0, 0, 0, 0).is_empty());
    }

    // Containment queries

    #[test]
    fn test_contains() {
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
    fn test_contains_negative_coords() {
        let rect = RectArea::new(-5, -5, 10, 10);
        assert!(rect.contains(-5, -5));
        assert!(rect.contains(0, 0));
        assert!(rect.contains(4, 4));
        assert!(!rect.contains(-6, 0));
        assert!(!rect.contains(5, 0));
    }

    // Intersection cases

    #[test]
    fn test_intersection() {
        let rect1 = RectArea::new(10, 20, 30, 40);
        let rect2 = RectArea::new(11, 22, 300, 400);
        let rect3 = RectArea::new(5, 6, 10, 20);
        let rect4 = RectArea::new(1, 2, 3, 4);
        let rect5 = RectArea::new(0, 0, 0, 0);

        assert_eq!(rect1.intersection(rect2), RectArea::new(11, 22, 29, 38));
        assert_eq!(rect1.intersection(rect3), RectArea::new(10, 20, 5, 6));
        assert!(rect1.intersection(rect4).is_empty());
        assert!(rect1.intersection(rect5).is_empty());
    }

    #[test]
    fn test_self_intersection() {
        let rect = RectArea::new(10, 20, 30, 40);
        assert_eq!(rect.intersection(rect), rect);
    }

    #[test]
    fn test_intersection_commutativity() {
        let a = RectArea::new(0, 0, 20, 20);
        let b = RectArea::new(10, 10, 30, 30);
        assert_eq!(a.intersection(b), b.intersection(a));
    }

    #[test]
    fn test_intersection_edge_touching() {
        // Rects share exactly one column of pixels
        let a = RectArea::new(0, 0, 10, 10);
        let b = RectArea::new(9, 0, 10, 10);
        let result = a.intersection(b);
        assert_eq!(result.width(), 1);
        assert_eq!(result.height(), 10);

        // Rects share exactly one row of pixels
        let c = RectArea::new(0, 9, 10, 10);
        let result = a.intersection(c);
        assert_eq!(result.width(), 10);
        assert_eq!(result.height(), 1);
    }

    #[test]
    fn test_intersection_adjacent_no_overlap() {
        let a = RectArea::new(0, 0, 10, 10);
        // The second rect starts just past the first rect's right edge.
        let b = RectArea::new(10, 0, 10, 10);
        assert!(a.intersection(b).is_empty());
    }

    #[test]
    fn test_intersection_distant_coordinates() {
        let a = RectArea::new(i32::MIN, i32::MIN, 1, 1);
        let b = RectArea::new(i32::MAX - 1, i32::MAX - 1, 1, 1);
        assert!(a.intersection(b).is_empty());
        assert!(b.intersection(a).is_empty());
    }
}
