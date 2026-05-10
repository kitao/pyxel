use crate::cube::vec3::{RcVec3, Vec3};
use crate::image::{rgb24_to_rgb8, Rgb24};

// Shading LUT mapping (col, level) to a (primary, secondary) pair plus a
// scene-wide light direction. Each cell is either flat (primary ==
// secondary) or a 50:50 checker between primary and secondary on a 2x2
// pixel pattern. The LUT is built from a palette and an HSV ideal target.

pub const LEVEL_COUNT: usize = 4;

// (primary, secondary) — palette indices. primary == secondary collapses
// to a flat fill; otherwise the rasterizer renders a 2x2 checker between
// the two.

type Entry = (i32, i32);

pub struct Shading {
    data: Vec<Vec<Entry>>,
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

    fn compute(palette: &[Rgb24]) -> Vec<Vec<Entry>> {
        // Per-lv ideal: same hue/saturation as the base, value shifted by
        // STEP. Candidate quality = weighted-Euclidean HSV distance to
        // that ideal. The same distance function gates "is this dither
        // pair too far apart to read as one color" by feeding it the
        // pair's two constituents instead of (entry, ideal).
        //
        // Distance ≥ REJECT_THRESHOLD = "different color". A picker that
        // can't find anything inside the threshold falls back to base
        // flat (= the palette has no usable shade for this hue).
        // STEP keeps the ramp's per-step lightness change at a flavor
        // level — large enough to read as a visible step in the 4-lv
        // ramp, small enough that adjacent face brightness levels in 3D
        // don't read as "neon-sign" abrupt color changes.
        const STEP: f32 = 0.01;
        const V_DARK_TERMINAL: f32 = 0.05;
        // distance() axis weights. Saturation and value are neutral so
        // their full-range deltas (= 1.0) saturate REJECT_THRESHOLD on
        // their own — pure white vs pure black scores 1.0 in V, gray
        // vs full-saturation hue scores 1.0 in S. Hue is the sensitive
        // axis: WH=8 means a hue gap of 1/8 of the color wheel
        // (= 45°) at full saturation already scores 1.0, i.e. is
        // treated as a different color. This matches the toon-shading
        // requirement that the ramp must stay inside the source's
        // hue band.
        const WH: f32 = 8.0;
        const WS: f32 = 1.0;
        const WV: f32 = 1.0;
        // Achromatic ↔ chromatic crossing penalty, asymmetric.
        //   chromatic source → achromatic candidate (CROSSING_C2A): the
        //     source's hue is preserved by the ideal, so a gray candidate
        //     reads as "the ramp dropped its hue" — strong penalty.
        //   achromatic source → chromatic candidate (CROSSING_A2C): a
        //     gray/black/white source has no hue to preserve, so a
        //     chromatic highlight is a deliberate accent (e.g. "black
        //     lv 3 = navy") — light penalty.
        // The asymmetry is a direct consequence of HSV's structure: H
        // is undefined for S=0, so "preserve hue" is one-sided.
        // distance() expects a = source/ideal side, b = candidate side.
        const ACHROMATIC_THRESHOLD: f32 = 0.05;
        const CROSSING_C2A: f32 = 0.80;
        const CROSSING_A2C: f32 = 0.10;
        const REJECT_THRESHOLD: f32 = 1.0;
        // Pattern C lv 1 quality gate: lv 1 in Pattern C is a flat X
        // shown directly to the eye, so X must be at least as good as
        // an independently-picked lv 1 (= Pattern A's lv 1 solo best).
        // The 1.1 multiplier allows a small slack so a near-best X with
        // a meaningfully better lv 0 dither can still win — Pattern C
        // exists precisely to trade off lv 0 quality, but never at the
        // cost of lv 1 quality.
        const PATTERN_C_LV1_SLACK: f32 = 1.1;

        let n = palette.len();
        if n == 0 {
            return vec![];
        }
        // Per-entry sRGB-linear (for physically-correct dither blends)
        // and HSV (for the distance score).
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
        // sRGB relative luminance (Y, BT.709) per palette entry, used
        // exclusively to enforce ramp monotonicity. HSV's V channel is
        // the per-pixel max — it makes pure blue and pure white tie at
        // V=1, even though their perceived brightness differs by a
        // factor of nine. Luma is the perceptually-correct ordering.
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
        // Weighted-Euclidean HSV distance from a source-side color a to a
        // candidate-side color b. dH is the circular minimum of the hue
        // gap, scaled by the lower of the two saturations so a hue gap
        // collapses to zero whenever one side has no hue (S=0). The
        // crossing penalty is asymmetric: only the source side decides
        // whether we're "leaving a hue behind" (= big penalty) or
        // "adding a hue to a gray" (= small penalty).
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
        // HSV of an entry's visual color. A flat returns the palette
        // entry's HSV directly. A dither blends the two constituents in
        // sRGB-linear space (= physically-correct optical mixing) and
        // re-encodes through the sRGB transfer curve before converting
        // to HSV. This matches what the eye sees at 2x2-pixel scale: a
        // single mid-color, not two flickering colors.
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
        // Pick two entries (lv 1, lv 0) by enumerating three connectivity
        // patterns and keeping the lowest-total-distance pair. Returns
        // None when the cheapest pattern still exceeds 2 × REJECT.
        //
        //   Pattern A: lv 0 flat, lv 1 flat (= no dither at all)
        //   Pattern B: lv 0 flat X, lv 1 dither (source, X)
        //   Pattern C: lv 0 dither (X, Y), lv 1 flat X
        //
        // All three keep continuity: B/C share X across the two levels;
        // A's two flats need no shared anchor.
        let pick_two_pattern = |source: usize,
                                ideal_1: (f32, f32, f32),
                                ideal_0: (f32, f32, f32)|
         -> Option<(Entry, Entry)> {
            let base_entry: Entry = (source as i32, source as i32);
            let source_luma = luma[source];
            // Ramp monotonicity: shade-side candidates must sit
            // strictly below the source's perceived brightness — equal
            // luma reads as "no shade step" and would let a same-color
            // duplicate (different palette index, same RGB) collapse
            // the ramp into a flat repeat.
            let darker_than_source = |entry: Entry| -> bool { entry_luma(entry) < source_luma };
            // Solo best lv 1 flat score across all candidates. Used
            // both as Pattern A's inner-loop seed and as Pattern C's
            // lv 1 quality gate (= the X chosen for Pattern C must be
            // at least as good a lv 1 as Pattern A would have picked
            // independently, modulo a small slack).
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
            // Pattern A: independent flat per lv (= no dither). Searched
            // as a (lv 1, lv 0) pair so the ramp's full monotonicity
            // (lv 0 ≤ lv 1 ≤ lv 2) is enforced.
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
            // Pattern B: lv 1 = (source, X) dither, lv 0 = X flat.
            // Drop X when its distance from source already reads as a
            // different color (= dither would flicker, not blend).
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
            // Pattern C: lv 1 = X flat, lv 0 = (X, Y) dither. Y's value
            // must not exceed X's (keeps the ramp monotone) and (X, Y)
            // must read as one color.
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
        // lv 3: pick from {flat(X), dither(source, X)} that minimizes
        // distance to the ideal. Single-lv version of the above.
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
                let mut row: Vec<Entry> = vec![flat(c); LEVEL_COUNT];

                // lv 3 (= highlight): ideal one STEP brighter. When V is
                // already saturated, the highlight target moves along the
                // saturation axis instead — desaturating by STEP is the
                // pixel-art convention for "highlight on a fully bright
                // hue" (= bleached toward white). When both V and S are
                // saturated (= no room to move), the lv stays at base
                // flat.
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

                // lv 0/1 (= shade): ideal_1 one STEP darker, ideal_0 two
                // STEPs darker.
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
        let r_mut = rc_mut!(&r);
        r_mut.set(0, 0, (5, 7));
        assert_eq!(r_mut.get(0, 0), (5, 7));
    }

    #[test]
    fn test_build_resets_table() {
        let r = Shading::new(&pyxel_default());
        let r_mut = rc_mut!(&r);
        r_mut.set(0, 0, (99, 99));
        r_mut.build(&pyxel_default());
        let (p, s) = r_mut.get(0, 0);
        assert!(p != 99 || s != 99);
    }
}
