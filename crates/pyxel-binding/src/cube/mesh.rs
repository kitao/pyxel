use pyo3::prelude::*;

use super::float_buffer::FloatBuffer;
use super::int_buffer::IntBuffer;
use crate::image_wrapper::Image;

define_wrapper!(Mesh, pyxel::cube::Mesh);

#[pymethods]
impl Mesh {
    // Constructor

    #[new]
    #[pyo3(signature = (positions=None, indices=None, normals=None, uvs=None, image=None, colkey=None))]
    fn new(
        positions: Option<PyRef<'_, FloatBuffer>>,
        indices: Option<PyRef<'_, IntBuffer>>,
        normals: Option<PyRef<'_, FloatBuffer>>,
        uvs: Option<PyRef<'_, FloatBuffer>>,
        image: Option<PyRef<'_, Image>>,
        colkey: Option<i32>,
    ) -> Self {
        let mesh = pyxel::cube::Mesh::new();
        {
            let m = rc_mut!(&mesh);
            if let Some(p) = positions {
                m.positions = p.inner.clone();
            }
            if let Some(i) = indices {
                m.indices = Some(i.inner.clone());
            }
            if let Some(n) = normals {
                m.normals = Some(n.inner.clone());
            }
            if let Some(u) = uvs {
                m.uvs = Some(u.inner.clone());
            }
            if let Some(img) = image {
                m.image = Some(img.inner.clone());
            }
            m.colkey = colkey;
        }
        Self::wrap(mesh)
    }

    // Members

    #[getter]
    fn positions(&self) -> FloatBuffer {
        FloatBuffer::wrap(self.inner_ref().positions.clone())
    }

    #[setter]
    fn set_positions(&self, v: PyRef<'_, FloatBuffer>) {
        self.inner_mut().positions = v.inner.clone();
    }

    #[getter]
    fn indices(&self) -> Option<IntBuffer> {
        self.inner_ref()
            .indices
            .as_ref()
            .map(|i| IntBuffer::wrap(i.clone()))
    }

    #[setter]
    fn set_indices(&self, v: Option<PyRef<'_, IntBuffer>>) {
        self.inner_mut().indices = v.as_ref().map(|i| i.inner.clone());
    }

    #[getter]
    fn normals(&self) -> Option<FloatBuffer> {
        self.inner_ref()
            .normals
            .as_ref()
            .map(|n| FloatBuffer::wrap(n.clone()))
    }

    #[setter]
    fn set_normals(&self, v: Option<PyRef<'_, FloatBuffer>>) {
        self.inner_mut().normals = v.as_ref().map(|n| n.inner.clone());
    }

    #[getter]
    fn uvs(&self) -> Option<FloatBuffer> {
        self.inner_ref()
            .uvs
            .as_ref()
            .map(|u| FloatBuffer::wrap(u.clone()))
    }

    #[setter]
    fn set_uvs(&self, v: Option<PyRef<'_, FloatBuffer>>) {
        self.inner_mut().uvs = v.as_ref().map(|u| u.inner.clone());
    }

    #[getter]
    fn image(&self) -> Option<Image> {
        self.inner_ref()
            .image
            .as_ref()
            .map(|i| Image::wrap(i.clone()))
    }

    #[setter]
    fn set_image(&self, v: Option<PyRef<'_, Image>>) {
        self.inner_mut().image = v.as_ref().map(|i| i.inner.clone());
    }

    #[getter]
    fn colkey(&self) -> Option<i32> {
        self.inner_ref().colkey
    }

    #[setter]
    fn set_colkey(&self, v: Option<i32>) {
        self.inner_mut().colkey = v;
    }

    // Dunder

    fn __repr__(&self) -> String {
        let m = self.inner_ref();
        let pos_size = rc_ref!(&m.positions).size();
        let idx_size = m.indices.as_ref().map_or(0, |i| rc_ref!(i).size());
        format!("Mesh(positions={pos_size}, indices={idx_size})")
    }
}

pub fn add_mesh_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    Ok(())
}
