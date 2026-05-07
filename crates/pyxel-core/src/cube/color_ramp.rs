use crate::image::{rgb24_to_rgb8, Rgb24};
use crate::pyxel::colors;

// Color LUT mapping (col, level) to a (primary, secondary, ratio) triple.
// `primary` and `secondary` are palette indices; `ratio` is 0..RATIO_COUNT
// describing how often `secondary` wins over `primary` in the 4x4 Bayer
// dither pattern. `primary == secondary` collapses to a flat fill.

pub const LEVEL_COUNT: usize = 16;
// 4x4 Bayer matrix has 16 distinct thresholds, giving 16 dither
// gradations. Matches LEVEL_COUNT so the brightness ramp and the dither
// resolution share the same step count.
pub const RATIO_COUNT: u8 = 16;

// (primary, secondary, ratio) — primary/secondary are palette indices,
// ratio is the count of 4x4 Bayer cells that pick `secondary`. Kept
// file-private so callers see plain (i32, i32, u8) tuples on the API
// surface and the ColorRamp module owns the layout.
type Entry = (i32, i32, u8);

pub struct ColorRamp {
    data: Vec<Vec<Entry>>,
}

define_rc_type!(RcColorRamp, ColorRamp);

impl ColorRamp {
    pub fn new() -> RcColorRamp {
        let data = Self::compute_default();
        new_rc_type!(ColorRamp { data })
    }

    pub fn build(&mut self) {
        self.data = Self::compute_default();
    }

    pub fn get(&self, col: usize, level: usize) -> (i32, i32, u8) {
        self.data[col][level]
    }

    pub fn set(&mut self, col: usize, level: usize, value: (i32, i32, u8)) {
        self.data[col][level] = value;
    }

    pub fn palette_size(&self) -> usize {
        self.data.len()
    }

    fn compute_default() -> Vec<Vec<Entry>> {
        let palette: Vec<Rgb24> = colors().clone();
        let rgb: Vec<(f32, f32, f32)> = palette
            .iter()
            .map(|&p| {
                let (r, g, b) = rgb24_to_rgb8(p);
                (r as f32, g as f32, b as f32)
            })
            .collect();
        (0..rgb.len())
            .map(|col_idx| {
                (0..LEVEL_COUNT)
                    .map(|level| Self::resolve(&rgb, col_idx as i32, level))
                    .collect()
            })
            .collect()
    }

    // Brute-force the best (primary, secondary, ratio) triple whose
    // averaged RGB is closest to `base * factor`. Every primary is a
    // candidate (not anchored), every secondary is a candidate, every
    // ratio in 0..RATIO_COUNT is tried. ratio == 0 collapses to a flat
    // `primary` cell — the search naturally falls back to that when no
    // mix improves on a single color.
    fn resolve(rgb: &[(f32, f32, f32)], col: i32, level: usize) -> Entry {
        let factor = level as f32 / (LEVEL_COUNT - 1) as f32;
        let (br, bg, bb) = rgb[col as usize];
        let tr = br * factor;
        let tg = bg * factor;
        let tb = bb * factor;
        let mut best: Entry = (0, 0, 0);
        let mut best_dist = f32::INFINITY;
        let inv_count = 1.0 / RATIO_COUNT as f32;
        for i in 0..rgb.len() {
            let (ar, ag, ab) = rgb[i];
            for j in 0..rgb.len() {
                let (sr, sg, sb) = rgb[j];
                for ratio in 0..RATIO_COUNT {
                    let t = ratio as f32 * inv_count;
                    let mr = ar + (sr - ar) * t;
                    let mg = ag + (sg - ag) * t;
                    let mb = ab + (sb - ab) * t;
                    let dr = mr - tr;
                    let dg = mg - tg;
                    let db = mb - tb;
                    let dist = dr * dr + dg * dg + db * db;
                    if dist < best_dist {
                        best_dist = dist;
                        best = (i as i32, j as i32, ratio);
                    }
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dimensions() {
        let r = ColorRamp::new();
        let r = rc_ref!(&r);
        assert!(r.palette_size() > 0);
        for row in &r.data {
            assert_eq!(row.len(), LEVEL_COUNT);
        }
    }

    #[test]
    fn test_brightest_level_matches_self() {
        // Level 15 (factor 1.0) → target == col self → primary == col,
        // ratio == 0 (flat fill, secondary is irrelevant in that case).
        let r = ColorRamp::new();
        let r = rc_ref!(&r);
        for col in 0..r.palette_size() {
            let (primary, _, ratio) = r.get(col, LEVEL_COUNT - 1);
            assert_eq!(primary, col as i32);
            assert_eq!(ratio, 0);
        }
    }

    #[test]
    fn test_get_set() {
        let r = ColorRamp::new();
        let r_mut = rc_mut!(&r);
        r_mut.set(0, 0, (5, 7, 8));
        assert_eq!(r_mut.get(0, 0), (5, 7, 8));
    }

    #[test]
    fn test_build_resets_table() {
        let r = ColorRamp::new();
        let r_mut = rc_mut!(&r);
        r_mut.set(0, 0, (99, 99, 0));
        r_mut.build();
        let (p, s, _) = r_mut.get(0, 0);
        assert!(p != 99 || s != 99);
    }

    #[test]
    fn test_ratio_within_bounds() {
        let r = ColorRamp::new();
        let r = rc_ref!(&r);
        for row in &r.data {
            for &(_, _, ratio) in row {
                assert!(ratio < RATIO_COUNT);
            }
        }
    }
}
