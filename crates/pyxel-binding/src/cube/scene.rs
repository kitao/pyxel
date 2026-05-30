use std::cell::RefCell;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyTuple};
use pyxel::cube::raster::{compute_clip_rect, matmul, projection_matrix, view_matrix};
use pyxel::cube::scene::{clear_draw_context, reset_draw_state, set_draw_context, DrawContext};

use super::camera::Camera;
use super::contact::Contact;
use super::mat4::Mat4;
use super::node::Node;
use super::raycast_hit::RaycastHit;
use super::vec3::Vec3;
use crate::image_wrapper::Image;

// Node-derived root that owns the clear color and drives update / draw.
// The inherited Node provides hierarchy; this wrapper layers on the
// scene-level state defined in core::cube::Scene and the per-frame
// DrawContext that Node draw commands consume.

#[pyclass(unsendable, from_py_object, extends = Node, subclass)]
#[derive(Clone)]
pub struct Scene {
    pub(crate) state: pyxel::cube::RcScene,
    camera: RefCell<Camera>,
}

#[pymethods]
impl Scene {
    #[new]
    #[pyo3(signature = (*_args, **_kwargs))]
    fn new(_args: &Bound<'_, PyTuple>, _kwargs: Option<&Bound<'_, PyDict>>) -> (Self, Node) {
        let scene = Scene {
            state: pyxel::cube::Scene::new(),
            camera: RefCell::new(Camera::wrap(pyxel::cube::Camera::new())),
        };
        (scene, Node::wrap(pyxel::cube::Node::new()))
    }

    #[getter]
    fn clear_color(&self) -> Option<i32> {
        rc_ref!(self.state).clear_color
    }

    #[setter]
    fn set_clear_color(&self, v: Option<i32>) {
        rc_mut!(self.state).clear_color = v;
    }

    #[getter]
    fn camera(&self) -> Camera {
        self.camera.borrow().clone()
    }

    #[setter]
    fn set_camera(&self, camera: Camera) {
        *self.camera.borrow_mut() = camera;
    }

    fn update(slf: PyRef<'_, Self>, py: Python<'_>) -> PyResult<()> {
        let scene_inner_node = slf.as_super().inner.clone();
        let any = slf.into_pyobject(py)?.into_any();
        // Step 1: on_update traversal (visits scene + every active node).
        traverse_update(&any)?;
        // Step 2: motion integration.
        pyxel::cube::Scene::integrate_motion(&scene_inner_node);
        // Steps 3-6: AABB refresh, broad / narrow phase, response resolution.
        let pairs = pyxel::cube::Scene::detect_contacts(&scene_inner_node);
        // Step 7: notification — both sides, deterministic pre-order.
        for pair in pairs {
            let py_a = find_py_node_in_tree(&any, &pair.node_a)?;
            let py_b = find_py_node_in_tree(&any, &pair.node_b)?;
            if let (Some(a), Some(b)) = (py_a, py_b) {
                let contact_a = Contact::wrap(pair.contact_a);
                let contact_b = Contact::wrap(pair.contact_b);
                a.bind(py)
                    .call_method1("on_collide", (b.clone_ref(py), contact_a))?;
                b.bind(py).call_method1("on_collide", (a, contact_b))?;
            }
        }
        // Step 8: deferred destruction. Walk the tree post-order
        // collecting flagged nodes, fire on_destroy on each (leaves
        // first), then detach from the parent links on both the core
        // and Python sides.
        let destroyed = pyxel::cube::Scene::collect_destroyed_post_order(&scene_inner_node);
        for inner in &destroyed {
            if let Some(py_node) = find_py_node_in_tree(&any, inner)? {
                py_node.bind(py).call_method0("on_destroy")?;
            }
        }
        for inner in &destroyed {
            if let Some(py_node) = find_py_node_in_tree(&any, inner)? {
                py_node.bind(py).borrow().detach_from_parent_py(py);
            }
            pyxel::cube::Scene::detach_destroyed(inner);
        }
        Ok(())
    }

