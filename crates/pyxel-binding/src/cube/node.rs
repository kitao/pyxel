use std::cell::RefCell;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyxel::cube::draw::DrawState;
use pyxel::cube::scene::with_draw_context;
use pyxel::cube::Node as InnerNode;

use super::collider::Collider;
use super::contact::Contact;
use super::mat4::Mat4;
use super::shading::Shading;
use super::vec3::Vec3;

type Uvs = ((f32, f32), (f32, f32), (f32, f32), (f32, f32));

// Python-facing Node wrapper. Holds the core hierarchy state (RcNode) plus
// a list of `Py<Node>` for the children so that user-defined subclasses
// (`class Player(Node):`) keep their identity through the scene tree —
// returning a fresh `Node::wrap(...)` from `children` would strip the
// Python override and silence on_update / on_draw dispatch.

#[pyclass(unsendable, from_py_object, subclass)]
pub struct Node {
    pub(crate) inner: pyxel::cube::RcNode,
    children: RefCell<Vec<Py<Node>>>,
    // Strong ref back to the parent wrapper; the cyclic ref is resolved
    // by __traverse__ / __clear__ so Python's gc can collect detached
    // subtrees.
    parent: RefCell<Option<Py<Node>>>,
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
            parent: RefCell::new(self.parent.borrow().as_ref().map(|p| p.clone_ref(py))),
        })
        .expect("Node clone requires the Python GIL to be attached")
    }
}

impl Node {
    pub fn wrap(inner: pyxel::cube::RcNode) -> Self {
        Self {
            inner,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
        }
    }

    // Detach this node from its Python-side parent wrapper. Used by
    // Scene.update step 8 to finalize a destroyed node's removal.
    pub(crate) fn detach_from_parent_py(&self, py: Python<'_>) {
        let parent = self.parent.borrow_mut().take();
        if let Some(parent_py) = parent {
            let pb = parent_py.bind(py).borrow();
            pb.children
                .borrow_mut()
                .retain(|c| !std::rc::Rc::ptr_eq(&c.bind(py).borrow().inner, &self.inner));
        }
    }

    #[allow(dead_code)]
    pub(crate) fn inner_ref(&self) -> &pyxel::cube::Node {
        rc_ref!(self.inner)
    }

    #[allow(clippy::mut_from_ref)]
    pub(crate) fn inner_mut(&self) -> &mut pyxel::cube::Node {
        rc_mut!(self.inner)
    }

    fn world_mat(&self) -> pyxel::cube::Mat4 {
        let world_rc = InnerNode::world_transform(&self.inner);
        *rc_ref!(&world_rc)
    }

    fn world_mat_compose(&self, local: pyxel::cube::Mat4) -> pyxel::cube::Mat4 {
        let world_rc = InnerNode::world_transform(&self.inner);
        let composed = rc_ref!(&world_rc).mul_mat(&local);
        *rc_ref!(&composed)
    }

    // Resolve the scene-wide cascade `shading`. Returns owned RcShading
    // so callers can borrow it through `rc_ref!` without conflicting
    // with the immutable borrow on `self.inner`.
    fn resolve_shading(&self) -> Option<pyxel::cube::RcShading> {
        InnerNode::effective_shading(&self.inner)
    }

