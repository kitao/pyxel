#![allow(clippy::many_single_char_names)]

use crate::cube::camera::Camera;
use crate::cube::color_ramp::ColorRamp;
use crate::cube::light::Light;
use crate::cube::mat4::Mat4;
use crate::cube::vec3::Vec3;
use crate::image::{Image, RcImage};
use crate::tilemap::RcTilemap;

// 4x4 matrix in column-major form (each row is one matrix row, indexed
// `m[row][col]`). Used for the combined view-projection matrix that the
// rasterizer applies to every world-space point.
pub type Mat4x4 = [[f32; 4]; 4];

// Texture source for textured draw commands. Mirrors the cube `sprite` /
// `plane` argument, which accepts either an Image or a Tilemap.
pub enum Texture {
    Image(RcImage),
    Tilemap(RcTilemap),
}

// View-projection helpers

// Build the view matrix as the inverse of the camera's transform.
pub fn view_matrix(camera: &Camera) -> Mat4x4 {
    let inv_rc = rc_ref!(&camera.transform).inverse();
    rc_ref!(&inv_rc).data
}

// Build the projection matrix from camera fov / near / far / aspect, or an
// orthographic projection when `ortho_size` is set.
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

// 4x4 matrix multiplication.
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

// Apply a Mat4 transform to a Vec3 (treats v as a position; full 4x4
// multiply with implicit w=1).
pub fn mat_apply(mat: &Mat4, v: &Vec3) -> Vec3 {
    let rc = mat.mul_vec(v);
    *rc_ref!(&rc)
}

// Project a world-space position through the combined view-projection
// matrix `m` into a screen-space (sx, sy, ndc_z) triple.
//
// Points behind the near plane (cw <= 0) return None. Off-screen points
// (NDC outside [-1, 1]) and points just past the far plane (ndc_z > 1)
// are intentionally still projected so partially off-screen primitives
// keep contributing to in-view pixels via the viewport-rect clip and the
// z-test. Without this, large primitives flicker every time floating-
// point round-off pushes a vertex's NDC z just past 1.0.
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

// Cross product of edges (p1-p0) x (p2-p0) — gives the unscaled face
// normal of the CCW-wound triangle.
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

// Camera right and up axes as world-space directions, taken from the
// camera transform's column vectors. Used by billboard sprites and by
// `screen_circle` to pick a stable edge sample direction.
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

// Shading: pick a palette index from the color ramp based on lighting.
// Output level = clamp(ambient + max(0, dot(face_normal, -light_dir)) *
// intensity) mapped onto the ramp's 16 brightness levels.
pub fn shade(
    color_ramp: &ColorRamp,
    light: &Light,
    base_col: i32,
    normal: Option<&Vec3>,
) -> u8 {
    let palette_size = color_ramp.palette_size();
    if palette_size == 0 {
        return base_col.max(0) as u8;
    }
    let directional = match normal {
        Some(n) => {
            let dir = rc_ref!(&light.direction);
            let n_len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
            let d_len = (dir.x * dir.x + dir.y * dir.y + dir.z * dir.z).sqrt();
            if n_len == 0.0 || d_len == 0.0 {
                0.0
            } else {
                let dot = -(n.x * dir.x + n.y * dir.y + n.z * dir.z) / (n_len * d_len);
                dot.max(0.0) * light.intensity
            }
        }
        None => 0.0,
    };
    let level_f = (light.ambient + directional) * 15.0;
    let level = level_f.clamp(0.0, 15.0).round() as usize;
    let col = base_col.clamp(0, palette_size as i32 - 1) as usize;
    color_ramp.get(col, level) as u8
}

// Clip rect

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

// Compute the pixel-aligned clip rect that clamps the given viewport
// rectangle to the target image bounds.
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

// Pixel write with depth test. Out-of-clip pixels are dropped silently.
#[inline]
pub fn write_pixel(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    x: i32,
    y: i32,
    z: f32,
    col: u8,
    clip: ClipRect,
) {
    if !clip.contains(x, y) {
        return;
    }
    let i = (y as usize) * depth_w as usize + x as usize;
    if z < depth[i] {
        depth[i] = z;
        target.canvas.write_data(x as usize, y as usize, col);
    }
}

// Rasterizers