    #[pyo3(signature = (origin, direction, max_distance=None, hit_triggers=false, tags=None))]
    fn raycast(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        origin: PyRef<'_, Vec3>,
        direction: PyRef<'_, Vec3>,
        max_distance: Option<f32>,
        hit_triggers: bool,
        tags: Option<Vec<String>>,
    ) -> PyResult<Option<RaycastHit>> {
        let scene_inner_node = slf.as_super().inner.clone();
        let scene_any = slf.into_pyobject(py)?.into_any();
        let origin_v = *origin.inner_ref();
        let direction_v = *direction.inner_ref();
        let max_dist = max_distance.unwrap_or(f32::INFINITY);
        let hit = pyxel::cube::Scene::raycast(
            &scene_inner_node,
            origin_v,
            direction_v,
            max_dist,
            hit_triggers,
            tags.as_deref(),
        );
        match hit {
            Some(info) => Ok(Some(wrap_raycast_hit(&scene_any, info)?)),
            None => Ok(None),
        }
    }

    #[pyo3(signature = (origin, direction, max_distance=None, hit_triggers=false, tags=None))]
    fn raycast_all(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        origin: PyRef<'_, Vec3>,
        direction: PyRef<'_, Vec3>,
        max_distance: Option<f32>,
        hit_triggers: bool,
        tags: Option<Vec<String>>,
    ) -> PyResult<Vec<RaycastHit>> {
        let scene_inner_node = slf.as_super().inner.clone();
        let scene_any = slf.into_pyobject(py)?.into_any();
        let origin_v = *origin.inner_ref();
        let direction_v = *direction.inner_ref();
        let max_dist = max_distance.unwrap_or(f32::INFINITY);
        let infos = pyxel::cube::Scene::raycast_all(
            &scene_inner_node,
            origin_v,
            direction_v,
            max_dist,
            hit_triggers,
            tags.as_deref(),
        );
        let mut out: Vec<RaycastHit> = Vec::with_capacity(infos.len());
        for info in infos {
            out.push(wrap_raycast_hit(&scene_any, info)?);
        }
        Ok(out)
    }

    #[pyo3(signature = (center, radius, hit_triggers=false, tags=None))]
    fn overlap_sphere(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        center: PyRef<'_, Vec3>,
        radius: f32,
        hit_triggers: bool,
        tags: Option<Vec<String>>,
    ) -> PyResult<Vec<Py<Node>>> {
        let scene_inner_node = slf.as_super().inner.clone();
        let scene_any = slf.into_pyobject(py)?.into_any();
        let center_v = *center.inner_ref();
        let inner_results = pyxel::cube::Scene::overlap_sphere(
            &scene_inner_node,
            center_v,
            radius,
            hit_triggers,
            tags.as_deref(),
        );
        wrap_node_results(&scene_any, &inner_results)
    }

    #[pyo3(signature = (transform, size, hit_triggers=false, tags=None))]
    fn overlap_box(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        transform: PyRef<'_, Mat4>,
        size: PyRef<'_, Vec3>,
        hit_triggers: bool,
        tags: Option<Vec<String>>,
    ) -> PyResult<Vec<Py<Node>>> {
        let scene_inner_node = slf.as_super().inner.clone();
        let scene_any = slf.into_pyobject(py)?.into_any();
        let transform_m = *transform.inner_ref();
        let size_v = *size.inner_ref();
        let inner_results = pyxel::cube::Scene::overlap_box(
            &scene_inner_node,
            &transform_m,
            size_v,
            hit_triggers,
            tags.as_deref(),
        );
        wrap_node_results(&scene_any, &inner_results)
    }

