use std::cell::RefCell;

use pyo3::prelude::*;
use pyxel::cube::raster::{
    project_ellipse_perimeter, project_rect_corners, rasterize_circle_border,
    rasterize_circle_filled, rasterize_line, rasterize_textured_triangle, rasterize_triangle,
    screen_circle, sprite_corners, world_to_screen, write_pixel, ELLIPSE_SEGMENTS,
};
use pyxel::cube::scene::with_draw_context;
use pyxel::cube::{Node as InnerNode, Vec3 as InnerVec3};
use pyxel::{Image as InnerImage, RcImage as InnerRcImage};

use super::color_ramp::ColorRamp;
use super::light::Light;
use super::mat4::Mat4;
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

    // World-space position of a node-local Vec3 (composes the node's
    // ancestor chain into a single transform and applies it).
    fn world_pos(&self, local: InnerVec3) -> InnerVec3 {
        let world_mat = InnerNode::world_transform(&self.inner);
        pyxel::cube::raster::mat_apply(rc_ref!(&world_mat), &local)
    }

    // World-space matrix for a node-local Mat4 (compose ancestor world
    // transform with the local placement).
    fn world_mat_compose(&self, local: pyxel::cube::Mat4) -> pyxel::cube::Mat4 {
        let world_rc = InnerNode::world_transform(&self.inner);
        let composed = rc_ref!(&world_rc).mul_mat(&local);
        *rc_ref!(&composed)
    }
}

// Build a sampler closure that reads from `img` using affine UV in [0,1].
// Out-of-range UVs are clamped to the image bounds.
fn make_image_sampler(img: &InnerImage) -> impl Fn(f32, f32, i32, i32) -> i32 + '_ {
    let w = img.width() as f32;
    let h = img.height() as f32;
    let max_x = (img.width() as i32 - 1).max(0);
    let max_y = (img.height() as i32 - 1).max(0);
    move |u, v, _x, _y| {
        let xi = ((u * w).floor() as i32).clamp(0, max_x);
        let yi = ((v * h).floor() as i32).clamp(0, max_y);
        i32::from(img.pixel(xi as f32, yi as f32))
    }
}

// Image sampler that applies the ramp's per-level (primary, secondary,
// ratio) blend at each pixel. Per-pixel dithering keeps the source
// picture recognizable (each texel lands in its own ramp cell, so the
// per-base palette mapping survives) while the secondary/ratio terms
// add a Bayer-mediated brightness gradient across the face.
fn make_shaded_sampler<'a>(
    img: &'a InnerImage,
    ramp: &'a pyxel::cube::ColorRamp,
    level: usize,
) -> impl Fn(f32, f32, i32, i32) -> i32 + 'a {
    let w = img.width() as f32;
    let h = img.height() as f32;
    let max_x = (img.width() as i32 - 1).max(0);
    let max_y = (img.height() as i32 - 1).max(0);
    let palette_size = ramp.palette_size();
    move |u, v, x, y| {
        let xi = ((u * w).floor() as i32).clamp(0, max_x);
        let yi = ((v * h).floor() as i32).clamp(0, max_y);
        let base = i32::from(img.pixel(xi as f32, yi as f32));
        if palette_size == 0 {
            base
        } else {
            let base_idx = base.clamp(0, palette_size as i32 - 1) as usize;
            let (primary, secondary, ratio) = ramp.get(base_idx, level);
            i32::from(pyxel::cube::raster::dither_pick(
                primary, secondary, ratio, x, y,
            ))
        }
    }
}

// Rasterize a textured quad as two triangles. Row-major corner / uv
// order: 0=top-left, 1=top-right, 2=bottom-left, 3=bottom-right.
fn draw_textured_quad(
    ctx: &mut pyxel::cube::scene::DrawContext,
    corners: [(f32, f32, f32); 4],
    uvs: Uvs,
    img: &InnerRcImage,
    colkey: Option<i32>,
) {
    let target_mut = rc_mut!(&ctx.target);
    let scene_mut = rc_mut!(&ctx.scene);
    let depth_w = scene_mut.depth_w;
    let depth = scene_mut.depth.as_mut_slice();
    let img_ref = rc_ref!(img);
    let sampler = make_image_sampler(img_ref);
    rasterize_textured_triangle(
        target_mut, depth, depth_w, corners[0], corners[1], corners[2], uvs.0, uvs.1, uvs.2,
        &sampler, colkey, ctx.clip,
    );
    rasterize_textured_triangle(
        target_mut, depth, depth_w, corners[1], corners[3], corners[2], uvs.1, uvs.3, uvs.2,
        &sampler, colkey, ctx.clip,
    );
}

