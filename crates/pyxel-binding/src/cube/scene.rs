use pyo3::prelude::*;

use super::camera::Camera;
use super::node::Node;
use crate::image_wrapper::Image;

// Node-derived root that owns the clear color and drives update / draw.
// The inherited Node (see super::node) provides the hierarchy itself; this
// wrapper layers on the scene-level state defined in core::cube::Scene.

#[pyclass(unsendable, from_py_object, extends = Node)]
#[derive(Clone)]
pub struct Scene {
    pub(crate) state: pyxel::cube::RcScene,
}

#[pymethods]
impl Scene {
    #[new]
    fn new() -> (Self, Node) {
        let scene = Scene {
            state: pyxel::cube::Scene::new(),
        };
        let node = Node::wrap(pyxel::cube::Node::new());
        (scene, node)
    }

    #[getter]
    fn clear_color(&self) -> Option<i32> {
        rc_ref!(self.state).clear_color
    }

    #[setter]
    fn set_clear_color(&self, v: Option<i32>) {
        rc_mut!(self.state).clear_color = v;
    }

    fn update(self_: PyRef<'_, Self>) {
        // Tree traversal + on_update dispatch is wired in phase D once Node
        // draw commands and Python lifecycle hooks are in place.
        let _ = self_;
    }

    #[pyo3(signature = (x, y, w, h, camera, screen=None))]
    fn draw(
        self_: PyRef<'_, Self>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        camera: PyRef<'_, Camera>,
        screen: Option<PyRef<'_, Image>>,
    ) {
        // Clear + traversal + on_draw dispatch is wired in phase D once the
        // rasterizer is reattached as Node draw commands.
        let _ = (self_, x, y, w, h, camera, screen);
    }

    fn __repr__(&self) -> String {
        format!("Scene(clear_color={:?})", rc_ref!(self.state).clear_color)
    }
}

pub fn add_scene_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scene>()?;
    Ok(())
}
