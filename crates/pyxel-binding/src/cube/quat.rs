use pyo3::prelude::*;

use super::mat4::Mat4;
use super::vec3::Vec3;

define_frozen_wrapper!(Quat, pyxel::cube::Quat);

#[pymethods]
impl Quat {
    // Constructor

    #[new]
    #[pyo3(signature = (x=0.0, y=0.0, z=0.0, w=1.0))]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::wrap(pyxel::cube::Quat::new(x, y, z, w))
    }

    // Constants

    // Python class attributes intentionally expose uppercase constant names.
    #[classattr]
    #[allow(non_snake_case)]
    fn IDENTITY() -> Self {
        Self::wrap(pyxel::cube::Quat::identity())
    }

    // Attributes

    #[getter]
    fn x(&self) -> f32 {
        self.inner_ref().x
    }

    #[getter]
    fn y(&self) -> f32 {
        self.inner_ref().y
    }

    #[getter]
    fn z(&self) -> f32 {
        self.inner_ref().z
    }

    #[getter]
    fn w(&self) -> f32 {
        self.inner_ref().w
    }

    // Dunder methods

    fn __repr__(&self) -> String {
        let q = self.inner_ref();
        format!("Quat({}, {}, {}, {})", q.x, q.y, q.z, q.w)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner_ref() == other.inner_ref()
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let q = self.inner_ref();
        q.x.to_bits().hash(&mut hasher);
        q.y.to_bits().hash(&mut hasher);
        q.z.to_bits().hash(&mut hasher);
        q.w.to_bits().hash(&mut hasher);
        hasher.finish()
    }

    fn __getitem__(&self, key: isize) -> PyResult<f32> {
        let q = self.inner_ref();
        match key {
            0 => Ok(q.x),
            1 => Ok(q.y),
            2 => Ok(q.z),
            3 => Ok(q.w),
            _ => Err(pyo3::exceptions::PyIndexError::new_err(
                "Quat index out of range",
            )),
        }
    }

    fn __iter__(slf: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let q = slf.inner_ref();
        let list = pyo3::types::PyList::new(py, [q.x, q.y, q.z, q.w])?;
        Ok(list.call_method0("__iter__")?.unbind())
    }

    // PyO3 dunder methods must take &self even when the body does not use
    // it; refactoring to an associated function would not register the
    // method on the Python class.
    #[allow(clippy::unused_self)]
    fn __len__(&self) -> usize {
        4
    }

    fn __mul__<'py>(&self, py: Python<'py>, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(quat) = other.extract::<Quat>() {
            let result = Quat::wrap(self.inner_ref().mul_quat(quat.inner_ref()));
            Ok(result.into_pyobject(py)?.into_any().unbind())
        } else if let Ok(vec) = other.extract::<Vec3>() {
            let result = Vec3::wrap(self.inner_ref().mul_vec(vec.inner_ref()));
            Ok(result.into_pyobject(py)?.into_any().unbind())
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "Quat * other: other must be Quat or Vec3",
            ))
        }
    }

    fn __neg__(&self) -> Self {
        Self::wrap(self.inner_ref().neg())
    }

    // Class-method factories

    #[staticmethod]
    fn from_axis_angle(axis: PyRef<'_, Vec3>, deg: f32) -> Self {
        Self::wrap(pyxel::cube::Quat::from_axis_angle(axis.inner_ref(), deg))
    }

    #[staticmethod]
    fn from_euler(rot: PyRef<'_, Vec3>) -> Self {
        Self::wrap(pyxel::cube::Quat::from_euler(rot.inner_ref()))
    }

    #[staticmethod]
    fn from_two_vectors(from_vec: PyRef<'_, Vec3>, to_vec: PyRef<'_, Vec3>) -> Self {
        Self::wrap(pyxel::cube::Quat::from_two_vectors(
            from_vec.inner_ref(),
            to_vec.inner_ref(),
        ))
    }

    #[staticmethod]
    fn from_matrix(mat: PyRef<'_, Mat4>) -> Self {
        Self::wrap(pyxel::cube::Quat::from_matrix(mat.inner_ref()))
    }

    #[staticmethod]
    #[pyo3(signature = (forward, up=None))]
    fn from_direction(forward: PyRef<'_, Vec3>, up: Option<PyRef<'_, Vec3>>) -> Self {
        let default_up = pyxel::cube::Vec3::up();
        let up_inner = up
            .as_ref()
            .map_or_else(|| rc_ref!(&default_up), |u| u.inner_ref());
        Self::wrap(pyxel::cube::Quat::from_direction(
            forward.inner_ref(),
            up_inner,
        ))
    }

    // Unary operations

    fn conjugate(&self) -> Self {
        Self::wrap(self.inner_ref().conjugate())
    }

    fn inverse(&self) -> Self {
        Self::wrap(self.inner_ref().inverse())
    }

    fn normalize(&self) -> Self {
        Self::wrap(self.inner_ref().normalize())
    }

    fn length(&self) -> f32 {
        self.inner_ref().length()
    }

    fn length_squared(&self) -> f32 {
        self.inner_ref().length_squared()
    }

    // Binary operations

    fn dot(&self, other: &Self) -> f32 {
        self.inner_ref().dot(other.inner_ref())
    }

    fn angle_to(&self, other: &Self) -> f32 {
        self.inner_ref().angle_to(other.inner_ref())
    }

    // Conversions

    fn to_matrix(&self) -> Mat4 {
        Mat4::wrap(self.inner_ref().to_matrix())
    }

    fn to_euler(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().to_euler())
    }

    fn to_axis_angle(&self) -> (Vec3, f32) {
        let (axis, deg) = self.inner_ref().to_axis_angle();
        (Vec3::wrap(axis), deg)
    }

    // Interpolation

    fn slerp(&self, other: &Self, t: f32) -> Self {
        Self::wrap(self.inner_ref().slerp(other.inner_ref(), t))
    }
}

pub fn add_quat_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Quat>()?;
    Ok(())
}
