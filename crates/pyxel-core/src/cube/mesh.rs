#![allow(clippy::many_single_char_names)]

use crate::cube::vec3::Vec3;
use crate::image::RcImage;

// Single geometry asset. Vertices, faces, and UVs live in flat contiguous
// buffers; faces are triangle indices into the vertex array. Mutable, so
// callers can deform the mesh per frame for dynamic effects.

pub type Face = [u32; 3];
pub type Uv = (f32, f32);

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub uvs: Option<Vec<Uv>>,
    pub col: i32,
    pub image: Option<RcImage>,
}

define_rc_type!(RcMesh, Mesh);

impl Mesh {
    // Constructor

    pub fn new() -> RcMesh {
        new_rc_type!(Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            uvs: None,
            col: 7,
            image: None,
        })
    }

    pub fn from_vertices(
        vertices: Vec<Vec3>,
        faces: Vec<Face>,
        uvs: Option<Vec<Uv>>,
        col: i32,
        image: Option<RcImage>,
    ) -> RcMesh {
        new_rc_type!(Mesh {
            vertices,
            faces,
            uvs,
            col,
            image,
        })
    }

    // Primitive factories. Faces wind counter-clockwise as seen from outside.

    pub fn box_(size: Vec3, col: i32) -> RcMesh {
        let hx = size.x * 0.5;
        let hy = size.y * 0.5;
        let hz = size.z * 0.5;
        let vertices = vec![
            Vec3 {
                x: -hx,
                y: -hy,
                z: -hz,
            },
            Vec3 {
                x: hx,
                y: -hy,
                z: -hz,
            },
            Vec3 {
                x: hx,
                y: hy,
                z: -hz,
            },
            Vec3 {
                x: -hx,
                y: hy,
                z: -hz,
            },
            Vec3 {
                x: -hx,
                y: -hy,
                z: hz,
            },
            Vec3 {
                x: hx,
                y: -hy,
                z: hz,
            },
            Vec3 {
                x: hx,
                y: hy,
                z: hz,
            },
            Vec3 {
                x: -hx,
                y: hy,
                z: hz,
            },
        ];
        let faces = vec![
            [0, 2, 1],
            [0, 3, 2],
            [4, 5, 6],
            [4, 6, 7],
            [0, 4, 7],
            [0, 7, 3],
            [1, 2, 6],
            [1, 6, 5],
            [3, 7, 6],
            [3, 6, 2],
            [0, 1, 5],
            [0, 5, 4],
        ];
        new_rc_type!(Mesh {
            vertices,
            faces,
            uvs: None,
            col,
            image: None,
        })
    }

    pub fn sphere(radius: f32, segments: usize, col: i32) -> RcMesh {
        let lat_steps = (segments / 2).max(2);
        let lng_steps = segments.max(3);
        let stride = lng_steps + 1;
        let mut vertices = Vec::with_capacity((lat_steps + 1) * stride);
        for j in 0..=lat_steps {
            let theta = std::f32::consts::PI * j as f32 / lat_steps as f32;
            let y = theta.cos() * radius;
            let r = theta.sin() * radius;
            for i in 0..=lng_steps {
                let phi = 2.0 * std::f32::consts::PI * i as f32 / lng_steps as f32;
                let x = r * phi.cos();
                let z = r * phi.sin();
                vertices.push(Vec3 { x, y, z });
            }
        }
        let mut faces = Vec::with_capacity(lat_steps * lng_steps * 2);
        for j in 0..lat_steps {
            for i in 0..lng_steps {
                let a = (j * stride + i) as u32;
                let b = a + 1;
                let c = (j * stride + stride + i) as u32;
                let d = c + 1;
                faces.push([a, b, d]);
                faces.push([a, d, c]);
            }
        }
        new_rc_type!(Mesh {
            vertices,
            faces,
            uvs: None,
            col,
            image: None,
        })
    }

    pub fn cylinder(radius: f32, height: f32, segments: usize, col: i32) -> RcMesh {
        let n = segments.max(3);
        let hh = height * 0.5;
        let mut vertices = Vec::with_capacity(2 * (n + 1));
        for i in 0..=n {
            let phi = 2.0 * std::f32::consts::PI * i as f32 / n as f32;
            let x = radius * phi.cos();
            let z = radius * phi.sin();
            vertices.push(Vec3 { x, y: -hh, z });
            vertices.push(Vec3 { x, y: hh, z });
        }
        let mut faces = Vec::with_capacity(n * 2);
        for i in 0..n {
            let a = (2 * i) as u32;
            let b = a + 1;
            let c = a + 2;
            let d = a + 3;
            faces.push([a, c, d]);
            faces.push([a, d, b]);
        }
        // Caps are deferred; phase 2 will close the top and bottom faces.
        new_rc_type!(Mesh {
            vertices,
            faces,
            uvs: None,
            col,
            image: None,
        })
    }

    pub fn plane(w: f32, h: f32, col: i32) -> RcMesh {
        let hw = w * 0.5;
        let hh = h * 0.5;
        // Lies in the local XZ plane (Y up convention).
        let vertices = vec![
            Vec3 {
                x: -hw,
                y: 0.0,
                z: -hh,
            },
            Vec3 {
                x: hw,
                y: 0.0,
                z: -hh,
            },
            Vec3 {
                x: hw,
                y: 0.0,
                z: hh,
            },
            Vec3 {
                x: -hw,
                y: 0.0,
                z: hh,
            },
        ];
        let faces = vec![[0, 1, 2], [0, 2, 3]];
        new_rc_type!(Mesh {
            vertices,
            faces,
            uvs: None,
            col,
            image: None,
        })
    }

    // Sizes

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    // Per-element access

    pub fn get_vertex(&self, i: usize) -> Vec3 {
        self.vertices[i]
    }

    pub fn set_vertex(&mut self, i: usize, v: Vec3) {
        self.vertices[i] = v;
    }

    pub fn get_uv(&self, i: usize) -> Uv {
        self.uvs.as_ref().map_or((0.0, 0.0), |u| u[i])
    }

    pub fn set_uv(&mut self, i: usize, uv: Uv) {
        if self.uvs.is_none() {
            self.uvs = Some(vec![(0.0, 0.0); self.vertices.len()]);
        }
        if let Some(uvs) = self.uvs.as_mut() {
            uvs[i] = uv;
        }
    }

    pub fn get_face(&self, i: usize) -> Face {
        self.faces[i]
    }

    pub fn set_face(&mut self, i: usize, f: Face) {
        self.faces[i] = f;
    }

    // Resize

    pub fn resize(&mut self, vertex_count: usize, face_count: usize) {
        self.vertices.resize(
            vertex_count,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
        self.faces.resize(face_count, [0, 0, 0]);
        if let Some(uvs) = self.uvs.as_mut() {
            uvs.resize(vertex_count, (0.0, 0.0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_mesh() {
        let m = Mesh::new();
        let m = rc_ref!(&m);
        assert_eq!(m.vertex_count(), 0);
        assert_eq!(m.face_count(), 0);
        assert_eq!(m.col, 7);
        assert!(m.uvs.is_none());
        assert!(m.image.is_none());
    }

    #[test]
    fn test_box_factory() {
        let m = Mesh::box_(
            Vec3 {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
            4,
        );
        let m = rc_ref!(&m);
        assert_eq!(m.vertex_count(), 8);
        assert_eq!(m.face_count(), 12);
        assert_eq!(m.col, 4);
    }

    #[test]
    fn test_sphere_factory() {
        let m = Mesh::sphere(1.0, 8, 12);
        let m = rc_ref!(&m);
        // 8 segments -> lat 4 steps, lng 8 steps
        // vertex count = (4+1) * (8+1) = 45
        assert_eq!(m.vertex_count(), 45);
        // face count = 4 * 8 * 2 = 64
        assert_eq!(m.face_count(), 64);
    }

    #[test]
    fn test_plane_factory() {
        let m = Mesh::plane(2.0, 1.0, 11);
        let m = rc_ref!(&m);
        assert_eq!(m.vertex_count(), 4);
        assert_eq!(m.face_count(), 2);
    }

    #[test]
    fn test_resize() {
        let m = Mesh::new();
        let m_mut = rc_mut!(&m);
        m_mut.resize(4, 2);
        assert_eq!(m_mut.vertex_count(), 4);
        assert_eq!(m_mut.face_count(), 2);
    }

    #[test]
    fn test_set_vertex() {
        let m = Mesh::new();
        let m_mut = rc_mut!(&m);
        m_mut.resize(3, 1);
        m_mut.set_vertex(
            0,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        );
        let v = m_mut.get_vertex(0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_set_uv() {
        let m = Mesh::new();
        let m_mut = rc_mut!(&m);
        m_mut.resize(2, 0);
        m_mut.set_uv(1, (0.5, 0.75));
        assert_eq!(m_mut.get_uv(0), (0.0, 0.0));
        assert_eq!(m_mut.get_uv(1), (0.5, 0.75));
    }

    #[test]
    fn test_from_vertices() {
        let vertices = vec![
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ];
        let faces = vec![[0, 1, 2]];
        let m = Mesh::from_vertices(vertices, faces, None, 8, None);
        let m = rc_ref!(&m);
        assert_eq!(m.vertex_count(), 3);
        assert_eq!(m.face_count(), 1);
        assert_eq!(m.col, 8);
    }
}
