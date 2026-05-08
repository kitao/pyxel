use std::cell::UnsafeCell;
use std::rc::{Rc, Weak};

use crate::cube::collider::RcCollider;
use crate::cube::light::RcLight;
use crate::cube::mat4::{Mat4, RcMat4};
use crate::cube::mesh::RcMesh;
use crate::cube::shade_ramp::RcShadeRamp;

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
    pub light: Option<RcLight>,
    pub shade_ramp: Option<RcShadeRamp>,
    pub collider: Option<RcCollider>,
    pub parent: Option<WeakNode>,
    pub children: Vec<RcNode>,
    pub attached_mesh: Option<RcMesh>,
}

define_rc_type!(RcNode, Node);

impl Node {
    pub fn new() -> RcNode {
        new_rc_type!(Node {
            name: String::new(),
            transform: Mat4::identity(),
            active: true,
            visible: true,
            light: None,
            shade_ramp: None,
            collider: None,
            parent: None,
            children: Vec::new(),
            attached_mesh: None,
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

    pub fn destroy(node: &RcNode) {
        Self::detach(node);
        // Subtree links remain intact for callers that still hold references;
        // dropping the last Rc reclaims memory naturally.
    }

    pub fn parent(node: &RcNode) -> Option<RcNode> {
        rc_ref!(node).parent.as_ref().and_then(Weak::upgrade)
    }

    pub fn children(node: &RcNode) -> Vec<RcNode> {
        rc_ref!(node).children.clone()
    }

    // Subtree DFS pre-order, matching `self` first.
    pub fn find(start: &RcNode, name: &str) -> Option<RcNode> {
        if rc_ref!(start).name == name {
            return Some(start.clone());
        }
        for child in &rc_ref!(start).children {
            if let Some(found) = Self::find(child, name) {
                return Some(found);
            }
        }
        None
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
    // ancestor's value. Used for `light` and `shade_ramp` cascade.

    pub fn effective_light(node: &RcNode) -> Option<RcLight> {
        if let Some(l) = rc_ref!(node).light.clone() {
            return Some(l);
        }
        Self::parent(node).and_then(|p| Self::effective_light(&p))
    }

    pub fn effective_shade_ramp(node: &RcNode) -> Option<RcShadeRamp> {
        if let Some(r) = rc_ref!(node).shade_ramp.clone() {
            return Some(r);
        }
        Self::parent(node).and_then(|p| Self::effective_shade_ramp(&p))
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
        assert!(rc_ref!(&p).children.is_empty());
        assert!(Node::parent(&c).is_none());
    }

    #[test]
    fn test_find() {
        let root = Node::new();
        let a = Node::new();
        let b = Node::new();
        rc_mut!(&a).name = "head".to_string();
        rc_mut!(&b).name = "arm".to_string();
        Node::add_child(&root, &a);
        Node::add_child(&root, &b);
        let found = Node::find(&root, "arm").unwrap();
        assert!(Rc::ptr_eq(&found, &b));
    }

    #[test]
    fn test_find_self() {
        let n = Node::new();
        rc_mut!(&n).name = "self".to_string();
        let found = Node::find(&n, "self").unwrap();
        assert!(Rc::ptr_eq(&found, &n));
    }

    #[test]
    fn test_find_missing() {
        let n = Node::new();
        assert!(Node::find(&n, "absent").is_none());
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
