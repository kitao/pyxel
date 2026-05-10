use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyxel::cube::raster::{compute_clip_rect, matmul, projection_matrix, view_matrix};
use pyxel::cube::scene::{clear_draw_context, set_draw_context, DrawContext};

use super::camera::Camera;
use super::node::Node;
use crate::image_wrapper::Image;

// Node-derived root that owns the clear color and drives update / draw.
// The inherited Node provides hierarchy; this wrapper layers on the
// scene-level state defined in core::cube::Scene and the per-frame
// DrawContext that Node draw commands consume.

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
        // Scene seeds a default Shading from the current Pyxel palette so
        // descendants always resolve a non-None effective shading through
        // the inherit-from-ancestor cascade. Users can swap it on the
        // Scene for global changes or override per-subtree.
        let inner_node = pyxel::cube::Node::new();
        {
            let n = rc_mut!(&inner_node);
            let palette = pyxel::colors().clone();
            n.shading = Some(pyxel::cube::Shading::new(&palette));
        }
        let node = Node::wrap(inner_node);
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

    fn update(self_: Bound<'_, Self>) -> PyResult<()> {
        let any = self_.into_any();
        traverse_update(&any)
    }

    #[pyo3(signature = (x, y, w, h, camera, screen=None))]
    fn draw(
        self_: Bound<'_, Self>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        camera: PyRef<'_, Camera>,
        screen: Option<PyRef<'_, Image>>,
    ) -> PyResult<()> {
        let scene_state = self_.borrow().state.clone();
        let target_rc = match screen.as_ref() {
            Some(s) => s.inner.clone(),
            None => pyxel::screen().clone(),
        };
        let target_w = rc_ref!(&target_rc).width();
        let target_h = rc_ref!(&target_rc).height();
        {
            let scene_mut = rc_mut!(&scene_state);
            scene_mut.ensure_depth(target_w, target_h);
            if let Some(col) = scene_mut.clear_color {
                rc_mut!(&target_rc).clear(col as u8);
                scene_mut.clear_depth();
            }
        }
        let cam_inner = camera.inner.clone();
        let view = view_matrix(rc_ref!(&cam_inner));
        let proj = projection_matrix(rc_ref!(&cam_inner), w as f32, h as f32);
        let vp = matmul(&proj, &view);
        let clip = compute_clip_rect(x as f32, y as f32, w as f32, h as f32, target_w, target_h);
        set_draw_context(DrawContext {
            target: target_rc,
            vp,
            vp_x: x as f32,
            vp_y: y as f32,
            vp_w: w as f32,
            vp_h: h as f32,
            clip,
            camera: cam_inner,
            scene: scene_state,
            // Defaults; each draw command rebinds these from its own
            // keyword arguments before invoking the rasterizer.
            dither_alpha: 1.0,
            depth_test: true,
            depth_write: true,
        });
        let any = self_.into_any();
        let result = traverse_draw(&any);
        clear_draw_context();
        result
    }

    fn __repr__(&self) -> String {
        format!("Scene(clear_color={:?})", rc_ref!(self.state).clear_color)
    }
}

// Pre-order tree traversal that respects `active` cascade and dispatches
// each node's `on_update`. Subtrees rooted at an inactive node are
// skipped entirely.
fn traverse_update(node: &Bound<'_, PyAny>) -> PyResult<()> {
    let active: bool = node.getattr("active")?.extract()?;
    if !active {
        return Ok(());
    }
    node.call_method0("on_update")?;
    let children = node.getattr("children")?;
    let children_iter = children.try_iter()?;
    for child in children_iter {
        traverse_update(&child?)?;
    }
    Ok(())
}

// Pre-order tree traversal that respects `visible` cascade and
// dispatches each node's `on_draw` inside the active draw context.
fn traverse_draw(node: &Bound<'_, PyAny>) -> PyResult<()> {
    let visible: bool = node.getattr("visible")?.extract()?;
    if !visible {
        return Ok(());
    }
    node.call_method0("on_draw")?;
    let children = node.getattr("children")?;
    let children_iter = children.try_iter()?;
    for child in children_iter {
        traverse_draw(&child?)?;
    }
    Ok(())
}

pub fn add_scene_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scene>()?;
    Ok(())
}
