use crate::cube::math::Vec3;
use crate::image::Color;

/// UV texture coordinate.
#[derive(Clone, Copy, Debug)]
pub struct Uv {
    pub u: f32,
    pub v: f32,
}

impl Uv {
    pub const fn new(u: f32, v: f32) -> Self {
        Self { u, v }
    }
}

/// Material applied to a face — solid color or textured.
#[derive(Clone, Copy, Debug)]
pub enum FaceMaterial {
    Color(Color),
    Texture { img: u32, uv0: Uv, uv1: Uv, uv2: Uv },
}

/// A single triangle defined by vertex indices into a node's vertex list.
#[derive(Clone, Copy, Debug)]
pub struct Face {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,
    pub material: FaceMaterial,
    pub double_sided: bool,
}

impl Face {
    pub const fn new(v0: u32, v1: u32, v2: u32, material: FaceMaterial) -> Self {
        Self {
            v0,
            v1,
            v2,
            material,
            double_sided: false,
        }
    }
}

/// A node in the model hierarchy, holding geometry and child nodes.
#[derive(Clone, Debug)]
pub struct ModelNode {
    pub name: String,
    pub pos: Vec3,
    pub rot: Vec3,
    pub scale: Vec3,
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub children: Vec<ModelNode>,
}

impl ModelNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pos: Vec3::ZERO,
            rot: Vec3::ZERO,
            scale: Vec3::new(1.0, 1.0, 1.0),
            vertices: Vec::new(),
            faces: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Recursively search for a node by name (immutable).
    pub fn find_node(&self, name: &str) -> Option<&ModelNode> {
        if self.name == name {
            return Some(self);
        }
        self.children.iter().find_map(|c| c.find_node(name))
    }

    /// Recursively search for a node by name (mutable).
    pub fn find_node_mut(&mut self, name: &str) -> Option<&mut ModelNode> {
        if self.name == name {
            return Some(self);
        }
        self.children.iter_mut().find_map(|c| c.find_node_mut(name))
    }

    /// Add a vertex and return its index.
    fn push_vertex(&mut self, v: Vec3) -> u32 {
        let idx = self.vertices.len() as u32;
        self.vertices.push(v);
        idx
    }

    /// Add a solid-color triangle.
    pub fn add_tri(&mut self, v0: Vec3, v1: Vec3, v2: Vec3, col: Color) {
        let i0 = self.push_vertex(v0);
        let i1 = self.push_vertex(v1);
        let i2 = self.push_vertex(v2);
        self.faces
            .push(Face::new(i0, i1, i2, FaceMaterial::Color(col)));
    }

    /// Add a textured triangle.
    pub fn add_tri_tex(
        &mut self,
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        img: u32,
        uv0: Uv,
        uv1: Uv,
        uv2: Uv,
    ) {
        let i0 = self.push_vertex(v0);
        let i1 = self.push_vertex(v1);
        let i2 = self.push_vertex(v2);
        self.faces.push(Face::new(
            i0,
            i1,
            i2,
            FaceMaterial::Texture { img, uv0, uv1, uv2 },
        ));
    }
}

/// A 3D model composed of a hierarchy of nodes.
pub struct Model {
    pub root: ModelNode,
}

define_rc_type!(RcModel, Model);

impl Model {
    fn empty() -> Self {
        Self {
            root: ModelNode::new("root"),
        }
    }

    pub fn new() -> RcModel {
        new_rc_type!(Self::empty())
    }

    /// Add a solid-color triangle to the root node.
    pub fn tri(&mut self, v0: Vec3, v1: Vec3, v2: Vec3, col: Color) {
        self.root.add_tri(v0, v1, v2, col);
    }

    /// Add a textured triangle to the root node.
    pub fn tri_tex(&mut self, v0: Vec3, v1: Vec3, v2: Vec3, img: u32, uv0: Uv, uv1: Uv, uv2: Uv) {
        self.root.add_tri_tex(v0, v1, v2, img, uv0, uv1, uv2);
    }

