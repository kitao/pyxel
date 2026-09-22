// Color math uses the conventional r/g/b and L/a/b component names.
#![allow(clippy::many_single_char_names)]

use crate::cube::vec3::{RcVec3, Vec3};
use crate::image::{rgb24_to_rgb8, Rgb24};

pub const LEVEL_COUNT: usize = 8;
// The source color is an explicit anchor, not a requirement for an odd width.
pub const BASE_LEVEL: usize = (2 * (LEVEL_COUNT - 1) + 1) / 3;
type Entry = (i32, i32);
const CHROMA_WEIGHT: f32 = 2.0;

// Each cell is a flat color or a screen-space 50:50 checker pair.
pub struct Shading {
    data: Vec<[Entry; LEVEL_COUNT]>,
    pub direction: RcVec3,
}

define_rc_type!(RcShading, Shading);

impl Shading {
    pub fn new(palette: &[Rgb24]) -> RcShading {
        new_rc_type!(Shading {
            data: Self::compute(palette),
            direction: Vec3::new(0.5, -1.0, -0.8),
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
        let colors: Vec<_> = palette.iter().map(|&rgb| PaletteColor::new(rgb)).collect();
        // RGB ordering makes ties independent of palette index order.
        let mut unique: Vec<_> = (0..palette.len()).collect();
        unique.sort_by_key(|&i| palette[i]);
        unique.dedup_by_key(|i| palette[*i]);
        let candidates = make_candidates(&colors, &unique);
        let choices: Vec<_> = unique
            .iter()
            .map(|&source| select_candidates(source, &colors, &candidates))
            .collect();
        let mut ramps: Vec<_> = unique
            .iter()
            .zip(&choices)
            .map(|(&source, selected)| plan_ramp(source, &colors, &candidates, selected, &[]))
            .collect();
        // Resolve texture-color collisions against the other ramps, not just
        // against each color's target in isolation. RGB order keeps this
        // deterministic when the palette is reordered.
        for _ in 0..2 {
            for (position, &source) in unique.iter().enumerate() {
                let mut neighbors: Vec<_> = unique
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != position)
                    .map(|(i, &other)| (other, &ramps[i]))
                    .collect();
                neighbors.sort_by(|&(a, _), &(b, _)| {
                    color_error(colors[source].lab, colors[a].lab)
                        .total_cmp(&color_error(colors[source].lab, colors[b].lab))
                });
                neighbors.truncate(6);
                let row = plan_ramp(source, &colors, &candidates, &choices[position], &neighbors);
                ramps[position] = row;
            }
        }
        let mut data = vec![[(0, 0); LEVEL_COUNT]; palette.len()];
        for (&source, row) in unique.iter().zip(ramps) {
            data[source] = row.map(|i| candidates[i].entry);
        }
        for source in 0..palette.len() {
            let canonical = unique
                .binary_search_by_key(&palette[source], |&i| palette[i])
                .map(|i| unique[i])
                .unwrap();
            let row = data[canonical];
            data[source] = row.map(|(p, q)| {
                let keep_source = |i: i32| {
                    if palette[i as usize] == palette[source] {
                        source as i32
                    } else {
                        i
                    }
                };
                (keep_source(p), keep_source(q))
            });
        }
        data
    }
}

struct PaletteColor {
    linear: [f32; 3],
    lab: [f32; 3],
    luma: f32,
}

impl PaletteColor {
    fn new(rgb: Rgb24) -> Self {
        let (r, g, b) = rgb24_to_rgb8(rgb);
        let linear = [r, g, b].map(|c| srgb_to_linear(c as f32 / 255.0));
        Self {
            linear,
            lab: linear_to_oklab(linear),
            luma: luminance(linear),
        }
    }
}

struct Candidate {
    entry: Entry,
    lab: [f32; 3],
    chroma: f32,
    low: f32,
    high: f32,
    grain: f32,
}

impl Candidate {
    fn preserves_hue(&self, source: &PaletteColor, colors: &[PaletteColor]) -> bool {
        let chroma = source.lab[1].hypot(source.lab[2]);
        // Keep chromatic surfaces recognizable instead of replacing them with
        // gray, while allowing muted colors elsewhere in the same hue family.
        if chroma > 0.025 && self.chroma < chroma * 0.35 {
            return false;
        }
        [self.entry.0, self.entry.1].into_iter().all(|index| {
            let color = &colors[index as usize];
            let other = color.lab[1].hypot(color.lab[2]);
            // Check the visible endpoints, not just a pair's average. Neutral
            // pixels can shade any hue; a neutral source must stay neutral.
            // Allow palette-style hue shifts up to 80 degrees, but exclude
            // opposing hues such as yellow and blue, even if their mean fits.
            other <= 0.025
                || (chroma > 0.025
                    && source.lab[1] * color.lab[1] + source.lab[2] * color.lab[2]
                        >= 0.173_648_18 * chroma * other)
        })
    }

