use super::camera::Camera;
use super::light::Light;
use super::math::{Mat4, Vec3};
use super::model::{Model, ModelNode};
use super::rasterizer::{
    self, RasterTri, ScreenVertex, ShadePalette, ZBuffer, DEFAULT_SHADE_PALETTE,
};
use crate::canvas::Canvas;
use crate::image::Color;
use crate::pyxel;

struct SceneNode {
    model: *mut Model,
    pos: Vec3,
    rot: Vec3,
    scale: Vec3,
}

pub struct Scene {
    nodes: Vec<SceneNode>,
    lights: Vec<Light>,
    zbuf: ZBuffer,
    pub shade_palette: ShadePalette,
}

impl Scene {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            lights: vec![Light::default()],
            zbuf: ZBuffer::new(1, 1),
            shade_palette: DEFAULT_SHADE_PALETTE,
        }
    }

    pub fn add(&mut self, model: *mut Model, pos: Vec3, rot: Vec3, scale: Vec3) {
        self.nodes.push(SceneNode {
            model,
            pos,
            rot,
            scale,
        });
    }

    pub fn remove_all(&mut self) {
        self.nodes.clear();
    }

    pub fn set_light(&mut self, index: usize, light: Light) {
        if index < self.lights.len() {
            self.lights[index] = light;
        } else {
            self.lights.resize_with(index, Light::default);
            self.lights.push(light);
        }
    }

    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(&mut self, x: i32, y: i32, w: u32, h: u32, camera: &Camera) {
        if w == 0 || h == 0 {
            return;
        }

        // Resize and clear Z-buffer
        self.zbuf.resize(w, h);

        // View-projection matrix
        let aspect = w as f32 / h as f32;
        let view = camera.view_matrix();
        let proj = camera.projection_matrix(aspect);
        let vp = proj * view;

        // Screen canvas (singleton)
        let canvas = &mut pyxel::screen().canvas;

        // Gather image data for texturing
        let images_vec = pyxel::images();
        let mut img_data: Vec<&[Color]> = Vec::new();
        let mut img_widths: Vec<u32> = Vec::new();
        for &img_ptr in images_vec.iter() {
            let img = unsafe { &*img_ptr };
            img_data.push(&img.canvas.data);
            img_widths.push(img.canvas.width());
        }

        let hw = w as f32 * 0.5;
        let hh = h as f32 * 0.5;
        let cam_pos = camera.pos;

        for scene_node in &self.nodes {
            let model = unsafe { &*scene_node.model };

            // Build model matrix: translation * rotation_z * rotation_y * rotation_x * scale
            let model_mat = Mat4::translation(scene_node.pos.x, scene_node.pos.y, scene_node.pos.z)
                * Mat4::rotation_z(scene_node.rot.z)
                * Mat4::rotation_y(scene_node.rot.y)
                * Mat4::rotation_x(scene_node.rot.x)
                * Mat4::scale(scene_node.scale.x, scene_node.scale.y, scene_node.scale.z);

            render_node(
                canvas,
                &mut self.zbuf,
                &model.root,
                &model_mat,
                &vp,
                hw,
                hh,
                x,
                y,
                &img_data,
                &img_widths,
                cam_pos,
                &self.lights,
                &self.shade_palette,
            );
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::too_many_arguments)]
fn render_node(
    canvas: &mut Canvas<Color>,
    zbuf: &mut ZBuffer,
    node: &ModelNode,
    parent_mat: &Mat4,
    vp: &Mat4,
    hw: f32,
    hh: f32,
    ox: i32,
    oy: i32,
    images: &[&[Color]],
    image_widths: &[u32],
    cam_pos: Vec3,
    lights: &[Light],
    palette: &ShadePalette,
) {
    // Build local transform matrix
    let local = Mat4::translation(node.pos.x, node.pos.y, node.pos.z)
        * Mat4::rotation_z(node.rot.z)
        * Mat4::rotation_y(node.rot.y)
        * Mat4::rotation_x(node.rot.x)
        * Mat4::scale(node.scale.x, node.scale.y, node.scale.z);
    let world = *parent_mat * local;
    let mvp = *vp * world;

    for face in &node.faces {
        let p0 = node.vertices[face.v0 as usize];
        let p1 = node.vertices[face.v1 as usize];
        let p2 = node.vertices[face.v2 as usize];

        // Transform to world space
        let wp0 = world.transform_point(p0);
        let wp1 = world.transform_point(p1);
        let wp2 = world.transform_point(p2);

        // Face normal
        let edge1 = wp1 - wp0;
        let edge2 = wp2 - wp0;
        let normal = edge1.cross(edge2).normalize();

        // Back-face culling
        let view_dir = (cam_pos - wp0).normalize();
        let facing = normal.dot(view_dir);
        if facing < 0.0 && !face.double_sided {
            continue;
        }
        let shade_normal = if facing < 0.0 { -normal } else { normal };

        // Compute shade (diffuse only)
        let ambient = 0.4_f32;
        let mut diffuse = 0.0_f32;
        for l in lights {
            let ndl = shade_normal.dot(-l.dir).max(0.0);
            diffuse += ndl * 0.6;
        }
        let shade = (ambient + diffuse).min(1.0);

        // Project vertices (with w for near-plane check)
        let (cp0, w0) = mvp.transform_point_w(p0);
        let (cp1, w1) = mvp.transform_point_w(p1);
        let (cp2, w2) = mvp.transform_point_w(p2);

        // Skip triangles with any vertex behind the camera (w <= 0)
        if w0 <= 0.0 || w1 <= 0.0 || w2 <= 0.0 {
            continue;
        }
        // Far plane clipping
        if cp0.z > 1.0 && cp1.z > 1.0 && cp2.z > 1.0 {
            continue;
        }

        // NDC to screen
        let sv0 = ScreenVertex {
            x: (cp0.x + 1.0) * hw,
            y: (1.0 - cp0.y) * hh,
            z: cp0.z,
        };
        let sv1 = ScreenVertex {
            x: (cp1.x + 1.0) * hw,
            y: (1.0 - cp1.y) * hh,
            z: cp1.z,
        };
        let sv2 = ScreenVertex {
            x: (cp2.x + 1.0) * hw,
            y: (1.0 - cp2.y) * hh,
            z: cp2.z,
        };

        let tri = RasterTri {
            sv0,
            sv1,
            sv2,
            material: face.material,
            shade,
        };

        rasterizer::rasterize_triangle(canvas, zbuf, &tri, ox, oy, images, image_widths, palette);
    }

    // Recurse into children
    for child in &node.children {
        render_node(
            canvas,
            zbuf,
            child,
            &world,
            vp,
            hw,
            hh,
            ox,
            oy,
            images,
            image_widths,
            cam_pos,
            lights,
            palette,
        );
    }
}
