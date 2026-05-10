use std::cell::RefCell;

use pyo3::prelude::*;
use pyxel::cube::draw::DrawState;
use pyxel::cube::scene::with_draw_context;
use pyxel::cube::Node as InnerNode;

use super::camera::Camera;
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
        })
        .expect("Node clone requires the Python GIL to be attached")
    }
}

impl Node {
    pub fn wrap(inner: pyxel::cube::RcNode) -> Self {
        Self {
            inner,
            children: RefCell::new(Vec::new()),
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

    #[allow(clippy::too_many_arguments)]
    fn with_state(
        &self,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
        f: impl FnOnce(&mut pyxel::cube::scene::DrawContext, pyxel::cube::draw::DrawState),
    ) {
        let shading = if shaded { self.resolve_shading() } else { None };
        let shading_ref = shading
            .as_ref()
            .map(|s: &pyxel::cube::RcShading| rc_ref!(s));
        let state = DrawState {
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            shading: shading_ref,
        };
        with_draw_context(|ctx| f(ctx, state));
    }
}

#[pymethods]
impl Node {
    #[classattr]
    const PRIM_POINTS: i32 = pyxel::cube::draw::PRIM_POINTS;
    #[classattr]
    const PRIM_LINES: i32 = pyxel::cube::draw::PRIM_LINES;
    #[classattr]
    const PRIM_TRIANGLES: i32 = pyxel::cube::draw::PRIM_TRIANGLES;

    #[classattr]
    const BILLBOARD_OFF: i32 = pyxel::cube::draw::BILLBOARD_OFF;
    #[classattr]
    const BILLBOARD_ON: i32 = pyxel::cube::draw::BILLBOARD_ON;
    #[classattr]
    const BILLBOARD_FIXED_Y: i32 = pyxel::cube::draw::BILLBOARD_FIXED_Y;

    #[new]
    fn new() -> Self {
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
    #[allow(clippy::unused_self)]
    fn parent(&self) -> Option<Node> {
        None
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

    #[getter]
    #[allow(clippy::unused_self)]
    fn camera(&self) -> PyResult<Camera> {
        let camera_rc = pyxel::cube::scene::with_draw_context(|ctx| ctx.camera.clone());
        match camera_rc {
            Some(rc) => Ok(Camera::wrap(rc)),
            None => Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Node.camera is only valid inside on_draw",
            )),
        }
    }

    fn world_transform(&self) -> Mat4 {
        Mat4::wrap(InnerNode::world_transform(&self.inner))
    }

    fn find(slf: PyRef<'_, Node>, py: Python<'_>, name: &str) -> Option<Py<Node>> {
        if slf.inner_ref().name == name {
            return Some(slf.into_pyobject(py).ok()?.unbind());
        }
        for child in slf.children.borrow().iter() {
            let bound = child.bind(py);
            let inner_ref: PyRef<'_, Node> = bound.borrow();
            if let Some(found) = Node::find(inner_ref, py, name) {
                return Some(found);
            }
        }
        None
    }

    fn add_child(&self, py: Python<'_>, child: Py<Node>) {
        let child_inner = {
            let bound = child.bind(py).borrow();
            bound.inner.clone()
        };
        InnerNode::add_child(&self.inner, &child_inner);
        self.children.borrow_mut().push(child);
    }

    fn remove_child(&self, py: Python<'_>, child: Py<Node>) {
        let child_inner = {
            let bound = child.bind(py).borrow();
            bound.inner.clone()
        };
        InnerNode::remove_child(&self.inner, &child_inner);
        self.children.borrow_mut().retain(|c| {
            let c_inner = c.bind(py).borrow().inner.clone();
            !std::rc::Rc::ptr_eq(&c_inner, &child_inner)
        });
    }

    fn destroy(&self) {
        InnerNode::destroy(&self.inner);
    }

    #[allow(clippy::unused_self)]
    fn on_update(&self) {}

    #[allow(clippy::unused_self)]
    fn on_draw(&self) {}

    #[allow(clippy::unused_self, unused_variables)]
    #[pyo3(signature = (other, contact=None))]
    fn on_collide(&self, other: PyRef<'_, Node>, contact: Option<PyRef<'_, Contact>>) {}

    #[allow(clippy::unused_self)]
    fn on_destroy(&self) {}

    #[pyo3(signature = (pos, col, *, dither_alpha=1.0, depth_test=true, depth_write=true))]
    fn pset(
        &self,
        pos: PyRef<'_, Vec3>,
        col: i32,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state(
            false,
            dither_alpha,
            depth_test,
            depth_write,
            pyxel::cube::draw::BILLBOARD_OFF,
            |ctx, state| {
                pyxel::cube::draw::pset(ctx, &world_mat, &local, col, state);
            },
        );
    }

    #[pyo3(signature = (p1, p2, col, *, dither_alpha=1.0, depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn line(
        &self,
        p1: PyRef<'_, Vec3>,
        p2: PyRef<'_, Vec3>,
        col: i32,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat();
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        self.with_state(
            false,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                pyxel::cube::draw::line(ctx, &world_mat, &v1, &v2, col, state);
            },
        );
    }

