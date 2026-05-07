// 1-D i32 buffer; size changes only through `resize` (cube-design.md §10).

pub struct IntBuffer {
    data: Vec<i32>,
}

define_rc_type!(RcIntBuffer, IntBuffer);

impl IntBuffer {
    pub fn with_size(size: usize) -> RcIntBuffer {
        new_rc_type!(IntBuffer {
            data: vec![0; size],
        })
    }

    pub fn from_values(values: Vec<i32>) -> RcIntBuffer {
        new_rc_type!(IntBuffer { data: values })
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, i: usize) -> i32 {
        self.data[i]
    }

    pub fn set(&mut self, i: usize, value: i32) {
        self.data[i] = value;
    }

    pub fn fill(&mut self, value: i32) {
        for v in &mut self.data {
            *v = value;
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, 0);
    }

    pub fn data(&self) -> &[i32] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [i32] {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_size_zero_filled() {
        let b = IntBuffer::with_size(4);
        let r = rc_ref!(&b);
        assert_eq!(r.size(), 4);
        for i in 0..4 {
            assert_eq!(r.get(i), 0);
        }
    }

    #[test]
    fn test_from_values() {
        let b = IntBuffer::from_values(vec![1, 2, 3]);
        let r = rc_ref!(&b);
        assert_eq!(r.size(), 3);
        assert_eq!(r.get(0), 1);
        assert_eq!(r.get(2), 3);
    }

    #[test]
    fn test_set_get() {
        let b = IntBuffer::with_size(2);
        rc_mut!(&b).set(0, 42);
        rc_mut!(&b).set(1, -7);
        let r = rc_ref!(&b);
        assert_eq!(r.get(0), 42);
        assert_eq!(r.get(1), -7);
    }

    #[test]
    fn test_fill() {
        let b = IntBuffer::with_size(3);
        rc_mut!(&b).fill(99);
        let r = rc_ref!(&b);
        for i in 0..3 {
            assert_eq!(r.get(i), 99);
        }
    }

    #[test]
    fn test_resize_grow_zero_fills_tail() {
        let b = IntBuffer::from_values(vec![1, 2]);
        rc_mut!(&b).resize(4);
        let r = rc_ref!(&b);
        assert_eq!(r.size(), 4);
        assert_eq!(r.get(0), 1);
        assert_eq!(r.get(1), 2);
        assert_eq!(r.get(2), 0);
        assert_eq!(r.get(3), 0);
    }

    #[test]
    fn test_resize_shrink_truncates() {
        let b = IntBuffer::from_values(vec![1, 2, 3, 4]);
        rc_mut!(&b).resize(2);
        let r = rc_ref!(&b);
        assert_eq!(r.size(), 2);
        assert_eq!(r.get(0), 1);
        assert_eq!(r.get(1), 2);
    }
}