    /// Create a unit cube (Z-up) with 12 solid-color triangles.
    pub fn cube(col: Color) -> RcModel {
        // Vertices: index = 4*z + 2*y + x  (z=0 bottom, z=1 top)
        //   0: (-0.5, -0.5, -0.5)  bottom-left-front
        //   1: ( 0.5, -0.5, -0.5)  bottom-right-front
        //   2: ( 0.5,  0.5, -0.5)  bottom-right-back
        //   3: (-0.5,  0.5, -0.5)  bottom-left-back
        //   4: (-0.5, -0.5,  0.5)  top-left-front
        //   5: ( 0.5, -0.5,  0.5)  top-right-front
        //   6: ( 0.5,  0.5,  0.5)  top-right-back
        //   7: (-0.5,  0.5,  0.5)  top-left-back
        let v = [
            Vec3::new(-0.5, -0.5, -0.5), // 0
            Vec3::new(0.5, -0.5, -0.5),  // 1
            Vec3::new(0.5, 0.5, -0.5),   // 2
            Vec3::new(-0.5, 0.5, -0.5),  // 3
            Vec3::new(-0.5, -0.5, 0.5),  // 4
            Vec3::new(0.5, -0.5, 0.5),   // 5
            Vec3::new(0.5, 0.5, 0.5),    // 6
            Vec3::new(-0.5, 0.5, 0.5),   // 7
        ];

        let mut model = Self::empty();
        let r = &mut model.root;

        // Bottom face (z = -0.5), normal = -Z: winding CCW when viewed from -Z
        r.add_tri(v[0], v[2], v[1], col);
        r.add_tri(v[0], v[3], v[2], col);
        // Top face (z = +0.5), normal = +Z: winding CCW when viewed from +Z
        r.add_tri(v[4], v[5], v[6], col);
        r.add_tri(v[4], v[6], v[7], col);
        // Front face (y = -0.5), normal = -Y: winding CCW when viewed from -Y
        r.add_tri(v[0], v[1], v[5], col);
        r.add_tri(v[0], v[5], v[4], col);
        // Back face (y = +0.5), normal = +Y: winding CCW when viewed from +Y
        r.add_tri(v[2], v[3], v[7], col);
        r.add_tri(v[2], v[7], v[6], col);
        // Left face (x = -0.5), normal = -X: winding CCW when viewed from -X
        r.add_tri(v[0], v[4], v[7], col);
        r.add_tri(v[0], v[7], v[3], col);
        // Right face (x = +0.5), normal = +X: winding CCW when viewed from +X
        r.add_tri(v[1], v[2], v[6], col);
        r.add_tri(v[1], v[6], v[5], col);

        new_rc_type!(model)
    }

    /// Create a unit cube with textured faces.
    /// Each face maps to (u, v, u+w, v+h) in the image bank.
    #[allow(clippy::many_single_char_names)]
    pub fn tex_cube(img: u32, u: f32, v: f32, w: f32, h: f32) -> RcModel {
        let v8 = [
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(-0.5, 0.5, -0.5),
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(0.5, -0.5, 0.5),
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(-0.5, 0.5, 0.5),
        ];
        let uv00 = Uv::new(u, v);
        let uv10 = Uv::new(u + w, v);
        let uv01 = Uv::new(u, v + h);
        let uv11 = Uv::new(u + w, v + h);

        let mut model = Self::empty();
        let r = &mut model.root;
        // Match the solid cube's triangle definitions with UV mapping
        // Bottom (z=-0.5): (0,2,1), (0,3,2)
        r.add_tri_tex(v8[0], v8[2], v8[1], img, uv00, uv11, uv10);
        r.add_tri_tex(v8[0], v8[3], v8[2], img, uv00, uv01, uv11);
        // Top (z=+0.5): (4,5,6), (4,6,7)
        r.add_tri_tex(v8[4], v8[5], v8[6], img, uv00, uv10, uv11);
        r.add_tri_tex(v8[4], v8[6], v8[7], img, uv00, uv11, uv01);
        // Front (y=-0.5): (0,1,5), (0,5,4)
        r.add_tri_tex(v8[0], v8[1], v8[5], img, uv00, uv10, uv11);
        r.add_tri_tex(v8[0], v8[5], v8[4], img, uv00, uv11, uv01);
        // Back (y=+0.5): (2,3,7), (2,7,6)
        r.add_tri_tex(v8[2], v8[3], v8[7], img, uv00, uv10, uv11);
        r.add_tri_tex(v8[2], v8[7], v8[6], img, uv00, uv11, uv01);
        // Left (x=-0.5): (0,4,7), (0,7,3)
        r.add_tri_tex(v8[0], v8[4], v8[7], img, uv00, uv10, uv11);
        r.add_tri_tex(v8[0], v8[7], v8[3], img, uv00, uv11, uv01);
        // Right (x=+0.5): (1,2,6), (1,6,5)
        r.add_tri_tex(v8[1], v8[2], v8[6], img, uv00, uv10, uv11);
        r.add_tri_tex(v8[1], v8[6], v8[5], img, uv00, uv11, uv01);
        new_rc_type!(model)
    }

