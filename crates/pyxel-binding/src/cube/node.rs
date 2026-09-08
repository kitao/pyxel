use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple, PyType};
use pyxel::cube::draw::{DrawState, Uvs as QuadUvs};
use pyxel::cube::mesh::ColImage;
use pyxel::cube::raster::{
    camera_clip_row, compute_clip_rect, matmul, projection_matrix, view_matrix,
};
use pyxel::cube::scene::{
    reset_draw_state, set_draw_context, take_draw_context, with_draw_context, DrawContext,
};
use pyxel::cube::Node as InnerNode;

use super::camera::Camera;
use super::collider::Collider;
use super::contact::Contact;
use super::mat4::Mat4;
use super::motion::Motion;
use super::raycast_hit::RaycastHit;
use super::shading::Shading;
use super::vec3::Vec3;

#[derive(Clone)]
struct DrawablePrimitive {
    primitive: pyxel::cube::RcPrimitive,
    col_img: ColImage,
    colkey: Option<i32>,
}

#[derive(Clone)]
struct MotionPlayer {
    motion: pyxel::cube::RcMotion,
    frame: f32,
    looping: bool,
    speed: f32,
}

// Cache child wrappers so user-defined Node subclasses retain their Python
// identity and on_update/on_draw overrides throughout the scene tree.

#[pyclass(module = "pyxel.cube", unsendable, from_py_object, subclass)]
pub struct Node {
    pub(crate) inner: pyxel::cube::RcNode,
    children: RefCell<Vec<Py<Node>>>,
    drawable_primitive: RefCell<Option<DrawablePrimitive>>,
    // Strong ref back to the parent wrapper; the cyclic ref is resolved
    // by __traverse__ / __clear__ so Python's gc can collect detached
    // subtrees.
    parent: RefCell<Option<Py<Node>>>,
    mesh_source: RefCell<Option<pyxel::cube::RcMesh>>,
    mesh_part_index: RefCell<Option<usize>>,
    motion_player: RefCell<Option<MotionPlayer>>,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Python::try_attach(|py| Self {
            inner: self.inner.clone(),
            children: RefCell::new(
                self.children
                    .borrow()
                    .iter()
                    .map(|c| c.clone_ref(py))
                    .collect(),
            ),
            drawable_primitive: RefCell::new(self.drawable_primitive.borrow().clone()),
            parent: RefCell::new(self.parent.borrow().as_ref().map(|p| p.clone_ref(py))),
            mesh_source: RefCell::new(self.mesh_source.borrow().clone()),
            mesh_part_index: RefCell::new(*self.mesh_part_index.borrow()),
            motion_player: RefCell::new(self.motion_player.borrow().clone()),
        })
        .expect("Node clone requires the Python GIL to be attached")
    }
}

impl Node {
    pub fn wrap(inner: pyxel::cube::RcNode) -> Self {
        Self {
            inner,
            children: RefCell::new(Vec::new()),
            drawable_primitive: RefCell::new(None),
            parent: RefCell::new(None),
            mesh_source: RefCell::new(None),
            mesh_part_index: RefCell::new(None),
            motion_player: RefCell::new(None),
        }
    }

    pub(crate) fn detach_from_parent_py(&self, py: Python<'_>) {
        let parent = self.parent.borrow_mut().take();
        if let Some(parent_py) = parent {
            let pb = parent_py.bind(py).borrow();
            pb.children
                .borrow_mut()
                .retain(|c| !Rc::ptr_eq(&c.bind(py).borrow().inner, &self.inner));
        }
    }

