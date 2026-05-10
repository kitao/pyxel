#![allow(clippy::many_single_char_names)]

use crate::cube::camera::Camera;
use crate::cube::mat4::Mat4;
use crate::cube::shading::{Shading, LEVEL_COUNT};
use crate::cube::vec3::Vec3;
use crate::image::{Image, RcImage};
use crate::tilemap::RcTilemap;

// 4x4 matrix in row-major form (m[i][j] is row i, column j). Used as the
// combined view-projection matrix applied to every world-space point.
pub type Mat4x4 = [[f32; 4]; 4];

// Texture source for textured draw commands. Mirrors the cube `sprite` /
// `plane` argument, which accepts either an Image or a Tilemap.
pub enum Texture {
    Image(RcImage),
    Tilemap(RcTilemap),
}

// Pixel-aligned destination rectangle that bounds rasterizer output.
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
    let right = (vp_x + vp_w).ceil() as i32 - 1;
    let bottom = (vp_y + vp_h).ceil() as i32 - 1;
    ClipRect {
        left: left.max(0),
        top: top.max(0),
        right: right.min(target_w as i32 - 1),
        bottom: bottom.min(target_h as i32 - 1),
    }
}

// View-projection helpers

pub fn view_matrix(camera: &Camera) -> Mat4x4 {
    let inv_rc = rc_ref!(&camera.transform).inverse();
    rc_ref!(&inv_rc).data
}

// Perspective unless `ortho_size` is set, then orthographic.
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

// Apply a Mat4 transform to a Vec3 with implicit w=1.
pub fn mat_apply(mat: &Mat4, v: &Vec3) -> Vec3 {
    let rc = mat.mul_vec(v);
    *rc_ref!(&rc)
}

// Project a world position to screen space; None when behind the near
// plane (cw <= 0). Off-screen and far-plane-overshoot points still
// project so partially off-screen primitives keep contributing through
// the viewport clip and z-test, avoiding fp-roundoff flicker at z = 1.
pub fn world_to_screen(
    pos: &Vec3,
    m: &Mat4x4,
    vp_x: f32,
    vp_y: f32,
    vp_w: f32,
    vp_h: f32,
) -> Option<(f32, f32, f32)> {
    let cx = m[0][0] * pos.x + m[0][1] * pos.y + m[0][2] * pos.z + m[0][3];
    let cy = m[1][0] * pos.x + m[1][1] * pos.y + m[1][2] * pos.z + m[1][3];
    let cz = m[2][0] * pos.x + m[2][1] * pos.y + m[2][2] * pos.z + m[2][3];
    let cw = m[3][0] * pos.x + m[3][1] * pos.y + m[3][2] * pos.z + m[3][3];
    if cw <= 0.0 {
        return None;
    }
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

// Camera right and up axes as world-space directions; used by billboard
// sprites and screen_circle's edge sample.
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

// Project the four corners of a Mat4-positioned rectangle (the rectangle
// lies in `mat`'s local XY plane, sized w x h, centered at the local
// origin) into screen space. Row-major corner order: 0=top-left,
// 1=top-right, 2=bottom-left, 3=bottom-right — matches uv layout used
// by sprite / plane.
pub fn project_rect_corners(
    mat: &Mat4,
    w: f32,
    h: f32,
    vp: &Mat4x4,
    vp_x: f32,
    vp_y: f32,
    vp_w: f32,
    vp_h: f32,
) -> [Option<(f32, f32, f32)>; 4] {
    let hw = w * 0.5;
    let hh = h * 0.5;
    let local = [
        Vec3 {
            x: -hw,
            y: hh,
            z: 0.0,
        },
        Vec3 {
            x: hw,
            y: hh,
            z: 0.0,
        },
        Vec3 {
            x: -hw,
            y: -hh,
            z: 0.0,
        },
        Vec3 {
            x: hw,
            y: -hh,
            z: 0.0,
        },
    ];
    std::array::from_fn(|i| {
        let world = mat_apply(mat, &local[i]);
        world_to_screen(&world, vp, vp_x, vp_y, vp_w, vp_h)
    })
}

// Number of segments approximating an ellipse perimeter. 24 was chosen
// for visible smoothness at SD pixel scales.
pub const ELLIPSE_SEGMENTS: usize = 24;

// Project ELLIPSE_SEGMENTS points along an ellipse perimeter (in mat's
// local XY plane, axes w/2 and h/2) into screen space.
pub fn project_ellipse_perimeter(
    mat: &Mat4,
    w: f32,
    h: f32,
    vp: &Mat4x4,
    vp_x: f32,
    vp_y: f32,
    vp_w: f32,
    vp_h: f32,
) -> [Option<(f32, f32, f32)>; ELLIPSE_SEGMENTS] {
    let hw = w * 0.5;
    let hh = h * 0.5;
    std::array::from_fn(|i| {
        let theta = 2.0 * std::f32::consts::PI * (i as f32) / (ELLIPSE_SEGMENTS as f32);
        let local = Vec3 {
            x: hw * theta.cos(),
            y: hh * theta.sin(),
            z: 0.0,
        };
        let world = mat_apply(mat, &local);
        world_to_screen(&world, vp, vp_x, vp_y, vp_w, vp_h)
    })
}

// Billboard sprite corners: a quad facing the camera, rotated by
// `angle_deg` in screen space (around view-z). Row-major corner order
// matches project_rect_corners.
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
    camera: &Camera,
    vp_x: f32,
    vp_y: f32,
    vp_w: f32,
    vp_h: f32,
) -> Option<(f32, f32, f32, f32)> {
    let (right, _up) = camera_right_up(camera);
    let center = world_to_screen(pos, m, vp_x, vp_y, vp_w, vp_h)?;
    let edge_pos = Vec3 {
        x: pos.x + radius * right.x,
        y: pos.y + radius * right.y,
        z: pos.z + radius * right.z,
    };
    let edge = world_to_screen(&edge_pos, m, vp_x, vp_y, vp_w, vp_h)?;
    let dx = edge.0 - center.0;
    let dy = edge.1 - center.1;
    let screen_r = (dx * dx + dy * dy).sqrt();
    Some((center.0, center.1, screen_r, center.2))
}

