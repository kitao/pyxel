use std::cell::UnsafeCell;
use std::rc::{Rc, Weak};

use crate::cube::collider::RcCollider;
use crate::cube::mat4::{Mat4, RcMat4};
use crate::cube::mesh::RcMesh;
use crate::cube::shading::RcShading;
use crate::cube::vec3::{RcVec3, Vec3};

pub type WeakNode = Weak<UnsafeCell<Node>>;

// Hierarchy instance. Holds local transform, draw / lifecycle state, and
// child links. Parent is a weak ref to avoid Rc cycles between parent and
// children. The optional `attached_mesh` is the mesh placed by
// `Mesh::create_node`; the default `on_draw` will render it at the local
// origin.

pub struct Node {
    pub name: String,
    pub transform: RcMat4,
    pub active: bool,
    pub visible: bool,
    pub shading: Option<RcShading>,
    pub collider: Option<RcCollider>,
    pub tags: Vec<String>,
    pub parent: Option<WeakNode>,
    pub children: Vec<RcNode>,
    pub attached_mesh: Option<RcMesh>,
    // Set by destroy() and cascaded to the subtree. Scene.update step 8
    // (cube-design.md § 16) collects flagged nodes post-order, fires
    // on_destroy, and detaches them. The flag is exposed read-only as
    // Node.destroyed so user hooks can early-return after a destroy().
    pub destroyed: bool,
}

define_rc_type!(RcNode, Node);

impl Node {
    pub fn new() -> RcNode {
        new_rc_type!(Node {
            name: String::new(),
            transform: Mat4::identity(),
            active: true,
            visible: true,
            shading: None,
            collider: None,
            tags: Vec::new(),
            parent: None,
            children: Vec::new(),
            attached_mesh: None,
            destroyed: false,
        })
    }

    // Hierarchy operations

    pub fn add_child(parent: &RcNode, child: &RcNode) {
        Self::detach(child);
        rc_mut!(child).parent = Some(Rc::downgrade(parent));
        rc_mut!(parent).children.push(child.clone());
    }

    pub fn remove_child(parent: &RcNode, child: &RcNode) {
        rc_mut!(child).parent = None;
        rc_mut!(parent).children.retain(|c| !Rc::ptr_eq(c, child));
    }

    pub fn detach(child: &RcNode) {
        let parent_rc = rc_ref!(child).parent.as_ref().and_then(Weak::upgrade);
        if let Some(parent_rc) = parent_rc {
            rc_mut!(&parent_rc)
                .children
                .retain(|c| !Rc::ptr_eq(c, child));
        }
        rc_mut!(child).parent = None;
    }

    // Flag this node and every descendant as destroyed without
    // touching parent / child links. Scene.update step 8 collects
    // the flagged nodes post-order, fires on_destroy, then detaches.
    pub fn destroy(node: &RcNode) {
        Self::mark_destroyed_recursive(node);
    }

    fn mark_destroyed_recursive(node: &RcNode) {
        rc_mut!(node).destroyed = true;
        let children = rc_ref!(node).children.clone();
        for child in &children {
            Self::mark_destroyed_recursive(child);
        }
    }

    pub fn parent(node: &RcNode) -> Option<RcNode> {
        rc_ref!(node).parent.as_ref().and_then(Weak::upgrade)
    }

    pub fn children(node: &RcNode) -> Vec<RcNode> {
        rc_ref!(node).children.clone()
    }

    // Subtree DFS pre-order; returns every node whose `name` matches.
    pub fn find_by_name(start: &RcNode, name: &str) -> Vec<RcNode> {
        let mut out = Vec::new();
        Self::collect_by_name(start, name, &mut out);
        out
    }

    fn collect_by_name(node: &RcNode, name: &str, out: &mut Vec<RcNode>) {
        if rc_ref!(node).name == name {
            out.push(node.clone());
        }
        let children = rc_ref!(node).children.clone();
        for child in &children {
            Self::collect_by_name(child, name, out);
        }
    }

    // Subtree DFS pre-order; returns every node carrying any of `tags`.
    pub fn find_by_tags(start: &RcNode, tags: &[String]) -> Vec<RcNode> {
        let mut out = Vec::new();
        Self::collect_by_tags(start, tags, &mut out);
        out
    }

    fn collect_by_tags(node: &RcNode, tags: &[String], out: &mut Vec<RcNode>) {
        let node_tags = rc_ref!(node).tags.clone();
        if tags.iter().any(|t| node_tags.iter().any(|nt| nt == t)) {
            out.push(node.clone());
        }
        let children = rc_ref!(node).children.clone();
        for child in &children {
            Self::collect_by_tags(child, tags, out);
        }
    }