    pub(crate) fn inner_ref(&self) -> std::cell::Ref<'_, pyxel::cube::Node> {
        rc_ref!(self.inner)
    }

    pub(crate) fn inner_mut(&self) -> std::cell::RefMut<'_, pyxel::cube::Node> {
        rc_mut!(self.inner)
    }

    fn world_mat(&self) -> pyxel::cube::Mat4 {
        InnerNode::world_transform_value(&self.inner)
    }

    fn world_mat_compose(&self, local: pyxel::cube::Mat4) -> pyxel::cube::Mat4 {
        InnerNode::world_transform_value(&self.inner).mul_mat_value(&local)
    }

    fn with_state_from_ctx(
        &self,
        billboard: i32,
        f: impl FnOnce(&mut pyxel::cube::scene::DrawContext, pyxel::cube::draw::DrawState),
    ) {
        // Resolve the per-Node cascading shading once. The rasterizer will
        // only consult it when ctx.shaded is true.
        let shading_rc = InnerNode::effective_shading(&self.inner);
        with_draw_context(|ctx| {
            let shading_ref = if ctx.shaded {
                shading_rc.as_ref().map(|s| rc_ref!(s))
            } else {
                None
            };
            let state = DrawState {
                shaded: ctx.shaded,
                dither_alpha: ctx.dither_alpha,
                depth_test: ctx.depth_test,
                depth_write: ctx.depth_write,
                billboard,
                shading: shading_ref.as_deref(),
            };
            f(ctx, state);
        });
    }

    fn draw_attached_primitive(&self) -> PyResult<()> {
        let Some(drawable) = self.drawable_primitive.borrow().clone() else {
            return Ok(());
        };
        let world_mat = self.world_mat();
        let p = rc_ref!(&drawable.primitive);
        self.draw_primitive(&world_mat, &p, &drawable.col_img, drawable.colkey)
    }

    // Emits one Primitive through draw::prim under the active draw state,
    // mapping empty attributes to the rasterizer's None and its message to
    // a ValueError.
    fn draw_primitive(
        &self,
        world_mat: &pyxel::cube::Mat4,
        p: &pyxel::cube::Primitive,
        col_img: &ColImage,
        colkey: Option<i32>,
    ) -> PyResult<()> {
        let indices = (!p.indices.is_empty()).then_some(p.indices.as_slice());
        let normals = (!p.normals.is_empty()).then_some(p.normals.as_slice());
        let uvs = (!p.uvs.is_empty()).then_some(p.uvs.as_slice());
        let (col_flat, col_image) = col_img.as_flat_and_image();
        let mut inner_result: Option<Result<(), &str>> = None;

        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            inner_result = Some(pyxel::cube::draw::prim(
                ctx,
                world_mat,
                p.mode,
                p.cull,
                &p.positions,
                indices,
                normals,
                uvs,
                col_flat,
                col_image.as_ref(),
                colkey,
                state,
            ));
        });
        match inner_result {
            Some(Err(msg)) => Err(PyValueError::new_err(msg)),
            Some(Ok(())) | None => Ok(()),
        }
    }

    // Pre-order subtree walk collecting the nodes whose core state
    // satisfies `matches`.
    fn collect_matching(
        node: &Py<Node>,
        py: Python<'_>,
        matches: &dyn Fn(&InnerNode) -> bool,
        out: &mut Vec<Py<Node>>,
    ) {
        let bound = node.bind(py);
        let n = bound.borrow();
        if matches(&n.inner_ref()) {
            out.push(node.clone_ref(py));
        }
        for child in n.children.borrow().iter() {
            Self::collect_matching(child, py, matches, out);
        }
    }

    fn validate_motion_source(&self, motion: &Motion) -> PyResult<pyxel::cube::RcMesh> {
        let Some(node_source) = self.mesh_source.borrow().clone() else {
            return Err(PyValueError::new_err(
                "Node motion methods require a tree created by Node.from_mesh",
            ));
        };
        if !Rc::ptr_eq(&node_source, motion.source_mesh()) {
            return Err(PyValueError::new_err(
                "motion must come from the same Mesh as the Node.from_mesh tree",
            ));
        }
        Ok(node_source)
    }

    fn collect_mesh_nodes(
        root: &Py<Node>,
        py: Python<'_>,
        source: &pyxel::cube::RcMesh,
        out: &mut Vec<(usize, Py<Node>)>,
    ) {
        let bound = root.bind(py);
        let node = bound.borrow();
        let node_source = node.mesh_source.borrow().clone();
        let node_index = *node.mesh_part_index.borrow();
        if node_source
            .as_ref()
            .is_some_and(|node_source| Rc::ptr_eq(node_source, source))
        {
            if let Some(index) = node_index {
                out.push((index, root.clone_ref(py)));
            }
        }

        for child in node.children.borrow().iter() {
            Self::collect_mesh_nodes(child, py, source, out);
        }
    }

    fn apply_motion_inner(
        root: &Py<Node>,
        py: Python<'_>,
        source: &pyxel::cube::RcMesh,
        motion: &pyxel::cube::RcMotion,
        frame: f32,
        looping: bool,
    ) {
        let sampled = rc_ref!(motion).sample(frame, looping);
        let mut nodes = Vec::new();
        Self::collect_mesh_nodes(root, py, source, &mut nodes);
        nodes.sort_unstable_by_key(|(part_index, _)| *part_index);

        let mut node_cursor = 0;
        for (part_index, transform) in sampled {
            while node_cursor < nodes.len() && nodes[node_cursor].0 < part_index {
                node_cursor += 1;
            }

            let mut matching_cursor = node_cursor;
            while matching_cursor < nodes.len() && nodes[matching_cursor].0 == part_index {
                nodes[matching_cursor]
                    .1
                    .bind(py)
                    .borrow()
                    .inner_mut()
                    .transform = pyxel::cube::Mat4::from_rows(transform.data);
                matching_cursor += 1;
            }
            node_cursor = matching_cursor;
        }
    }
}

#[pymethods]
impl Node {
    #[new]
    #[classmethod]
    #[pyo3(signature = (*args, **kwargs), text_signature = "()")]
    fn new(
        cls: &Bound<'_, PyType>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Subclass __init__ arguments pass through this inherited constructor.
        let node_type = cls.py().get_type::<Node>();
        let is_exact_node = cls.as_ptr() == node_type.as_ptr();
        let has_kwargs = kwargs.is_some_and(|kwargs| !kwargs.is_empty());
        if is_exact_node && (!args.is_empty() || has_kwargs) {
            return Err(PyTypeError::new_err("Node() takes no arguments"));
        }
        Ok(Self::wrap(InnerNode::new()))
    }

