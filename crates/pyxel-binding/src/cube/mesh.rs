use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyxel::cube::mesh::ColImage;

use super::mat4::Mat4;
use super::motion::Motion;
use super::primitive::Primitive;
use crate::image_wrapper::Image;

define_wrapper!(Mesh, pyxel::cube::Mesh, module = "pyxel.cube");

#[pymethods]
impl Mesh {
    #[new]
    #[pyo3(signature = (
        primitives=None,
        transforms=None,
        parents=None,
        names=None,
        col_img=None,
        colkey=None,
    ))]
    fn new(
        primitives: Option<Vec<Option<PyRef<'_, Primitive>>>>,
        transforms: Option<Vec<PyRef<'_, Mat4>>>,
        parents: Option<Vec<i32>>,
        names: Option<Vec<String>>,
        col_img: Option<Bound<'_, PyAny>>,
        colkey: Option<i32>,
    ) -> PyResult<Self> {
        let mesh = pyxel::cube::Mesh::new();
        {
            let mut m = rc_mut!(&mesh);
            if let Some(ps) = primitives {
                m.primitives = ps.into_iter().map(|p| p.map(|p| p.inner.clone())).collect();
            }
            if let Some(ts) = transforms {
                m.transforms = ts.iter().map(|t| t.inner.clone()).collect();
            }
            if let Some(ps) = parents {
                m.parents = ps;
            }
            if let Some(ns) = names {
                m.names = ns;
            } else if !m.primitives.is_empty() {
                m.names = vec![String::new(); m.primitives.len()];
            }
            if let Some(ci) = col_img {
                m.col_img = parse_col_img(&ci)?;
            }
            m.colkey = colkey;
        }
        rc_ref!(&mesh).validate().map_err(PyValueError::new_err)?;
        Ok(Self::wrap(mesh))
    }

    #[staticmethod]
    #[pyo3(signature = (filename, *, colkey=None, fps=30.0))]
    fn from_glb(filename: &str, colkey: Option<i32>, fps: f32) -> PyResult<Self> {
        pyxel::cube::Mesh::from_glb(filename, colkey, fps)
            .map(Self::wrap)
            .map_err(PyValueError::new_err)
    }

    // Parts (parallel arrays)

    #[getter]
    fn primitives(&self) -> Vec<Option<Primitive>> {
        self.inner_ref()
            .primitives
            .iter()
            .map(|p| p.as_ref().map(|p| Primitive::wrap(p.clone())))
            .collect()
    }

    #[setter]
    fn set_primitives(&self, v: Vec<Option<PyRef<'_, Primitive>>>) -> PyResult<()> {
        let value = v.into_iter().map(|p| p.map(|p| p.inner.clone())).collect();
        let mut mesh = self.inner_mut();
        let previous = std::mem::replace(&mut mesh.primitives, value);
        if let Err(error) = mesh.validate() {
            mesh.primitives = previous;
            return Err(PyValueError::new_err(error));
        }
        mesh.reset_collision_geometry_tracking();
        Ok(())
    }

    #[getter]
    fn transforms(&self) -> Vec<Mat4> {
        self.inner_ref()
            .transforms
            .iter()
            .map(|t| Mat4::wrap(t.clone()))
            .collect()
    }

    #[setter]
    fn set_transforms(&self, v: Vec<PyRef<'_, Mat4>>) -> PyResult<()> {
        let value = v.iter().map(|t| t.inner.clone()).collect();
        let mut mesh = self.inner_mut();
        let previous = std::mem::replace(&mut mesh.transforms, value);
        if let Err(error) = mesh.validate() {
            mesh.transforms = previous;
            return Err(PyValueError::new_err(error));
        }
        *mesh.bvh.borrow_mut() = None;
        *mesh.local_aabb.borrow_mut() = None;
        Ok(())
    }

    #[getter]
    fn parents(&self) -> Vec<i32> {
        self.inner_ref().parents.clone()
    }

    #[setter]
    fn set_parents(&self, v: Vec<i32>) -> PyResult<()> {
        let mut mesh = self.inner_mut();
        let previous = std::mem::replace(&mut mesh.parents, v);
        if let Err(error) = mesh.validate() {
            mesh.parents = previous;
            return Err(PyValueError::new_err(error));
        }
        *mesh.bvh.borrow_mut() = None;
        *mesh.local_aabb.borrow_mut() = None;
        Ok(())
    }

    #[getter]
    fn names(&self) -> Vec<String> {
        let inner = self.inner_ref();
        if inner.names.is_empty() && !inner.primitives.is_empty() {
            vec![String::new(); inner.primitives.len()]
        } else {
            inner.names.clone()
        }
    }

    #[setter]
    fn set_names(&self, v: Vec<String>) -> PyResult<()> {
        let mut mesh = self.inner_mut();
        let previous = std::mem::replace(&mut mesh.names, v);
        if let Err(error) = mesh.validate() {
            mesh.names = previous;
            return Err(PyValueError::new_err(error));
        }
        Ok(())
    }

    #[getter]
    fn motions(&self) -> Vec<Motion> {
        self.inner_ref()
            .motions
            .iter()
            .map(|m| Motion::wrap_with_source(m.clone(), self.inner.clone()))
            .collect()
    }

    // Shared material

    #[getter]
    fn col_img(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match &self.inner_ref().col_img {
            ColImage::Color(c) => Ok(c.into_pyobject(py)?.into_any().unbind()),
            ColImage::Image(img) => Ok(Image::wrap(img.clone())
                .into_pyobject(py)?
                .into_any()
                .unbind()),
        }
    }

    #[setter]
    fn set_col_img(&self, v: Bound<'_, PyAny>) -> PyResult<()> {
        self.inner_mut().col_img = parse_col_img(&v)?;
        Ok(())
    }

    #[getter]
    fn colkey(&self) -> Option<i32> {
        self.inner_ref().colkey
    }

    #[setter]
    fn set_colkey(&self, v: Option<i32>) {
        self.inner_mut().colkey = v;
    }

    fn __repr__(&self) -> String {
        let m = self.inner_ref();
        format!("Mesh(parts={})", m.primitives.len())
    }

    fn descendants(&self, i: i32) -> Vec<i32> {
        self.inner_ref().descendants(i)
    }
}

// Internal helpers

pub(crate) fn parse_col_img(v: &Bound<'_, PyAny>) -> PyResult<ColImage> {
    if let Ok(c) = v.extract::<i32>() {
        return Ok(ColImage::Color(c));
    }
    if let Ok(img_ref) = v.cast::<Image>() {
        return Ok(ColImage::Image(img_ref.borrow().inner.clone()));
    }
    Err(PyTypeError::new_err("col_img must be int or Image"))
}

pub(crate) fn resolve_col_img(col_img: Option<&Bound<'_, PyAny>>) -> PyResult<ColImage> {
    col_img.map_or(Ok(ColImage::Color(7)), parse_col_img)
}

pub fn add_mesh_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    Ok(())
}
