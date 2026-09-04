// Drawing formulas use conventional x/y/z/u/v component names.
#![allow(clippy::many_single_char_names)]

// World-space geometry shares the `prim` path; circles and text rasterize in screen space.

use std::sync::OnceLock;

use crate::cube::camera::RcCamera;
use crate::cube::mat4::Mat4;
use crate::cube::primitive::{
    self, Primitive, CULL_BACK, CULL_FRONT, CULL_NONE, MODE_LINES, MODE_POINTS, MODE_TRIANGLES,
};
use crate::cube::raster::{
    dither_pick, face_shade_level, lookup_ramp, mat_apply, mat_apply_dir, rasterize_circle_border,
    rasterize_circle_filled, rasterize_line, rasterize_textured_triangle, rasterize_triangle,
    screen_circle, sprite_corners, tri_normal, world_to_screen, write_pixel, ELLIPSE_SEGMENTS,
};
use crate::cube::scene::DrawContext;
use crate::cube::shading::Shading;
use crate::cube::vec3::Vec3;
use crate::font::Font;
use crate::image::{Image, RcImage};
use crate::settings::{FONT_HEIGHT, FONT_WIDTH, MAX_FONT_CODE, MIN_FONT_CODE, NUM_FONT_COLS};

const CLIP_FRONT_EPSILON: f32 = 1e-4;
type ScreenPoint = (f32, f32, f32);

#[derive(Clone, Copy)]
struct ClipVertex {
    world: Vec3,
    uv: (f32, f32),
}

#[derive(Clone, Copy)]
struct ProjectedClipVertex {
    screen: ScreenPoint,
    uv: (f32, f32),
}

#[derive(Clone, Copy)]
struct ClippedTriangle {
    vertices: [ClipVertex; 4],
    len: usize,
}

#[derive(Clone, Copy)]
struct ProjectedPolygon {
    vertices: [ProjectedClipVertex; 4],
    len: usize,
}

// Internal billboard modes for camera-facing draw commands
pub const BILLBOARD_OFF: i32 = 0;
pub const BILLBOARD_ON: i32 = 1;

pub type Uvs = ((f32, f32), (f32, f32), (f32, f32), (f32, f32));

// Per-call shading, depth, dither, and billboard state passed to rasterizers
#[derive(Clone, Copy)]
pub struct DrawState<'a> {
    pub shaded: bool,
    pub dither_alpha: f32,
    pub depth_test: bool,
    pub depth_write: bool,
    pub billboard: i32,
    pub shading: Option<&'a Shading>,
}

impl DrawState<'_> {
    pub fn unshaded() -> Self {
        Self {
            shaded: false,
            dither_alpha: 1.0,
            depth_test: true,
            depth_write: true,
            billboard: BILLBOARD_OFF,
            shading: None,
        }
    }
}

// Signed area of the triangle in Y-down screen space. Front faces (CCW in
// world space) project to a negative area; back faces project to positive.
#[inline]
fn signed_screen_area(p0: (f32, f32, f32), p1: (f32, f32, f32), p2: (f32, f32, f32)) -> f32 {
    (p1.0 - p0.0) * (p2.1 - p0.1) - (p1.1 - p0.1) * (p2.0 - p0.0)
}

// Decide whether to skip a face under the given cull mode. Degenerate
// faces (area == 0) are skipped under any non-NONE cull, matching the
// convention that they have no front side to draw.
#[inline]
fn should_cull(area: f32, cull: i32) -> bool {
    (cull == CULL_BACK && area >= 0.0) || (cull == CULL_FRONT && area <= 0.0)
}

fn prepare_draw(ctx: &mut DrawContext, world_mat: &Mat4, state: &DrawState) -> Mat4 {
    ctx.dither_alpha = state.dither_alpha.clamp(0.0, 1.0);
    ctx.depth_test = state.depth_test;
    ctx.depth_write = state.depth_write;
    apply_billboard(world_mat, ctx, state.billboard)
}

