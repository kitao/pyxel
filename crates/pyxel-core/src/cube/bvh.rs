use crate::cube::collision::Aabb;
use crate::cube::vec3::Vec3;

// Flat AABB tree for static mesh colliders. Build is one-shot; the
// tree is never refit. PS1-scale meshes (~1000 triangles) use top-down
// median split — SAH offers no measurable improvement at that size.

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

// Threshold for stopping recursion. Each leaf holds at most one
// triangle, giving predictable per-query cost on PS1-scale meshes
// (~1000 triangles) without the bookkeeping cost of a larger leaf.
const MAX_LEAF_TRIANGLES: usize = 1;

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

    // Returns the index of the node it just emitted into `nodes`.
    // `tri_indices` is the subslice of triangle indices this subtree
    // owns; it is permuted in place so that the final flat permutation
    // (= caller's mutable view of the original slice) yields contiguous
    // leaf ranges.
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

    // Walk the tree and call `visit` for every triangle whose owning
    // leaf's AABB overlaps the query AABB. Branches that miss the
    // query AABB are pruned without recursion.
    pub fn query_aabb(&self, query: &Aabb, mut visit: impl FnMut([u32; 3])) {
        if self.nodes.is_empty() {
            return;
        }
        let mut stack: Vec<i32> = vec![0];
        while let Some(idx) = stack.pop() {
            let node = self.nodes[idx as usize];
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
                stack.push(node.left);
                stack.push(node.right);
            }
        }
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
        assert!(bvh.triangles.is_empty());
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
        // Root + two leaves
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
        assert_eq!(hits.len(), 1);
        let t = hits[0];
        let v0 = bvh.positions[t[0] as usize];
        assert!(v0.x < 50.0);
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
        // 4 separated triangles → 4 leaves (MAX_LEAF_TRIANGLES = 1).
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
        let mut hits = 0;
        bvh.query_aabb(&query, |_| hits += 1);
        assert_eq!(hits, 3);
    }

    #[test]
    fn test_split_axis_uses_longest_extent_y() {
        // Triangles stretched along Y so the median split picks Y. The
        // two resulting leaves' AABBs must lie on opposite sides of the
        // Y midpoint, confirming the split happened on the Y axis (an
        // X-axis split would leave both leaves spanning the full Y
        // range).
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
                x: 0.0,
                y: 100.0,
                z: 0.0,
            },
            Vec3 {
                x: 1.0,
                y: 100.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 101.0,
                z: 0.0,
            },
        ];
        let triangles = vec![[0u32, 1, 2], [3, 4, 5]];
        let bvh = Bvh::build(positions, triangles);
        // Root AABB Y extent (~101) should dominate X / Z.
        let root = bvh.nodes[0].aabb;
        let ex = root.max.x - root.min.x;
        let ey = root.max.y - root.min.y;
        let ez = root.max.z - root.min.z;
        assert!(ey > ex && ey > ez);
        let leaf_aabbs: Vec<_> = bvh.nodes.iter().filter(|n| n.left == -1).collect();
        assert_eq!(leaf_aabbs.len(), 2);
        // Leaf 0 wraps y≈0..1, leaf 1 wraps y≈100..101 (or vice versa).
        // Either way the leaves must not overlap in Y, which is the
        // signature of a Y-axis split.
        let (a, b) = (leaf_aabbs[0].aabb, leaf_aabbs[1].aabb);
        let y_disjoint = a.max.y < b.min.y || b.max.y < a.min.y;
        assert!(
            y_disjoint,
            "leaves overlap in Y; split did not occur on Y axis: a=({}..{}) b=({}..{})",
            a.min.y, a.max.y, b.min.y, b.max.y,
        );
    }
}