    #[pyo3(signature = (p1, p2, p3, col, *, shaded=true, dither_alpha=1.0,
                        depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn tri(
        &self,
        p1: PyRef<'_, Vec3>,
        p2: PyRef<'_, Vec3>,
        p3: PyRef<'_, Vec3>,
        col: i32,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat();
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        let v3 = *p3.inner_ref();
        self.with_state(
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                pyxel::cube::draw::tri(ctx, &world_mat, &v1, &v2, &v3, col, state);
            },
        );
    }

    #[pyo3(signature = (p1, p2, p3, col, *, dither_alpha=1.0,
                        depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn trib(
        &self,
        p1: PyRef<'_, Vec3>,
        p2: PyRef<'_, Vec3>,
        p3: PyRef<'_, Vec3>,
        col: i32,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat();
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        let v3 = *p3.inner_ref();
        self.with_state(
            false,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                pyxel::cube::draw::trib(ctx, &world_mat, &v1, &v2, &v3, col, state);
            },
        );
    }

    #[pyo3(signature = (pos, r, col, *, dither_alpha=1.0, depth_test=true, depth_write=true))]
    fn circ(
        &self,
        pos: PyRef<'_, Vec3>,
        r: f32,
        col: i32,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state(
            false,
            dither_alpha,
            depth_test,
            depth_write,
            pyxel::cube::draw::BILLBOARD_OFF,
            |ctx, state| {
                pyxel::cube::draw::circ(ctx, &world_mat, &local, r, col, state);
            },
        );
    }

    #[pyo3(signature = (pos, r, col, *, dither_alpha=1.0, depth_test=true, depth_write=true))]
    fn circb(
        &self,
        pos: PyRef<'_, Vec3>,
        r: f32,
        col: i32,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state(
            false,
            dither_alpha,
            depth_test,
            depth_write,
            pyxel::cube::draw::BILLBOARD_OFF,
            |ctx, state| {
                pyxel::cube::draw::circb(ctx, &world_mat, &local, r, col, state);
            },
        );
    }

    #[pyo3(signature = (pos, r, col, *, shaded=true, dither_alpha=1.0,
                        depth_test=true, depth_write=true))]
    #[allow(clippy::too_many_arguments)]
    fn sphere(
        &self,
        pos: PyRef<'_, Vec3>,
        r: f32,
        col: i32,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state(
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            pyxel::cube::draw::BILLBOARD_OFF,
            |ctx, state| {
                pyxel::cube::draw::sphere(ctx, &world_mat, &local, r, col, state);
            },
        );
    }

    #[pyo3(signature = (pos, r, col, *, dither_alpha=1.0, depth_test=true, depth_write=true))]
    fn sphereb(
        &self,
        pos: PyRef<'_, Vec3>,
        r: f32,
        col: i32,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        self.with_state(
            false,
            dither_alpha,
            depth_test,
            depth_write,
            pyxel::cube::draw::BILLBOARD_OFF,
            |ctx, state| {
                pyxel::cube::draw::sphereb(ctx, &world_mat, &local, r, col, state);
            },
        );
    }

    #[pyo3(signature = (mat, w, h, col, *, shaded=true, dither_alpha=1.0,
                        depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn rect(
        &self,
        mat: PyRef<'_, Mat4>,
        w: f32,
        h: f32,
        col: i32,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state(
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                pyxel::cube::draw::rect(ctx, &world_mat, w, h, col, state);
            },
        );
    }

    #[pyo3(signature = (mat, w, h, col, *, dither_alpha=1.0,
                        depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn rectb(
        &self,
        mat: PyRef<'_, Mat4>,
        w: f32,
        h: f32,
        col: i32,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state(
            false,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                pyxel::cube::draw::rectb(ctx, &world_mat, w, h, col, state);
            },
        );
    }

    #[pyo3(signature = (mat, w, h, col, *, shaded=true, dither_alpha=1.0,
                        depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn elli(
        &self,
        mat: PyRef<'_, Mat4>,
        w: f32,
        h: f32,
        col: i32,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state(
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                pyxel::cube::draw::elli(ctx, &world_mat, w, h, col, state);
            },
        );
    }

    #[pyo3(signature = (mat, w, h, col, *, dither_alpha=1.0,
                        depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn ellib(
        &self,
        mat: PyRef<'_, Mat4>,
        w: f32,
        h: f32,
        col: i32,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        self.with_state(
            false,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                pyxel::cube::draw::ellib(ctx, &world_mat, w, h, col, state);
            },
        );
    }

    #[pyo3(signature = (mat, size, col, *, shaded=true, dither_alpha=1.0,
                        depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn r#box(
        &self,
        mat: PyRef<'_, Mat4>,
        size: PyRef<'_, Vec3>,
        col: i32,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let size_v = *size.inner_ref();
        self.with_state(
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                pyxel::cube::draw::box_solid(ctx, &world_mat, &size_v, col, state);
            },
        );
    }

    #[pyo3(signature = (mat, size, col, *, dither_alpha=1.0,
                        depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn boxb(
        &self,
        mat: PyRef<'_, Mat4>,
        size: PyRef<'_, Vec3>,
        col: i32,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let size_v = *size.inner_ref();
        self.with_state(
            false,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                pyxel::cube::draw::boxb(ctx, &world_mat, &size_v, col, state);
            },
        );
    }

    #[pyo3(signature = (pos, s, col, *, font=None, dither_alpha=1.0,
                        depth_test=true, depth_write=true))]
    #[allow(clippy::too_many_arguments)]
    fn text(
        &self,
        pos: PyRef<'_, Vec3>,
        s: &str,
        col: i32,
        font: Option<PyRef<'_, crate::font_wrapper::Font>>,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
    ) {
        let world_mat = self.world_mat();
        let pos_v = *pos.inner_ref();
        let state = DrawState {
            shaded: false,
            dither_alpha,
            depth_test,
            depth_write,
            billboard: pyxel::cube::draw::BILLBOARD_OFF,
            shading: None,
        };
        let font_rc = font.as_ref().map(|f| f.inner.clone());
        with_draw_context(|ctx| {
            let font_ref: Option<&mut pyxel::Font> = font_rc.as_ref().map(|f| rc_mut!(f));
            pyxel::cube::draw::text(ctx, &world_mat, &pos_v, s, col, font_ref, state);
        });
    }

    #[pyo3(signature = (pos, img, uvs, w, h, *, colkey=None, angle=0.0,
                        shaded=false, dither_alpha=1.0,
                        depth_test=true, depth_write=true))]
    #[allow(clippy::too_many_arguments)]
    fn sprite(
        &self,
        pos: PyRef<'_, Vec3>,
        img: PyRef<'_, crate::image_wrapper::Image>,
        uvs: Uvs,
        w: f32,
        h: f32,
        colkey: Option<i32>,
        angle: f32,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
    ) {
        let world_mat = self.world_mat();
        let local = *pos.inner_ref();
        let img_inner = img.inner.clone();
        let shading = if shaded { self.resolve_shading() } else { None };
        let shading_ref = shading.as_ref().map(|s| rc_ref!(s));
        let state = DrawState {
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard: pyxel::cube::draw::BILLBOARD_ON,
            shading: shading_ref,
        };
        with_draw_context(|ctx| {
            pyxel::cube::draw::sprite(
                ctx, &world_mat, &local, &img_inner, uvs, w, h, colkey, angle, state,
            );
        });
    }

    #[pyo3(signature = (mat, img, uvs, w, h, *, colkey=None, shaded=true,
                        dither_alpha=1.0, depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn plane(
        &self,
        mat: PyRef<'_, Mat4>,
        img: PyRef<'_, crate::image_wrapper::Image>,
        uvs: Uvs,
        w: f32,
        h: f32,
        colkey: Option<i32>,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let img_inner = img.inner.clone();
        let shading = if shaded { self.resolve_shading() } else { None };
        let shading_ref = shading.as_ref().map(|s| rc_ref!(s));
        let state = DrawState {
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            shading: shading_ref,
        };
        with_draw_context(|ctx| {
            pyxel::cube::draw::plane(ctx, &world_mat, &img_inner, uvs, w, h, colkey, state);
        });
    }

    #[pyo3(signature = (mat, mesh_asset, *, col=7, shaded=true,
                        dither_alpha=1.0, depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn mesh(
        &self,
        mat: PyRef<'_, Mat4>,
        mesh_asset: PyRef<'_, super::mesh::Mesh>,
        col: i32,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) -> PyResult<()> {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let mesh_inner = mesh_asset.inner.clone();
        let is_empty = {
            let m = rc_ref!(&mesh_inner);
            rc_ref!(&m.positions).size() == 0
        };
        if is_empty {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Mesh.positions is empty; assign a non-empty FloatBuffer before drawing",
            ));
        }
        self.with_state(
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            |ctx, state| {
                let m = rc_ref!(&mesh_inner);
                pyxel::cube::draw::mesh(ctx, &world_mat, m, col, state);
            },
        );
        Ok(())
    }

    #[pyo3(signature = (mat, mode, positions, *, indices=None, normals=None,
                        uvs=None, first=0, count=None, col=None, colkey=None,
                        shaded=true, dither_alpha=1.0,
                        depth_test=true, depth_write=true,
                        billboard=pyxel::cube::draw::BILLBOARD_OFF))]
    #[allow(clippy::too_many_arguments)]
    fn prim(
        &self,
        mat: PyRef<'_, Mat4>,
        mode: i32,
        positions: PyRef<'_, super::float_buffer::FloatBuffer>,
        indices: Option<PyRef<'_, super::int_buffer::IntBuffer>>,
        normals: Option<PyRef<'_, super::float_buffer::FloatBuffer>>,
        uvs: Option<PyRef<'_, super::float_buffer::FloatBuffer>>,
        first: usize,
        count: Option<usize>,
        col: Option<&Bound<'_, PyAny>>,
        colkey: Option<i32>,
        shaded: bool,
        dither_alpha: f32,
        depth_test: bool,
        depth_write: bool,
        billboard: i32,
    ) -> PyResult<()> {
        use pyo3::exceptions::{PyTypeError, PyValueError};

        let world_mat = self.world_mat_compose(*mat.inner_ref());

        let positions_data: Vec<f32> = positions.inner_ref().data().to_vec();
        let indices_data: Option<Vec<i32>> =
            indices.as_ref().map(|i| i.inner_ref().data().to_vec());
        let normals_data: Option<Vec<f32>> =
            normals.as_ref().map(|n| n.inner_ref().data().to_vec());
        let uvs_data: Option<Vec<f32>> = uvs.as_ref().map(|u| u.inner_ref().data().to_vec());

        let (col_flat, col_image) = match col {
            Some(c) => {
                if let Ok(i) = c.extract::<i32>() {
                    (i, None)
                } else if let Ok(img_ref) = c.cast::<crate::image_wrapper::Image>() {
                    (0, Some(img_ref.borrow().inner.clone()))
                } else {
                    return Err(PyTypeError::new_err("col must be int or Image"));
                }
            }
            None => (7, None),
        };

        let shading = if shaded { self.resolve_shading() } else { None };
        let shading_ref = shading.as_ref().map(|s| rc_ref!(s));
        let state = DrawState {
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            shading: shading_ref,
        };

        let result = with_draw_context(|ctx| {
            pyxel::cube::draw::prim(
                ctx,
                &world_mat,
                mode,
                &positions_data,
                indices_data.as_deref(),
                normals_data.as_deref(),
                uvs_data.as_deref(),
                first,
                count,
                col_flat,
                col_image.as_ref(),
                colkey,
                state,
            )
        });

        match result {
            Some(Err(msg)) => Err(PyValueError::new_err(msg)),
            Some(Ok(())) | None => Ok(()),
        }
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