// The camera's viewing direction is its camera-to-world transform's -Z column.
// Positive offsets push depth away from the camera.
fn depth_offset_shift(camera: &RcCamera, offset: f32) -> Vec3 {
    if offset == 0.0 {
        return Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    let cam = rc_ref!(camera);
    let m = rc_ref!(&cam.transform).data;
    Vec3 {
        x: -m[0][2] * offset,
        y: -m[1][2] * offset,
        z: -m[2][2] * offset,
    }
}

// Only depth changes; the original screen position and size are preserved.
// A zero shift avoids the second projection.
fn project_offset(
    pos: &Vec3,
    vp: &[[f32; 4]; 4],
    clip_row: &[f32; 4],
    vp_x: f32,
    vp_y: f32,
    vp_w: f32,
    vp_h: f32,
    shift: &Vec3,
) -> Option<(f32, f32, f32)> {
    let p = world_to_screen(pos, vp, clip_row, vp_x, vp_y, vp_w, vp_h)?;
    if shift.x == 0.0 && shift.y == 0.0 && shift.z == 0.0 {
        return Some(p);
    }
    let shifted = Vec3 {
        x: pos.x + shift.x,
        y: pos.y + shift.y,
        z: pos.z + shift.z,
    };
    match world_to_screen(&shifted, vp, clip_row, vp_x, vp_y, vp_w, vp_h) {
        Some(d) => Some((p.0, p.1, d.2)),
        None => Some(p),
    }
}

// View-space distance in front of the camera plane (raster::camera_clip_row).
fn clip_front(pos: &Vec3, clip_row: &[f32; 4]) -> f32 {
    clip_row[0] * pos.x + clip_row[1] * pos.y + clip_row[2] * pos.z + clip_row[3]
}

fn lerp_world(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
    Vec3 {
        x: a.x + (b.x - a.x) * t,
        y: a.y + (b.y - a.y) * t,
        z: a.z + (b.z - a.z) * t,
    }
}

fn lerp_clip_vertex(a: ClipVertex, b: ClipVertex, t: f32) -> ClipVertex {
    ClipVertex {
        world: lerp_world(&a.world, &b.world, t),
        uv: (
            a.uv.0 + (b.uv.0 - a.uv.0) * t,
            a.uv.1 + (b.uv.1 - a.uv.1) * t,
        ),
    }
}

fn clip_triangle_to_near(vertices: [ClipVertex; 3], clip_row: &[f32; 4]) -> ClippedTriangle {
    let mut clipped = ClippedTriangle {
        vertices: [vertices[0]; 4],
        len: 0,
    };
    for i in 0..3 {
        let prev = vertices[(i + 2) % 3];
        let curr = vertices[i];
        let prev_front = clip_front(&prev.world, clip_row);
        let curr_front = clip_front(&curr.world, clip_row);
        let prev_inside = prev_front > CLIP_FRONT_EPSILON;
        let curr_inside = curr_front > CLIP_FRONT_EPSILON;

        if prev_inside != curr_inside {
            let t = (CLIP_FRONT_EPSILON - prev_front) / (curr_front - prev_front);
            clipped.vertices[clipped.len] = lerp_clip_vertex(prev, curr, t);
            clipped.len += 1;
        }
        if curr_inside {
            clipped.vertices[clipped.len] = curr;
            clipped.len += 1;
        }
    }
    clipped
}

fn project_clipped_vertices(
    vertices: &ClippedTriangle,
    ctx: &DrawContext,
    z_shift: &Vec3,
) -> Option<ProjectedPolygon> {
    let empty = ProjectedClipVertex {
        screen: (0.0, 0.0, 0.0),
        uv: (0.0, 0.0),
    };
    let mut out = ProjectedPolygon {
        vertices: [empty; 4],
        len: 0,
    };
    for i in 0..vertices.len {
        let vertex = vertices.vertices[i];
        let screen = project_offset(
            &vertex.world,
            &ctx.vp,
            &ctx.clip_row,
            ctx.vp_x,
            ctx.vp_y,
            ctx.vp_w,
            ctx.vp_h,
            z_shift,
        )?;
        out.vertices[out.len] = ProjectedClipVertex {
            screen,
            uv: vertex.uv,
        };
        out.len += 1;
    }
    Some(out)
}

fn draw_projected_triangle(
    ctx: &mut DrawContext,
    vertices: [ProjectedClipVertex; 3],
    cull: i32,
    normal: Option<&Vec3>,
    col_flat: i32,
    col_image: Option<&RcImage>,
    colkey: Option<i32>,
    state: DrawState,
) {
    let [a, b, c] = vertices;
    if cull != CULL_NONE {
        let area = signed_screen_area(a.screen, b.screen, c.screen);
        if should_cull(area, cull) {
            return;
        }
    }

    let depth_w = ctx.depth_w;
    let clip = ctx.clip;
    let dither_alpha = ctx.dither_alpha;
    let depth_test = ctx.depth_test;
    let depth_write = ctx.depth_write;

    if let Some(img_rc) = col_image {
        let img_ref = rc_ref!(img_rc);
        if img_ref.width() == 0 || img_ref.height() == 0 {
            return;
        }
        if let Some(normal) = normal {
            let shading = state.shading.unwrap();
            let direction = rc_ref!(&shading.direction);
            let level = face_shade_level(&direction, Some(normal));
            let sampler = make_shaded_sampler(&img_ref, shading, level, colkey);
            let mut target_mut = rc_mut!(&ctx.target);
            let depth = ctx.depth.as_mut_slice();
            rasterize_textured_triangle(
                &mut target_mut,
                depth,
                depth_w,
                a.screen,
                b.screen,
                c.screen,
                a.uv,
                b.uv,
                c.uv,
                &sampler,
                clip,
                dither_alpha,
                depth_test,
                depth_write,
            );
        } else {
            let sampler = make_image_sampler(&img_ref, colkey);
            let mut target_mut = rc_mut!(&ctx.target);
            let depth = ctx.depth.as_mut_slice();
            rasterize_textured_triangle(
                &mut target_mut,
                depth,
                depth_w,
                a.screen,
                b.screen,
                c.screen,
                a.uv,
                b.uv,
                c.uv,
                &sampler,
                clip,
                dither_alpha,
                depth_test,
                depth_write,
            );
        }
    } else {
        let entry = match normal {
            Some(normal) => lookup_ramp(state.shading.unwrap(), col_flat, Some(normal)),
            None => (col_flat, col_flat),
        };
        let mut target_mut = rc_mut!(&ctx.target);
        let depth = ctx.depth.as_mut_slice();
        rasterize_triangle(
            &mut target_mut,
            depth,
            depth_w,
            a.screen,
            b.screen,
            c.screen,
            entry.0 as u8,
            entry.1 as u8,
            clip,
            dither_alpha,
            depth_test,
            depth_write,
        );
    }
}

fn project_line_segment(
    p0: &Vec3,
    p1: &Vec3,
    ctx: &DrawContext,
    z_shift: &Vec3,
) -> Option<(ScreenPoint, ScreenPoint)> {
    let front0 = clip_front(p0, &ctx.clip_row);
    let front1 = clip_front(p1, &ctx.clip_row);
    if front0 <= CLIP_FRONT_EPSILON && front1 <= CLIP_FRONT_EPSILON {
        return None;
    }

    let mut q0 = *p0;
    let mut q1 = *p1;
    if front0 <= CLIP_FRONT_EPSILON {
        let t = (CLIP_FRONT_EPSILON - front0) / (front1 - front0);
        q0 = lerp_world(p0, p1, t);
    } else if front1 <= CLIP_FRONT_EPSILON {
        let t = (CLIP_FRONT_EPSILON - front0) / (front1 - front0);
        q1 = lerp_world(p0, p1, t);
    }

    let s0 = project_offset(
        &q0,
        &ctx.vp,
        &ctx.clip_row,
        ctx.vp_x,
        ctx.vp_y,
        ctx.vp_w,
        ctx.vp_h,
        z_shift,
    )?;
    let s1 = project_offset(
        &q1,
        &ctx.vp,
        &ctx.clip_row,
        ctx.vp_x,
        ctx.vp_y,
        ctx.vp_w,
        ctx.vp_h,
        z_shift,
    )?;
    Some((s0, s1))
}

fn apply_billboard(world_mat: &Mat4, ctx: &DrawContext, mode: i32) -> Mat4 {
    if mode == BILLBOARD_OFF {
        return *world_mat;
    }
    let cam = rc_ref!(&ctx.camera);
    let cam_world = *rc_ref!(&cam.transform);
    // Camera basis (columns of cam_world.rot block).
    let cam_x = Vec3 {
        x: cam_world.data[0][0],
        y: cam_world.data[1][0],
        z: cam_world.data[2][0],
    };
    let cam_y = Vec3 {
        x: cam_world.data[0][1],
        y: cam_world.data[1][1],
        z: cam_world.data[2][1],
    };
    let cam_z = Vec3 {
        x: cam_world.data[0][2],
        y: cam_world.data[1][2],
        z: cam_world.data[2][2],
    };
    // Recover translation and scale from the original world matrix; we
    // only override the rotation part for billboard alignment.
    let pos = Vec3 {
        x: world_mat.data[0][3],
        y: world_mat.data[1][3],
        z: world_mat.data[2][3],
    };
    let scale_x = (world_mat.data[0][0].powi(2)
        + world_mat.data[1][0].powi(2)
        + world_mat.data[2][0].powi(2))
    .sqrt();
    let scale_y = (world_mat.data[0][1].powi(2)
        + world_mat.data[1][1].powi(2)
        + world_mat.data[2][1].powi(2))
    .sqrt();
    let scale_z = (world_mat.data[0][2].powi(2)
        + world_mat.data[1][2].powi(2)
        + world_mat.data[2][2].powi(2))
    .sqrt();
    // Spherical (BILLBOARD_ON): adopt camera basis directly.
    let mut out = Mat4::identity_value();
    out.data[0][0] = cam_x.x * scale_x;
    out.data[1][0] = cam_x.y * scale_x;
    out.data[2][0] = cam_x.z * scale_x;
    out.data[0][1] = cam_y.x * scale_y;
    out.data[1][1] = cam_y.y * scale_y;
    out.data[2][1] = cam_y.z * scale_y;
    out.data[0][2] = cam_z.x * scale_z;
    out.data[1][2] = cam_z.y * scale_z;
    out.data[2][2] = cam_z.z * scale_z;
    out.data[0][3] = pos.x;
    out.data[1][3] = pos.y;
    out.data[2][3] = pos.z;
    out
}

// Image samplers used by textured prim TRIANGLES

fn make_image_sampler(
    img: &Image,
    colkey: Option<i32>,
) -> impl Fn(f32, f32, i32, i32) -> Option<i32> + '_ {
    let w = img.width() as f32;
    let h = img.height() as f32;
    let max_x = (img.width() as i32 - 1).max(0);
    let max_y = (img.height() as i32 - 1).max(0);
    move |u, v, _x, _y| {
        let xi = (u * w) as i32;
        let yi = (v * h) as i32;
        let xi = xi.clamp(0, max_x);
        let yi = yi.clamp(0, max_y);
        // Indices are already clamped; read directly instead of re-rounding
        // and re-clipping through pixel().
        let col = i32::from(img.canvas.read_data(xi as usize, yi as usize));
        colkey.is_none_or(|key| col != key).then_some(col)
    }
}

