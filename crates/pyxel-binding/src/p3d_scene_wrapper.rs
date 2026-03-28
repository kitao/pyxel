use pyo3::prelude::*;
use pyxel::p3d;

use crate::p3d_camera_wrapper::Camera;
use crate::p3d_light_wrapper::Light;
use crate::p3d_math_wrapper::Vec3;
use crate::p3d_model_wrapper::Model;

#[pyclass(name = "Scene")]
pub struct Scene {
    pub inner: p3d::Scene,
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}

#[pymethods]
impl Scene {
    #[new]
    fn new() -> Self {
        Self {
            inner: p3d::Scene::new(),
        }
    }

    #[pyo3(signature = (model, pos=None, rot=None, scale=None))]
    fn add(&mut self, model: &Model, pos: Option<&Vec3>, rot: Option<&Vec3>, scale: Option<&Vec3>) {
        let pos = pos.map_or(p3d::Vec3::ZERO, |v| v.inner);
        let rot = rot.map_or(p3d::Vec3::ZERO, |v| v.inner);
        let scale = scale.map_or(p3d::Vec3::new(1.0, 1.0, 1.0), |v| v.inner);
        self.inner.add(model.inner, pos, rot, scale);
    }

    fn remove_all(&mut self) {
        self.inner.remove_all();
    }

    fn set_light(&mut self, index: usize, light: &Light) {
        self.inner.set_light(
            index,
            p3d::Light {
                dir: light.inner.dir,
            },
        );
    }

    fn draw(&mut self, x: i32, y: i32, w: u32, h: u32, camera: &Camera) {
        self.inner.draw(x, y, w, h, &camera.inner);
    }
}

pub fn add_p3d_scene_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scene>()?;
    Ok(())
}
