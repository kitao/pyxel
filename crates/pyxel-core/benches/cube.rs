// Micro-benchmarks for the cube hot paths listed in docs/coding-policy.md:
// per-pixel 3D rasterization and per-frame 3D collision / BVH queries.
// No external harness: each benchmark warms up, times batched iterations
// with Instant, and prints the median ns/iter over SAMPLE_COUNT samples.
// All inputs are fixed so runs are deterministic. Run with:
// cargo bench -p pyxel-core --features sdl2_static

use std::hint::black_box;
use std::time::Instant;

use pyxel::cube::bvh::Bvh;
use pyxel::cube::collision::Aabb;
use pyxel::cube::draw::{self, DrawState, BILLBOARD_OFF};
use pyxel::cube::motion::{MotionChannel, MotionInterpolation, MotionTarget, MotionValues};
use pyxel::cube::primitive::{CULL_NONE, MODE_TRIANGLES};
use pyxel::cube::raster::{
    camera_clip_row, compute_clip_rect, matmul, projection_matrix, view_matrix,
};
use pyxel::cube::scene::{DrawContext, Scene};
use pyxel::cube::{
    Camera, Collider, Mat4, Mesh, Motion, Node, Primitive, Quat, RcMesh, RcNode, Shading, Vec3,
};
use pyxel::{Image, RcImage, Rgb24};

// Mirrors of the crate-internal rc_ref! / rc_mut! macros (utils.rs), which
// are not exported; benches use the public checked shared-owner aliases.
macro_rules! rc_ref {
    ($rc:expr) => {
        ($rc).borrow()
    };
}

macro_rules! rc_mut {
    ($rc:expr) => {
        ($rc).borrow_mut()
    };
}

// Timed samples per benchmark; the median over them is reported.
const SAMPLE_COUNT: usize = 17;

// Default Pyxel 16-color palette for the Shading fixture.
const PALETTE: [Rgb24; 16] = [
    0x000000, 0x2B335F, 0x7E2072, 0x19959C, 0x8B4852, 0x395C98, 0xA9C1FF, 0xEEEEEE, 0xD4186C,
    0xD38441, 0xE9C35B, 0x70C6A9, 0x7696DE, 0xA3A3A3, 0xFF9798, 0xEDC7B0,
];

// Raster target size in pixels; the benchmark triangle spans a 64x64-px
// bounding box inside it.
const TARGET_SIZE: u32 = 80;

// Grid mesh resolution: GRID_DIVISIONS^2 cells x 2 = 2048 triangles.
const GRID_DIVISIONS: u32 = 32;
const GRID_EXTENT: f32 = 8.0;

// Right triangle whose projection spans a 64x64-px bounding box at
// z = -2 under the default 60-degree camera on the 80x80 viewport.
const TRIANGLE_POSITIONS: [f32; 9] = [-0.92, -0.92, -2.0, 0.92, -0.92, -2.0, -0.92, 0.92, -2.0];

fn main() {
    bench_raster_flat_triangle();
    bench_raster_flat_offscreen();
    bench_raster_textured_shaded_triangle();
    bench_raster_textured_shaded_occluded();
    bench_raster_textured_shaded_large();
    bench_bvh_query_ray();
    bench_bvh_query_aabb();
    bench_mesh_aabb_from_mesh();
    bench_motion_sample();
    bench_node_find_by_tags();
    bench_scene_walk_contacts();
}

// Benchmarks

// One flat unshaded triangle per iteration through draw::prim, the public
// entry above rasterize_triangle. The depth refill plays the per-frame
// depth clear so every iteration rasterizes identical pixels.
fn bench_raster_flat_triangle() {
    let mut ctx = make_draw_context(TARGET_SIZE);
    let world = Mat4::identity_value();
    run_bench("raster_flat_triangle", 6_000, |_| {
        ctx.depth.fill(f32::INFINITY);
        draw::prim(
            &mut ctx,
            &world,
            MODE_TRIANGLES,
            CULL_NONE,
            black_box(&TRIANGLE_POSITIONS),
            None,
            None,
            None,
            7,
            None,
            None,
            DrawState::unshaded(),
        )
        .unwrap();
        black_box(&ctx.depth);
    });
}

