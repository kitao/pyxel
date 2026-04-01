use pyo3::prelude::*;
use pyxel::cube;

use crate::cube::math_wrapper::Vec3;

#[pyclass(name = "Model")]
pub struct Model {
    pub inner: *mut cube::Model,
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    #[allow(clippy::mut_from_ref)]
    fn inner_mut(&self) -> &mut cube::Model {
        unsafe { &mut *self.inner }
    }
}

#[pymethods]
impl Model {
    #[new]
    fn new() -> Self {
        Self {
            inner: Box::into_raw(Box::new(cube::Model::new())),
        }
    }

    #[staticmethod]
    fn cube(col: u8) -> Self {
        Self {
            inner: Box::into_raw(Box::new(cube::Model::cube(col))),
        }
    }

    #[staticmethod]
    fn plane(col: u8) -> Self {
        Self {
            inner: Box::into_raw(Box::new(cube::Model::plane(col))),
        }
    }

    #[staticmethod]
    fn pyramid(col: u8) -> Self {
        Self {
            inner: Box::into_raw(Box::new(cube::Model::pyramid(col))),
        }
    }

    #[staticmethod]
    fn sphere(col: u8) -> Self {
        Self {
            inner: Box::into_raw(Box::new(cube::Model::sphere(col))),
        }
    }

    #[staticmethod]
    fn tex_cube(img: u32, u: f32, v: f32, w: f32, h: f32) -> Self {
        Self {
            inner: Box::into_raw(Box::new(cube::Model::tex_cube(img, u, v, w, h))),
        }
    }

    #[staticmethod]
    fn tex_pyramid(img: u32, u: f32, v: f32, w: f32, h: f32) -> Self {
        Self {
            inner: Box::into_raw(Box::new(cube::Model::tex_pyramid(img, u, v, w, h))),
        }
    }

    #[staticmethod]
    fn tex_sphere(img: u32, u: f32, v: f32, w: f32, h: f32) -> Self {
        Self {
            inner: Box::into_raw(Box::new(cube::Model::tex_sphere(img, u, v, w, h))),
        }
    }

    fn tri(&self, v0: &Vec3, v1: &Vec3, v2: &Vec3, col: u8) {
        self.inner_mut().tri(v0.inner, v1.inner, v2.inner, col);
    }

    #[pyo3(signature = (v0, v1, v2, img, uv0, uv1, uv2))]
    fn tri_tex(
        &self,
        v0: &Vec3,
        v1: &Vec3,
        v2: &Vec3,
        img: u32,
        uv0: (f32, f32),
        uv1: (f32, f32),
        uv2: (f32, f32),
    ) {
        self.inner_mut().tri_tex(
            v0.inner,
            v1.inner,
            v2.inner,
            img,
            cube::Uv::new(uv0.0, uv0.1),
            cube::Uv::new(uv1.0, uv1.1),
            cube::Uv::new(uv2.0, uv2.1),
        );
    }

    fn node_pos(&self, name: &str) -> Option<Vec3> {
        let model = unsafe { &*self.inner };
        model.node(name).map(|n| Vec3 { inner: n.pos })
    }

    fn set_node_rot(&self, name: &str, rot: &Vec3) {
        if let Some(node) = self.inner_mut().node_mut(name) {
            node.rot = rot.inner;
        }
    }
}

pub fn add_cube_model_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Model>()?;
    Ok(())
}
