use pyo3::prelude::*;
use pyxel::cube;

#[pyclass(name = "Vec3", from_py_object)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub inner: cube::Vec3,
}

#[pymethods]
impl Vec3 {
    #[new]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            inner: cube::Vec3::new(x, y, z),
        }
    }

    #[getter]
    fn x(&self) -> f32 {
        self.inner.x
    }

    #[getter]
    fn y(&self) -> f32 {
        self.inner.y
    }

    #[getter]
    fn z(&self) -> f32 {
        self.inner.z
    }

    fn dot(&self, other: &Vec3) -> f32 {
        self.inner.dot(other.inner)
    }

    fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            inner: self.inner.cross(other.inner),
        }
    }

    fn length(&self) -> f32 {
        self.inner.length()
    }

    fn normalize(&self) -> Vec3 {
        Vec3 {
            inner: self.inner.normalize(),
        }
    }

    fn __add__(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            inner: self.inner + other.inner,
        }
    }

    fn __sub__(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            inner: self.inner - other.inner,
        }
    }

    fn __mul__(&self, scalar: f32) -> Vec3 {
        Vec3 {
            inner: self.inner * scalar,
        }
    }

    fn __neg__(&self) -> Vec3 {
        Vec3 { inner: -self.inner }
    }

    fn __repr__(&self) -> String {
        format!("Vec3({}, {}, {})", self.inner.x, self.inner.y, self.inner.z)
    }
}

#[pyclass(name = "Mat4", from_py_object)]
#[derive(Clone, Copy)]
pub struct Mat4 {
    pub inner: cube::Mat4,
}

#[pymethods]
impl Mat4 {
    #[staticmethod]
    fn identity() -> Mat4 {
        Mat4 {
            inner: cube::Mat4::identity(),
        }
    }

    #[staticmethod]
    fn translation(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4 {
            inner: cube::Mat4::translation(x, y, z),
        }
    }

    #[staticmethod]
    fn rotation_x(deg: f32) -> Mat4 {
        Mat4 {
            inner: cube::Mat4::rotation_x(deg),
        }
    }

    #[staticmethod]
    fn rotation_y(deg: f32) -> Mat4 {
        Mat4 {
            inner: cube::Mat4::rotation_y(deg),
        }
    }

    #[staticmethod]
    fn rotation_z(deg: f32) -> Mat4 {
        Mat4 {
            inner: cube::Mat4::rotation_z(deg),
        }
    }

    #[staticmethod]
    fn scale(sx: f32, sy: f32, sz: f32) -> Mat4 {
        Mat4 {
            inner: cube::Mat4::scale(sx, sy, sz),
        }
    }

    #[staticmethod]
    fn look_at(eye: &Vec3, target: &Vec3, up: &Vec3) -> Mat4 {
        Mat4 {
            inner: cube::Mat4::look_at(eye.inner, target.inner, up.inner),
        }
    }

    #[staticmethod]
    fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        Mat4 {
            inner: cube::Mat4::perspective(fov, aspect, near, far),
        }
    }

    fn transform_point(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            inner: self.inner.transform_point(v.inner),
        }
    }

    fn __mul__(&self, other: &Mat4) -> Mat4 {
        Mat4 {
            inner: self.inner * other.inner,
        }
    }

    fn __repr__(&self) -> String {
        let m = &self.inner.m;
        format!(
            "Mat4([{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}])",
            m[0],
            m[1],
            m[2],
            m[3],
            m[4],
            m[5],
            m[6],
            m[7],
            m[8],
            m[9],
            m[10],
            m[11],
            m[12],
            m[13],
            m[14],
            m[15]
        )
    }
}

pub fn add_cube_math_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Vec3>()?;
    m.add_class::<Mat4>()?;
    Ok(())
}
