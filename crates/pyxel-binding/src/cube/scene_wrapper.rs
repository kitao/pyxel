use pyo3::prelude::*;
use pyxel::cube;

use crate::cube::camera_wrapper::Camera;
use crate::cube::light_wrapper::Light;
use crate::cube::math_wrapper::Vec3;
use crate::cube::model_wrapper::Model;

define_wrapper!(Scene, pyxel::cube::Scene);

#[pymethods]
impl Scene {
    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::cube::Scene::new())
    }

    #[pyo3(signature = (model, pos=None, rot=None, scale=None))]
    fn add(&self, model: &Model, pos: Option<&Vec3>, rot: Option<&Vec3>, scale: Option<&Vec3>) {
        let pos = pos.map_or(cube::Vec3::ZERO, |v| v.inner);
        let rot = rot.map_or(cube::Vec3::ZERO, |v| v.inner);
        let scale = scale.map_or(cube::Vec3::new(1.0, 1.0, 1.0), |v| v.inner);
        self.inner_mut().add(model.inner.clone(), pos, rot, scale);
    }

    fn remove_all(&self) {
        self.inner_mut().remove_all();
    }

    fn set_light(&self, index: usize, light: &Light) {
        self.inner_mut().set_light(index, rc_ref!(light.inner));
    }

    fn draw(&self, x: i32, y: i32, w: u32, h: u32, camera: &Camera) {
        self.inner_mut().draw(x, y, w, h, rc_ref!(camera.inner));
    }
}

pub fn add_cube_scene_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scene>()?;
    Ok(())
}
