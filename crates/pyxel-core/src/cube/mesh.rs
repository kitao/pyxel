use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::cube::bvh::Bvh;
use crate::cube::collision::Aabb;
use crate::cube::mat4::{Mat4, RcMat4};
use crate::cube::motion::RcMotion;
use crate::cube::primitive::{RcPrimitive, MODE_TRIANGLES};
use crate::cube::vec3::Vec3;
use crate::image::RcImage;

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

#[derive(Clone)]
pub struct Material {
    pub col_img: ColImage,
    pub colkey: Option<i32>,
}

// Part arrays are parallel; validate() enforces parents[i] < i for topological traversal.
// Per-part material slots override the shared col_img.
pub struct Mesh {
    pub primitives: Vec<Option<RcPrimitive>>,
    pub transforms: Vec<RcMat4>,
    pub parents: Vec<i32>,
    pub names: Vec<String>,
    pub motions: Vec<RcMotion>,
    pub col_img: ColImage,
    pub colkey: Option<i32>,
    pub materials: Vec<Material>,
    pub material_indices: Vec<Option<usize>>,
    // Lazy collision BVH. Built on first mesh-collider query and invalidated
    // when geometry or transforms change.
    pub bvh: RefCell<Option<Bvh>>,
    // Shares BVH invalidation; Aabb::from_mesh transforms its eight corners
    // instead of every vertex.
    pub local_aabb: RefCell<Option<Aabb>>,
    collision_geometry_dirty: Arc<AtomicBool>,
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
            materials: Vec::new(),
            material_indices: Vec::new(),
            bvh: RefCell::new(None),
            local_aabb: RefCell::new(None),
            collision_geometry_dirty: Arc::new(AtomicBool::new(true)),
        })
    }

    pub fn from_glb(filename: &str, colkey: Option<i32>, fps: f32) -> Result<RcMesh, String> {
        crate::cube::glb_parser::parse_glb(filename, colkey, fps)
    }

    // Queries must use mesh-local space, including the composed part transforms.
    pub fn with_collision_bvh<R>(&self, f: impl FnOnce(&Bvh) -> R) -> R {
        self.refresh_collision_geometry();
        if self.bvh.borrow().is_none() {
            let (positions, triangles) = self.collect_triangles();
            *self.bvh.borrow_mut() = Some(Bvh::build(positions, triangles));
        }
        let guard = self.bvh.borrow();
        f(guard.as_ref().unwrap())
    }

    // Include every primitive mode; an empty mesh yields a point at the local origin.
    pub fn local_aabb(&self) -> Aabb {
        self.refresh_collision_geometry();
        if let Some(aabb) = *self.local_aabb.borrow() {
            return aabb;
        }
        let identity = Mat4::identity_value();
        let world_per_part = self.compose_world_transforms(&identity);
        let mut min = Vec3 {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        };
        let mut max = Vec3 {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
            z: f32::NEG_INFINITY,
        };
        let mut any = false;
        for (i, prim_opt) in self.primitives.iter().enumerate() {
            let Some(prim_rc) = prim_opt else {
                continue;
            };
            let prim = rc_ref!(prim_rc);
            let world = world_per_part[i];
            for chunk in prim.positions.as_chunks::<3>().0 {
                let p = world.mul_vec_value(&Vec3 {
                    x: chunk[0],
                    y: chunk[1],
                    z: chunk[2],
                });
                min.x = min.x.min(p.x);
                min.y = min.y.min(p.y);
                min.z = min.z.min(p.z);
                max.x = max.x.max(p.x);
                max.y = max.y.max(p.y);
                max.z = max.z.max(p.z);
                any = true;
            }
        }
        if !any {
            let origin = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            min = origin;
            max = origin;
        }
        let aabb = Aabb { min, max };
        *self.local_aabb.borrow_mut() = Some(aabb);
        aabb
    }

    fn refresh_collision_geometry(&self) {
        if !self.collision_geometry_dirty.load(Ordering::Relaxed) {
            return;
        }
        *self.bvh.borrow_mut() = None;
        *self.local_aabb.borrow_mut() = None;
        for primitive in self.primitives.iter().flatten() {
            rc_mut!(primitive).register_collision_dependent(&self.collision_geometry_dirty);
        }
        self.collision_geometry_dirty
            .store(false, Ordering::Relaxed);
    }

    pub fn reset_collision_geometry_tracking(&mut self) {
        self.collision_geometry_dirty = Arc::new(AtomicBool::new(true));
        *self.bvh.get_mut() = None;
        *self.local_aabb.get_mut() = None;
    }

    fn collect_triangles(&self) -> (Vec<Vec3>, Vec<[u32; 3]>) {
        let identity = Mat4::identity_value();
        let world_per_part = self.compose_world_transforms(&identity);
        let mut positions: Vec<Vec3> = Vec::new();
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        // Merge transformed triangle primitives into one mesh-local stream
        for (i, prim_opt) in self.primitives.iter().enumerate() {
            let Some(prim_rc) = prim_opt else {
                continue;
            };
            let prim = rc_ref!(prim_rc);
            if prim.mode != MODE_TRIANGLES {
                continue;
            }
            let world = world_per_part[i];
            let base_index = positions.len() as u32;
            for chunk in prim.positions.as_chunks::<3>().0 {
                let local = Vec3 {
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
                // Skip out-of-range indices, including negative values cast to usize.
                let vert_count = prim.positions.len() / 3;
                for tri in prim.indices.as_chunks::<3>().0 {
                    let i0 = tri[0] as usize;
                    let i1 = tri[1] as usize;
                    let i2 = tri[2] as usize;
                    if i0 >= vert_count || i1 >= vert_count || i2 >= vert_count {
                        continue;
                    }
                    triangles.push([
                        base_index + i0 as u32,
                        base_index + i1 as u32,
                        base_index + i2 as u32,
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
            || (!self.material_indices.is_empty() && self.material_indices.len() != n)
        {
            return Err(format!(
                "Mesh parallel arrays length mismatch: primitives={}, transforms={}, parents={}, names={}, material_indices={}",
                n,
                self.transforms.len(),
                self.parents.len(),
                self.names.len(),
                self.material_indices.len(),
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
        for (i, material_index) in self.material_indices.iter().enumerate() {
            if let Some(material_index) = material_index {
                if *material_index >= self.materials.len() {
                    return Err(format!(
                        "Mesh.material_indices[{i}] = {material_index} is out of range for material count {}",
                        self.materials.len(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn material_for_part(&self, part_index: usize) -> Material {
        if let Some(Some(material_index)) = self.material_indices.get(part_index) {
            return self.materials[*material_index].clone();
        }
        Material {
            col_img: self.col_img.clone(),
            colkey: self.colkey,
        }
    }

    // Topological order lets children reuse their already-composed parent transforms.
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
        assert_eq!(m.motions, [] as [RcMotion; 0]);
        assert!(m.colkey.is_none());
        assert_eq!(m.materials.len(), 0);
        assert_eq!(m.material_indices, [] as [Option<usize>; 0]);
    }

    #[test]
    fn test_validate_topological_order_ok() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
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
            let mut m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new()), Some(Primitive::new())];
            m.transforms = vec![Mat4::identity(), Mat4::identity()];
            m.parents = vec![1, -1];
        }
        assert_eq!(
            rc_ref!(&m).validate().unwrap_err(),
            "Mesh.parents[0] = 1 violates topological order (must be < 0)"
        );
    }

    #[test]
    fn test_validate_rejects_length_mismatch() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new()), Some(Primitive::new())];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1, 0];
        }
        assert_eq!(
            rc_ref!(&m).validate().unwrap_err(),
            "Mesh parallel arrays length mismatch: primitives=2, transforms=1, parents=2, names=0, material_indices=0"
        );
    }

    #[test]
    fn test_validate_rejects_material_index_length_mismatch() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new()), Some(Primitive::new())];
            m.transforms = vec![Mat4::identity(), Mat4::identity()];
            m.parents = vec![-1, 0];
            m.material_indices = vec![None];
        }
        assert_eq!(
            rc_ref!(&m).validate().unwrap_err(),
            "Mesh parallel arrays length mismatch: primitives=2, transforms=2, parents=2, names=0, material_indices=1"
        );
    }

    #[test]
    fn test_validate_rejects_material_index_out_of_range() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new())];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1];
            m.material_indices = vec![Some(0)];
        }
        assert_eq!(
            rc_ref!(&m).validate().unwrap_err(),
            "Mesh.material_indices[0] = 0 is out of range for material count 0"
        );
    }

    #[test]
    fn test_validate_rejects_invalid_parent_index() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
            m.primitives = vec![Some(Primitive::new())];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-2];
        }
        assert_eq!(
            rc_ref!(&m).validate().unwrap_err(),
            "Mesh.parents[0] = -2 < -1"
        );
    }

    #[test]
    fn test_descendants() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
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
            let mut m = rc_mut!(&m);
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
        assert_eq!(flat, 0);
        assert!(img_opt.is_some());
    }

    #[test]
    fn test_material_for_part_defaults_to_mesh_material() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
            m.col_img = ColImage::Color(5);
            m.colkey = Some(0);
        }
        let material = rc_ref!(&m).material_for_part(0);

        assert!(matches!(material.col_img, ColImage::Color(5)));
        assert_eq!(material.colkey, Some(0));
    }

    #[test]
    fn test_material_for_part_uses_part_material() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
            m.materials = vec![Material {
                col_img: ColImage::Color(8),
                colkey: Some(1),
            }];
            m.material_indices = vec![Some(0)];
        }
        let material = rc_ref!(&m).material_for_part(0);

        assert!(matches!(material.col_img, ColImage::Color(8)));
        assert_eq!(material.colkey, Some(1));
    }

    #[test]
    fn test_compose_world_transforms_with_rotation() {
        // The parent's Y rotation maps the child's +X translation to world -Z.
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
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
            let mut m = rc_mut!(&m);
            m.primitives = vec![None];
            m.transforms = vec![Mat4::from_translation(&Vec3 {
                x: 5.0,
                y: 0.0,
                z: 0.0,
            })];
            m.parents = vec![-1];
        }
        let m = rc_ref!(&m);
        let root_rc = Mat4::from_translation(&Vec3 {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        });
        let root: Mat4 = *rc_ref!(&root_rc);
        let world = m.compose_world_transforms(&root);
        assert_eq!(world.len(), 1);
        let pos0_rc = world[0].pos();
        let pos0 = rc_ref!(&pos0_rc);
        assert_eq!(pos0.x, 15.0, "pos0.x = {}", pos0.x);
    }

    #[test]
    fn test_compose_world_transforms_chain() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
            m.primitives = vec![None, None, None];
            let t = Mat4::from_translation(&Vec3 {
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
        assert_eq!(pos0.x, 1.0);
        assert_eq!(pos1.x, 2.0);
        assert_eq!(pos2.x, 3.0);
    }

    #[test]
    fn test_primitive_geometry_change_invalidates_collision_caches() {
        let m = Mesh::new();
        let prim = Primitive::new();
        {
            let mut g = rc_mut!(&prim);
            g.positions = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];
            g.indices = vec![0, 1, 2];
        }
        {
            let mut m = rc_mut!(&m);
            m.primitives = vec![Some(prim.clone())];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1];
        }

        let m = rc_ref!(&m);
        assert_eq!(m.local_aabb().max.x, 1.0);
        let first_x = m.with_collision_bvh(|bvh| bvh.positions[0].x);
        assert_eq!(first_x, -1.0);

        {
            let mut primitive = rc_mut!(&prim);
            for position in primitive.positions.as_chunks_mut::<3>().0 {
                position[0] += 100.0;
            }
            primitive.mark_collision_geometry_changed();
        }

        assert_eq!(m.local_aabb().min.x, 99.0);
        let moved_x = m.with_collision_bvh(|bvh| bvh.positions[0].x);
        assert_eq!(moved_x, 99.0);
    }

    #[test]
    fn test_collision_cache_steady_state_does_not_borrow_primitive() {
        let primitive = Primitive::new();
        {
            let mut primitive_ref = rc_mut!(&primitive);
            primitive_ref.positions = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];
            primitive_ref.indices = vec![0, 1, 2];
        }
        let mesh = Mesh::new();
        {
            let mut mesh_ref = rc_mut!(&mesh);
            mesh_ref.primitives = vec![Some(primitive.clone())];
            mesh_ref.transforms = vec![Mat4::identity()];
            mesh_ref.parents = vec![-1];
        }

        let mesh_ref = rc_ref!(&mesh);
        assert!(mesh_ref.bvh.borrow().is_none());
        assert_eq!(mesh_ref.local_aabb().max.x, 1.0);
        assert_eq!(mesh_ref.with_collision_bvh(|bvh| bvh.positions.len()), 3);
        assert!(mesh_ref.bvh.borrow().is_some());
        assert_eq!(
            mesh_ref.with_collision_bvh(|bvh| bvh.nodes.iter().filter(|n| n.left == -1).count()),
            1
        );

        let _primitive_ref = rc_mut!(&primitive);
        assert_eq!(mesh_ref.local_aabb().max.x, 1.0);
        assert_eq!(mesh_ref.with_collision_bvh(|bvh| bvh.positions.len()), 3);
    }

    #[test]
    fn test_unrelated_primitive_change_does_not_invalidate_collision_cache() {
        let changed_primitive = Primitive::new();
        let unchanged_primitive = Primitive::new();
        for primitive in [&changed_primitive, &unchanged_primitive] {
            let mut primitive_ref = rc_mut!(primitive);
            primitive_ref.positions = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];
            primitive_ref.indices = vec![0, 1, 2];
        }
        let changed_mesh = Mesh::new();
        let unchanged_mesh = Mesh::new();
        for (mesh, primitive) in [
            (&changed_mesh, &changed_primitive),
            (&unchanged_mesh, &unchanged_primitive),
        ] {
            let mut mesh_ref = rc_mut!(mesh);
            mesh_ref.primitives = vec![Some(primitive.clone())];
            mesh_ref.transforms = vec![Mat4::identity()];
            mesh_ref.parents = vec![-1];
        }

        assert_eq!(rc_ref!(&changed_mesh).local_aabb().max.x, 1.0);
        assert_eq!(rc_ref!(&unchanged_mesh).local_aabb().max.x, 1.0);
        {
            let mut primitive_ref = rc_mut!(&changed_primitive);
            primitive_ref.positions[0] = -2.0;
            primitive_ref.mark_collision_geometry_changed();
        }

        let _primitive_ref = rc_mut!(&unchanged_primitive);
        assert_eq!(rc_ref!(&unchanged_mesh).local_aabb().max.x, 1.0);
    }

    #[test]
    fn test_shared_primitive_change_invalidates_every_collision_cache() {
        let primitive = Primitive::new();
        {
            let mut primitive_ref = rc_mut!(&primitive);
            primitive_ref.positions = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];
            primitive_ref.indices = vec![0, 1, 2];
        }
        let first_mesh = Mesh::new();
        let second_mesh = Mesh::new();
        for mesh in [&first_mesh, &second_mesh] {
            let mut mesh_ref = rc_mut!(mesh);
            mesh_ref.primitives = vec![Some(primitive.clone())];
            mesh_ref.transforms = vec![Mat4::identity()];
            mesh_ref.parents = vec![-1];
        }

        for mesh in [&first_mesh, &second_mesh] {
            let mesh_ref = rc_ref!(mesh);
            assert_eq!(mesh_ref.local_aabb().max.x, 1.0);
            assert_eq!(mesh_ref.with_collision_bvh(|bvh| bvh.positions[0].x), -1.0);
        }
        {
            let mut primitive_ref = rc_mut!(&primitive);
            for position in primitive_ref.positions.as_chunks_mut::<3>().0 {
                position[0] += 100.0;
            }
            primitive_ref.mark_collision_geometry_changed();
        }

        for mesh in [&first_mesh, &second_mesh] {
            let mesh_ref = rc_ref!(mesh);
            assert_eq!(mesh_ref.local_aabb().min.x, 99.0);
            assert_eq!(mesh_ref.with_collision_bvh(|bvh| bvh.positions[0].x), 99.0);
        }
    }

    #[test]
    fn test_replacing_primitives_rebinds_collision_invalidation() {
        let first_primitive = Primitive::new();
        let second_primitive = Primitive::new();
        {
            let mut primitive_ref = rc_mut!(&first_primitive);
            primitive_ref.positions = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];
            primitive_ref.indices = vec![0, 1, 2];
        }
        {
            let mut primitive_ref = rc_mut!(&second_primitive);
            primitive_ref.positions = vec![9.0, -1.0, 0.0, 11.0, -1.0, 0.0, 10.0, 1.0, 0.0];
            primitive_ref.indices = vec![0, 1, 2];
        }
        let mesh = Mesh::new();
        {
            let mut mesh_ref = rc_mut!(&mesh);
            mesh_ref.primitives = vec![Some(first_primitive.clone())];
            mesh_ref.transforms = vec![Mat4::identity()];
            mesh_ref.parents = vec![-1];
        }

        assert_eq!(rc_ref!(&mesh).local_aabb().min.x, -1.0);
        {
            let mut mesh_ref = rc_mut!(&mesh);
            mesh_ref.primitives[0] = Some(second_primitive.clone());
            mesh_ref.reset_collision_geometry_tracking();
        }
        assert_eq!(rc_ref!(&mesh).local_aabb().min.x, 9.0);

        {
            let mut primitive_ref = rc_mut!(&first_primitive);
            primitive_ref.positions[0] = -100.0;
            primitive_ref.mark_collision_geometry_changed();
        }
        {
            let _second_primitive_ref = rc_mut!(&second_primitive);
            assert_eq!(rc_ref!(&mesh).local_aabb().min.x, 9.0);
        }
        {
            let mut primitive_ref = rc_mut!(&second_primitive);
            for position in primitive_ref.positions.as_chunks_mut::<3>().0 {
                position[0] += 100.0;
            }
            primitive_ref.mark_collision_geometry_changed();
        }
        assert_eq!(rc_ref!(&mesh).local_aabb().min.x, 109.0);
    }

    #[test]
    fn test_with_collision_bvh_skips_out_of_range_indices() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
            let prim = Primitive::new();
            {
                let mut g = rc_mut!(&prim);
                g.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
                g.indices = vec![0, 1, 2, 0, 1, 99, 0, -1, 2];
            }
            m.primitives = vec![Some(prim)];
            m.transforms = vec![Mat4::identity()];
            m.parents = vec![-1];
        }
        let m = rc_ref!(&m);
        let leaf_count =
            m.with_collision_bvh(|bvh| bvh.nodes.iter().filter(|n| n.left == -1).count());
        assert_eq!(leaf_count, 1);
    }

    #[test]
    fn test_compose_world_transforms_branching() {
        let m = Mesh::new();
        {
            let mut m = rc_mut!(&m);
            m.primitives = vec![None, None, None];
            m.transforms = vec![
                Mat4::identity(),
                Mat4::from_translation(&Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                }),
                Mat4::from_translation(&Vec3 {
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
        assert_eq!((pos1.x, pos1.y), (1.0, 0.0));
        assert_eq!((pos2.x, pos2.y), (0.0, 1.0));
    }
}
