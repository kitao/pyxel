use crate::image::{rgb24_to_rgb8, Rgb24};
use crate::pyxel::colors;

// Color LUT: data[col][level] = palette index. Mutable.
// Row count follows pyxel.colors length; column count is fixed at LEVEL_COUNT.

pub const LEVEL_COUNT: usize = 16;

pub struct Ramp {
    pub data: Vec<Vec<i32>>,
}

define_rc_type!(RcRamp, Ramp);

impl Ramp {
    pub fn new() -> RcRamp {
        let data = Self::compute_default();
        new_rc_type!(Ramp { data })
    }

    pub fn build(&mut self) {
        self.data = Self::compute_default();
    }

    pub fn get(&self, col: usize, level: usize) -> i32 {
        self.data[col][level]
    }

    pub fn set(&mut self, col: usize, level: usize, value: i32) {
        self.data[col][level] = value;
    }

    pub fn palette_size(&self) -> usize {
        self.data.len()
    }

    fn compute_default() -> Vec<Vec<i32>> {
        let palette: Vec<Rgb24> = colors().clone();
        let n = palette.len();
        (0..n)
            .map(|col_idx| {
                let (br, bg, bb) = rgb24_to_rgb8(palette[col_idx]);
                let br = br as f32;
                let bg = bg as f32;
                let bb = bb as f32;
                (0..LEVEL_COUNT)
                    .map(|level| {
                        let factor = level as f32 / (LEVEL_COUNT - 1) as f32;
                        Self::nearest(&palette, br * factor, bg * factor, bb * factor)
                    })
                    .collect()
            })
            .collect()
    }

    fn nearest(palette: &[Rgb24], target_r: f32, target_g: f32, target_b: f32) -> i32 {
        let mut best_idx: i32 = 0;
        let mut best_dist = f32::INFINITY;
        for (i, &pal_rgb) in palette.iter().enumerate() {
            let (r, g, b) = rgb24_to_rgb8(pal_rgb);
            let dr = r as f32 - target_r;
            let dg = g as f32 - target_g;
            let db = b as f32 - target_b;
            let dist = dr * dr + dg * dg + db * db;
            if dist < best_dist {
                best_dist = dist;
                best_idx = i as i32;
            }
        }
        best_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dimensions() {
        let r = Ramp::new();
        let r = rc_ref!(&r);
        // Row count matches pyxel.colors length (default 16)
        assert!(r.palette_size() > 0);
        // Each row has 16 levels
        for row in &r.data {
            assert_eq!(row.len(), LEVEL_COUNT);
        }
    }

    #[test]
    fn test_brightest_level_matches_self() {
        let r = Ramp::new();
        let r = rc_ref!(&r);
        // Level 15 (brightness factor = 1.0) should map to the col itself,
        // since the nearest palette color to "col's RGB * 1.0" is col itself.
        for col in 0..r.palette_size() {
            assert_eq!(r.get(col, LEVEL_COUNT - 1), col as i32);
        }
    }

    #[test]
    fn test_get_set() {
        let r = Ramp::new();
        let r_mut = rc_mut!(&r);
        r_mut.set(0, 0, 42);
        assert_eq!(r_mut.get(0, 0), 42);
    }

    #[test]
    fn test_build_resets_table() {
        let r = Ramp::new();
        let r_mut = rc_mut!(&r);
        r_mut.set(0, 0, 99);
        r_mut.build();
        // After rebuild, the modified cell should be back to default.
        assert_ne!(r_mut.get(0, 0), 99);
    }
}