fn make_shaded_sampler<'a>(
    img: &'a Image,
    shading: &'a Shading,
    level: usize,
    colkey: Option<i32>,
) -> impl Fn(f32, f32, i32, i32) -> Option<i32> + 'a {
    let sample = make_image_sampler(img, colkey);
    let palette_size = shading.palette_size();
    move |u, v, x, y| {
        let base = sample(u, v, x, y)?;
        if palette_size == 0 {
            Some(base)
        } else {
            let base_idx = base.clamp(0, palette_size as i32 - 1) as usize;
            let (primary, secondary) = shading.get(base_idx, level);
            Some(i32::from(dither_pick(primary, secondary, x, y)))
        }
    }
}

pub fn prim(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    mode: i32,
    cull: i32,
    positions: &[f32],
    indices: Option<&[i32]>,
    normals: Option<&[f32]>,
    uvs: Option<&[f32]>,
    col_flat: i32,
    col_image: Option<&RcImage>,
    colkey: Option<i32>,
    state: DrawState,
) -> Result<(), &'static str> {
    if !positions.len().is_multiple_of(3) {
        return Err("positions length must be a multiple of 3");
    }
    let vertex_count = positions.len() / 3;
    if let Some(uvs) = uvs {
        if uvs.len() != vertex_count * 2 {
            return Err("uvs length must equal vertex_count * 2");
        }
    }
    let step_count = match indices {
        Some(idx) => idx.len(),
        None => vertex_count,
    };
    match mode {
        MODE_TRIANGLES if !step_count.is_multiple_of(3) => {
            return Err(if indices.is_some() {
                "indices length must be a multiple of 3 for MODE_TRIANGLES"
            } else {
                "positions vertex count must be a multiple of 3 for MODE_TRIANGLES"
            });
        }
        MODE_LINES if !step_count.is_multiple_of(2) => {
            return Err(if indices.is_some() {
                "indices length must be a multiple of 2 for MODE_LINES"
            } else {
                "positions vertex count must be a multiple of 2 for MODE_LINES"
            });
        }
        _ => {}
    }
    let world_mat = prepare_draw(ctx, world_mat, &state);
    let z_shift = depth_offset_shift(&ctx.camera, ctx.depth_offset);
    let lit = state.shaded && state.shading.is_some();
    // Cache shared indexed vertices. Lines are projected after clipping.
    ctx.vertex_cache.clear();
    ctx.vertex_cache.reserve(vertex_count);
    for i in 0..vertex_count {
        let base = i * 3;
        let local = Vec3 {
            x: positions[base],
            y: positions[base + 1],
            z: positions[base + 2],
        };
        let world = mat_apply(&world_mat, &local);
        let screen = if mode == MODE_LINES {
            None
        } else {
            project_offset(
                &world,
                &ctx.vp,
                &ctx.clip_row,
                ctx.vp_x,
                ctx.vp_y,
                ctx.vp_w,
                ctx.vp_h,
                &z_shift,
            )
        };
        ctx.vertex_cache.push((world, screen));
    }
    let resolve_vertex_index = |step: usize| -> Result<usize, &'static str> {
        let raw = match indices {
            Some(idx) => idx[step],
            None => step as i32,
        };
        if raw < 0 || (raw as usize) >= vertex_count {
            return Err("indices must be within vertex range");
        }
        Ok(raw as usize)
    };
    match mode {
        MODE_TRIANGLES => {
            let face_count = step_count / 3;
            if let Some(n) = normals {
                if lit && n.len() != face_count * 3 {
                    return Err("normals length must equal face_count * 3 when shaded");
                }
            }
            if col_image.is_some() && uvs.is_none() {
                return Err("textured prim requires uvs");
            }
            // Cofactors carry oriented face normals through reflections and
            // singular scales: cross(Mu, Mv) = cofactor(M) cross(u, v).
            let normal_mat = (lit && normals.is_some()).then(|| {
                let m = &world_mat.data;
                Mat4 {
                    data: [
                        [
                            m[1][1] * m[2][2] - m[1][2] * m[2][1],
                            m[1][2] * m[2][0] - m[1][0] * m[2][2],
                            m[1][0] * m[2][1] - m[1][1] * m[2][0],
                            0.0,
                        ],
                        [
                            m[0][2] * m[2][1] - m[0][1] * m[2][2],
                            m[0][0] * m[2][2] - m[0][2] * m[2][0],
                            m[0][1] * m[2][0] - m[0][0] * m[2][1],
                            0.0,
                        ],
                        [
                            m[0][1] * m[1][2] - m[0][2] * m[1][1],
                            m[0][2] * m[1][0] - m[0][0] * m[1][2],
                            m[0][0] * m[1][1] - m[0][1] * m[1][0],
                            0.0,
                        ],
                        [0.0, 0.0, 0.0, 1.0],
                    ],
                }
            });
            for f in 0..face_count {
                let i0 = resolve_vertex_index(f * 3)?;
                let i1 = resolve_vertex_index(f * 3 + 1)?;
                let i2 = resolve_vertex_index(f * 3 + 2)?;
                let (v0, p0) = ctx.vertex_cache[i0];
                let (v1, p1) = ctx.vertex_cache[i1];
                let (v2, p2) = ctx.vertex_cache[i2];
                let face_normal = || -> Vec3 {
                    match (normals, normal_mat.as_ref()) {
                        (Some(n), Some(mat)) => mat_apply_dir(
                            mat,
                            &Vec3 {
                                x: n[f * 3],
                                y: n[f * 3 + 1],
                                z: n[f * 3 + 2],
                            },
                        ),
                        _ => tri_normal(&v0, &v1, &v2),
                    }
                };
                let uv0 = uvs.map_or((0.0, 0.0), |uvs| (uvs[i0 * 2], uvs[i0 * 2 + 1]));
                let uv1 = uvs.map_or((0.0, 0.0), |uvs| (uvs[i1 * 2], uvs[i1 * 2 + 1]));
                let uv2 = uvs.map_or((0.0, 0.0), |uvs| (uvs[i2 * 2], uvs[i2 * 2 + 1]));
                let normal = lit.then(face_normal);

                if let (Some(p0), Some(p1), Some(p2)) = (p0, p1, p2) {
                    draw_projected_triangle(
                        ctx,
                        [
                            ProjectedClipVertex {
                                screen: p0,
                                uv: uv0,
                            },
                            ProjectedClipVertex {
                                screen: p1,
                                uv: uv1,
                            },
                            ProjectedClipVertex {
                                screen: p2,
                                uv: uv2,
                            },
                        ],
                        cull,
                        normal.as_ref(),
                        col_flat,
                        col_image,
                        colkey,
                        state,
                    );
                } else {
                    let clipped = clip_triangle_to_near(
                        [
                            ClipVertex { world: v0, uv: uv0 },
                            ClipVertex { world: v1, uv: uv1 },
                            ClipVertex { world: v2, uv: uv2 },
                        ],
                        &ctx.clip_row,
                    );
                    if clipped.len < 3 {
                        continue;
                    }
                    let Some(projected) = project_clipped_vertices(&clipped, ctx, &z_shift) else {
                        continue;
                    };
                    for i in 1..projected.len - 1 {
                        draw_projected_triangle(
                            ctx,
                            [
                                projected.vertices[0],
                                projected.vertices[i],
                                projected.vertices[i + 1],
                            ],
                            cull,
                            normal.as_ref(),
                            col_flat,
                            col_image,
                            colkey,
                            state,
                        );
                    }
                }
            }
        }
        MODE_LINES => {
            let line_count = step_count / 2;
            let depth_w = ctx.depth_w;
            for l in 0..line_count {
                let i0 = resolve_vertex_index(l * 2)?;
                let i1 = resolve_vertex_index(l * 2 + 1)?;
                let (w0, _) = ctx.vertex_cache[i0];
                let (w1, _) = ctx.vertex_cache[i1];
                if let Some((p0, p1)) = project_line_segment(&w0, &w1, ctx, &z_shift) {
                    let mut target_mut = rc_mut!(&ctx.target);
                    let depth = ctx.depth.as_mut_slice();
                    rasterize_line(
                        &mut target_mut,
                        depth,
                        depth_w,
                        p0,
                        p1,
                        col_flat as u8,
                        col_flat as u8,
                        ctx.clip,
                        ctx.dither_alpha,
                        ctx.depth_test,
                        ctx.depth_write,
                    );
                }
            }
        }
        MODE_POINTS => {
            let mut target_mut = rc_mut!(&ctx.target);
            let depth_w = ctx.depth_w;
            let depth = ctx.depth.as_mut_slice();
            for s in 0..step_count {
                let i0 = resolve_vertex_index(s)?;
                let (_, p0) = ctx.vertex_cache[i0];
                if let Some((sx, sy, sz)) = p0 {
                    let xi = sx.round() as i32;
                    let yi = sy.round() as i32;
                    if ctx.clip.contains(xi, yi) {
                        write_pixel(
                            &mut target_mut,
                            depth,
                            depth_w,
                            xi,
                            yi,
                            sz,
                            col_flat as u8,
                            ctx.dither_alpha,
                            ctx.depth_test,
                            ctx.depth_write,
                        );
                    }
                }
            }
        }
        _ => return Err("mode must be MODE_TRIANGLES, MODE_LINES, or MODE_POINTS"),
    }
    Ok(())
}

