use crate::cube::vec3::Vec3;

// Static vertex-data asset. Carries vertex attributes (positions /
// normals / uvs), topology (indices, prim mode), and back-face cull
// mode. Shareable across many Node draw calls and Mesh parts.

pub const PRIM_POINTS: i32 = 0;
pub const PRIM_LINES: i32 = 1;
pub const PRIM_TRIANGLES: i32 = 2;

pub const CULL_NONE: i32 = 0;
pub const CULL_BACK: i32 = 1;
pub const CULL_FRONT: i32 = 2;

pub struct Geometry {
    pub positions: Vec<f32>,
    pub normals: Option<Vec<f32>>,
    pub uvs: Option<Vec<f32>>,
    pub indices: Option<Vec<i32>>,
    pub prim: i32,
    pub cull: i32,
}

define_rc_type!(RcGeometry, Geometry);

impl Geometry {
    pub fn new() -> RcGeometry {
        new_rc_type!(Geometry {
            positions: Vec::new(),
            normals: None,
            uvs: None,
            indices: None,
            prim: PRIM_TRIANGLES,
            cull: CULL_BACK,
        })
    }

    // Per-face flat normals: one (nx, ny, nz) per triangle, matching the
    // layout draw::prim consumes. Non-triangle topology, empty positions,
    // and out-of-range indices each yield a zero entry instead of a hit.
    pub fn compute_normals(&mut self) {
        let vertex_count = self.positions.len() / 3;
        if vertex_count == 0 || self.prim != PRIM_TRIANGLES {
            self.normals = Some(Vec::new());
            return;
        }
        let face_count = match &self.indices {
            Some(idx) => idx.len() / 3,
            None => vertex_count / 3,
        };
        let mut out = vec![0.0_f32; face_count * 3];
        for f in 0..face_count {
            let (a, b, c) = match &self.indices {
                Some(idx) => {
                    let i0 = idx[f * 3] as usize;
                    let i1 = idx[f * 3 + 1] as usize;
                    let i2 = idx[f * 3 + 2] as usize;
                    if i0 >= vertex_count || i1 >= vertex_count || i2 >= vertex_count {
                        continue;
                    }
                    (i0, i1, i2)
                }
                None => (f * 3, f * 3 + 1, f * 3 + 2),
            };
            let pa = read_vec3(&self.positions, a);
            let pb = read_vec3(&self.positions, b);
            let pc = read_vec3(&self.positions, c);
            let face_normal = vec3_normalize(vec3_cross(vec3_sub(pb, pa), vec3_sub(pc, pa)));
            write_vec3(&mut out, f, face_normal);
        }
        self.normals = Some(out);
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
        let g = Geometry::new();
        let g = rc_ref!(&g);
        assert!(g.positions.is_empty());
        assert!(g.normals.is_none());
        assert!(g.uvs.is_none());
        assert!(g.indices.is_none());
        assert_eq!(g.prim, PRIM_TRIANGLES);
        assert_eq!(g.cull, CULL_BACK);
    }

    #[test]
    fn test_compute_normals_flat_triangle_no_indices() {
        let g = Geometry::new();
        {
            let g = rc_mut!(&g);
            g.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
            g.compute_normals();
        }
        let g = rc_ref!(&g);
        let n = g.normals.as_ref().unwrap();
        // Per-face normals: 1 triangle → 3 floats.
        assert_eq!(n.len(), 3);
        assert!(n[0].abs() < 1e-5);
        assert!(n[1].abs() < 1e-5);
        assert!((n[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_compute_normals_with_indices() {
        let g = Geometry::new();
        {
            let g = rc_mut!(&g);
            g.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
            g.indices = Some(vec![0, 1, 2]);
            g.compute_normals();
        }
        let g = rc_ref!(&g);
        let n = g.normals.as_ref().unwrap();
        assert_eq!(n.len(), 3);
        assert!((n[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_compute_normals_two_triangles_have_distinct_face_normals() {
        // Triangle 0 in z=0 plane (normal +Z), triangle 1 in x=1 plane
        // (normal +X). Per-face layout means the two normals occupy
        // disjoint output slots.
        let g = Geometry::new();
        {
            let g = rc_mut!(&g);
            g.positions = vec![
                0.0, 0.0, 0.0, // 0
                1.0, 0.0, 0.0, // 1
                0.0, 1.0, 0.0, // 2
                1.0, 0.0, 1.0, // 3
                1.0, 1.0, 0.0, // 4
            ];
            g.indices = Some(vec![0, 1, 2, 1, 4, 3]);
            g.compute_normals();
        }
        let g = rc_ref!(&g);
        let n = g.normals.as_ref().unwrap();
        // Face count = 2 → 6 floats.
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
        let g = Geometry::new();
        {
            let g = rc_mut!(&g);
            g.compute_normals();
        }
        let g = rc_ref!(&g);
        assert_eq!(g.normals.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn test_compute_normals_non_triangle_prim() {
        let g = Geometry::new();
        {
            let g = rc_mut!(&g);
            g.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
            g.prim = PRIM_LINES;
            g.compute_normals();
        }
        let g = rc_ref!(&g);
        // Non-triangle prim has no face normal concept; output is empty.
        assert_eq!(g.normals.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn test_indices_out_of_range_skipped() {
        let g = Geometry::new();
        {
            let g = rc_mut!(&g);
            g.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
            g.indices = Some(vec![0, 1, 99]); // 99 is out of range
            g.compute_normals();
        }
        let g = rc_ref!(&g);
        let n = g.normals.as_ref().unwrap();
        // 1 face slot allocated, but the out-of-range index leaves it zero.
        assert_eq!(n.len(), 3);
        for v in n.iter() {
            assert_eq!(*v, 0.0);
        }
    }
}