// A fully offscreen triangle measures projection and clipping rejection
// without mixing the result with pixel coverage.
fn bench_raster_flat_offscreen() {
    let mut ctx = make_draw_context(TARGET_SIZE);
    let world = *rc_ref!(&Mat4::from_translation(&Vec3 {
        x: 100.0,
        y: 0.0,
        z: 0.0,
    }));
    run_bench("raster_flat_offscreen", 100_000, |_| {
        draw::prim(
            &mut ctx,
            &world,
            MODE_TRIANGLES,
            CULL_NONE,
            black_box(&TRIANGLE_POSITIONS),
            None,
            None,
            None,
            7,
            None,
            None,
            DrawState::unshaded(),
        )
        .unwrap();
    });
}

// The same triangle through the textured + shaded sampler path inside
// draw::prim (make_shaded_sampler is private, so the public prim entry
// carries the texture + Shading into rasterize_textured_triangle).
fn bench_raster_textured_shaded_triangle() {
    bench_raster_textured_shaded(
        "raster_textured_shaded_triangle",
        3_000,
        TARGET_SIZE,
        f32::INFINITY,
    );
}

// Fully occluded geometry measures the early-depth path: projection and
// coverage still run, while hidden pixels must not sample or shade texels.
fn bench_raster_textured_shaded_occluded() {
    bench_raster_textured_shaded(
        "raster_textured_shaded_occluded",
        6_000,
        TARGET_SIZE,
        f32::NEG_INFINITY,
    );
}

// Doubling each viewport axis makes the same projected triangle cover
// roughly four times as many pixels, exposing per-pixel scaling.
fn bench_raster_textured_shaded_large() {
    bench_raster_textured_shaded(
        "raster_textured_shaded_160px",
        800,
        TARGET_SIZE * 2,
        f32::INFINITY,
    );
}

fn bench_raster_textured_shaded(
    name: &str,
    iters_per_sample: u32,
    target_size: u32,
    depth_clear: f32,
) {
    let mut ctx = make_draw_context(target_size);
    let world = Mat4::identity_value();
    // The face normal is tilted so the Lambert level lands mid-ramp
    // instead of the degenerate level 0.
    let normals = [0.0, 0.8, 0.6];
    let uvs = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
    let texture = make_texture();
    let shading = Shading::new(&PALETTE);
    let shading_ref = rc_ref!(&shading);
    let state = DrawState {
        shaded: true,
        dither_alpha: 1.0,
        depth_test: true,
        depth_write: true,
        billboard: BILLBOARD_OFF,
        shading: Some(&shading_ref),
    };
    run_bench(name, iters_per_sample, |_| {
        ctx.depth.fill(depth_clear);
        draw::prim(
            &mut ctx,
            &world,
            MODE_TRIANGLES,
            CULL_NONE,
            black_box(&TRIANGLE_POSITIONS),
            None,
            Some(&normals),
            Some(&uvs),
            0,
            Some(&texture),
            None,
            state,
        )
        .unwrap();
        black_box(&ctx.depth);
    });
}

// Ray queries against a 2048-triangle grid BVH, cycling through 16 fixed
// tilted downward rays scattered over the grid.
fn bench_bvh_query_ray() {
    let (positions, triangles) = make_grid_geometry();
    let bvh = Bvh::build(positions, triangles);
    let rays: Vec<(Vec3, Vec3)> = (0..16)
        .map(|i| {
            (
                grid_probe_point(i, 5.0),
                Vec3 {
                    x: 0.2,
                    y: -1.0,
                    z: 0.15,
                },
            )
        })
        .collect();
    run_bench("bvh_query_ray", 300_000, |i| {
        let (origin, direction) = rays[(i as usize) % rays.len()];
        let mut hits = 0_u32;
        bvh.query_ray(black_box(origin), black_box(direction), 20.0, |_| hits += 1);
        black_box(hits);
    });
}

