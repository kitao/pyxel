use std::cell::RefCell;

use crate::cube::bvh::Bvh;
use crate::cube::mat4::{Mat4, RcMat4};
use crate::cube::motion::RcMotion;
use crate::cube::primitive::RcPrimitive;
use crate::image::RcImage;

// Asset container for a hierarchical 3D model. primitives / transforms /
// parents are parallel arrays; col_img holds either a flat color
// (ColImage::Color) or a shared texture (ColImage::Image). parents[i] < i
// is required (topological order); validate() enforces this.

#[derive(Clone)]
pub enum ColImage {
    Color(i32),
    Image(RcImage),
}

impl ColImage {
    pub fn as_flat_and_image(&self) -> (i32, Option<RcImage>) {
        match self {
            Self::Color(c) => (*c, None),
            Self::Image(img) => (0, Some(img.clone())),
        }
    }
}

pub struct Mesh {
    pub primitives: Vec<Option<RcPrimitive>>,
    pub transforms: Vec<RcMat4>,
    pub parents: Vec<i32>,
    pub names: Vec<String>,
    pub motions: Vec<RcMotion>,
    pub col_img: ColImage,
    pub colkey: Option<i32>,
    // Lazy collision BVH. Built on first mesh-collider query; never
    // refit (cube-design.md § 11.1: dynamic mesh colliders are out of
    // scope). Not exposed through the binding.
    pub bvh: RefCell<Option<Bvh>>,
}

define_rc_type!(RcMesh, Mesh);

impl Mesh {
    pub fn new() -> RcMesh {
        new_rc_type!(Mesh {
            primitives: Vec::new(),
            transforms: Vec::new(),
            parents: Vec::new(),
            names: Vec::new(),
            motions: Vec::new(),
            col_img: ColImage::Color(7),
            colkey: None,
            bvh: RefCell::new(None),
        })
    }

    pub fn from_glb(filename: &str, colkey: Option<i32>, fps: f32) -> Result<RcMesh, String> {
        crate::cube::glb_parser::parse_glb(filename, colkey, fps)
    }

    // Build the collision BVH if absent and call `f` with a borrowed
    // reference. The BVH is stored in mesh-local space (parts composed
    // with the identity outer transform); callers transform the query
    // AABB into this frame before traversing.
    pub fn with_collision_bvh<R>(&self, f: impl FnOnce(&Bvh) -> R) -> R {
        if self.bvh.borrow().is_none() {
            let (positions, triangles) = self.collect_triangles();
            *self.bvh.borrow_mut() = Some(Bvh::build(positions, triangles));
        }
        let guard = self.bvh.borrow();
        f(guard.as_ref().unwrap())
    }

    fn collect_triangles(&self) -> (Vec<crate::cube::vec3::Vec3>, Vec<[u32; 3]>) {
        let identity = Mat4::identity_value();
        let world_per_part = self.compose_world_transforms(&identity);
        let mut positions: Vec<crate::cube::vec3::Vec3> = Vec::new();
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        for (i, prim_opt) in self.primitives.iter().enumerate() {
            let Some(prim_rc) = prim_opt else {
                continue;
            };
            let prim = rc_ref!(prim_rc);
            if prim.mode != crate::cube::primitive::MODE_TRIANGLES {
                continue;
            }
            let world = world_per_part[i];
            let base_index = positions.len() as u32;
            for chunk in prim.positions.chunks_exact(3) {
                let local = crate::cube::vec3::Vec3 {
                    x: chunk[0],
                    y: chunk[1],
                    z: chunk[2],
                };
                positions.push(world.mul_vec_value(&local));
            }
            if prim.indices.is_empty() {
                let vert_count = (prim.positions.len() / 3) as u32;
                let mut t = 0u32;
                while t + 2 < vert_count {
                    triangles.push([base_index + t, base_index + t + 1, base_index + t + 2]);
                    t += 3;
                }
            } else {
                for tri in prim.indices.chunks_exact(3) {
                    triangles.push([
                        base_index + tri[0] as u32,
                        base_index + tri[1] as u32,
                        base_index + tri[2] as u32,
                    ]);
                }
            }
        }
        (positions, triangles)
    }

    pub fn validate(&self) -> Result<(), String> {
        let n = self.primitives.len();
        if self.transforms.len() != n
            || self.parents.len() != n
            || (!self.names.is_empty() && self.names.len() != n)
        {
            return Err(format!(
                "Mesh parallel arrays length mismatch: primitives={}, transforms={}, parents={}, names={}",
                n,
                self.transforms.len(),
                self.parents.len(),
                self.names.len(),
            ));
        }
        for (i, &p) in self.parents.iter().enumerate() {
            if p < -1 {
                return Err(format!("Mesh.parents[{i}] = {p} < -1"));
            }
            if p >= i as i32 {
                return Err(format!(
                    "Mesh.parents[{i}] = {p} violates topological order (must be < {i})"
                ));
            }
        }
        Ok(())
    }