    fn with_state_from_ctx(
        &self,
        billboard: i32,
        f: impl FnOnce(&mut pyxel::cube::scene::DrawContext, pyxel::cube::draw::DrawState),
    ) {
        // Resolve the per-Node cascading shading once. The rasterizer will
        // only consult it when ctx.shaded is true.
        let shading_rc = self.resolve_shading();
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
                shading: shading_ref,
            };
            f(ctx, state);
        });
    }

    fn collect_by_name(node: &Py<Node>, py: Python<'_>, name: &str, out: &mut Vec<Py<Node>>) {
        let bound = node.bind(py);
        let n = bound.borrow();
        if n.inner_ref().name == name {
            out.push(node.clone_ref(py));
        }
        let children: Vec<Py<Node>> = n
            .children
            .borrow()
            .iter()
            .map(|c| c.clone_ref(py))
            .collect();
        drop(n);
        for child in &children {
            Self::collect_by_name(child, py, name, out);
        }
    }

    fn collect_by_tags(node: &Py<Node>, py: Python<'_>, tags: &[String], out: &mut Vec<Py<Node>>) {
        let bound = node.bind(py);
        let n = bound.borrow();
        let node_tags = n.inner_ref().tags.clone();
        if tags.iter().any(|t| node_tags.iter().any(|nt| nt == t)) {
            out.push(node.clone_ref(py));
        }
        let children: Vec<Py<Node>> = n
            .children
            .borrow()
            .iter()
            .map(|c| c.clone_ref(py))
            .collect();
        drop(n);
        for child in &children {
            Self::collect_by_tags(child, py, tags, out);
        }
    }
}

#[pymethods]
impl Node {
    #[classattr]
    const BILLBOARD_OFF: i32 = pyxel::cube::draw::BILLBOARD_OFF;
    #[classattr]
    const BILLBOARD_ON: i32 = pyxel::cube::draw::BILLBOARD_ON;
    #[classattr]
    const BILLBOARD_FIXED_Y: i32 = pyxel::cube::draw::BILLBOARD_FIXED_Y;

    #[new]
    #[pyo3(signature = (*_args, **_kwargs))]
    fn new(_args: &Bound<'_, PyTuple>, _kwargs: Option<&Bound<'_, PyDict>>) -> Self {
        Self::wrap(InnerNode::new())
    }

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

    // Scene-wide shading cascade. None inherits from the closest non-None
    // ancestor; Scene seeds a default Shading at construction.
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
    fn parent(&self, py: Python<'_>) -> Option<Py<Node>> {
        self.parent.borrow().as_ref().map(|p| p.clone_ref(py))
    }

    #[getter]
    fn scene(slf: PyRef<'_, Node>, py: Python<'_>) -> PyResult<Option<Py<super::scene::Scene>>> {
        // Walk up via cached parent until a Scene wrapper is reached.
        let mut current: Py<Node> = slf.into_pyobject(py)?.unbind();
        loop {
            let bound = current.bind(py).clone();
            if let Ok(scene_bound) = bound.cast::<super::scene::Scene>() {
                return Ok(Some(scene_bound.clone().unbind()));
            }
            let parent_opt = bound
                .borrow()
                .parent
                .borrow()
                .as_ref()
                .map(|p| p.clone_ref(py));
            match parent_opt {
                Some(p) => current = p,
                None => return Ok(None),
            }
        }
    }

    #[getter]
    fn tags(&self) -> Vec<String> {
        self.inner_ref().tags.clone()
    }