// AABB queries against the same grid BVH, cycling through 16 fixed
// 1.2-unit query boxes scattered over the grid.
fn bench_bvh_query_aabb() {
    let (positions, triangles) = make_grid_geometry();
    let bvh = Bvh::build(positions, triangles);
    let queries: Vec<Aabb> = (0..16)
        .map(|i| {
            let center = grid_probe_point(i, 0.3);
            Aabb {
                min: Vec3 {
                    x: center.x - 0.6,
                    y: center.y - 0.6,
                    z: center.z - 0.6,
                },
                max: Vec3 {
                    x: center.x + 0.6,
                    y: center.y + 0.6,
                    z: center.z + 0.6,
                },
            }
        })
        .collect();
    run_bench("bvh_query_aabb", 200_000, |i| {
        let query = &queries[(i as usize) % queries.len()];
        let mut hits = 0_u32;
        bvh.query_aabb(black_box(query), |_| hits += 1);
        black_box(hits);
    });
}

// Aabb::from_mesh over the 2048-triangle grid mesh (1089 vertices) with a
// fixed rotated + translated world transform, the mesh-collider AABB
// refresh cost paid per frame per mesh collider.
fn bench_mesh_aabb_from_mesh() {
    let mesh = make_grid_mesh();
    let rotation = Mat4::from_axis_angle(
        &Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        30.0,
    );
    let translation = Mat4::from_translation(&Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    });
    let transform = rc_ref!(&translation).mul_mat_value(&rc_ref!(&rotation));
    run_bench("mesh_aabb_from_mesh", 20_000, |_| {
        let aabb = Aabb::from_mesh(&rc_ref!(&mesh), black_box(&transform));
        black_box(aabb.min.x + aabb.max.z);
    });
}

// Motion::sample on a 4-channel x 60-key clip, sweeping a fractional
// frame across the looping clip. 960 iterations cycle the frame sequence
// exactly, so every sample runs an identical mix of key spans.
fn bench_motion_sample() {
    let motion = make_motion();
    run_bench("motion_sample", 96_000, |i| {
        let frame = (i % 240) as f32 * 0.25;
        black_box(motion.sample(black_box(frame), true).len());
    });
}

// Tag lookup across the same 201-node tree, isolated from collision so
// traversal allocation and comparison costs remain visible.
fn bench_node_find_by_tags() {
    let root = make_scene_tree();
    let enemy_tags = vec![String::from("enemy")];
    run_bench("node_find_by_tags", 50_000, |_| {
        black_box(Node::find_by_tags(black_box(&root), black_box(&enemy_tags)).len());
    });
}

// Scene pipeline walk over a 201-node tree with 50 sphere colliders (40
// overlapping pairs): integrate_motion + detect_contacts, plus
// find_by_tags for the tags path, which those two stages do not read.
fn bench_scene_walk_contacts() {
    let root = make_scene_tree();
    let enemy_tags = vec![String::from("enemy")];
    run_bench("scene_walk_contacts", 1_500, |_| {
        Scene::integrate_motion(black_box(&root));
        let pairs = Scene::detect_contacts(black_box(&root));
        let tagged = Node::find_by_tags(&root, &enemy_tags);
        black_box(pairs.len() + tagged.len());
    });
}

// Bench harness