// Shortcut commands fabricate buffers and route through prim

pub fn pset(ctx: &mut DrawContext, world_mat: &Mat4, local: &Vec3, col: i32, state: DrawState) {
    let positions = [local.x, local.y, local.z];
    let _ = prim(
        ctx,
        world_mat,
        MODE_POINTS,
        CULL_NONE,
        &positions,
        None,
        None,
        None,
        col,
        None,
        None,
        state,
    );
}

pub fn line(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    p1: &Vec3,
    p2: &Vec3,
    col: i32,
    state: DrawState,
) {
    let positions = [p1.x, p1.y, p1.z, p2.x, p2.y, p2.z];
    let _ = prim(
        ctx, world_mat, MODE_LINES, CULL_NONE, &positions, None, None, None, col, None, None, state,
    );
}

pub fn tri(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    p1: &Vec3,
    p2: &Vec3,
    p3: &Vec3,
    col: i32,
    state: DrawState,
) {
    let positions = [p1.x, p1.y, p1.z, p2.x, p2.y, p2.z, p3.x, p3.y, p3.z];
    let _ = prim(
        ctx,
        world_mat,
        MODE_TRIANGLES,
        CULL_NONE,
        &positions,
        None,
        None,
        None,
        col,
        None,
        None,
        state,
    );
}

pub fn trib(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    p1: &Vec3,
    p2: &Vec3,
    p3: &Vec3,
    col: i32,
    state: DrawState,
) {
    let positions = [
        p1.x, p1.y, p1.z, p2.x, p2.y, p2.z, p2.x, p2.y, p2.z, p3.x, p3.y, p3.z, p3.x, p3.y, p3.z,
        p1.x, p1.y, p1.z,
    ];
    let _ = prim(
        ctx, world_mat, MODE_LINES, CULL_NONE, &positions, None, None, None, col, None, None, state,
    );
}

// rect / rectb lay out the rectangle in world_mat's local XY plane.
pub fn rect(ctx: &mut DrawContext, world_mat: &Mat4, w: f32, h: f32, col: i32, state: DrawState) {
    let scaled = scale_axes(world_mat, w * 0.5, h * 0.5, 1.0);
    let _ = prim(
        ctx,
        &scaled,
        MODE_TRIANGLES,
        CULL_NONE,
        &UNIT_RECT_POSITIONS,
        Some(&RECT_TRI_INDICES),
        None,
        None,
        col,
        None,
        None,
        state,
    );
}

pub fn rectb(ctx: &mut DrawContext, world_mat: &Mat4, w: f32, h: f32, col: i32, state: DrawState) {
    let scaled = scale_axes(world_mat, w * 0.5, h * 0.5, 1.0);
    let _ = prim(
        ctx,
        &scaled,
        MODE_LINES,
        CULL_NONE,
        &UNIT_RECT_POSITIONS,
        Some(&RECT_EDGE_INDICES),
        None,
        None,
        col,
        None,
        None,
        state,
    );
}

pub fn elli(ctx: &mut DrawContext, world_mat: &Mat4, w: f32, h: f32, col: i32, state: DrawState) {
    let scaled = scale_axes(world_mat, w * 0.5, h * 0.5, 1.0);
    let _ = prim(
        ctx,
        &scaled,
        MODE_TRIANGLES,
        CULL_NONE,
        unit_ellipse_positions(),
        Some(&ELLIPSE_TRI_INDICES),
        None,
        None,
        col,
        None,
        None,
        state,
    );
}

pub fn ellib(ctx: &mut DrawContext, world_mat: &Mat4, w: f32, h: f32, col: i32, state: DrawState) {
    let scaled = scale_axes(world_mat, w * 0.5, h * 0.5, 1.0);
    let _ = prim(
        ctx,
        &scaled,
        MODE_LINES,
        CULL_NONE,
        unit_ellipse_positions(),
        Some(&ELLIPSE_EDGE_INDICES),
        None,
        None,
        col,
        None,
        None,
        state,
    );
}

fn primitive_normals(g: &Primitive) -> Option<&[f32]> {
    if g.normals.is_empty() {
        None
    } else {
        Some(g.normals.as_slice())
    }
}

fn primitive_uvs(g: &Primitive) -> Option<&[f32]> {
    if g.uvs.is_empty() {
        None
    } else {
        Some(g.uvs.as_slice())
    }
}

pub fn box_solid(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    size: &Vec3,
    col_flat: i32,
    col_image: Option<&RcImage>,
    colkey: Option<i32>,
    state: DrawState,
) {
    let scaled = scale_axes(world_mat, size.x, size.y, size.z);
    let g = if col_image.is_some() {
        primitive::unit_box_textured()
    } else {
        primitive::unit_box_solid()
    };
    let uvs = if col_image.is_some() {
        primitive_uvs(g)
    } else {
        None
    };
    let _ = prim(
        ctx,
        &scaled,
        g.mode,
        g.cull,
        g.positions.as_slice(),
        Some(g.indices.as_slice()),
        primitive_normals(g),
        uvs,
        col_flat,
        col_image,
        colkey,
        state,
    );
}

pub fn boxb(ctx: &mut DrawContext, world_mat: &Mat4, size: &Vec3, col: i32, state: DrawState) {
    let scaled = scale_axes(world_mat, size.x, size.y, size.z);
    let g = primitive::unit_box_solid();
    let _ = prim(
        ctx,
        &scaled,
        MODE_LINES,
        CULL_NONE,
        g.positions.as_slice(),
        Some(&primitive::BOX_EDGE_INDICES),
        None,
        None,
        col,
        None,
        None,
        state,
    );
}