    /// Create a unit plane on the XY ground with 2 solid-color triangles.
    /// Normal points in the +Z direction.
    pub fn plane(col: Color) -> RcModel {
        let v = [
            Vec3::new(-0.5, -0.5, 0.0), // bottom-left
            Vec3::new(0.5, -0.5, 0.0),  // bottom-right
            Vec3::new(0.5, 0.5, 0.0),   // top-right
            Vec3::new(-0.5, 0.5, 0.0),  // top-left
        ];

        let mut model = Self::empty();
        model.root.add_tri(v[0], v[1], v[2], col);
        model.root.add_tri(v[0], v[2], v[3], col);
        new_rc_type!(model)
    }

    /// Create a square pyramid (base on XY plane, apex at +Z).
    pub fn pyramid(col: Color) -> RcModel {
        let base = [
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(-0.5, 0.5, -0.5),
        ];
        let apex = Vec3::new(0.0, 0.0, 0.5);

        let mut model = Self::empty();
        let r = &mut model.root;

        r.add_tri(base[0], base[2], base[1], col);
        r.add_tri(base[0], base[3], base[2], col);
        r.add_tri(base[0], base[1], apex, col);
        r.add_tri(base[1], base[2], apex, col);
        r.add_tri(base[2], base[3], apex, col);
        r.add_tri(base[3], base[0], apex, col);

        new_rc_type!(model)
    }

    /// Create a textured square pyramid.
    #[allow(clippy::many_single_char_names)]
    pub fn tex_pyramid(img: u32, u: f32, v: f32, w: f32, h: f32) -> RcModel {
        let base = [
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(-0.5, 0.5, -0.5),
        ];
        let apex = Vec3::new(0.0, 0.0, 0.5);
        let uv00 = Uv::new(u, v);
        let uv10 = Uv::new(u + w, v);
        let uv01 = Uv::new(u, v + h);
        let uv11 = Uv::new(u + w, v + h);
        let uvm = Uv::new(u + w * 0.5, v);

        let mut model = Self::empty();
        let r = &mut model.root;

        r.add_tri_tex(base[0], base[2], base[1], img, uv00, uv11, uv10);
        r.add_tri_tex(base[0], base[3], base[2], img, uv00, uv01, uv11);
        r.add_tri_tex(base[0], base[1], apex, img, uv00, uv10, uvm);
        r.add_tri_tex(base[1], base[2], apex, img, uv00, uv10, uvm);
        r.add_tri_tex(base[2], base[3], apex, img, uv00, uv10, uvm);
        r.add_tri_tex(base[3], base[0], apex, img, uv00, uv10, uvm);

        new_rc_type!(model)
    }