    #[pyo3(signature = (x, y, w, h, screen=None))]
    fn draw(
        self_: Bound<'_, Self>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        screen: Option<PyRef<'_, Image>>,
    ) -> PyResult<()> {
        let scene_state = self_.borrow().state.clone();
        let target_rc = match screen.as_ref() {
            Some(s) => s.inner.clone(),
            None => pyxel::screen().clone(),
        };
        let target_w = rc_ref!(&target_rc).width();
        let target_h = rc_ref!(&target_rc).height();
        let clear_color = rc_ref!(&scene_state).clear_color;
        if let Some(col) = clear_color {
            rc_mut!(&target_rc).clear(col as u8);
        }
        // Depth buffer is owned by the receiver Node's cache. For Scene
        // backward-compatibility (Tasks 1-6 transitional state), allocate
        // a fresh per-call buffer here; Task 6 routes this entry point
        // through Node::draw so the cache is reused across frames.
        let depth_size = (target_w * target_h) as usize;
        let depth = vec![f32::INFINITY; depth_size];
        let cam_inner = self_.borrow().camera.borrow().inner.clone();
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
            depth,
            depth_w: target_w,
            depth_h: target_h,
            // Per-on_draw modifiers; reset_draw_state() is invoked by
            // traverse_draw before each Node.on_draw so these defaults are
            // re-seeded per-node.
            dither_alpha: 1.0,
            depth_test: true,
            depth_write: true,
            shaded: true,
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
    reset_draw_state();
    node.call_method0("on_draw")?;
    let children = node.getattr("children")?;
    let children_iter = children.try_iter()?;
    for child in children_iter {
        traverse_draw(&child?)?;
    }
    Ok(())
}

// Resolve an inner RcNode to the matching Py<Node> wrapper by walking
// the scene tree. The lookup is O(N) per call; spatial queries are not
// expected to fire on the hot path.
fn find_py_node_in_tree(
    root: &Bound<'_, PyAny>,
    target: &pyxel::cube::RcNode,
) -> PyResult<Option<Py<Node>>> {
    if let Ok(node_bound) = root.cast::<Node>() {
        if std::rc::Rc::ptr_eq(&node_bound.borrow().inner, target) {
            return Ok(Some(node_bound.clone().unbind()));
        }
    }
    let children = root.getattr("children")?;
    let children_iter = children.try_iter()?;
    for child in children_iter {
        let child = child?;
        if let Some(found) = find_py_node_in_tree(&child, target)? {
            return Ok(Some(found));
        }
    }
    Ok(None)
}

fn wrap_raycast_hit(
    scene_any: &Bound<'_, PyAny>,
    info: pyxel::cube::scene::RaycastHitInfo,
) -> PyResult<RaycastHit> {
    // Resolve the inner RcNode to its scene-tree Py<Node> so
    // `hit.node is scene_node` holds (mirrors the overlap_* path).
    let py_node = find_py_node_in_tree(scene_any, &info.node)?;
    let rch = pyxel::cube::RaycastHit::new();
    {
        let r = rc_mut!(&rch);
        r.node = Some(info.node);
        r.point = pyxel::cube::Vec3::new(info.point.x, info.point.y, info.point.z);
        r.normal = pyxel::cube::Vec3::new(info.normal.x, info.normal.y, info.normal.z);
        r.distance = info.distance;
    }
    Ok(match py_node {
        Some(p) => RaycastHit::wrap_with_py_node(rch, p),
        None => RaycastHit::wrap(rch),
    })
}

fn wrap_node_results(
    scene_any: &Bound<'_, PyAny>,
    inner: &[pyxel::cube::RcNode],
) -> PyResult<Vec<Py<Node>>> {
    let mut out: Vec<Py<Node>> = Vec::with_capacity(inner.len());
    for rc in inner {
        if let Some(py_node) = find_py_node_in_tree(scene_any, rc)? {
            out.push(py_node);
        }
    }
    Ok(out)
}

pub fn add_scene_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scene>()?;
    Ok(())
}