    // Local transform basis columns, normalized. Pyxel cube convention:
    // forward = -Z axis, right = +X axis, up = +Y axis.
    pub fn forward(node: &RcNode) -> RcVec3 {
        let t_rc = rc_ref!(node).transform.clone();
        let t = rc_ref!(&t_rc);
        let len_sq = t.data[0][2].powi(2) + t.data[1][2].powi(2) + t.data[2][2].powi(2);
        if len_sq < 1e-24 {
            return Vec3::forward();
        }
        let inv = 1.0 / len_sq.sqrt();
        Vec3::new(
            -t.data[0][2] * inv,
            -t.data[1][2] * inv,
            -t.data[2][2] * inv,
        )
    }

    pub fn right(node: &RcNode) -> RcVec3 {
        let t_rc = rc_ref!(node).transform.clone();
        let t = rc_ref!(&t_rc);
        let len_sq = t.data[0][0].powi(2) + t.data[1][0].powi(2) + t.data[2][0].powi(2);
        if len_sq < 1e-24 {
            return Vec3::right();
        }
        let inv = 1.0 / len_sq.sqrt();
        Vec3::new(
            t.data[0][0] * inv,
            t.data[1][0] * inv,
            t.data[2][0] * inv,
        )
    }

    pub fn up(node: &RcNode) -> RcVec3 {
        let t_rc = rc_ref!(node).transform.clone();
        let t = rc_ref!(&t_rc);
        let len_sq = t.data[0][1].powi(2) + t.data[1][1].powi(2) + t.data[2][1].powi(2);
        if len_sq < 1e-24 {
            return Vec3::up();
        }
        let inv = 1.0 / len_sq.sqrt();
        Vec3::new(
            t.data[0][1] * inv,
            t.data[1][1] * inv,
            t.data[2][1] * inv,
        )
    }

    pub fn world_transform(node: &RcNode) -> RcMat4 {
        match Self::parent(node) {
            Some(parent) => {
                let parent_world = Self::world_transform(&parent);
                let local = rc_ref!(node).transform.clone();
                rc_ref!(&parent_world).mul_mat(rc_ref!(&local))
            }
            None => rc_ref!(node).transform.clone(),
        }
    }

    // Effective inheritance: this node's value, or the closest non-None
    // ancestor's value. Used for `shading` cascade.

    pub fn effective_shading(node: &RcNode) -> Option<RcShading> {
        if let Some(s) = rc_ref!(node).shading.clone() {
            return Some(s);
        }
        Self::parent(node).and_then(|p| Self::effective_shading(&p))
    }

    // Effective gating: parent-dominant. False at any ancestor halts the
    // subtree. Used for `active` and `visible` cascade.

    pub fn effective_active(node: &RcNode) -> bool {
        if !rc_ref!(node).active {
            return false;
        }
        match Self::parent(node) {
            Some(parent) => Self::effective_active(&parent),
            None => true,
        }
    }

    pub fn effective_visible(node: &RcNode) -> bool {
        if !rc_ref!(node).visible {
            return false;
        }
        match Self::parent(node) {
            Some(parent) => Self::effective_visible(&parent),
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::vec3::Vec3;

    #[test]
    fn test_default() {
        let n = Node::new();
        let r = rc_ref!(&n);
        assert_eq!(r.name, "");
        assert!(r.active);
        assert!(r.visible);
        assert!(r.parent.is_none());
        assert!(r.children.is_empty());
        assert!(r.attached_mesh.is_none());
    }

    #[test]
    fn test_add_child() {
        let p = Node::new();
        let c = Node::new();
        Node::add_child(&p, &c);
        assert_eq!(rc_ref!(&p).children.len(), 1);
        assert!(Rc::ptr_eq(&Node::parent(&c).unwrap(), &p));
    }

    #[test]
    fn test_destroy_marks_subtree_without_detaching() {
        let root = Node::new();
        let mid = Node::new();
        let leaf = Node::new();
        Node::add_child(&root, &mid);
        Node::add_child(&mid, &leaf);
        Node::destroy(&mid);
        // Flag set on mid + leaf, not on root.
        assert!(!rc_ref!(&root).destroyed);
        assert!(rc_ref!(&mid).destroyed);
        assert!(rc_ref!(&leaf).destroyed);
        // Tree links untouched (deferred removal happens in Scene
        // step 8, not in destroy()).
        assert_eq!(Node::children(&root).len(), 1);
        assert_eq!(Node::children(&mid).len(), 1);
    }

    #[test]
    fn test_reparent() {
        let p1 = Node::new();
        let p2 = Node::new();
        let c = Node::new();
        Node::add_child(&p1, &c);
        Node::add_child(&p2, &c);
        assert_eq!(rc_ref!(&p1).children.len(), 0);
        assert_eq!(rc_ref!(&p2).children.len(), 1);
        assert!(Rc::ptr_eq(&Node::parent(&c).unwrap(), &p2));
    }

    #[test]
    fn test_remove_child() {
        let p = Node::new();
        let c = Node::new();
        Node::add_child(&p, &c);
        Node::remove_child(&p, &c);
        assert!(rc_ref!(&p).children.is_empty());
        assert!(Node::parent(&c).is_none());
    }

    #[test]
    fn test_destroy() {
        let p = Node::new();
        let c = Node::new();
        Node::add_child(&p, &c);
        Node::destroy(&c);
        // Deferred semantics: the flag is set, but parent / child
        // links survive until Scene step 8 detaches them.
        assert!(rc_ref!(&c).destroyed);
        assert_eq!(rc_ref!(&p).children.len(), 1);
        assert!(Node::parent(&c).is_some());
    }

    #[test]
    fn test_find_by_name() {
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).name = "head".to_string();
        rc_mut!(&b).name = "arm".to_string();
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);
        let found = Node::find_by_name(&root, "arm");
        assert_eq!(found.len(), 1);
        assert!(Rc::ptr_eq(&found[0], &b));
    }

