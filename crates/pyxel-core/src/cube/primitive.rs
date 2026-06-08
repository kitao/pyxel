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

// File-private raw Vec3 math. Avoids RcVec3 allocation on the hot path
// inside compute_normals.

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
        for v in n.iter() {
            assert_eq!(*v, 0.0);
        }
    }
}
