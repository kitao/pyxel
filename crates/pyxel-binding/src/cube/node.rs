use std::cell::RefCell;

use pyo3::prelude::*;
use pyxel::cube::draw::DrawState;
use pyxel::cube::scene::with_draw_context;
use pyxel::cube::Node as InnerNode;

use super::camera::Camera;
use super::collider::Collider;
use super::contact::Contact;
use super::light::Light;
use super::mat4::Mat4;
use super::shade_ramp::ShadeRamp;
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

    // World transform of this node (composition of all ancestor transforms).
    fn world_mat(&self) -> pyxel::cube::Mat4 {
        let world_rc = InnerNode::world_transform(&self.inner);
        *rc_ref!(&world_rc)
    }

    // World-space matrix for a node-local Mat4 (compose ancestor world
    // transform with the local placement).
    fn world_mat_compose(&self, local: pyxel::cube::Mat4) -> pyxel::cube::Mat4 {
        let world_rc = InnerNode::world_transform(&self.inner);
        let composed = rc_ref!(&world_rc).mul_mat(&local);
        *rc_ref!(&composed)
    }

    // Resolve scene-wide cascade values (light / shade_ramp). Returns
    // owned RcLight / RcShadeRamp clones so callers can borrow them
    // through `rc_ref!` without conflicting with the immutable borrow
    // on `self.inner`.
    fn resolve_lighting(
        &self,
    ) -> (
        Option<pyxel::cube::RcLight>,
        Option<pyxel::cube::RcShadeRamp>,
    ) {
        (
            InnerNode::effective_light(&self.inner),
            InnerNode::effective_shade_ramp(&self.inner),
        )
    }

    // Run `f` with a fully resolved DrawState. Each draw method shares
    // this pattern (lookup light/ramp, build state, call into draw),
    // so funneling it through a single helper keeps the per-method
    // boilerplate to one line.
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
        let (light, ramp) = if shaded {
            self.resolve_lighting()
        } else {
            (None, None)
        };
        let light_ref = light.as_ref().map(|l: &pyxel::cube::RcLight| rc_ref!(l));
        let ramp_ref = ramp.as_ref().map(|r: &pyxel::cube::RcShadeRamp| rc_ref!(r));
        let state = DrawState {
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            light: light_ref,
            ramp: ramp_ref,
        };
        with_draw_context(|ctx| f(ctx, state));
    }
}

#[pymethods]
impl Node {
    // Primitive mode constants for `prim` (OpenGL ordering: POINTS=0,
    // LINES=1, TRIANGLES=2; future LINE_STRIP / LINE_LOOP /
    // TRIANGLE_STRIP / TRIANGLE_FAN keep the relative position).

    #[classattr]
    const PRIM_POINTS: i32 = pyxel::cube::draw::PRIM_POINTS;
    #[classattr]
    const PRIM_LINES: i32 = pyxel::cube::draw::PRIM_LINES;
    #[classattr]
    const PRIM_TRIANGLES: i32 = pyxel::cube::draw::PRIM_TRIANGLES;

    // Billboard mode constants (mirror Godot BillboardMode).
    #[classattr]
    const BILLBOARD_OFF: i32 = pyxel::cube::draw::BILLBOARD_OFF;
    #[classattr]
    const BILLBOARD_ON: i32 = pyxel::cube::draw::BILLBOARD_ON;
    #[classattr]
    const BILLBOARD_FIXED_Y: i32 = pyxel::cube::draw::BILLBOARD_FIXED_Y;

    // Constructor

    #[new]
    fn new() -> Self {
        Self::wrap(InnerNode::new())
    }

    // Identification

    #[getter]
    fn name(&self) -> String {
        self.inner_ref().name.clone()
    }

    #[setter]
    fn set_name(&self, v: String) {
        self.inner_mut().name = v;
    }

    // Transform

    #[getter]
    fn transform(&self) -> Mat4 {
        Mat4::wrap(self.inner_ref().transform.clone())
    }

    #[setter]
    fn set_transform(&self, v: PyRef<'_, Mat4>) {
        self.inner_mut().transform = v.inner.clone();
    }

    // Cascade flags

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

    // Scene-wide lighting cascade (None inherits from the closest non-None
    // ancestor; Scene seeds defaults at construction).

    #[getter]
    fn light(&self) -> Option<Light> {
        self.inner_ref()
            .light
            .as_ref()
            .map(|l| Light::wrap(l.clone()))
    }

    #[setter]
    fn set_light(&self, v: Option<PyRef<'_, Light>>) {
        self.inner_mut().light = v.as_ref().map(|l| l.inner.clone());
    }

    #[getter]
    fn shade_ramp(&self) -> Option<ShadeRamp> {
        self.inner_ref()
            .shade_ramp
            .as_ref()
            .map(|r| ShadeRamp::wrap(r.clone()))
    }

    #[setter]
    fn set_shade_ramp(&self, v: Option<PyRef<'_, ShadeRamp>>) {
        self.inner_mut().shade_ramp = v.as_ref().map(|r| r.inner.clone());
    }

    // Collider slot — currently a placeholder; the collision pipeline
    // is deferred (cube-design.md § 15). Stored here so user code can
    // already round-trip `node.collider = Collider()` setups.

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

