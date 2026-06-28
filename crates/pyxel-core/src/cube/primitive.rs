use std::sync::OnceLock;

use crate::cube::vec3::Vec3;

// Static vertex-data asset. Carries vertex attributes (positions /
// normals / uvs) and topology (indices, mode); mode and cull are
// indices-intrinsic. Shareable across many Node draw calls and Mesh
// parts. Empty normals / uvs / indices mean "absent".

pub const MODE_POINTS: i32 = 0;
pub const MODE_LINES: i32 = 1;
pub const MODE_TRIANGLES: i32 = 2;

pub const CULL_NONE: i32 = 0;
pub const CULL_BACK: i32 = 1;
pub const CULL_FRONT: i32 = 2;

pub struct Primitive {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub indices: Vec<i32>,
    pub mode: i32,
    pub cull: i32,
}

define_rc_type!(RcPrimitive, Primitive);

impl Primitive {
    pub fn new() -> RcPrimitive {
        new_rc_type!(Primitive {
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
            mode: MODE_TRIANGLES,
            cull: CULL_BACK,
        })
    }

    // Factories

    pub fn plane(width: f32, height: f32) -> RcPrimitive {
        clone_scaled(unit_plane(), width * 0.5, height * 0.5, 1.0)
    }

    pub fn r#box(size: &Vec3) -> RcPrimitive {
        clone_scaled(unit_box_textured(), size.x, size.y, size.z)
    }

    pub fn sphere(radius: f32) -> RcPrimitive {
        clone_scaled(unit_sphere_textured(), radius, radius, radius)
    }

    // Normal computation

    // Per-face flat normals: one (nx, ny, nz) per triangle, matching the
    // layout draw::prim consumes. Non-triangle topology and empty positions
    // yield empty output; an out-of-range index yields a zero entry. Empty
    // indices means sequential (vertices consumed 0, 1, 2, ...).
    pub fn compute_normals(&mut self) {
        let vertex_count = self.positions.len() / 3;
        if vertex_count == 0 || self.mode != MODE_TRIANGLES {
            self.normals = Vec::new();
            return;
        }
        let sequential = self.indices.is_empty();
        let face_count = if sequential {
            vertex_count / 3
        } else {
            self.indices.len() / 3
        };
        let mut out = vec![0.0_f32; face_count * 3];
        for f in 0..face_count {
            let (a, b, c) = if sequential {
                (f * 3, f * 3 + 1, f * 3 + 2)
            } else {
                let i0 = self.indices[f * 3] as usize;
                let i1 = self.indices[f * 3 + 1] as usize;
                let i2 = self.indices[f * 3 + 2] as usize;
                if i0 >= vertex_count || i1 >= vertex_count || i2 >= vertex_count {
                    continue;
                }
                (i0, i1, i2)
            };
            let pa = read_vec3(&self.positions, a);
            let pb = read_vec3(&self.positions, b);
            let pc = read_vec3(&self.positions, c);
            let face_normal = vec3_normalize(vec3_cross(vec3_sub(pb, pa), vec3_sub(pc, pa)));
            write_vec3(&mut out, f, face_normal);
        }
        self.normals = out;
    }
}

// Factory geometry helpers

pub(crate) fn unit_plane() -> &'static Primitive {
    static PRIMITIVE: OnceLock<Primitive> = OnceLock::new();
    PRIMITIVE.get_or_init(|| {
        make_primitive(
            MODE_TRIANGLES,
            CULL_NONE,
            UNIT_PLANE_POSITIONS.to_vec(),
            PLANE_TRI_INDICES.to_vec(),
            PLANE_UVS.to_vec(),
        )
    })
}

pub(crate) fn unit_box_solid() -> &'static Primitive {
    static PRIMITIVE: OnceLock<Primitive> = OnceLock::new();
    PRIMITIVE.get_or_init(|| {
        make_primitive(
            MODE_TRIANGLES,
            CULL_BACK,
            UNIT_BOX_POSITIONS.to_vec(),
            BOX_SOLID_TRI_INDICES.to_vec(),
            Vec::new(),
        )
    })
}

pub(crate) fn unit_box_textured() -> &'static Primitive {
    static PRIMITIVE: OnceLock<Primitive> = OnceLock::new();
    PRIMITIVE.get_or_init(|| {
        make_primitive(
            MODE_TRIANGLES,
            CULL_BACK,
            BOX_TEXTURED_POSITIONS.to_vec(),
            BOX_TEXTURED_TRI_INDICES.to_vec(),
            BOX_TEXTURED_UVS.to_vec(),
        )
    })
}