    #[test]
    fn test_find_by_name_self() {
        let n = Node::new();
        rc_mut!(&n).name = "self".to_string();
        let found = Node::find_by_name(&n, "self");
        assert_eq!(found.len(), 1);
        assert!(Rc::ptr_eq(&found[0], &n));
    }

    #[test]
    fn test_find_by_name_missing() {
        let n = Node::new();
        assert!(Node::find_by_name(&n, "absent").is_empty());
    }

    #[test]
    fn test_find_by_name_multiple_matches() {
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).name = "zako".to_string();
        rc_mut!(&b).name = "zako".to_string();
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);
        let found = Node::find_by_name(&root, "zako");
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn test_find_by_tags_single() {
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).tags = vec!["enemy".to_string()];
        rc_mut!(&b).tags = vec!["player".to_string()];
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);
        let found = Node::find_by_tags(&root, &["enemy".to_string()]);
        assert_eq!(found.len(), 1);
        assert!(Rc::ptr_eq(&found[0], &a));
    }

    #[test]
    fn test_find_by_tags_any_match() {
        let root = Node::new();
        let a = Node::new();
        rc_mut!(&a).tags = vec!["enemy".to_string(), "boss".to_string()];
        Node::add_child(&root, &a);
        let found = Node::find_by_tags(&root, &["boss".to_string(), "player".to_string()]);
        assert_eq!(found.len(), 1);
        assert!(Rc::ptr_eq(&found[0], &a));
    }

    #[test]
    fn test_forward_identity() {
        let n = Node::new();
        let f = Node::forward(&n);
        let f = rc_ref!(&f);
        assert!((f.x - 0.0).abs() < 1e-4);
        assert!((f.y - 0.0).abs() < 1e-4);
        assert!((f.z - (-1.0)).abs() < 1e-4);
    }

    #[test]
    fn test_right_identity() {
        let n = Node::new();
        let r = Node::right(&n);
        let r = rc_ref!(&r);
        assert!((r.x - 1.0).abs() < 1e-4);
        assert!((r.y - 0.0).abs() < 1e-4);
        assert!((r.z - 0.0).abs() < 1e-4);
    }

    #[test]
    fn test_up_identity() {
        let n = Node::new();
        let u = Node::up(&n);
        let u = rc_ref!(&u);
        assert!((u.x - 0.0).abs() < 1e-4);
        assert!((u.y - 1.0).abs() < 1e-4);
        assert!((u.z - 0.0).abs() < 1e-4);
    }

    #[test]
    fn test_world_transform_root() {
        let n = Node::new();
        rc_mut!(&n).transform = Mat4::from_translation(rc_ref!(&Vec3::new(1.0, 2.0, 3.0)));
        let world = Node::world_transform(&n);
        let world = rc_ref!(&world);
        let pos = world.pos();
        let pos = rc_ref!(&pos);
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);
    }

    #[test]
    fn test_world_transform_nested() {
        let p = Node::new();
        let c = Node::new();
        rc_mut!(&p).transform = Mat4::from_translation(rc_ref!(&Vec3::new(1.0, 0.0, 0.0)));
        rc_mut!(&c).transform = Mat4::from_translation(rc_ref!(&Vec3::new(0.0, 2.0, 0.0)));
        Node::add_child(&p, &c);
        let world = Node::world_transform(&c);
        let world = rc_ref!(&world);
        let pos = world.pos();
        let pos = rc_ref!(&pos);
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
    }

    #[test]
    fn test_effective_active_cascade() {
        let p = Node::new();
        let c = Node::new();
        Node::add_child(&p, &c);
        assert!(Node::effective_active(&c));
        rc_mut!(&p).active = false;
        assert!(!Node::effective_active(&c));
    }

    #[test]
    fn test_effective_visible_cascade() {
        let p = Node::new();
        let c = Node::new();
        Node::add_child(&p, &c);
        assert!(Node::effective_visible(&c));
        rc_mut!(&p).visible = false;
        assert!(!Node::effective_visible(&c));
    }
}
