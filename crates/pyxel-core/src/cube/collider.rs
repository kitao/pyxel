// Empty collider placeholder. The collision-detection pipeline
// (sphere / box / mesh shapes, broad-phase queries, contact generation)
// is deferred to a later iteration; see cube-design.md § 15. The class
// exists so user code can already declare `node.collider = Collider()`
// — the cube runtime simply stores the value and never traverses it.

pub struct Collider;

define_rc_type!(RcCollider, Collider);

impl Collider {
    pub fn new() -> RcCollider {
        new_rc_type!(Collider)
    }
}
