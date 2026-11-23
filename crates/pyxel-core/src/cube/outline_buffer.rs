// Screen-space outlines share the rasterized coverage of primitives and models.

use std::cmp::Ordering;

use super::raster::{depth_in_clip_range, dither_allows, ClipRect};
use crate::image::Image;

#[derive(Clone, Copy, Default)]
struct Surface {
    group: u32,
    dx: f32,
    dy: f32,
    phases: u16,
    depth_phases: u16,
    ink: u8,
    depth_test: bool,
    depth_write: bool,
}

#[derive(Clone, Copy)]
struct Fragment {
    key: u32,
    z: f32,
}

const EMPTY: Fragment = Fragment {
    key: 0,
    z: f32::INFINITY,
};

impl Fragment {
    // Keys follow draw order. A later draw can replace an earlier color when
    // either depth testing or the earlier draw's depth write was disabled.
    #[inline]
    fn wins(self, other: Self, surfaces: &[Surface]) -> bool {
        match self.key.cmp(&other.key) {
            Ordering::Greater => {
                !surfaces[self.key as usize].depth_test
                    || !surfaces[other.key as usize].depth_write
                    || self.z < other.z
            }
            Ordering::Less => {
                surfaces[other.key as usize].depth_test
                    && surfaces[self.key as usize].depth_write
                    && self.z <= other.z
            }
            Ordering::Equal => self.z < other.z,
        }
    }
}

#[derive(Clone, Copy)]
struct Visibility {
    color: Fragment,
    depth: f32,
}

// Geometry groups suppress interior edges. Only pixels touched by dithering
// or nonstandard depth state need separate visibility at the five destinations.
#[derive(Clone, Copy)]
struct Sample {
    front: Fragment,
    other_group: u32,
    variants: u32,
}

impl Sample {
    #[inline]
    fn visible(self, destination: usize, variants: &[[Visibility; 5]]) -> Fragment {
        if self.variants == 0 {
            self.front
        } else {
            variants[self.variants as usize - 1][destination].color
        }
    }
}

const EMPTY_SAMPLE: Sample = Sample {
    front: EMPTY,
    other_group: 0,
    variants: 0,
};

const SKIP: u32 = u32::MAX;

#[derive(Default)]
pub struct OutlineBuffer {
    samples: Vec<Sample>,
    variants: Vec<[Visibility; 5]>,
    surfaces: Vec<Surface>,
    groups: std::collections::HashMap<usize, u32>,
    pending: bool,
    halo: std::collections::HashMap<usize, f32>,
    stride: usize,
    origin_x: i32,
    origin_y: i32,
    bounds: Option<[i32; 4]>,
    current: u32,
    flat_key: Option<u32>,
    pub clip: Option<ClipRect>,
    pub active: bool,
    pub group: Option<(usize, u8)>,
    pub slope: [f32; 2],
    pub decal: bool,
}

// Texture snapshots copy image pixels and palette, never in-progress draw scratch.
impl Clone for OutlineBuffer {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl Image {
    pub fn begin_outline(&mut self, clip: ClipRect) {
        if clip.right < clip.left || clip.bottom < clip.top {
            return;
        }
        let stride = (clip.right - clip.left) as usize + 3;

        let buffer = &mut self.outline_buffer;
        buffer.stride = stride;
        buffer.origin_x = clip.left - 1;
        buffer.origin_y = clip.top - 1;
        buffer.pending = true;
        buffer.halo.clear();
        buffer.surfaces.clear();
        buffer.variants.clear();
        buffer.surfaces.push(Surface {
            phases: u16::MAX,
            depth_phases: u16::MAX,
            depth_test: true,
            depth_write: true,
            ..Surface::default()
        });
        buffer.groups.clear();
        buffer.bounds = None;
        buffer.active = true;
        buffer.clip = Some(clip);
        buffer.flat_key = None;
    }

    pub fn end_outline(&mut self) {
        self.outline_buffer.active = false;
        self.outline_buffer.clip = None;
        self.outline_buffer.group = None;
    }