pub(crate) fn unit_sphere_solid() -> &'static Primitive {
    static PRIMITIVE: OnceLock<Primitive> = OnceLock::new();
    PRIMITIVE.get_or_init(|| {
        make_primitive(
            MODE_TRIANGLES,
            CULL_BACK,
            unit_icosa_lv1_positions().to_vec(),
            unit_icosa_lv1_tri_indices().to_vec(),
            Vec::new(),
        )
    })
}

pub(crate) fn unit_sphere_textured() -> &'static Primitive {
    static PRIMITIVE: OnceLock<Primitive> = OnceLock::new();
    PRIMITIVE.get_or_init(build_unit_sphere_textured)
}

pub(crate) fn unit_sphere_wire() -> &'static Primitive {
    static PRIMITIVE: OnceLock<Primitive> = OnceLock::new();
    PRIMITIVE.get_or_init(|| Primitive {
        positions: unit_icosa_lv1_positions().to_vec(),
        normals: Vec::new(),
        uvs: Vec::new(),
        indices: unit_icosa_lv1_edge_indices().to_vec(),
        mode: MODE_LINES,
        cull: CULL_NONE,
    })
}

fn clone_scaled(base: &Primitive, sx: f32, sy: f32, sz: f32) -> RcPrimitive {
    let p = Primitive::new();
    {
        let p = rc_mut!(&p);
        p.mode = base.mode;
        p.indices.clone_from(&base.indices);
        p.uvs.clone_from(&base.uvs);
        p.cull = base.cull;
        p.positions.reserve(base.positions.len());
        for chunk in base.positions.chunks(3) {
            p.positions.push(chunk[0] * sx);
            p.positions.push(chunk[1] * sy);
            p.positions.push(chunk[2] * sz);
        }
        p.compute_normals();
    }
    p
}

fn make_primitive(
    mode: i32,
    cull: i32,
    positions: Vec<f32>,
    indices: Vec<i32>,
    uvs: Vec<f32>,
) -> Primitive {
    let mut p = Primitive {
        positions,
        normals: Vec::new(),
        uvs,
        indices,
        mode,
        cull,
    };
    p.compute_normals();
    p
}

#[rustfmt::skip]
const UNIT_BOX_POSITIONS: [f32; 24] = [
    -0.5, -0.5, -0.5,   0.5, -0.5, -0.5,   0.5,  0.5, -0.5,  -0.5,  0.5, -0.5,
    -0.5, -0.5,  0.5,   0.5, -0.5,  0.5,   0.5,  0.5,  0.5,  -0.5,  0.5,  0.5,
];

#[rustfmt::skip]
const BOX_SOLID_TRI_INDICES: [i32; 36] = [
    0, 2, 1,  0, 3, 2,
    4, 5, 6,  4, 6, 7,
    0, 1, 5,  0, 5, 4,
    3, 6, 2,  3, 7, 6,
    0, 4, 7,  0, 7, 3,
    1, 2, 6,  1, 6, 5,
];

pub(crate) const BOX_EDGE_INDICES: [i32; 24] = [
    0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
];

#[rustfmt::skip]
const BOX_TEXTURED_POSITIONS: [f32; 72] = [
    -0.5, -0.5, -0.5,   0.5, -0.5, -0.5,   0.5,  0.5, -0.5,  -0.5,  0.5, -0.5,
    -0.5, -0.5,  0.5,   0.5, -0.5,  0.5,   0.5,  0.5,  0.5,  -0.5,  0.5,  0.5,
    -0.5, -0.5, -0.5,   0.5, -0.5, -0.5,  -0.5, -0.5,  0.5,   0.5, -0.5,  0.5,
    -0.5,  0.5, -0.5,   0.5,  0.5, -0.5,  -0.5,  0.5,  0.5,   0.5,  0.5,  0.5,
    -0.5, -0.5, -0.5,  -0.5,  0.5, -0.5,  -0.5, -0.5,  0.5,  -0.5,  0.5,  0.5,
     0.5, -0.5, -0.5,   0.5,  0.5, -0.5,   0.5, -0.5,  0.5,   0.5,  0.5,  0.5,
];

#[rustfmt::skip]
const BOX_TEXTURED_UVS: [f32; 48] = [
    1.0, 1.0,  0.0, 1.0,  0.0, 0.0,  1.0, 0.0,
    0.0, 1.0,  1.0, 1.0,  1.0, 0.0,  0.0, 0.0,
    0.0, 1.0,  1.0, 1.0,  0.0, 0.0,  1.0, 0.0,
    0.0, 0.0,  1.0, 0.0,  0.0, 1.0,  1.0, 1.0,
    0.0, 1.0,  0.0, 0.0,  1.0, 1.0,  1.0, 0.0,
    1.0, 1.0,  1.0, 0.0,  0.0, 1.0,  0.0, 0.0,
];