    // Compose per-part world transforms by walking parents forward in
    // topological order (parents[i] < i, enforced by validate). `root`
    // is the outer transform applied to every root part. The returned
    // vector has the same length as the parallel arrays.
    pub fn compose_world_transforms(&self, root: &Mat4) -> Vec<Mat4> {
        let n = self.primitives.len();
        let mut world: Vec<Mat4> = Vec::with_capacity(n);
        for i in 0..n {
            let local: Mat4 = *rc_ref!(&self.transforms[i]);
            let combined: Mat4 = if self.parents[i] == -1 {
                root.mul_mat_value(&local)
            } else {
                world[self.parents[i] as usize].mul_mat_value(&local)
            };
            world.push(combined);
        }
        world
    }

    pub fn descendants(&self, root: i32) -> Vec<i32> {
        let n = self.parents.len();
        if root < 0 || (root as usize) >= n {
            return Vec::new();
        }
        let mut in_subtree = vec![false; n];
        in_subtree[root as usize] = true;
        let mut result = Vec::new();
        for j in (root as usize + 1)..n {
            let p = self.parents[j];
            if p >= 0 && in_subtree[p as usize] {
                in_subtree[j] = true;
                result.push(j as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::mat4::Mat4;
    use crate::cube::primitive::Primitive;
    use crate::cube::vec3::Vec3;

    #[test]
    fn test_new_empty() {
        let m = Mesh::new();
        let m = rc_ref!(&m);
        assert!(m.primitives.is_empty());
        assert!(m.transforms.is_empty());
        assert!(m.parents.is_empty());
        assert!(m.motions.is_empty());
        assert!(matches!(m.col_img, ColImage::Color(7)));
        assert!(m.colkey.is_none());
    }

    #[test]
    fn test_validate_topological_order_ok() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new()), Some(Primitive::new())];
            m.transforms = vec![Mat4::identity(), Mat4::identity()];
            m.parents = vec![-1, 0];
        }
        assert!(rc_ref!(&m).validate().is_ok());
    }