// 4x4 Bayer ordered-dither thresholds for the alpha (= per-pixel
// transparency) gate inside `write_pixel`. The shading LUT itself uses
// only flat or 50:50 checker, which `dither_pick` handles directly via
// the 2x2 parity (no Bayer matrix needed for that case).
pub const BAYER4: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

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

// Per-face brightness level (0..LEVEL_COUNT-1) from the shading's light
// direction and the face normal. Pure Lambert: brightness scales with
// max(0, dot(face_normal, -light_dir)).
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

// Resolve `(base_col, normal)` to a `(primary, secondary)` pair at the
// face's brightness level. Hot-path callers should reuse this once per
// face and dither at each pixel via `dither_pick`. Returns a degenerate
// `(base_col, base_col)` pair when the LUT is empty.
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

// Returns the LUT primary at the face's brightness level. Callers that
// want dithering should use `lookup_ramp` and `dither_pick` instead.
pub fn shade(shading: &Shading, base_col: i32, normal: Option<&Vec3>) -> u8 {
    let (primary, _) = lookup_ramp(shading, base_col, normal);
    primary as u8
}

// Pixel write with depth test. Callers are responsible for clip
// containment; bbox-driven rasterizers below already drop out-of-clip
// pixels at the loop bounds, so this hot-path function does not re-check.
#[inline]
#[allow(clippy::too_many_arguments)]
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
    // Bayer-pattern alpha gate. dither_alpha == 1.0 always passes;
    // 0.0 always rejects; intermediate values produce a regular
    // 4x4 stipple pattern.
    if dither_alpha < 1.0 {
        let bayer = BAYER4[(y.rem_euclid(4)) as usize][(x.rem_euclid(4)) as usize];
        let threshold = (bayer as f32 + 0.5) / 16.0;
        if (1.0 - dither_alpha) >= threshold {
            return;
        }
    }
    let i = (y as usize) * depth_w as usize + x as usize;
    if depth_test && z >= depth[i] {
        return;
    }
    if depth_write {
        depth[i] = z;
    }
    target.canvas.write_data(x as usize, y as usize, col);
}

// Rasterizers