#[rustfmt::skip]
const BOX_TEXTURED_TRI_INDICES: [i32; 36] = [
     0,  2,  1,   0,  3,  2,
     4,  5,  6,   4,  6,  7,
     8,  9, 11,   8, 11, 10,
    12, 15, 13,  12, 14, 15,
    16, 18, 19,  16, 19, 17,
    20, 21, 23,  20, 23, 22,
];

#[rustfmt::skip]
const UNIT_PLANE_POSITIONS: [f32; 12] = [
    -1.0,  1.0, 0.0,
     1.0,  1.0, 0.0,
    -1.0, -1.0, 0.0,
     1.0, -1.0, 0.0,
];

const PLANE_TRI_INDICES: [i32; 6] = [0, 1, 2, 1, 3, 2];
const PLANE_UVS: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];

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
        for (edge_index, edge_pair) in ICOSA_BASE_EDGE_INDICES.chunks(2).enumerate() {
            let (a, b) = (edge_pair[0], edge_pair[1]);
            let m = 12 + edge_index as i32;
            out[cursor] = a;
            out[cursor + 1] = m;
            out[cursor + 2] = m;
            out[cursor + 3] = b;
            cursor += 4;
        }
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

fn build_unit_sphere_textured() -> Primitive {
    let base_positions = unit_icosa_lv1_positions();
    let base_indices = unit_icosa_lv1_tri_indices();
    let vertex_count = base_positions.len() / 3;
    let mut base_uvs = Vec::with_capacity(vertex_count * 2);
    for i in 0..vertex_count {
        let x = base_positions[i * 3];
        let y = base_positions[i * 3 + 1];
        let z = base_positions[i * 3 + 2];
        base_uvs.push(z.atan2(x) / (2.0 * std::f32::consts::PI) + 0.5);
        base_uvs.push(0.5 - y.asin() / std::f32::consts::PI);
    }

    let face_count = base_indices.len() / 3;
    let mut positions = Vec::with_capacity(base_positions.len());
    let mut uvs = Vec::with_capacity(base_uvs.len());
    let mut indices = Vec::with_capacity(base_indices.len());
    let mut vertex_map: Vec<[Option<i32>; 2]> = vec![[None; 2]; vertex_count];

    for f in 0..face_count {
        let i0 = base_indices[f * 3] as usize;
        let i1 = base_indices[f * 3 + 1] as usize;
        let i2 = base_indices[f * 3 + 2] as usize;
        let u0 = base_uvs[i0 * 2];
        let u1 = base_uvs[i1 * 2];
        let u2 = base_uvs[i2 * 2];
        let straddles_seam = (u0.max(u1).max(u2) - u0.min(u1).min(u2)) > 0.5;
        let side = |u: f32| usize::from(straddles_seam && u < 0.5);
        indices.push(add_sphere_uv_vertex(
            &mut positions,
            &mut uvs,
            &mut vertex_map,
            base_positions,
            &base_uvs,
            i0,
            side(u0),
        ));
        indices.push(add_sphere_uv_vertex(
            &mut positions,
            &mut uvs,
            &mut vertex_map,
            base_positions,
            &base_uvs,
            i1,
            side(u1),
        ));
        indices.push(add_sphere_uv_vertex(
            &mut positions,
            &mut uvs,
            &mut vertex_map,
            base_positions,
            &base_uvs,
            i2,
            side(u2),
        ));
    }
    make_primitive(MODE_TRIANGLES, CULL_BACK, positions, indices, uvs)
}

fn add_sphere_uv_vertex(
    positions: &mut Vec<f32>,
    uvs: &mut Vec<f32>,
    vertex_map: &mut [[Option<i32>; 2]],
    base_positions: &[f32],
    base_uvs: &[f32],
    base_idx: usize,
    seam_side: usize,
) -> i32 {
    if let Some(existing) = vertex_map[base_idx][seam_side] {
        return existing;
    }
    let new_idx = (positions.len() / 3) as i32;
    positions.extend_from_slice(&base_positions[base_idx * 3..base_idx * 3 + 3]);
    let mut u = base_uvs[base_idx * 2];
    if seam_side == 1 && u < 0.5 {
        u += 1.0;
    }
    uvs.extend_from_slice(&[u, base_uvs[base_idx * 2 + 1]]);
    vertex_map[base_idx][seam_side] = Some(new_idx);
    new_idx
}

