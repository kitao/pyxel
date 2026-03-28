use crate::canvas::Canvas;
use crate::image::Color;
use crate::p3d::model::{FaceMaterial, Uv};

/// Screen-space vertex after projection.
#[derive(Clone, Copy)]
pub struct ScreenVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Triangle ready for rasterization.
#[derive(Clone, Copy)]
pub struct RasterTri {
    pub sv0: ScreenVertex,
    pub sv1: ScreenVertex,
    pub sv2: ScreenVertex,
    pub material: FaceMaterial,
    pub shade: f32,
}

/// Depth buffer.
pub struct ZBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
}

impl ZBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![f32::INFINITY; (width * height) as usize],
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(f32::INFINITY);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.data.resize((width * height) as usize, f32::INFINITY);
        self.clear();
    }

    /// Returns true if z is closer than the stored value, and updates the buffer.
    pub fn test_and_set(&mut self, x: u32, y: u32, z: f32) -> bool {
        let idx = (y * self.width + x) as usize;
        if z <= self.data[idx] {
            self.data[idx] = z;
            true
        } else {
            false
        }
    }
}

/// Number of shade palette levels (bright → dark).
pub const NUM_SHADE_LEVELS: usize = 5;

/// Shade palette: for each level, a 16→16 color remap.
/// Level 0 = brightest (highlight), Level 4 = darkest.
pub type ShadePalette = [[Color; 16]; NUM_SHADE_LEVELS];

/// Default shade palette built from the original shade chains.
/// Includes a highlight level (brighter than base).
pub const DEFAULT_SHADE_PALETTE: ShadePalette = [
    // Level 0: highlight — dark colors brighten, bright colors stay same
    [13, 5, 14, 11, 9, 12, 6, 7, 14, 10, 10, 11, 12, 13, 14, 15],
    // Level 1: original color (identity)
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    // Level 2: one step darker
    [0, 0, 1, 1, 1, 1, 12, 13, 2, 4, 9, 3, 5, 5, 8, 9],
    // Level 3: two steps darker
    [0, 0, 0, 0, 0, 0, 5, 5, 1, 1, 4, 1, 1, 1, 2, 4],
    // Level 4: darkest
    [0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1],
];

/// Shading result computed once per face: one or two palette remaps.
#[derive(Clone, Copy)]
pub enum FaceShade {
    /// All pixels use one palette remap.
    Solid(usize),
    /// 50% checkerboard between two adjacent levels.
    Checker(usize, usize),
}

/// Compute the face shade level from a shade factor.
/// Call once per face, not per pixel.
pub fn compute_face_shade(shade: f32) -> FaceShade {
    // Map shade (0.0..1.5) to palette levels (0=highlight .. 4=darkest)
    // shade 1.0 → level 1 (base), 0.0 → level 4 (darkest)
    let last = (NUM_SHADE_LEVELS - 2) as f32; // levels 1..4
    let pos = ((1.0 - shade.clamp(0.0, 1.0)) * last).min(last);
    let num_steps = 2 * (NUM_SHADE_LEVELS - 2);
    let step = (pos * 2.0) as usize;
    let step = step.min(num_steps);
    let level = step / 2 + 1;

    if step.is_multiple_of(2) {
        FaceShade::Solid(level)
    } else {
        FaceShade::Checker(level, level + 1)
    }
}

/// Resolve a pixel color from a precomputed face shade.
pub fn resolve_shade(
    face_shade: FaceShade,
    col: Color,
    px: i32,
    py: i32,
    palette: &ShadePalette,
) -> Color {
    match face_shade {
        FaceShade::Solid(level) => palette[level][(col & 0x0F) as usize],
        FaceShade::Checker(a, b) => {
            let level = if (px + py) & 1 == 0 { a } else { b };
            palette[level][(col & 0x0F) as usize]
        }
    }
}