    #[setter]
    fn set_tags(&self, v: Vec<String>) {
        self.inner_mut().tags = v;
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
    fn children<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyTuple>> {
        let core_children = InnerNode::children(&self.inner);
        let mut cache = self.children.borrow_mut();
        cache.retain(|cached| {
            let cached_inner = cached.bind(py).borrow().inner.clone();
            core_children
                .iter()
                .any(|c| std::rc::Rc::ptr_eq(&cached_inner, c))
        });
        let mut items: Vec<Py<Node>> = Vec::with_capacity(core_children.len());
        for c_inner in &core_children {
            if let Some(cached) = cache
                .iter()
                .find(|cached| std::rc::Rc::ptr_eq(&cached.bind(py).borrow().inner, c_inner))
            {
                items.push(cached.clone_ref(py));
            }
        }
        pyo3::types::PyTuple::new(py, items)
    }

    fn world_transform(&self) -> Mat4 {
        Mat4::wrap(InnerNode::world_transform(&self.inner))
    }

    fn find_by_name(slf: PyRef<'_, Node>, py: Python<'_>, name: &str) -> PyResult<Vec<Py<Node>>> {
        let self_py: Py<Node> = slf.into_pyobject(py)?.unbind();
        let mut out: Vec<Py<Node>> = Vec::new();
        Self::collect_by_name(&self_py, py, name, &mut out);
        Ok(out)
    }

    fn find_by_tags(
        slf: PyRef<'_, Node>,
        py: Python<'_>,
        tags: &Bound<'_, pyo3::types::PyAny>,
    ) -> PyResult<Vec<Py<Node>>> {
        let tag_list: Vec<String> = if let Ok(s) = tags.extract::<String>() {
            vec![s]
        } else if let Ok(v) = tags.extract::<Vec<String>>() {
            v
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "tags must be str or list[str]",
            ));
        };
        let self_py: Py<Node> = slf.into_pyobject(py)?.unbind();
        let mut out: Vec<Py<Node>> = Vec::new();
        Self::collect_by_tags(&self_py, py, &tag_list, &mut out);
        Ok(out)
    }

    fn add_child(slf: Bound<'_, Self>, py: Python<'_>, child: Py<Node>) {
        let child_inner = child.bind(py).borrow().inner.clone();
        InnerNode::add_child(&slf.borrow().inner, &child_inner);
        // Detach from any previous parent's wrapper cache.
        let prev_parent: Option<Py<Node>> = child.bind(py).borrow().parent.borrow_mut().take();
        if let Some(prev) = prev_parent {
            let pb = prev.bind(py).borrow();
            pb.children
                .borrow_mut()
                .retain(|c| !std::rc::Rc::ptr_eq(&c.bind(py).borrow().inner, &child_inner));
        }
        let self_py: Py<Node> = slf.clone().unbind();
        *child.bind(py).borrow().parent.borrow_mut() = Some(self_py);
        slf.borrow().children.borrow_mut().push(child);
    }

    fn remove_child(slf: Bound<'_, Self>, py: Python<'_>, child: Py<Node>) {
        let child_inner = child.bind(py).borrow().inner.clone();
        InnerNode::remove_child(&slf.borrow().inner, &child_inner);
        slf.borrow()
            .children
            .borrow_mut()
            .retain(|c| !std::rc::Rc::ptr_eq(&c.bind(py).borrow().inner, &child_inner));
        *child.bind(py).borrow().parent.borrow_mut() = None;
    }

    // Flag-only destroy (cube-design.md § 16 step 8). Scene.update
    // walks the tree post-order, fires on_destroy, and detaches the
    // flagged nodes at the end of the frame. Parent / child links
    // stay intact for the rest of the current frame so traversal
    // remains safe.
    fn destroy(slf: PyRef<'_, Self>) {
        InnerNode::destroy(&slf.inner);
    }

    #[getter]
    fn destroyed(&self) -> bool {
        self.inner_ref().destroyed
    }

    #[allow(clippy::unused_self)]
    fn on_update(&self) {}

    #[allow(clippy::unused_self)]
    fn on_draw(&self) {}

    #[allow(clippy::unused_self, unused_variables)]
    fn on_collide(&self, other: PyRef<'_, Node>, contact: PyRef<'_, Contact>) {}

    #[allow(clippy::unused_self)]
    fn on_destroy(&self) {}

    // State setters for the per-on_draw draw modifiers. Called from inside
    // on_draw, they mutate the active draw context; subsequent primitives
    // in the same on_draw consult these values. The values reset to
    // defaults at the entry of every Node's on_draw (see scene.rs
    // reset_draw_state). Outside a draw scope, these are no-ops.
    fn dither(&self, alpha: f32) {
        with_draw_context(|ctx| {
            ctx.dither_alpha = alpha.clamp(0.0, 1.0);
        });
    }

    fn depth_test(&self, on: bool) {
        with_draw_context(|ctx| {
            ctx.depth_test = on;
        });
    }

    fn depth_write(&self, on: bool) {
        with_draw_context(|ctx| {
            ctx.depth_write = on;
        });
    }

    fn shaded(&self, on: bool) {
        with_draw_context(|ctx| {
            ctx.shaded = on;
        });
    }

    #[pyo3(signature = (pos, col))]
    fn pset(
        &self,
        pos: PyRef<'_, Vec3>,
        col: i32,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::pset(ctx, &world_mat, &local, col, state);
        });
    }

    #[pyo3(signature = (p1, p2, col, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn line(
        &self,
        p1: PyRef<'_, Vec3>,
        p2: PyRef<'_, Vec3>,
        col: i32,
        billboard: i32,
    ) {
        let world_mat = self.world_mat();
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::line(ctx, &world_mat, &v1, &v2, col, state);
        });
    }

    #[pyo3(signature = (p1, p2, p3, col, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn tri(
        &self,
        p1: PyRef<'_, Vec3>,
        p2: PyRef<'_, Vec3>,
        p3: PyRef<'_, Vec3>,
        col: i32,
        billboard: i32,
    ) {
        let world_mat = self.world_mat();
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        let v3 = *p3.inner_ref();
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::tri(ctx, &world_mat, &v1, &v2, &v3, col, state);
        });
    }

    #[pyo3(signature = (p1, p2, p3, col, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn trib(
        &self,
        p1: PyRef<'_, Vec3>,
        p2: PyRef<'_, Vec3>,
        p3: PyRef<'_, Vec3>,
        col: i32,
        billboard: i32,
    ) {
        let world_mat = self.world_mat();
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        let v3 = *p3.inner_ref();
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::trib(ctx, &world_mat, &v1, &v2, &v3, col, state);
        });
    }

    #[pyo3(signature = (pos, r, col))]
    fn circ(
        &self,
        pos: PyRef<'_, Vec3>,
        r: f32,
        col: i32,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::circ(ctx, &world_mat, &local, r, col, state);
        });
    }

    #[pyo3(signature = (pos, r, col))]
    fn circb(
        &self,
        pos: PyRef<'_, Vec3>,
        r: f32,
        col: i32,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::circb(ctx, &world_mat, &local, r, col, state);
        });
    }

    #[pyo3(signature = (pos, r, col))]
    fn sphere(
        &self,
        pos: PyRef<'_, Vec3>,
        r: f32,
        col: i32,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::sphere(ctx, &world_mat, &local, r, col, state);
        });
    }

    #[pyo3(signature = (pos, r, col))]
    fn sphereb(
        &self,
        pos: PyRef<'_, Vec3>,
        r: f32,
        col: i32,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            pyxel::cube::draw::sphereb(ctx, &world_mat, &local, r, col, state);
        });
    }

    #[pyo3(signature = (mat, w, h, col, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn rect(
        &self,
        mat: PyRef<'_, Mat4>,
        w: f32,
        h: f32,
        col: i32,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::rect(ctx, &world_mat, w, h, col, state);
        });
    }

    #[pyo3(signature = (mat, w, h, col, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn rectb(
        &self,
        mat: PyRef<'_, Mat4>,
        w: f32,
        h: f32,
        col: i32,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::rectb(ctx, &world_mat, w, h, col, state);
        });
    }

    #[pyo3(signature = (mat, w, h, col, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn elli(
        &self,
        mat: PyRef<'_, Mat4>,
        w: f32,
        h: f32,
        col: i32,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::elli(ctx, &world_mat, w, h, col, state);
        });
    }

    #[pyo3(signature = (mat, w, h, col, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn ellib(
        &self,
        mat: PyRef<'_, Mat4>,
        w: f32,
        h: f32,
        col: i32,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::ellib(ctx, &world_mat, w, h, col, state);
        });
    }

    #[pyo3(signature = (mat, size, col, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn r#box(
        &self,
        mat: PyRef<'_, Mat4>,
        size: PyRef<'_, Vec3>,
        col: i32,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let size_v = *size.inner_ref();
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::box_solid(ctx, &world_mat, &size_v, col, state);
        });
    }

    #[pyo3(signature = (mat, size, col, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn boxb(
        &self,
        mat: PyRef<'_, Mat4>,
        size: PyRef<'_, Vec3>,
        col: i32,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let size_v = *size.inner_ref();
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::boxb(ctx, &world_mat, &size_v, col, state);
        });
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
        self.with_state_from_ctx(pyxel::cube::draw::BILLBOARD_OFF, |ctx, state| {
            let font_ref: Option<&mut pyxel::Font> = font_rc.as_ref().map(|f| rc_mut!(f));
            pyxel::cube::draw::text(ctx, &world_mat, &pos_v, s, col, font_ref, state);
        });
    }

    #[pyo3(signature = (pos, img, uvs, w, h, *, colkey=None, angle=0.0))]
    fn sprite(
        &self,
        pos: PyRef<'_, Vec3>,
        img: PyRef<'_, crate::image_wrapper::Image>,
        uvs: Uvs,
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

    #[pyo3(signature = (mat, img, uvs, w, h, *, colkey=None, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn plane(
        &self,
        mat: PyRef<'_, Mat4>,
        img: PyRef<'_, crate::image_wrapper::Image>,
        uvs: Uvs,
        w: f32,
        h: f32,
        colkey: Option<i32>,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let img_inner = img.inner.clone();
        self.with_state_from_ctx(billboard, |ctx, state| {
            pyxel::cube::draw::plane(ctx, &world_mat, &img_inner, uvs, w, h, colkey, state);
        });
    }

    #[pyo3(signature = (mat, mesh_asset, *, billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn mesh(
        &self,
        mat: PyRef<'_, Mat4>,
        mesh_asset: PyRef<'_, super::mesh::Mesh>,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let mesh_inner = mesh_asset.inner.clone();
        self.with_state_from_ctx(billboard, |ctx, state| {
            let m = rc_ref!(&mesh_inner);
            pyxel::cube::draw::mesh(ctx, &world_mat, m, state);
        });
    }

    #[pyo3(signature = (mat, geom, *, col_img=None, colkey=None,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    fn prim(
        &self,
        mat: PyRef<'_, Mat4>,
        geom: PyRef<'_, super::geometry::Geometry>,
        col_img: Option<&Bound<'_, PyAny>>,
        colkey: Option<i32>,
        billboard: i32,
    ) -> PyResult<()> {
        use pyo3::exceptions::{PyTypeError, PyValueError};

        let world_mat = self.world_mat_compose(*mat.inner_ref());

        let (positions_data, indices_data, normals_data, uvs_data, prim_mode, cull_mode) = {
            let g = rc_ref!(&geom.inner);
            (
                g.positions.clone(),
                g.indices.clone(),
                g.normals.clone(),
                g.uvs.clone(),
                g.prim,
                g.cull,
            )
        };

        let (col_flat, col_image) = match col_img {
            Some(c) => {
                if let Ok(i) = c.extract::<i32>() {
                    (i, None)
                } else if let Ok(img_ref) = c.cast::<crate::image_wrapper::Image>() {
                    (0, Some(img_ref.borrow().inner.clone()))
                } else {
                    return Err(PyTypeError::new_err("col_img must be int or Image"));
                }
            }
            None => (7, None),
        };

        // Route the draw through with_state_from_ctx so the shared state
        // (shaded / dither_alpha / depth_test / depth_write) is read from
        // the active DrawContext like every other primitive. The closure
        // returns (), so we capture the rasterizer result in an outer
        // variable and propagate any error after the call returns.
        let mut inner_result: Option<Result<(), &str>> = None;
        self.with_state_from_ctx(billboard, |ctx, state| {
            inner_result = Some(pyxel::cube::draw::prim(
                ctx,
                &world_mat,
                prim_mode,
                cull_mode,
                &positions_data,
                indices_data.as_deref(),
                normals_data.as_deref(),
                uvs_data.as_deref(),
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

    fn __repr__(&self) -> String {
        let n = self.inner_ref();
        format!(
            "Node(name={:?}, children={})",
            n.name,
            self.children.borrow().len()
        )
    }
}

pub fn add_node_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Node>()?;
    Ok(())
}
