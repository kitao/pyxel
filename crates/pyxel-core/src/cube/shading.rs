// Color math uses the conventional RGB and Oklab component names.
#![allow(clippy::many_single_char_names)]

use crate::cube::vec3::{RcVec3, Vec3};
use crate::image::{rgb24_to_rgb8, Rgb24};

pub const LEVEL_COUNT: usize = 4;

type Entry = (i32, i32);

// Each palette-derived table cell is a flat color or a 50:50 dither pair.
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
        let colors: Vec<PaletteColor> = palette.iter().map(|&rgb| PaletteColor::new(rgb)).collect();

        colors
            .iter()
            .enumerate()
            .map(|(source, color)| {
                // The source competes with other colors: sparse palettes may repeat
                // a shade rather than invent an unrelated color or a harsh checker.
                let shadow = pick_neighbor(&colors, source, -0.20, 0.32);
                let highlight = pick_neighbor(&colors, source, 0.15, 0.23);
                let base = (source as i32, source as i32);
                let dark = (shadow as i32, shadow as i32);
                // A rejected midpoint must not become the full shadow: that
                // merges painted details before the face reaches deep shadow.
                let middle = if color.can_dither(&colors[shadow]) {
                    (source as i32, shadow as i32)
                } else {
                    base
                };
                let bright = if color.can_dither(&colors[highlight]) {
                    (source as i32, highlight as i32)
                } else {
                    base
                };
                [dark, middle, base, bright]
            })
            .collect()
    }
}

struct PaletteColor {
    lab: [f32; 3],
    luma: f32,
}

impl PaletteColor {
    fn new(rgb: Rgb24) -> Self {
        let (r, g, b) = rgb24_to_rgb8(rgb);
        let r = srgb_to_linear(r as f32 / 255.0);
        let g = srgb_to_linear(g as f32 / 255.0);
        let b = srgb_to_linear(b as f32 / 255.0);
        Self {
            lab: linear_rgb_to_oklab(r, g, b),
            luma: 0.2126 * r + 0.7152 * g + 0.0722 * b,
        }
    }

    fn compatible_with(&self, other: &Self) -> bool {
        let chroma = self.lab[1].hypot(self.lab[2]);
        let other_chroma = other.lab[1].hypot(other.lab[2]);
        // Shading must not turn neutral paint into a saturated color, or add
        // saturation to shadows. Small differences accommodate palette spacing.
        if other_chroma > chroma + 0.015 {
            return false;
        }
        if chroma < 0.03 || other_chroma < 0.03 {
            return true;
        }

        // Retain the hue family; a neutral shade has no hue to compare.
        let hue_dot = self.lab[1] * other.lab[1] + self.lab[2] * other.lab[2];
        hue_dot >= 0.5 * chroma * other_chroma
    }

    fn can_dither(&self, other: &Self) -> bool {
        // Judge the two visible pixels, not only their averaged color.
        let lightness_gap = (self.lab[0] - other.lab[0]).abs();
        let color_gap = (self.lab[1] - other.lab[1]).hypot(self.lab[2] - other.lab[2]);
        lightness_gap <= 0.15 && color_gap <= 0.07
    }
}

fn pick_neighbor(colors: &[PaletteColor], source: usize, shift: f32, max_gap: f32) -> usize {
    let color = &colors[source];
    let target = [
        (color.lab[0] + shift).clamp(0.0, 1.0),
        color.lab[1] * 0.95,
        color.lab[2] * 0.95,
    ];
    let distance = |lab: &[f32; 3]| {
        (lab[0] - target[0]).powi(2) + (lab[1] - target[1]).powi(2) + (lab[2] - target[2]).powi(2)
    };
    let mut best = source;
    let mut best_score = distance(&color.lab);

    for (index, candidate) in colors.iter().enumerate() {
        let gap = candidate.lab[0] - color.lab[0];
        if gap * shift <= 0.0
            || gap.abs() > max_gap
            || (candidate.luma - color.luma) * shift <= 0.0
            || !color.compatible_with(candidate)
        {
            continue;
        }

        let score = distance(&candidate.lab);
        if score < best_score {
            best = index;
            best_score = score;
        }
    }
    best
}