/// Rasterize a single triangle into a canvas with Z-buffering.
///
/// - `ox`, `oy`: screen offset (viewport top-left)
/// - `images`: image bank pixel slices for texture lookup
/// - `image_widths`: width of each image bank
#[allow(clippy::too_many_arguments)]
pub fn rasterize_triangle(
    canvas: &mut Canvas<Color>,
    zbuf: &mut ZBuffer,
    tri: &RasterTri,
    ox: i32,
    oy: i32,
    images: &[&[Color]],
    image_widths: &[u32],
    palette: &ShadePalette,
) {
    // Precompute face shade level (once per triangle, not per pixel).
    let face_shade = compute_face_shade(tri.shade);

    // Unpack UVs from material (zero for solid-color triangles).
    let (uv0, uv1, uv2) = match tri.material {
        FaceMaterial::Texture { uv0, uv1, uv2, .. } => (uv0, uv1, uv2),
        FaceMaterial::Color(_) => (Uv::new(0.0, 0.0), Uv::new(0.0, 0.0), Uv::new(0.0, 0.0)),
    };

    // Sort vertices top-to-bottom by y.
    let (mut va, mut vb, mut vc) = (tri.sv0, tri.sv1, tri.sv2);
    let (mut ua, mut ub, mut uc) = (uv0, uv1, uv2);

    if vb.y < va.y {
        std::mem::swap(&mut va, &mut vb);
        std::mem::swap(&mut ua, &mut ub);
    }
    if vc.y < va.y {
        std::mem::swap(&mut va, &mut vc);
        std::mem::swap(&mut ua, &mut uc);
    }
    if vc.y < vb.y {
        std::mem::swap(&mut vb, &mut vc);
        std::mem::swap(&mut ub, &mut uc);
    }

    // Integer y range.
    let y_top = va.y.ceil() as i32;
    let y_mid = vb.y.ceil() as i32;
    let y_bot = vc.y.ceil() as i32;

    let total_dy = vc.y - va.y;

    // Rasterize upper half [y_top, y_mid) and lower half [y_mid, y_bot).
    for y in y_top..y_bot {
        let t_long = if total_dy > 0.0 {
            (y as f32 - va.y) / total_dy
        } else {
            0.0
        };

        // x/z/uv on the long edge va→vc.
        let x_long = va.x + (vc.x - va.x) * t_long;
        let z_long = va.z + (vc.z - va.z) * t_long;
        let u_long = ua.u + (uc.u - ua.u) * t_long;
        let v_long = ua.v + (uc.v - ua.v) * t_long;

        // x/z/uv on the short edge: va→vb (upper) or vb→vc (lower).
        let (x_short, z_short, u_short, v_short) = if y < y_mid {
            let dy_ab = vb.y - va.y;
            let t_short = if dy_ab > 0.0 {
                (y as f32 - va.y) / dy_ab
            } else {
                0.0
            };
            (
                va.x + (vb.x - va.x) * t_short,
                va.z + (vb.z - va.z) * t_short,
                ua.u + (ub.u - ua.u) * t_short,
                ua.v + (ub.v - ua.v) * t_short,
            )
        } else {
            let dy_bc = vc.y - vb.y;
            let t_short = if dy_bc > 0.0 {
                (y as f32 - vb.y) / dy_bc
            } else {
                0.0
            };
            (
                vb.x + (vc.x - vb.x) * t_short,
                vb.z + (vc.z - vb.z) * t_short,
                ub.u + (uc.u - ub.u) * t_short,
                ub.v + (uc.v - ub.v) * t_short,
            )
        };

        // Establish left/right.
        let (xl, xr, zl, zr, ul, ur, vl, vr) = if x_long <= x_short {
            (
                x_long, x_short, z_long, z_short, u_long, u_short, v_long, v_short,
            )
        } else {
            (
                x_short, x_long, z_short, z_long, u_short, u_long, v_short, v_long,
            )
        };

        // Pixel x range on this scanline.
        let x_start = xl.ceil() as i32;
        let x_end = xr.ceil() as i32;
        let span = xr - xl;

        let py = y + oy;
        if py < 0 || py >= zbuf.height as i32 || py >= canvas.height() as i32 {
            continue;
        }

        for x in x_start..x_end {
            let px = x + ox;
            if px < 0 || px >= zbuf.width as i32 || px >= canvas.width() as i32 {
                continue;
            }

            // Interpolate z across the scanline.
            let t_x = if span > 0.0 {
                (x as f32 - xl) / span
            } else {
                0.0
            };
            let z = zl + (zr - zl) * t_x;

            if !zbuf.test_and_set(px as u32, py as u32, z) {
                continue;
            }

            // Resolve color through shade palette.
            let base_col = match tri.material {
                FaceMaterial::Color(c) => c,
                FaceMaterial::Texture { img, .. } => {
                    let img_idx = img as usize;
                    if img_idx >= images.len() {
                        continue;
                    }
                    let iw = image_widths[img_idx];
                    let ih = if iw > 0 {
                        images[img_idx].len() as u32 / iw
                    } else {
                        continue;
                    };
                    let u = ul + (ur - ul) * t_x;
                    let v = vl + (vr - vl) * t_x;
                    let tx = (u * iw as f32).rem_euclid(iw as f32) as u32;
                    let ty = (v * ih as f32).rem_euclid(ih as f32) as u32;
                    images[img_idx][(ty * iw + tx) as usize]
                }
            };
            let col = resolve_shade(face_shade, base_col, px, py, palette);

            canvas.write_data(px as usize, py as usize, col);
        }
    }
}