// Scale the world matrix to reuse cached unit geometry without allocating
// vertex arrays for each draw.

// Unit rectangle winding matches the plane primitive.
const UNIT_RECT_POSITIONS: [f32; 12] = [
    -1.0, 1.0, 0.0, // top-left
    1.0, 1.0, 0.0, // top-right
    -1.0, -1.0, 0.0, // bottom-left
    1.0, -1.0, 0.0, // bottom-right
];
const RECT_TRI_INDICES: [i32; 6] = [0, 1, 2, 1, 3, 2];
const RECT_EDGE_INDICES: [i32; 8] = [0, 1, 1, 3, 3, 2, 2, 0];

// Vertex 0 is the center; vertices 1..=ELLIPSE_SEGMENTS form the perimeter.
const ELLIPSE_TRI_INDICES: [i32; 72] = [
    0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 7, 0, 7, 8, 0, 8, 9, 0, 9, 10, 0, 10, 11, 0,
    11, 12, 0, 12, 13, 0, 13, 14, 0, 14, 15, 0, 15, 16, 0, 16, 17, 0, 17, 18, 0, 18, 19, 0, 19, 20,
    0, 20, 21, 0, 21, 22, 0, 22, 23, 0, 23, 24, 0, 24, 1,
];
const ELLIPSE_EDGE_INDICES: [i32; 48] = [
    1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15,
    15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 1,
];

fn unit_ellipse_positions() -> &'static [f32; 75] {
    static POSITIONS: OnceLock<[f32; 75]> = OnceLock::new();
    POSITIONS.get_or_init(|| {
        let mut p = [0.0_f32; 75];
        for i in 0..ELLIPSE_SEGMENTS {
            let theta = 2.0 * std::f32::consts::PI * (i as f32) / (ELLIPSE_SEGMENTS as f32);
            let base = (i + 1) * 3;
            p[base] = theta.cos();
            p[base + 1] = theta.sin();
        }
        p
    })
}

// Scale the linear part while preserving translation.
fn scale_axes(world_mat: &Mat4, sx: f32, sy: f32, sz: f32) -> Mat4 {
    let mut out = *world_mat;
    for row in 0..3 {
        out.data[row][0] *= sx;
        out.data[row][1] *= sy;
        out.data[row][2] *= sz;
    }
    out
}

// Translate in local coordinates while preserving the linear part.
fn translate_local(world_mat: &Mat4, local: &Vec3) -> Mat4 {
    let translated = mat_apply(world_mat, local);
    let mut out = *world_mat;
    out.data[0][3] = translated.x;
    out.data[1][3] = translated.y;
    out.data[2][3] = translated.z;
    out
}

// A level-1 subdivided icosahedron has 42 vertices, 80 triangles, and 120 edges.
pub fn sphere(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    local: &Vec3,
    r: f32,
    col_flat: i32,
    col_image: Option<&RcImage>,
    colkey: Option<i32>,
    state: DrawState,
) {
    let translated = translate_local(world_mat, local);
    let scaled = scale_axes(&translated, r, r, r);
    let g = if col_image.is_some() {
        primitive::unit_sphere_textured()
    } else {
        primitive::unit_sphere_solid()
    };
    let uvs = if col_image.is_some() {
        primitive_uvs(g)
    } else {
        None
    };
    let _ = prim(
        ctx,
        &scaled,
        g.mode,
        g.cull,
        g.positions.as_slice(),
        Some(g.indices.as_slice()),
        primitive_normals(g),
        uvs,
        col_flat,
        col_image,
        colkey,
        state,
    );
}

pub fn sphereb(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    local: &Vec3,
    r: f32,
    col: i32,
    state: DrawState,
) {
    let translated = translate_local(world_mat, local);
    let scaled = scale_axes(&translated, r, r, r);
    let g = primitive::unit_sphere_wire();
    let _ = prim(
        ctx,
        &scaled,
        g.mode,
        g.cull,
        g.positions.as_slice(),
        Some(g.indices.as_slice()),
        None,
        None,
        col,
        None,
        None,
        state,
    );
}

// circ / circb are screen-aligned: their projected geometry depends on
// the camera, so they bypass the world-space prim path and use the
// screen-space rasterizer directly with constant depth.
pub fn circ(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    local: &Vec3,
    r: f32,
    col: i32,
    state: DrawState,
) {
    let world_mat = prepare_draw(ctx, world_mat, &state);
    let world = mat_apply(&world_mat, local);
    let z_shift = depth_offset_shift(&ctx.camera, ctx.depth_offset);
    let camera = rc_ref!(&ctx.camera);
    let projected = screen_circle(
        &world,
        r,
        &ctx.vp,
        &ctx.clip_row,
        &camera,
        ctx.vp_x,
        ctx.vp_y,
        ctx.vp_w,
        ctx.vp_h,
    );
    if let Some((sx, sy, sr, sz)) = projected {
        let sz = project_offset(
            &world,
            &ctx.vp,
            &ctx.clip_row,
            ctx.vp_x,
            ctx.vp_y,
            ctx.vp_w,
            ctx.vp_h,
            &z_shift,
        )
        .map_or(sz, |p| p.2);
        let mut target_mut = rc_mut!(&ctx.target);
        let depth_w = ctx.depth_w;
        rasterize_circle_filled(
            &mut target_mut,
            ctx.depth.as_mut_slice(),
            depth_w,
            sx,
            sy,
            sr,
            sz,
            col as u8,
            col as u8,
            ctx.clip,
            ctx.dither_alpha,
            ctx.depth_test,
            ctx.depth_write,
        );
    }
}

pub fn circb(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    local: &Vec3,
    r: f32,
    col: i32,
    state: DrawState,
) {
    let world_mat = prepare_draw(ctx, world_mat, &state);
    let world = mat_apply(&world_mat, local);
    let z_shift = depth_offset_shift(&ctx.camera, ctx.depth_offset);
    let camera = rc_ref!(&ctx.camera);
    let projected = screen_circle(
        &world,
        r,
        &ctx.vp,
        &ctx.clip_row,
        &camera,
        ctx.vp_x,
        ctx.vp_y,
        ctx.vp_w,
        ctx.vp_h,
    );
    if let Some((sx, sy, sr, sz)) = projected {
        let sz = project_offset(
            &world,
            &ctx.vp,
            &ctx.clip_row,
            ctx.vp_x,
            ctx.vp_y,
            ctx.vp_w,
            ctx.vp_h,
            &z_shift,
        )
        .map_or(sz, |p| p.2);
        let mut target_mut = rc_mut!(&ctx.target);
        let depth_w = ctx.depth_w;
        rasterize_circle_border(
            &mut target_mut,
            ctx.depth.as_mut_slice(),
            depth_w,
            sx,
            sy,
            sr,
            sz,
            col as u8,
            col as u8,
            ctx.clip,
            ctx.dither_alpha,
            ctx.depth_test,
            ctx.depth_write,
        );
    }
}

