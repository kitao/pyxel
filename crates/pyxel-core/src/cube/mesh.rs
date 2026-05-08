use crate::cube::float_buffer::{FloatBuffer, RcFloatBuffer};
use crate::cube::int_buffer::RcIntBuffer;
use crate::image::RcImage;

// Geometry asset for `Node.mesh(mat, mesh_asset, ...)`. positions /
// normals / uvs are flat f32 buffers; indices is a flat i32 buffer of
// triangle vertex indices (PRIM_TRIANGLES winding). Every component
// except positions is optional; positions defaults to an empty
// FloatBuffer so `Mesh::new()` succeeds without arguments and members
// can be assigned afterwards.

pub struct Mesh {
    pub positions: RcFloatBuffer,
    pub indices: Option<RcIntBuffer>,
    pub normals: Option<RcFloatBuffer>,
    pub uvs: Option<RcFloatBuffer>,
    pub image: Option<RcImage>,
    pub colkey: Option<i32>,
}

define_rc_type!(RcMesh, Mesh);

impl Mesh {
    pub fn new() -> RcMesh {
        new_rc_type!(Mesh {
            positions: FloatBuffer::with_size(0),
            indices: None,
            normals: None,
            uvs: None,
            image: None,
            colkey: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::int_buffer::IntBuffer;

    #[test]
    fn test_new_empty() {
        let m = Mesh::new();
        let m = rc_ref!(&m);
        assert_eq!(rc_ref!(&m.positions).size(), 0);
        assert!(m.indices.is_none());
        assert!(m.normals.is_none());
        assert!(m.uvs.is_none());
        assert!(m.image.is_none());
        assert!(m.colkey.is_none());
    }

    #[test]
    fn test_assign_members() {
        let mesh = Mesh::new();
        let positions = FloatBuffer::from_values(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        let indices = IntBuffer::from_values(vec![0, 1, 2]);
        let normals = FloatBuffer::from_values(vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0]);
        let uvs = FloatBuffer::from_values(vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0]);
        {
            let m = rc_mut!(&mesh);
            m.positions = positions;
            m.indices = Some(indices);
            m.normals = Some(normals);
            m.uvs = Some(uvs);
            m.colkey = Some(3);
        }
        let m = rc_ref!(&mesh);
        assert_eq!(rc_ref!(&m.positions).size(), 9);
        assert_eq!(rc_ref!(m.indices.as_ref().unwrap()).size(), 3);
        assert_eq!(rc_ref!(m.normals.as_ref().unwrap()).size(), 9);
        assert_eq!(rc_ref!(m.uvs.as_ref().unwrap()).size(), 6);
        assert_eq!(m.colkey, Some(3));
    }
}