    pub fn set_outline_group(&mut self, group: Option<(usize, u8)>, depth: &[f32], unusual: bool) {
        let width = self.width() as usize;
        let buffer = &mut self.outline_buffer;
        let Some(clip) = buffer.clip else {
            return;
        };
        // Ordinary leading draws already recorded depth in the camera. Only the
        // one-pixel margin outside the clipped target needs separate storage.
        if buffer.pending
            && (group.is_some() || unusual || self.canvas.alpha < 1.0 || self.canvas.alpha.is_nan())
        {
            let count = buffer.stride * ((clip.bottom - clip.top) as usize + 3);
            buffer.samples.resize(count, EMPTY_SAMPLE);
            buffer.samples.fill(EMPTY_SAMPLE);
            for y in clip.top..=clip.bottom {
                let index = y as usize * width + clip.left as usize;
                let row_width = (clip.right - clip.left + 1) as usize;
                let scratch_index = (y - buffer.origin_y) as usize * buffer.stride + 1;
                for (sample, &z) in buffer.samples[scratch_index..scratch_index + row_width]
                    .iter_mut()
                    .zip(&depth[index..index + row_width])
                {
                    sample.front.z = z;
                }
            }
            for (scratch_index, z) in buffer.halo.drain() {
                buffer.samples[scratch_index].front.z = z;
            }
            buffer.pending = false;
        }
        buffer.group = group;
        buffer.current = group.map_or(0, |(owner, _)| {
            let id = buffer.groups.len() as u32 + 1;
            *buffer.groups.entry(owner).or_insert(id)
        });
        buffer.flat_key = None;
    }

    pub(crate) fn outline_surface(
        &mut self,
        alpha: f32,
        flat: bool,
        depth_test: bool,
        depth_write: bool,
    ) -> u32 {
        let buffer = &mut self.outline_buffer;
        if buffer.decal {
            return SKIP;
        }
        if flat {
            if let Some(key) = buffer.flat_key {
                return key;
            }
        }
        let target_alpha = self.canvas.alpha;
        let key = if buffer.pending {
            0
        } else {
            let mut phases = 0;
            let mut depth_phases = 0;
            if alpha >= 1.0 && target_alpha >= 1.0 {
                phases = u16::MAX;
                depth_phases = u16::MAX;
            } else {
                for y in 0..4 {
                    for x in 0..4 {
                        if dither_allows(alpha, x, y) {
                            let phase = 1 << (4 * y + x);
                            depth_phases |= phase;
                            if self.canvas.should_write(x, y) {
                                phases |= phase;
                            }
                        }
                    }
                }
            }
            if depth_phases == 0 || phases == 0 && !depth_write {
                SKIP
            } else {
                let key = buffer.surfaces.len() as u32;
                let [dx, dy] = if flat { [0.0, 0.0] } else { buffer.slope };
                buffer.surfaces.push(Surface {
                    group: buffer.current,
                    dx,
                    dy,
                    phases,
                    depth_phases,
                    ink: buffer
                        .group
                        .map_or(0, |(_, color)| self.palette[color as usize]),
                    depth_test,
                    depth_write,
                });
                key
            }
        };
        if flat {
            buffer.flat_key = Some(key);
        }
        key
    }

    #[inline]
    pub(crate) fn include_outline_bounds(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        if self.outline_buffer.current == 0 || self.outline_buffer.decal {
            return;
        }
        match &mut self.outline_buffer.bounds {
            Some(bounds) => {
                bounds[0] = bounds[0].min(x0);
                bounds[1] = bounds[1].min(y0);
                bounds[2] = bounds[2].max(x1);
                bounds[3] = bounds[3].max(y1);
            }
            None => self.outline_buffer.bounds = Some([x0, y0, x1, y1]),
        }
    }

