use pyo3::prelude::*;
use pyxel::cube::mesh::{Face, Uv};

use super::vec3::Vec3;
use crate::image_wrapper::Image;

define_wrapper!(Mesh, pyxel::cube::Mesh);

#[pymethods]
impl Mesh {
    // Constructor

    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::cube::Mesh::new())
    }

    // Factories

    #[staticmethod]
    #[pyo3(signature = (vertices, faces, uvs=None, col=7, image=None))]
    fn from_vertices(
        vertices: Vec<PyRef<'_, Vec3>>,
        faces: Vec<(u32, u32, u32)>,
        uvs: Option<Vec<(f32, f32)>>,
        col: i32,
        image: Option<PyRef<'_, Image>>,
    ) -> Self {
        let inner_vertices: Vec<pyxel::cube::Vec3> =
            vertices.iter().map(|v| *v.inner_ref()).collect();
        let inner_faces: Vec<Face> = faces.into_iter().map(|(a, b, c)| [a, b, c]).collect();
        let inner_image = image.as_ref().map(|i| i.inner.clone());
        Self::wrap(pyxel::cube::Mesh::from_vertices(
            inner_vertices,
            inner_faces,
            uvs,
            col,
            inner_image,
        ))
    }

    #[staticmethod]
    #[pyo3(name = "box", signature = (size, col=7))]
    fn box_(size: PyRef<'_, Vec3>, col: i32) -> Self {
        Self::wrap(pyxel::cube::Mesh::box_(*size.inner_ref(), col))
    }

    #[staticmethod]
    #[pyo3(signature = (radius, segments=16, col=7))]
    fn sphere(radius: f32, segments: usize, col: i32) -> Self {
        Self::wrap(pyxel::cube::Mesh::sphere(radius, segments, col))
    }

    #[staticmethod]
    #[pyo3(signature = (radius, height, segments=16, col=7))]
    fn cylinder(radius: f32, height: f32, segments: usize, col: i32) -> Self {
        Self::wrap(pyxel::cube::Mesh::cylinder(radius, height, segments, col))
    }

    #[staticmethod]
    #[pyo3(signature = (w, h, col=7))]
    fn plane(w: f32, h: f32, col: i32) -> Self {
        Self::wrap(pyxel::cube::Mesh::plane(w, h, col))
    }

    // Sizes

    #[getter]
    fn vertex_count(&self) -> usize {
        self.inner_ref().vertex_count()
    }

    #[getter]
    fn face_count(&self) -> usize {
        self.inner_ref().face_count()
    }

    // Per-element access

    fn get_vertex(&self, i: usize) -> PyResult<Vec3> {
        let m = self.inner_ref();
        if i >= m.vertex_count() {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Mesh vertex index out of range",
            ));
        }
        let v = m.get_vertex(i);
        Ok(Vec3::wrap(pyxel::cube::Vec3::new(v.x, v.y, v.z)))
    }

    fn set_vertex(&self, i: usize, v: PyRef<'_, Vec3>) -> PyResult<()> {
        if i >= self.inner_ref().vertex_count() {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Mesh vertex index out of range",
            ));
        }
        self.inner_mut().set_vertex(i, *v.inner_ref());
        Ok(())
    }

    fn get_uv(&self, i: usize) -> PyResult<Uv> {
        let m = self.inner_ref();
        if i >= m.vertex_count() {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Mesh uv index out of range",
            ));
        }
        Ok(m.get_uv(i))
    }

    fn set_uv(&self, i: usize, uv: Uv) -> PyResult<()> {
        if i >= self.inner_ref().vertex_count() {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Mesh uv index out of range",
            ));
        }
        self.inner_mut().set_uv(i, uv);
        Ok(())
    }

    fn get_face(&self, i: usize) -> PyResult<(u32, u32, u32)> {
        let m = self.inner_ref();
        if i >= m.face_count() {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Mesh face index out of range",
            ));
        }
        let f = m.get_face(i);
        Ok((f[0], f[1], f[2]))
    }

    fn set_face(&self, i: usize, f: (u32, u32, u32)) -> PyResult<()> {
        if i >= self.inner_ref().face_count() {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Mesh face index out of range",
            ));
        }
        self.inner_mut().set_face(i, [f.0, f.1, f.2]);
        Ok(())
    }

    // Resize

    fn resize(&self, vertex_count: usize, face_count: usize) {
        self.inner_mut().resize(vertex_count, face_count);
    }

    // Color and texture

    #[getter]
    fn col(&self) -> i32 {
        self.inner_ref().col
    }

    #[setter]
    fn set_col(&self, v: i32) {
        self.inner_mut().col = v;
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

    // Dunder

    fn __repr__(&self) -> String {
        let m = self.inner_ref();
        format!(
            "Mesh(vertices={}, faces={}, col={})",
            m.vertex_count(),
            m.face_count(),
            m.col
        )
    }
}

pub fn add_mesh_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    Ok(())
}
