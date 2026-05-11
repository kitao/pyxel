use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;

use super::geometry::Geometry;
use super::mat4::Mat4;
use crate::image_wrapper::Image;

define_wrapper!(Mesh, pyxel::cube::Mesh);

#[pymethods]
impl Mesh {
    // Constructor

    #[new]
    #[pyo3(signature = (
        geometries=None,
        transforms=None,
        parents=None,
        col_img=None,
        colkey=None,
    ))]
    fn new(
        py: Python<'_>,
        geometries: Option<Vec<Option<PyRef<'_, Geometry>>>>,
        transforms: Option<Vec<PyRef<'_, Mat4>>>,
        parents: Option<Vec<i32>>,
        col_img: Option<Bound<'_, PyAny>>,
        colkey: Option<i32>,
    ) -> PyResult<Self> {
        let mesh = pyxel::cube::Mesh::new();
        {
            let m = rc_mut!(&mesh);
            if let Some(gs) = geometries {
                m.geometries = gs.into_iter().map(|g| g.map(|g| g.inner.clone())).collect();
            }
            if let Some(ts) = transforms {
                m.transforms = ts.iter().map(|t| t.inner.clone()).collect();
            }
            if let Some(ps) = parents {
                m.parents = ps;
            }
            if let Some(ci) = col_img {
                m.col_img = parse_col_img(py, &ci)?;
            }
            m.colkey = colkey;
        }
        rc_ref!(&mesh).validate().map_err(PyValueError::new_err)?;
        Ok(Self::wrap(mesh))
    }

    // Parts (parallel arrays)

    #[getter]
    fn geometries(&self, py: Python<'_>) -> PyResult<Py<PyList>> {
        let inner = self.inner_ref();
        let items: Vec<Py<PyAny>> = inner
            .geometries
            .iter()
            .map(|g| match g {
                Some(g) => match Geometry::wrap(g.clone()).into_pyobject(py) {
                    Ok(b) => b.into_any().unbind(),
                    Err(_) => py.None(),
                },
                None => py.None(),
            })
            .collect();
        Ok(PyList::new(py, items)?.unbind())
    }

    #[setter]
    fn set_geometries(&self, v: Vec<Option<PyRef<'_, Geometry>>>) -> PyResult<()> {
        self.inner_mut().geometries = v.into_iter().map(|g| g.map(|g| g.inner.clone())).collect();
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

    // Shared material

    #[getter]
    fn col_img(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match &self.inner_ref().col_img {
            pyxel::cube::mesh::ColImage::Color(c) => Ok(c.into_pyobject(py)?.into_any().unbind()),
            pyxel::cube::mesh::ColImage::Image(img) => Ok(Image::wrap(img.clone())
                .into_pyobject(py)?
                .into_any()
                .unbind()),
        }
    }

    #[setter]
    fn set_col_img(&self, py: Python<'_>, v: Bound<'_, PyAny>) -> PyResult<()> {
        self.inner_mut().col_img = parse_col_img(py, &v)?;
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

    // Methods

    fn descendants(&self, i: i32) -> Vec<i32> {
        self.inner_ref().descendants(i)
    }

    // Dunder

    fn __repr__(&self) -> String {
        let m = self.inner_ref();
        format!("Mesh(parts={})", m.geometries.len())
    }
}

pub(super) fn parse_col_img(
    _py: Python<'_>,
    v: &Bound<'_, PyAny>,
) -> PyResult<pyxel::cube::mesh::ColImage> {
    if let Ok(c) = v.extract::<i32>() {
        return Ok(pyxel::cube::mesh::ColImage::Color(c));
    }
    if let Ok(img_ref) = v.cast::<Image>() {
        return Ok(pyxel::cube::mesh::ColImage::Image(
            img_ref.borrow().inner.clone(),
        ));
    }
    Err(PyTypeError::new_err("col_img must be int or Image"))
}

pub fn add_mesh_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    Ok(())
}