    fn eligible(&self, source: usize, base_luma: f32, level: usize) -> bool {
        match level.cmp(&BASE_LEVEL) {
            std::cmp::Ordering::Less => self.high <= base_luma,
            std::cmp::Ordering::Equal => self.entry == (source as i32, source as i32),
            std::cmp::Ordering::Greater => self.low >= base_luma,
        }
    }
}

fn make_candidates(colors: &[PaletteColor], unique: &[usize]) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    for (position, &p) in unique.iter().enumerate() {
        let a = &colors[p];
        candidates.push(Candidate {
            entry: (p as i32, p as i32),
            lab: a.lab,
            chroma: a.lab[1].hypot(a.lab[2]),
            low: a.luma,
            high: a.luma,
            grain: 0.0,
        });
        for &q in &unique[position + 1..] {
            let b = &colors[q];
            let dl = a.lab[0] - b.lab[0];
            let da = a.lab[1] - b.lab[1];
            let db = a.lab[2] - b.lab[2];
            let chroma_a = a.lab[1].hypot(a.lab[2]);
            let chroma_b = b.lab[1].hypot(b.lab[2]);
            let hue_dot = a.lab[1] * b.lab[1] + a.lab[2] * b.lab[2];
            let chroma_limit = if chroma_a.min(chroma_b) <= 0.04 {
                0.26
            } else {
                0.24
            };
            // Visible dots cannot be judged solely by their optical average.
            // Neutrals have no reliable hue, but the same lightness-contrast
            // limit applies: bright yellow/black is still a conspicuous pattern.
            if dl.abs() > 0.55
                || da * da + db * db > chroma_limit * chroma_limit
                || (chroma_a > 0.04
                    && chroma_b > 0.04
                    && hue_dot < 0.258_819_04 * chroma_a * chroma_b)
            {
                continue;
            }
            let average = std::array::from_fn(|i| (a.linear[i] + b.linear[i]) * 0.5);
            let lab = linear_to_oklab(average);
            let entry = if a.luma <= b.luma {
                (p as i32, q as i32)
            } else {
                (q as i32, p as i32)
            };
            let grain = 0.0001
                + 0.04 * (dl.abs() - 0.08).max(0.0).powi(2)
                + 0.35 * (da.hypot(db) - 0.04).max(0.0).powi(2);
            candidates.push(Candidate {
                entry,
                lab,
                chroma: lab[1].hypot(lab[2]),
                low: a.luma.min(b.luma),
                high: a.luma.max(b.luma),
                // Score neutral and colored pairs alike; black must not get
                // an artificial advantage over a nearby colored shadow.
                grain,
            });
        }
    }
    candidates
}

fn target_color(base: [f32; 3], level: usize) -> [f32; 3] {
    // Keep the hue while lowering exposure. Highlights add lightness and
    // reduce chroma; their lift is bounded to keep dark colors recognizable.
    if level <= BASE_LEVEL {
        let amount = level as f32 / BASE_LEVEL as f32;
        // Half the linear light is the darkest target. Oklab scales with its
        // cube root. Palette matching may retain a lighter, less grainy shadow.
        let scale = (0.5 + 0.5 * amount).cbrt();
        [base[0] * scale, base[1] * scale, base[2] * scale]
    } else {
        let amount = (level - BASE_LEVEL) as f32 / (LEVEL_COUNT - 1 - BASE_LEVEL) as f32;
        let lift = 0.18_f32.min(base[0] * 0.3).min((1.0 - base[0]) * 0.8);
        let chroma = 1.0 - 0.3 * amount;
        [base[0] + lift * amount, base[1] * chroma, base[2] * chroma]
    }
}

