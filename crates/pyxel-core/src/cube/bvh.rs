use crate::cube::collision::Aabb;
use crate::cube::vec3::Vec3;

// Flat AABB tree for static mesh colliders, built by top-down median split.
// Geometry changes rebuild the tree; nodes are not refitted.

pub struct Bvh {
    pub nodes: Vec<BvhNode>,
    pub triangles: Vec<[u32; 3]>,
    pub positions: Vec<Vec3>,
}

#[derive(Clone, Copy)]
pub struct BvhNode {
    pub aabb: Aabb,
    // -1 == leaf; otherwise index into nodes.
    pub left: i32,
    pub right: i32,
    // Valid only when leaf (left == -1). Range [tri_first, tri_first + tri_count).
    pub tri_first: u32,
    pub tri_count: u32,
}

const MAX_LEAF_TRIANGLES: usize = 1;

// Median splits bound depth by ceil(log2(triangle count)) <= 32 for u32 indices.
// Traversal holds at most depth + 1 entries; 64 avoids per-query allocation.
const QUERY_STACK_CAPACITY: usize = 64;

impl Bvh {
    pub fn build(positions: Vec<Vec3>, triangles: Vec<[u32; 3]>) -> Self {
        if triangles.is_empty() {
            return Self {
                nodes: Vec::new(),
                triangles,
                positions,
            };
        }
        let mut tri_indices: Vec<u32> = (0..triangles.len() as u32).collect();
        let mut nodes: Vec<BvhNode> = Vec::new();
        Self::build_recursive(&positions, &triangles, &mut tri_indices, 0, &mut nodes);
        let mut permuted: Vec<[u32; 3]> = Vec::with_capacity(triangles.len());
        for &idx in &tri_indices {
            permuted.push(triangles[idx as usize]);
        }
        Self {
            nodes,
            triangles: permuted,
            positions,
        }
    }

    // Permute triangle indices into contiguous leaf ranges and return the root index.
    fn build_recursive(
        positions: &[Vec3],
        triangles: &[[u32; 3]],
        tri_indices: &mut [u32],
        offset: usize,
        nodes: &mut Vec<BvhNode>,
    ) -> i32 {
        let aabb = subset_aabb(positions, triangles, tri_indices);
        let n = tri_indices.len();
        let node_index = nodes.len() as i32;
        nodes.push(BvhNode {
            aabb,
            left: -1,
            right: -1,
            tri_first: offset as u32,
            tri_count: n as u32,
        });
        if n <= MAX_LEAF_TRIANGLES {
            return node_index;
        }
        // Median split along the longest extent axis.
        let extent_x = aabb.max.x - aabb.min.x;
        let extent_y = aabb.max.y - aabb.min.y;
        let extent_z = aabb.max.z - aabb.min.z;
        let axis = if extent_x >= extent_y && extent_x >= extent_z {
            0usize
        } else if extent_y >= extent_z {
            1
        } else {
            2
        };
        tri_indices.sort_by(|&a, &b| {
            let ca = triangle_centroid(positions, triangles, a)[axis];
            let cb = triangle_centroid(positions, triangles, b)[axis];
            ca.partial_cmp(&cb).unwrap_or(std::cmp::Ordering::Equal)
        });
        let mid = n / 2;
        let (left_slice, right_slice) = tri_indices.split_at_mut(mid);
        let left = Self::build_recursive(positions, triangles, left_slice, offset, nodes);
        let right = Self::build_recursive(positions, triangles, right_slice, offset + mid, nodes);
        nodes[node_index as usize].left = left;
        nodes[node_index as usize].right = right;
        node_index
    }

    // Visit triangles whose leaf AABBs overlap the query.
    pub fn query_aabb(&self, query: &Aabb, mut visit: impl FnMut([u32; 3])) {
        if self.nodes.is_empty() {
            return;
        }
        // stack[0] == 0 seeds the root.
        let mut stack = [0_i32; QUERY_STACK_CAPACITY];
        let mut top = 1_usize;
        while top > 0 {
            top -= 1;
            let node = self.nodes[stack[top] as usize];
            if !node.aabb.overlaps(query) {
                continue;
            }
            if node.left == -1 {
                let start = node.tri_first as usize;
                let end = start + node.tri_count as usize;
                for tri in &self.triangles[start..end] {
                    visit(*tri);
                }
            } else {
                stack[top] = node.left;
                stack[top + 1] = node.right;
                top += 2;
            }
        }
    }

