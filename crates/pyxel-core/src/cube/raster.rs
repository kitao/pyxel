// Raster formulas use conventional x/y/z/u/v component names.
#![allow(clippy::many_single_char_names)]

use crate::cube::camera::Camera;
use crate::cube::shading::{Shading, LEVEL_COUNT};
use crate::cube::vec3::{sum_wide_products, Vec3};
use crate::image::Image;
use crate::utils::{f32_to_i32, f32_to_u32};

// Row-major matrix: m[i][j] is row i, column j.
pub type Mat4x4 = [[f32; 4]; 4];

type ScreenPoint = (f32, f32, f32);

// A geometric border uses its own face depth at the same pixel center as
// the fill. Keeping a single depth value preserves ordering across primitives.
#[derive(Clone, Copy)]
pub(crate) struct ScreenPlane {
    origin: ScreenPoint,
    dx: f32,
    dy: f32,
}

impl ScreenPlane {
    pub(crate) fn from_triangle(p0: ScreenPoint, p1: ScreenPoint, p2: ScreenPoint) -> Option<Self> {
        let area = edge_function((p0.0, p0.1), (p1.0, p1.1), (p2.0, p2.1));
        if area.abs() < 1e-6 {
            return None;
        }
        Some(Self {
            origin: p0,
            dx: edge_function((p0.2, p0.1), (p1.2, p1.1), (p2.2, p2.1)) / area,
            dy: edge_function((p0.0, p0.2), (p1.0, p1.2), (p2.0, p2.2)) / area,
        })
    }

    pub(crate) fn slope_squared(self) -> f32 {
        self.dx * self.dx + self.dy * self.dy
    }