fn color_error(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dl = a[0] - b[0];
    let da = a[1] - b[1];
    let db = a[2] - b[2];
    dl * dl + CHROMA_WEIGHT * (da * da + db * db)
}

fn appearance_error(candidate: &Candidate, target: [f32; 3], base_chroma: f32) -> f32 {
    // A neutral surface should not acquire saturated shadows merely because
    // the palette lacks a nearby gray.
    color_error(candidate.lab, target)
        + candidate.grain
        + 8.0 * (candidate.chroma - base_chroma).max(0.0).powi(2)
}

fn select_candidates(
    source: usize,
    colors: &[PaletteColor],
    candidates: &[Candidate],
) -> Vec<usize> {
    let base = &colors[source];
    let flat = (source as i32, source as i32);
    let base_index = candidates.iter().position(|c| c.entry == flat).unwrap();
    let targets: [_; LEVEL_COUNT] = std::array::from_fn(|level| target_color(base.lab, level));
    let base_chroma = base.lab[1].hypot(base.lab[2]);
    let score = |c: &Candidate, level: usize| appearance_error(c, targets[level], base_chroma);
    let compatible: Vec<_> = candidates
        .iter()
        .enumerate()
        .filter(|(_, candidate)| candidate.preserves_hue(base, colors))
        .collect();

    // Bound the path search even for 256 colors. The source is always available,
    // so sparse palettes require neither new colors nor unsuitable mixtures.
    let mut selected = vec![base_index];
    for level in 0..LEVEL_COUNT {
        let mut best = [(f32::INFINITY, base_index); 4];
        for &(index, candidate) in &compatible {
            if !candidate.eligible(source, base.luma, level) {
                continue;
            }
            let cost = score(candidate, level);
            if let Some(slot) = best.iter().position(|&(previous, _)| cost < previous) {
                best[slot..].rotate_right(1);
                best[slot] = (cost, index);
            }
        }
        selected.extend(best.map(|(_, index)| index));
    }
    selected.sort_unstable();
    selected.dedup();
    selected
}

fn plan_ramp(
    source: usize,
    colors: &[PaletteColor],
    candidates: &[Candidate],
    selected: &[usize],
    neighbors: &[(usize, &[usize; LEVEL_COUNT])],
) -> [usize; LEVEL_COUNT] {
    let base = &colors[source];
    let targets: [_; LEVEL_COUNT] = std::array::from_fn(|level| target_color(base.lab, level));
    let base_chroma = base.lab[1].hypot(base.lab[2]);
    let score = |c: &Candidate, level: usize| {
        let mut cost = appearance_error(c, targets[level], base_chroma);
        for &(other, row) in neighbors {
            let target = target_color(colors[other].lab, level);
            let desired = color_error(targets[level], target).sqrt();
            // Only protect separations that the limited palette can reasonably
            // retain. Large color differences need not retain their full size.
            let minimum = 0.5 * desired.min(0.2);
            let other_lab = candidates[row[level]].lab;
            // Project onto the original color difference. Merely maximizing
            // distance would reward unrelated hues or reversed contrast.
            let actual = ((c.lab[0] - other_lab[0]) * (targets[level][0] - target[0])
                + CHROMA_WEIGHT
                    * ((c.lab[1] - other_lab[1]) * (targets[level][1] - target[1])
                        + (c.lab[2] - other_lab[2]) * (targets[level][2] - target[2])))
                / desired.max(1.0e-6);
            cost += 0.8 * (minimum - actual).max(0.0).powi(2);
        }
        cost
    };
    let count = selected.len();
    let mut costs = vec![f32::INFINITY; count];
    let mut previous = vec![vec![0; count]; LEVEL_COUNT];
    for (level, parents) in previous.iter_mut().enumerate() {
        let mut next = vec![f32::INFINITY; count];
        for (to, &index) in selected.iter().enumerate() {
            let candidate = &candidates[index];
            if !candidate.eligible(source, base.luma, level) {
                continue;
            }
            let cost = score(candidate, level);
            if level == 0 {
                next[to] = cost;
                continue;
            }
            for (from, &old_index) in selected.iter().enumerate() {
                let old = &candidates[old_index];
                // Both checker phases brighten together; no phase swaps or
                // alternating pixels getting darker as the light gets brighter.
                if old.low > candidate.low || old.high > candidate.high {
                    continue;
                }
                let transition = if old.entry == candidate.entry {
                    0.0
                } else {
                    (0.0003 + 0.025 * color_error(old.lab, candidate.lab))
                        * (LEVEL_COUNT - 1) as f32
                        / 15.0
                };
                let total = costs[from] + transition + cost;
                if total < next[to] {
                    next[to] = total;
                    parents[to] = from;
                }
            }
        }
        costs = next;
    }
    let mut current = (0..count)
        .min_by(|&a, &b| costs[a].total_cmp(&costs[b]))
        .unwrap();
    let mut row = [0; LEVEL_COUNT];
    for level in (0..LEVEL_COUNT).rev() {
        row[level] = selected[current];
        current = previous[level][current];
    }
    row
}