    // Walk the tree and call `visit` for every triangle whose owning
    // leaf's AABB the ray can reach within `max_t`. `direction` need not
    // be normalized: `max_t` is in units of the direction's length, so a
    // ray mapped into mesh-local space keeps its world t parameterization.
    pub fn query_ray(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_t: f32,
        mut visit: impl FnMut([u32; 3]),
    ) {
        if self.nodes.is_empty() {
            return;
        }
        let inv_dir = Vec3 {
            x: 1.0 / direction.x,
            y: 1.0 / direction.y,
            z: 1.0 / direction.z,
        };
        // stack[0] == 0 seeds the root.
        let mut stack = [0_i32; QUERY_STACK_CAPACITY];
        let mut top = 1_usize;
        while top > 0 {
            top -= 1;
            let node = self.nodes[stack[top] as usize];
            if !ray_reaches_aabb(origin, inv_dir, &node.aabb, max_t) {
                continue;
            }
            if node.left == -1 {
                let start = node.tri_first as usize;
                let end = start + node.tri_count as usize;
                for tri in &self.triangles[start..end] {
                    visit(*tri);
                }
            } else {
                stack[top] = node.left;
                stack[top + 1] = node.right;
                top += 2;
            }
        }
    }
}

// Conservative slab test for BVH pruning within [0, max_t].
fn ray_reaches_aabb(origin: Vec3, inv_dir: Vec3, aabb: &Aabb, max_t: f32) -> bool {
    let (tx_enter, tx_exit) = ray_slab_interval(origin.x, inv_dir.x, aabb.min.x, aabb.max.x);
    let (ty_enter, ty_exit) = ray_slab_interval(origin.y, inv_dir.y, aabb.min.y, aabb.max.y);
    let (tz_enter, tz_exit) = ray_slab_interval(origin.z, inv_dir.z, aabb.min.z, aabb.max.z);
    let t_enter = tx_enter.max(ty_enter).max(tz_enter).max(0.0);
    let t_exit = tx_exit.min(ty_exit).min(tz_exit).min(max_t);
    t_enter <= t_exit
}

#[inline]
fn ray_slab_interval(origin: f32, inv_dir: f32, min: f32, max: f32) -> (f32, f32) {
    let t1 = (min - origin) * inv_dir;
    let t2 = (max - origin) * inv_dir;
    // A zero-direction ray on a slab face has a NaN product but stays in the slab.
    if t1.is_nan() || t2.is_nan() {
        (f32::NEG_INFINITY, f32::INFINITY)
    } else {
        (t1.min(t2), t1.max(t2))
    }
}

fn subset_aabb(positions: &[Vec3], triangles: &[[u32; 3]], indices: &[u32]) -> Aabb {
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
    for &tri_idx in indices {
        let tri = triangles[tri_idx as usize];
        for &vi in &tri {
            let p = positions[vi as usize];
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }
    }
    Aabb { min, max }
}