    #[staticmethod]
    fn from_mesh(py: Python<'_>, mesh: PyRef<'_, super::mesh::Mesh>) -> PyResult<Py<Node>> {
        // Python allocations can trigger GC callbacks that mutate the source mesh.
        let (nodes, parents) = {
            let mesh_inner = mesh.inner_ref();
            mesh_inner.validate().map_err(PyValueError::new_err)?;

            let count = mesh_inner.primitives.len();
            let mut nodes = Vec::with_capacity(count);
            for i in 0..count {
                let inner = InnerNode::new();
                {
                    let mut node_inner = rc_mut!(&inner);
                    node_inner.name = mesh_inner.names.get(i).cloned().unwrap_or_default();
                    node_inner.transform = mesh_inner.transforms[i].clone();
                }

                let node = Self::wrap(inner);
                *node.mesh_source.borrow_mut() = Some(mesh.inner.clone());
                *node.mesh_part_index.borrow_mut() = Some(i);
                if let Some(primitive) = &mesh_inner.primitives[i] {
                    let material = mesh_inner.material_for_part(i);
                    *node.drawable_primitive.borrow_mut() = Some(DrawablePrimitive {
                        primitive: primitive.clone(),
                        col_img: material.col_img,
                        colkey: material.colkey,
                    });
                }
                nodes.push(node);
            }

            (nodes, mesh_inner.parents.clone())
        };

        let nodes = nodes
            .into_iter()
            .map(|node| Py::new(py, node))
            .collect::<PyResult<Vec<_>>>()?;

        let mut root_indices = Vec::new();
        for (i, &parent) in parents.iter().enumerate() {
            if parent == -1 {
                root_indices.push(i);
            } else {
                let parent_node = nodes[parent as usize].bind(py).clone();
                let child_node = nodes[i].clone_ref(py);
                Self::add_child(parent_node, py, child_node)?;
            }
        }

        if root_indices.len() == 1 {
            Ok(nodes[root_indices[0]].clone_ref(py))
        } else {
            let synthetic_root = Self::wrap(InnerNode::new());
            *synthetic_root.mesh_source.borrow_mut() = Some(mesh.inner.clone());
            let root = Py::new(py, synthetic_root)?;
            for index in root_indices {
                Self::add_child(root.bind(py).clone(), py, nodes[index].clone_ref(py))?;
            }
            Ok(root)
        }
    }

    // Data attributes

    #[getter]
    fn name(&self) -> String {
        self.inner_ref().name.clone()
    }

    #[setter]
    fn set_name(&self, v: String) {
        self.inner_mut().name = v;
    }

    #[getter]
    fn transform(&self) -> Mat4 {
        Mat4::wrap(self.inner_ref().transform.clone())
    }

    #[setter]
    fn set_transform(&self, v: PyRef<'_, Mat4>) {
        self.inner_mut().transform = v.inner.clone();
    }

    #[getter]
    fn active(&self) -> bool {
        self.inner_ref().active
    }

    #[setter]
    fn set_active(&self, v: bool) {
        self.inner_mut().active = v;
    }

    #[getter]
    fn visible(&self) -> bool {
        self.inner_ref().visible
    }

    #[setter]
    fn set_visible(&self, v: bool) {
        self.inner_mut().visible = v;
    }

    // Calling draw() on this node uses its camera or the nearest ancestor's camera.
    #[getter]
    fn camera(&self) -> Option<Camera> {
        self.inner_ref()
            .camera
            .as_ref()
            .map(|c| Camera::wrap(c.clone()))
    }

    #[setter]
    fn set_camera(&self, v: Option<PyRef<'_, Camera>>) {
        self.inner_mut().camera = v.as_ref().map(|c| c.inner.clone());
    }

    // None inherits ancestor shading; without any shading, geometry draws unlit.
    #[getter]
    fn shading(&self) -> Option<Shading> {
        self.inner_ref()
            .shading
            .as_ref()
            .map(|s| Shading::wrap(s.clone()))
    }

    #[setter]
    fn set_shading(&self, v: Option<PyRef<'_, Shading>>) {
        self.inner_mut().shading = v.as_ref().map(|s| s.inner.clone());
    }

    #[getter]
    fn collider(&self) -> Option<Collider> {
        self.inner_ref()
            .collider
            .as_ref()
            .map(|c| Collider::wrap(c.clone()))
    }

    #[setter]
    fn set_collider(&self, v: Option<PyRef<'_, Collider>>) {
        self.inner_mut().collider = v.as_ref().map(|c| c.inner.clone());
    }

    #[getter]
    fn tags(&self) -> Vec<String> {
        self.inner_ref().tags.clone()
    }

    #[setter]
    fn set_tags(&self, v: Vec<String>) {
        self.inner_mut().tags = v;
    }

    // Properties

    #[getter]
    fn parent(&self, py: Python<'_>) -> Option<Py<Node>> {
        self.parent.borrow().as_ref().map(|p| p.clone_ref(py))
    }

    const CHILD_INDEX_LINEAR_MAX: usize = 16;

    #[getter]
    fn children<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let core_children = InnerNode::children(&self.inner);
        let mut cache = self.children.borrow_mut();

        // Reconcile wrappers with core children while preserving Python identity.
        // Wide nodes use a pointer index to keep lookup linear.
        let mut items: Vec<Py<Node>> = Vec::with_capacity(core_children.len());
        if core_children.len() <= Self::CHILD_INDEX_LINEAR_MAX {
            for c_inner in &core_children {
                if let Some(cached) = cache
                    .iter()
                    .find(|cached| Rc::ptr_eq(&cached.bind(py).borrow().inner, c_inner))
                {
                    items.push(cached.clone_ref(py));
                }
            }
        } else {
            let by_ptr: HashMap<*const _, &Py<Node>> = cache
                .iter()
                .map(|cached| (Rc::as_ptr(&cached.bind(py).borrow().inner), cached))
                .collect();
            for c_inner in &core_children {
                if let Some(cached) = by_ptr.get(&Rc::as_ptr(c_inner)) {
                    items.push(cached.clone_ref(py));
                }
            }
        }

        let unchanged = items.len() == cache.len()
            && items
                .iter()
                .zip(cache.iter())
                .all(|(a, b)| a.as_ptr() == b.as_ptr());
        if !unchanged {
            *cache = items.iter().map(|p| p.clone_ref(py)).collect();
        }