// Signed area of triangle abc (positive when CCW in screen space, where
// +X is right and +Y is down). Used as a barycentric divisor.
#[inline]
fn edge_function(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

// Filled triangle with linear depth interpolation. Both faces draw
// (no back-face culling — cube spec leaves winding-driven culling out).
// Each pixel center is tested against three edge functions; pixels are
// inside when all three carry the same sign as the triangle's signed
// area.
pub fn rasterize_triangle(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    p0: (f32, f32, f32),
    p1: (f32, f32, f32),
    p2: (f32, f32, f32),
    col: u8,
    clip: ClipRect,
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
                write_pixel(target, depth, depth_w, x, y, z, col, clip);
            }
        }
    }
}

// Bresenham-style 3D line with linear depth interpolation. Both endpoints
// are screen-space (sx, sy, ndc_z). Width is fixed at 1 pixel regardless
// of distance (cube spec: line is distance-independent width).
pub fn rasterize_line(
    target: &mut Image,
    depth: &mut [f32],
    depth_w: u32,
    p0: (f32, f32, f32),
    p1: (f32, f32, f32),
    col: u8,
    clip: ClipRect,
) {
    let x1 = p0.0.round() as i32;
    let y1 = p0.1.round() as i32;
    let x2 = p1.0.round() as i32;
    let y2 = p1.1.round() as i32;
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    if dx == 0 && dy == 0 {
        write_pixel(target, depth, depth_w, x1, y1, p0.2, col, clip);
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
        write_pixel(
            target,
            depth,
            depth_w,
            fx.round() as i32,
            fy.round() as i32,
            fz,
            col,
            clip,
        );
        fx += dx_f;
        fy += dy_f;
        fz += dz_f;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::camera::Camera;

    fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
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
    fn test_world_to_screen_center() {
        // Origin point with a simple identity-like view-projection: place
        // the origin at the center of the viewport.
        let camera = Camera::new();
        let v = view_matrix(rc_ref!(&camera));
        let p = projection_matrix(rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        // Default camera looks down -Z; pick a point at (0,0,-2) which is
        // in front of the camera.
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
    fn test_shade_no_normal_uses_ambient() {
        let ramp = ColorRamp::new();
        let ramp = rc_ref!(&ramp);
        let light = Light::new();
        let light_mut = rc_mut!(&light);
        light_mut.ambient = 1.0;
        light_mut.intensity = 1.0;
        // ambient=1.0 + no normal -> level 15 (full brightness).
        // For col 7, level 15 should map back to col 7 itself.
        let col = shade(ramp, light_mut, 7, None);
        assert_eq!(col, 7);
    }

    #[test]
    fn test_shade_zero_brightness() {
        let ramp = ColorRamp::new();
        let ramp = rc_ref!(&ramp);
        let light = Light::new();
        let light_mut = rc_mut!(&light);
        light_mut.ambient = 0.0;
        light_mut.intensity = 0.0;
        // brightness=0 -> level 0 -> darkest entry (typically col 0 in
        // default Pyxel palette).
        let col = shade(ramp, light_mut, 7, None);
        // Just verify it is a valid palette index, exact value depends on
        // ramp's nearest-color algorithm.
        assert!(col < 64);
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
        // Viewport extends past the target — clip clamps to target bounds.
        let clip = compute_clip_rect(0.0, 0.0, 300.0, 200.0, 256, 192);
        assert_eq!(clip.left, 0);
        assert_eq!(clip.top, 0);
        assert_eq!(clip.right, 255);
        assert_eq!(clip.bottom, 191);
    }

    #[test]
    fn test_compute_clip_rect_offset_viewport() {
        // Viewport in upper-right corner, fully inside target.
        let clip = compute_clip_rect(64.0, 0.0, 64.0, 48.0, 256, 192);
        assert_eq!(clip.left, 64);
        assert_eq!(clip.top, 0);
        assert_eq!(clip.right, 127);
        assert_eq!(clip.bottom, 47);
    }

    #[test]
    fn test_compute_clip_rect_negative_origin_clamped() {
        // Negative viewport origin clamps to 0.
        let clip = compute_clip_rect(-10.0, -5.0, 100.0, 100.0, 256, 192);
        assert_eq!(clip.left, 0);
        assert_eq!(clip.top, 0);
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
    fn test_write_pixel_inside_clip() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let img_mut = rc_mut!(&img);
        write_pixel(img_mut, &mut depth, 8, 3, 3, 0.0, 5, clip);
        assert_eq!(img_mut.canvas.read_data(3, 3), 5);
        assert_eq!(depth[3 * 8 + 3], 0.0);
    }

    #[test]
    fn test_write_pixel_outside_clip_dropped() {
        let (img, mut depth, _) = make_target_and_depth(8, 8);
        let img_mut = rc_mut!(&img);
        let small_clip = ClipRect {
            left: 0,
            top: 0,
            right: 1,
            bottom: 1,
        };
        write_pixel(img_mut, &mut depth, 8, 5, 5, 0.0, 7, small_clip);
        // Pixel at (5,5) was never written because clip excludes it.
        assert_eq!(img_mut.canvas.read_data(5, 5), 0);
        assert_eq!(depth[5 * 8 + 5], f32::INFINITY);
    }

    #[test]
    fn test_write_pixel_z_test_blocks_far() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let img_mut = rc_mut!(&img);
        // Near pixel first.
        write_pixel(img_mut, &mut depth, 8, 4, 4, 0.0, 10, clip);
        // Farther write attempt — should be rejected.
        write_pixel(img_mut, &mut depth, 8, 4, 4, 0.5, 11, clip);
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
            clip,
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
            clip,
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
            clip,
        );
        assert_eq!(img_mut.canvas.read_data(3, 3), 5);
        assert!((depth[3 * 8 + 3] - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_rasterize_line_depth_z_test() {
        let (img, mut depth, clip) = make_target_and_depth(8, 8);
        let img_mut = rc_mut!(&img);
        // Closer (z=0.0) line drawn first.
        rasterize_line(
            img_mut,
            &mut depth,
            8,
            (0.0, 0.0, 0.0),
            (4.0, 0.0, 0.0),
            10,
            clip,
        );
        // Farther (z=0.5) line on the same span — must not overwrite.
        rasterize_line(
            img_mut,
            &mut depth,
            8,
            (0.0, 0.0, 0.5),
            (4.0, 0.0, 0.5),
            11,
            clip,
        );
        for x in 0..=4 {
            assert_eq!(img_mut.canvas.read_data(x, 0), 10);
        }
    }

    #[test]
    fn test_rasterize_triangle_fills_interior() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let img_mut = rc_mut!(&img);
        // Right triangle with vertices at (2,2), (12,2), (2,12), CCW.
        rasterize_triangle(
            img_mut,
            &mut depth,
            16,
            (2.0, 2.0, 0.0),
            (12.0, 2.0, 0.0),
            (2.0, 12.0, 0.0),
            9,
            clip,
        );
        // A point clearly inside the triangle should be filled.
        assert_eq!(img_mut.canvas.read_data(4, 4), 9);
        // A point outside the hypotenuse should be untouched.
        assert_eq!(img_mut.canvas.read_data(10, 10), 0);
    }

    #[test]
    fn test_rasterize_triangle_z_interpolated() {
        let (img, mut depth, clip) = make_target_and_depth(16, 16);
        let img_mut = rc_mut!(&img);
        // Triangle with z=0.0 at p0, z=0.5 at p1, z=1.0 at p2.
        rasterize_triangle(
            img_mut,
            &mut depth,
            16,
            (0.0, 0.0, 0.0),
            (15.0, 0.0, 0.5),
            (0.0, 15.0, 1.0),
            7,
            clip,
        );
        // Depth at the (0,0) corner approximates p0's z.
        assert!(depth[0] < 0.1);
        // Depth far from p0 trends toward 0.5 / 1.0 at the other corners.
        assert!(depth[15] > 0.3);
    }

    #[test]
    fn test_rasterize_triangle_back_face_also_drawn() {
        // CW winding (negative signed area) still rasterizes — cube has no
        // back-face culling.
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
            clip,
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
            clip,
        );
        // No pixel was written to color 5.
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
            small_clip,
        );
        // Pixel inside clip filled.
        assert_eq!(img_mut.canvas.read_data(2, 2), 6);
        // Pixel outside clip untouched.
        assert_eq!(img_mut.canvas.read_data(10, 1), 0);
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
            small_clip,
        );
        // Pixels 0..=3 written, 4..=7 dropped.
        for x in 0..=3 {
            assert_eq!(img_mut.canvas.read_data(x, 0), 5);
        }
        for x in 4..=7 {
            assert_eq!(img_mut.canvas.read_data(x, 0), 0);
        }
    }
}