fn triangle_centroid(positions: &[Vec3], triangles: &[[u32; 3]], tri_idx: u32) -> [f32; 3] {
    let tri = triangles[tri_idx as usize];
    let a = positions[tri[0] as usize];
    let b = positions[tri[1] as usize];
    let c = positions[tri[2] as usize];
    [
        (a.x + b.x + c.x) / 3.0,
        (a.y + b.y + c.y) / 3.0,
        (a.z + b.z + c.z) / 3.0,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_mesh_build() {
        let bvh = Bvh::build(Vec::new(), Vec::new());
        assert!(bvh.nodes.is_empty());
        assert_eq!(bvh.triangles, [] as [[u32; 3]; 0]);
    }

    #[test]
    fn test_single_triangle_build_produces_one_leaf() {
        let positions = vec![
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ];
        let triangles = vec![[0u32, 1, 2]];
        let bvh = Bvh::build(positions, triangles);
        assert_eq!(bvh.nodes.len(), 1);
        assert_eq!(bvh.nodes[0].tri_count, 1);
        assert_eq!(bvh.nodes[0].left, -1);
    }

    #[test]
    fn test_two_separated_triangles_split_into_two_leaves() {
        let positions = vec![
            // Triangle A at x ≈ 0
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            // Triangle B at x ≈ 100
            Vec3 {
                x: 100.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 101.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 100.0,
                y: 1.0,
                z: 0.0,
            },
        ];
        let triangles = vec![[0u32, 1, 2], [3, 4, 5]];
        let bvh = Bvh::build(positions, triangles);
        assert_eq!(bvh.nodes.len(), 3);
        assert_ne!(bvh.nodes[0].left, -1);
        assert!(bvh.nodes[0].aabb.min.x <= 0.0);
        assert!(bvh.nodes[0].aabb.max.x >= 101.0);
    }

    #[test]
    fn test_root_aabb_contains_all_triangle_vertices() {
        let positions = vec![
            Vec3 {
                x: -5.0,
                y: -2.0,
                z: 0.0,
            },
            Vec3 {
                x: 5.0,
                y: -2.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 3.0,
                z: 0.0,
            },
            Vec3 {
                x: -5.0,
                y: -2.0,
                z: 10.0,
            },
            Vec3 {
                x: 5.0,
                y: -2.0,
                z: 10.0,
            },
            Vec3 {
                x: 0.0,
                y: 3.0,
                z: 10.0,
            },
        ];
        let triangles = vec![[0u32, 1, 2], [3, 4, 5]];
        let bvh = Bvh::build(positions, triangles);
        let root = bvh.nodes[0].aabb;
        assert!(root.min.x <= -5.0 && root.max.x >= 5.0);
        assert!(root.min.y <= -2.0 && root.max.y >= 3.0);
        assert!(root.min.z <= 0.0 && root.max.z >= 10.0);
    }

    #[test]
    fn test_query_returns_overlapping_triangle_only() {
        let positions = vec![
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 100.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 101.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 100.0,
                y: 1.0,
                z: 0.0,
            },
        ];
        let triangles = vec![[0u32, 1, 2], [3, 4, 5]];
        let bvh = Bvh::build(positions, triangles);
        let query = Aabb {
            min: Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            max: Vec3 {
                x: 2.0,
                y: 2.0,
                z: 1.0,
            },
        };
        let mut hits = Vec::new();
        bvh.query_aabb(&query, |tri| hits.push(tri));
        assert_eq!(hits, [[0, 1, 2]]);
    }

    #[test]
    fn test_query_returns_empty_when_aabb_misses_all() {
        let positions = vec![
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ];
        let bvh = Bvh::build(positions, vec![[0u32, 1, 2]]);
        let query = Aabb {
            min: Vec3 {
                x: 100.0,
                y: 100.0,
                z: 100.0,
            },
            max: Vec3 {
                x: 101.0,
                y: 101.0,
                z: 101.0,
            },
        };
        let mut hits = 0;
        bvh.query_aabb(&query, |_| hits += 1);
        assert_eq!(hits, 0);
    }

    fn unit_triangle(offset: Vec3) -> ([Vec3; 3], [u32; 3]) {
        (
            [
                offset,
                Vec3 {
                    x: offset.x + 1.0,
                    y: offset.y,
                    z: offset.z,
                },
                Vec3 {
                    x: offset.x,
                    y: offset.y + 1.0,
                    z: offset.z,
                },
            ],
            [0u32, 1, 2],
        )
    }

    #[test]
    fn test_multi_triangle_build_produces_single_triangle_leaves() {
        let mut positions: Vec<Vec3> = Vec::new();
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        for i in 0..4 {
            let (verts, tri) = unit_triangle(Vec3 {
                x: i as f32 * 10.0,
                y: 0.0,
                z: 0.0,
            });
            let base = positions.len() as u32;
            positions.extend_from_slice(&verts);
            triangles.push([tri[0] + base, tri[1] + base, tri[2] + base]);
        }
        let bvh = Bvh::build(positions, triangles);
        let leaf_count = bvh.nodes.iter().filter(|n| n.left == -1).count();
        assert_eq!(leaf_count, 4);
    }

    #[test]
    fn test_query_hits_every_triangle_when_aabb_contains_all() {
        let mut positions: Vec<Vec3> = Vec::new();
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        for i in 0..3 {
            let (verts, tri) = unit_triangle(Vec3 {
                x: i as f32,
                y: 0.0,
                z: 0.0,
            });
            let base = positions.len() as u32;
            positions.extend_from_slice(&verts);
            triangles.push([tri[0] + base, tri[1] + base, tri[2] + base]);
        }
        let bvh = Bvh::build(positions, triangles);
        let query = Aabb {
            min: Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            max: Vec3 {
                x: 10.0,
                y: 10.0,
                z: 10.0,
            },
        };
        let mut hits = Vec::new();
        bvh.query_aabb(&query, |tri| hits.push(tri));
        hits.sort_unstable();
        assert_eq!(hits, [[0, 1, 2], [3, 4, 5], [6, 7, 8]]);
    }

    fn two_separated_triangles() -> Bvh {
        let positions = vec![
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 100.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 101.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 100.0,
                y: 1.0,
                z: 0.0,
            },
        ];
        Bvh::build(positions, vec![[0u32, 1, 2], [3, 4, 5]])
    }

    #[test]
    fn test_query_ray_prunes_off_axis_leaf() {
        let bvh = two_separated_triangles();
        let mut hits = Vec::new();
        bvh.query_ray(
            Vec3 {
                x: 0.25,
                y: 0.25,
                z: -5.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            f32::INFINITY,
            |tri| hits.push(tri),
        );
        assert_eq!(hits, [[0, 1, 2]]);
    }

    #[test]
    fn test_query_ray_visits_both_leaves_along_x() {
        let bvh = two_separated_triangles();
        let mut hits = Vec::new();
        bvh.query_ray(
            Vec3 {
                x: -1.0,
                y: 0.5,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            f32::INFINITY,
            |tri| hits.push(tri),
        );
        hits.sort_unstable();
        assert_eq!(hits, [[0, 1, 2], [3, 4, 5]]);
    }

    #[test]
    fn test_query_ray_max_t_prunes_far_leaf() {
        let bvh = two_separated_triangles();
        let mut hits = 0;
        bvh.query_ray(
            Vec3 {
                x: -1.0,
                y: 0.5,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            10.0,
            |_| hits += 1,
        );
        assert_eq!(hits, 1);
    }

    #[test]
    fn test_query_ray_negative_direction_reaches_leaf() {
        let bvh = two_separated_triangles();
        let mut hits = Vec::new();
        bvh.query_ray(
            Vec3 {
                x: 200.0,
                y: 0.5,
                z: 0.0,
            },
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            f32::INFINITY,
            |tri| hits.push(tri),
        );
        hits.sort_unstable();
        assert_eq!(hits, [[0, 1, 2], [3, 4, 5]]);
    }

    #[test]
    fn test_query_ray_behind_origin_is_pruned() {
        let bvh = two_separated_triangles();
        let mut hits = 0;
        bvh.query_ray(
            Vec3 {
                x: -10.0,
                y: 0.5,
                z: 0.0,
            },
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            f32::INFINITY,
            |_| hits += 1,
        );
        assert_eq!(hits, 0);
    }

    #[test]
    fn test_query_ray_origin_inside_leaf_aabb() {
        let bvh = two_separated_triangles();
        let mut hits = 0;
        bvh.query_ray(
            Vec3 {
                x: 0.5,
                y: 0.25,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            f32::INFINITY,
            |_| hits += 1,
        );
        assert_eq!(hits, 1);
    }

    #[test]
    fn test_query_ray_includes_parallel_faces_with_signed_zero() {
        let bvh = Bvh::build(
            vec![
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
            ],
            vec![[0, 1, 2]],
        );
        for travel_axis in 0..3 {
            for face_axis in 0..3 {
                if travel_axis == face_axis {
                    continue;
                }
                for face in [0.0, 1.0] {
                    for zero in [0.0, -0.0] {
                        let mut origin = [0.5; 3];
                        origin[travel_axis] = -1.0;
                        origin[face_axis] = face;
                        let origin = Vec3 {
                            x: origin[0],
                            y: origin[1],
                            z: origin[2],
                        };
                        let mut direction = [zero; 3];
                        direction[travel_axis] = 1.0;
                        let direction = Vec3 {
                            x: direction[0],
                            y: direction[1],
                            z: direction[2],
                        };
                        for (max_t, expected) in [(0.5, 0), (1.0, 1), (2.0, 1)] {
                            let mut hits = 0;
                            bvh.query_ray(origin, direction, max_t, |_| hits += 1);
                            assert_eq!(
                                hits, expected,
                                "travel_axis={travel_axis}, face_axis={face_axis}, face={face}, zero={zero}, max_t={max_t}",
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_query_ray_parallel_to_degenerate_aabbs() {
        for travel_axis in 0..3 {
            for extent in [[1.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0; 3]] {
                let mut max = [0.0; 3];
                for i in 0..3 {
                    max[(travel_axis + i) % 3] = extent[i];
                }
                let min = Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let max = Vec3 {
                    x: max[0],
                    y: max[1],
                    z: max[2],
                };
                let bvh = Bvh::build(vec![min, max, min], vec![[0, 1, 2]]);
                for zero in [0.0, -0.0] {
                    let mut origin = [0.0; 3];
                    origin[travel_axis] = -1.0;
                    let origin = Vec3 {
                        x: origin[0],
                        y: origin[1],
                        z: origin[2],
                    };
                    let mut direction = [zero; 3];
                    direction[travel_axis] = 1.0;
                    let direction = Vec3 {
                        x: direction[0],
                        y: direction[1],
                        z: direction[2],
                    };
                    for (max_t, expected) in [(0.5, 0), (1.0, 1)] {
                        let mut hits = 0;
                        bvh.query_ray(origin, direction, max_t, |_| hits += 1);
                        assert_eq!(
                            hits, expected,
                            "axis={travel_axis}, extent={extent:?}, zero={zero}, max_t={max_t}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_query_ray_parallel_outside_aabb_is_pruned() {
        let bvh = two_separated_triangles();
        for y in [-1.0, 2.0] {
            for zero in [0.0, -0.0] {
                let mut hits = 0;
                bvh.query_ray(
                    Vec3 { x: -1.0, y, z: 0.0 },
                    Vec3 {
                        x: 1.0,
                        y: zero,
                        z: zero,
                    },
                    200.0,
                    |_| hits += 1,
                );
                assert_eq!(hits, 0, "y={y}, zero={zero}");
            }
        }
    }

    #[test]
    fn test_split_axis_uses_longest_extent_y() {
        // An X split groups columns; a Y split groups rows.
        let mut positions = Vec::new();
        let mut triangles = Vec::new();
        for (x, y) in [(0.0, 0.0), (0.0, 100.0), (10.0, 0.0), (10.0, 100.0)] {
            let (verts, _) = unit_triangle(Vec3 { x, y, z: 0.0 });
            let base = positions.len() as u32;
            positions.extend_from_slice(&verts);
            triangles.push([base, base + 1, base + 2]);
        }
        let bvh = Bvh::build(positions, triangles);
        let root = bvh.nodes[0];
        let left = bvh.nodes[root.left as usize];
        let right = bvh.nodes[root.right as usize];
        assert_eq!(left.tri_count, 2);
        assert_eq!(right.tri_count, 2);
        assert_eq!(left.aabb.min.y, 0.0);
        assert_eq!(left.aabb.max.y, 1.0);
        assert_eq!(right.aabb.min.y, 100.0);
        assert_eq!(right.aabb.max.y, 101.0);
        assert_eq!(
            bvh.triangles,
            [[0, 1, 2], [6, 7, 8], [3, 4, 5], [9, 10, 11]]
        );
    }
}