        // The tuple allocation can run Python's gc, whose __traverse__
        // reads the cache, so release the borrow first.
        drop(cache);
        PyTuple::new(py, items)
    }

    #[getter]
    fn destroyed(&self) -> bool {
        self.inner_ref().destroyed
    }

    #[getter]
    fn forward(&self) -> Vec3 {
        Vec3::wrap(InnerNode::forward(&self.inner))
    }

    #[getter]
    fn right(&self) -> Vec3 {
        Vec3::wrap(InnerNode::right(&self.inner))
    }

    #[getter]
    fn up(&self) -> Vec3 {
        Vec3::wrap(InnerNode::up(&self.inner))
    }

    #[getter]
    fn effective_camera(&self) -> Option<Camera> {
        InnerNode::effective_camera(&self.inner).map(Camera::wrap)
    }

    #[getter]
    fn effective_shading(&self) -> Option<Shading> {
        InnerNode::effective_shading(&self.inner).map(Shading::wrap)
    }

    fn __repr__(&self) -> String {
        let n = self.inner_ref();
        format!(
            "Node(name={:?}, children={})",
            n.name,
            self.children.borrow().len()
        )
    }

    // Hierarchy management

    #[getter]
    fn world_transform(&self) -> Mat4 {
        Mat4::wrap(InnerNode::world_transform(&self.inner))
    }

    fn find_by_name(slf: PyRef<'_, Node>, py: Python<'_>, name: &str) -> PyResult<Vec<Py<Node>>> {
        let self_py: Py<Node> = slf.into_pyobject(py)?.unbind();
        let mut out: Vec<Py<Node>> = Vec::new();
        Self::collect_matching(&self_py, py, &|n| n.name == name, &mut out);
        Ok(out)
    }

    fn find_by_tags(
        slf: PyRef<'_, Node>,
        py: Python<'_>,
        tags: Vec<String>,
    ) -> PyResult<Vec<Py<Node>>> {
        let self_py: Py<Node> = slf.into_pyobject(py)?.unbind();
        let mut out: Vec<Py<Node>> = Vec::new();
        Self::collect_matching(
            &self_py,
            py,
            &|n| tags.iter().any(|t| n.tags.contains(t)),
            &mut out,
        );
        Ok(out)
    }

    #[pyo3(signature = (node), text_signature = "($self, /, node)")]
    fn add_child(slf: Bound<'_, Self>, py: Python<'_>, node: Py<Node>) -> PyResult<()> {
        let node_inner = node.bind(py).borrow().inner.clone();
        if !InnerNode::add_child(&slf.borrow().inner, &node_inner) {
            return Err(PyValueError::new_err("add_child would create a cycle"));
        }
        node.bind(py).borrow().detach_from_parent_py(py);
        let self_py: Py<Node> = slf.clone().unbind();
        *node.bind(py).borrow().parent.borrow_mut() = Some(self_py);
        slf.borrow().children.borrow_mut().push(node);
        Ok(())
    }

    #[pyo3(signature = (node), text_signature = "($self, /, node)")]
    fn remove_child(slf: Bound<'_, Self>, py: Python<'_>, node: Py<Node>) -> PyResult<()> {
        let node_inner = node.bind(py).borrow().inner.clone();
        if !InnerNode::remove_child(&slf.borrow().inner, &node_inner) {
            return Err(PyValueError::new_err(
                "remove_child requires a direct child",
            ));
        }
        slf.borrow()
            .children
            .borrow_mut()
            .retain(|c| !Rc::ptr_eq(&c.bind(py).borrow().inner, &node_inner));
        *node.bind(py).borrow().parent.borrow_mut() = None;
        Ok(())
    }

    // Defer post-order notification and detachment to the end of Node.update
    // so parent/child links remain stable during the current traversal.
    fn destroy(slf: PyRef<'_, Self>) {
        InnerNode::destroy(&slf.inner);
    }

    // Motion

    #[pyo3(signature = (motion, frame, *, r#loop=true))]
    fn apply_motion(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        motion: PyRef<'_, Motion>,
        frame: f32,
        r#loop: bool,
    ) -> PyResult<()> {
        let source = slf.validate_motion_source(&motion)?;
        let motion_inner = motion.inner.clone();
        let self_py: Py<Node> = slf.into_pyobject(py)?.unbind();
        Self::apply_motion_inner(&self_py, py, &source, &motion_inner, frame, r#loop);
        Ok(())
    }

    #[pyo3(signature = (motion, *, r#loop=true, speed=1.0, start_frame=0.0))]
    fn play_motion(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        motion: PyRef<'_, Motion>,
        r#loop: bool,
        speed: f32,
        start_frame: f32,
    ) -> PyResult<()> {
        if !speed.is_finite() {
            return Err(PyValueError::new_err("speed must be finite"));
        }
        if !start_frame.is_finite() {
            return Err(PyValueError::new_err("start_frame must be finite"));
        }

        let source = slf.validate_motion_source(&motion)?;
        let motion_inner = motion.inner.clone();
        // Bound looping playheads before the first increment to retain small steps.
        let start_frame = if r#loop {
            rc_ref!(&motion_inner).resolve_frame(start_frame, true)
        } else {
            start_frame
        };

        let self_py: Py<Node> = slf.into_pyobject(py)?.unbind();
        Self::apply_motion_inner(&self_py, py, &source, &motion_inner, start_frame, r#loop);
        *self_py.bind(py).borrow().motion_player.borrow_mut() = Some(MotionPlayer {
            motion: motion_inner,
            frame: start_frame,
            looping: r#loop,
            speed,
        });
        Ok(())
    }

    fn stop_motion(&self) {
        *self.motion_player.borrow_mut() = None;
    }

    // Lifecycle hooks
    // PyO3 requires these Python override points to be instance methods even
    // when the default implementation does not read self or its arguments.

    #[allow(clippy::unused_self)]
    fn on_update(&self) {}

    #[allow(clippy::unused_self)]
    fn on_draw(&self) {}

    #[allow(clippy::unused_self, unused_variables)]
    fn on_collide(&self, other: PyRef<'_, Node>, contact: PyRef<'_, Contact>) {}

    #[allow(clippy::unused_self)]
    fn on_destroy(&self) {}

    // Drawing state

    // PyO3 requires instance receivers; these setters use the active draw context.
    #[allow(clippy::unused_self)]
    fn dither(&self, alpha: f32) {
        with_draw_context(|ctx| {
            ctx.dither_alpha = alpha.clamp(0.0, 1.0);
        });
    }

    #[allow(clippy::unused_self)]
    fn depth_test(&self, on: bool) {
        with_draw_context(|ctx| {
            ctx.depth_test = on;
        });
    }

    #[allow(clippy::unused_self)]
    fn depth_write(&self, on: bool) {
        with_draw_context(|ctx| {
            ctx.depth_write = on;
        });
    }

    #[allow(clippy::unused_self)]
    fn depth_offset(&self, offset: f32) {
        with_draw_context(|ctx| {
            ctx.depth_offset = offset;
        });
    }

    #[allow(clippy::unused_self)]
    fn shaded(&self, on: bool) {
        with_draw_context(|ctx| {
            ctx.shaded = on;
        });
    }

    // Linework

    fn pset(&self, pos: PyRef<'_, Vec3>, col: i32) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::pset(ctx, &world_mat, &local, col, state);
        });
    }

    fn line(&self, p1: PyRef<'_, Vec3>, p2: PyRef<'_, Vec3>, col: i32) {
        let world_mat = self.world_mat();
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::line(ctx, &world_mat, &v1, &v2, col, state);
        });
    }

    // 2D polygons

    fn tri(&self, p1: PyRef<'_, Vec3>, p2: PyRef<'_, Vec3>, p3: PyRef<'_, Vec3>, col: i32) {
        let world_mat = self.world_mat();
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        let v3 = *p3.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::tri(ctx, &world_mat, &v1, &v2, &v3, col, state);
        });
    }

    fn trib(&self, p1: PyRef<'_, Vec3>, p2: PyRef<'_, Vec3>, p3: PyRef<'_, Vec3>, col: i32) {
        let world_mat = self.world_mat();
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        let v3 = *p3.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::trib(ctx, &world_mat, &v1, &v2, &v3, col, state);
        });
    }

    fn rect(&self, mat: PyRef<'_, Mat4>, w: f32, h: f32, col: i32) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::rect(ctx, &world_mat, w, h, col, state);
        });
    }

    fn rectb(&self, mat: PyRef<'_, Mat4>, w: f32, h: f32, col: i32) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::rectb(ctx, &world_mat, w, h, col, state);
        });
    }

    // 2D curves

    fn circ(&self, pos: PyRef<'_, Vec3>, r: f32, col: i32) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_ON, |ctx, state| {
            pyxel::cube::draw::circ(ctx, &world_mat, &local, r, col, state);
        });
    }

    fn circb(&self, pos: PyRef<'_, Vec3>, r: f32, col: i32) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_ON, |ctx, state| {
            pyxel::cube::draw::circb(ctx, &world_mat, &local, r, col, state);
        });
    }

    fn elli(&self, mat: PyRef<'_, Mat4>, w: f32, h: f32, col: i32) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::elli(ctx, &world_mat, w, h, col, state);
        });
    }

    fn ellib(&self, mat: PyRef<'_, Mat4>, w: f32, h: f32, col: i32) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::ellib(ctx, &world_mat, w, h, col, state);
        });
    }

    // 3D solids

    #[pyo3(signature = (mat, size, col_img=None, *, colkey=None))]
    fn r#box(
        &self,
        mat: PyRef<'_, Mat4>,
        size: PyRef<'_, Vec3>,
        col_img: Option<&Bound<'_, PyAny>>,
        colkey: Option<i32>,
    ) -> PyResult<()> {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let size_v = *size.inner_ref();
        let (col_flat, col_image) = super::mesh::resolve_col_img(col_img)?.as_flat_and_image();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::box_solid(
                ctx,
                &world_mat,
                &size_v,
                col_flat,
                col_image.as_ref(),
                colkey,
                state,
            );
        });
        Ok(())
    }

    fn boxb(&self, mat: PyRef<'_, Mat4>, size: PyRef<'_, Vec3>, col: i32) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let size_v = *size.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::boxb(ctx, &world_mat, &size_v, col, state);
        });
    }

    #[pyo3(signature = (pos, r, col_img=None, *, colkey=None))]
    fn sphere(
        &self,
        pos: PyRef<'_, Vec3>,
        r: f32,
        col_img: Option<&Bound<'_, PyAny>>,
        colkey: Option<i32>,
    ) -> PyResult<()> {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        let (col_flat, col_image) = super::mesh::resolve_col_img(col_img)?.as_flat_and_image();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::sphere(
                ctx,
                &world_mat,
                &local,
                r,
                col_flat,
                col_image.as_ref(),
                colkey,
                state,
            );
        });
        Ok(())
    }

    fn sphereb(&self, pos: PyRef<'_, Vec3>, r: f32, col: i32) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::sphereb(ctx, &world_mat, &local, r, col, state);
        });
    }

    // Textured quads

    #[pyo3(signature = (mat, img, uvs, w, h, *, colkey=None))]
    fn plane(
        &self,
        mat: PyRef<'_, Mat4>,
        img: PyRef<'_, crate::image_wrapper::Image>,
        uvs: QuadUvs,
        w: f32,
        h: f32,
        colkey: Option<i32>,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let img_inner = img.inner.clone();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::plane(ctx, &world_mat, &img_inner, uvs, w, h, colkey, state);
        });
    }

    #[pyo3(signature = (pos, img, uvs, w, h, *, colkey=None, angle=0.0))]
    fn sprite(
        &self,
        pos: PyRef<'_, Vec3>,
        img: PyRef<'_, crate::image_wrapper::Image>,
        uvs: QuadUvs,
        w: f32,
        h: f32,
        colkey: Option<i32>,
        angle: f32,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        let img_inner = img.inner.clone();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_ON, |ctx, state| {
            pyxel::cube::draw::sprite(
                ctx, &world_mat, &local, &img_inner, uvs, w, h, colkey, angle, state,
            );
        });
    }

    // Asset-based

    #[pyo3(signature = (mat, primitive, col_img=None, *, colkey=None))]
    fn prim(
        &self,
        mat: PyRef<'_, Mat4>,
        primitive: PyRef<'_, super::primitive::Primitive>,
        col_img: Option<&Bound<'_, PyAny>>,
        colkey: Option<i32>,
    ) -> PyResult<()> {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let col_img = super::mesh::resolve_col_img(col_img)?;
        // Borrowed for the duration of the draw; the draw runs Rust-only
        // code, so the proxy-backed arrays cannot change mid-draw.
        let p = rc_ref!(&primitive.inner);
        self.draw_primitive(&world_mat, &p, &col_img, colkey)
    }

    #[pyo3(signature = (pos, s, col, *, font=None))]
    fn text(
        &self,
        pos: PyRef<'_, Vec3>,
        s: &str,
        col: i32,
        font: Option<PyRef<'_, crate::font_wrapper::Font>>,
    ) {
        let world_mat = self.world_mat();
        let pos_v = *pos.inner_ref();
        let font_rc = font.as_ref().map(|f| f.inner.clone());
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_ON, |ctx, state| {
            let mut font_guard = font_rc.as_ref().map(|f| rc_mut!(f));
            let font_ref = font_guard.as_deref_mut();
            pyxel::cube::draw::text(ctx, &world_mat, &pos_v, s, col, font_ref, state);
        });
    }

    // Frame lifecycle

    // Each phase visits the whole subtree before the next phase begins.
    fn update(slf: PyRef<'_, Self>, py: Python<'_>) -> PyResult<()> {
        let root_inner = slf.inner.clone();
        let any = slf.into_pyobject(py)?.into_any();
        traverse_update(&any)?;
        traverse_motion_players(&any)?;
        pyxel::cube::Scene::integrate_motion(&root_inner);

        let pairs = pyxel::cube::Scene::detect_contacts(&root_inner);
        if !pairs.is_empty() {
            let node_index = build_py_node_index(&any)?;
            for pair in pairs {
                let py_a = find_indexed_py_node(&node_index, &pair.node_a, py);
                let py_b = find_indexed_py_node(&node_index, &pair.node_b, py);
                if let (Some(a), Some(b)) = (py_a, py_b) {
                    let contact_a = Contact::wrap(pair.contact_a);
                    let contact_b = Contact::wrap(pair.contact_b);
                    a.bind(py)
                        .call_method1("on_collide", (b.clone_ref(py), contact_a))?;
                    b.bind(py).call_method1("on_collide", (a, contact_b))?;
                }
            }
        }

        let destroyed = pyxel::cube::Scene::collect_destroyed_post_order(&root_inner);
        if !destroyed.is_empty() {
            let node_index = build_py_node_index(&any)?;
            for inner in &destroyed {
                if let Some(py_node) = find_indexed_py_node(&node_index, inner, py) {
                    py_node.bind(py).call_method0("on_destroy")?;
                }
            }

            for inner in &destroyed {
                if let Some(py_node) = find_indexed_py_node(&node_index, inner, py) {
                    py_node.bind(py).borrow().detach_from_parent_py(py);
                }
                pyxel::cube::Scene::detach_destroyed(inner);
            }
        }
        Ok(())
    }

    #[pyo3(signature = (x, y, w, h, target=None))]
    fn draw(
        slf: Bound<'_, Self>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        target: Option<PyRef<'_, crate::image_wrapper::Image>>,
    ) -> PyResult<()> {
        // A nested draw would replace the active context and lose the outer
        // camera's depth buffer; multi-view rendering uses sequential draws.
        if with_draw_context(|_| ()).is_some() {
            return Err(PyValueError::new_err(
                "draw cannot be called from inside on_draw",
            ));
        }

        let node_inner = slf.borrow().inner.clone();
        let cam_inner = InnerNode::effective_camera(&node_inner).ok_or_else(|| {
            PyValueError::new_err("draw requires a camera on this node or an ancestor")
        })?;
        let target_rc = match target.as_ref() {
            Some(t) => t.inner.clone(),
            None => pyxel::screen().clone(),
        };
        let target_w = rc_ref!(&target_rc).width();
        let target_h = rc_ref!(&target_rc).height();
        if let Some(col) = rc_ref!(&cam_inner).clear_color {
            rc_mut!(&target_rc).clear(col as u8);
        }

        // Lend camera buffers to the draw context for this traversal.
        rc_mut!(&cam_inner).ensure_depth(target_w, target_h);
        rc_mut!(&cam_inner).clear_depth();
        let depth = std::mem::take(&mut rc_mut!(&cam_inner).depth);
        let vertex_cache = std::mem::take(&mut rc_mut!(&cam_inner).vertex_scratch);

        let view = view_matrix(&rc_ref!(&cam_inner));
        let proj = projection_matrix(&rc_ref!(&cam_inner), w as f32, h as f32);
        let vp = matmul(&proj, &view);
        let clip_row = camera_clip_row(&view);
        let clip = compute_clip_rect(x as f32, y as f32, w as f32, h as f32, target_w, target_h);

        set_draw_context(DrawContext {
            target: target_rc,
            vp,
            clip_row,
            vp_x: x as f32,
            vp_y: y as f32,
            vp_w: w as f32,
            vp_h: h as f32,
            clip,
            camera: cam_inner.clone(),

            depth,
            depth_w: target_w,
            depth_h: target_h,
            vertex_cache,

            dither_alpha: 1.0,
            depth_test: true,
            depth_write: true,
            depth_offset: 0.0,
            shaded: true,
        });

        let any = slf.into_any();
        let result = traverse_draw(&any);
        if let Some(ctx) = take_draw_context() {
            rc_mut!(&cam_inner).depth = ctx.depth;
            rc_mut!(&cam_inner).vertex_scratch = ctx.vertex_cache;
        }
        result
    }

    // Spatial queries on this Node's subtree

    #[pyo3(signature = (origin, direction, max_distance=None, *, hit_triggers=false, tags=None))]
    fn raycast(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        origin: PyRef<'_, Vec3>,
        direction: PyRef<'_, Vec3>,
        max_distance: Option<f32>,
        hit_triggers: bool,
        tags: Option<Vec<String>>,
    ) -> PyResult<Option<super::raycast_hit::RaycastHit>> {
        let root_inner = slf.inner.clone();
        let root_any = slf.into_pyobject(py)?.into_any();
        let origin_v = *origin.inner_ref();
        let direction_v = *direction.inner_ref();
        let max_dist = max_distance.unwrap_or(f32::INFINITY);
        let hit = pyxel::cube::Scene::raycast(
            &root_inner,
            origin_v,
            direction_v,
            max_dist,
            hit_triggers,
            tags.as_deref(),
        );

        match hit {
            Some(info) => {
                let node_index = build_py_node_index(&root_any)?;
                Ok(Some(wrap_raycast_hit(&node_index, py, info)))
            }
            None => Ok(None),
        }
    }

    #[pyo3(signature = (origin, direction, max_distance=None, *, hit_triggers=false, tags=None))]
    fn raycast_all(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        origin: PyRef<'_, Vec3>,
        direction: PyRef<'_, Vec3>,
        max_distance: Option<f32>,
        hit_triggers: bool,
        tags: Option<Vec<String>>,
    ) -> PyResult<Vec<super::raycast_hit::RaycastHit>> {
        let root_inner = slf.inner.clone();
        let root_any = slf.into_pyobject(py)?.into_any();
        let origin_v = *origin.inner_ref();
        let direction_v = *direction.inner_ref();
        let max_dist = max_distance.unwrap_or(f32::INFINITY);
        let infos = pyxel::cube::Scene::raycast_all(
            &root_inner,
            origin_v,
            direction_v,
            max_dist,
            hit_triggers,
            tags.as_deref(),
        );
        if infos.is_empty() {
            return Ok(Vec::new());
        }

        let node_index = build_py_node_index(&root_any)?;
        let mut out: Vec<super::raycast_hit::RaycastHit> = Vec::with_capacity(infos.len());
        for info in infos {
            out.push(wrap_raycast_hit(&node_index, py, info));
        }
        Ok(out)
    }

    #[pyo3(signature = (center, radius, *, hit_triggers=false, tags=None))]
    fn overlap_sphere(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        center: PyRef<'_, Vec3>,
        radius: f32,
        hit_triggers: bool,
        tags: Option<Vec<String>>,
    ) -> PyResult<Vec<Py<Node>>> {
        let root_inner = slf.inner.clone();
        let root_any = slf.into_pyobject(py)?.into_any();
        let center_v = *center.inner_ref();
        let inner_results = pyxel::cube::Scene::overlap_sphere(
            &root_inner,
            center_v,
            radius,
            hit_triggers,
            tags.as_deref(),
        );
        if inner_results.is_empty() {
            return Ok(Vec::new());
        }

        let node_index = build_py_node_index(&root_any)?;
        Ok(wrap_node_results(&node_index, py, &inner_results))
    }

    #[pyo3(signature = (mat, size, *, hit_triggers=false, tags=None))]
    fn overlap_box(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        mat: PyRef<'_, Mat4>,
        size: PyRef<'_, Vec3>,
        hit_triggers: bool,
        tags: Option<Vec<String>>,
    ) -> PyResult<Vec<Py<Node>>> {
        let root_inner = slf.inner.clone();
        let root_any = slf.into_pyobject(py)?.into_any();
        let mat_m = *mat.inner_ref();
        let size_v = *size.inner_ref();
        let inner_results = pyxel::cube::Scene::overlap_box(
            &root_inner,
            &mat_m,
            size_v,
            hit_triggers,
            tags.as_deref(),
        );
        if inner_results.is_empty() {
            return Ok(Vec::new());
        }

        let node_index = build_py_node_index(&root_any)?;
        Ok(wrap_node_results(&node_index, py, &inner_results))
    }

    // Python GC integration

    fn __traverse__(&self, visit: pyo3::PyVisit<'_>) -> Result<(), pyo3::PyTraverseError> {
        for child in self.children.borrow().iter() {
            visit.call(child)?;
        }
        if let Some(parent) = self.parent.borrow().as_ref() {
            visit.call(parent)?;
        }
        Ok(())
    }

    fn __clear__(&self) {
        self.children.borrow_mut().clear();
        *self.parent.borrow_mut() = None;
    }
}