fn luminance([r, g, b]: [f32; 3]) -> f32 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

// Oklab matrices: https://bottosson.github.io/posts/oklab/
fn linear_to_oklab([r, g, b]: [f32; 3]) -> [f32; 3] {
    let l = (0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b).cbrt();
    let m = (0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b).cbrt();
    let s = (0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b).cbrt();
    [
        0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
        1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
        0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::DEFAULT_COLORS;

    #[test]
    fn test_empty_and_single_color_palettes() {
        assert_eq!(Shading::compute(&[]), Vec::<[Entry; LEVEL_COUNT]>::new());
        assert_eq!(Shading::compute(&[0x879abc]), vec![[(0, 0); LEVEL_COUNT]]);
    }

    #[test]
    fn test_build_replaces_manual_edits() {
        let shading = Shading::new(&DEFAULT_COLORS);
        let mut shading = rc_mut!(&shading);
        shading.set(0, BASE_LEVEL, (99, 99));
        shading.build(&[0x123456]);
        assert_eq!(shading.palette_size(), 1);
        assert_eq!(shading.get(0, BASE_LEVEL), (0, 0));
    }

    #[test]
    fn test_both_checker_phases_are_monotone() {
        let colors: Vec<_> = DEFAULT_COLORS
            .iter()
            .map(|&c| PaletteColor::new(c))
            .collect();
        for (source, row) in Shading::compute(&DEFAULT_COLORS).iter().enumerate() {
            assert_eq!(row[BASE_LEVEL], (source as i32, source as i32));
            for pair in row.windows(2) {
                assert!(colors[pair[0].0 as usize].luma <= colors[pair[1].0 as usize].luma);
                assert!(colors[pair[0].1 as usize].luma <= colors[pair[1].1 as usize].luma);
            }
        }
    }

    #[test]
    fn test_palette_reordering_preserves_rgb_results() {
        let original = Shading::compute(&DEFAULT_COLORS);
        let mut palette = DEFAULT_COLORS;
        palette.reverse();
        let reversed = Shading::compute(&palette);
        for source in 0..palette.len() {
            for level in 0..LEVEL_COUNT {
                let (p, q) = original[source][level];
                let (rp, rq) = reversed[palette.len() - 1 - source][level];
                assert_eq!(DEFAULT_COLORS[p as usize], palette[rp as usize]);
                assert_eq!(DEFAULT_COLORS[q as usize], palette[rq as usize]);
            }
        }
    }

    #[test]
    fn test_yellow_avoids_both_unrelated_blue_and_high_contrast_black_dither() {
        let rows = Shading::compute(&[0xe9c35b, 0x2b335f, 0x000000]);
        assert!(rows[0].iter().all(|&entry| entry == (0, 0)));
        // Black can still shade a dark color when its visible contrast fits.
        assert!(rows[1][..BASE_LEVEL].contains(&(2, 1)));
    }

    #[test]
    fn test_cream_keeps_its_color_when_only_neutral_shades_are_available() {
        let row = Shading::compute(&[0xedc7b0, 0x000000, 0xa3a3a3, 0xeeeeee])[0];
        assert!(row.iter().all(|&(p, q)| p == 0 || q == 0));
        assert!(row.contains(&(2, 0)));
        assert!(!row.contains(&(1, 0)));
    }

    #[test]
    fn test_duplicate_colors_keep_their_own_base_index() {
        let palette = [0xffffff, 0x336699, 0x000000, 0x336699];
        let rows = Shading::compute(&palette);
        assert_eq!(rows[1][BASE_LEVEL], (1, 1));
        assert_eq!(rows[3][BASE_LEVEL], (3, 3));
        for (&a, &b) in rows[1].iter().zip(&rows[3]) {
            let rgb = |(p, q): Entry| (palette[p as usize], palette[q as usize]);
            assert_eq!(rgb(a), rgb(b));
        }
    }

    #[test]
    fn test_opposing_hues_do_not_form_pairs() {
        for row in Shading::compute(&[0x000000, 0xff0000, 0x00ffff, 0xffffff]) {
            assert!(row
                .iter()
                .all(|&(p, q)| (p, q) != (1, 2) && (p, q) != (2, 1)));
        }
    }

    #[test]
    fn test_white_stays_neutral_without_suitable_shadow_colors() {
        let palette = [
            0x000000, 0xff0000, 0x00ff00, 0x0000ff, 0xffff00, 0x00ffff, 0xff00ff, 0xffffff,
        ];
        let row = Shading::compute(&palette)[7];
        assert!(row
            .iter()
            .all(|&(p, q)| [0, 7].contains(&p) && [0, 7].contains(&q)));
    }

    #[test]
    fn test_default_grays_do_not_acquire_colored_shadows_or_highlights() {
        let rows = Shading::compute(&DEFAULT_COLORS);
        for source in [0, 7, 13] {
            for &(p, q) in &rows[source] {
                for index in [p, q] {
                    let (r, g, b) = rgb24_to_rgb8(DEFAULT_COLORS[index as usize]);
                    assert_eq!(r, g);
                    assert_eq!(g, b);
                }
            }
        }
    }

    #[test]
    fn test_default_warm_surfaces_allow_related_palette_hue_shifts() {
        let rows = Shading::compute(&DEFAULT_COLORS);
        // Red can shade toward magenta; yellow can shade toward orange.
        assert!(rows[8][..BASE_LEVEL].contains(&(2, 8)));
        assert!(rows[10][..BASE_LEVEL].contains(&(9, 10)));
        let warm_or_neutral = [0, 2, 4, 7, 8, 9, 10, 13, 14, 15];
        for source in [4, 8, 9, 10, 14, 15] {
            for &(p, q) in &rows[source] {
                assert!(
                    warm_or_neutral.contains(&p),
                    "source {source}, endpoint {p}"
                );
                assert!(
                    warm_or_neutral.contains(&q),
                    "source {source}, endpoint {q}"
                );
            }
        }
    }

    #[test]
    fn test_dense_grays_choose_flat_colors() {
        let palette: Vec<_> = (0..256).map(|c| c * 0x010101).collect();
        let rows = Shading::compute(&palette);
        assert!(rows[128].iter().all(|&(p, q)| p == q));
        assert!(rows[128][0].0 < 128);
        assert!(rows[128][LEVEL_COUNT - 1].0 > 128);
    }

    #[test]
    fn test_custom_palettes_preserve_base_and_each_checker_phase() {
        let palettes = [
            (0..256).map(|c| c * 0x010101).collect::<Vec<_>>(),
            (0..64).map(|i| (i * 0x9e3779) & 0xffffff).collect(),
            vec![0x5d2137, 0x9e3650, 0xc16b61, 0xe3b184],
            vec![0x172d40, 0x23493d, 0x3a726b, 0x8fa4aa],
        ];
        for palette in palettes {
            let colors: Vec<_> = palette.iter().map(|&c| PaletteColor::new(c)).collect();
            for (source, row) in Shading::compute(&palette).iter().enumerate() {
                assert_eq!(row[BASE_LEVEL], (source as i32, source as i32));
                for &(p, q) in row {
                    assert!((0..palette.len() as i32).contains(&p));
                    assert!((0..palette.len() as i32).contains(&q));
                }
                for pair in row.windows(2) {
                    assert!(colors[pair[0].0 as usize].luma <= colors[pair[1].0 as usize].luma);
                    assert!(colors[pair[0].1 as usize].luma <= colors[pair[1].1 as usize].luma);
                }
            }
        }
    }
}