pub fn sprite(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    local: &Vec3,
    img: &RcImage,
    uvs: Uvs,
    w: f32,
    h: f32,
    colkey: Option<i32>,
    angle: f32,
    state: DrawState,
) {
    let world = mat_apply(world_mat, local);
    let corners = {
        let camera = rc_ref!(&ctx.camera);
        sprite_corners(&world, w, h, angle, &camera)
    };
    let positions = [
        corners[0].x,
        corners[0].y,
        corners[0].z,
        corners[1].x,
        corners[1].y,
        corners[1].z,
        corners[2].x,
        corners[2].y,
        corners[2].z,
        corners[3].x,
        corners[3].y,
        corners[3].z,
    ];
    let uv_array = [
        uvs.0 .0, uvs.0 .1, uvs.1 .0, uvs.1 .1, uvs.2 .0, uvs.2 .1, uvs.3 .0, uvs.3 .1,
    ];
    let indices = [0_i32, 1, 2, 1, 3, 2];
    let identity = Mat4::identity_value();
    // Corners already form a world-space billboard.
    // Camera-facing sprites have no meaningful lit normal and render unshaded.
    let mut sprite_state = state;
    sprite_state.billboard = BILLBOARD_OFF;
    sprite_state.shaded = false;
    let _ = prim(
        ctx,
        &identity,
        MODE_TRIANGLES,
        CULL_NONE,
        &positions,
        Some(&indices),
        None,
        Some(&uv_array),
        0,
        Some(img),
        colkey,
        sprite_state,
    );
}

pub fn plane(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    img: &RcImage,
    uvs: Uvs,
    w: f32,
    h: f32,
    colkey: Option<i32>,
    state: DrawState,
) {
    let scaled = scale_axes(world_mat, w * 0.5, h * 0.5, 1.0);
    let g = primitive::unit_plane();
    let uv_array = [
        uvs.0 .0, uvs.0 .1, uvs.1 .0, uvs.1 .1, uvs.2 .0, uvs.2 .1, uvs.3 .0, uvs.3 .1,
    ];
    let _ = prim(
        ctx,
        &scaled,
        g.mode,
        g.cull,
        g.positions.as_slice(),
        Some(g.indices.as_slice()),
        primitive_normals(g),
        Some(&uv_array),
        0,
        Some(img),
        colkey,
        state,
    );
}

fn measure_text(font: Option<&mut Font>, text: &str) -> (i32, i32) {
    let (text_w, line_height) = if let Some(font) = font {
        let mut max_w = 0;
        for line in text.split('\n') {
            let w = font.text_width(line);
            if w > max_w {
                max_w = w;
            }
        }
        let (line_height, _ascent) = font.line_metrics();
        (max_w, line_height)
    } else {
        let max_chars = text
            .split('\n')
            .map(|line| {
                line.chars()
                    .filter(|c| (MIN_FONT_CODE..=MAX_FONT_CODE).contains(c))
                    .count()
            })
            .max()
            .unwrap_or(0);
        (max_chars as i32 * FONT_WIDTH as i32, FONT_HEIGHT as i32)
    };
    let line_count = text.split('\n').count() as i32;
    (text_w, line_count * line_height)
}

fn for_each_builtin_text_pixel(text: &str, mut emit: impl FnMut(i32, i32)) {
    let font_image = crate::pyxel::font_image();
    let img_ref = rc_ref!(&font_image);
    let font_data = &img_ref.canvas.data;
    let font_w = img_ref.canvas.width() as usize;
    let mut cur_x = 0_i32;
    let mut cur_y = 0_i32;
    for c in text.chars() {
        if c == '\n' {
            cur_x = 0;
            cur_y += FONT_HEIGHT as i32;
            continue;
        }
        if !(MIN_FONT_CODE..=MAX_FONT_CODE).contains(&c) {
            continue;
        }
        let code = c as i32 - MIN_FONT_CODE as i32;
        let src_x = (code % NUM_FONT_COLS as i32) as usize * FONT_WIDTH as usize;
        let src_y = (code / NUM_FONT_COLS as i32) as usize * FONT_HEIGHT as usize;
        for fy in 0..FONT_HEIGHT as usize {
            for fx in 0..FONT_WIDTH as usize {
                let idx = (src_y + fy) * font_w + (src_x + fx);
                if font_data[idx] != 0 {
                    emit(cur_x + fx as i32, cur_y + fy as i32);
                }
            }
        }
        cur_x += FONT_WIDTH as i32;
    }
}