fn push_children_reverse(node: &Bound<'_, Node>, py: Python<'_>, stack: &mut Vec<Py<PyAny>>) {
    let node_ref = node.borrow();
    for child in node_ref.children.borrow().iter().rev() {
        stack.push(child.clone_ref(py).into_any());
    }
}

// Pre-order tree traversal that skips inactive subtrees and dispatches updates.
fn traverse_update(root: &Bound<'_, PyAny>) -> PyResult<()> {
    let py = root.py();
    let mut stack = vec![root.clone().unbind()];
    while let Some(node) = stack.pop() {
        let node = node.bind(py);
        if !node.getattr("active")?.extract::<bool>()? {
            continue;
        }
        node.call_method0("on_update")?;
        push_children_reverse(node.cast::<Node>()?, py, &mut stack);
    }
    Ok(())
}

// Pre-order playback traversal that follows the active cascade.
fn traverse_motion_players(root: &Bound<'_, PyAny>) -> PyResult<()> {
    let py = root.py();
    let mut stack = vec![root.clone().unbind()];

    while let Some(node) = stack.pop() {
        let node = node.bind(py);
        if !node.getattr("active")?.extract::<bool>()? {
            continue;
        }

        let node_bound = node.cast::<Node>()?;
        let node_ref = node_bound.borrow();
        let source = node_ref.mesh_source.borrow().clone();
        let player = node_ref.motion_player.borrow().clone();
        drop(node_ref);

        if let (Some(mut player), Some(source)) = (player, source) {
            player.frame += player.speed;
            player.frame = rc_ref!(&player.motion).resolve_frame(player.frame, player.looping);
            let node_py = node_bound.clone().unbind();
            Node::apply_motion_inner(
                &node_py,
                node.py(),
                &source,
                &player.motion,
                player.frame,
                player.looping,
            );
            *node_py.bind(node.py()).borrow().motion_player.borrow_mut() = Some(player);
        }

        push_children_reverse(node_bound, py, &mut stack);
    }
    Ok(())
}