#[pymethods]
impl Node {
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

    // Inheritable subtree state

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
    fn color_ramp(&self) -> Option<ColorRamp> {
        self.inner_ref()
            .color_ramp
            .as_ref()
            .map(|r| ColorRamp::wrap(r.clone()))
    }

    #[setter]
    fn set_color_ramp(&self, v: Option<PyRef<'_, ColorRamp>>) {
        self.inner_mut().color_ramp = v.as_ref().map(|r| r.inner.clone());
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
    fn children(&self, py: Python<'_>) -> Vec<Py<Node>> {
        self.children
            .borrow()
            .iter()
            .map(|c| c.clone_ref(py))
            .collect()
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
        // Detach from previous parent (Python-tracked + core-tracked).
        // For now `parent` is not tracked on the Python side, so just
        // ensure the core hierarchy stays consistent.
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
    // Registering them here lets PyO3's dispatch find the override on the
    // subclass via the standard Python MRO.

    #[allow(clippy::unused_self)]
    fn on_update(&self) {}

    #[allow(clippy::unused_self)]
    fn on_draw(&self) {}

    #[allow(clippy::unused_self)]
    #[pyo3(signature = (_other, _contact=None))]
    fn on_collide(&self, _other: PyRef<'_, Node>, _contact: Option<Py<PyAny>>) {}

    #[allow(clippy::unused_self)]
    fn on_destroy(&self) {}

    // Immediate-mode draw commands (callable from on_draw scope)

    fn pset(&self, pos: PyRef<'_, Vec3>, col: i32) {
        let world = self.world_pos(*pos.inner_ref());
        with_draw_context(|ctx| {
            let projected =
                world_to_screen(&world, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h);
            if let Some((sx, sy, sz)) = projected {
                let xi = sx.round() as i32;
                let yi = sy.round() as i32;
                if !ctx.clip.contains(xi, yi) {
                    return;
                }
                let target_mut = rc_mut!(&ctx.target);
                let scene_mut = rc_mut!(&ctx.scene);
                let depth_w = scene_mut.depth_w;
                write_pixel(
                    target_mut,
                    scene_mut.depth.as_mut_slice(),
                    depth_w,
                    xi,
                    yi,
                    sz,
                    col as u8,
                );
            }
        });
    }

    fn line(&self, p1: PyRef<'_, Vec3>, p2: PyRef<'_, Vec3>, col: i32) {
        let w1 = self.world_pos(*p1.inner_ref());
        let w2 = self.world_pos(*p2.inner_ref());
        with_draw_context(|ctx| {
            let s1 = world_to_screen(&w1, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h);
            let s2 = world_to_screen(&w2, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h);
            if let (Some(s1), Some(s2)) = (s1, s2) {
                let target_mut = rc_mut!(&ctx.target);
                let scene_mut = rc_mut!(&ctx.scene);
                let depth_w = scene_mut.depth_w;
                rasterize_line(
                    target_mut,
                    scene_mut.depth.as_mut_slice(),
                    depth_w,
                    s1,
                    s2,
                    col as u8,
                    col as u8,
                    0,
                    ctx.clip,
                );
            }
        });
    }

    fn tri(&self, p1: PyRef<'_, Vec3>, p2: PyRef<'_, Vec3>, p3: PyRef<'_, Vec3>, col: i32) {
        let w1 = self.world_pos(*p1.inner_ref());
        let w2 = self.world_pos(*p2.inner_ref());
        let w3 = self.world_pos(*p3.inner_ref());
        with_draw_context(|ctx| {
            let s1 = world_to_screen(&w1, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h);
            let s2 = world_to_screen(&w2, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h);
            let s3 = world_to_screen(&w3, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h);
            if let (Some(s1), Some(s2), Some(s3)) = (s1, s2, s3) {
                let target_mut = rc_mut!(&ctx.target);
                let scene_mut = rc_mut!(&ctx.scene);
                let depth_w = scene_mut.depth_w;
                rasterize_triangle(
                    target_mut,
                    scene_mut.depth.as_mut_slice(),
                    depth_w,
                    s1,
                    s2,
                    s3,
                    col as u8,
                    col as u8,
                    0,
                    ctx.clip,
                );
            }
        });
    }

    fn trib(&self, p1: PyRef<'_, Vec3>, p2: PyRef<'_, Vec3>, p3: PyRef<'_, Vec3>, col: i32) {
        // Outline = 3 lines, reusing the line draw command for clipping +
        // depth interpolation.
        let v1 = *p1.inner_ref();
        let v2 = *p2.inner_ref();
        let v3 = *p3.inner_ref();
        let w1 = self.world_pos(v1);
        let w2 = self.world_pos(v2);
        let w3 = self.world_pos(v3);
        with_draw_context(|ctx| {
            let s1 = world_to_screen(&w1, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h);
            let s2 = world_to_screen(&w2, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h);
            let s3 = world_to_screen(&w3, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h);
            let target_mut = rc_mut!(&ctx.target);
            let scene_mut = rc_mut!(&ctx.scene);
            let depth_w = scene_mut.depth_w;
            let depth = scene_mut.depth.as_mut_slice();
            for (a, b) in [(s1, s2), (s2, s3), (s3, s1)] {
                if let (Some(a), Some(b)) = (a, b) {
                    rasterize_line(
                        target_mut, depth, depth_w, a, b, col as u8, col as u8, 0, ctx.clip,
                    );
                }
            }
        });
    }

    fn circ(&self, pos: PyRef<'_, Vec3>, r: f32, col: i32) {
        let world = self.world_pos(*pos.inner_ref());
        with_draw_context(|ctx| {
            let projected = screen_circle(
                &world,
                r,
                &ctx.vp,
                rc_ref!(&ctx.camera),
                ctx.vp_x,
                ctx.vp_y,
                ctx.vp_w,
                ctx.vp_h,
            );
            if let Some((sx, sy, sr, sz)) = projected {
                let target_mut = rc_mut!(&ctx.target);
                let scene_mut = rc_mut!(&ctx.scene);
                let depth_w = scene_mut.depth_w;
                rasterize_circle_filled(
                    target_mut,
                    scene_mut.depth.as_mut_slice(),
                    depth_w,
                    sx,
                    sy,
                    sr,
                    sz,
                    col as u8,
                    col as u8,
                    0,
                    ctx.clip,
                );
            }
        });
    }

    fn circb(&self, pos: PyRef<'_, Vec3>, r: f32, col: i32) {
        let world = self.world_pos(*pos.inner_ref());
        with_draw_context(|ctx| {
            let projected = screen_circle(
                &world,
                r,
                &ctx.vp,
                rc_ref!(&ctx.camera),
                ctx.vp_x,
                ctx.vp_y,
                ctx.vp_w,
                ctx.vp_h,
            );
            if let Some((sx, sy, sr, sz)) = projected {
                let target_mut = rc_mut!(&ctx.target);
                let scene_mut = rc_mut!(&ctx.scene);
                let depth_w = scene_mut.depth_w;
                rasterize_circle_border(
                    target_mut,
                    scene_mut.depth.as_mut_slice(),
                    depth_w,
                    sx,
                    sy,
                    sr,
                    sz,
                    col as u8,
                    col as u8,
                    0,
                    ctx.clip,
                );
            }
        });
    }

    fn rect(&self, mat: PyRef<'_, Mat4>, w: f32, h: f32, col: i32) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        with_draw_context(|ctx| {
            let corners = project_rect_corners(
                &world_mat, w, h, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h,
            );
            if let [Some(p0), Some(p1), Some(p2), Some(p3)] = corners {
                let target_mut = rc_mut!(&ctx.target);
                let scene_mut = rc_mut!(&ctx.scene);
                let depth_w = scene_mut.depth_w;
                let depth = scene_mut.depth.as_mut_slice();
                rasterize_triangle(
                    target_mut, depth, depth_w, p0, p1, p2, col as u8, col as u8, 0, ctx.clip,
                );
                rasterize_triangle(
                    target_mut, depth, depth_w, p1, p3, p2, col as u8, col as u8, 0, ctx.clip,
                );
            }
        });
    }

    fn rectb(&self, mat: PyRef<'_, Mat4>, w: f32, h: f32, col: i32) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        with_draw_context(|ctx| {
            let corners = project_rect_corners(
                &world_mat, w, h, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h,
            );
            let target_mut = rc_mut!(&ctx.target);
            let scene_mut = rc_mut!(&ctx.scene);
            let depth_w = scene_mut.depth_w;
            let depth = scene_mut.depth.as_mut_slice();
            // Top, right, bottom, left edges.
            for (a, b) in [(0, 1), (1, 3), (3, 2), (2, 0)] {
                if let (Some(p0), Some(p1)) = (corners[a], corners[b]) {
                    rasterize_line(
                        target_mut, depth, depth_w, p0, p1, col as u8, col as u8, 0, ctx.clip,
                    );
                }
            }
        });
    }

    fn elli(&self, mat: PyRef<'_, Mat4>, w: f32, h: f32, col: i32) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        with_draw_context(|ctx| {
            let perim = project_ellipse_perimeter(
                &world_mat, w, h, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h,
            );
            // Fan triangulation around the projected center (mat origin).
            let center = world_to_screen(
                &pyxel::cube::raster::mat_apply(
                    &world_mat,
                    &InnerVec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ),
                &ctx.vp,
                ctx.vp_x,
                ctx.vp_y,
                ctx.vp_w,
                ctx.vp_h,
            );
            if let Some(c) = center {
                let target_mut = rc_mut!(&ctx.target);
                let scene_mut = rc_mut!(&ctx.scene);
                let depth_w = scene_mut.depth_w;
                let depth = scene_mut.depth.as_mut_slice();
                for i in 0..ELLIPSE_SEGMENTS {
                    let a = perim[i];
                    let b = perim[(i + 1) % ELLIPSE_SEGMENTS];
                    if let (Some(a), Some(b)) = (a, b) {
                        rasterize_triangle(
                            target_mut, depth, depth_w, c, a, b, col as u8, col as u8, 0, ctx.clip,
                        );
                    }
                }
            }
        });
    }

    fn ellib(&self, mat: PyRef<'_, Mat4>, w: f32, h: f32, col: i32) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        with_draw_context(|ctx| {
            let perim = project_ellipse_perimeter(
                &world_mat, w, h, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h,
            );
            let target_mut = rc_mut!(&ctx.target);
            let scene_mut = rc_mut!(&ctx.scene);
            let depth_w = scene_mut.depth_w;
            let depth = scene_mut.depth.as_mut_slice();
            for i in 0..ELLIPSE_SEGMENTS {
                let a = perim[i];
                let b = perim[(i + 1) % ELLIPSE_SEGMENTS];
                if let (Some(a), Some(b)) = (a, b) {
                    rasterize_line(
                        target_mut, depth, depth_w, a, b, col as u8, col as u8, 0, ctx.clip,
                    );
                }
            }
        });
    }

    #[pyo3(signature = (pos, img, uvs, w, h, colkey=None, angle=0.0))]
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
    ) {
        let world_pos = self.world_pos(*pos.inner_ref());
        let img_inner = img.inner.clone();
        with_draw_context(|ctx| {
            let corners = sprite_corners(&world_pos, w, h, angle, rc_ref!(&ctx.camera));
            let projected: [_; 4] = std::array::from_fn(|i| {
                world_to_screen(&corners[i], &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h)
            });
            if let [Some(p0), Some(p1), Some(p2), Some(p3)] = projected {
                draw_textured_quad(ctx, [p0, p1, p2, p3], uvs, &img_inner, colkey);
            }
        });
    }

    #[pyo3(signature = (mat, img, uvs, w, h, colkey=None))]
    #[allow(clippy::too_many_arguments)]
    fn plane(
        &self,
        mat: PyRef<'_, Mat4>,
        img: PyRef<'_, crate::image_wrapper::Image>,
        uvs: Uvs,
        w: f32,
        h: f32,
        colkey: Option<i32>,
    ) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let img_inner = img.inner.clone();
        with_draw_context(|ctx| {
            let corners = project_rect_corners(
                &world_mat, w, h, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h,
            );
            if let [Some(p0), Some(p1), Some(p2), Some(p3)] = corners {
                draw_textured_quad(ctx, [p0, p1, p2, p3], uvs, &img_inner, colkey);
            }
        });
    }

    fn mesh(&self, mat: PyRef<'_, Mat4>, mesh: PyRef<'_, super::mesh::Mesh>) {
        let world_mat = self.world_mat_compose(*mat.inner_ref());
        let mesh_inner = mesh.inner.clone();
        // Resolve effective lighting from this node's hierarchy. None at
        // every ancestor means flat-color (no shading); otherwise each
        // face is shaded by its world-space normal against the light.
        let effective_light = InnerNode::effective_light(&self.inner);
        let effective_ramp = InnerNode::effective_color_ramp(&self.inner);
        with_draw_context(|ctx| {
            let m = rc_ref!(&mesh_inner);
            let world_vertices: Vec<InnerVec3> = m
                .vertices
                .iter()
                .map(|v| pyxel::cube::raster::mat_apply(&world_mat, v))
                .collect();
            let projected: Vec<Option<(f32, f32, f32)>> = world_vertices
                .iter()
                .map(|w| world_to_screen(w, &ctx.vp, ctx.vp_x, ctx.vp_y, ctx.vp_w, ctx.vp_h))
                .collect();
            let target_mut = rc_mut!(&ctx.target);
            let scene_mut = rc_mut!(&ctx.scene);
            let depth_w = scene_mut.depth_w;
            let depth = scene_mut.depth.as_mut_slice();
            let base_col = m.col;
            let image = m.image.clone();
            let uvs_vec = m.uvs.clone();
            for face in &m.faces {
                let i0 = face[0] as usize;
                let i1 = face[1] as usize;
                let i2 = face[2] as usize;
                let p0 = projected[i0];
                let p1 = projected[i1];
                let p2 = projected[i2];
                if let (Some(p0), Some(p1), Some(p2)) = (p0, p1, p2) {
                    let normal = pyxel::cube::raster::tri_normal(
                        &world_vertices[i0],
                        &world_vertices[i1],
                        &world_vertices[i2],
                    );
                    if let (Some(image), Some(uvs)) = (image.as_ref(), uvs_vec.as_ref()) {
                        let img = rc_ref!(image);
                        let uv0 = uvs[i0];
                        let uv1 = uvs[i1];
                        let uv2 = uvs[i2];
                        // For textured faces, compute the face's flat-shading
                        // level once and lookup ramp[base, level] per pixel
                        // so the texture also reacts to lighting.
                        match (&effective_light, &effective_ramp) {
                            (Some(light), Some(ramp)) => {
                                let level = pyxel::cube::raster::face_shade_level(
                                    rc_ref!(light),
                                    Some(&normal),
                                );
                                let ramp_ref = rc_ref!(ramp);
                                let sampler = make_shaded_sampler(img, ramp_ref, level);
                                rasterize_textured_triangle(
                                    target_mut, depth, depth_w, p0, p1, p2, uv0, uv1, uv2,
                                    &sampler, None, ctx.clip,
                                );
                            }
                            _ => {
                                let sampler = make_image_sampler(img);
                                rasterize_textured_triangle(
                                    target_mut, depth, depth_w, p0, p1, p2, uv0, uv1, uv2,
                                    &sampler, None, ctx.clip,
                                );
                            }
                        }
                    } else {
                        let entry = match (&effective_light, &effective_ramp) {
                            (Some(light), Some(ramp)) => pyxel::cube::raster::lookup_ramp(
                                rc_ref!(ramp),
                                rc_ref!(light),
                                base_col,
                                Some(&normal),
                            ),
                            _ => (base_col, base_col, 0_u8),
                        };
                        rasterize_triangle(
                            target_mut,
                            depth,
                            depth_w,
                            p0,
                            p1,
                            p2,
                            entry.0 as u8,
                            entry.1 as u8,
                            entry.2,
                            ctx.clip,
                        );
                    }
                }
            }
        });
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
