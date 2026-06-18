use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyxel::cube::mesh::ColImage;

use super::mat4::Mat4;
use super::primitive::Primitive;
use crate::image_wrapper::Image;

define_wrapper!(Mesh, pyxel::cube::Mesh);

#[pymethods]
impl Mesh {
    // Constructor

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
            let m = rc_mut!(&mesh);
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

    // Parts (parallel arrays)

    #[getter]
    fn primitives(&self, py: Python<'_>) -> PyResult<Py<PyList>> {
        let inner = self.inner_ref();
        let items: Vec<Py<PyAny>> = inner
            .primitives
            .iter()
            .map(|p| match p {
                Some(p) => match Primitive::wrap(p.clone()).into_pyobject(py) {
                    Ok(b) => b.into_any().unbind(),
                    Err(_) => py.None(),
                },
                None => py.None(),
            })
            .collect();
        Ok(PyList::new(py, items)?.unbind())
    }

    #[setter]
    fn set_primitives(&self, v: Vec<Option<PyRef<'_, Primitive>>>) -> PyResult<()> {
        self.inner_mut().primitives = v.into_iter().map(|p| p.map(|p| p.inner.clone())).collect();
        self.inner_ref().validate().map_err(PyValueError::new_err)
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
        self.inner_mut().transforms = v.iter().map(|t| t.inner.clone()).collect();
        self.inner_ref().validate().map_err(PyValueError::new_err)
    }

    #[getter]
    fn parents(&self) -> Vec<i32> {
        self.inner_ref().parents.clone()
    }

    #[setter]
    fn set_parents(&self, v: Vec<i32>) -> PyResult<()> {
        self.inner_mut().parents = v;
        self.inner_ref().validate().map_err(PyValueError::new_err)
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
        self.inner_mut().names = v;
        self.inner_ref().validate().map_err(PyValueError::new_err)
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

    // Dunder

    fn __repr__(&self) -> String {
        let m = self.inner_ref();
        format!("Mesh(parts={})", m.primitives.len())
    }

    // Methods

    fn descendants(&self, i: i32) -> Vec<i32> {
        self.inner_ref().descendants(i)
    }
}

fn parse_col_img(v: &Bound<'_, PyAny>) -> PyResult<ColImage> {
    if let Ok(c) = v.extract::<i32>() {
        return Ok(ColImage::Color(c));
    }
    if let Ok(img_ref) = v.cast::<Image>() {
        return Ok(ColImage::Image(img_ref.borrow().inner.clone()));
    }
    Err(PyTypeError::new_err("col_img must be int or Image"))
}

pub fn add_mesh_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    Ok(())
}