    #[inline]
    pub(crate) fn record_outline_sample(&mut self, x: i32, y: i32, z: f32, key: u32) {
        if key == SKIP || !depth_in_clip_range(z) {
            return;
        }
        let index = (y - self.outline_buffer.origin_y) as usize * self.outline_buffer.stride
            + (x - self.outline_buffer.origin_x) as usize;
        if self.outline_buffer.pending {
            if !self.outline_buffer.clip.unwrap().contains(x, y) {
                let value = self
                    .outline_buffer
                    .halo
                    .entry(index)
                    .or_insert(f32::INFINITY);
                *value = value.min(z);
            }
            return;
        }
        let sample = &mut self.outline_buffer.samples[index];
        let surfaces = &self.outline_buffer.surfaces;
        let incoming = Fragment { key, z };
        let next = surfaces[key as usize];
        let front = surfaces[sample.front.key as usize];
        if sample.variants != 0 || next.phases != u16::MAX || !next.depth_test || !next.depth_write
        {
            let variants = &mut self.outline_buffer.variants;
            if sample.variants == 0 {
                variants.push(
                    [Visibility {
                        color: sample.front,
                        depth: sample.front.z,
                    }; 5],
                );
                sample.variants = variants.len() as u32;
            }
            let lanes = &mut variants[sample.variants as usize - 1];
            // Lane zero is this pixel. A neighbor uses the lane pointing back
            // at the destination so holes never become geometric boundaries.
            for (lane, (dx, dy)) in lanes
                .iter_mut()
                .zip([(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)])
            {
                let phase = 1 << (((y + dy) & 3) * 4 + ((x + dx) & 3));
                if next.depth_phases & phase != 0 && (!next.depth_test || z < lane.depth) {
                    if next.depth_write {
                        lane.depth = z;
                    }
                    if next.phases & phase != 0 {
                        lane.color = incoming;
                    }
                }
            }
        }
        if next.phases != 0 {
            if z < sample.front.z {
                if front.group != 0 && front.group != next.group {
                    sample.other_group = front.group;
                }
                sample.front = incoming;
            } else if next.group != 0 && next.group != front.group {
                sample.other_group = next.group;
            }
        }
    }

    #[inline]
    pub(crate) fn tracks_triangle(&self, pass: ClipRect) -> bool {
        let buffer = &self.outline_buffer;
        if !buffer.active || !buffer.pending {
            return buffer.active;
        }
        let clip = buffer.clip.unwrap();
        !clip.contains(pass.left, pass.top) || !clip.contains(pass.right, pass.bottom)
    }

    pub fn composite_outline(&mut self, depth: &[f32]) {
        let Some([x0, y0, x1, y1]) = self.outline_buffer.bounds.take() else {
            return;
        };
        let width = self.width() as usize;

        let buffer = &mut self.outline_buffer;
        let clip = buffer.clip.unwrap();
        for y in (y0 - 1).max(clip.top)..=(y1 + 1).min(clip.bottom) {
            for x in (x0 - 1).max(clip.left)..=(x1 + 1).min(clip.right) {
                let (x, y) = (x as usize, y as usize);
                let index = y * width + x;
                let scratch_index = (y as i32 - buffer.origin_y) as usize * buffer.stride
                    + (x as i32 - buffer.origin_x) as usize;
                let center = buffer.samples[scratch_index];
                let neighbors = [
                    buffer.samples[scratch_index - 1],
                    buffer.samples[scratch_index + 1],
                    buffer.samples[scratch_index - buffer.stride],
                    buffer.samples[scratch_index + buffer.stride],
                ];
                let surfaces = &buffer.surfaces;
                let body = center.visible(0, &buffer.variants);
                let mut best = body;
                let mut ink = None;
                for (direction, neighbor) in neighbors.into_iter().enumerate() {
                    let neighbor = neighbor.visible(direction + 1, &buffer.variants);
                    let surface = surfaces[neighbor.key as usize];
                    if surface.group == 0
                        || surface.group == surfaces[center.front.key as usize].group
                        || surface.group == center.other_group
                        || surface.group == surfaces[body.key as usize].group
                    {
                        continue;
                    }
                    // Evaluate the source plane at the destination to preserve sloped occluders.
                    let edge = Fragment {
                        key: neighbor.key,
                        z: neighbor.z
                            + match direction {
                                0 => surface.dx,
                                1 => -surface.dx,
                                2 => surface.dy,
                                _ => -surface.dy,
                            },
                    };
                    if edge.wins(best, surfaces)
                        && (!surface.depth_test
                            || edge.z < depth[index]
                            || edge.z == body.z
                                && edge.z.to_bits().abs_diff(depth[index].to_bits()) <= 4)
                    {
                        best = edge;
                        ink = Some(surface.ink);
                    }
                }
                if let Some(color) = ink {
                    self.canvas.data[index] = color;
                }
            }
        }
    }
}