// Pre-order tree traversal that skips hidden subtrees and dispatches draws.
fn traverse_draw(root: &Bound<'_, PyAny>) -> PyResult<()> {
    let py = root.py();
    let mut stack = vec![root.clone().unbind()];
    while let Some(node) = stack.pop() {
        let node = node.bind(py);
        if !node.getattr("visible")?.extract::<bool>()? {
            continue;
        }
        reset_draw_state();
        node.call_method0("on_draw")?;
        let node_bound = node.cast::<Node>()?;
        node_bound.borrow().draw_attached_primitive()?;
        push_children_reverse(node_bound, py, &mut stack);
    }
    Ok(())
}

// Map core nodes once per operation so results retain their Python wrapper identity.
type PyNodeIndex = HashMap<usize, Py<Node>>;

fn node_key(node: &pyxel::cube::RcNode) -> usize {
    Rc::as_ptr(node) as usize
}

fn build_py_node_index(root: &Bound<'_, PyAny>) -> PyResult<PyNodeIndex> {
    let py = root.py();
    let mut index = HashMap::new();
    let mut stack = vec![root.clone().unbind()];

    while let Some(node) = stack.pop() {
        let node_bound = node.bind(py).cast::<Node>()?;
        let node_ref = node_bound.borrow();
        let key = node_key(&node_ref.inner);
        for child in node_ref.children.borrow().iter().rev() {
            stack.push(child.clone_ref(py).into_any());
        }
        drop(node_ref);
        index.insert(key, node_bound.clone().unbind());
    }
    Ok(index)
}

