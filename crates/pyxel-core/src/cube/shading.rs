// Color math uses the conventional r/g/b and h/s/v component names.
#![allow(clippy::many_single_char_names)]

use crate::cube::vec3::{RcVec3, Vec3};
use crate::image::{rgb24_to_rgb8, Rgb24};

// Palette-derived shading LUT and scene-wide light direction. Each cell is
// either flat or a 50:50 dither pair.

pub const LEVEL_COUNT: usize = 4;

type Entry = (i32, i32);

pub struct Shading {
    data: Vec<[Entry; LEVEL_COUNT]>,
    pub direction: RcVec3,
}

define_rc_type!(RcShading, Shading);

impl Shading {
    pub fn new(palette: &[Rgb24]) -> RcShading {
        let data = Self::compute(palette);
        new_rc_type!(Shading {
            data,
            direction: Vec3::down(),
        })
    }

    pub fn build(&mut self, palette: &[Rgb24]) {
        self.data = Self::compute(palette);
    }

    pub fn get(&self, col: usize, level: usize) -> Entry {
        self.data[col][level]
    }

    pub fn set(&mut self, col: usize, level: usize, value: Entry) {
        self.data[col][level] = value;
    }

    pub fn palette_size(&self) -> usize {
        self.data.len()
    }

    fn compute(palette: &[Rgb24]) -> Vec<[Entry; LEVEL_COUNT]> {
        // Score palette candidates against HSV targets shifted by STEP;
        // candidates beyond REJECT_THRESHOLD fall back to the source color.
        const STEP: f32 = 0.01;
        const V_DARK_TERMINAL: f32 = 0.05;
        // A 45-degree hue gap at full saturation reaches the rejection score.
        const WH: f32 = 8.0;
        const WS: f32 = 1.0;
        const WV: f32 = 1.0;
        // Dropping a source hue is penalized more than adding an accent to an
        // achromatic source, whose hue is undefined.
        const ACHROMATIC_THRESHOLD: f32 = 0.05;
        const CROSSING_C2A: f32 = 0.80;
        const CROSSING_A2C: f32 = 0.10;
        const REJECT_THRESHOLD: f32 = 1.0;
        // Permit a slightly weaker flat lv 1 when it enables a better lv 0 pair.
        const PATTERN_C_LV1_SLACK: f32 = 1.1;

        let n = palette.len();
        if n == 0 {
            return vec![];
        }
        // Precompute linear RGB for optical blends and HSV for scoring.
        let lin: Vec<(f32, f32, f32)> = palette
            .iter()
            .map(|&p| {
                let (r, g, b) = rgb24_to_rgb8(p);
                (
                    srgb_to_linear(r as f32 / 255.0),
                    srgb_to_linear(g as f32 / 255.0),
                    srgb_to_linear(b as f32 / 255.0),
                )
            })
            .collect();
        let hsv: Vec<(f32, f32, f32)> = palette
            .iter()
            .map(|&p| {
                let (r, g, b) = rgb24_to_rgb8(p);
                rgb_to_hsv(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
            })
            .collect();
        // Use relative luminance rather than HSV value for perceptual ordering.
        let luma: Vec<f32> = lin
            .iter()
            .map(|&(r, g, b)| 0.2126 * r + 0.7152 * g + 0.0722 * b)
            .collect();
        let entry_luma = |entry: Entry| -> f32 {
            let (p, s) = entry;
            if p == s {
                luma[p as usize]
            } else {
                (luma[p as usize] + luma[s as usize]) * 0.5
            }
        };
        // Scale the circular hue gap by the lower saturation, then apply the
        // directional chromatic/achromatic crossing penalty.
        let distance = |a: (f32, f32, f32), b: (f32, f32, f32)| -> f32 {
            let raw_dh = (a.0 - b.0).abs();
            let dh = raw_dh.min(1.0 - raw_dh) * a.1.min(b.1);
            let ds = a.1 - b.1;
            let dv = a.2 - b.2;
            let th = WH * dh;
            let ts = WS * ds;
            let tv = WV * dv;
            let raw = (th * th + ts * ts + tv * tv).sqrt();
            let a_chromatic = a.1 >= ACHROMATIC_THRESHOLD;
            let b_chromatic = b.1 >= ACHROMATIC_THRESHOLD;
            let crossing = if a_chromatic && !b_chromatic {
                CROSSING_C2A
            } else if !a_chromatic && b_chromatic {
                CROSSING_A2C
            } else {
                0.0
            };
            raw + crossing
        };
        // Score a dither by its optical blend in linear RGB, not by either
        // constituent in isolation.
        let entry_hsv = |entry: Entry| -> (f32, f32, f32) {
            let (p, s) = entry;
            if p == s {
                hsv[p as usize]
            } else {
                let (rp, gp, bp) = lin[p as usize];
                let (rs, gs, bs) = lin[s as usize];
                let r = linear_to_srgb((rp + rs) * 0.5);
                let g = linear_to_srgb((gp + gs) * 0.5);
                let b = linear_to_srgb((bp + bs) * 0.5);
                rgb_to_hsv(r, g, b)
            }
        };
        // Pick the lowest-total-distance connected pattern for lv 1 and lv 0.
        //
        //   Pattern A: lv 0 flat, lv 1 flat (= no dither at all)
        //   Pattern B: lv 0 flat X, lv 1 dither (source, X)
        //   Pattern C: lv 0 dither (X, Y), lv 1 flat X
        //
        // Patterns B and C share X across levels to keep the ramp continuous.
        let pick_two_pattern = |source: usize,
                                ideal_1: (f32, f32, f32),
                                ideal_0: (f32, f32, f32)|
         -> Option<(Entry, Entry)> {
            let base_entry: Entry = (source as i32, source as i32);
            let source_luma = luma[source];
            // Reject equal-luma duplicates so each shade is visibly darker.
            let darker_than_source = |entry: Entry| -> bool { entry_luma(entry) < source_luma };
            // Use the best independent flat as Pattern C's lv 1 quality gate.
            let mut lv1_solo_best = f32::INFINITY;
            for t in 0..n {
                if t == source {
                    continue;
                }
                let lv1: Entry = (t as i32, t as i32);
                if !darker_than_source(lv1) {
                    continue;
                }
                let s = distance(ideal_1, entry_hsv(lv1));
                if s < lv1_solo_best {
                    lv1_solo_best = s;
                }
            }
            let lv1_gate = lv1_solo_best * PATTERN_C_LV1_SLACK;
            // Pattern A: two independent flats with monotone luminance
            let mut best_a: (f32, Entry, Entry) = (f32::INFINITY, base_entry, base_entry);
            for t1 in 0..n {
                if t1 == source {
                    continue;
                }
                let lv1: Entry = (t1 as i32, t1 as i32);
                if !darker_than_source(lv1) {
                    continue;
                }
                let l1 = entry_luma(lv1);
                let s1 = distance(ideal_1, entry_hsv(lv1));
                for (t0, &t0_luma) in luma.iter().enumerate() {
                    if t0 == source || t0_luma >= l1 {
                        continue;
                    }
                    let lv0: Entry = (t0 as i32, t0 as i32);
                    let s = s1 + distance(ideal_0, entry_hsv(lv0));
                    if s < best_a.0 {
                        best_a = (s, lv1, lv0);
                    }
                }
            }
            // Pattern B: lv 1 = (source, X), lv 0 = X
            let mut best_b: (f32, Entry, Entry) = (f32::INFINITY, base_entry, base_entry);
            for x in 0..n {
                if x == source {
                    continue;
                }
                if distance(hsv[source], hsv[x]) > REJECT_THRESHOLD {
                    continue;
                }
                let lv1: Entry = (source as i32, x as i32);
                let lv0: Entry = (x as i32, x as i32);
                if !darker_than_source(lv1) || !darker_than_source(lv0) {
                    continue;
                }
                let s = distance(ideal_1, entry_hsv(lv1)) + distance(ideal_0, entry_hsv(lv0));
                if s < best_b.0 {
                    best_b = (s, lv1, lv0);
                }
            }
            // Pattern C: lv 1 = X, lv 0 = (X, Y), with compatible colors
            let mut best_c: (f32, Entry, Entry) = (f32::INFINITY, base_entry, base_entry);
            for x in 0..n {
                if x == source {
                    continue;
                }
                let lv1: Entry = (x as i32, x as i32);
                if !darker_than_source(lv1) {
                    continue;
                }
                let s1 = distance(ideal_1, entry_hsv(lv1));
                if s1 > lv1_gate {
                    continue;
                }
                let lx = luma[x];
                for y in 0..n {
                    if y == source || y == x {
                        continue;
                    }
                    if luma[y] >= lx {
                        continue;
                    }
                    if distance(hsv[x], hsv[y]) > REJECT_THRESHOLD {
                        continue;
                    }
                    let lv0: Entry = (x as i32, y as i32);
                    let s = s1 + distance(ideal_0, entry_hsv(lv0));
                    if s < best_c.0 {
                        best_c = (s, lv1, lv0);
                    }
                }
            }
            let (total, lv1, lv0) = if best_a.0 <= best_b.0 && best_a.0 <= best_c.0 {
                (best_a.0, best_a.1, best_a.2)
            } else if best_b.0 <= best_c.0 {
                (best_b.0, best_b.1, best_b.2)
            } else {
                (best_c.0, best_c.1, best_c.2)
            };
            if total > REJECT_THRESHOLD * 2.0 {
                None
            } else {
                Some((lv1, lv0))
            }
        };
        // Pick lv 3 independently from a flat or source/candidate dither.
        let pick_one_pattern = |source: usize, ideal: (f32, f32, f32)| -> Option<Entry> {
            let base_entry: Entry = (source as i32, source as i32);
            let mut best_score = f32::INFINITY;
            let mut best_entry = base_entry;
            let source_luma = luma[source];
            let brighter_than_source = |entry: Entry| -> bool { entry_luma(entry) > source_luma };
            for t in 0..n {
                if t == source {
                    continue;
                }
                let f: Entry = (t as i32, t as i32);
                if brighter_than_source(f) {
                    let s_f = distance(ideal, entry_hsv(f));
                    if s_f < best_score {
                        best_score = s_f;
                        best_entry = f;
                    }
                }
                let d: Entry = (source as i32, t as i32);
                if brighter_than_source(d) && distance(hsv[source], hsv[t]) <= REJECT_THRESHOLD {
                    let s_d = distance(ideal, entry_hsv(d));
                    if s_d < best_score {
                        best_score = s_d;
                        best_entry = d;
                    }
                }
            }
            if best_score > REJECT_THRESHOLD {
                None
            } else {
                Some(best_entry)
            }
        };
        let flat = |idx: usize| -> Entry { (idx as i32, idx as i32) };
        (0..n)
            .map(|c| {
                let (h, s, v) = hsv[c];
                let mut row = [flat(c); LEVEL_COUNT];

                // Desaturate saturated colors so their highlight moves toward white.
                let ideal_3 = if v + STEP <= 1.0 {
                    (h, s, v + STEP)
                } else if s > 0.0 {
                    (h, (s - STEP).max(0.0), v)
                } else {
                    (h, s, v)
                };
                if ideal_3 != (h, s, v) {
                    if let Some(entry) = pick_one_pattern(c, ideal_3) {
                        row[3] = entry;
                    }
                }

                // Target one and two STEP decrements for lv 1 and lv 0.
                let ideal_1_v = (v - STEP).max(V_DARK_TERMINAL);
                let ideal_0_v = (v - 2.0 * STEP).max(V_DARK_TERMINAL);
                if ideal_1_v < v {
                    if let Some((lv1, lv0)) =
                        pick_two_pattern(c, (h, s, ideal_1_v), (h, s, ideal_0_v))
                    {
                        row[1] = lv1;
                        row[0] = lv0;
                    }
                }

                row
            })
            .collect()
    }
}

// Color-space helpers

// sRGB transfer curve (per-channel gamma decode). Input/output in 0..1.
#[inline]
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

// sRGB transfer curve (per-channel gamma encode). Input/output in 0..1.
#[inline]
fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

// HSV from sRGB. Input r, g, b in 0..1 sRGB. Output (H, S, V) with H in
// 0..1 (cyclic, 0 = red), S and V in 0..1.
#[inline]
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        (((g - b) / delta).rem_euclid(6.0)) / 6.0
    } else if max == g {
        ((b - r) / delta + 2.0) / 6.0
    } else {
        ((r - g) / delta + 4.0) / 6.0
    };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    (h, s, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pyxel_default() -> Vec<Rgb24> {
        vec![
            0x000000, 0x2B335F, 0x7E2072, 0x19959C, 0x8B4852, 0x395C98, 0xA9C1FF, 0xEEEEEE,
            0xD4186C, 0xD38441, 0xE9C35B, 0x70C6A9, 0x7696DE, 0xA3A3A3, 0xFF9798, 0xEDC7B0,
        ]
    }

    #[test]
    fn test_default_dimensions() {
        let r = Shading::new(&pyxel_default());
        let r = rc_ref!(&r);
        assert_eq!(r.palette_size(), 16);
        for row in &r.data {
            assert_eq!(row.len(), LEVEL_COUNT);
        }
    }

    #[test]
    fn test_get_set() {
        let r = Shading::new(&pyxel_default());
        let mut r_mut = rc_mut!(&r);
        r_mut.set(0, 0, (5, 7));
        assert_eq!(r_mut.get(0, 0), (5, 7));
    }

    #[test]
    fn test_build_resets_table() {
        let r = Shading::new(&pyxel_default());
        let mut r_mut = rc_mut!(&r);
        r_mut.set(0, 0, (99, 99));
        r_mut.build(&pyxel_default());
        // Rebuild recomputes the deterministic default-palette entry:
        // black has no darker shade, so lv 0 stays flat black.
        assert_eq!(r_mut.get(0, 0), (0, 0));
    }

    #[test]
    fn test_empty_palette_returns_empty_data() {
        let r = Shading::new(&[]);
        let r = rc_ref!(&r);
        assert_eq!(r.palette_size(), 0);
    }

    #[test]
    fn test_direction_default_is_down() {
        let r = Shading::new(&pyxel_default());
        let r = rc_ref!(&r);
        let dir = *rc_ref!(&r.direction);
        assert_eq!(dir.x, 0.0);
        assert_eq!(dir.y, -1.0);
        assert_eq!(dir.z, 0.0);
    }

    fn srgb_to_linear(c: f32) -> f32 {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    fn entry_luma(palette: &[Rgb24], primary: i32, secondary: i32) -> f32 {
        let component =
            |idx: i32, shift: u32| ((palette[idx as usize] >> shift) & 0xFF) as f32 / 255.0;
        let pixel_luma = |idx: i32| {
            let r = srgb_to_linear(component(idx, 16));
            let g = srgb_to_linear(component(idx, 8));
            let b = srgb_to_linear(component(idx, 0));
            0.2126 * r + 0.7152 * g + 0.0722 * b
        };
        if primary == secondary {
            pixel_luma(primary)
        } else {
            (pixel_luma(primary) + pixel_luma(secondary)) * 0.5
        }
    }

    #[test]
    fn test_pyxel_default_ramp_is_monotone() {
        // Default palette must produce a strictly non-decreasing luma
        // ramp lv 0 ≤ lv 1 ≤ lv 2 ≤ lv 3 for every column.
        let pal = pyxel_default();
        let r = Shading::new(&pal);
        let r = rc_ref!(&r);
        for col in 0..pal.len() {
            let lumas: [f32; 4] = std::array::from_fn(|lv| {
                let (p, s) = r.get(col, lv);
                entry_luma(&pal, p, s)
            });
            for lv in 1..4 {
                assert!(
                    lumas[lv] >= lumas[lv - 1] - 1e-6,
                    "col {col} ramp not monotone: {lumas:?}",
                );
            }
        }
    }
}
