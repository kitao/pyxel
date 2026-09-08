// Raster formulas use conventional x/y/z/u/v component names.
#![allow(clippy::many_single_char_names)]

use crate::cube::camera::Camera;
use crate::cube::mat4::Mat4;
use crate::cube::shading::{Shading, LEVEL_COUNT};
use crate::cube::vec3::Vec3;
use crate::image::Image;
use crate::utils::{f32_to_i32, f32_to_u32};

// Row-major matrix: m[i][j] is row i, column j.
pub type Mat4x4 = [[f32; 4]; 4];

type ScreenPoint = (f32, f32, f32);

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

// View-projection helpers

pub fn view_matrix(camera: &Camera) -> Mat4x4 {
    rc_ref!(&camera.transform).inverse_value().data
}

pub fn projection_matrix(camera: &Camera, vp_w: f32, vp_h: f32) -> Mat4x4 {
    let aspect = if vp_h == 0.0 { 1.0 } else { vp_w / vp_h };
    let near = camera.near;
    let far = camera.far;

    if let Some(size) = camera.ortho_size {
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
    }
}

pub fn matmul(a: &Mat4x4, b: &Mat4x4) -> Mat4x4 {
    let mut r = [[0.0_f32; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                r[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    r
}

// Dot with a world position (w = 1) to get distance in front of the camera.
// Orthographic clipping needs this because its projection w is constant.
pub fn camera_clip_row(view: &Mat4x4) -> [f32; 4] {
    [-view[2][0], -view[2][1], -view[2][2], -view[2][3]]
}

// Return a value to avoid RcVec3 allocation per vertex.
pub fn mat_apply(mat: &Mat4, v: &Vec3) -> Vec3 {
    mat.mul_vec_value(v)
}

pub fn mat_apply_dir(mat: &Mat4, v: &Vec3) -> Vec3 {
    mat.mul_dir_value(v)
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

    let ndc_x = cx / cw;
    let ndc_y = cy / cw;
    let ndc_z = cz / cw;
    let sx = vp_x + (ndc_x + 1.0) * 0.5 * vp_w;
    let sy = vp_y + (1.0 - (ndc_y + 1.0) * 0.5) * vp_h;
    Some((sx, sy, ndc_z))
}

// Geometry helpers

// Unscaled face normal of the CCW triangle (cross product of two edges).
pub fn tri_normal(p0: &Vec3, p1: &Vec3, p2: &Vec3) -> Vec3 {
    let e1 = Vec3 {
        x: p1.x - p0.x,
        y: p1.y - p0.y,
        z: p1.z - p0.z,
    };
    let e2 = Vec3 {
        x: p2.x - p0.x,
        y: p2.y - p0.y,
        z: p2.z - p0.z,
    };

    Vec3 {
        x: e1.y * e2.z - e1.z * e2.y,
        y: e1.z * e2.x - e1.x * e2.z,
        z: e1.x * e2.y - e1.y * e2.x,
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

// Enough segments for smooth ellipses at SD resolution.
pub const ELLIPSE_SEGMENTS: usize = 24;

// Billboard sprite corners: a quad facing the camera, rotated by
// `angle_deg` in screen space (around view-z). Corners are returned in
// row-major order: top-left, top-right, bottom-left, bottom-right.
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
// Keep coplanar outlines visible over filled surfaces.
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

// Lambert brightness: max(0, dot(normal, -light_direction)).
pub fn face_shade_level(direction: &Vec3, normal: Option<&Vec3>) -> usize {
    let dot_factor = match normal {
        Some(n) => {
            let n_len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
            let d_len =
                (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z)
                    .sqrt();
            if n_len == 0.0 || d_len == 0.0 {
                0.0
            } else {
                let dot =
                    -(n.x * direction.x + n.y * direction.y + n.z * direction.z) / (n_len * d_len);
                dot.max(0.0)
            }
        }
        None => 0.0,
    };

    let max_level = (LEVEL_COUNT - 1) as f32;
    let level_f = dot_factor * max_level;
    level_f.clamp(0.0, max_level).round() as usize
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

// Signed doubled area, used as the barycentric divisor.
#[inline]
fn edge_function(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
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

    for y in by_min..=by_max {
        let py = y as f32 + 0.5;
        // A triangle intersects each scanline in one contiguous span. Once
        // the loop leaves that span, no later pixel on the row can be inside.
        let mut was_inside = false;

        for x in bx_min..=bx_max {
            let p = (x as f32 + 0.5, py);
            let w0 = edge_function((p1.0, p1.1), (p2.0, p2.1), p);
            if !edge_inside(w0, include_w0, pos_area) {
                if was_inside {
                    break;
                }
                continue;
            }

            let w1 = edge_function((p2.0, p2.1), (p0.0, p0.1), p);
            if !edge_inside(w1, include_w1, pos_area) {
                if was_inside {
                    break;
                }
                continue;
            }

            let w2 = edge_function((p0.0, p0.1), (p1.0, p1.1), p);
            if !edge_inside(w2, include_w2, pos_area) {
                if was_inside {
                    break;
                }
                continue;
            }
            was_inside = true;

            let bary0 = w0 * inv_area;
            let bary1 = w1 * inv_area;
            let bary2 = w2 * inv_area;
            let z = bary0 * p0.2 + bary1 * p1.2 + bary2 * p2.2;
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

    for y in by_min..=by_max {
        let py = y as f32 + 0.5;
        // Same lazy-edge, contiguous-span row scan as rasterize_triangle.
        let mut was_inside = false;

        for x in bx_min..=bx_max {
            let p = (x as f32 + 0.5, py);
            let w0 = edge_function((p1.0, p1.1), (p2.0, p2.1), p);
            if !edge_inside(w0, include_w0, pos_area) {
                if was_inside {
                    break;
                }
                continue;
            }

            let w1 = edge_function((p2.0, p2.1), (p0.0, p0.1), p);
            if !edge_inside(w1, include_w1, pos_area) {
                if was_inside {
                    break;
                }
                continue;
            }

            let w2 = edge_function((p0.0, p0.1), (p1.0, p1.1), p);
            if !edge_inside(w2, include_w2, pos_area) {
                if was_inside {
                    break;
                }
                continue;
            }
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
    let min_x = clip.left as f32;
    let max_x = clip.right as f32;
    let min_y = clip.top as f32;
    let max_y = clip.bottom as f32;
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

#[inline]
fn line_depth(z: f32) -> f32 {
    if depth_in_clip_range(z) {
        (z - LINE_DEPTH_BIAS).max(-1.0)
    } else {
        f32::INFINITY
    }
}

// Floating-point DDA line with linear z interpolation and fixed 1-pixel width.
// Clip before stepping so near-plane clipped edges do not traverse off-screen pixels.
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
    let Some((p0, p1)) = clip_screen_line_to_rect(p0, p1, clip) else {
        return;
    };

    let x1 = p0.0.round() as i32;
    let y1 = p0.1.round() as i32;
    let x2 = p1.0.round() as i32;
    let y2 = p1.1.round() as i32;
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    if dx == 0 && dy == 0 {
        if clip.contains(x1, y1) {
            let col = dither_pick(primary as i32, secondary as i32, x1, y1);
            write_pixel(
                target,
                depth,
                depth_w,
                x1,
                y1,
                line_depth(p0.2),
                col,
                dither_alpha,
                depth_test,
                depth_write,
            );
        }
        return;
    }

    let steps = dx.max(dy);
    let inv = 1.0 / steps as f32;
    let dx_f = (x2 - x1) as f32 * inv;
    let dy_f = (y2 - y1) as f32 * inv;
    let dz_f = (p1.2 - p0.2) * inv;
    let mut fx = x1 as f32;
    let mut fy = y1 as f32;
    let mut fz = p0.2;

    for _ in 0..=steps {
        let xi = fx.round() as i32;
        let yi = fy.round() as i32;
        if clip.contains(xi, yi) {
            let col = dither_pick(primary as i32, secondary as i32, xi, yi);
            write_pixel(
                target,
                depth,
                depth_w,
                xi,
                yi,
                line_depth(fz),
                col,
                dither_alpha,
                depth_test,
                depth_write,
            );
        }

        fx += dx_f;
        fy += dy_f;
        fz += dz_f;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::RcImage;

    fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
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
    fn test_mat_apply_translation() {
        let mat = Mat4::from_translation(&rc_ref!(&Vec3::new(1.0, 2.0, 3.0)));
        let result = mat_apply(&rc_ref!(&mat), &vec3(0.0, 0.0, 0.0));
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_mat_apply_identity_preserves_vec3() {
        let mat = Mat4::identity();
        let result = mat_apply(&rc_ref!(&mat), &vec3(4.0, 5.0, 6.0));
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 5.0);
        assert_eq!(result.z, 6.0);
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
        assert!(p1.0 <= 31.0);
        assert_eq!(p0.1, 16.0);
        assert_eq!(p1.1, 16.0);
        assert_eq!(p0.2, 0.5);
        assert!((p1.2 - 0.5000155).abs() < 1e-6);
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
    fn test_rasterize_line_matches_2d_line_pixels() {
        let expected = Image::new(32, 32);
        rc_mut!(&expected).draw_line(4.0, 5.0, 27.0, 16.0, 7);

        let (actual, mut depth, clip) = make_target_and_depth(32, 32);
        rasterize_line(
            &mut rc_mut!(&actual),
            &mut depth,
            32,
            (4.0, 5.0, 0.0),
            (27.0, 16.0, 0.5),
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
}