    #[test]
    fn test_validate_rejects_forward_parent() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new()), Some(Primitive::new())];
            m.transforms = vec![Mat4::identity(), Mat4::identity()];
            m.parents = vec![1, -1];
        }
        assert!(rc_ref!(&m).validate().is_err());
    }

    #[test]
    fn test_validate_rejects_length_mismatch() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new()), Some(Primitive::new())];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1, 0];
        }
        assert!(rc_ref!(&m).validate().is_err());
    }

    #[test]
    fn test_validate_rejects_invalid_parent_index() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new())];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-2];
        }
        assert!(rc_ref!(&m).validate().is_err());
    }

    #[test]
    fn test_descendants() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![None, None, None, None];
            m.transforms = vec![
                Mat4::identity(),
                Mat4::identity(),
                Mat4::identity(),
                Mat4::identity(),
            ];
            m.parents = vec![-1, 0, 0, 2];
        }
        let m = rc_ref!(&m);
        assert_eq!(m.descendants(0), vec![1, 2, 3]);
        assert_eq!(m.descendants(2), vec![3]);
        assert_eq!(m.descendants(3), Vec::<i32>::new());
    }

    #[test]
    fn test_descendants_out_of_range() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new())];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1];
        }
        let m = rc_ref!(&m);
        assert_eq!(m.descendants(-1), Vec::<i32>::new());
        assert_eq!(m.descendants(5), Vec::<i32>::new());
    }

    #[test]
    fn test_col_img_as_flat_and_image() {
        let ci = ColImage::Color(5);
        let (flat, img) = ci.as_flat_and_image();
        assert_eq!(flat, 5);
        assert!(img.is_none());
    }

    #[test]
    fn test_col_img_as_flat_and_image_for_image_variant() {
        let img = crate::image::Image::new(4, 4);
        let ci = ColImage::Image(img);
        let (flat, img_opt) = ci.as_flat_and_image();
        // Image variant returns flat=0 and the wrapped image.
        assert_eq!(flat, 0);
        assert!(img_opt.is_some());
    }

    #[test]
    fn test_compose_world_transforms_with_rotation() {
        // Parent rotates 90° around Y, child translates +X by 1.
        // Child's world position should land at -Z=1 in world.
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![None, None];
            m.transforms = vec![
                Mat4::from_axis_angle(
                    &Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    90.0,
                ),
                Mat4::from_translation(&Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                }),
            ];
            m.parents = vec![-1, 0];
        }
        let m = rc_ref!(&m);
        let root_rc = Mat4::identity();
        let root = *rc_ref!(&root_rc);
        let world = m.compose_world_transforms(&root);
        let pos1 = world[1].pos();
        let pos1 = rc_ref!(&pos1);
        assert!(pos1.x.abs() < 1e-4);
        assert!((pos1.z - (-1.0)).abs() < 1e-4);
    }

    #[test]
    fn test_compose_world_transforms_single_root() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![None];
            m.transforms = vec![Mat4::from_translation(&crate::cube::vec3::Vec3 {
                x: 5.0,
                y: 0.0,
                z: 0.0,
            })];
            m.parents = vec![-1];
        }
        let m = rc_ref!(&m);
        let root_rc = Mat4::from_translation(&crate::cube::vec3::Vec3 {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        });
        let root: Mat4 = *rc_ref!(&root_rc);
        let world = m.compose_world_transforms(&root);
        // world[0] = root * T(5,0,0) → position (15, 0, 0)
        assert_eq!(world.len(), 1);
        let pos0_rc = world[0].pos();
        let pos0 = rc_ref!(&pos0_rc);
        assert_eq!(pos0.x, 15.0, "pos0.x = {}", pos0.x);
    }

    #[test]
    fn test_compose_world_transforms_chain() {
        // Three-deep chain: root -> 0 -> 1 -> 2, each adds (1, 0, 0)
        // translation. The final part 2 should end at (3, 0, 0) when
        // the outer root is identity.
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![None, None, None];
            let t = Mat4::from_translation(&crate::cube::vec3::Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            });
            m.transforms = vec![t.clone(), t.clone(), t];
            m.parents = vec![-1, 0, 1];
        }
        let m = rc_ref!(&m);
        let root_rc = Mat4::identity();
        let root = *rc_ref!(&root_rc);
        let world = m.compose_world_transforms(&root);
        assert_eq!(world.len(), 3);
        let pos0_rc = world[0].pos();
        let pos1_rc = world[1].pos();
        let pos2_rc = world[2].pos();
        let pos0 = rc_ref!(&pos0_rc);
        let pos1 = rc_ref!(&pos1_rc);
        let pos2 = rc_ref!(&pos2_rc);
        assert!((pos0.x - 1.0).abs() < 1e-5);
        assert!((pos1.x - 2.0).abs() < 1e-5);
        assert!((pos2.x - 3.0).abs() < 1e-5);
    }

    #[test]
    fn test_with_collision_bvh_builds_lazily_and_caches() {
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            let prim = Primitive::new();
            {
                let g = rc_mut!(&prim);
                g.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
                g.indices = vec![0, 1, 2];
            }
            m.primitives = vec![Some(prim)];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1];
        }
        let m = rc_ref!(&m);
        assert!(m.bvh.borrow().is_none());
        let leaf_count =
            m.with_collision_bvh(|bvh| bvh.nodes.iter().filter(|n| n.left == -1).count());
        assert_eq!(leaf_count, 1);
        assert!(m.bvh.borrow().is_some());
    }

    #[test]
    fn test_compose_world_transforms_branching() {
        // Tree: 0 (root) -> 1, 0 -> 2. parts 1 and 2 are siblings.
        let m = Mesh::new();
        {
            let m = rc_mut!(&m);
            m.primitives = vec![None, None, None];
            m.transforms = vec![
                Mat4::identity(),
                Mat4::from_translation(&crate::cube::vec3::Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                }),
                Mat4::from_translation(&crate::cube::vec3::Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }),
            ];
            m.parents = vec![-1, 0, 0];
        }
        let m = rc_ref!(&m);
        let root_rc = Mat4::identity();
        let root = *rc_ref!(&root_rc);
        let world = m.compose_world_transforms(&root);
        assert_eq!(world.len(), 3);
        let pos1_rc = world[1].pos();
        let pos2_rc = world[2].pos();
        let pos1 = rc_ref!(&pos1_rc);
        let pos2 = rc_ref!(&pos2_rc);
        // Siblings inherit identity from root, so each is translated by its own local.
        assert!((pos1.x - 1.0).abs() < 1e-5);
        assert!((pos1.y).abs() < 1e-5);
        assert!((pos2.x).abs() < 1e-5);
        assert!((pos2.y - 1.0).abs() < 1e-5);
    }
}
