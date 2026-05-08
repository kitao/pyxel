use pyo3::prelude::*;

use super::mat4::Mat4;

define_frozen_wrapper!(Vec3, pyxel::cube::Vec3);

#[pymethods]
impl Vec3 {
    // Constructor

    #[new]
    #[pyo3(signature = (x=0.0, y=0.0, z=0.0))]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self::wrap(pyxel::cube::Vec3::new(x, y, z))
    }

    // Constants

    #[classattr]
    #[allow(non_snake_case)]
    fn ZERO() -> Self {
        Self::wrap(pyxel::cube::Vec3::zero())
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn ONE() -> Self {
        Self::wrap(pyxel::cube::Vec3::one())
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn RIGHT() -> Self {
        Self::wrap(pyxel::cube::Vec3::right())
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn LEFT() -> Self {
        Self::wrap(pyxel::cube::Vec3::left())
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn UP() -> Self {
        Self::wrap(pyxel::cube::Vec3::up())
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn DOWN() -> Self {
        Self::wrap(pyxel::cube::Vec3::down())
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn FORWARD() -> Self {
        Self::wrap(pyxel::cube::Vec3::forward())
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn BACK() -> Self {
        Self::wrap(pyxel::cube::Vec3::back())
    }

    // Attributes (read-only)

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

    // Dunder methods

    fn __repr__(&self) -> String {
        let v = self.inner_ref();
        format!("Vec3({}, {}, {})", v.x, v.y, v.z)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner_ref() == other.inner_ref()
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let v = self.inner_ref();
        v.x.to_bits().hash(&mut hasher);
        v.y.to_bits().hash(&mut hasher);
        v.z.to_bits().hash(&mut hasher);
        hasher.finish()
    }

    fn __getitem__(&self, key: isize) -> PyResult<f32> {
        let v = self.inner_ref();
        match key {
            0 => Ok(v.x),
            1 => Ok(v.y),
            2 => Ok(v.z),
            _ => Err(pyo3::exceptions::PyIndexError::new_err(
                "Vec3 index out of range",
            )),
        }
    }

    fn __iter__(slf: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let v = slf.inner_ref();
        let list = pyo3::types::PyList::new(py, [v.x, v.y, v.z])?;
        Ok(list.call_method0("__iter__")?.unbind())
    }

    // PyO3 dunder methods must take &self even when the body does not use
    // it; refactoring to an associated function would not register the
    // method on the Python class.
    #[allow(clippy::unused_self)]
    fn __len__(&self) -> usize {
        3
    }

    fn __add__(&self, other: &Self) -> Self {
        Self::wrap(self.inner_ref().add(other.inner_ref()))
    }

    fn __sub__(&self, other: &Self) -> Self {
        Self::wrap(self.inner_ref().sub(other.inner_ref()))
    }

    fn __mul__(&self, scalar: f32) -> Self {
        Self::wrap(self.inner_ref().mul(scalar))
    }

    fn __rmul__(&self, scalar: f32) -> Self {
        Self::wrap(self.inner_ref().mul(scalar))
    }

    fn __truediv__(&self, scalar: f32) -> Self {
        Self::wrap(self.inner_ref().div(scalar))
    }

    fn __neg__(&self) -> Self {
        Self::wrap(self.inner_ref().neg())
    }

    // Math methods

    fn dot(&self, other: &Self) -> f32 {
        self.inner_ref().dot(other.inner_ref())
    }

    fn cross(&self, other: &Self) -> Self {
        Self::wrap(self.inner_ref().cross(other.inner_ref()))
    }

    fn length(&self) -> f32 {
        self.inner_ref().length()
    }

    fn length_squared(&self) -> f32 {
        self.inner_ref().length_squared()
    }

    fn distance_to(&self, other: &Self) -> f32 {
        self.inner_ref().distance_to(other.inner_ref())
    }

    fn distance_squared_to(&self, other: &Self) -> f32 {
        self.inner_ref().distance_squared_to(other.inner_ref())
    }

    fn angle_to(&self, other: &Self) -> f32 {
        self.inner_ref().angle_to(other.inner_ref())
    }

    fn normalize(&self) -> Self {
        Self::wrap(self.inner_ref().normalize())
    }

    fn clamp_length(&self, max_length: f32) -> Self {
        Self::wrap(self.inner_ref().clamp_length(max_length))
    }

    fn min(&self, other: &Self) -> Self {
        Self::wrap(self.inner_ref().min(other.inner_ref()))
    }

    fn max(&self, other: &Self) -> Self {
        Self::wrap(self.inner_ref().max(other.inner_ref()))
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::wrap(self.inner_ref().lerp(other.inner_ref(), t))
    }

    fn slerp(&self, other: &Self, t: f32) -> Self {
        Self::wrap(self.inner_ref().slerp(other.inner_ref(), t))
    }

    fn reflect(&self, normal: &Self) -> Self {
        Self::wrap(self.inner_ref().reflect(normal.inner_ref()))
    }

    fn project(&self, other: &Self) -> Self {
        Self::wrap(self.inner_ref().project(other.inner_ref()))
    }

    // Coordinate system conversions

    fn to_local(&self, mat: PyRef<'_, Mat4>) -> Self {
        Self::wrap(self.inner_ref().to_local(mat.inner_ref()))
    }

    fn to_world(&self, mat: PyRef<'_, Mat4>) -> Self {
        Self::wrap(self.inner_ref().to_world(mat.inner_ref()))
    }

    fn to_local_dir(&self, mat: PyRef<'_, Mat4>) -> Self {
        Self::wrap(self.inner_ref().to_local_dir(mat.inner_ref()))
    }

    fn to_world_dir(&self, mat: PyRef<'_, Mat4>) -> Self {
        Self::wrap(self.inner_ref().to_world_dir(mat.inner_ref()))
    }
}

pub fn add_vec3_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Vec3>()?;
    Ok(())
}