// Signed area of triangle abc (positive when CCW with +Y down). Acts as
// a barycentric divisor.
#[inline]
fn edge_function(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

// Filled triangle with linear z interpolation. Both windings draw — cube
// has no back-face culling. Per-pixel dither between `primary` and
// `secondary` based on `ratio` (0..16); `primary == secondary` or
// `ratio == 0` collapses to a flat fill.
#[allow(clippy::too_many_arguments)]
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
    for y in by_min..=by_max {
        for x in bx_min..=bx_max {
            let p = (x as f32 + 0.5, y as f32 + 0.5);
            let w0 = edge_function((p1.0, p1.1), (p2.0, p2.1), p);
            let w1 = edge_function((p2.0, p2.1), (p0.0, p0.1), p);
            let w2 = edge_function((p0.0, p0.1), (p1.0, p1.1), p);
            let inside = if area > 0.0 {
                w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
            } else {
                w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
            };
            if inside {
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
}

// Filled triangle with linear UV + z interpolation. The sampler receives
// `(u, v, x, y)` so it can mix in screen-space dither when shading a
// textured face. colkey drops pixels whose source matches the key.
// Interpolation is affine — good enough for cube's pixel-art scale.
#[allow(clippy::too_many_arguments)]
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
    colkey: Option<i32>,
    clip: ClipRect,
    dither_alpha: f32,
    depth_test: bool,
    depth_write: bool,
) where
    F: Fn(f32, f32, i32, i32) -> i32,
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
    for y in by_min..=by_max {
        for x in bx_min..=bx_max {
            let p = (x as f32 + 0.5, y as f32 + 0.5);
            let w0 = edge_function((p1.0, p1.1), (p2.0, p2.1), p);
            let w1 = edge_function((p2.0, p2.1), (p0.0, p0.1), p);
            let w2 = edge_function((p0.0, p0.1), (p1.0, p1.1), p);
            let inside = if area > 0.0 {
                w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
            } else {
                w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
            };
            if !inside {
                continue;
            }
            let bary0 = w0 * inv_area;
            let bary1 = w1 * inv_area;
            let bary2 = w2 * inv_area;
            let z = bary0 * p0.2 + bary1 * p1.2 + bary2 * p2.2;
            let u = bary0 * uv0.0 + bary1 * uv1.0 + bary2 * uv2.0;
            let v = bary0 * uv0.1 + bary1 * uv1.1 + bary2 * uv2.1;
            let col = sampler(u, v, x, y);
            if let Some(key) = colkey {
                if col == key {
                    continue;
                }
            }
            write_pixel(
                target,
                depth,
                depth_w,
                x,
                y,
                z,
                col as u8,
                dither_alpha,
                depth_test,
                depth_write,
            );
        }
    }
}

// Filled screen-space circle at constant depth. cx / cy / radius are in
// pixels (project a world-space circle through `screen_circle` first).
#[allow(clippy::too_many_arguments)]
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
    let r_int = radius.ceil() as i32;
    let r2 = radius * radius;
    let cx_int = cx.round() as i32;
    let cy_int = cy.round() as i32;
    let bx_min = (cx_int - r_int).max(clip.left);
    let bx_max = (cx_int + r_int).min(clip.right);
    let by_min = (cy_int - r_int).max(clip.top);
    let by_max = (cy_int + r_int).min(clip.bottom);
    for y in by_min..=by_max {
        for x in bx_min..=bx_max {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            if dx * dx + dy * dy <= r2 {
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
}

// 1-pixel-thick screen-space circle outline. The band [radius - 0.5,
// radius + 0.5] keeps the ring isotropic at any distance.
#[allow(clippy::too_many_arguments)]
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
    let r_int = radius.ceil() as i32;
    let outer = radius + 0.5;
    let inner = (radius - 0.5).max(0.0);
    let outer2 = outer * outer;
    let inner2 = inner * inner;
    let cx_int = cx.round() as i32;
    let cy_int = cy.round() as i32;
    let bx_min = (cx_int - r_int).max(clip.left);
    let bx_max = (cx_int + r_int).min(clip.right);
    let by_min = (cy_int - r_int).max(clip.top);
    let by_max = (cy_int + r_int).min(clip.bottom);
    for y in by_min..=by_max {
        for x in bx_min..=bx_max {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            let d2 = dx * dx + dy * dy;
            if d2 <= outer2 && d2 >= inner2 {
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
}

// Bresenham-style 3D line with linear z interpolation. Width is fixed at
// 1 pixel regardless of distance. The line span is not pre-clipped, so
// each step is checked against `clip` here before write_pixel.
#[allow(clippy::too_many_arguments)]
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
                p0.2,
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
                fz,
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

    #[test]
    fn test_matmul_identity() {
        let identity: Mat4x4 = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let result = matmul(&identity, &identity);
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((result[i][j] - expected).abs() < 1e-6);
            }
        }
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
        let v = view_matrix(rc_ref!(&camera));
        // Identity camera transform yields identity view matrix.
        assert!((v[0][0] - 1.0).abs() < 1e-6);
        assert!((v[1][1] - 1.0).abs() < 1e-6);
        assert!((v[2][2] - 1.0).abs() < 1e-6);
        assert!((v[3][3] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_projection_matrix_perspective_aspect() {
        let camera = Camera::new();
        let p = projection_matrix(rc_ref!(&camera), 256.0, 192.0);
        // Perspective: p[0][0] = f / aspect, p[1][1] = f.
        let f = 1.0 / (rc_ref!(&camera).fov.to_radians() * 0.5).tan();
        let aspect = 256.0 / 192.0;
        assert!((p[0][0] - f / aspect).abs() < 1e-4);
        assert!((p[1][1] - f).abs() < 1e-4);
        // Last row uses w = -z (standard perspective divide).
        assert!((p[3][2] - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_projection_matrix_orthographic() {
        let camera = Camera::new();
        rc_mut!(&camera).ortho_size = Some(10.0);
        let p = projection_matrix(rc_ref!(&camera), 200.0, 100.0);
        // Orthographic: p[0][0] = 1 / (size/2 * aspect), p[1][1] = 1 / (size/2).
        let half_h = 5.0_f32;
        let half_w = half_h * 2.0;
        assert!((p[0][0] - 1.0 / half_w).abs() < 1e-6);
        assert!((p[1][1] - 1.0 / half_h).abs() < 1e-6);
        // Orthographic last row stays affine.
        assert!((p[3][3] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_mat_apply_translation() {
        let mat = Mat4::from_translation(rc_ref!(&Vec3::new(1.0, 2.0, 3.0)));
        let result = mat_apply(rc_ref!(&mat), &vec3(0.0, 0.0, 0.0));
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_mat_apply_identity_preserves_vec3() {
        let mat = Mat4::identity();
        let result = mat_apply(rc_ref!(&mat), &vec3(4.0, 5.0, 6.0));
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 5.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_world_to_screen_center() {
        let camera = Camera::new();
        let v = view_matrix(rc_ref!(&camera));
        let p = projection_matrix(rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        // Default camera looks down -Z; pick a point in front.
        let result = world_to_screen(&vec3(0.0, 0.0, -2.0), &vp, 0.0, 0.0, 256.0, 192.0);
        let (sx, sy, _z) = result.expect("point in front of camera should project");
        assert!((sx - 128.0).abs() < 1e-3);
        assert!((sy - 96.0).abs() < 1e-3);
    }

    #[test]
    fn test_world_to_screen_behind_camera() {
        let camera = Camera::new();
        let v = view_matrix(rc_ref!(&camera));
        let p = projection_matrix(rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        // A point behind the camera (+Z by default) returns None.
        let result = world_to_screen(&vec3(0.0, 0.0, 5.0), &vp, 0.0, 0.0, 256.0, 192.0);
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
        let (right, up) = camera_right_up(rc_ref!(&camera));
        // Identity camera: right = +X, up = +Y.
        assert!((right.x - 1.0).abs() < 1e-6);
        assert_eq!(right.y, 0.0);
        assert_eq!(right.z, 0.0);
        assert_eq!(up.x, 0.0);
        assert!((up.y - 1.0).abs() < 1e-6);
        assert_eq!(up.z, 0.0);
    }

    #[test]
    fn test_screen_circle_centered() {
        let camera = Camera::new();
        let v = view_matrix(rc_ref!(&camera));
        let p = projection_matrix(rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        let result = screen_circle(
            &vec3(0.0, 0.0, -2.0),
            0.5,
            &vp,
            rc_ref!(&camera),
            0.0,
            0.0,
            256.0,
            192.0,
        );
        let (sx, sy, sr, _z) = result.expect("circle in view should project");
        assert!((sx - 128.0).abs() < 1e-3);
        assert!((sy - 96.0).abs() < 1e-3);
        assert!(sr > 0.0);
    }

    #[test]
    fn test_screen_circle_behind_camera() {
        let camera = Camera::new();
        let v = view_matrix(rc_ref!(&camera));
        let p = projection_matrix(rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        let result = screen_circle(
            &vec3(0.0, 0.0, 5.0),
            0.5,
            &vp,
            rc_ref!(&camera),
            0.0,
            0.0,
            256.0,
            192.0,
        );
        assert!(result.is_none());
    }

    fn pyxel_default_palette() -> Vec<crate::image::Rgb24> {
        vec![
            0x000000, 0x2B335F, 0x7E2072, 0x19959C, 0x8B4852, 0x395C98, 0xA9C1FF, 0xEEEEEE,
            0xD4186C, 0xD38441, 0xE9C35B, 0x70C6A9, 0x7696DE, 0xA3A3A3, 0xFF9798, 0xEDC7B0,
        ]
    }

    #[test]
    fn test_shade_no_normal_returns_lv0() {
        // No normal → directional factor = 0 → level 0 (= darkest plateau).
        let shading = Shading::new(&pyxel_default_palette());
        let shading = rc_ref!(&shading);
        let col = shade(shading, 7, None);
        assert!(col < 16);
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
    fn test_write_pixel_writes_at_coord() {
        let (img, mut depth, _) = make_target_and_depth(8, 8);
        let img_mut = rc_mut!(&img);
        write_pixel(img_mut, &mut depth, 8, 3, 3, 0.0, 5, 1.0, true, true);
        assert_eq!(img_mut.canvas.read_data(3, 3), 5);
        assert_eq!(depth[3 * 8 + 3], 0.0);
    }

    #[test]
    fn test_write_pixel_z_test_blocks_far() {
        let (img, mut depth, _) = make_target_and_depth(8, 8);
        let img_mut = rc_mut!(&img);
        write_pixel(img_mut, &mut depth, 8, 4, 4, 0.0, 10, 1.0, true, true);
        // A farther write attempt is rejected by the depth test.
        write_pixel(img_mut, &mut depth, 8, 4, 4, 0.5, 11, 1.0, true, true);
        assert_eq!(img_mut.canvas.read_data(4, 4), 10);
    }

    #[test]
    fn test_rasterize_line_horizontal() {
        let (img, mut depth, clip) = make_target_and_depth(32, 32);
        let img_mut = rc_mut!(&img);
        rasterize_line(
            img_mut,
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
        let img_mut = rc_mut!(&img);
        rasterize_line(
            img_mut,
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
        let img_mut = rc_mut!(&img);
        rasterize_line(
            img_mut,
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
        assert!((depth[3 * 8 + 3] - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_rasterize_line_depth_z_test() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let img_mut = rc_mut!(&img);
        rasterize_line(
            img_mut,
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
            img_mut,
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

    #[test]
    fn test_rasterize_line_clip_rejects_outside() {
        let (img, mut depth, _) = make_target_and_depth(8, 8);
        let img_mut = rc_mut!(&img);
        let small_clip = ClipRect {
            left: 0,
            top: 0,
            right: 3,
            bottom: 3,
        };
        rasterize_line(
            img_mut,
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
    fn test_rasterize_triangle_fills_interior() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let img_mut = rc_mut!(&img);
        rasterize_triangle(
            img_mut,
            &mut depth,
            16,
            (2.0, 2.0, 0.0),
            (12.0, 2.0, 0.0),
            (2.0, 12.0, 0.0),
            9,
            9,
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(img_mut.canvas.read_data(4, 4), 9);
        assert_eq!(img_mut.canvas.read_data(10, 10), 0);
    }

    #[test]
    fn test_rasterize_triangle_z_interpolated() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let img_mut = rc_mut!(&img);
        rasterize_triangle(
            img_mut,
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
        assert!(depth[0] < 0.1);
        assert!(depth[15] > 0.3);
    }

    #[test]
    fn test_rasterize_triangle_back_face_also_drawn() {
        // CW winding still rasterizes — cube has no back-face culling.
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let img_mut = rc_mut!(&img);
        rasterize_triangle(
            img_mut,
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

    #[test]
    fn test_rasterize_triangle_degenerate_skipped() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let img_mut = rc_mut!(&img);
        // Three colinear points -> zero area -> skip.
        rasterize_triangle(
            img_mut,
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
        let img_mut = rc_mut!(&img);
        let small_clip = ClipRect {
            left: 0,
            top: 0,
            right: 7,
            bottom: 7,
        };
        rasterize_triangle(
            img_mut,
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
    fn test_rasterize_textured_triangle_constant_sampler() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let img_mut = rc_mut!(&img);
        rasterize_textured_triangle(
            img_mut,
            &mut depth,
            16,
            (2.0, 2.0, 0.0),
            (12.0, 2.0, 0.0),
            (2.0, 12.0, 0.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            |_, _, _, _| 13,
            None,
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(img_mut.canvas.read_data(4, 4), 13);
        assert_eq!(img_mut.canvas.read_data(2, 11), 13);
    }

    #[test]
    fn test_rasterize_textured_triangle_uv_interpolation() {
        let (img, mut depth, clip) = make_target_and_depth(32, 32);
        let img_mut = rc_mut!(&img);
        rasterize_textured_triangle(
            img_mut,
            &mut depth,
            32,
            (5.0, 5.0, 0.0),
            (25.0, 5.0, 0.0),
            (5.0, 25.0, 0.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            |u, _v, _, _| if u < 0.5 { 4 } else { 9 },
            None,
            clip,
            1.0,
            true,
            true,
        );
        // Near p0 (u ~ 0) -> color 4; near p1 (u ~ 1) -> color 9.
        assert_eq!(img_mut.canvas.read_data(7, 6), 4);
        assert_eq!(img_mut.canvas.read_data(22, 6), 9);
    }

    #[test]
    fn test_rasterize_textured_triangle_colkey_skips() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let img_mut = rc_mut!(&img);
        rasterize_textured_triangle(
            img_mut,
            &mut depth,
            16,
            (2.0, 2.0, 0.0),
            (12.0, 2.0, 0.0),
            (2.0, 12.0, 0.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            |_, _, _, _| 0,
            Some(0),
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(img_mut.canvas.read_data(4, 4), 0);
        assert_eq!(depth[4 * 16 + 4], f32::INFINITY);
    }

    #[test]
    fn test_rasterize_textured_triangle_depth_test() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let img_mut = rc_mut!(&img);
        rasterize_textured_triangle(
            img_mut,
            &mut depth,
            16,
            (2.0, 2.0, 0.0),
            (12.0, 2.0, 0.0),
            (2.0, 12.0, 0.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            |_, _, _, _| 7,
            None,
            clip,
            1.0,
            true,
            true,
        );
        rasterize_textured_triangle(
            img_mut,
            &mut depth,
            16,
            (2.0, 2.0, 0.5),
            (12.0, 2.0, 0.5),
            (2.0, 12.0, 0.5),
            (0.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            |_, _, _, _| 11,
            None,
            clip,
            1.0,
            true,
            true,
        );
        assert_eq!(img_mut.canvas.read_data(4, 4), 7);
    }

    #[test]
    fn test_rasterize_circle_filled_center_and_edge() {
        let (img, mut depth, clip) = make_target_and_depth(32, 32);
        let img_mut = rc_mut!(&img);
        rasterize_circle_filled(
            img_mut, &mut depth, 32, 16.0, 16.0, 5.0, 0.0, 12, 0, clip, 1.0, true, true,
        );
        assert_eq!(img_mut.canvas.read_data(16, 16), 12);
        assert_eq!(img_mut.canvas.read_data(20, 16), 12);
        assert_eq!(img_mut.canvas.read_data(25, 25), 0);
    }

    #[test]
    fn test_rasterize_circle_border_thin_ring() {
        let (img, mut depth, clip) = make_target_and_depth(32, 32);
        let img_mut = rc_mut!(&img);
        rasterize_circle_border(
            img_mut, &mut depth, 32, 16.0, 16.0, 5.0, 0.0, 8, 0, clip, 1.0, true, true,
        );
        // Center pixel is NOT filled (border only).
        assert_eq!(img_mut.canvas.read_data(16, 16), 0);
        // Pixel near the rim is filled.
        assert_eq!(img_mut.canvas.read_data(20, 16), 8);
    }

    #[test]
    fn test_rasterize_circle_filled_z_test() {
        let (img, mut depth, clip) = make_target_and_depth(32, 32);
        let img_mut = rc_mut!(&img);
        rasterize_circle_filled(
            img_mut, &mut depth, 32, 16.0, 16.0, 5.0, 0.0, 10, 0, clip, 1.0, true, true,
        );
        rasterize_circle_filled(
            img_mut, &mut depth, 32, 16.0, 16.0, 5.0, 0.5, 11, 0, clip, 1.0, true, true,
        );
        assert_eq!(img_mut.canvas.read_data(16, 16), 10);
    }

    #[test]
    fn test_rasterize_circle_filled_clip_rejects_outside() {
        let (img, mut depth, _) = make_target_and_depth(32, 32);
        let img_mut = rc_mut!(&img);
        let small_clip = ClipRect {
            left: 0,
            top: 0,
            right: 7,
            bottom: 7,
        };
        rasterize_circle_filled(
            img_mut, &mut depth, 32, 3.0, 3.0, 10.0, 0.0, 5, 0, small_clip, 1.0, true, true,
        );
        assert_eq!(img_mut.canvas.read_data(3, 3), 5);
        assert_eq!(img_mut.canvas.read_data(16, 16), 0);
    }
}