// Runs one warmup sample plus SAMPLE_COUNT timed samples of
// `iters_per_sample` iterations each, passing `f` the sample-local
// iteration index so input cycles repeat identically across samples.
fn run_bench(name: &str, iters_per_sample: u32, mut f: impl FnMut(u32)) {
    for i in 0..iters_per_sample {
        f(i);
    }
    let mut samples = [0.0_f64; SAMPLE_COUNT];
    for sample in &mut samples {
        let start = Instant::now();
        for i in 0..iters_per_sample {
            f(i);
        }
        *sample = start.elapsed().as_nanos() as f64 / f64::from(iters_per_sample);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = samples[SAMPLE_COUNT / 2];
    println!("{name}: {median:.1} ns/iter ({iters_per_sample} iters x {SAMPLE_COUNT} samples)");
}

// Fixtures

// Square render target with the default camera at the origin. Each
// raster iteration refills the depth buffer like a frame clear.
fn make_draw_context(target_size: u32) -> DrawContext {
    let camera = Camera::new();
    let size = target_size as f32;
    let camera_ref = rc_ref!(&camera);
    let view = view_matrix(&camera_ref);
    let projection = projection_matrix(&camera_ref, size, size);
    drop(camera_ref);
    DrawContext {
        target: Image::new(target_size, target_size),
        vp: matmul(&projection, &view),
        clip_row: camera_clip_row(&view),
        vp_x: 0.0,
        vp_y: 0.0,
        vp_w: size,
        vp_h: size,
        clip: compute_clip_rect(0.0, 0.0, size, size, target_size, target_size),
        camera,
        depth: vec![f32::INFINITY; (target_size * target_size) as usize],
        depth_w: target_size,
        depth_h: target_size,
        vertex_cache: Vec::new(),
        dither_alpha: 1.0,
        depth_test: true,
        depth_write: true,
        depth_offset: 0.0,
        shaded: true,
    }
}

// 16x16 texture with a deterministic per-pixel color pattern.
fn make_texture() -> RcImage {
    let image = Image::new(16, 16);
    let mut image_ref = rc_mut!(&image);
    for y in 0..16_u32 {
        for x in 0..16_u32 {
            image_ref.set_pixel(x as f32, y as f32, ((x + y * 3) % 16) as u8);
        }
    }
    drop(image_ref);
    image
}

// Deterministic bumpy grid: (GRID_DIVISIONS + 1)^2 vertices and
// GRID_DIVISIONS^2 x 2 triangles over [-GRID_EXTENT, GRID_EXTENT] on the
// XZ plane, with a small modular height pattern so BVH splits vary.
fn make_grid_geometry() -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let n = GRID_DIVISIONS;
    let step = GRID_EXTENT * 2.0 / n as f32;
    let mut positions = Vec::with_capacity(((n + 1) * (n + 1)) as usize);
    for iz in 0..=n {
        for ix in 0..=n {
            positions.push(Vec3 {
                x: -GRID_EXTENT + ix as f32 * step,
                y: ((ix * 3 + iz * 5) % 7) as f32 * 0.1,
                z: -GRID_EXTENT + iz as f32 * step,
            });
        }
    }
    let mut triangles = Vec::with_capacity((n * n * 2) as usize);
    for iz in 0..n {
        for ix in 0..n {
            let i0 = iz * (n + 1) + ix;
            let i1 = i0 + 1;
            let i2 = i0 + n + 1;
            let i3 = i2 + 1;
            triangles.push([i0, i2, i1]);
            triangles.push([i1, i2, i3]);
        }
    }
    (positions, triangles)
}

// One of 16 fixed probe points on a 4x4 lattice over the grid interior.
fn grid_probe_point(index: u32, y: f32) -> Vec3 {
    Vec3 {
        x: -6.0 + (index % 4) as f32 * 3.7,
        y,
        z: -6.0 + (index / 4) as f32 * 3.7,
    }
}

// The grid geometry wrapped in a single-part Mesh.
fn make_grid_mesh() -> RcMesh {
    let (positions, triangles) = make_grid_geometry();
    let primitive = Primitive::new();
    {
        let mut primitive_ref = rc_mut!(&primitive);
        primitive_ref.positions = positions.iter().flat_map(|v| [v.x, v.y, v.z]).collect();
        primitive_ref.indices = triangles
            .iter()
            .flat_map(|t| [t[0] as i32, t[1] as i32, t[2] as i32])
            .collect();
    }
    let mesh = Mesh::new();
    {
        let mut mesh_ref = rc_mut!(&mesh);
        mesh_ref.primitives = vec![Some(primitive)];
        mesh_ref.transforms = vec![Mat4::identity()];
        mesh_ref.parents = vec![-1];
    }
    mesh
}