fn find_indexed_py_node(
    index: &PyNodeIndex,
    target: &pyxel::cube::RcNode,
    py: Python<'_>,
) -> Option<Py<Node>> {
    index.get(&node_key(target)).map(|node| node.clone_ref(py))
}

fn wrap_raycast_hit(
    index: &PyNodeIndex,
    py: Python<'_>,
    info: pyxel::cube::scene::RaycastHitInfo,
) -> RaycastHit {
    let py_node = find_indexed_py_node(index, &info.node, py);
    let rch = pyxel::cube::RaycastHit::new();
    {
        let mut r = rc_mut!(&rch);
        r.node = Some(info.node);
        *rc_mut!(&r.point) = info.point;
        *rc_mut!(&r.normal) = info.normal;
        r.distance = info.distance;
    }
    match py_node {
        Some(p) => RaycastHit::wrap_with_py_node(rch, p),
        None => RaycastHit::wrap(rch),
    }
}

fn wrap_node_results(
    index: &PyNodeIndex,
    py: Python<'_>,
    inner: &[pyxel::cube::RcNode],
) -> Vec<Py<Node>> {
    let mut out: Vec<Py<Node>> = Vec::with_capacity(inner.len());
    for rc in inner {
        if let Some(py_node) = find_indexed_py_node(index, rc, py) {
            out.push(py_node);
        }
    }
    out
}

pub fn add_node_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Node>()?;
    Ok(())
}