// File-private raw Vec3 math. Avoids RcVec3 allocation in geometry
// helpers and on the hot path inside compute_normals.

fn read_vec3(buf: &[f32], i: usize) -> Vec3 {
    let base = i * 3;
    Vec3 {
        x: buf[base],
        y: buf[base + 1],
        z: buf[base + 2],
    }
}

fn write_vec3(buf: &mut [f32], i: usize, v: Vec3) {
    let base = i * 3;
    buf[base] = v.x;
    buf[base + 1] = v.y;
    buf[base + 2] = v.z;
}

fn vec3_sub(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

fn vec3_cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

fn vec3_normalize(v: Vec3) -> Vec3 {
    let len_sq = v.x * v.x + v.y * v.y + v.z * v.z;
    if len_sq < 1e-20 {
        return Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    let inv = 1.0 / len_sq.sqrt();
    Vec3 {
        x: v.x * inv,
        y: v.y * inv,
        z: v.z * inv,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_defaults() {
        let p = Primitive::new();
        let p = rc_ref!(&p);
        assert!(p.positions.is_empty());
        assert!(p.normals.is_empty());
        assert!(p.uvs.is_empty());
        assert!(p.indices.is_empty());
        assert_eq!(p.mode, MODE_TRIANGLES);
        assert_eq!(p.cull, CULL_BACK);
    }

    #[test]
    fn test_compute_normals_empty_indices_is_sequential() {
        // Empty indices => vertices consumed 0, 1, 2, ...; 1 triangle, +Z.
        let p = Primitive::new();
        {
            let p = rc_mut!(&p);
            p.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
            p.compute_normals();
        }
        let p = rc_ref!(&p);
        let n = &p.normals;
        // Per-face normals: 1 triangle -> 3 floats.
        assert_eq!(n.len(), 3);
        assert!(n[0].abs() < 1e-5);
        assert!(n[1].abs() < 1e-5);
        assert!((n[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_compute_normals_with_indices() {
        let p = Primitive::new();
        {
            let p = rc_mut!(&p);
            p.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
            p.indices = vec![0, 1, 2];
            p.compute_normals();
        }
        let p = rc_ref!(&p);
        let n = &p.normals;
        assert_eq!(n.len(), 3);
        assert!((n[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_compute_normals_two_triangles_have_distinct_face_normals() {
        // Triangle 0 in z=0 plane (normal +Z), triangle 1 in x=1 plane
        // (normal +X). Per-face layout means the two normals occupy
        // disjoint output slots.
        let p = Primitive::new();
        {
            let p = rc_mut!(&p);
            p.positions = vec![
                0.0, 0.0, 0.0, // 0
                1.0, 0.0, 0.0, // 1
                0.0, 1.0, 0.0, // 2
                1.0, 0.0, 1.0, // 3
                1.0, 1.0, 0.0, // 4
            ];
            p.indices = vec![0, 1, 2, 1, 4, 3];
            p.compute_normals();
        }
        let p = rc_ref!(&p);
        let n = &p.normals;
        // Face count = 2 -> 6 floats.
        assert_eq!(n.len(), 6);
        // Face 0 normal = +Z.
        assert!(n[0].abs() < 1e-5);
        assert!(n[1].abs() < 1e-5);
        assert!((n[2] - 1.0).abs() < 1e-5);
        // Face 1 normal = +X.
        assert!((n[3] - 1.0).abs() < 1e-5);
        assert!(n[4].abs() < 1e-5);
        assert!(n[5].abs() < 1e-5);
    }

    #[test]
    fn test_compute_normals_empty_positions() {
        let p = Primitive::new();
        {
            let p = rc_mut!(&p);
            p.compute_normals();
        }
        let p = rc_ref!(&p);
        assert_eq!(p.normals.len(), 0);
    }

    #[test]
    fn test_compute_normals_non_triangle_mode() {
        let p = Primitive::new();
        {
            let p = rc_mut!(&p);
            p.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
            p.mode = MODE_LINES;
            p.compute_normals();
        }
        let p = rc_ref!(&p);
        // Non-triangle mode has no face normal concept; output is empty.
        assert_eq!(p.normals.len(), 0);
    }

    #[test]
    fn test_indices_out_of_range_skipped() {
        let p = Primitive::new();
        {
            let p = rc_mut!(&p);
            p.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
            p.indices = vec![0, 1, 99]; // 99 is out of range
            p.compute_normals();
        }
        let p = rc_ref!(&p);
        let n = &p.normals;
        // 1 face slot allocated, but the out-of-range index leaves it zero.
        assert_eq!(n.len(), 3);
        for v in n {
            assert_eq!(*v, 0.0);
        }
    }
}