// 60-key linear clip: translation / rotation / scale channels on part 0
// and a translation channel on part 1.
fn make_motion() -> Motion {
    const KEY_COUNT: usize = 60;
    let inputs: Vec<f32> = (0..KEY_COUNT).map(|k| k as f32).collect();
    let translations: Vec<Vec3> = (0..KEY_COUNT)
        .map(|k| Vec3 {
            x: k as f32 * 0.1,
            y: (k % 5) as f32 * 0.2,
            z: (k % 3) as f32 * 0.3,
        })
        .collect();
    let rotations: Vec<Quat> = (0..KEY_COUNT)
        .map(|k| {
            let half_angle = (k as f32 * 3.0).to_radians() * 0.5;
            Quat {
                x: 0.0,
                y: half_angle.sin(),
                z: 0.0,
                w: half_angle.cos(),
            }
        })
        .collect();
    let scales: Vec<Vec3> = (0..KEY_COUNT)
        .map(|k| Vec3 {
            x: 1.0 + k as f32 * 0.01,
            y: 1.0,
            z: 1.0,
        })
        .collect();
    let channel = |part_index, target, values| MotionChannel {
        part_index,
        target,
        inputs: inputs.clone(),
        values,
        interpolation: MotionInterpolation::Linear,
    };
    Motion {
        name: String::from("bench"),
        length: (KEY_COUNT - 1) as f32,
        base_transforms: vec![Mat4::identity_value(), Mat4::identity_value()],
        channels: vec![
            channel(
                0,
                MotionTarget::Translation,
                MotionValues::Translations(translations.clone()),
            ),
            channel(
                0,
                MotionTarget::Rotation,
                MotionValues::Rotations(rotations),
            ),
            channel(0, MotionTarget::Scale, MotionValues::Scales(scales)),
            channel(
                1,
                MotionTarget::Translation,
                MotionValues::Translations(translations),
            ),
        ],
    }
}

// 201-node tree: root + 10 groups x 19 leaves. The first 5 leaves per
// group carry sphere colliders spaced 0.9 apart (radius 0.5), so the 4
// adjacent pairs per group overlap and drive the narrow phase. Zero
// velocities keep the tree stationary across iterations.
fn make_scene_tree() -> RcNode {
    const GROUP_COUNT: usize = 10;
    const LEAVES_PER_GROUP: usize = 19;
    const COLLIDERS_PER_GROUP: usize = 5;
    let root = Node::new();
    // Build each group and its leaves
    for g in 0..GROUP_COUNT {
        let group = Node::new();
        {
            let mut group_ref = rc_mut!(&group);
            group_ref.transform =
                Mat4::from_translation(&rc_ref!(&Vec3::new(g as f32 * 4.0, 0.0, 0.0)));
            group_ref.tags = vec![String::from("group")];
        }
        Node::add_child(&root, &group);
        for l in 0..LEAVES_PER_GROUP {
            let leaf = Node::new();
            {
                let mut leaf_ref = rc_mut!(&leaf);
                leaf_ref.transform =
                    Mat4::from_translation(&rc_ref!(&Vec3::new(0.0, 0.0, l as f32 * 0.9)));
                if l < COLLIDERS_PER_GROUP {
                    leaf_ref.tags = vec![String::from("enemy")];
                    leaf_ref.collider = Some(Collider::new(
                        Vec3::zero(),
                        0.5,
                        None,
                        false,
                        false,
                        1.0,
                        0.3,
                        0.5,
                        Vec3::zero(),
                        Vec3::zero(),
                    ));
                } else {
                    leaf_ref.tags = vec![String::from("decor")];
                }
            }
            Node::add_child(&group, &leaf);
        }
    }
    root
}
