#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]

// High-level cube draw commands. All commands consume the active
// DrawContext (target image, viewport, clip, scene depth buffer, camera)
// and a world transform, and ultimately route through `prim` so all
// per-pixel decisions (depth, shading, texture sampling, dither) live in
// one place. Higher-level shortcuts fabricate vertex / index / uv arrays
// and pass them through prim.

use std::sync::OnceLock;

use crate::cube::camera::RcCamera;
use crate::cube::mat4::Mat4;
use crate::cube::mesh::Mesh;
use crate::cube::primitive::{
    CULL_BACK, CULL_FRONT, CULL_NONE, MODE_LINES, MODE_POINTS, MODE_TRIANGLES,
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

const CLIP_W_EPSILON: f32 = 1e-4;
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

// Primitive draw modes are owned by `Primitive` (see primitive.rs); this
// file imports them at the top. Values follow OpenGL ordering
// (GL_POINTS=0, GL_LINES=1, GL_TRIANGLES=4 — cube uses 0/1/2 internally
// but keeps the relative ordering so future MODE_LINE_STRIP / LINE_LOOP
// / TRIANGLE_STRIP / TRIANGLE_FAN additions can interleave the GL
// numbering as needed).

// Billboard modes (binding mirrors these as Node class attrs).
// Mirrors Godot's BillboardMode (DISABLED / ENABLED / FIXED_Y).
pub const BILLBOARD_OFF: i32 = 0;
pub const BILLBOARD_ON: i32 = 1;
pub const BILLBOARD_FIXED_Y: i32 = 2;

pub type Uvs = ((f32, f32), (f32, f32), (f32, f32), (f32, f32));

// Per-call modifier bundle. Bound by each draw command before it calls
// the rasterizer; the rasterizer reads `ctx.dither_alpha`,
// `ctx.depth_test`, `ctx.depth_write`. `shaded` decides whether the
// `shading` is consulted for the per-face brightness; `billboard`
// rewrites `world_mat` rotation so the surface faces the camera.
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

// Signed area of the triangle in Y-down screen space. Positive area =
// CCW winding = front-facing.
#[inline]
fn signed_screen_area(p0: (f32, f32, f32), p1: (f32, f32, f32), p2: (f32, f32, f32)) -> f32 {
    (p1.0 - p0.0) * (p2.1 - p0.1) - (p1.1 - p0.1) * (p2.0 - p0.0)
}

// Decide whether to skip a face under the given cull mode. Degenerate
// faces (area == 0) are skipped under any non-NONE cull, matching the
// convention that they have no front side to draw.
#[inline]
fn should_cull(area: f32, cull: i32) -> bool {
    // Pyxel cube uses CCW outward winding (front face from outside).
    // Projecting onto the Y-down screen flips the sign: front faces yield
    // a negative signed_screen_area, back faces yield a positive one.
    // Degenerate faces (area == 0) are skipped under any non-NONE cull.
    (cull == CULL_BACK && area >= 0.0) || (cull == CULL_FRONT && area <= 0.0)
}

// Apply billboard rewriting and per-call modifiers to ctx, returning the
// possibly-rewritten world matrix.
fn prepare_draw(ctx: &mut DrawContext, world_mat: &Mat4, state: &DrawState) -> Mat4 {
    ctx.dither_alpha = state.dither_alpha.clamp(0.0, 1.0);
    ctx.depth_test = state.depth_test;
    ctx.depth_write = state.depth_write;
    apply_billboard(world_mat, ctx, state.billboard)
}

// World-space shift for a depth offset: `offset` units along the camera's
// view direction (the look direction is the -Z column of the
// camera-to-world transform). Positive pushes away from the camera,
// negative toward it. Returns a zero vector when no offset is set.
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

// Project `pos` to screen, then replace only its depth with the depth of
// `pos + shift`. Screen x/y stay at the original projection, so a depth
// offset never moves or resizes the draw; it only shifts the depth used
// for the test / write. A zero `shift` is the no-offset fast path (single
// projection).
fn project_offset(
    pos: &Vec3,
    vp: &[[f32; 4]; 4],
    vp_x: f32,
    vp_y: f32,
    vp_w: f32,
    vp_h: f32,
    shift: &Vec3,
) -> Option<(f32, f32, f32)> {
    let p = world_to_screen(pos, vp, vp_x, vp_y, vp_w, vp_h)?;
    if shift.x == 0.0 && shift.y == 0.0 && shift.z == 0.0 {
        return Some(p);
    }
    let shifted = Vec3 {
        x: pos.x + shift.x,
        y: pos.y + shift.y,
        z: pos.z + shift.z,
    };
    match world_to_screen(&shifted, vp, vp_x, vp_y, vp_w, vp_h) {
        Some(d) => Some((p.0, p.1, d.2)),
        None => Some(p),
    }
}

fn clip_w(pos: &Vec3, vp: &[[f32; 4]; 4]) -> f32 {
    vp[3][0] * pos.x + vp[3][1] * pos.y + vp[3][2] * pos.z + vp[3][3]
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

fn clip_triangle_to_near(vertices: [ClipVertex; 3], vp: &[[f32; 4]; 4]) -> ClippedTriangle {
    let mut clipped = ClippedTriangle {
        vertices: [vertices[0]; 4],
        len: 0,
    };
    for i in 0..3 {
        let prev = vertices[(i + 2) % 3];
        let curr = vertices[i];
        let prev_w = clip_w(&prev.world, vp);
        let curr_w = clip_w(&curr.world, vp);
        let prev_inside = prev_w > CLIP_W_EPSILON;
        let curr_inside = curr_w > CLIP_W_EPSILON;

        if prev_inside != curr_inside {
            let t = (CLIP_W_EPSILON - prev_w) / (curr_w - prev_w);
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
        if let Some(normal) = normal {
            let shading = state.shading.unwrap();
            let direction = rc_ref!(&shading.direction);
            let level = face_shade_level(direction, Some(normal));
            let sampler = make_shaded_sampler(img_ref, shading, level);
            let target_mut = rc_mut!(&ctx.target);
            let depth = ctx.depth.as_mut_slice();
            rasterize_textured_triangle(
                target_mut,
                depth,
                depth_w,
                a.screen,
                b.screen,
                c.screen,
                a.uv,
                b.uv,
                c.uv,
                &sampler,
                colkey,
                clip,
                dither_alpha,
                depth_test,
                depth_write,
            );
        } else {
            let sampler = make_image_sampler(img_ref);
            let target_mut = rc_mut!(&ctx.target);
            let depth = ctx.depth.as_mut_slice();
            rasterize_textured_triangle(
                target_mut,
                depth,
                depth_w,
                a.screen,
                b.screen,
                c.screen,
                a.uv,
                b.uv,
                c.uv,
                &sampler,
                colkey,
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
        let target_mut = rc_mut!(&ctx.target);
        let depth = ctx.depth.as_mut_slice();
        rasterize_triangle(
            target_mut,
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
    let w0 = clip_w(p0, &ctx.vp);
    let w1 = clip_w(p1, &ctx.vp);
    if w0 <= CLIP_W_EPSILON && w1 <= CLIP_W_EPSILON {
        return None;
    }

    let mut q0 = *p0;
    let mut q1 = *p1;
    if w0 <= CLIP_W_EPSILON {
        let t = (CLIP_W_EPSILON - w0) / (w1 - w0);
        q0 = lerp_world(p0, p1, t);
    } else if w1 <= CLIP_W_EPSILON {
        let t = (CLIP_W_EPSILON - w0) / (w1 - w0);
        q1 = lerp_world(p0, p1, t);
    }

    let s0 = project_offset(
        &q0, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h, z_shift,
    )?;
    let s1 = project_offset(
        &q1, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h, z_shift,
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
    let (rx, ry, rz) = if mode == BILLBOARD_FIXED_Y {
        // Cylindrical: keep world Y; rebuild X / Z from camera horizontal.
        let world_up = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        // Project camera Z onto the horizontal plane to derive new forward.
        let mut forward = Vec3 {
            x: cam_z.x,
            y: 0.0,
            z: cam_z.z,
        };
        let len = (forward.x * forward.x + forward.z * forward.z).sqrt();
        if len > 1e-6 {
            forward.x /= len;
            forward.z /= len;
        } else {
            forward.z = 1.0;
        }
        let right = Vec3 {
            x: world_up.y * forward.z - world_up.z * forward.y,
            y: world_up.z * forward.x - world_up.x * forward.z,
            z: world_up.x * forward.y - world_up.y * forward.x,
        };
        (right, world_up, forward)
    } else {
        // Spherical (BILLBOARD_ON): adopt camera basis directly.
        (cam_x, cam_y, cam_z)
    };
    let mut out = Mat4::identity_value();
    out.data[0][0] = rx.x * scale_x;
    out.data[1][0] = rx.y * scale_x;
    out.data[2][0] = rx.z * scale_x;
    out.data[0][1] = ry.x * scale_y;
    out.data[1][1] = ry.y * scale_y;
    out.data[2][1] = ry.z * scale_y;
    out.data[0][2] = rz.x * scale_z;
    out.data[1][2] = rz.y * scale_z;
    out.data[2][2] = rz.z * scale_z;
    out.data[0][3] = pos.x;
    out.data[1][3] = pos.y;
    out.data[2][3] = pos.z;
    out.data[3][3] = 1.0;
    out
}

// Image samplers used by textured prim TRIANGLES.

fn make_image_sampler(img: &Image) -> impl Fn(f32, f32, i32, i32) -> i32 + '_ {
    let w = img.width() as f32;
    let h = img.height() as f32;
    let max_x = (img.width() as i32 - 1).max(0);
    let max_y = (img.height() as i32 - 1).max(0);
    move |u, v, _x, _y| {
        let xi = ((u * w).floor() as i32).clamp(0, max_x);
        let yi = ((v * h).floor() as i32).clamp(0, max_y);
        i32::from(img.pixel(xi as f32, yi as f32))
    }
}

fn make_shaded_sampler<'a>(
    img: &'a Image,
    shading: &'a Shading,
    level: usize,
) -> impl Fn(f32, f32, i32, i32) -> i32 + 'a {
    let w = img.width() as f32;
    let h = img.height() as f32;
    let max_x = (img.width() as i32 - 1).max(0);
    let max_y = (img.height() as i32 - 1).max(0);
    let palette_size = shading.palette_size();
    move |u, v, x, y| {
        let xi = ((u * w).floor() as i32).clamp(0, max_x);
        let yi = ((v * h).floor() as i32).clamp(0, max_y);
        let base = i32::from(img.pixel(xi as f32, yi as f32));
        if palette_size == 0 {
            base
        } else {
            let base_idx = base.clamp(0, palette_size as i32 - 1) as usize;
            let (primary, secondary) = shading.get(base_idx, level);
            i32::from(dither_pick(primary, secondary, x, y))
        }
    }
}

// Universal primitive draw. Triangles / Lines / Points are dispatched
// here; all higher-level commands route through this function.
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
    let world_mat = prepare_draw(ctx, world_mat, &state);
    let z_shift = depth_offset_shift(&ctx.camera, ctx.depth_offset);
    let lit = state.shaded && state.shading.is_some();
    // Transform and project every vertex once into the per-draw scratch
    // cache. Indexed tables (box / sphere / rect) reference shared
    // vertices from several faces; the cache keeps each vertex's
    // transform + projection single no matter how many faces consume it.
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
        let screen = project_offset(
            &world, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h, &z_shift,
        );
        ctx.vertex_cache.push((world, screen));
    }
    let resolve_vertex_index = |step: usize| -> Result<usize, &'static str> {
        let raw = match indices {
            Some(idx) => idx[step],
            None => step as i32,
        };
        if raw < 0 || (raw as usize) >= vertex_count {
            return Err("index out of vertex range");
        }
        Ok(raw as usize)
    };
    match mode {
        MODE_TRIANGLES => {
            if !step_count.is_multiple_of(3) {
                return Err("TRIANGLES requires step count to be a multiple of 3");
            }
            let face_count = step_count / 3;
            if let Some(n) = normals {
                if lit && n.len() != face_count * 3 {
                    return Err("normals length must equal face_count * 3 when shaded");
                }
            }
            if col_image.is_some() && uvs.is_none() {
                return Err("textured prim requires uvs");
            }
            for f in 0..face_count {
                let i0 = resolve_vertex_index(f * 3)?;
                let i1 = resolve_vertex_index(f * 3 + 1)?;
                let i2 = resolve_vertex_index(f * 3 + 2)?;
                let (v0, p0) = ctx.vertex_cache[i0];
                let (v1, p1) = ctx.vertex_cache[i1];
                let (v2, p2) = ctx.vertex_cache[i2];
                let face_normal = || -> Vec3 {
                    match normals {
                        // Stored normals are model-space (e.g. from
                        // Primitive::compute_normals). Carry them into world
                        // space so shading matches the world-space light
                        // direction; the auto path below already yields a
                        // world-space normal from the world vertices.
                        Some(n) => mat_apply_dir(
                            &world_mat,
                            &Vec3 {
                                x: n[f * 3],
                                y: n[f * 3 + 1],
                                z: n[f * 3 + 2],
                            },
                        ),
                        None => tri_normal(&v0, &v1, &v2),
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
                        &ctx.vp,
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
            if !step_count.is_multiple_of(2) {
                return Err("LINES requires step count to be a multiple of 2");
            }
            let line_count = step_count / 2;
            let depth_w = ctx.depth_w;
            for l in 0..line_count {
                let i0 = resolve_vertex_index(l * 2)?;
                let i1 = resolve_vertex_index(l * 2 + 1)?;
                let (w0, _) = ctx.vertex_cache[i0];
                let (w1, _) = ctx.vertex_cache[i1];
                if let Some((p0, p1)) = project_line_segment(&w0, &w1, ctx, &z_shift) {
                    let target_mut = rc_mut!(&ctx.target);
                    let depth = ctx.depth.as_mut_slice();
                    rasterize_line(
                        target_mut,
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
            let target_mut = rc_mut!(&ctx.target);
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
                            target_mut,
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
        _ => return Err("invalid prim mode"),
    }
    Ok(())
}

// Shortcut commands fabricate buffers and route through prim.

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
    // Outline = three lines.
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

// elli / ellib triangulate / outline the unit ellipse, then fold the
// caller's (w, h) into the world matrix as XY scale.
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

// Box / boxb: cube faces / edges as 3D solid primitives. Folds `size`
// into the world matrix as per-axis scale of the cached unit cube.
//
// When `col_image` is Some, the textured path is used: each face is
// rendered with a full-texture UV span (0..1 × 0..1) using the
// face-aware UV convention defined alongside BOX_UNROLLED_POSITIONS
// below. The `col_flat` value is ignored in that case.
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
    if col_image.is_some() {
        // Textured path: use the unrolled 24-vertex layout so each face
        // has its own UV coordinates (shared-vertex layout cannot assign
        // different UVs to the same vertex for adjacent faces).
        let _ = prim(
            ctx,
            &scaled,
            MODE_TRIANGLES,
            CULL_BACK,
            &BOX_UNROLLED_POSITIONS,
            Some(&BOX_UNROLLED_TRI_INDICES),
            None,
            Some(&BOX_UNROLLED_UVS),
            col_flat,
            col_image,
            colkey,
            state,
        );
    } else {
        // Flat-color path: keep the original 8-vertex shared layout so
        // there is zero per-frame allocation overhead.
        let _ = prim(
            ctx,
            &scaled,
            MODE_TRIANGLES,
            CULL_BACK,
            &UNIT_BOX_POSITIONS,
            Some(&BOX_TRI_INDICES),
            None,
            None,
            col_flat,
            None,
            None,
            state,
        );
    }
}

pub fn boxb(ctx: &mut DrawContext, world_mat: &Mat4, size: &Vec3, col: i32, state: DrawState) {
    let scaled = scale_axes(world_mat, size.x, size.y, size.z);
    let _ = prim(
        ctx,
        &scaled,
        MODE_LINES,
        CULL_NONE,
        &UNIT_BOX_POSITIONS,
        Some(&BOX_EDGE_INDICES),
        None,
        None,
        col,
        None,
        None,
        state,
    );
}

// Cached vertex / index tables for primitive draw shapes. Each shape's
// positions are stored in unit form (radius 1 / half-extent 1); per-call
// `size` / `r` / `w` / `h` is folded into the world matrix rather than
// the vertex data, so per-frame allocation is zero.

// Unit cube: 8 vertices at ±0.5 on each axis. CCW outward winding when
// viewed from outside.
const UNIT_BOX_POSITIONS: [f32; 24] = [
    -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, 0.5, 0.5, -0.5,
    0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5,
];
const BOX_TRI_INDICES: [i32; 36] = [
    0, 2, 1, 0, 3, 2, // -Z face
    4, 5, 6, 4, 6, 7, // +Z face
    0, 1, 5, 0, 5, 4, // -Y face
    3, 6, 2, 3, 7, 6, // +Y face
    0, 4, 7, 0, 7, 3, // -X face
    1, 2, 6, 1, 6, 5, // +X face
];
const BOX_EDGE_INDICES: [i32; 24] = [
    0, 1, 1, 2, 2, 3, 3, 0, // back face square
    4, 5, 5, 6, 6, 7, 7, 4, // front face square
    0, 4, 1, 5, 2, 6, 3, 7, // four connecting edges
];

// Unrolled box: 24 vertices (4 per face × 6 faces). Each face owns its
// own copy of its 4 corners so every vertex can have a unique UV without
// sharing conflicts. Face order mirrors BOX_TRI_INDICES: -Z, +Z, -Y, +Y,
// -X, +X. Positions are identical to UNIT_BOX_POSITIONS but replicated.
//
// UV convention (the sampler maps v=0 to image row 0 / the top, so +V
// points toward local "down" on the side faces to keep them upright):
//   +X face: UV +U = local -Z,  UV +V = local -Y
//   -X face: UV +U = local +Z,  UV +V = local -Y
//   +Z face: UV +U = local +X,  UV +V = local -Y
//   -Z face: UV +U = local -X,  UV +V = local -Y
//   +Y face: UV +U = local +X,  UV +V = local +Z
//   -Y face: UV +U = local +X,  UV +V = local -Z
#[rustfmt::skip]
const BOX_UNROLLED_POSITIONS: [f32; 72] = [
    // -Z face (z=-0.5): vertices 0-3
    -0.5, -0.5, -0.5,   0.5, -0.5, -0.5,   0.5, 0.5, -0.5,  -0.5, 0.5, -0.5,
    // +Z face (z=+0.5): vertices 4-7
    -0.5, -0.5,  0.5,   0.5, -0.5,  0.5,   0.5, 0.5,  0.5,  -0.5, 0.5,  0.5,
    // -Y face (y=-0.5): vertices 8-11
    -0.5, -0.5, -0.5,   0.5, -0.5, -0.5,  -0.5, -0.5, 0.5,   0.5, -0.5, 0.5,
    // +Y face (y=+0.5): vertices 12-15
    -0.5,  0.5, -0.5,   0.5,  0.5, -0.5,  -0.5,  0.5, 0.5,   0.5,  0.5, 0.5,
    // -X face (x=-0.5): vertices 16-19
    -0.5, -0.5, -0.5,  -0.5,  0.5, -0.5,  -0.5, -0.5, 0.5,  -0.5,  0.5, 0.5,
    // +X face (x=+0.5): vertices 20-23
     0.5, -0.5, -0.5,   0.5,  0.5, -0.5,   0.5, -0.5, 0.5,   0.5,  0.5, 0.5,
];

// UV coordinates for BOX_UNROLLED_POSITIONS. Layout mirrors the
// per-face convention: U and V each span [0, 1] across the face.
#[rustfmt::skip]
const BOX_UNROLLED_UVS: [f32; 48] = [
    // -Z face: U = (-x+0.5), V = (-y+0.5)
    //   v0(-0.5,-0.5,-0.5): U=1.0 V=1.0
    //   v1( 0.5,-0.5,-0.5): U=0.0 V=1.0
    //   v2( 0.5, 0.5,-0.5): U=0.0 V=0.0
    //   v3(-0.5, 0.5,-0.5): U=1.0 V=0.0
    1.0, 1.0,  0.0, 1.0,  0.0, 0.0,  1.0, 0.0,
    // +Z face: U = (x+0.5), V = (-y+0.5)
    //   v4(-0.5,-0.5, 0.5): U=0.0 V=1.0
    //   v5( 0.5,-0.5, 0.5): U=1.0 V=1.0
    //   v6( 0.5, 0.5, 0.5): U=1.0 V=0.0
    //   v7(-0.5, 0.5, 0.5): U=0.0 V=0.0
    0.0, 1.0,  1.0, 1.0,  1.0, 0.0,  0.0, 0.0,
    // -Y face: U = (x+0.5), V = (-z+0.5)
    //   v8 (-0.5,-0.5,-0.5): U=0.0 V=1.0
    //   v9 ( 0.5,-0.5,-0.5): U=1.0 V=1.0
    //   v10(-0.5,-0.5, 0.5): U=0.0 V=0.0
    //   v11( 0.5,-0.5, 0.5): U=1.0 V=0.0
    0.0, 1.0,  1.0, 1.0,  0.0, 0.0,  1.0, 0.0,
    // +Y face: U = (x+0.5), V = (z+0.5)
    //   v12(-0.5, 0.5,-0.5): U=0.0 V=0.0
    //   v13( 0.5, 0.5,-0.5): U=1.0 V=0.0
    //   v14(-0.5, 0.5, 0.5): U=0.0 V=1.0
    //   v15( 0.5, 0.5, 0.5): U=1.0 V=1.0
    0.0, 0.0,  1.0, 0.0,  0.0, 1.0,  1.0, 1.0,
    // -X face: U = (z+0.5), V = (-y+0.5)
    //   v16(-0.5,-0.5,-0.5): U=0.0 V=1.0
    //   v17(-0.5, 0.5,-0.5): U=0.0 V=0.0
    //   v18(-0.5,-0.5, 0.5): U=1.0 V=1.0
    //   v19(-0.5, 0.5, 0.5): U=1.0 V=0.0
    0.0, 1.0,  0.0, 0.0,  1.0, 1.0,  1.0, 0.0,
    // +X face: U = (-z+0.5), V = (-y+0.5)
    //   v20( 0.5,-0.5,-0.5): U=1.0 V=1.0
    //   v21( 0.5, 0.5,-0.5): U=1.0 V=0.0
    //   v22( 0.5,-0.5, 0.5): U=0.0 V=1.0
    //   v23( 0.5, 0.5, 0.5): U=0.0 V=0.0
    1.0, 1.0,  1.0, 0.0,  0.0, 1.0,  0.0, 0.0,
];

// Triangle indices for BOX_UNROLLED_POSITIONS. Winding order mirrors
// BOX_TRI_INDICES (CCW outward from outside) but uses the unrolled
// per-face vertex offsets.
#[rustfmt::skip]
const BOX_UNROLLED_TRI_INDICES: [i32; 36] = [
    // -Z face (verts 0-3): tri (0,2,1), (0,3,2)
     0,  2,  1,   0,  3,  2,
    // +Z face (verts 4-7): tri (4,5,6), (4,6,7)
     4,  5,  6,   4,  6,  7,
    // -Y face (verts 8-11): tri (8,9,11), (8,11,10)
     8,  9, 11,   8, 11, 10,
    // +Y face (verts 12-15): tri (12,15,13), (12,14,15)
    12, 15, 13,  12, 14, 15,
    // -X face (verts 16-19): tri (16,18,19), (16,19,17)
    16, 18, 19,  16, 19, 17,
    // +X face (verts 20-23): tri (20,21,23), (20,23,22)
    20, 21, 23,  20, 23, 22,
];

// Unit rectangle: 4 vertices at ±1 on the XY plane. RECT_TRI_INDICES
// reproduces the legacy rect / plane winding (top-left, top-right,
// bottom-left, bottom-right). Shared between rect / rectb / plane.
const UNIT_RECT_POSITIONS: [f32; 12] = [
    -1.0, 1.0, 0.0, // top-left
    1.0, 1.0, 0.0, // top-right
    -1.0, -1.0, 0.0, // bottom-left
    1.0, -1.0, 0.0, // bottom-right
];
const RECT_TRI_INDICES: [i32; 6] = [0, 1, 2, 1, 3, 2];
const RECT_EDGE_INDICES: [i32; 8] = [0, 1, 1, 3, 3, 2, 2, 0];

// Base icosahedron: 12 vertices on the unit sphere (|v| = 1), 20
// outward triangles (CCW from outside), 30 edges. Coordinates use
// (1/n, t/n, 0) permutations where t = (1 + √5) / 2 and n = √(1 + t²)
// ≈ 1.902. Used as the seed for the level-1 subdivision below.
const ICOSA_BASE_POSITIONS: [f32; 36] = [
    -0.525_731_1,
    0.850_650_8,
    0.0,
    0.525_731_1,
    0.850_650_8,
    0.0,
    -0.525_731_1,
    -0.850_650_8,
    0.0,
    0.525_731_1,
    -0.850_650_8,
    0.0,
    0.0,
    -0.525_731_1,
    0.850_650_8,
    0.0,
    0.525_731_1,
    0.850_650_8,
    0.0,
    -0.525_731_1,
    -0.850_650_8,
    0.0,
    0.525_731_1,
    -0.850_650_8,
    0.850_650_8,
    0.0,
    -0.525_731_1,
    0.850_650_8,
    0.0,
    0.525_731_1,
    -0.850_650_8,
    0.0,
    -0.525_731_1,
    -0.850_650_8,
    0.0,
    0.525_731_1,
];
const ICOSA_BASE_TRI_INDICES: [i32; 60] = [
    0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, 1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7, 1,
    8, 3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, 4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9, 8, 1,
];
const ICOSA_BASE_EDGE_INDICES: [i32; 60] = [
    0, 1, 0, 5, 0, 7, 0, 10, 0, 11, 1, 5, 1, 7, 1, 8, 1, 9, 2, 3, 2, 4, 2, 6, 2, 10, 2, 11, 3, 4,
    3, 6, 3, 8, 3, 9, 4, 5, 4, 9, 4, 11, 5, 9, 5, 11, 6, 7, 6, 8, 6, 10, 7, 8, 7, 10, 8, 9, 10, 11,
];

// Level-1 subdivision: each base triangle splits into 4 sub-triangles
// via edge-midpoint insertion; midpoints are re-projected onto the unit
// sphere. Yields 42 vertices / 80 triangles / 120 edges — bigger than
// the bare icosahedron but small enough to stay retro and read as a
// sphere instead of a 6-sided silhouette.
fn unit_icosa_lv1_positions() -> &'static [f32; 126] {
    static POSITIONS: OnceLock<[f32; 126]> = OnceLock::new();
    POSITIONS.get_or_init(|| {
        let mut p = [0.0_f32; 126];
        p[..36].copy_from_slice(&ICOSA_BASE_POSITIONS);
        for (edge_index, edge_pair) in ICOSA_BASE_EDGE_INDICES.chunks(2).enumerate() {
            let a = edge_pair[0] as usize;
            let b = edge_pair[1] as usize;
            let mx = (ICOSA_BASE_POSITIONS[a * 3] + ICOSA_BASE_POSITIONS[b * 3]) * 0.5;
            let my = (ICOSA_BASE_POSITIONS[a * 3 + 1] + ICOSA_BASE_POSITIONS[b * 3 + 1]) * 0.5;
            let mz = (ICOSA_BASE_POSITIONS[a * 3 + 2] + ICOSA_BASE_POSITIONS[b * 3 + 2]) * 0.5;
            let inv_len = (mx * mx + my * my + mz * mz).sqrt().recip();
            let dst = 36 + edge_index * 3;
            p[dst] = mx * inv_len;
            p[dst + 1] = my * inv_len;
            p[dst + 2] = mz * inv_len;
        }
        p
    })
}

// Locate the midpoint vertex index for base edge (a, b). 30 entries —
// a linear scan is fine for the init-time builders.
fn icosa_midpoint_vertex(a: i32, b: i32) -> i32 {
    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
    for (edge_index, edge_pair) in ICOSA_BASE_EDGE_INDICES.chunks(2).enumerate() {
        let (ea, eb) = (edge_pair[0], edge_pair[1]);
        let (e_lo, e_hi) = if ea < eb { (ea, eb) } else { (eb, ea) };
        if e_lo == lo && e_hi == hi {
            return 12 + edge_index as i32;
        }
    }
    unreachable!("icosa edge ({a}, {b}) not in ICOSA_BASE_EDGE_INDICES")
}

fn unit_icosa_lv1_tri_indices() -> &'static [i32; 240] {
    static INDICES: OnceLock<[i32; 240]> = OnceLock::new();
    INDICES.get_or_init(|| {
        let mut out = [0_i32; 240];
        for (tri_index, tri) in ICOSA_BASE_TRI_INDICES.chunks(3).enumerate() {
            let (a, b, c) = (tri[0], tri[1], tri[2]);
            let m_ab = icosa_midpoint_vertex(a, b);
            let m_bc = icosa_midpoint_vertex(b, c);
            let m_ca = icosa_midpoint_vertex(c, a);
            // 4 sub-triangles preserve the base triangle's CCW winding.
            let dst = tri_index * 12;
            out[dst..dst + 3].copy_from_slice(&[a, m_ab, m_ca]);
            out[dst + 3..dst + 6].copy_from_slice(&[b, m_bc, m_ab]);
            out[dst + 6..dst + 9].copy_from_slice(&[c, m_ca, m_bc]);
            out[dst + 9..dst + 12].copy_from_slice(&[m_ab, m_bc, m_ca]);
        }
        out
    })
}

fn unit_icosa_lv1_edge_indices() -> &'static [i32; 240] {
    static INDICES: OnceLock<[i32; 240]> = OnceLock::new();
    INDICES.get_or_init(|| {
        let mut out = [0_i32; 240];
        let mut cursor = 0;
        // 60 sub-edges: each base edge splits at its midpoint into 2 halves.
        for (edge_index, edge_pair) in ICOSA_BASE_EDGE_INDICES.chunks(2).enumerate() {
            let (a, b) = (edge_pair[0], edge_pair[1]);
            let m = 12 + edge_index as i32;
            out[cursor] = a;
            out[cursor + 1] = m;
            out[cursor + 2] = m;
            out[cursor + 3] = b;
            cursor += 4;
        }
        // 60 internal edges: 3 mid-mid edges per base triangle.
        for tri in ICOSA_BASE_TRI_INDICES.chunks(3) {
            let (a, b, c) = (tri[0], tri[1], tri[2]);
            let m_ab = icosa_midpoint_vertex(a, b);
            let m_bc = icosa_midpoint_vertex(b, c);
            let m_ca = icosa_midpoint_vertex(c, a);
            out[cursor] = m_ab;
            out[cursor + 1] = m_bc;
            out[cursor + 2] = m_bc;
            out[cursor + 3] = m_ca;
            out[cursor + 4] = m_ca;
            out[cursor + 5] = m_ab;
            cursor += 6;
        }
        out
    })
}

// Unit ellipse / circle on the XY plane: center vertex 0 + ELLIPSE_SEGMENTS
// perimeter vertices at radius 1. Per-call (w, h) becomes (hw, hh) and is
// folded into the world matrix as XY scale, so a 2-by-1 ellipse is a
// single mat scale rather than a per-vertex multiply.
const ELLIPSE_TRI_INDICES: [i32; 72] = [
    0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 7, 0, 7, 8, 0, 8, 9, 0, 9, 10, 0, 10, 11, 0,
    11, 12, 0, 12, 13, 0, 13, 14, 0, 14, 15, 0, 15, 16, 0, 16, 17, 0, 17, 18, 0, 18, 19, 0, 19, 20,
    0, 20, 21, 0, 21, 22, 0, 22, 23, 0, 23, 24, 0, 24, 1,
];
const ELLIPSE_EDGE_INDICES: [i32; 48] = [
    1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15,
    15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 1,
];

// Lazily-built unit ellipse positions: index 0 is the center, indices
// 1..=ELLIPSE_SEGMENTS are perimeter points at (cos θ, sin θ, 0).
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

// Compose `world_mat` with a per-axis scale on the linear (3x3) part,
// leaving the translation column intact. Used by primitive draw helpers
// that fold size / radius into the matrix instead of the vertex data.
fn scale_axes(world_mat: &Mat4, sx: f32, sy: f32, sz: f32) -> Mat4 {
    let mut out = *world_mat;
    for row in 0..3 {
        out.data[row][0] *= sx;
        out.data[row][1] *= sy;
        out.data[row][2] *= sz;
    }
    out
}

// Compose `world_mat` with a translation by `local` (interpreted in
// world_mat's local coordinates), leaving the linear part intact.
fn translate_local(world_mat: &Mat4, local: &Vec3) -> Mat4 {
    let translated = mat_apply(world_mat, local);
    let mut out = *world_mat;
    out.data[0][3] = translated.x;
    out.data[1][3] = translated.y;
    out.data[2][3] = translated.z;
    out
}

// sphere / sphereb: level-1 subdivided icosahedron (42 vertices / 80
// triangles / 120 edges) centered at `local`, scaled by `r`. Folds the
// radius into the world matrix as uniform scale.
//
// When `col_image` is Some, the textured path is used with equirectangular
// (lat/long) UV mapping:
//   u = atan2(z, x) / (2π) + 0.5   (longitude, seam at x<0 meridian)
//   v = 0.5 - asin(y) / π           (latitude, v=0 north pole, v=1 south pole)
//
// Vertices on the seam (u≈0 or u≈1) are duplicated in the textured path so
// that triangles straddling the seam get the correct u=0/u=1 split. For the
// flat-color path, no UV computation or vertex duplication is needed.
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

    if col_image.is_some() {
        // Textured path: compute per-vertex equirectangular UVs and handle
        // the seam. Triangles whose vertices span the u=0/u=1 seam are
        // rebuilt with duplicated vertices so the u coordinate is
        // consistent on each side (u=0 on the left, u=1 on the right).

        let base_positions = unit_icosa_lv1_positions();
        let base_indices = unit_icosa_lv1_tri_indices();
        let vertex_count = base_positions.len() / 3;

        // Compute UV for every base vertex using:
        //   u = atan2(z, x) / (2π) + 0.5  (lon in [-π, π] → [0, 1])
        //   v = 0.5 - asin(y) / π          (lat in [-π/2, π/2] → [1, 0])
        let mut base_uvs = Vec::with_capacity(vertex_count * 2);
        for i in 0..vertex_count {
            let bx = base_positions[i * 3];
            let by = base_positions[i * 3 + 1];
            let bz = base_positions[i * 3 + 2];
            let u = bz.atan2(bx) / (2.0 * std::f32::consts::PI) + 0.5;
            // v=0 at the north pole: the sampler maps v=0 to image row 0
            // (top), so the texture's top wraps onto the sphere's top.
            let v = 0.5 - by.asin() / std::f32::consts::PI;
            base_uvs.push(u);
            base_uvs.push(v);
        }

        // Build output buffers, expanding seam-straddling triangles by
        // duplicating vertices with corrected u coordinates.
        let face_count = base_indices.len() / 3;
        let mut out_positions: Vec<f32> = Vec::with_capacity(base_positions.len());
        let mut out_uvs: Vec<f32> = Vec::with_capacity(base_uvs.len());
        let mut out_indices: Vec<i32> = Vec::with_capacity(base_indices.len());

        // Map (base_vertex_index, seam_side) → output index.
        // seam_side: 0 = normal, 1 = duplicated with u adjusted toward 1.
        let mut vertex_map: Vec<[Option<i32>; 2]> = vec![[None; 2]; vertex_count];

        let get_or_add = |out_positions: &mut Vec<f32>,
                          out_uvs: &mut Vec<f32>,
                          vertex_map: &mut Vec<[Option<i32>; 2]>,
                          base_idx: usize,
                          seam_side: usize|
         -> i32 {
            if let Some(existing) = vertex_map[base_idx][seam_side] {
                return existing;
            }
            let new_idx = (out_positions.len() / 3) as i32;
            let bx = base_positions[base_idx * 3];
            let by = base_positions[base_idx * 3 + 1];
            let bz = base_positions[base_idx * 3 + 2];
            out_positions.push(bx);
            out_positions.push(by);
            out_positions.push(bz);
            let mut u = base_uvs[base_idx * 2];
            let v = base_uvs[base_idx * 2 + 1];
            // seam_side=1 means the vertex should appear on the u=1 side.
            if seam_side == 1 && u < 0.5 {
                u += 1.0;
            }
            out_uvs.push(u);
            out_uvs.push(v);
            vertex_map[base_idx][seam_side] = Some(new_idx);
            new_idx
        };

        for f in 0..face_count {
            let i0 = base_indices[f * 3] as usize;
            let i1 = base_indices[f * 3 + 1] as usize;
            let i2 = base_indices[f * 3 + 2] as usize;
            let u0 = base_uvs[i0 * 2];
            let u1 = base_uvs[i1 * 2];
            let u2 = base_uvs[i2 * 2];

            // Check if this triangle straddles the seam. The seam runs
            // along the atan2 discontinuity (u≈0 / u≈1 boundary at z≈0,
            // x<0). When the max-min u spread across the three vertices
            // exceeds 0.5 the triangle must be split at the seam.
            let u_min = u0.min(u1).min(u2);
            let u_max = u0.max(u1).max(u2);
            let straddles_seam = (u_max - u_min) > 0.5;

            let (s0, s1, s2) = if straddles_seam {
                // Vertices with low u (< 0.5) are on the left side of the
                // seam; assign them seam_side=1 so u is bumped to ~1.
                let side = |u: f32| usize::from(u < 0.5);
                (side(u0), side(u1), side(u2))
            } else {
                (0, 0, 0)
            };

            let o0 = get_or_add(&mut out_positions, &mut out_uvs, &mut vertex_map, i0, s0);
            let o1 = get_or_add(&mut out_positions, &mut out_uvs, &mut vertex_map, i1, s1);
            let o2 = get_or_add(&mut out_positions, &mut out_uvs, &mut vertex_map, i2, s2);
            out_indices.push(o0);
            out_indices.push(o1);
            out_indices.push(o2);
        }

        let _ = prim(
            ctx,
            &scaled,
            MODE_TRIANGLES,
            CULL_NONE,
            &out_positions,
            Some(&out_indices),
            None,
            Some(&out_uvs),
            col_flat,
            col_image,
            colkey,
            state,
        );
    } else {
        // Flat-color path: keep the original shared-vertex layout with
        // zero per-call allocation.
        let _ = prim(
            ctx,
            &scaled,
            MODE_TRIANGLES,
            CULL_NONE,
            unit_icosa_lv1_positions(),
            Some(unit_icosa_lv1_tri_indices()),
            None,
            None,
            col_flat,
            None,
            None,
            state,
        );
    }
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
    let _ = prim(
        ctx,
        &scaled,
        MODE_LINES,
        CULL_NONE,
        unit_icosa_lv1_positions(),
        Some(unit_icosa_lv1_edge_indices()),
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
        &world, r, &ctx.vp, camera, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h,
    );
    if let Some((sx, sy, sr, sz)) = projected {
        let sz = project_offset(
            &world, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h, &z_shift,
        )
        .map_or(sz, |p| p.2);
        let target_mut = rc_mut!(&ctx.target);
        let depth_w = ctx.depth_w;
        rasterize_circle_filled(
            target_mut,
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
        &world, r, &ctx.vp, camera, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h,
    );
    if let Some((sx, sy, sr, sz)) = projected {
        let sz = project_offset(
            &world, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h, &z_shift,
        )
        .map_or(sz, |p| p.2);
        let target_mut = rc_mut!(&ctx.target);
        let depth_w = ctx.depth_w;
        rasterize_circle_border(
            target_mut,
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

// sprite is a billboard quad oriented to face the camera. Corners are
// computed in world space here; the prim call uses an identity world
// transform because the corners are already world-positioned.
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
    let camera = rc_ref!(&ctx.camera);
    let corners = sprite_corners(&world, w, h, angle, camera);
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
    // sprite uses an identity world_mat (corners are already world-space)
    // and disables billboard rewriting (the corners themselves are the
    // billboard). It also forces `shaded = false`: a camera-facing
    // billboard has no meaningful lit normal, so sprites render unshaded
    // by spec (decoration / particle default; see cube-design.md shaded
    // defaults).
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
    let uv_array = [
        uvs.0 .0, uvs.0 .1, uvs.1 .0, uvs.1 .1, uvs.2 .0, uvs.2 .1, uvs.3 .0, uvs.3 .1,
    ];
    let _ = prim(
        ctx,
        &scaled,
        MODE_TRIANGLES,
        CULL_NONE,
        &UNIT_RECT_POSITIONS,
        Some(&RECT_TRI_INDICES),
        None,
        Some(&uv_array),
        0,
        Some(img),
        colkey,
        state,
    );
}

// Draw a hierarchical Mesh asset. Each part's world transform is
// composed in topological order (parents[i] < i is validated at Mesh
// construction). Per-part vertex / index / uv / normal data and the
// prim / cull mode come from the part's Primitive; col_img and colkey
// are shared across the whole mesh.
pub fn mesh(ctx: &mut DrawContext, world_mat: &Mat4, mesh: &Mesh, state: DrawState) {
    if mesh.primitives.is_empty() {
        return;
    }
    let world = mesh.compose_world_transforms(world_mat);
    let (col_flat, col_image) = mesh.col_img.as_flat_and_image();
    for (i, prim_opt) in mesh.primitives.iter().enumerate() {
        let Some(prim_rc) = prim_opt.as_ref() else {
            continue;
        };
        let g = rc_ref!(prim_rc);
        if g.positions.is_empty() {
            continue;
        }
        let _ = prim(
            ctx,
            &world[i],
            g.mode,
            g.cull,
            &g.positions,
            if g.indices.is_empty() {
                None
            } else {
                Some(g.indices.as_slice())
            },
            if g.normals.is_empty() {
                None
            } else {
                Some(g.normals.as_slice())
            },
            if g.uvs.is_empty() {
                None
            } else {
                Some(g.uvs.as_slice())
            },
            col_flat,
            col_image.as_ref(),
            mesh.colkey,
            state,
        );
    }
}

// Text rendering uses Vec3-positioned, screen-space glyphs.
// `pos` is projected to screen, then each visible glyph pixel
// is plotted through `write_pixel` at the glyph's screen offset.
// Always camera-facing; ancestor rotation / scale do not affect
// glyph layout (cube-design.md § 12.5).

// Compute pixel-space bounding box and emit each visible glyph pixel
// to `out` in a single pass that borrows `font` only once. Returns
// (text_w, text_h) of the glyph cluster origin-anchored at (0, 0).
// Used by `text` to recenter the glyph cluster around the projected
// screen point.
fn collect_text_geometry(
    mut font: Option<&mut Font>,
    text: &str,
    out: &mut Vec<(i32, i32)>,
) -> (i32, i32) {
    // Phase 1: measure (consumes the borrow only within this block).
    let (text_w, line_height) = if let Some(font) = font.as_deref_mut() {
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
    let text_h = line_count * line_height;
    // Phase 2: walk pixels (now we can move `font`).
    if let Some(font) = font {
        font.for_each_pixel(0, 0, text, |px, py| out.push((px, py)));
    } else {
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
                        out.push((cur_x + fx as i32, cur_y + fy as i32));
                    }
                }
            }
            cur_x += FONT_WIDTH as i32;
        }
    }
    (text_w, text_h)
}

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
        &world, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h, &z_shift,
    );
    let Some((sx_f, sy_f, sz)) = projected else {
        return;
    };
    let sx = sx_f.round() as i32;
    let sy = sy_f.round() as i32;
    let mut pixel_xy: Vec<(i32, i32)> = Vec::new();
    let (text_w, text_h) = collect_text_geometry(font, text_str, &mut pixel_xy);
    if text_w == 0 || text_h == 0 || pixel_xy.is_empty() {
        return;
    }
    // Center pivot: shift glyph cluster so its bounding box centers on
    // the projected screen point. Glyphs render in 2D pixels; depth uses
    // the projected z so depth_test / depth_write still apply.
    let cx = sx - text_w / 2;
    let cy = sy - text_h / 2;
    let target_mut = rc_mut!(&ctx.target);
    let depth_w = ctx.depth_w;
    let depth = ctx.depth.as_mut_slice();
    for (px, py) in pixel_xy {
        let x = cx + px;
        let y = cy + py;
        if x < ctx.clip.left || x > ctx.clip.right || y < ctx.clip.top || y > ctx.clip.bottom {
            continue;
        }
        write_pixel(
            target_mut,
            depth,
            depth_w,
            x,
            y,
            sz,
            col as u8,
            state.dither_alpha,
            state.depth_test,
            state.depth_write,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_screen_area_ccw_positive() {
        // CCW in Y-down screen: (0,0), (1,0), (0,1) → triangle pointing
        // away from camera with +Y down has positive signed area.
        let area = signed_screen_area((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        assert!(area > 0.0);
    }

    #[test]
    fn test_signed_screen_area_cw_negative() {
        // CW winding produces negative signed area.
        let area = signed_screen_area((0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 0.0, 0.0));
        assert!(area < 0.0);
    }

    #[test]
    fn test_signed_screen_area_degenerate_zero() {
        // Collinear points produce zero signed area.
        let area = signed_screen_area((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (2.0, 0.0, 0.0));
        assert_eq!(area, 0.0);
    }

    #[test]
    fn test_should_cull_back_skips_back_face() {
        // CULL_BACK: skip when area >= 0 (back-facing or degenerate) under
        // CCW-front + Y-down convention.
        assert!(should_cull(1.0, CULL_BACK));
        assert!(should_cull(0.0, CULL_BACK));
        assert!(!should_cull(-1.0, CULL_BACK));
    }

    #[test]
    fn test_should_cull_front_skips_front_face() {
        // CULL_FRONT: skip when area <= 0 (front-facing or degenerate).
        assert!(should_cull(-1.0, CULL_FRONT));
        assert!(should_cull(0.0, CULL_FRONT));
        assert!(!should_cull(1.0, CULL_FRONT));
    }

    #[test]
    fn test_should_cull_none_draws_everything() {
        // CULL_NONE: never skip, regardless of area sign.
        assert!(!should_cull(1.0, CULL_NONE));
        assert!(!should_cull(-1.0, CULL_NONE));
        assert!(!should_cull(0.0, CULL_NONE));
    }

    fn near_clip_test_vp() -> [[f32; 4]; 4] {
        let mut vp = [[0.0; 4]; 4];
        vp[3][2] = -1.0;
        vp
    }

    fn clip_vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> ClipVertex {
        ClipVertex {
            world: Vec3 { x, y, z },
            uv: (u, v),
        }
    }

    fn assert_clip_vertices_inside(vertices: &ClippedTriangle, vp: &[[f32; 4]; 4]) {
        for i in 0..vertices.len {
            let w = clip_w(&vertices.vertices[i].world, vp);
            assert!(w >= CLIP_W_EPSILON - 1e-6, "vertex {i} w={w}");
        }
    }

    #[test]
    fn clip_triangle_to_near_keeps_all_inside_vertices() {
        let vp = near_clip_test_vp();
        let vertices = [
            clip_vertex(-1.0, -1.0, -1.0, 0.0, 0.0),
            clip_vertex(1.0, -1.0, -1.0, 1.0, 0.0),
            clip_vertex(0.0, 1.0, -1.0, 0.5, 1.0),
        ];
        let clipped = clip_triangle_to_near(vertices, &vp);

        assert_eq!(clipped.len, 3);
        for (actual, expected) in clipped.vertices.iter().zip(vertices) {
            assert_eq!(actual.world, expected.world);
            assert_eq!(actual.uv, expected.uv);
        }
    }

    #[test]
    fn clip_triangle_to_near_rejects_all_behind_vertices() {
        let vp = near_clip_test_vp();
        let vertices = [
            clip_vertex(-1.0, -1.0, 1.0, 0.0, 0.0),
            clip_vertex(1.0, -1.0, 1.0, 1.0, 0.0),
            clip_vertex(0.0, 1.0, 1.0, 0.5, 1.0),
        ];
        let clipped = clip_triangle_to_near(vertices, &vp);

        assert_eq!(clipped.len, 0);
    }

    #[test]
    fn clip_triangle_to_near_one_vertex_behind_returns_quad() {
        let vp = near_clip_test_vp();
        let vertices = [
            clip_vertex(-1.0, -1.0, -1.0, 0.0, 0.0),
            clip_vertex(1.0, -1.0, -1.0, 1.0, 0.0),
            clip_vertex(0.0, 1.0, 1.0, 0.5, 1.0),
        ];
        let clipped = clip_triangle_to_near(vertices, &vp);

        assert_eq!(clipped.len, 4);
        assert_clip_vertices_inside(&clipped, &vp);
    }

    #[test]
    fn clip_triangle_to_near_two_vertices_behind_returns_triangle() {
        let vp = near_clip_test_vp();
        let vertices = [
            clip_vertex(-1.0, -1.0, -1.0, 0.0, 0.0),
            clip_vertex(1.0, -1.0, 1.0, 1.0, 0.0),
            clip_vertex(0.0, 1.0, 1.0, 0.5, 1.0),
        ];
        let clipped = clip_triangle_to_near(vertices, &vp);

        assert_eq!(clipped.len, 3);
        assert_clip_vertices_inside(&clipped, &vp);
    }

    #[test]
    fn depth_offset_negative_moves_toward_camera() {
        use crate::cube::camera::Camera;
        use crate::cube::raster::{matmul, projection_matrix, view_matrix};
        let camera = Camera::new();
        let v = view_matrix(rc_ref!(&camera));
        let p = projection_matrix(rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        // Off-axis point in front of the default (-Z looking) camera.
        let pos = Vec3 {
            x: 1.0,
            y: 0.5,
            z: -3.0,
        };
        let base = world_to_screen(&pos, &vp, 0.0, 0.0, 256.0, 192.0).unwrap();
        // Negative offset = toward the camera: depth shrinks, screen x/y
        // stay put (the offset must not move the draw).
        let near = depth_offset_shift(&camera, -0.5);
        let near_p = project_offset(&pos, &vp, 0.0, 0.0, 256.0, 192.0, &near).unwrap();
        assert!((near_p.0 - base.0).abs() < 1e-4);
        assert!((near_p.1 - base.1).abs() < 1e-4);
        assert!(near_p.2 < base.2);
        // Positive offset = away: depth grows.
        let far = depth_offset_shift(&camera, 0.5);
        let far_p = project_offset(&pos, &vp, 0.0, 0.0, 256.0, 192.0, &far).unwrap();
        assert!(far_p.2 > base.2);
    }

    #[test]
    fn depth_offset_zero_is_identity() {
        use crate::cube::camera::Camera;
        use crate::cube::raster::{matmul, projection_matrix, view_matrix};
        let camera = Camera::new();
        let v = view_matrix(rc_ref!(&camera));
        let p = projection_matrix(rc_ref!(&camera), 256.0, 192.0);
        let vp = matmul(&p, &v);
        let pos = Vec3 {
            x: 1.0,
            y: 0.5,
            z: -3.0,
        };
        let base = world_to_screen(&pos, &vp, 0.0, 0.0, 256.0, 192.0).unwrap();
        let zero = depth_offset_shift(&camera, 0.0);
        let same = project_offset(&pos, &vp, 0.0, 0.0, 256.0, 192.0, &zero).unwrap();
        assert_eq!(base, same);
    }

    #[test]
    fn line_clips_endpoint_behind_camera_instead_of_dropping() {
        use crate::cube::camera::Camera;
        use crate::cube::raster::{compute_clip_rect, matmul, projection_matrix, view_matrix};
        use crate::cube::scene::DrawContext;

        let target = Image::new(64, 64);
        rc_mut!(&target).clear(2);
        let camera = Camera::new();
        let vp = matmul(
            &projection_matrix(rc_ref!(&camera), 64.0, 64.0),
            &view_matrix(rc_ref!(&camera)),
        );
        let clip = compute_clip_rect(0.0, 0.0, 64.0, 64.0, 64, 64);
        let mut ctx = DrawContext {
            target: target.clone(),
            vp,
            vp_x: 0.0,
            vp_y: 0.0,
            vp_w: 64.0,
            vp_h: 64.0,
            clip,
            camera,
            depth: vec![f32::INFINITY; 64 * 64],
            depth_w: 64,
            depth_h: 64,
            vertex_cache: Vec::new(),
            dither_alpha: 1.0,
            depth_test: true,
            depth_write: true,
            depth_offset: 0.0,
            shaded: false,
        };
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
    fn triangle_clips_vertex_behind_camera_instead_of_dropping() {
        use crate::cube::camera::Camera;
        use crate::cube::raster::{compute_clip_rect, matmul, projection_matrix, view_matrix};
        use crate::cube::scene::DrawContext;

        let target = Image::new(64, 64);
        rc_mut!(&target).clear(2);
        let camera = Camera::new();
        let vp = matmul(
            &projection_matrix(rc_ref!(&camera), 64.0, 64.0),
            &view_matrix(rc_ref!(&camera)),
        );
        let clip = compute_clip_rect(0.0, 0.0, 64.0, 64.0, 64, 64);
        let mut ctx = DrawContext {
            target: target.clone(),
            vp,
            vp_x: 0.0,
            vp_y: 0.0,
            vp_w: 64.0,
            vp_h: 64.0,
            clip,
            camera,
            depth: vec![f32::INFINITY; 64 * 64],
            depth_w: 64,
            depth_h: 64,
            vertex_cache: Vec::new(),
            dither_alpha: 1.0,
            depth_test: true,
            depth_write: true,
            depth_offset: 0.0,
            shaded: false,
        };
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
    fn box_solid_culls_back_faces() {
        use crate::cube::camera::Camera;
        use crate::cube::raster::{compute_clip_rect, matmul, projection_matrix, view_matrix};
        use crate::cube::scene::DrawContext;

        let target = Image::new(64, 64);
        rc_mut!(&target).clear(2);
        let camera = Camera::new();
        let vp = matmul(
            &projection_matrix(rc_ref!(&camera), 64.0, 64.0),
            &view_matrix(rc_ref!(&camera)),
        );
        let clip = compute_clip_rect(0.0, 0.0, 64.0, 64.0, 64, 64);
        let mut ctx = DrawContext {
            target: target.clone(),
            vp,
            vp_x: 0.0,
            vp_y: 0.0,
            vp_w: 64.0,
            vp_h: 64.0,
            clip,
            camera,
            depth: vec![f32::INFINITY; 64 * 64],
            depth_w: 64,
            depth_h: 64,
            vertex_cache: Vec::new(),
            dither_alpha: 1.0,
            depth_test: true,
            depth_write: true,
            depth_offset: 0.0,
            shaded: false,
        };

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
    fn shaded_stored_normals_track_rotation() {
        // Regression: stored (model-space) normals must be rotated into
        // world space before shading, so a shaded draw on a rotated
        // transform lights the same as the auto path (which derives a
        // world-space normal from the rotated vertices). Without the
        // transform a rotated mesh keeps its unrotated lighting.
        use crate::cube::camera::Camera;
        use crate::cube::primitive::Primitive;
        use crate::cube::raster::{compute_clip_rect, matmul, projection_matrix, view_matrix};
        use crate::cube::scene::DrawContext;

        let palette: Vec<crate::image::Rgb24> = vec![
            0x000000, 0x2B335F, 0x7E2072, 0x19959C, 0x8B4852, 0x395C98, 0xA9C1FF, 0xEEEEEE,
            0xD4186C, 0xD38441, 0xE9C35B, 0x70C6A9, 0x7696DE, 0xA3A3A3, 0xFF9798, 0xEDC7B0,
        ];
        let shading_rc = Shading::new(&palette);
        rc_mut!(&shading_rc).direction = Vec3::new(0.0, 0.0, -1.0); // light travels -Z

        // A 4×4 quad in the model XY plane (two triangles). compute_normals
        // fills per-face model-space normals.
        let geom = Primitive::new();
        {
            let g = rc_mut!(&geom);
            g.positions = vec![
                -2.0, 2.0, 0.0, 2.0, 2.0, 0.0, -2.0, -2.0, 0.0, 2.0, -2.0, 0.0,
            ];
            g.indices = vec![0, 1, 2, 1, 3, 2];
            g.cull = CULL_NONE;
            g.compute_normals();
        }
        let model_normals = rc_ref!(&geom).normals.clone();
        let positions = rc_ref!(&geom).positions.clone();
        let indices = rc_ref!(&geom).indices.clone();

        // world = translate(0, 0, -3) * rotateY(180): the quad sits in front
        // of the camera with its face normal flipped by the spin.
        let spin = Mat4::from_axis_angle(rc_ref!(&Vec3::new(0.0, 1.0, 0.0)), 180.0);
        let trans = Mat4::from_translation(rc_ref!(&Vec3::new(0.0, 0.0, -3.0)));
        let world_rc = rc_ref!(&trans).mul_mat(rc_ref!(&spin));
        let world = *rc_ref!(&world_rc);

        // The model normal and its world-space image must fall on different
        // shade levels, otherwise the test could not tell the two apart.
        let model_n = Vec3 {
            x: model_normals[0],
            y: model_normals[1],
            z: model_normals[2],
        };
        let world_n = mat_apply_dir(&world, &model_n);
        let dir = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_ne!(
            face_shade_level(&dir, Some(&world_n)),
            face_shade_level(&dir, Some(&model_n)),
            "test setup must distinguish world-space vs model-space normal"
        );

        let render = |normals: Option<&[f32]>| -> u8 {
            let target = Image::new(64, 64);
            rc_mut!(&target).clear(2); // sentinel so an undrawn quad is detectable
            let camera = Camera::new();
            let vp = matmul(
                &projection_matrix(rc_ref!(&camera), 64.0, 64.0),
                &view_matrix(rc_ref!(&camera)),
            );
            let clip = compute_clip_rect(0.0, 0.0, 64.0, 64.0, 64, 64);
            let mut ctx = DrawContext {
                target: target.clone(),
                vp,
                vp_x: 0.0,
                vp_y: 0.0,
                vp_w: 64.0,
                vp_h: 64.0,
                clip,
                camera: camera.clone(),
                depth: vec![f32::INFINITY; 64 * 64],
                depth_w: 64,
                depth_h: 64,
                vertex_cache: Vec::new(),
                dither_alpha: 1.0,
                depth_test: true,
                depth_write: true,
                depth_offset: 0.0,
                shaded: true,
            };
            let shading_ref = rc_ref!(&shading_rc);
            let state = DrawState {
                shaded: true,
                dither_alpha: 1.0,
                depth_test: true,
                depth_write: true,
                billboard: BILLBOARD_OFF,
                shading: Some(shading_ref),
            };
            prim(
                &mut ctx,
                &world,
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
            rc_ref!(&target).pixel(32.0, 32.0)
        };

        let auto = render(None);
        let stored = render(Some(&model_normals));
        assert_ne!(auto, 2, "quad must cover the center pixel");
        assert_eq!(
            stored, auto,
            "stored-normal shading must match the auto path under rotation"
        );
    }
}
