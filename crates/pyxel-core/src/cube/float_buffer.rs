// 1-D f32 buffer; size changes only through `resize` (cube-design.md §10).

pub struct FloatBuffer {
    data: Vec<f32>,
}

define_rc_type!(RcFloatBuffer, FloatBuffer);

impl FloatBuffer {
    pub fn with_size(size: usize) -> RcFloatBuffer {
        new_rc_type!(FloatBuffer {
            data: vec![0.0; size],
        })
    }

    pub fn from_values(values: Vec<f32>) -> RcFloatBuffer {
        new_rc_type!(FloatBuffer { data: values })
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, i: usize) -> f32 {
        self.data[i]
    }

    pub fn set(&mut self, i: usize, value: f32) {
        self.data[i] = value;
    }

    pub fn fill(&mut self, value: f32) {
        for v in &mut self.data {
            *v = value;
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, 0.0);
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_size_zero_filled() {
        let b = FloatBuffer::with_size(4);
        let r = rc_ref!(&b);
        assert_eq!(r.size(), 4);
        for i in 0..4 {
            assert_eq!(r.get(i), 0.0);
        }
    }

    #[test]
    fn test_from_values() {
        let b = FloatBuffer::from_values(vec![1.0, 2.0, 3.0]);
        let r = rc_ref!(&b);
        assert_eq!(r.size(), 3);
        assert_eq!(r.get(0), 1.0);
        assert_eq!(r.get(2), 3.0);
    }

    #[test]
    fn test_set_get() {
        let b = FloatBuffer::with_size(2);
        rc_mut!(&b).set(0, 1.5);
        rc_mut!(&b).set(1, 2.5);
        let r = rc_ref!(&b);
        assert_eq!(r.get(0), 1.5);
        assert_eq!(r.get(1), 2.5);
    }

    #[test]
    fn test_fill() {
        let b = FloatBuffer::with_size(3);
        rc_mut!(&b).fill(7.0);
        let r = rc_ref!(&b);
        for i in 0..3 {
            assert_eq!(r.get(i), 7.0);
        }
    }

    #[test]
    fn test_resize_grow_zero_fills_tail() {
        let b = FloatBuffer::from_values(vec![1.0, 2.0]);
        rc_mut!(&b).resize(4);
        let r = rc_ref!(&b);
        assert_eq!(r.size(), 4);
        assert_eq!(r.get(0), 1.0);
        assert_eq!(r.get(1), 2.0);
        assert_eq!(r.get(2), 0.0);
        assert_eq!(r.get(3), 0.0);
    }

    #[test]
    fn test_resize_shrink_truncates() {
        let b = FloatBuffer::from_values(vec![1.0, 2.0, 3.0, 4.0]);
        rc_mut!(&b).resize(2);
        let r = rc_ref!(&b);
        assert_eq!(r.size(), 2);
        assert_eq!(r.get(0), 1.0);
        assert_eq!(r.get(1), 2.0);
    }
}
