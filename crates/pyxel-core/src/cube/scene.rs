use std::cell::Cell;

use crate::cube::camera::RcCamera;
use crate::cube::raster::{ClipRect, Mat4x4};
use crate::image::RcImage;

// Scene-level state shared with the binding wrapper. The Node hierarchy
// itself lives on the inherited Node (see binding/cube/scene.rs); Scene
// only owns the clear color and the depth buffer used by the rasterizer.

pub struct Scene {
    pub clear_color: Option<i32>,
    pub depth: Vec<f32>,
    pub depth_w: u32,
    pub depth_h: u32,
}

define_rc_type!(RcScene, Scene);

impl Scene {
    pub fn new() -> RcScene {
        new_rc_type!(Scene {
            clear_color: None,
            depth: Vec::new(),
            depth_w: 0,
            depth_h: 0,
        })
    }

    pub fn ensure_depth(&mut self, w: u32, h: u32) {
        if self.depth_w != w || self.depth_h != h {
            self.depth = vec![f32::INFINITY; (w * h) as usize];
            self.depth_w = w;
            self.depth_h = h;
        }
    }

    pub fn clear_depth(&mut self) {
        self.depth.fill(f32::INFINITY);
    }
}

// Per-frame rasterizer context shared between Scene::draw and each Node's
// draw commands. Built at the start of Scene::draw, looked up by Node
// draw commands through `with_draw_context`, torn down on draw end.

pub struct DrawContext {
    pub target: RcImage,
    pub vp: Mat4x4,
    pub vp_x: f32,
    pub vp_y: f32,
    pub vp_w: f32,
    pub vp_h: f32,
    pub clip: ClipRect,
    pub camera: RcCamera,
    pub scene: RcScene,
}

thread_local! {
    // Current draw context, set by Scene::draw for the duration of the
    // tree traversal. Single-threaded by design (cube runs on Pyxel's
    // main thread); thread_local is the minimal carrier for the
    // Rust-Python boundary.
    static CURRENT_DRAW_CONTEXT: Cell<Option<DrawContext>> = const { Cell::new(None) };
}

pub fn set_draw_context(ctx: DrawContext) {
    CURRENT_DRAW_CONTEXT.with(|c| c.set(Some(ctx)));
}

pub fn clear_draw_context() {
    CURRENT_DRAW_CONTEXT.with(|c| c.set(None));
}

// Run `f` with mutable access to the current draw context, returning
// None when no context is active (i.e., outside Scene::draw).
pub fn with_draw_context<R>(f: impl FnOnce(&mut DrawContext) -> R) -> Option<R> {
    CURRENT_DRAW_CONTEXT.with(|cell| {
        let mut ctx = cell.take()?;
        let result = f(&mut ctx);
        cell.set(Some(ctx));
        Some(result)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let s = Scene::new();
        let s = rc_ref!(&s);
        assert!(s.clear_color.is_none());
        assert_eq!(s.depth_w, 0);
        assert_eq!(s.depth_h, 0);
        assert!(s.depth.is_empty());
    }

    #[test]
    fn test_ensure_depth() {
        let s = Scene::new();
        let s_mut = rc_mut!(&s);
        s_mut.ensure_depth(64, 48);
        assert_eq!(s_mut.depth_w, 64);
        assert_eq!(s_mut.depth_h, 48);
        assert_eq!(s_mut.depth.len(), 64 * 48);
    }

    #[test]
    fn test_ensure_depth_resize() {
        let s = Scene::new();
        let s_mut = rc_mut!(&s);
        s_mut.ensure_depth(64, 48);
        s_mut.ensure_depth(128, 96);
        assert_eq!(s_mut.depth.len(), 128 * 96);
    }

    #[test]
    fn test_clear_depth() {
        let s = Scene::new();
        let s_mut = rc_mut!(&s);
        s_mut.ensure_depth(8, 8);
        s_mut.depth[0] = 0.5;
        s_mut.clear_depth();
        assert_eq!(s_mut.depth[0], f32::INFINITY);
    }

    #[test]
    fn test_with_draw_context_outside_scope_returns_none() {
        // No context set: with_draw_context returns None.
        let result = with_draw_context(|_| 42);
        assert!(result.is_none());
    }
}