    fn z_at(self, x: f32, y: f32) -> f32 {
        self.origin.2 + self.dx * (x - self.origin.0) + self.dy * (y - self.origin.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ClipRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl ClipRect {
    #[inline]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }
}

// Clamp the viewport rectangle to the target image bounds, in pixels.
pub fn compute_clip_rect(
    vp_x: f32,
    vp_y: f32,
    vp_w: f32,
    vp_h: f32,
    target_w: u32,
    target_h: u32,
) -> ClipRect {
    let left = vp_x.floor() as i32;
    let top = vp_y.floor() as i32;
    let right = ((vp_x + vp_w).ceil() as i32).saturating_sub(1);
    let bottom = ((vp_y + vp_h).ceil() as i32).saturating_sub(1);
    ClipRect {
        left: left.max(0),
        top: top.max(0),
        right: (right as i64).min(target_w as i64 - 1) as i32,
        bottom: (bottom as i64).min(target_h as i64 - 1) as i32,
    }
}

// The clip is target-clamped; clearing ignores 2D camera, clip and dithering.
pub fn clear_viewport(target: &mut Image, clip: &ClipRect, color: u8) {
    if clip.left > clip.right || clip.top > clip.bottom {
        return;
    }

    let color = target.palette[color as usize];
    let stride = target.width() as usize;
    for y in clip.top..=clip.bottom {
        let row = y as usize * stride;
        target.canvas.data[row + clip.left as usize..=row + clip.right as usize].fill(color);
    }
}

// View-projection helpers

pub fn view_matrix(camera: &Camera) -> Mat4x4 {
    rc_ref!(&camera.transform).inverse_value().data
}

pub fn projection_matrix(camera: &Camera, vp_w: f32, vp_h: f32) -> Mat4x4 {
    let aspect = if vp_h == 0.0 { 1.0 } else { vp_w / vp_h };
    let near = camera.near;
    let far = camera.far;

    let depth_sum = far + near;
    let depth_delta = far - near;
    let depth_product = 2.0 * far * near;
    let needs_width = !depth_sum.is_finite()
        || !depth_delta.is_finite()
        || (camera.ortho_size.is_none() && !depth_product.is_normal() && far != 0.0 && near != 0.0);
    if needs_width && near.is_finite() && far.is_finite() && aspect.is_finite() {
        return projection_matrix_wide(camera, aspect);
    }

    let matrix = if let Some(size) = camera.ortho_size {
        let half_h = size * 0.5;
        let half_w = half_h * aspect;
        [
            [1.0 / half_w, 0.0, 0.0, 0.0],
            [0.0, 1.0 / half_h, 0.0, 0.0],
            [0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near)],
            [0.0, 0.0, 0.0, 1.0],
        ]
    } else {
        let f = 1.0 / (camera.fov.to_radians() * 0.5).tan();
        [
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (far + near) / (near - far),
                2.0 * far * near / (near - far),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ]
    };
    if matrix.iter().flatten().any(|value| !value.is_finite())
        && near.is_finite()
        && far.is_finite()
        && aspect.is_finite()
    {
        projection_matrix_wide(camera, aspect)
    } else {
        matrix
    }
}

// A homogeneous matrix may be scaled without changing its projected coordinates.
// Keep the ordinary f32 path; recover overflowing coefficients or underflowing depth terms.
#[cold]
fn projection_matrix_wide(camera: &Camera, aspect: f32) -> Mat4x4 {
    let near = f64::from(camera.near);
    let far = f64::from(camera.far);
    let aspect = f64::from(aspect);
    let matrix = if let Some(size) = camera.ortho_size {
        let half_h = f64::from(size) * 0.5;
        let half_w = half_h * aspect;
        [
            [1.0 / half_w, 0.0, 0.0, 0.0],
            [0.0, 1.0 / half_h, 0.0, 0.0],
            [0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near)],
            [0.0, 0.0, 0.0, 1.0],
        ]
    } else {
        // Match the ordinary angular rounding; only the depth arithmetic needs width.
        let f = f64::from(1.0 / (camera.fov.to_radians() * 0.5).tan());
        [
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (far + near) / (near - far),
                2.0 * far * near / (near - far),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ]
    };
    let largest = matrix
        .iter()
        .flatten()
        .map(|value| value.abs())
        .fold(0.0_f64, f64::max);
    let scale = if largest.is_finite() {
        (largest / f64::from(f32::MAX)).max(1.0)
    } else {
        1.0
    };
    matrix.map(|row| row.map(|value| (value / scale) as f32))
}

pub fn matmul(a: &Mat4x4, b: &Mat4x4) -> Mat4x4 {
    let mut result = [[0.0_f32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    if result.iter().flatten().all(|value| value.is_finite()) {
        result
    } else {
        recover_projective_product(a, b, result)
    }
}

#[cold]
fn recover_projective_product(a: &Mat4x4, b: &Mat4x4, fallback: Mat4x4) -> Mat4x4 {
    if !a.iter().chain(b).flatten().all(|value| value.is_finite()) {
        return fallback;
    }
    let wide: [[f64; 4]; 4] = std::array::from_fn(|i| {
        std::array::from_fn(|j| sum_wide_products(std::array::from_fn(|k| (a[i][k], b[k][j]))))
    });
    let largest = wide
        .iter()
        .flatten()
        .map(|value| value.abs())
        .fold(0.0_f64, f64::max);
    let scale = (largest / f64::from(f32::MAX)).max(1.0);
    wide.map(|row| row.map(|value| (value / scale) as f32))
}

// Dot with a world position (w = 1) to get distance in front of the camera.
// Orthographic clipping needs this because its projection w is constant.
pub fn camera_clip_row(view: &Mat4x4) -> [f32; 4] {
    [-view[2][0], -view[2][1], -view[2][2], -view[2][3]]
}

// Reject points at or behind the camera. Keep off-screen and far-plane
// overshoot vertices so partially visible primitives can still rasterize.
pub fn world_to_screen(
    pos: &Vec3,
    m: &Mat4x4,
    clip_row: &[f32; 4],
    vp_x: f32,
    vp_y: f32,
    vp_w: f32,
    vp_h: f32,
) -> Option<(f32, f32, f32)> {
    let front = clip_row[0] * pos.x + clip_row[1] * pos.y + clip_row[2] * pos.z + clip_row[3];
    if front <= 0.0 {
        return None;
    }

    let cx = m[0][0] * pos.x + m[0][1] * pos.y + m[0][2] * pos.z + m[0][3];
    let cy = m[1][0] * pos.x + m[1][1] * pos.y + m[1][2] * pos.z + m[1][3];
    let cz = m[2][0] * pos.x + m[2][1] * pos.y + m[2][2] * pos.z + m[2][3];
    let cw = m[3][0] * pos.x + m[3][1] * pos.y + m[3][2] * pos.z + m[3][3];

    let [ndc_x, ndc_y, ndc_z] = if [cx, cy, cz, cw].iter().all(|value| value.is_finite()) {
        [cx / cw, cy / cw, cz / cw]
    } else if pos.is_finite() && m.iter().flatten().all(|value| value.is_finite()) {
        // Divide wide homogeneous components before narrowing: the ratio can fit
        // even when a component or a cancelling partial sum does not.
        let clip = m.map(|row| {
            sum_wide_products([
                (row[0], pos.x),
                (row[1], pos.y),
                (row[2], pos.z),
                (row[3], 1.0),
            ])
        });
        [
            (clip[0] / clip[3]) as f32,
            (clip[1] / clip[3]) as f32,
            (clip[2] / clip[3]) as f32,
        ]
    } else {
        [cx / cw, cy / cw, cz / cw]
    };
    let sx = vp_x + (ndc_x + 1.0) * 0.5 * vp_w;
    let sy = vp_y + (1.0 - (ndc_y + 1.0) * 0.5) * vp_h;
    Some((sx, sy, ndc_z))
}

// Geometry helpers

// CCW face normal. Preserve ordinary area vectors; recover direction
// before finite edge/cross intermediates overflow or underflow.
pub fn tri_normal(p0: &Vec3, p1: &Vec3, p2: &Vec3) -> Vec3 {
    let e1 = [p1.x - p0.x, p1.y - p0.y, p1.z - p0.z];
    let e2 = [p2.x - p0.x, p2.y - p0.y, p2.z - p0.z];
    let factors = [
        (e1[1], e2[2]),
        (e1[2], e2[1]),
        (e1[2], e2[0]),
        (e1[0], e2[2]),
        (e1[0], e2[1]),
        (e1[1], e2[0]),
    ];
    let products = factors.map(|(a, b)| a * b);
    let normal = Vec3 {
        x: products[0] - products[1],
        y: products[2] - products[3],
        z: products[4] - products[5],
    };
    let needs_width = !normal.is_finite()
        || products
            .iter()
            .zip(factors)
            .any(|(product, (a, b))| !product.is_normal() && a != 0.0 && b != 0.0);
    if needs_width && p0.is_finite() && p1.is_finite() && p2.is_finite() {
        let e1 = [
            f64::from(p1.x) - f64::from(p0.x),
            f64::from(p1.y) - f64::from(p0.y),
            f64::from(p1.z) - f64::from(p0.z),
        ];
        let e2 = [
            f64::from(p2.x) - f64::from(p0.x),
            f64::from(p2.y) - f64::from(p0.y),
            f64::from(p2.z) - f64::from(p0.z),
        ];
        return narrow_face_normal([
            e1[1] * e2[2] - e1[2] * e2[1],
            e1[2] * e2[0] - e1[0] * e2[2],
            e1[0] * e2[1] - e1[1] * e2[0],
        ]);
    }
    normal
}

// Face shading consumes direction: rescale the finite wide result before narrowing it.
pub(crate) fn narrow_face_normal(normal: [f64; 3]) -> Vec3 {
    let scale = normal.iter().map(|v| v.abs()).fold(0.0_f64, f64::max);
    if scale == 0.0 {
        return Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    Vec3 {
        x: (normal[0] / scale) as f32,
        y: (normal[1] / scale) as f32,
        z: (normal[2] / scale) as f32,
    }
}

// Camera right and up axes in world space.
pub fn camera_right_up(camera: &Camera) -> (Vec3, Vec3) {
    let m = rc_ref!(&camera.transform).data;
    (
        Vec3 {
            x: m[0][0],
            y: m[1][0],
            z: m[2][0],
        },
        Vec3 {
            x: m[0][1],
            y: m[1][1],
            z: m[2][1],
        },
    )
}

// Billboard sprite corners: a quad facing the camera, rotated by
// `angle_deg` counterclockwise as seen from the camera (around view-z).
// Corners are returned in row-major order: top-left, top-right, bottom-left, bottom-right.
pub fn sprite_corners(pos: &Vec3, w: f32, h: f32, angle_deg: f32, camera: &Camera) -> [Vec3; 4] {
    let (right, up) = camera_right_up(camera);
    let rad = angle_deg.to_radians();
    let c = rad.cos();
    let s = rad.sin();

    let rright = Vec3 {
        x: c * right.x + s * up.x,
        y: c * right.y + s * up.y,
        z: c * right.z + s * up.z,
    };
    let rup = Vec3 {
        x: -s * right.x + c * up.x,
        y: -s * right.y + c * up.y,
        z: -s * right.z + c * up.z,
    };

    let hw = w * 0.5;
    let hh = h * 0.5;
    [
        Vec3 {
            x: pos.x - hw * rright.x + hh * rup.x,
            y: pos.y - hw * rright.y + hh * rup.y,
            z: pos.z - hw * rright.z + hh * rup.z,
        },
        Vec3 {
            x: pos.x + hw * rright.x + hh * rup.x,
            y: pos.y + hw * rright.y + hh * rup.y,
            z: pos.z + hw * rright.z + hh * rup.z,
        },
        Vec3 {
            x: pos.x - hw * rright.x - hh * rup.x,
            y: pos.y - hw * rright.y - hh * rup.y,
            z: pos.z - hw * rright.z - hh * rup.z,
        },
        Vec3 {
            x: pos.x + hw * rright.x - hh * rup.x,
            y: pos.y + hw * rright.y - hh * rup.y,
            z: pos.z + hw * rright.z - hh * rup.z,
        },
    ]
}

// Project a screen-aligned circle. The radius is sampled along the
// camera's right axis; sampling world +X collapses to zero when the
// camera aligns with that axis.
pub fn screen_circle(
    pos: &Vec3,
    radius: f32,
    m: &Mat4x4,
    clip_row: &[f32; 4],
    camera: &Camera,
    vp_x: f32,
    vp_y: f32,
    vp_w: f32,
    vp_h: f32,
) -> Option<(f32, f32, f32, f32)> {
    let (right, _up) = camera_right_up(camera);
    let center = world_to_screen(pos, m, clip_row, vp_x, vp_y, vp_w, vp_h)?;
    let edge_pos = Vec3 {
        x: pos.x + radius * right.x,
        y: pos.y + radius * right.y,
        z: pos.z + radius * right.z,
    };
    let edge = world_to_screen(&edge_pos, m, clip_row, vp_x, vp_y, vp_w, vp_h)?;

    let dx = edge.0 - center.0;
    let dy = edge.1 - center.1;
    let screen_r = (dx * dx + dy * dy).sqrt();
    Some((center.0, center.1, screen_r, center.2))
}

// Pixel output and shading

// Alpha uses Bayer thresholds; shading uses the checker in dither_pick.
pub const BAYER4: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];
// Match Canvas circle boundary rounding.
const CIRCLE_ROUNDING_BIAS: f32 = 0.01;
// Resolve floating-point ties between a border and its own surface.
const LINE_DEPTH_BIAS: f32 = 1.0e-5;

// Pick between primary and secondary for the LUT cell at pixel (x, y).
// `primary == secondary` is a flat fill; otherwise a 2x2 checker.
#[inline]
pub fn dither_pick(primary: i32, secondary: i32, x: i32, y: i32) -> u8 {
    if primary == secondary {
        return primary as u8;
    }
    if (x + y).rem_euclid(2) == 0 {
        primary as u8
    } else {
        secondary as u8
    }
}

// Quantize positive Lambert brightness into the palette's shade levels.
pub fn face_shade_level(direction: &Vec3, normal: Option<&Vec3>) -> usize {
    let dot_factor = match normal {
        Some(n) => {
            let n_len_sq = n.x * n.x + n.y * n.y + n.z * n.z;
            let d_len_sq =
                direction.x * direction.x + direction.y * direction.y + direction.z * direction.z;
            let n_len = n_len_sq.sqrt();
            let d_len = d_len_sq.sqrt();
            let length_product = n_len * d_len;
            let dot = n.x * direction.x + n.y * direction.y + n.z * direction.z;
            if (!n_len_sq.is_normal()
                || !d_len_sq.is_normal()
                || !length_product.is_normal()
                || !dot.is_finite())
                && n.is_finite()
                && direction.is_finite()
            {
                let length_product = n.length_f64() * direction.length_f64();
                if length_product == 0.0 {
                    return 0;
                }
                let dot = f64::from(n.x) * f64::from(direction.x)
                    + f64::from(n.y) * f64::from(direction.y)
                    + f64::from(n.z) * f64::from(direction.z);
                (-dot / length_product) as f32
            } else {
                if n_len == 0.0 || d_len == 0.0 {
                    return 0;
                }
                -dot / length_product
            }
        }
        None => return 0,
    };

    (dot_factor.clamp(0.0, 1.0) * (LEVEL_COUNT - 1) as f32).round() as usize
}

// Resolve the shade once per face, then reuse the pair with dither_pick.
pub fn lookup_ramp(shading: &Shading, base_col: i32, normal: Option<&Vec3>) -> (i32, i32) {
    let palette_size = shading.palette_size();
    if palette_size == 0 {
        let c = base_col.max(0);
        return (c, c);
    }

    let direction = rc_ref!(&shading.direction);
    let level = face_shade_level(&direction, normal);
    let col = base_col.clamp(0, palette_size as i32 - 1) as usize;
    shading.get(col, level)
}

// Callers enforce clip bounds to avoid a second check per pixel.
#[inline]
pub fn write_pixel(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    x: i32,
    y: i32,
    z: f32,
    col: u8,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) {
    let Some(i) = visible_pixel_index(depth, depth_w, x, y, z, dither_alpha, depth_test) else {
        return;
    };
    write_visible_pixel(target, depth, i, x, y, z, col, depth_write);
}

#[inline]
fn visible_pixel_index(
    depth: &[f32],
    depth_w: u32,
    x: i32,
    y: i32,
    z: f32,
    dither_alpha: f32,
    depth_test: bool,
) -> Option<usize> {
    if !depth_in_clip_range(z) {
        return None;
    }
    if dither_alpha < 1.0 {
        let bayer = BAYER4[(y.rem_euclid(4)) as usize][(x.rem_euclid(4)) as usize];
        let threshold = (bayer as f32 + 0.5) / 16.0;
        if (1.0 - dither_alpha) >= threshold {
            return None;
        }
    }

    let i = (y as usize) * depth_w as usize + x as usize;
    if depth_test && z >= depth[i] {
        return None;
    }
    Some(i)
}

#[inline]
fn depth_in_clip_range(z: f32) -> bool {
    // Projection arithmetic can move exact near/far endpoints a few f32 ULPs.
    const EPSILON: f32 = 8.0 * f32::EPSILON;
    (-1.0 - EPSILON..=1.0 + EPSILON).contains(&z)
}

#[inline]
fn write_visible_pixel(
    target: &mut Image,
    depth: &mut [f32],
    i: usize,
    x: i32,
    y: i32,
    z: f32,
    col: u8,
    depth_write: bool,
) {
    if depth_write {
        depth[i] = z;
    }
    target.canvas.write_data(x as usize, y as usize, col);
}

// Rasterizers

// Signed doubled area, also used for culling and the border plane's determinants.
// Four epsilons conservatively bound rounding in the two differences per
// product and their subtraction; subnormal uncertainty also takes the wide path.
#[inline]
pub(crate) fn edge_function(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    let left = (b.0 - a.0) * (c.1 - a.1);
    let right = (b.1 - a.1) * (c.0 - a.0);
    let edge = left - right;
    let error = (4.0 * f32::EPSILON * (left.abs() + right.abs())).max(f32::MIN_POSITIVE);
    if edge.abs() > error {
        edge
    } else {
        edge_function_wide(a, b, c)
    }
}

// Hoist a conservative cancellation bound over the clipped pixel rectangle.
// Ordinary weights retain their f32 operation order. Only uncertain edge signs
// use a wider, shared expression, preserving complementary boundary ownership.
struct CoverageEdge {
    a: (f32, f32),
    b: (f32, f32),
    error: f32,
}

impl CoverageEdge {
    fn new(a: (f32, f32), b: (f32, f32), clip: ClipRect) -> Self {
        let max_dx = (clip.left as f32 + 0.5 - a.0)
            .abs()
            .max((clip.right as f32 + 0.5 - a.0).abs());
        let max_dy = (clip.top as f32 + 0.5 - a.1)
            .abs()
            .max((clip.bottom as f32 + 0.5 - a.1).abs());
        let magnitude = (b.0 - a.0).abs() * max_dy + (b.1 - a.1).abs() * max_dx;
        Self {
            a,
            b,
            error: (4.0 * f32::EPSILON * magnitude).max(f32::MIN_POSITIVE),
        }
    }

    #[inline]
    fn inside_weight(&self, p: (f32, f32), include_boundary: bool, pos_area: bool) -> Option<f32> {
        let edge =
            (self.b.0 - self.a.0) * (p.1 - self.a.1) - (self.b.1 - self.a.1) * (p.0 - self.a.0);
        if pos_area {
            if edge > self.error {
                return Some(edge);
            }
            if edge < -self.error {
                return None;
            }
        } else {
            if edge < -self.error {
                return Some(edge);
            }
            if edge > self.error {
                return None;
            }
        }
        let edge = edge_function_wide(self.a, self.b, p);
        edge_inside(edge, include_boundary, pos_area).then_some(edge)
    }
}

#[cold]
fn edge_function_wide(a: (f32, f32), b: (f32, f32), p: (f32, f32)) -> f32 {
    let reverse = a.0 > b.0 || (a.0 == b.0 && a.1 > b.1);
    let (a, b) = if reverse { (b, a) } else { (a, b) };
    let edge = (f64::from(b.0) - f64::from(a.0)) * (f64::from(p.1) - f64::from(a.1))
        - (f64::from(b.1) - f64::from(a.1)) * (f64::from(p.0) - f64::from(a.0));
    (if reverse { -edge } else { edge }) as f32
}

#[inline]
fn is_top_left_edge(a: (f32, f32), b: (f32, f32)) -> bool {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    dy < 0.0 || (dy == 0.0 && dx > 0.0)
}

#[inline]
fn includes_edge_boundary(a: (f32, f32), b: (f32, f32), pos_area: bool) -> bool {
    if pos_area {
        is_top_left_edge(a, b)
    } else {
        is_top_left_edge(b, a)
    }
}

#[inline]
fn edge_inside(w: f32, include_boundary: bool, pos_area: bool) -> bool {
    if pos_area {
        w > 0.0 || (w == 0.0 && include_boundary)
    } else {
        w < 0.0 || (w == 0.0 && include_boundary)
    }
}

// Interpolate z linearly. Both windings draw; draw::prim handles culling.
pub fn rasterize_triangle(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    p0: (f32, f32, f32),
    p1: (f32, f32, f32),
    p2: (f32, f32, f32),
    primary: u8,
    secondary: u8,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) {
    let area = edge_function((p0.0, p0.1), (p1.0, p1.1), (p2.0, p2.1));
    if area.abs() < 1e-6 {
        return;
    }
    let inv_area = 1.0 / area;

    let min_x = p0.0.min(p1.0).min(p2.0).floor() as i32;
    let max_x = p0.0.max(p1.0).max(p2.0).ceil() as i32;
    let min_y = p0.1.min(p1.1).min(p2.1).floor() as i32;
    let max_y = p0.1.max(p1.1).max(p2.1).ceil() as i32;
    let bx_min = min_x.max(clip.left);
    let bx_max = max_x.min(clip.right);
    let by_min = min_y.max(clip.top);
    let by_max = max_y.min(clip.bottom);
    if bx_min > bx_max || by_min > by_max {
        return;
    }

    // Winding and top-left ownership are hoisted out of the pixel loop.
    // The half-open edge rule keeps adjacent triangles from drawing the
    // same shared-edge pixel in different orders.
    let pos_area = area > 0.0;
    let include_w0 = includes_edge_boundary((p1.0, p1.1), (p2.0, p2.1), pos_area);
    let include_w1 = includes_edge_boundary((p2.0, p2.1), (p0.0, p0.1), pos_area);
    let include_w2 = includes_edge_boundary((p0.0, p0.1), (p1.0, p1.1), pos_area);
    let pixel_bounds = ClipRect {
        left: bx_min,
        top: by_min,
        right: bx_max,
        bottom: by_max,
    };
    let edge0 = CoverageEdge::new((p1.0, p1.1), (p2.0, p2.1), pixel_bounds);
    let edge1 = CoverageEdge::new((p2.0, p2.1), (p0.0, p0.1), pixel_bounds);
    let edge2 = CoverageEdge::new((p0.0, p0.1), (p1.0, p1.1), pixel_bounds);

    for y in by_min..=by_max {
        let py = y as f32 + 0.5;
        // A triangle intersects each scanline in one contiguous span. Once
        // the loop leaves that span, no later pixel on the row can be inside.
        let mut was_inside = false;

        for x in bx_min..=bx_max {
            let p = (x as f32 + 0.5, py);
            let Some(w0) = edge0.inside_weight(p, include_w0, pos_area) else {
                if was_inside {
                    break;
                }
                continue;
            };

            let Some(w1) = edge1.inside_weight(p, include_w1, pos_area) else {
                if was_inside {
                    break;
                }
                continue;
            };

            let Some(w2) = edge2.inside_weight(p, include_w2, pos_area) else {
                if was_inside {
                    break;
                }
                continue;
            };
            was_inside = true;

            let bary0 = w0 * inv_area;
            let bary1 = w1 * inv_area;
            let bary2 = w2 * inv_area;
            let z = bary0 * p0.2 + bary1 * p1.2 + bary2 * p2.2;
            let col = dither_pick(primary as i32, secondary as i32, x, y);
            if let Some(i) = visible_pixel_index(depth, depth_w, x, y, z, dither_alpha, depth_test)
            {
                write_visible_pixel(target, depth, i, x, y, z, col, depth_write);
            }
        }
    }
}

// UV and z interpolation is affine for the pixel-art scale. The sampler
// receives screen coordinates for dithering; None skips transparent pixels.
pub fn rasterize_textured_triangle<F>(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    p0: (f32, f32, f32),
    p1: (f32, f32, f32),
    p2: (f32, f32, f32),
    uv0: (f32, f32),
    uv1: (f32, f32),
    uv2: (f32, f32),
    sampler: F,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) where
    F: Fn(f32, f32, i32, i32) -> Option<i32>,
{
    let area = edge_function((p0.0, p0.1), (p1.0, p1.1), (p2.0, p2.1));
    if area.abs() < 1e-6 {
        return;
    }
    let inv_area = 1.0 / area;

    let min_x = p0.0.min(p1.0).min(p2.0).floor() as i32;
    let max_x = p0.0.max(p1.0).max(p2.0).ceil() as i32;
    let min_y = p0.1.min(p1.1).min(p2.1).floor() as i32;
    let max_y = p0.1.max(p1.1).max(p2.1).ceil() as i32;
    let bx_min = min_x.max(clip.left);
    let bx_max = max_x.min(clip.right);
    let by_min = min_y.max(clip.top);
    let by_max = max_y.min(clip.bottom);
    if bx_min > bx_max || by_min > by_max {
        return;
    }

    let pos_area = area > 0.0;
    let include_w0 = includes_edge_boundary((p1.0, p1.1), (p2.0, p2.1), pos_area);
    let include_w1 = includes_edge_boundary((p2.0, p2.1), (p0.0, p0.1), pos_area);
    let include_w2 = includes_edge_boundary((p0.0, p0.1), (p1.0, p1.1), pos_area);
    let pixel_bounds = ClipRect {
        left: bx_min,
        top: by_min,
        right: bx_max,
        bottom: by_max,
    };
    let edge0 = CoverageEdge::new((p1.0, p1.1), (p2.0, p2.1), pixel_bounds);
    let edge1 = CoverageEdge::new((p2.0, p2.1), (p0.0, p0.1), pixel_bounds);
    let edge2 = CoverageEdge::new((p0.0, p0.1), (p1.0, p1.1), pixel_bounds);

    for y in by_min..=by_max {
        let py = y as f32 + 0.5;
        // Same lazy-edge, contiguous-span row scan as rasterize_triangle.
        let mut was_inside = false;

        for x in bx_min..=bx_max {
            let p = (x as f32 + 0.5, py);
            let Some(w0) = edge0.inside_weight(p, include_w0, pos_area) else {
                if was_inside {
                    break;
                }
                continue;
            };

            let Some(w1) = edge1.inside_weight(p, include_w1, pos_area) else {
                if was_inside {
                    break;
                }
                continue;
            };

            let Some(w2) = edge2.inside_weight(p, include_w2, pos_area) else {
                if was_inside {
                    break;
                }
                continue;
            };
            was_inside = true;

            let bary0 = w0 * inv_area;
            let bary1 = w1 * inv_area;
            let bary2 = w2 * inv_area;
            let z = bary0 * p0.2 + bary1 * p1.2 + bary2 * p2.2;
            if let Some(i) = visible_pixel_index(depth, depth_w, x, y, z, dither_alpha, depth_test)
            {
                let u = bary0 * uv0.0 + bary1 * uv1.0 + bary2 * uv2.0;
                let v = bary0 * uv0.1 + bary1 * uv1.1 + bary2 * uv2.1;
                if let Some(col) = sampler(u, v, x, y) {
                    write_visible_pixel(target, depth, i, x, y, z, col as u8, depth_write);
                }
            }
        }
    }
}

// Filled screen-space circle at constant depth. cx / cy / radius are in
// pixels (project a world-space circle through `screen_circle` first).
pub fn rasterize_circle_filled(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    cx: f32,
    cy: f32,
    radius: f32,
    z: f32,
    primary: u8,
    secondary: u8,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) {
    if !depth_in_clip_range(z) {
        return;
    }
    let x = f32_to_i32(cx);
    let y = f32_to_i32(cy);
    let radius = f32_to_u32(radius);
    let ranges = circle_scan_ranges(x, y, radius, clip);
    let x = i64::from(x);
    let y = i64::from(y);

    for xi in ranges.into_iter().flat_map(|(start, end)| start..=end) {
        let (x1, y1, x2, y2) = circle_area(radius, xi);
        rasterize_circle_column(
            target,
            depth,
            depth_w,
            y + y1,
            y + y2,
            x + x1,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
        rasterize_circle_column(
            target,
            depth,
            depth_w,
            y + y1,
            y + y2,
            x + x2,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );

        rasterize_circle_row(
            target,
            depth,
            depth_w,
            x + y1,
            x + y2,
            y + x1,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
        rasterize_circle_row(
            target,
            depth,
            depth_w,
            x + y1,
            x + y2,
            y + x2,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
    }
}

// Rounded screen-space circle outline at constant depth.
pub fn rasterize_circle_border(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    cx: f32,
    cy: f32,
    radius: f32,
    z: f32,
    primary: u8,
    secondary: u8,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) {
    if !depth_in_clip_range(z) {
        return;
    }
    let x = f32_to_i32(cx);
    let y = f32_to_i32(cy);
    let radius = f32_to_u32(radius);
    let ranges = circle_scan_ranges(x, y, radius, clip);
    let x = i64::from(x);
    let y = i64::from(y);

    for xi in ranges.into_iter().flat_map(|(start, end)| start..=end) {
        let (x1, y1, x2, y2) = circle_area(radius, xi);
        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x + x1,
            y + y1,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x + x2,
            y + y1,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x + x1,
            y + y2,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x + x2,
            y + y2,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );

        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x + y1,
            y + x1,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x + y1,
            y + x2,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x + y2,
            y + x1,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x + y2,
            y + x2,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
    }
}

// Every symmetric span or pixel has one coordinate at +/- xi. Only the
// offsets crossing a viewport axis can contribute, even when the circle encloses it.
fn circle_scan_ranges(x: i32, y: i32, radius: u32, clip: ClipRect) -> [(i64, i64); 2] {
    if clip.left > clip.right || clip.top > clip.bottom {
        return [(1, 0); 2];
    }

    let mut ranges = [
        circle_axis_range(x, clip.left, clip.right),
        circle_axis_range(y, clip.top, clip.bottom),
    ];
    for range in &mut ranges {
        range.1 = range.1.min(i64::from(radius));
    }

    if ranges[0].0 > ranges[1].0 {
        ranges.swap(0, 1);
    }
    if ranges[1].0 <= ranges[0].1 + 1 {
        ranges[0].1 = ranges[0].1.max(ranges[1].1);
        ranges[1] = (1, 0);
    }
    ranges
}

fn circle_axis_range(center: i32, min: i32, max: i32) -> (i64, i64) {
    let min = i64::from(min) - i64::from(center);
    let max = i64::from(max) - i64::from(center);
    let nearest = if min <= 0 && max >= 0 {
        0
    } else {
        min.abs().min(max.abs())
    };
    (nearest, min.abs().max(max.abs()))
}

#[inline]
fn circle_area(radius: u32, x: i64) -> (i64, i64, i64, i64) {
    // Keep ordinary-radius pixel rounding; f64 retains subpixel precision once
    // f32 steps reach half a pixel. Offsets stay wide until viewport clipping.
    if radius < (1 << 22) {
        let r = radius as f32;
        let dx = x as f32;
        let dy = if r > 0.0 {
            r * (1.0 - dx * dx / (r * r)).sqrt()
        } else {
            r
        };
        (
            (-dx - CIRCLE_ROUNDING_BIAS).round() as i64,
            (-dy - CIRCLE_ROUNDING_BIAS).round() as i64,
            (dx + CIRCLE_ROUNDING_BIAS).round() as i64,
            (dy + CIRCLE_ROUNDING_BIAS).round() as i64,
        )
    } else {
        let r = f64::from(radius);
        let dx = x as f64;
        let dy = r * (1.0 - dx * dx / (r * r)).sqrt();
        let bias = f64::from(CIRCLE_ROUNDING_BIAS);
        (
            (-dx - bias).round() as i64,
            (-dy - bias).round() as i64,
            (dx + bias).round() as i64,
            (dy + bias).round() as i64,
        )
    }
}

#[inline]
fn rasterize_circle_pixel(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    x: i64,
    y: i64,
    z: f32,
    primary: u8,
    secondary: u8,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) {
    if x < i64::from(clip.left)
        || x > i64::from(clip.right)
        || y < i64::from(clip.top)
        || y > i64::from(clip.bottom)
    {
        return;
    }

    let x = x as i32;
    let y = y as i32;
    let col = dither_pick(primary as i32, secondary as i32, x, y);
    write_pixel(
        target,
        depth,
        depth_w,
        x,
        y,
        z,
        col,
        dither_alpha,
        depth_test,
        depth_write,
    );
}

#[inline]
fn rasterize_circle_row(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    x1: i64,
    x2: i64,
    y: i64,
    z: f32,
    primary: u8,
    secondary: u8,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) {
    if y < i64::from(clip.top) || y > i64::from(clip.bottom) {
        return;
    }
    let left = x1.max(i64::from(clip.left));
    let right = x2.min(i64::from(clip.right));

    for x in left..=right {
        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x,
            y,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
    }
}

#[inline]
fn rasterize_circle_column(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    y1: i64,
    y2: i64,
    x: i64,
    z: f32,
    primary: u8,
    secondary: u8,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) {
    if x < i64::from(clip.left) || x > i64::from(clip.right) {
        return;
    }
    let top = y1.max(i64::from(clip.top));
    let bottom = y2.min(i64::from(clip.bottom));

    for y in top..=bottom {
        rasterize_circle_pixel(
            target,
            depth,
            depth_w,
            x,
            y,
            z,
            primary,
            secondary,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
    }
}

fn update_line_clip(p: f32, q: f32, t0: &mut f32, t1: &mut f32) -> bool {
    if p == 0.0 {
        return q >= 0.0;
    }

    let t = q / p;
    if p < 0.0 {
        if t > *t1 {
            return false;
        }
        *t0 = (*t0).max(t);
    } else {
        if t < *t0 {
            return false;
        }
        *t1 = (*t1).min(t);
    }
    true
}

fn lerp_screen_point(p0: ScreenPoint, p1: ScreenPoint, t: f32) -> ScreenPoint {
    (
        p0.0 + (p1.0 - p0.0) * t,
        p0.1 + (p1.1 - p0.1) * t,
        p0.2 + (p1.2 - p0.2) * t,
    )
}

fn clip_screen_line_to_rect(
    p0: ScreenPoint,
    p1: ScreenPoint,
    clip: ClipRect,
) -> Option<(ScreenPoint, ScreenPoint)> {
    // Above one-pixel f32 spacing, clipping parameters can collapse the span.
    // Keep ordinary rounding and the original nonfinite-input path unchanged.
    if [p0.0, p0.1, p1.0, p1.1]
        .iter()
        .any(|value| value.abs() > (1 << 23) as f32)
        && [p0.0, p0.1, p0.2, p1.0, p1.1, p1.2]
            .iter()
            .all(|value| value.is_finite())
    {
        return clip_screen_line_to_rect_wide(p0, p1, clip);
    }

    let min_x = clip.left as f32;
    let max_x = clip.right as f32 + 1.0;
    let min_y = clip.top as f32;
    let max_y = clip.bottom as f32 + 1.0;
    let dx = p1.0 - p0.0;
    let dy = p1.1 - p0.1;
    let mut t0 = 0.0;
    let mut t1 = 1.0;

    if !update_line_clip(-dx, p0.0 - min_x, &mut t0, &mut t1)
        || !update_line_clip(dx, max_x - p0.0, &mut t0, &mut t1)
        || !update_line_clip(-dy, p0.1 - min_y, &mut t0, &mut t1)
        || !update_line_clip(dy, max_y - p0.1, &mut t0, &mut t1)
    {
        return None;
    }

    Some((lerp_screen_point(p0, p1, t0), lerp_screen_point(p0, p1, t1)))
}

fn clip_screen_line_to_rect_wide(
    p0: ScreenPoint,
    p1: ScreenPoint,
    clip: ClipRect,
) -> Option<(ScreenPoint, ScreenPoint)> {
    let origin = [f64::from(p0.0), f64::from(p0.1), f64::from(p0.2)];
    let end = [f64::from(p1.0), f64::from(p1.1), f64::from(p1.2)];
    let delta = [end[0] - origin[0], end[1] - origin[1], end[2] - origin[2]];
    let determinant = origin[0] * end[1] - end[0] * origin[1];
    let mut points = [origin, end];

    for (axis, boundary, minimum) in [
        (0, f64::from(clip.left), true),
        (0, f64::from(clip.right) + 1.0, false),
        (1, f64::from(clip.top), true),
        (1, f64::from(clip.bottom) + 1.0, false),
    ] {
        let inside = points.map(|point| {
            if minimum {
                point[axis] >= boundary
            } else {
                point[axis] <= boundary
            }
        });
        if !inside[0] && !inside[1] {
            return None;
        }
        if inside[0] == inside[1] {
            continue;
        }

        // Even f64 t/lerp can lose a viewport-sized interval on a long line.
        // Pin the crossed boundary; the original determinant retains the
        // other coordinate without subtracting two large interpolated values.
        let t = (boundary - origin[axis]) / delta[axis];
        let z = origin[2] + delta[2] * t;
        let intersection = if axis == 0 {
            [boundary, (delta[1] * boundary - determinant) / delta[0], z]
        } else {
            [(delta[0] * boundary + determinant) / delta[1], boundary, z]
        };
        points[usize::from(inside[0])] = intersection;
    }

    let [start, end] = points.map(|point| (point[0] as f32, point[1] as f32, point[2] as f32));
    Some((start, end))
}

#[inline]
fn line_depth(z: f32) -> f32 {
    if depth_in_clip_range(z) {
        (z - LINE_DEPTH_BIAS).max(-1.0)
    } else {
        f32::INFINITY
    }
}

// Select one pixel per major-axis cell on the original projected segment.
// Pixel centers and cell boundaries agree with triangle coverage.
pub fn rasterize_line(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    p0: (f32, f32, f32),
    p1: (f32, f32, f32),
    primary: u8,
    secondary: u8,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) {
    rasterize_border_line(
        target,
        depth,
        depth_w,
        p0,
        p1,
        primary,
        secondary,
        clip,
        dither_alpha,
        depth_test,
        depth_write,
        &[],
    );
}

pub(crate) fn rasterize_border_line(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    p0: (f32, f32, f32),
    p1: (f32, f32, f32),
    primary: u8,
    secondary: u8,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
    planes: &[ScreenPlane],
) {
    let Some((p0, p1)) = clip_screen_line_to_rect(p0, p1, clip) else {
        return;
    };

    let dx = p1.0 - p0.0;
    let dy = p1.1 - p0.1;
    let horizontal = dx.abs() >= dy.abs();
    let (a, b) = if horizontal {
        (p0.0, p1.0)
    } else {
        (p0.1, p1.1)
    };
    let first = a.min(b).floor() as i32;
    let last = a.max(b).floor() as i32;
    for coordinate in first..=last {
        let t = if a == b {
            0.0
        } else {
            ((coordinate as f32 + 0.5 - a) / (b - a)).clamp(0.0, 1.0)
        };
        let point = lerp_screen_point(p0, p1, t);
        let (x, y) = if horizontal {
            (coordinate, point.1.floor() as i32)
        } else {
            (point.0.floor() as i32, coordinate)
        };
        if clip.contains(x, y) {
            let col = dither_pick(primary as i32, secondary as i32, x, y);
            // For convex borders, the frontmost solid begins at the greatest
            // entry depth among its incident faces. Generic lines retain their
            // own interpolated depth; it never depends on previous pixels.
            let z = if planes.is_empty() {
                point.2
            } else {
                planes
                    .iter()
                    .map(|plane| plane.z_at(x as f32 + 0.5, y as f32 + 0.5))
                    .fold(f32::NEG_INFINITY, f32::max)
            };
            if !depth_in_clip_range(point.2) {
                continue;
            }
            write_pixel(
                target,
                depth,
                depth_w,
                x,
                y,
                line_depth(z),
                col,
                dither_alpha,
                depth_test,
                depth_write,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::mat4::Mat4;
    use crate::image::RcImage;

    fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[test]
    fn test_shading_quantizes_lambert_brightness() {
        let light = vec3(0.0, -1.0, 0.0);
        assert_eq!(face_shade_level(&light, Some(&vec3(1.0, 0.0, 0.0))), 0);
        assert_eq!(face_shade_level(&light, Some(&vec3(1.0, 1.0, 0.0))), 5);
        assert_eq!(face_shade_level(&light, Some(&vec3(0.0, 1.0, 0.0))), 7);
        assert_eq!(face_shade_level(&light, Some(&vec3(1.0, -0.25, 0.0))), 0);
        assert_eq!(face_shade_level(&light, Some(&vec3(0.0, -1.0, 0.0))), 0);
    }

    #[test]
    fn test_default_light_preserves_top_color_and_separates_side_levels() {
        let shading = Shading::new(&crate::settings::DEFAULT_COLORS);
        let shading = rc_ref!(&shading);
        let light = rc_ref!(&shading.direction);
        let top = vec3(0.0, 1.0, 0.0);

        assert_eq!(
            face_shade_level(&light, Some(&top)),
            crate::cube::shading::BASE_LEVEL
        );
        assert_eq!(lookup_ramp(&shading, 11, Some(&top)), (11, 11));
        assert_eq!(face_shade_level(&light, Some(&vec3(-1.0, 0.0, 0.0))), 3);
        assert_eq!(face_shade_level(&light, Some(&vec3(0.0, 0.0, 1.0))), 4);
    }

    #[test]
    fn test_shading_depends_on_direction_not_vector_length() {
        for normal in [vec3(1.0, 1.0, 0.0), vec3(0.0, -1.0, 0.0)] {
            let scaled = vec3(normal.x * 4.0, normal.y * 4.0, normal.z * 4.0);
            assert_eq!(
                face_shade_level(&vec3(0.0, -1.0, 0.0), Some(&normal)),
                face_shade_level(&vec3(0.0, -3.0, 0.0), Some(&scaled)),
            );
        }
    }

    #[test]
    fn test_shading_without_a_valid_normal_or_light_uses_darkest_level() {
        let light = vec3(0.0, -1.0, 0.0);
        let zero = vec3(0.0, 0.0, 0.0);
        assert_eq!(face_shade_level(&light, None), 0);
        assert_eq!(face_shade_level(&light, Some(&zero)), 0);
        assert_eq!(face_shade_level(&zero, Some(&light)), 0);
        assert_eq!(
            face_shade_level(&vec3(0.0, 0.0, f32::INFINITY), Some(&vec3(0.0, 0.0, -1.0))),
            0
        );
    }

    fn make_target_and_depth(w: u32, h: u32) -> (RcImage, Vec<f32>, ClipRect) {
        let img = Image::new(w, h);
        let depth = vec![f32::INFINITY; (w * h) as usize];
        let clip = ClipRect {
            left: 0,
            top: 0,
            right: w as i32 - 1,
            bottom: h as i32 - 1,
        };
        (img, depth, clip)
    }

    fn image_data(img: &RcImage) -> Vec<u8> {
        rc_ref!(img).canvas.data.clone()
    }

    #[test]
    fn test_camera_clip_range_applies_without_depth_testing() {
        for ortho_size in [None, Some(4.0)] {
            let camera = Camera::new();
            rc_mut!(&camera).near = 1.0;
            rc_mut!(&camera).far = 10.0;
            rc_mut!(&camera).ortho_size = ortho_size;
            let view = view_matrix(&rc_ref!(&camera));
            let vp = matmul(&projection_matrix(&rc_ref!(&camera), 16.0, 16.0), &view);
            let clip_row = camera_clip_row(&view);

            for depth_test in [false, true] {
                for (distance, expected) in [(0.5, 0), (1.0, 7), (2.0, 7), (10.0, 7), (20.0, 0)] {
                    let (image, mut depth, _) = make_target_and_depth(16, 16);
                    let (_, _, z) = world_to_screen(
                        &vec3(0.0, 0.0, -distance),
                        &vp,
                        &clip_row,
                        0.0,
                        0.0,
                        16.0,
                        16.0,
                    )
                    .unwrap();
                    write_pixel(
                        &mut rc_mut!(&image),
                        &mut depth,
                        16,
                        8,
                        8,
                        z,
                        7,
                        1.0,
                        depth_test,
                        true,
                    );
                    assert_eq!(rc_ref!(&image).canvas.data[8 * 16 + 8], expected);
                }
            }
        }
    }

    #[test]
    fn test_line_depth_bias_preserves_clip_range() {
        for end_x in [2.0, 4.0] {
            for (z, expected) in [
                (-1.0, 7),
                (1.0, 7),
                (-1.0 - LINE_DEPTH_BIAS * 0.5, 0),
                (1.0 + LINE_DEPTH_BIAS * 0.5, 0),
            ] {
                let (image, mut depth, clip) = make_target_and_depth(8, 8);
                rasterize_line(
                    &mut rc_mut!(&image),
                    &mut depth,
                    8,
                    (2.0, 2.0, z),
                    (end_x, 2.0, z),
                    7,
                    7,
                    clip,
                    1.0,
                    false,
                    true,
                );
                assert_eq!(rc_ref!(&image).canvas.data[2 * 8 + 2], expected);
            }
        }
    }

    #[test]
    fn test_triangle_clip_preserves_visible_portion() {
        for textured in [false, true] {
            let (image, mut depth, clip) = make_target_and_depth(4, 4);
            let points = [(0.0, 0.0, -2.0), (4.0, 0.0, 0.0), (0.0, 4.0, 0.0)];

            if textured {
                rasterize_textured_triangle(
                    &mut rc_mut!(&image),
                    &mut depth,
                    4,
                    points[0],
                    points[1],
                    points[2],
                    (0.0, 0.0),
                    (0.0, 0.0),
                    (0.0, 0.0),
                    |_, _, _, _| Some(7),
                    clip,
                    1.0,
                    false,
                    true,
                );
            } else {
                rasterize_triangle(
                    &mut rc_mut!(&image),
                    &mut depth,
                    4,
                    points[0],
                    points[1],
                    points[2],
                    7,
                    7,
                    clip,
                    1.0,
                    false,
                    true,
                );
            }

            let data = image_data(&image);
            assert_eq!(data[0], 0);
            assert_eq!(data[5], 7);
        }
    }

    #[test]
    fn test_matmul_identity() {
        let identity: Mat4x4 = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let result = matmul(&identity, &identity);
        assert_eq!(result, identity);
    }

    #[test]
    fn test_matmul_translation() {
        let translate: Mat4x4 = [
            [1.0, 0.0, 0.0, 2.0],
            [0.0, 1.0, 0.0, 3.0],
            [0.0, 0.0, 1.0, 4.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let scale: Mat4x4 = [
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        // translate * scale: scale applied first, then translation.
        let result = matmul(&translate, &scale);
        assert_eq!(result[0][0], 2.0);
        assert_eq!(result[0][3], 2.0);
        assert_eq!(result[1][3], 3.0);
        assert_eq!(result[2][3], 4.0);
    }

    #[test]
    fn test_view_matrix_identity_camera() {
        let camera = Camera::new();
        let v = view_matrix(&rc_ref!(&camera));
        assert_eq!(v, rc_ref!(&Mat4::identity()).data);
    }

    #[test]
    fn test_projection_matrix_perspective_aspect() {
        let camera = Camera::new();
        let p = projection_matrix(&rc_ref!(&camera), 256.0, 192.0);
        let f = 1.0 / (rc_ref!(&camera).fov.to_radians() * 0.5).tan();
        let aspect = 256.0 / 192.0;
        assert!((p[0][0] - f / aspect).abs() < 1e-4);
        assert!((p[1][1] - f).abs() < 1e-4);
        // Last row uses w = -z (standard perspective divide).
        assert_eq!(p[3][2], -1.0);
    }

    #[test]
    fn test_projection_matrix_orthographic() {
        let camera = Camera::new();
        rc_mut!(&camera).ortho_size = Some(10.0);
        let p = projection_matrix(&rc_ref!(&camera), 200.0, 100.0);
        let half_h = 5.0_f32;
        let half_w = half_h * 2.0;
        assert!((p[0][0] - 1.0 / half_w).abs() < 1e-6);
        assert!((p[1][1] - 1.0 / half_h).abs() < 1e-6);
        // Orthographic last row stays affine.
        assert_eq!(p[3][3], 1.0);
    }

    #[test]
    fn test_world_to_screen_center() {
        let camera = Camera::new();
        let v = view_matrix(&rc_ref!(&camera));
        let p = projection_matrix(&rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        let clip = camera_clip_row(&v);

        let result = world_to_screen(&vec3(0.0, 0.0, -2.0), &vp, &clip, 0.0, 0.0, 256.0, 192.0);
        let (sx, sy, _z) = result.expect("point in front of camera should project");
        assert!((sx - 128.0).abs() < 1e-3);
        assert!((sy - 96.0).abs() < 1e-3);
    }

    #[test]
    fn test_world_to_screen_behind_camera() {
        let camera = Camera::new();
        let v = view_matrix(&rc_ref!(&camera));
        let p = projection_matrix(&rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        let clip = camera_clip_row(&v);

        let result = world_to_screen(&vec3(0.0, 0.0, 5.0), &vp, &clip, 0.0, 0.0, 256.0, 192.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_face_shade_finite_magnitudes() {
        for normal_scale in [1e-30_f32, 1e-20, 1.0, 1e10, 1e20, 1e30] {
            for light_scale in [1e-30_f32, 1e-20, 1.0, 1e10, 1e20, 1e30] {
                assert_eq!(
                    face_shade_level(
                        &vec3(0.0, 0.0, -light_scale),
                        Some(&vec3(0.0, 0.0, normal_scale))
                    ),
                    LEVEL_COUNT - 1,
                    "normal={normal_scale} light={light_scale}"
                );
            }
        }
    }

    #[test]
    fn test_tri_normal_finite_range_orientation() {
        let origin = vec3(0.0, 0.0, 0.0);
        for scale in [1e-30_f32, 1e-20, 1.0, 1e10, 1e20, 1e30] {
            let x = vec3(scale, 0.0, 0.0);
            let y = vec3(0.0, scale, 0.0);
            for (p1, p2, sign) in [(x, y, 1.0), (y, x, -1.0)] {
                let n = tri_normal(&origin, &p1, &p2);
                assert!(
                    n.is_finite() && n.x == 0.0 && n.y == 0.0 && n.z * sign > 0.0,
                    "scale={scale} normal={n:?}"
                );
                assert_eq!(
                    face_shade_level(&vec3(0.0, 0.0, -sign), Some(&n)),
                    LEVEL_COUNT - 1
                );
            }
        }

        // Finite endpoints can have an edge difference beyond f32's range.
        let n = tri_normal(
            &vec3(-2e38, 0.0, 0.0),
            &vec3(2e38, 0.0, 0.0),
            &vec3(0.0, 1e38, 0.0),
        );
        assert!(n.is_finite() && n.x == 0.0 && n.y == 0.0 && n.z > 0.0);

        assert_eq!(
            tri_normal(&origin, &vec3(1.0, 0.0, 0.0), &vec3(2.0, 0.0, 0.0)),
            origin
        );
    }

    #[test]
    fn test_tri_normal_ccw() {
        // CCW triangle in the XY plane has normal pointing +Z.
        let n = tri_normal(
            &vec3(0.0, 0.0, 0.0),
            &vec3(1.0, 0.0, 0.0),
            &vec3(0.0, 1.0, 0.0),
        );
        assert_eq!(n.x, 0.0);
        assert_eq!(n.y, 0.0);
        assert_eq!(n.z, 1.0);
    }

    #[test]
    fn test_tri_normal_cw() {
        // CW triangle in the XY plane has normal pointing -Z.
        let n = tri_normal(
            &vec3(0.0, 0.0, 0.0),
            &vec3(0.0, 1.0, 0.0),
            &vec3(1.0, 0.0, 0.0),
        );
        assert_eq!(n.x, 0.0);
        assert_eq!(n.y, 0.0);
        assert_eq!(n.z, -1.0);
    }

    #[test]
    fn test_camera_right_up_default() {
        let camera = Camera::new();
        let (right, up) = camera_right_up(&rc_ref!(&camera));
        assert_eq!(right.x, 1.0);
        assert_eq!(right.y, 0.0);
        assert_eq!(right.z, 0.0);
        assert_eq!(up.x, 0.0);
        assert_eq!(up.y, 1.0);
        assert_eq!(up.z, 0.0);
    }

    #[test]
    fn test_screen_circle_centered() {
        let camera = Camera::new();
        let v = view_matrix(&rc_ref!(&camera));
        let p = projection_matrix(&rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        let clip = camera_clip_row(&v);

        let result = screen_circle(
            &vec3(0.0, 0.0, -2.0),
            0.5,
            &vp,
            &clip,
            &rc_ref!(&camera),
            0.0,
            0.0,
            256.0,
            192.0,
        );
        let (sx, sy, sr, _z) = result.expect("circle in view should project");
        assert!((sx - 128.0).abs() < 1e-3);
        assert!((sy - 96.0).abs() < 1e-3);
        // r = 0.5 at z = 2 under fov 60 / aspect 4:3 spans
        // 0.25 · cot(30°) / (4/3) in NDC-x = 24√3 px of the 256-wide viewport.
        assert!((sr - 24.0 * 3.0_f32.sqrt()).abs() < 1e-3);
    }

    #[test]
    fn test_screen_circle_behind_camera() {
        let camera = Camera::new();
        let v = view_matrix(&rc_ref!(&camera));
        let p = projection_matrix(&rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        let clip = camera_clip_row(&v);

        let result = screen_circle(
            &vec3(0.0, 0.0, 5.0),
            0.5,
            &vp,
            &clip,
            &rc_ref!(&camera),
            0.0,
            0.0,
            256.0,
            192.0,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_clip_rect_contains() {
        let clip = ClipRect {
            left: 10,
            top: 20,
            right: 30,
            bottom: 40,
        };

        assert!(clip.contains(10, 20));
        assert!(clip.contains(30, 40));
        assert!(clip.contains(20, 30));
        assert!(!clip.contains(9, 20));
        assert!(!clip.contains(31, 20));
        assert!(!clip.contains(20, 19));
        assert!(!clip.contains(20, 41));
    }

    #[test]
    fn test_compute_clip_rect_clamps_to_target() {
        let clip = compute_clip_rect(0.0, 0.0, 300.0, 200.0, 256, 192);
        assert_eq!(clip.left, 0);
        assert_eq!(clip.top, 0);
        assert_eq!(clip.right, 255);
        assert_eq!(clip.bottom, 191);
    }

    #[test]
    fn test_compute_clip_rect_offset_viewport() {
        let clip = compute_clip_rect(64.0, 0.0, 64.0, 48.0, 256, 192);
        assert_eq!(clip.left, 64);
        assert_eq!(clip.top, 0);
        assert_eq!(clip.right, 127);
        assert_eq!(clip.bottom, 47);
    }

    #[test]
    fn test_compute_clip_rect_negative_origin_clamped() {
        let clip = compute_clip_rect(-10.0, -5.0, 100.0, 100.0, 256, 192);
        assert_eq!(clip.left, 0);
        assert_eq!(clip.top, 0);
    }

    #[test]
    fn test_compute_clip_rect_extreme_negative_origin_is_empty() {
        let clip = compute_clip_rect(i32::MIN as f32, i32::MIN as f32, 1.0, 1.0, 8, 8);
        assert!(clip.right < clip.left);
        assert!(clip.bottom < clip.top);
    }

    #[test]
    fn test_compute_clip_rect_target_extents() {
        let empty = compute_clip_rect(0.0, 0.0, 8.0, 8.0, 0, 0);
        assert_eq!(empty.right, -1);
        assert_eq!(empty.bottom, -1);

        let wide = compute_clip_rect(0.0, 0.0, 8.0, 8.0, 1 << 31, 0);
        assert_eq!(wide.right, 7);
        assert_eq!(wide.bottom, -1);
    }

    #[test]
    fn test_write_pixel_writes_at_coord() {
        let (img, mut depth, _) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        write_pixel(&mut img_mut, &mut depth, 8, 3, 3, 0.0, 5, 1.0, true, true);
        assert_eq!(img_mut.canvas.read_data(3, 3), 5);
        assert_eq!(depth[3 * 8 + 3], 0.0);
    }

    #[test]
    fn test_rasterize_line_horizontal() {
        let (img, mut depth, clip) = make_target_and_depth(32, 32);
        let mut img_mut = rc_mut!(&img);
        rasterize_line(
            &mut img_mut,
            &mut depth,
            32,
            (5.0, 10.0, 0.0),
            (10.0, 10.0, 0.0),
            7,
            7,
            clip,
            1.0,
            true,
            true,
        );
        for x in 5..=10 {
            assert_eq!(img_mut.canvas.read_data(x, 10), 7);
        }
    }

    #[test]
    fn test_rasterize_line_vertical() {
        let (img, mut depth, clip) = make_target_and_depth(32, 32);
        let mut img_mut = rc_mut!(&img);
        rasterize_line(
            &mut img_mut,
            &mut depth,
            32,
            (5.0, 5.0, 0.0),
            (5.0, 12.0, 0.0),
            8,
            8,
            clip,
            1.0,
            true,
            true,
        );
        for y in 5..=12 {
            assert_eq!(img_mut.canvas.read_data(5, y), 8);
        }
    }

    #[test]
    fn test_rasterize_line_single_point() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        rasterize_line(
            &mut img_mut,
            &mut depth,
            8,
            (3.0, 3.0, 0.25),
            (3.0, 3.0, 0.25),
            5,
            5,
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(img_mut.canvas.read_data(3, 3), 5);
        assert_eq!(depth[3 * 8 + 3], line_depth(0.25));
    }

    #[test]
    fn test_rasterize_line_depth_z_test() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        rasterize_line(
            &mut img_mut,
            &mut depth,
            8,
            (0.0, 0.0, 0.0),
            (4.0, 0.0, 0.0),
            10,
            10,
            clip,
            1.0,
            true,
            true,
        );

        rasterize_line(
            &mut img_mut,
            &mut depth,
            8,
            (0.0, 0.0, 0.5),
            (4.0, 0.0, 0.5),
            11,
            11,
            clip,
            1.0,
            true,
            true,
        );
        for x in 0..=4 {
            assert_eq!(img_mut.canvas.read_data(x, 0), 10);
        }
    }

    fn draw_flat_square_surface(img_mut: &mut Image, depth: &mut [f32], clip: ClipRect) {
        rasterize_triangle(
            img_mut,
            depth,
            8,
            (0.0, 0.0, 0.5),
            (7.0, 0.0, 0.5),
            (0.0, 7.0, 0.5),
            3,
            3,
            clip,
            1.0,
            true,
            true,
        );
        rasterize_triangle(
            img_mut,
            depth,
            8,
            (7.0, 0.0, 0.5),
            (7.0, 7.0, 0.5),
            (0.0, 7.0, 0.5),
            3,
            3,
            clip,
            1.0,
            true,
            true,
        );
    }

    #[test]
    fn test_rasterize_line_equal_depth_over_surface_is_visible() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        draw_flat_square_surface(&mut img_mut, &mut depth, clip);

        rasterize_line(
            &mut img_mut,
            &mut depth,
            8,
            (2.0, 2.0, 0.5),
            (5.0, 2.0, 0.5),
            11,
            11,
            clip,
            1.0,
            true,
            true,
        );
        for x in 2..=5 {
            assert_eq!(img_mut.canvas.read_data(x, 2), 11);
        }
    }

    #[test]
    fn test_geometric_border_on_sloped_surface() {
        for textured in [false, true] {
            for line_first in [false, true] {
                for gap in [-0.01, 0.0, 0.01] {
                    for (origin_z, dz_dx, dz_dy) in [(0.35, 0.04, 0.02), (0.65, -0.04, -0.02)] {
                        let point = |x: f32, y: f32| (x, y, origin_z + dz_dx * x + dz_dy * y + gap);
                        let planes = [ScreenPlane::from_triangle(
                            point(0.0, 0.0),
                            point(8.0, 0.0),
                            point(0.0, 8.0),
                        )
                        .unwrap()];
                        let (image, mut depth, clip) = make_target_and_depth(8, 8);
                        let mut target = rc_mut!(&image);
                        for draw_line in [line_first, !line_first] {
                            if draw_line {
                                rasterize_border_line(
                                    &mut target,
                                    &mut depth,
                                    8,
                                    point(2.25, 2.25),
                                    point(5.25, 4.25),
                                    7,
                                    7,
                                    clip,
                                    1.0,
                                    true,
                                    true,
                                    &planes,
                                );
                            } else {
                                draw_oracle_plane(
                                    &mut target,
                                    &mut depth,
                                    clip,
                                    origin_z,
                                    dz_dx,
                                    dz_dy,
                                    textured,
                                    3,
                                );
                            }
                        }

                        if gap > 0.0 {
                            // The whole line is behind the same covering plane.
                            // A 0.01 separation is much larger than f32 roundoff.
                            assert!(
                            target.canvas.data.iter().all(|&color| color != 7),
                            "textured={textured} line_first={line_first} slope={dz_dx} hidden line"
                        );
                        } else {
                            // The segment spans strict interior columns 3 and 4.
                            // The line crosses this corridor regardless of rounding.
                            for x in 3..=4 {
                                assert!(
                                (1..=6).any(|y| target.canvas.read_data(x, y) == 7),
                                "textured={textured} line_first={line_first} gap={gap} slope={dz_dx} visible column={x}"
                            );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_rasterize_line_behind_surface_stays_hidden() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        draw_flat_square_surface(&mut img_mut, &mut depth, clip);

        rasterize_line(
            &mut img_mut,
            &mut depth,
            8,
            (2.0, 2.0, 0.51),
            (5.0, 2.0, 0.51),
            11,
            11,
            clip,
            1.0,
            true,
            true,
        );
        for x in 2..=5 {
            assert_eq!(img_mut.canvas.read_data(x, 2), 3);
        }
    }

    #[test]
    fn test_rasterize_line_and_crossing_surfaces_have_stable_depth_order() {
        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let (image, mut depth, clip) = make_target_and_depth(4, 4);
            let mut target = rc_mut!(&image);
            for primitive in order {
                if primitive == 2 {
                    rasterize_line(
                        &mut target,
                        &mut depth,
                        4,
                        (2.0, 2.0, 0.42),
                        (2.0, 3.0, 0.42),
                        7,
                        7,
                        clip,
                        1.0,
                        true,
                        true,
                    );
                    continue;
                }
                let point = |x: f32, y: f32| {
                    (
                        x,
                        y,
                        if primitive == 0 {
                            0.5 + 0.2 * (x - 2.5)
                        } else {
                            0.45
                        },
                    )
                };
                for points in [
                    [point(0.0, 0.0), point(4.0, 0.0), point(0.0, 4.0)],
                    [point(4.0, 0.0), point(4.0, 4.0), point(0.0, 4.0)],
                ] {
                    rasterize_triangle(
                        &mut target,
                        &mut depth,
                        4,
                        points[0],
                        points[1],
                        points[2],
                        3 + primitive,
                        3 + primitive,
                        clip,
                        1.0,
                        true,
                        true,
                    );
                }
            }
            assert_eq!(target.canvas.read_data(2, 2), 7, "order={order:?}");
        }
    }

    #[test]
    fn test_rasterize_line_clip_rejects_outside() {
        let (img, mut depth, _) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        let small_clip = ClipRect {
            left: 0,
            top: 0,
            right: 3,
            bottom: 3,
        };

        rasterize_line(
            &mut img_mut,
            &mut depth,
            8,
            (0.0, 0.0, 0.0),
            (7.0, 0.0, 0.0),
            5,
            5,
            small_clip,
            1.0,
            true,
            true,
        );
        for x in 0..=3 {
            assert_eq!(img_mut.canvas.read_data(x, 0), 5);
        }
        for x in 4..=7 {
            assert_eq!(img_mut.canvas.read_data(x, 0), 0);
        }
    }

    #[test]
    fn test_circle_scan_ranges_bound_enclosing_and_partial_circles() {
        let clip = ClipRect {
            left: 0,
            top: 0,
            right: 15,
            bottom: 15,
        };

        assert_eq!(circle_scan_ranges(8, 8, 100_000, clip), [(0, 8), (1, 0)]);
        assert_eq!(
            circle_scan_ranges(-99_995, 8, 100_000, clip),
            [(0, 8), (99_995, 100_000)]
        );
        assert!(circle_scan_ranges(-100_100, -100_100, 100_000, clip)
            .into_iter()
            .all(|(start, end)| start > end));
    }

    #[test]
    fn test_circle_huge_tangent_and_enclosing_pixels() {
        for border in [false, true] {
            for cx in [-2_000_000_000.0, 0.0, 2_000_000_000.0] {
                for alpha in [0.5, 1.0] {
                    for depth_write in [false, true] {
                        let (img, mut depth, clip) = make_target_and_depth(16, 16);
                        let draw = if border {
                            rasterize_circle_border
                        } else {
                            rasterize_circle_filled
                        };
                        draw(
                            &mut rc_mut!(&img),
                            &mut depth,
                            16,
                            cx,
                            0.0,
                            2_000_000_000.0,
                            0.25,
                            7,
                            3,
                            clip,
                            alpha,
                            true,
                            depth_write,
                        );

                        for y in 0..16 {
                            for x in 0..16 {
                                // At this radius the rounded tangent is the x=0 column;
                                // the opposite boundary lies beyond the viewport.
                                let covered = if border {
                                    cx != 0.0 && x == 0
                                } else {
                                    cx >= 0.0 || x == 0
                                };
                                let drawn = covered && (alpha == 1.0 || (x + y) % 2 != 0);
                                let color = if drawn {
                                    if (x + y) % 2 == 0 {
                                        7
                                    } else {
                                        3
                                    }
                                } else {
                                    0
                                };

                                assert_eq!(rc_ref!(&img).canvas.read_data(x, y), color);
                                assert_eq!(
                                    depth[y * 16 + x],
                                    if drawn && depth_write {
                                        0.25
                                    } else {
                                        f32::INFINITY
                                    }
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_circle_rejects_constant_depth_outside_camera_range() {
        for border in [false, true] {
            for z in [-2.0, 2.0, f32::NEG_INFINITY, f32::INFINITY, f32::NAN] {
                let (img, mut depth, clip) = make_target_and_depth(16, 16);
                let draw = if border {
                    rasterize_circle_border
                } else {
                    rasterize_circle_filled
                };
                draw(
                    &mut rc_mut!(&img),
                    &mut depth,
                    16,
                    8.0,
                    8.0,
                    1000.0,
                    z,
                    7,
                    3,
                    clip,
                    0.5,
                    false,
                    true,
                );
                assert_eq!(image_data(&img), vec![0; 256]);
                assert_eq!(depth, vec![f32::INFINITY; 256]);
            }
        }
    }

    #[test]
    fn test_circle_clip_keeps_tangent_pixels_and_rejects_disjoint_bounds() {
        for border in [false, true] {
            for (cx, cy, px, py) in [
                (-2.0, 3.0, 0, 3),
                (9.0, 3.0, 7, 3),
                (3.0, -2.0, 3, 0),
                (3.0, 9.0, 3, 7),
            ] {
                for outside in [false, true] {
                    let (img, mut depth, clip) = make_target_and_depth(8, 8);
                    let radius = if outside { 1.0 } else { 2.0 };
                    let draw = if border {
                        rasterize_circle_border
                    } else {
                        rasterize_circle_filled
                    };
                    draw(
                        &mut rc_mut!(&img),
                        &mut depth,
                        8,
                        cx,
                        cy,
                        radius,
                        0.0,
                        7,
                        7,
                        clip,
                        1.0,
                        true,
                        true,
                    );
                    assert_eq!(
                        rc_ref!(&img).canvas.read_data(px, py),
                        if outside { 0 } else { 7 }
                    );
                    if outside {
                        assert_eq!(image_data(&img), vec![0; 64]);
                        assert_eq!(depth, vec![f32::INFINITY; 64]);
                    }
                }
            }
        }
    }

    #[test]
    fn test_clip_screen_line_to_rect_limits_huge_span() {
        let clip = ClipRect {
            left: 0,
            top: 0,
            right: 31,
            bottom: 31,
        };

        let (p0, p1) =
            clip_screen_line_to_rect((-1_000_000.0, 16.0, 0.0), (1_000_000.0, 16.0, 1.0), clip)
                .unwrap();
        assert!(p0.0 >= 0.0);
        // Pixel 31 occupies the continuous interval [31, 32).
        assert!(p1.0 <= 32.0);
        assert_eq!(p0.1, 16.0);
        assert_eq!(p1.1, 16.0);
        assert_eq!(p0.2, 0.5);
        assert!((p1.2 - 0.500016).abs() < 1e-6);
    }

    #[test]
    fn test_clip_screen_line_to_rect_retains_wide_crossings_and_depth() {
        let clip = ClipRect {
            left: 0,
            top: 0,
            right: 7,
            bottom: 7,
        };
        for scale in [1e8_f32, 1e20, 2e38] {
            for diagonal in [false, true] {
                for transpose in [false, true] {
                    let point = |x, y, z| if transpose { (y, x, z) } else { (x, y, z) };
                    let ends = if diagonal {
                        [point(-scale, -scale, 0.0), point(scale, scale, 1.0)]
                    } else {
                        [point(-scale, 4.0, 0.0), point(scale, 4.0, 1.0)]
                    };
                    let y0 = if diagonal { 0.0 } else { 4.0 };
                    let y1 = if diagonal { 8.0 } else { 4.0 };
                    let expected = [
                        point(0.0, y0, 0.5),
                        point(8.0, y1, (0.5 + 4.0 / f64::from(scale)) as f32),
                    ];

                    for reverse in [false, true] {
                        let (input, expected) = if reverse {
                            ([ends[1], ends[0]], [expected[1], expected[0]])
                        } else {
                            (ends, expected)
                        };
                        let (start, end) =
                            clip_screen_line_to_rect(input[0], input[1], clip).unwrap();
                        for (actual, expected) in [start, end].into_iter().zip(expected) {
                            assert_eq!((actual.0, actual.1), (expected.0, expected.1));
                            // Midpoint depth and the small viewport offset differ by at most two f32 ULPs.
                            assert!((actual.2 - expected.2).abs() <= 1e-7);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_clip_screen_line_to_rect_wide_tangent_and_disjoint() {
        let clip = ClipRect {
            left: 0,
            top: 0,
            right: 7,
            bottom: 7,
        };
        let scale = 1e20_f32;
        let tangent =
            clip_screen_line_to_rect((-scale, scale, 0.0), (scale, -scale, 1.0), clip).unwrap();
        assert_eq!(tangent, ((0.0, 0.0, 0.5), (0.0, 0.0, 0.5)));

        assert!(clip_screen_line_to_rect((-scale, -4.0, 0.0), (scale, -4.0, 1.0), clip).is_none());
        assert!(clip_screen_line_to_rect((-4.0, 0.0, 0.0), (scale, -scale, 1.0), clip).is_none());
    }

    #[test]
    fn test_clip_screen_line_to_rect_wide_keeps_near_endpoint() {
        let clip = ClipRect {
            left: 0,
            top: 0,
            right: 7,
            bottom: 7,
        };
        let scale = 1e20_f32;
        let (start, end) =
            clip_screen_line_to_rect((0.0, 4.0, 0.0), (scale, scale, 1.0), clip).unwrap();
        assert_eq!(start, (0.0, 4.0, 0.0));
        assert_eq!((end.0, end.1), (4.0, 8.0));
        // The y=8 boundary is reached after four units of the stored endpoint span.
        let expected_depth = 4.0 / (f64::from(scale) - 4.0);
        assert!((f64::from(end.2) / expected_depth - 1.0).abs() < 2e-6);
    }

    #[test]
    fn test_clip_screen_line_to_rect_rejects_fully_outside_line() {
        let clip = ClipRect {
            left: 0,
            top: 0,
            right: 31,
            bottom: 31,
        };
        assert!(clip_screen_line_to_rect((-10.0, -5.0, 0.0), (-1.0, -5.0, 1.0), clip).is_none());
    }

    #[test]
    fn test_rasterize_line_samples_projected_pixel_cells() {
        for transpose in [false, true] {
            for reverse in [false, true] {
                let point = |x, y, z| if transpose { (y, x, z) } else { (x, y, z) };
                let (image, mut depth, clip) = make_target_and_depth(8, 8);
                let mut ends = [point(2.75, 2.75, 0.1), point(5.75, 4.75, 0.4)];
                if reverse {
                    ends.reverse();
                }
                rasterize_line(
                    &mut rc_mut!(&image),
                    &mut depth,
                    8,
                    ends[0],
                    ends[1],
                    7,
                    7,
                    clip,
                    1.0,
                    true,
                    true,
                );

                // The segment crosses columns 2..5. At their centers (clamped
                // to the segment) its y is 2.75, 3.25, 3.9167, and 4.5833.
                let cells = [(2, 2, 0.1), (3, 3, 0.175), (4, 3, 0.275), (5, 4, 0.375)];
                let mut expected = vec![0; 64];
                for (x, y, z) in cells {
                    let (x, y) = if transpose { (y, x) } else { (x, y) };
                    expected[y * 8 + x] = 7;
                    // Unit-scale interpolation differs by at most a few f32 ULPs.
                    assert!((depth[y * 8 + x] - line_depth(z)).abs() < 1e-6);
                }
                assert_eq!(image_data(&image), expected);
            }
        }
    }

    #[test]
    fn test_rasterize_line_keeps_the_last_viewport_cell() {
        let (image, mut depth, clip) = make_target_and_depth(8, 8);
        rasterize_line(
            &mut rc_mut!(&image),
            &mut depth,
            8,
            (7.25, 2.25, 0.5),
            (7.75, 2.25, 0.5),
            7,
            7,
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(rc_ref!(&image).canvas.read_data(7, 2), 7);
        assert!(image_data(&image)
            .iter()
            .enumerate()
            .all(|(i, &color)| color == if i == 2 * 8 + 7 { 7 } else { 0 }));
    }

    #[test]
    fn test_rasterize_triangle_z_interpolated() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let mut img_mut = rc_mut!(&img);
        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            16,
            (0.0, 0.0, 0.0),
            (15.0, 0.0, 0.5),
            (0.0, 15.0, 1.0),
            7,
            7,
            clip,
            1.0,
            true,
            true,
        );
        // z = x / 30 + y / 15, sampled at pixel centers.
        assert!((depth[0] - 0.05).abs() < 1e-6);
        assert!((depth[4 * 16 + 4] - 0.45).abs() < 1e-6);
    }

    #[test]
    fn test_rasterize_triangle_back_face_also_drawn() {
        // CW winding still rasterizes — culling happens upstream in draw::prim.
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let mut img_mut = rc_mut!(&img);
        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            16,
            (2.0, 2.0, 0.0),
            (2.0, 12.0, 0.0),
            (12.0, 2.0, 0.0),
            14,
            14,
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(img_mut.canvas.read_data(4, 4), 14);
    }

    fn draw_split_square(reverse_order: bool) -> Vec<u8> {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        let tri_a = |img_mut: &mut Image, depth: &mut [f32]| {
            rasterize_triangle(
                img_mut,
                depth,
                8,
                (1.0, 1.0, 0.0),
                (5.0, 1.0, 0.0),
                (1.0, 5.0, 0.0),
                3,
                3,
                clip,
                1.0,
                false,
                false,
            );
        };

        let tri_b = |img_mut: &mut Image, depth: &mut [f32]| {
            rasterize_triangle(
                img_mut,
                depth,
                8,
                (5.0, 1.0, 0.0),
                (5.0, 5.0, 0.0),
                (1.0, 5.0, 0.0),
                7,
                7,
                clip,
                1.0,
                false,
                false,
            );
        };

        if reverse_order {
            tri_b(&mut img_mut, &mut depth);
            tri_a(&mut img_mut, &mut depth);
        } else {
            tri_a(&mut img_mut, &mut depth);
            tri_b(&mut img_mut, &mut depth);
        }
        drop(img_mut);
        image_data(&img)
    }

    #[test]
    fn test_rasterize_triangle_shared_edge_is_order_independent() {
        assert_eq!(draw_split_square(false), draw_split_square(true));
    }

    #[test]
    fn test_rasterize_triangle_split_square_has_no_shared_edge_holes() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            8,
            (1.0, 1.0, 0.0),
            (5.0, 5.0, 0.0),
            (5.0, 1.0, 0.0),
            4,
            4,
            clip,
            1.0,
            false,
            false,
        );
        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            8,
            (1.0, 1.0, 0.0),
            (1.0, 5.0, 0.0),
            (5.0, 5.0, 0.0),
            4,
            4,
            clip,
            1.0,
            false,
            false,
        );

        for y in 1..5 {
            for x in 1..5 {
                assert_eq!(img_mut.canvas.read_data(x, y), 4, "pixel=({x}, {y})");
            }
        }
    }

    #[test]
    fn test_rasterize_triangle_split_square_depth_test_has_no_shared_edge_holes() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            8,
            (1.0, 1.0, 0.25),
            (5.0, 5.0, 0.25),
            (5.0, 1.0, 0.25),
            4,
            4,
            clip,
            1.0,
            true,
            true,
        );
        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            8,
            (1.0, 1.0, 0.25),
            (1.0, 5.0, 0.25),
            (5.0, 5.0, 0.25),
            4,
            4,
            clip,
            1.0,
            true,
            true,
        );

        for y in 1..5 {
            for x in 1..5 {
                assert_eq!(img_mut.canvas.read_data(x, y), 4, "pixel=({x}, {y})");
                assert_eq!(depth[y * 8 + x], 0.25, "pixel=({x}, {y})");
            }
        }
    }

    #[test]
    fn test_rasterize_triangle_large_split_square_has_no_shared_edge_holes() {
        const SIZE: u32 = 258;
        let (img, mut depth, clip) = make_target_and_depth(SIZE, SIZE);
        let mut img_mut = rc_mut!(&img);
        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            SIZE,
            (1.0, 1.0, 0.25),
            (257.0, 257.0, 0.25),
            (257.0, 1.0, 0.25),
            4,
            4,
            clip,
            1.0,
            true,
            true,
        );
        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            SIZE,
            (1.0, 1.0, 0.25),
            (1.0, 257.0, 0.25),
            (257.0, 257.0, 0.25),
            4,
            4,
            clip,
            1.0,
            true,
            true,
        );

        for y in 1..257 {
            for x in 1..257 {
                assert_eq!(img_mut.canvas.read_data(x, y), 4, "pixel=({x}, {y})");
            }
        }
    }

    #[test]
    fn test_rasterize_triangle_degenerate_skipped() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let mut img_mut = rc_mut!(&img);
        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            8,
            (0.0, 0.0, 0.0),
            (4.0, 0.0, 0.0),
            (8.0, 0.0, 0.0),
            5,
            5,
            clip,
            1.0,
            true,
            true,
        );
        for x in 0..8 {
            assert_eq!(img_mut.canvas.read_data(x, 0), 0);
        }
    }

    #[test]
    fn test_rasterize_triangle_clip_rejects_outside() {
        let (img, mut depth, _) = make_target_and_depth(16, 16);
        let mut img_mut = rc_mut!(&img);
        let small_clip = ClipRect {
            left: 0,
            top: 0,
            right: 7,
            bottom: 7,
        };

        rasterize_triangle(
            &mut img_mut,
            &mut depth,
            16,
            (0.0, 0.0, 0.0),
            (15.0, 0.0, 0.0),
            (0.0, 15.0, 0.0),
            6,
            6,
            small_clip,
            1.0,
            true,
            true,
        );
        assert_eq!(img_mut.canvas.read_data(2, 2), 6);
        assert_eq!(img_mut.canvas.read_data(10, 1), 0);
    }

    #[test]
    fn test_rasterize_textured_triangle_transparency_skips() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let mut img_mut = rc_mut!(&img);
        rasterize_textured_triangle(
            &mut img_mut,
            &mut depth,
            16,
            (2.0, 2.0, 0.0),
            (12.0, 2.0, 0.0),
            (2.0, 12.0, 0.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            |_, _, _, _| None,
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(img_mut.canvas.read_data(4, 4), 0);
        assert_eq!(depth[4 * 16 + 4], f32::INFINITY);
    }

    #[test]
    fn test_rasterize_textured_triangle_depth_rejection_skips_sampler() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        depth.fill(-1.0);
        let sample_count = std::cell::Cell::new(0);
        rasterize_textured_triangle(
            &mut rc_mut!(&img),
            &mut depth,
            16,
            (2.0, 2.0, 0.0),
            (12.0, 2.0, 0.0),
            (2.0, 12.0, 0.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            |_, _, _, _| {
                sample_count.set(sample_count.get() + 1);
                Some(7)
            },
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(sample_count.get(), 0);
    }

    #[test]
    fn test_rasterize_textured_triangle_zero_alpha_skips_sampler() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let sample_count = std::cell::Cell::new(0);
        rasterize_textured_triangle(
            &mut rc_mut!(&img),
            &mut depth,
            16,
            (2.0, 2.0, 0.0),
            (12.0, 2.0, 0.0),
            (2.0, 12.0, 0.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            |_, _, _, _| {
                sample_count.set(sample_count.get() + 1);
                Some(7)
            },
            clip,
            0.0,
            true,
            true,
        );
        assert_eq!(sample_count.get(), 0);
    }

    #[test]
    fn test_rasterize_circle_filled_matches_2d_circle_pixels() {
        let expected = Image::new(32, 32);
        rc_mut!(&expected).draw_circle(16.0, 16.0, 6.0, 7);

        let (actual, mut depth, clip) = make_target_and_depth(32, 32);
        rasterize_circle_filled(
            &mut rc_mut!(&actual),
            &mut depth,
            32,
            16.0,
            16.0,
            6.0,
            0.0,
            7,
            7,
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(image_data(&actual), image_data(&expected));
    }

    #[test]
    fn test_rasterize_circle_border_matches_2d_circb_pixels() {
        let expected = Image::new(32, 32);
        rc_mut!(&expected).draw_circle_border(16.0, 16.0, 6.0, 8);

        let (actual, mut depth, clip) = make_target_and_depth(32, 32);
        rasterize_circle_border(
            &mut rc_mut!(&actual),
            &mut depth,
            32,
            16.0,
            16.0,
            6.0,
            0.0,
            8,
            8,
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(image_data(&actual), image_data(&expected));
    }

    #[test]
    fn test_rasterize_circle_filled_clip_rejects_outside() {
        let (img, mut depth, clip) = make_target_and_depth(32, 32);
        let mut img_mut = rc_mut!(&img);
        let small_clip = ClipRect {
            left: 0,
            top: 0,
            right: 7,
            bottom: 7,
        };

        for (clip, outside_color) in [(small_clip, 0), (clip, 5)] {
            rasterize_circle_filled(
                &mut img_mut,
                &mut depth,
                32,
                3.0,
                3.0,
                10.0,
                0.0,
                5,
                5,
                clip,
                1.0,
                true,
                true,
            );
            assert_eq!(img_mut.canvas.read_data(3, 3), 5);
            assert_eq!(img_mut.canvas.read_data(10, 3), outside_color);
        }
    }

    fn draw_oracle_plane(
        target: &mut Image,
        depth: &mut [f32],
        clip: ClipRect,
        origin_z: f32,
        dz_dx: f32,
        dz_dy: f32,
        textured: bool,
        color: u8,
    ) {
        let point = |x: f32, y: f32| (x, y, origin_z + dz_dx * x + dz_dy * y);
        for points in [
            [point(0.0, 0.0), point(8.0, 0.0), point(0.0, 8.0)],
            [point(8.0, 0.0), point(8.0, 8.0), point(0.0, 8.0)],
        ] {
            if textured {
                rasterize_textured_triangle(
                    target,
                    depth,
                    8,
                    points[0],
                    points[1],
                    points[2],
                    (0.0, 0.0),
                    (0.0, 0.0),
                    (0.0, 0.0),
                    |_, _, _, _| Some(i32::from(color)),
                    clip,
                    1.0,
                    true,
                    true,
                );
            } else {
                rasterize_triangle(
                    target, depth, 8, points[0], points[1], points[2], color, color, clip, 1.0,
                    true, true,
                );
            }
        }
    }

    #[test]
    fn test_sloped_surface_keeps_geometric_depth_order() {
        for textured in [false, true] {
            for sloped_first in [false, true] {
                let (image, mut depth, clip) = make_target_and_depth(8, 8);
                let mut target = rc_mut!(&image);
                for sloped in [sloped_first, !sloped_first] {
                    if sloped {
                        draw_oracle_plane(
                            &mut target,
                            &mut depth,
                            clip,
                            0.4,
                            0.04,
                            0.0,
                            textured,
                            3,
                        );
                    } else {
                        draw_oracle_plane(
                            &mut target,
                            &mut depth,
                            clip,
                            0.5,
                            0.0,
                            0.0,
                            textured,
                            4,
                        );
                    }
                }

                // At pixel centers x=1.5 and x=3.5 the sloped plane has depths
                // 0.46 and 0.54. The flat plane is 0.5, so the nearest one changes.
                // No outline or golden image contributes to this depth oracle.
                assert_eq!(
                    target.canvas.read_data(1, 2),
                    3,
                    "textured={textured} sloped_first={sloped_first} near sloped face"
                );
                assert_eq!(
                    target.canvas.read_data(3, 2),
                    4,
                    "textured={textured} sloped_first={sloped_first} near flat face"
                );
            }
        }
    }

    #[test]
    fn test_projection_retains_finite_depth_across_camera_scales() {
        for (near, far, distance) in [
            (1e-30_f32, 1e-29_f32, 5e-30_f32),
            (1e20, 1e21, 5e20),
            (1e38, 2e38, 1.5e38),
            (2e38, 3e38, 2.5e38),
        ] {
            for ortho in [false, true] {
                let camera = Camera::new();
                let mut camera = rc_mut!(&camera);
                camera.near = near;
                camera.far = far;
                camera.ortho_size = ortho.then_some(4.0);
                let view = view_matrix(&camera);
                let projection = projection_matrix(&camera, 8.0, 8.0);
                let vp = matmul(&projection, &view);
                let result = world_to_screen(
                    &vec3(0.0, 0.0, -distance),
                    &vp,
                    &camera_clip_row(&view),
                    0.0,
                    0.0,
                    8.0,
                    8.0,
                )
                .unwrap();
                assert_eq!((result.0, result.1), (4.0, 4.0));
                let (n, f, d) = (f64::from(near), f64::from(far), f64::from(distance));
                // Independent normalized distance formula, evaluated before f32 storage.
                let expected = if ortho {
                    2.0 * (d - n) / (f - n) - 1.0
                } else {
                    1.0 - 2.0 * n * (f - d) / (d * (f - n))
                };
                assert!(
                    (f64::from(result.2) - expected).abs() < 2e-6,
                    "near={near} far={far} ortho={ortho} result={result:?}"
                );
                let (image, mut depth, _) = make_target_and_depth(8, 8);
                write_pixel(
                    &mut rc_mut!(&image),
                    &mut depth,
                    8,
                    4,
                    4,
                    result.2,
                    7,
                    1.0,
                    true,
                    true,
                );
                assert_eq!(rc_ref!(&image).canvas.read_data(4, 4), 7);
            }
        }
    }

    #[test]
    fn test_projection_retains_finite_depth_with_shifted_diagonal_camera() {
        for scale in [1.0_f32, 1e20, 1e38] {
            let axis = vec3(
                -std::f32::consts::FRAC_1_SQRT_2,
                -std::f32::consts::FRAC_1_SQRT_2,
                0.0,
            );
            let eye = vec3(axis.x * scale, axis.y * scale, 0.0);
            let point = vec3(
                eye.x + axis.x * (2.5 * scale),
                eye.y + axis.y * (2.5 * scale),
                0.0,
            );
            let target = vec3(eye.x + axis.x * scale, eye.y + axis.y * scale, 0.0);
            for ortho in [false, true] {
                let camera = Camera::new();
                let mut camera = rc_mut!(&camera);
                camera.near = 2.0 * scale;
                camera.far = 3.0 * scale;
                camera.ortho_size = ortho.then_some(scale);
                camera.transform = Mat4::look_at(&eye, &target, &vec3(0.0, 1.0, 0.0));
                let view = view_matrix(&camera);
                let projection = projection_matrix(&camera, 8.0, 8.0);
                let vp = matmul(&projection, &view);
                assert!(vp.iter().flatten().all(|value| value.is_finite()));
                let result =
                    world_to_screen(&point, &vp, &camera_clip_row(&view), 0.0, 0.0, 8.0, 8.0)
                        .unwrap();
                assert!((result.0 - 4.0).abs() < 2e-6 && (result.1 - 4.0).abs() < 2e-6);
                assert!(
                    (result.2 - if ortho { 0.0 } else { 0.2 }).abs() < 2e-6,
                    "scale={scale} ortho={ortho} result={result:?}"
                );
            }
        }
    }

    #[test]
    fn test_rasterize_adjacent_triangles_have_exact_shared_edge_ownership() {
        // The first rectangle comes from a projected, finite translated scene.
        // Integer coordinates at 2^-20 pixel units give an independent exact
        // half-plane oracle for every pixel, including its internal diagonal.
        for (left, top, right, bottom) in [
            (10.4574375, 10.4574375, 21.542564, 21.542562),
            (1.25, 2.75, 29.125, 28.5),
            (0.5, 0.5, 31.5, 31.5),
        ] {
            let points = [
                (left, top, 0.25),
                (right, top, 0.25),
                (left, bottom, 0.25),
                (right, bottom, 0.25),
            ];
            for diagonal in [[[0, 1, 2], [1, 3, 2]], [[0, 1, 3], [0, 3, 2]]] {
                for reversed in [false, true] {
                    for textured in [false, true] {
                        for depth_test in [false, true] {
                            let mut masks = Vec::new();
                            for indices in diagonal {
                                let mut triangle = indices.map(|index| points[index]);
                                if reversed {
                                    triangle.swap(0, 1);
                                }
                                let (image, mut depth, clip) = make_target_and_depth(32, 32);
                                let mut target = rc_mut!(&image);
                                if textured {
                                    rasterize_textured_triangle(
                                        &mut target,
                                        &mut depth,
                                        32,
                                        triangle[0],
                                        triangle[1],
                                        triangle[2],
                                        (triangle[0].0, triangle[0].1),
                                        (triangle[1].0, triangle[1].1),
                                        (triangle[2].0, triangle[2].1),
                                        |u, v, x, y| {
                                            assert!((u - (x as f32 + 0.5)).abs() < 1e-4);
                                            assert!((v - (y as f32 + 0.5)).abs() < 1e-4);
                                            Some(7)
                                        },
                                        clip,
                                        1.0,
                                        depth_test,
                                        depth_test,
                                    );
                                } else {
                                    rasterize_triangle(
                                        &mut target,
                                        &mut depth,
                                        32,
                                        triangle[0],
                                        triangle[1],
                                        triangle[2],
                                        7,
                                        7,
                                        clip,
                                        1.0,
                                        depth_test,
                                        depth_test,
                                    );
                                }
                                let fixed = |value: f32| (f64::from(value) * 1_048_576.0) as i128;
                                let cross =
                                    |a: (f32, f32, f32), b: (f32, f32, f32), x: f32, y: f32| {
                                        (fixed(b.0) - fixed(a.0)) * (fixed(y) - fixed(a.1))
                                            - (fixed(b.1) - fixed(a.1)) * (fixed(x) - fixed(a.0))
                                    };
                                let positive =
                                    cross(triangle[0], triangle[1], triangle[2].0, triangle[2].1)
                                        > 0;
                                for y in 0..32 {
                                    for x in 0..32 {
                                        let expected = (0..3).all(|i| {
                                            let mut a = triangle[i];
                                            let mut b = triangle[(i + 1) % 3];
                                            if !positive {
                                                std::mem::swap(&mut a, &mut b);
                                            }
                                            let determinant =
                                                cross(a, b, x as f32 + 0.5, y as f32 + 0.5);
                                            determinant > 0
                                                || (determinant == 0
                                                    && (b.1 < a.1 || (b.1 == a.1 && b.0 > a.0)))
                                        });
                                        assert_eq!(target.canvas.read_data(x, y) != 0, expected,
                                            "pixel=({x},{y}) triangle={triangle:?} textured={textured} depth={depth_test}");
                                    }
                                }
                                masks.push(target.canvas.data.clone());
                            }
                            for y in 0..32 {
                                for x in 0..32 {
                                    let count = usize::from(masks[0][y * 32 + x] != 0)
                                        + usize::from(masks[1][y * 32 + x] != 0);
                                    let inside = x as f32 + 0.5 >= left
                                        && x as f32 + 0.5 < right
                                        && y as f32 + 0.5 >= top
                                        && y as f32 + 0.5 < bottom;
                                    assert_eq!(
                                        count,
                                        usize::from(inside),
                                        "shared pixel=({x},{y})"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_coverage_edge_sign_near_collinear_pixel_centers() {
        let fixed = |value: f32| (f64::from(value) * 1_073_741_824.0) as i128;
        for x in 0..32 {
            for y in 0..32 {
                let p = (x as f32 + 0.5, y as f32 + 0.5);
                for direction in [(0.0, 4.125), (7.25, 0.0), (3.375, 9.125), (-5.125, 8.625)] {
                    let a = (p.0 - direction.0, p.1 - direction.1);
                    for ratio in [0.3, 1.0, 1.7] {
                        let b = (p.0 + direction.0 * ratio, p.1 + direction.1 * ratio);
                        for offset in [-1, 0, 1] {
                            if b.0 == 0.0 || b.0.abs() < 0.125 {
                                continue;
                            }
                            let b = (
                                f32::from_bits(b.0.to_bits().wrapping_add_signed(offset)),
                                b.1,
                            );
                            for coordinate in [a.0, a.1, b.0, b.1, p.0, p.1] {
                                assert_eq!(
                                    fixed(coordinate) as f64 / 1_073_741_824.0,
                                    f64::from(coordinate)
                                );
                            }
                            let exact = (fixed(b.0) - fixed(a.0)) * (fixed(p.1) - fixed(a.1))
                                - (fixed(b.1) - fixed(a.1)) * (fixed(p.0) - fixed(a.0));
                            let clip = ClipRect {
                                left: 0,
                                top: 0,
                                right: 31,
                                bottom: 31,
                            };
                            for positive in [false, true] {
                                let inside = CoverageEdge::new(a, b, clip)
                                    .inside_weight(p, false, positive)
                                    .is_some();
                                let reverse = CoverageEdge::new(b, a, clip)
                                    .inside_weight(p, false, !positive)
                                    .is_some();
                                assert_eq!(
                                    inside,
                                    if positive { exact > 0 } else { exact < 0 },
                                    "a={a:?} b={b:?} p={p:?}"
                                );
                                assert_eq!(inside, reverse);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_rasterize_thin_triangle_with_representable_area() {
        let points = [
            (21.542564, 10.4574375, 0.25),
            (10.4574375, 21.542562, 0.25),
            (12.5, 19.5, 0.25),
        ];
        let area = edge_function(
            (points[0].0, points[0].1),
            (points[1].0, points[1].1),
            (points[2].0, points[2].1),
        );
        assert!(area > 1e-6);
        let plane = ScreenPlane::from_triangle(points[0], points[1], points[2]).unwrap();
        assert_eq!(plane.z_at(11.5, 20.5), 0.25);
        for x_slope in [false, true] {
            let sloped = points.map(|(x, y, _)| (x, y, if x_slope { x } else { y } / 32.0));
            let plane = ScreenPlane::from_triangle(sloped[0], sloped[1], sloped[2]).unwrap();
            assert_eq!(plane.dx, if x_slope { 1.0 / 32.0 } else { 0.0 });
            assert_eq!(plane.dy, if x_slope { 0.0 } else { 1.0 / 32.0 });
            assert_eq!(
                plane.z_at(11.5, 20.5),
                if x_slope { 11.5 } else { 20.5 } / 32.0
            );
        }
        for reversed in [false, true] {
            for textured in [false, true] {
                let mut points = points;
                if reversed {
                    points.swap(0, 1);
                }
                let (image, mut depth, clip) = make_target_and_depth(32, 32);
                let mut target = rc_mut!(&image);
                if textured {
                    rasterize_textured_triangle(
                        &mut target,
                        &mut depth,
                        32,
                        points[0],
                        points[1],
                        points[2],
                        (0.0, 0.0),
                        (0.0, 0.0),
                        (0.0, 0.0),
                        |_, _, _, _| Some(7),
                        clip,
                        1.0,
                        true,
                        true,
                    );
                } else {
                    rasterize_triangle(
                        &mut target,
                        &mut depth,
                        32,
                        points[0],
                        points[1],
                        points[2],
                        7,
                        7,
                        clip,
                        1.0,
                        true,
                        true,
                    );
                }
                for (x, y) in [(11, 20), (10, 21)] {
                    assert_eq!(
                        target.canvas.read_data(x, y),
                        7,
                        "pixel=({x},{y}) reversed={reversed} textured={textured}"
                    );
                    assert!((depth[y * 32 + x] - 0.25).abs() < 1e-6);
                }
            }
        }
    }
}