// Screen-sized glyphs center on the projected anchor and share its depth.
// Ancestor rotation and scale do not affect the camera-facing glyph layout.
pub fn text(
    ctx: &mut DrawContext,
    world_mat: &Mat4,
    pos: &Vec3,
    text_str: &str,
    col: i32,
    font: Option<&mut Font>,
    state: DrawState,
) {
    if text_str.is_empty() {
        return;
    }
    let world = mat_apply(world_mat, pos);
    let z_shift = depth_offset_shift(&ctx.camera, ctx.depth_offset);
    let projected = project_offset(
        &world,
        &ctx.vp,
        &ctx.clip_row,
        ctx.vp_x,
        ctx.vp_y,
        ctx.vp_w,
        ctx.vp_h,
        &z_shift,
    );
    let Some((sx_f, sy_f, sz)) = projected else {
        return;
    };
    let sx = sx_f.round() as i32;
    let sy = sy_f.round() as i32;
    let mut font = font;
    let (text_w, text_h) = measure_text(font.as_deref_mut(), text_str);
    if text_w == 0 || text_h == 0 {
        return;
    }
    let cx = sx - text_w / 2;
    let cy = sy - text_h / 2;
    let mut target_mut = rc_mut!(&ctx.target);
    let depth_w = ctx.depth_w;
    let depth = ctx.depth.as_mut_slice();
    let clip = ctx.clip;
    let col = col as u8;
    let mut plot_pixel = |px: i32, py: i32| {
        let x = cx + px;
        let y = cy + py;
        if x < clip.left || x > clip.right || y < clip.top || y > clip.bottom {
            return;
        }
        write_pixel(
            &mut target_mut,
            depth,
            depth_w,
            x,
            y,
            sz,
            col,
            state.dither_alpha,
            state.depth_test,
            state.depth_write,
        );
    };
    if let Some(font) = font {
        font.for_each_pixel(0, 0, text_str, &mut plot_pixel);
    } else {
        for_each_builtin_text_pixel(text_str, &mut plot_pixel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::camera::Camera;
    use crate::cube::raster::{
        camera_clip_row, compute_clip_rect, matmul, projection_matrix, view_matrix,
    };

    fn draw_context_64(target: &RcImage, camera: &RcCamera, shaded: bool) -> DrawContext {
        let view = view_matrix(&rc_ref!(camera));
        DrawContext {
            target: target.clone(),
            vp: matmul(&projection_matrix(&rc_ref!(camera), 64.0, 64.0), &view),
            clip_row: camera_clip_row(&view),
            vp_x: 0.0,
            vp_y: 0.0,
            vp_w: 64.0,
            vp_h: 64.0,
            clip: compute_clip_rect(0.0, 0.0, 64.0, 64.0, 64, 64),
            camera: camera.clone(),
            depth: vec![f32::INFINITY; 64 * 64],
            depth_w: 64,
            depth_h: 64,
            vertex_cache: Vec::new(),
            dither_alpha: 1.0,
            depth_test: true,
            depth_write: true,
            depth_offset: 0.0,
            shaded,
        }
    }

    fn sampler_test_image() -> RcImage {
        let image = Image::new(2, 2);
        let mut image_ref = rc_mut!(&image);
        image_ref.canvas.write_data(0, 0, 1);
        image_ref.canvas.write_data(1, 0, 2);
        image_ref.canvas.write_data(0, 1, 3);
        image_ref.canvas.write_data(1, 1, 4);
        drop(image_ref);
        image
    }

    #[test]
    fn test_image_sampler_truncates_and_clamps_uvs() {
        let image = sampler_test_image();
        let image_ref = rc_ref!(&image);
        let sample = make_image_sampler(&image_ref, None);

        assert_eq!(sample(-0.25, 0.1, 0, 0), Some(1));
        assert_eq!(sample(0.49, 0.49, 0, 0), Some(1));
        assert_eq!(sample(0.5, 0.1, 0, 0), Some(2));
        assert_eq!(sample(0.1, 0.5, 0, 0), Some(3));
        assert_eq!(sample(1.25, 1.25, 0, 0), Some(4));
    }

    #[test]
    fn test_shaded_sampler_uses_same_uv_clamping() {
        let image = sampler_test_image();
        let shading = Shading::new(&[0; 5]);
        for color in 0..5 {
            rc_mut!(&shading).set(color, 0, (color as i32 + 10, color as i32 + 10));
        }
        let image_ref = rc_ref!(&image);
        let shading_ref = rc_ref!(&shading);
        let sample = make_shaded_sampler(&image_ref, &shading_ref, 0, None);

        assert_eq!(sample(-0.25, 0.1, 0, 0), Some(11));
        assert_eq!(sample(0.5, 0.1, 0, 0), Some(12));
        assert_eq!(sample(0.1, 0.5, 0, 0), Some(13));
        assert_eq!(sample(1.25, 1.25, 0, 0), Some(14));
    }

    #[test]
    fn test_prim_topology_errors_name_the_invalid_input() {
        let target = Image::new(64, 64);
        let camera = Camera::new();
        let mut ctx = draw_context_64(&target, &camera, false);
        let state = DrawState::unshaded();
        let identity = Mat4::identity_value();
        let cases = [
            (
                MODE_TRIANGLES,
                &[0.0; 9][..],
                Some(&[0, 1][..]),
                "indices length must be a multiple of 3 for MODE_TRIANGLES",
            ),
            (
                MODE_TRIANGLES,
                &[0.0; 6][..],
                None,
                "positions vertex count must be a multiple of 3 for MODE_TRIANGLES",
            ),
            (
                MODE_LINES,
                &[0.0; 6][..],
                Some(&[0][..]),
                "indices length must be a multiple of 2 for MODE_LINES",
            ),
            (
                MODE_LINES,
                &[0.0; 9][..],
                None,
                "positions vertex count must be a multiple of 2 for MODE_LINES",
            ),
        ];

        for (mode, positions, indices, expected) in cases {
            let result = prim(
                &mut ctx, &identity, mode, CULL_NONE, positions, indices, None, None, 7, None,
                None, state,
            );

            assert_eq!(result, Err(expected));
        }
    }

    #[test]
    fn test_signed_screen_area_degenerate_zero() {
        let area = signed_screen_area((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (2.0, 0.0, 0.0));
        assert_eq!(area, 0.0);
    }

    #[test]
    fn test_should_cull_back_skips_back_face() {
        assert!(should_cull(1.0, CULL_BACK));
        assert!(should_cull(0.0, CULL_BACK));
        assert!(!should_cull(-1.0, CULL_BACK));
    }

    #[test]
    fn test_should_cull_front_skips_front_face() {
        assert!(should_cull(-1.0, CULL_FRONT));
        assert!(should_cull(0.0, CULL_FRONT));
        assert!(!should_cull(1.0, CULL_FRONT));
    }

    #[test]
    fn test_should_cull_none_draws_everything() {
        assert!(!should_cull(1.0, CULL_NONE));
        assert!(!should_cull(-1.0, CULL_NONE));
        assert!(!should_cull(0.0, CULL_NONE));
    }

    fn near_clip_test_row() -> [f32; 4] {
        [0.0, 0.0, -1.0, 0.0]
    }

    fn clip_vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> ClipVertex {
        ClipVertex {
            world: Vec3 { x, y, z },
            uv: (u, v),
        }
    }

    fn assert_clip_vertices_inside(vertices: &ClippedTriangle, clip_row: &[f32; 4]) {
        for i in 0..vertices.len {
            let front = clip_front(&vertices.vertices[i].world, clip_row);
            assert!(
                front >= CLIP_FRONT_EPSILON - 1e-6,
                "vertex {i} front={front}"
            );
        }
    }

    #[test]
    fn test_clip_triangle_to_near_keeps_all_inside_vertices() {
        let clip_row = near_clip_test_row();
        let vertices = [
            clip_vertex(-1.0, -1.0, -1.0, 0.0, 0.0),
            clip_vertex(1.0, -1.0, -1.0, 1.0, 0.0),
            clip_vertex(0.0, 1.0, -1.0, 0.5, 1.0),
        ];
        let clipped = clip_triangle_to_near(vertices, &clip_row);

        assert_eq!(clipped.len, 3);
        for (actual, expected) in clipped.vertices.iter().zip(vertices) {
            assert_eq!(actual.world, expected.world);
            assert_eq!(actual.uv, expected.uv);
        }
    }

    #[test]
    fn test_clip_triangle_to_near_rejects_all_behind_vertices() {
        let clip_row = near_clip_test_row();
        let vertices = [
            clip_vertex(-1.0, -1.0, 1.0, 0.0, 0.0),
            clip_vertex(1.0, -1.0, 1.0, 1.0, 0.0),
            clip_vertex(0.0, 1.0, 1.0, 0.5, 1.0),
        ];
        let clipped = clip_triangle_to_near(vertices, &clip_row);

        assert_eq!(clipped.len, 0);
    }

    #[test]
    fn test_clip_triangle_to_near_one_vertex_behind_returns_quad() {
        let clip_row = near_clip_test_row();
        let vertices = [
            clip_vertex(-1.0, -1.0, -1.0, 0.0, 0.0),
            clip_vertex(1.0, -1.0, -1.0, 1.0, 0.0),
            clip_vertex(0.0, 1.0, 1.0, 0.5, 1.0),
        ];
        let clipped = clip_triangle_to_near(vertices, &clip_row);

        assert_eq!(clipped.len, 4);
        assert_clip_vertices_inside(&clipped, &clip_row);
    }

    #[test]
    fn test_clip_triangle_to_near_two_vertices_behind_returns_triangle() {
        let clip_row = near_clip_test_row();
        let vertices = [
            clip_vertex(-1.0, -1.0, -1.0, 0.0, 0.0),
            clip_vertex(1.0, -1.0, 1.0, 1.0, 0.0),
            clip_vertex(0.0, 1.0, 1.0, 0.5, 1.0),
        ];
        let clipped = clip_triangle_to_near(vertices, &clip_row);

        assert_eq!(clipped.len, 3);
        assert_clip_vertices_inside(&clipped, &clip_row);
    }

    #[test]
    fn test_depth_offset_negative_moves_toward_camera() {
        let camera = Camera::new();
        let v = view_matrix(&rc_ref!(&camera));
        let p = projection_matrix(&rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        let clip_row = camera_clip_row(&v);
        // Off-axis point in front of the default (-Z looking) camera.
        let pos = Vec3 {
            x: 1.0,
            y: 0.5,
            z: -3.0,
        };
        let base = world_to_screen(&pos, &vp, &clip_row, 0.0, 0.0, 256.0, 192.0).unwrap();
        let near = depth_offset_shift(&camera, -0.5);
        let near_p = project_offset(&pos, &vp, &clip_row, 0.0, 0.0, 256.0, 192.0, &near).unwrap();
        assert_eq!(near_p.0, base.0);
        assert_eq!(near_p.1, base.1);
        assert!(near_p.2 < base.2);
        let far = depth_offset_shift(&camera, 0.5);
        let far_p = project_offset(&pos, &vp, &clip_row, 0.0, 0.0, 256.0, 192.0, &far).unwrap();
        assert!(far_p.2 > base.2);
    }

    #[test]
    fn test_depth_offset_zero_is_identity() {
        let camera = Camera::new();
        let v = view_matrix(&rc_ref!(&camera));
        let p = projection_matrix(&rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        let clip_row = camera_clip_row(&v);
        let pos = Vec3 {
            x: 1.0,
            y: 0.5,
            z: -3.0,
        };
        let base = world_to_screen(&pos, &vp, &clip_row, 0.0, 0.0, 256.0, 192.0).unwrap();
        let zero = depth_offset_shift(&camera, 0.0);
        let same = project_offset(&pos, &vp, &clip_row, 0.0, 0.0, 256.0, 192.0, &zero).unwrap();
        assert_eq!(base, same);
    }

    #[test]
    fn test_line_clips_endpoint_behind_camera_instead_of_dropping() {
        let target = Image::new(64, 64);
        rc_mut!(&target).clear(2);
        let camera = Camera::new();
        let mut ctx = draw_context_64(&target, &camera, false);
        let state = DrawState::unshaded();
        let positions = [0.0, 0.0, -2.0, 0.5, 0.0, 1.0];

        prim(
            &mut ctx,
            &Mat4::identity_value(),
            MODE_LINES,
            CULL_NONE,
            &positions,
            None,
            None,
            None,
            7,
            None,
            None,
            state,
        )
        .unwrap();

        assert_eq!(rc_ref!(&target).pixel(32.0, 32.0), 7);
    }

    #[test]
    fn test_triangle_clips_vertex_behind_camera_instead_of_dropping() {
        let target = Image::new(64, 64);
        rc_mut!(&target).clear(2);
        let camera = Camera::new();
        let mut ctx = draw_context_64(&target, &camera, false);
        let state = DrawState::unshaded();
        let positions = [-2.0, -1.0, -2.0, 2.0, -1.0, -2.0, 0.0, 2.0, 1.0];

        prim(
            &mut ctx,
            &Mat4::identity_value(),
            MODE_TRIANGLES,
            CULL_NONE,
            &positions,
            None,
            None,
            None,
            7,
            None,
            None,
            state,
        )
        .unwrap();

        assert_eq!(rc_ref!(&target).pixel(32.0, 32.0), 7);
    }

    #[test]
    fn test_box_solid_culls_back_faces() {
        let target = Image::new(64, 64);
        rc_mut!(&target).clear(2);
        let camera = Camera::new();
        let mut ctx = draw_context_64(&target, &camera, false);

        box_solid(
            &mut ctx,
            &Mat4::identity_value(),
            &Vec3 {
                x: 4.0,
                y: 4.0,
                z: 4.0,
            },
            7,
            None,
            None,
            DrawState::unshaded(),
        );

        assert_eq!(rc_ref!(&target).pixel(32.0, 32.0), 2);
    }

    #[test]
    fn test_ortho_camera_clips_geometry_behind_camera() {
        // The orthographic w row is constant 1, so behind-camera clipping
        // must come from the camera clip row rather than clip-space w.
        let target = Image::new(64, 64);
        rc_mut!(&target).clear(2);
        let camera = Camera::new();
        rc_mut!(&camera).ortho_size = Some(10.0);
        let mut ctx = draw_context_64(&target, &camera, false);
        let state = DrawState::unshaded();

        let behind = [-2.0, -2.0, 2.0, 2.0, -2.0, 2.0, 0.0, 2.0, 2.0];
        prim(
            &mut ctx,
            &Mat4::identity_value(),
            MODE_TRIANGLES,
            CULL_NONE,
            &behind,
            None,
            None,
            None,
            7,
            None,
            None,
            state,
        )
        .unwrap();
        assert_eq!(rc_ref!(&target).pixel(32.0, 32.0), 2);

        let front = [-2.0, -2.0, -2.0, 2.0, -2.0, -2.0, 0.0, 2.0, -2.0];
        prim(
            &mut ctx,
            &Mat4::identity_value(),
            MODE_TRIANGLES,
            CULL_NONE,
            &front,
            None,
            None,
            None,
            7,
            None,
            None,
            state,
        )
        .unwrap();
        assert_eq!(rc_ref!(&target).pixel(32.0, 32.0), 7);
    }

    #[test]
    fn test_shaded_stored_normals_track_transforms() {
        let shading_rc = Shading::new(&[0; 16]);
        rc_mut!(&shading_rc).direction = Vec3::new(0.0, 0.0, -1.0);
        for level in 0..4 {
            rc_mut!(&shading_rc).set(7, level, (8 + level as i32, 8 + level as i32));
        }
        let geom = Primitive::new();
        {
            let mut g = rc_mut!(&geom);
            g.positions = vec![
                -2.0, 2.0, -2.0, 2.0, 2.0, 2.0, -2.0, -2.0, -2.0, 2.0, -2.0, 2.0,
            ];
            g.indices = vec![0, 2, 1, 1, 2, 3];
            g.cull = CULL_NONE;
            g.compute_normals();
        }
        let model_normals = rc_ref!(&geom).normals.clone();
        let positions = rc_ref!(&geom).positions.clone();
        let indices = rc_ref!(&geom).indices.clone();

        let render = |world: &Mat4, normals: Option<&[f32]>| -> Vec<u8> {
            let target = Image::new(64, 64);
            rc_mut!(&target).clear(2); // sentinel so an undrawn quad is detectable
            let camera = Camera::new();
            let mut ctx = draw_context_64(&target, &camera, true);
            let shading_ref = rc_ref!(&shading_rc);
            let state = DrawState {
                shaded: true,
                dither_alpha: 1.0,
                depth_test: true,
                depth_write: true,
                billboard: BILLBOARD_OFF,
                shading: Some(&shading_ref),
            };
            prim(
                &mut ctx,
                world,
                MODE_TRIANGLES,
                CULL_NONE,
                &positions,
                Some(&indices),
                normals,
                None,
                7,
                None,
                None,
                state,
            )
            .unwrap();
            let pixels = rc_ref!(&target).canvas.data.clone();
            pixels
        };
        let translation = Mat4::from_translation(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: -8.0,
        });
        let cases = [
            (Mat4::identity(), 10),
            (
                Mat4::from_axis_angle(
                    &Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    180.0,
                ),
                8,
            ),
            (
                Mat4::from_scale(&Vec3 {
                    x: 4.0,
                    y: 1.0,
                    z: 1.0,
                }),
                11,
            ),
            (
                Mat4::from_scale(&Vec3 {
                    x: -4.0,
                    y: 1.0,
                    z: 1.0,
                }),
                8,
            ),
            (
                Mat4::from_scale(&Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                }),
                11,
            ),
        ];
        for (transform, expected) in cases {
            let world = rc_ref!(&translation).mul_mat_value(&rc_ref!(&transform));
            let auto = render(&world, None);
            let stored = render(&world, Some(&model_normals));
            assert_eq!(auto[32 * 64 + 32], expected);
            assert_eq!(stored[32 * 64 + 32], expected, "transform={:?}", world.data);
            assert_eq!(stored, auto);
        }
        let opposite: Vec<f32> = model_normals.iter().map(|n| -n).collect();
        assert_eq!(
            render(&rc_ref!(&translation), Some(&opposite))[32 * 64 + 32],
            8
        );
    }
}