    // Hierarchy (read-only properties).
    // `parent` is intentionally None for now — adding parent references
    // would create a Python ref cycle (parent->child->parent) that the
    // GC cannot break without weakref plumbing. Tree traversal uses the
    // children list and does not need parent.

    #[getter]
    #[allow(clippy::unused_self)]
    fn parent(&self) -> Option<Node> {
        None
    }

    #[getter]
    fn children<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyTuple>> {
        // Reconcile binding-side cache with the core-side child list.
        // Core is the source of truth: re-parenting via `add_child` and
        // `destroy()` mutate the core list directly, so the binding
        // tuple is built from the core order and prunes any cached
        // entries that no longer correspond to a live child.
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

    // Active camera, valid only inside on_draw. Outside on_draw the
    // draw context is unset and accessing this getter raises so callers
    // notice the misuse instead of seeing stale data.
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

    // Methods

    fn world_transform(&self) -> Mat4 {
        Mat4::wrap(InnerNode::world_transform(&self.inner))
    }

    fn find(slf: PyRef<'_, Node>, py: Python<'_>, name: &str) -> Option<Py<Node>> {
        if slf.inner_ref().name == name {
            // Bind back to the running Py<Self> so Python override survives.
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

    // Lifecycle hooks (default no-op; user overrides in Python subclass).

    #[allow(clippy::unused_self)]
    fn on_update(&self) {}

    #[allow(clippy::unused_self)]
    fn on_draw(&self) {}

    // Collision hook — never invoked by the cube runtime today (the
    // collision pipeline is deferred; see cube-design.md § 15). The
    // signature is exposed so user subclasses can already define an
    // override that the future pipeline will call.
    #[allow(clippy::unused_self, unused_variables)]
    #[pyo3(signature = (other, contact=None))]
    fn on_collide(&self, other: PyRef<'_, Node>, contact: Option<PyRef<'_, Contact>>) {}

    #[allow(clippy::unused_self)]
    fn on_destroy(&self) {}

    // ===================================================================
    // Immediate-mode draw commands. Each method captures Python args,
    // resolves the world transform + scene-wide light/shade_ramp, builds
    // a DrawState carrying per-call modifiers, then delegates to
    // pyxel::cube::draw (single home for all draw logic).
    // ===================================================================

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

    // text: Vec3-anchored, screen-space glyph rendering. The 3D point is
    // projected and characters render in 2D pixels at the font's native
    // size — ancestor rotation / scale do not affect glyph layout
    // (cube-design.md § 12.5).
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
            light: None,
            ramp: None,
        };
        // Clone the Rc out so we can drop the PyRef before borrowing
        // `&mut Font`. Builtin (None) font case: just pass None — the
        // draw path resolves Pyxel's 4x6 glyph data directly.
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
        let (light, ramp) = if shaded {
            self.resolve_lighting()
        } else {
            (None, None)
        };
        let light_ref = light.as_ref().map(|l: &pyxel::cube::RcLight| rc_ref!(l));
        let ramp_ref = ramp.as_ref().map(|r: &pyxel::cube::RcShadeRamp| rc_ref!(r));
        let state = DrawState {
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard: pyxel::cube::draw::BILLBOARD_ON,
            light: light_ref,
            ramp: ramp_ref,
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
        let (light, ramp) = if shaded {
            self.resolve_lighting()
        } else {
            (None, None)
        };
        let light_ref = light.as_ref().map(|l: &pyxel::cube::RcLight| rc_ref!(l));
        let ramp_ref = ramp.as_ref().map(|r: &pyxel::cube::RcShadeRamp| rc_ref!(r));
        let state = DrawState {
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            light: light_ref,
            ramp: ramp_ref,
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
        // Drawing an empty Mesh raises at the call site (cube-design.md
        // § 11.2). Without this gate the draw silently no-ops and users
        // chase a missing mesh through the renderer.
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

        // Snapshot buffer contents — the rasterizer borrows scene + target
        // mutably while we still need to read these, so a copy keeps the
        // FFI boundary clean.
        let positions_data: Vec<f32> = positions.inner_ref().data().to_vec();
        let indices_data: Option<Vec<i32>> =
            indices.as_ref().map(|i| i.inner_ref().data().to_vec());
        let normals_data: Option<Vec<f32>> =
            normals.as_ref().map(|n| n.inner_ref().data().to_vec());
        let uvs_data: Option<Vec<f32>> = uvs.as_ref().map(|u| u.inner_ref().data().to_vec());

        // col: int → flat color, Image → textured.
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

        let (light, ramp) = if shaded {
            self.resolve_lighting()
        } else {
            (None, None)
        };
        let light_ref = light.as_ref().map(|l: &pyxel::cube::RcLight| rc_ref!(l));
        let ramp_ref = ramp.as_ref().map(|r: &pyxel::cube::RcShadeRamp| rc_ref!(r));
        let state = DrawState {
            shaded,
            dither_alpha,
            depth_test,
            depth_write,
            billboard,
            light: light_ref,
            ramp: ramp_ref,
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

    // Dunder

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
