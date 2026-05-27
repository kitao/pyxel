use pyo3::prelude::*;

use super::quat::Quat;
use super::vec3::Vec3;

define_frozen_wrapper!(Mat4, pyxel::cube::Mat4);

#[pymethods]
impl Mat4 {
    // Constructor

    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::cube::Mat4::identity())
    }

    // Constants

    #[classattr]
    #[allow(non_snake_case)]
    fn IDENTITY() -> Self {
        Self::wrap(pyxel::cube::Mat4::identity())
    }

    // Decomposed view

    #[getter]
    fn pos(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().pos())
    }

    #[getter]
    fn rot(&self) -> Quat {
        Quat::wrap(self.inner_ref().rot())
    }

    #[getter]
    fn scale(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().scale_vec())
    }

    // Dunder methods

    fn __repr__(&self) -> String {
        let m = self.inner_ref();
        format!(
            "Mat4([[{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}]])",
            m.data[0][0],
            m.data[0][1],
            m.data[0][2],
            m.data[0][3],
            m.data[1][0],
            m.data[1][1],
            m.data[1][2],
            m.data[1][3],
            m.data[2][0],
            m.data[2][1],
            m.data[2][2],
            m.data[2][3],
            m.data[3][0],
            m.data[3][1],
            m.data[3][2],
            m.data[3][3],
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner_ref() == other.inner_ref()
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let m = self.inner_ref();
        for row in 0..4 {
            for col in 0..4 {
                m.get(row, col).to_bits().hash(&mut hasher);
            }
        }
        hasher.finish()
    }

    fn __getitem__(&self, key: (usize, usize)) -> PyResult<f32> {
        let (row, col) = key;
        if row >= 4 || col >= 4 {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Mat4 index out of range",
            ));
        }
        Ok(self.inner_ref().get(row, col))
    }

    fn __mul__<'py>(&self, py: Python<'py>, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(mat) = other.extract::<Mat4>() {
            let result = Mat4::wrap(self.inner_ref().mul_mat(mat.inner_ref()));
            Ok(result.into_pyobject(py)?.into_any().unbind())
        } else if let Ok(vec) = other.extract::<Vec3>() {
            let result = Vec3::wrap(self.inner_ref().mul_vec(vec.inner_ref()));
            Ok(result.into_pyobject(py)?.into_any().unbind())
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "Mat4 * other: other must be Mat4 or Vec3",
            ))
        }
    }

    // Class-method factories

    #[staticmethod]
    fn from_translation(pos: PyRef<'_, Vec3>) -> Self {
        Self::wrap(pyxel::cube::Mat4::from_translation(pos.inner_ref()))
    }

    #[staticmethod]
    fn from_euler(euler: PyRef<'_, Vec3>) -> Self {
        Self::wrap(pyxel::cube::Mat4::from_euler(euler.inner_ref()))
    }

    #[staticmethod]
    fn from_quat(rot: PyRef<'_, Quat>) -> Self {
        Self::wrap(pyxel::cube::Mat4::from_quat(rot.inner_ref()))
    }

    #[staticmethod]
    fn from_scale(scale: PyRef<'_, Vec3>) -> Self {
        Self::wrap(pyxel::cube::Mat4::from_scale(scale.inner_ref()))
    }

    #[staticmethod]
    fn from_axis_angle(axis: PyRef<'_, Vec3>, deg: f32) -> Self {
        Self::wrap(pyxel::cube::Mat4::from_axis_angle(axis.inner_ref(), deg))
    }

    #[staticmethod]
    fn compose(pos: PyRef<'_, Vec3>, rot: PyRef<'_, Quat>, scale: PyRef<'_, Vec3>) -> Self {
        Self::wrap(pyxel::cube::Mat4::compose(
            pos.inner_ref(),
            rot.inner_ref(),
            scale.inner_ref(),
        ))
    }

    #[staticmethod]
    #[pyo3(signature = (eye, target, up=None))]
    fn look_at(eye: PyRef<'_, Vec3>, target: PyRef<'_, Vec3>, up: Option<PyRef<'_, Vec3>>) -> Self {
        let default_up = pyxel::cube::Vec3::up();
        let up_inner = up
            .as_ref()
            .map_or_else(|| rc_ref!(&default_up), |u| u.inner_ref());
        Self::wrap(pyxel::cube::Mat4::look_at(
            eye.inner_ref(),
            target.inner_ref(),
            up_inner,
        ))
    }

    // Mutate methods

    fn translate(&self, v: PyRef<'_, Vec3>) -> Self {
        Self::wrap(self.inner_ref().translate(v.inner_ref()))
    }

    fn rotate(&self, axis: PyRef<'_, Vec3>, deg: f32) -> Self {
        Self::wrap(self.inner_ref().rotate(axis.inner_ref(), deg))
    }

    fn rotate_x(&self, deg: f32) -> Self {
        Self::wrap(self.inner_ref().rotate_x(deg))
    }

    fn rotate_y(&self, deg: f32) -> Self {
        Self::wrap(self.inner_ref().rotate_y(deg))
    }

    fn rotate_z(&self, deg: f32) -> Self {
        Self::wrap(self.inner_ref().rotate_z(deg))
    }

    fn scale_by(&self, v: PyRef<'_, Vec3>) -> Self {
        Self::wrap(self.inner_ref().scale_by(v.inner_ref()))
    }

    // Matrix operations

    fn inverse(&self) -> Self {
        Self::wrap(self.inner_ref().inverse())
    }

    fn transpose(&self) -> Self {
        Self::wrap(self.inner_ref().transpose())
    }

    fn determinant(&self) -> f32 {
        self.inner_ref().determinant()
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

pub fn add_mat4_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mat4>()?;
    Ok(())
}