    /// Create a low-poly sphere (icosphere with one subdivision).
    pub fn sphere(col: Color) -> RcModel {
        let t = f32::midpoint(1.0, 5.0_f32.sqrt());
        let n = (1.0 + t * t).sqrt();
        let a = 1.0 / n * 0.5;
        let b = t / n * 0.5;

        // 12 icosahedron vertices
        let verts = [
            Vec3::new(-a, b, 0.0),
            Vec3::new(a, b, 0.0),
            Vec3::new(-a, -b, 0.0),
            Vec3::new(a, -b, 0.0),
            Vec3::new(0.0, -a, b),
            Vec3::new(0.0, a, b),
            Vec3::new(0.0, -a, -b),
            Vec3::new(0.0, a, -b),
            Vec3::new(b, 0.0, -a),
            Vec3::new(b, 0.0, a),
            Vec3::new(-b, 0.0, -a),
            Vec3::new(-b, 0.0, a),
        ];

        // 20 icosahedron faces
        let faces: [(usize, usize, usize); 20] = [
            (0, 11, 5),
            (0, 5, 1),
            (0, 1, 7),
            (0, 7, 10),
            (0, 10, 11),
            (1, 5, 9),
            (5, 11, 4),
            (11, 10, 2),
            (10, 7, 6),
            (7, 1, 8),
            (3, 9, 4),
            (3, 4, 2),
            (3, 2, 6),
            (3, 6, 8),
            (3, 8, 9),
            (4, 9, 5),
            (2, 4, 11),
            (6, 2, 10),
            (8, 6, 7),
            (9, 8, 1),
        ];

        let mut model = Self::empty();
        for (i0, i1, i2) in faces {
            model.root.add_tri(verts[i0], verts[i1], verts[i2], col);
        }
        new_rc_type!(model)
    }

    /// Create a textured low-poly sphere (icosphere).
    #[allow(clippy::many_single_char_names)]
    pub fn tex_sphere(img: u32, u: f32, v: f32, w: f32, h: f32) -> RcModel {
        let t = f32::midpoint(1.0, 5.0_f32.sqrt());
        let n = (1.0 + t * t).sqrt();
        let va = 1.0 / n * 0.5;
        let vb = t / n * 0.5;

        let verts = [
            Vec3::new(-va, vb, 0.0),
            Vec3::new(va, vb, 0.0),
            Vec3::new(-va, -vb, 0.0),
            Vec3::new(va, -vb, 0.0),
            Vec3::new(0.0, -va, vb),
            Vec3::new(0.0, va, vb),
            Vec3::new(0.0, -va, -vb),
            Vec3::new(0.0, va, -vb),
            Vec3::new(vb, 0.0, -va),
            Vec3::new(vb, 0.0, va),
            Vec3::new(-vb, 0.0, -va),
            Vec3::new(-vb, 0.0, va),
        ];

        let faces: [(usize, usize, usize); 20] = [
            (0, 11, 5),
            (0, 5, 1),
            (0, 1, 7),
            (0, 7, 10),
            (0, 10, 11),
            (1, 5, 9),
            (5, 11, 4),
            (11, 10, 2),
            (10, 7, 6),
            (7, 1, 8),
            (3, 9, 4),
            (3, 4, 2),
            (3, 2, 6),
            (3, 6, 8),
            (3, 8, 9),
            (4, 9, 5),
            (2, 4, 11),
            (6, 2, 10),
            (8, 6, 7),
            (9, 8, 1),
        ];

        let uv00 = Uv::new(u, v);
        let uv10 = Uv::new(u + w, v);
        let uv01 = Uv::new(u, v + h);

        let mut model = Self::empty();
        for (i0, i1, i2) in faces {
            model
                .root
                .add_tri_tex(verts[i0], verts[i1], verts[i2], img, uv00, uv10, uv01);
        }
        new_rc_type!(model)
    }

    /// Find a node by name (immutable).
    pub fn node(&self, name: &str) -> Option<&ModelNode> {
        self.root.find_node(name)
    }

    /// Find a node by name (mutable).
    pub fn node_mut(&mut self, name: &str) -> Option<&mut ModelNode> {
        self.root.find_node_mut(name)
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::empty()
    }
}