// Color-space conversion

#[inline]
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

// Oklab separates perceived lightness from color, so brightness alone cannot
// hide an incompatible hue pair. Bjorn Ottosson's published conversion:
// https://bottosson.github.io/posts/oklab/
fn linear_rgb_to_oklab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let l = (0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b).cbrt();
    let m = (0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b).cbrt();
    let s = (0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b).cbrt();
    [
        0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
        1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
        0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_8 * s,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::DEFAULT_COLORS;

    #[test]
    fn test_default_palette_size() {
        let r = Shading::new(&DEFAULT_COLORS);
        let r = rc_ref!(&r);
        assert_eq!(r.palette_size(), 16);
    }

    #[test]
    fn test_build_resets_table() {
        let r = Shading::new(&DEFAULT_COLORS);
        let mut r_mut = rc_mut!(&r);
        r_mut.set(0, 0, (99, 99));
        r_mut.build(&DEFAULT_COLORS);
        // Black has no darker shade, so level 0 stays flat black.
        assert_eq!(r_mut.get(0, 0), (0, 0));
    }

    #[test]
    fn test_empty_palette_returns_empty_data() {
        let r = Shading::new(&[]);
        let r = rc_ref!(&r);
        assert_eq!(r.palette_size(), 0);
    }

    #[test]
    fn test_sparse_palettes_preserve_source_instead_of_forcing_contrast() {
        for palette in [&[0x000000, 0xffffff][..], &[0x808080, 0x808080][..]] {
            let shading = Shading::compute(palette);
            for (col, row) in shading.iter().enumerate() {
                assert_eq!(*row, [(col as i32, col as i32); LEVEL_COUNT]);
            }
        }
    }

    #[test]
    fn test_default_paint_does_not_gain_unrelated_colors() {
        let shading = Shading::compute(&DEFAULT_COLORS);
        // Peach must not become pink, brown must not become purple, and gray
        // must not acquire a black checker. These changes obscure textures.
        for (source, excluded) in [(15, 14), (4, 2), (13, 0)] {
            for &(primary, secondary) in &shading[source] {
                assert_ne!(primary, excluded);
                assert_ne!(secondary, excluded);
            }
        }
    }

    #[test]
    fn test_midtones_preserve_painted_details_when_the_shadow_pair_is_unsuitable() {
        let shading = Shading::compute(&DEFAULT_COLORS);
        // Orange and brown occur next to each other in painted textures. They
        // may merge in deep shadow, but must remain distinct in the midtones.
        assert_eq!(shading[9][0], shading[4][0]);
        assert_eq!(shading[9][1], (9, 9));
        assert_eq!(shading[4][1], (4, 4));

        // Compatible pairs still provide an intermediate shade.
        assert_eq!(shading[10][1], (10, 9));
    }

    #[test]
    fn test_ramps_follow_palette_colors_after_reordering() {
        let mut palette = DEFAULT_COLORS;
        palette.reverse();
        let original = Shading::compute(&DEFAULT_COLORS);
        let reversed = Shading::compute(&palette);

        for col in 0..palette.len() {
            assert_eq!(original[col][2], (col as i32, col as i32));
            for level in 0..LEVEL_COUNT {
                let (p, s) = original[col][level];
                let (rp, rs) = reversed[palette.len() - 1 - col][level];
                assert_eq!(DEFAULT_COLORS[p as usize], palette[rp as usize]);
                assert_eq!(DEFAULT_COLORS[s as usize], palette[rs as usize]);
            }
        }
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
        let pal = DEFAULT_COLORS;
        let r = Shading::new(&pal);
        let r = rc_ref!(&r);

        for col in 0..pal.len() {
            let lumas: [f32; 4] = std::array::from_fn(|lv| {
                let (p, s) = r.get(col, lv);
                entry_luma(&pal, p, s)
            });

            for lv in 1..4 {
                assert!(
                    lumas[lv] >= lumas[lv - 1],
                    "col {col} ramp not monotone: {lumas:?}",
                );
            }
        }
    }
}
