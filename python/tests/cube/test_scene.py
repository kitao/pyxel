import pyxel
import pytest

from pyxel.cube import Camera, Collider, Mat4, Node, Shading, Vec3

# Frame-level pipeline (update + draw) and spatial queries are tested
# here against the universal Node API. Camera is a separately held
# instance; clear_color is passed as a draw() argument.


def palette() -> list[int]:
    return [pyxel.colors[i] for i in range(16)]


class TestUpdate:
    def test_update_no_children(self):
        # Empty subtree update must not crash.
        Node().update()


# Immediate-mode draw commands are no-op outside an active DrawContext
# (i.e., when called outside Node.draw). The tests confirm they do not
# crash when invoked from outside; functional rendering is exercised by
# integration tests / sample programs.
class TestImmediateDrawSafety:
    def test_pset(self):
        Node().pset(Vec3.ZERO, 7)

    def test_line(self):
        Node().line(Vec3.ZERO, Vec3(1, 0, 0), 7)

    def test_tri(self):
        n = Node()
        n.tri(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 7)
        n.trib(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 8)

    def test_circ(self):
        n = Node()
        n.circ(Vec3.ZERO, 1.0, 7)
        n.circb(Vec3.ZERO, 1.0, 8)

    def test_rect_family(self):
        n = Node()
        m = Mat4.IDENTITY
        n.rect(m, 1.0, 1.0, 7)
        n.rectb(m, 1.0, 1.0, 8)
        n.elli(m, 1.0, 1.0, 9)
        n.ellib(m, 1.0, 1.0, 10)


# Collision pipeline smoke tests. The detailed geometric correctness
# lives in the Rust unit tests under crates/pyxel-core/src/cube/; these
# verify that the Python-facing API plumbs the call through end-to-end.


def _ball(pos: Vec3, *, radius: float = 0.5, mass: float = 1.0) -> Node:
    n = Node()
    n.transform = Mat4.from_translation(pos)
    n.collider = Collider(radius=radius, mass=mass)
    return n


class _CollisionCounter(Node):
    def __init__(self, pos: Vec3):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.collider = Collider(radius=0.5)
        self.collide_count = 0

    def on_collide(self, other, contact):
        del other, contact
        self.collide_count += 1


class TestCollisionPipeline:
    def test_overlapping_spheres_fire_on_collide(self):
        # Two spheres at distance 0.5 with radius 0.5 each → overlap.
        root = Node()
        a = _CollisionCounter(Vec3(0, 0, 0))
        b = _CollisionCounter(Vec3(0.5, 0, 0))
        root.add_child(a)
        root.add_child(b)
        root.update()
        assert a.collide_count == 1
        assert b.collide_count == 1

    def test_separated_spheres_do_not_collide(self):
        root = Node()
        a = _CollisionCounter(Vec3(0, 0, 0))
        b = _CollisionCounter(Vec3(5, 0, 0))
        root.add_child(a)
        root.add_child(b)
        root.update()
        assert a.collide_count == 0
        assert b.collide_count == 0


class TestRaycast:
    def test_raycast_hits_nearer_sphere(self):
        root = Node()
        near = _ball(Vec3(0, 0, 0))
        far = _ball(Vec3(0, 0, -5))
        root.add_child(near)
        root.add_child(far)
        hit = root.raycast(Vec3(0, 0, 5), Vec3(0, 0, -1))
        assert hit is not None
        # The near sphere sits at z=0 with radius 0.5; the ray enters
        # its surface at z=0.5, so distance = 5 - 0.5 = 4.5.
        assert hit.distance == pytest.approx(4.5)
        # RaycastHit.node preserves the tree's Py<Node> instance
        # (binding mirrors the overlap_* identity path).
        assert hit.node is near
        del far  # silence unused-variable lint

    def test_raycast_returns_none_when_miss(self):
        root = Node()
        root.add_child(_ball(Vec3(0, 0, 0)))
        hit = root.raycast(Vec3(10, 10, 10), Vec3(1, 0, 0))
        assert hit is None

    def test_raycast_all_sorted_by_distance(self):
        root = Node()
        root.add_child(_ball(Vec3(0, 0, -1)))
        root.add_child(_ball(Vec3(0, 0, -3)))
        root.add_child(_ball(Vec3(0, 0, -2)))
        hits = root.raycast_all(Vec3(0, 0, 5), Vec3(0, 0, -1))
        assert len(hits) == 3
        for i in range(1, len(hits)):
            assert hits[i].distance >= hits[i - 1].distance


class TestOverlapQueries:
    def test_overlap_sphere_finds_overlapping_node(self):
        root = Node()
        inside = _ball(Vec3(0, 0, 0))
        outside = _ball(Vec3(10, 0, 0))
        root.add_child(inside)
        root.add_child(outside)
        nodes = root.overlap_sphere(Vec3.ZERO, 1.0)
        assert inside in nodes
        assert outside not in nodes

    def test_overlap_box_finds_overlapping_node(self):
        root = Node()
        inside = _ball(Vec3(0, 0, 0))
        outside = _ball(Vec3(10, 0, 0))
        root.add_child(inside)
        root.add_child(outside)
        nodes = root.overlap_box(Mat4.IDENTITY, Vec3(2, 2, 2))
        assert inside in nodes
        assert outside not in nodes

    def test_overlap_sphere_filters_by_tag(self):
        root = Node()
        enemy = _ball(Vec3(0, 0, 0))
        enemy.tags = ["enemy"]
        friend = _ball(Vec3(0.5, 0, 0))
        friend.tags = ["friend"]
        root.add_child(enemy)
        root.add_child(friend)
        nodes = root.overlap_sphere(Vec3.ZERO, 1.0, tags=["enemy"])
        assert enemy in nodes
        assert friend not in nodes

    def test_trigger_skipped_by_default(self):
        root = Node()
        trigger = _ball(Vec3(0, 0, 0))
        trigger.collider = Collider(radius=0.5, trigger=True)
        root.add_child(trigger)
        # hit_triggers default is False.
        nodes = root.overlap_sphere(Vec3.ZERO, 1.0)
        assert trigger not in nodes
        # Opt-in includes the trigger.
        nodes_with_triggers = root.overlap_sphere(Vec3.ZERO, 1.0, hit_triggers=True)
        assert trigger in nodes_with_triggers


class TestShading:
    def test_set_shading(self):
        n = Node()
        new_shading = Shading(palette())
        n.shading = new_shading
        # Shading.__getitem__ returns (primary, secondary).
        assert n.shading[0, 2] == new_shading[0, 2]


class TestStateSetterIsolation:
    """State set in one Node.on_draw must not leak to siblings or children.

    Pixel-level verification of the isolation contract is by manual
    visual inspection. The smoke tests below confirm the dispatch
    wiring does not raise when state is mutated in mid-on_draw and
    again in sibling/child on_draw bodies.
    """

    def test_sibling_isolation_runs_without_error(self):
        class A(Node):
            def on_draw(self):
                self.dither(0.5)
                self.depth_test(False)
                self.box(Mat4.IDENTITY, Vec3(1, 1, 1), 7)

        class B(Node):
            def on_draw(self):
                self.shaded(False)
                self.box(Mat4.IDENTITY, Vec3(1, 1, 1), 8)

        root = Node()
        cam = Camera()
        root.add_child(A())
        root.add_child(B())
        root.draw(0, 0, 64, 64, cam)

    def test_child_isolation_runs_without_error(self):
        class Parent(Node):
            def on_draw(self):
                self.depth_write(False)
                self.box(Mat4.IDENTITY, Vec3(1, 1, 1), 7)

        class Child(Node):
            def on_draw(self):
                self.box(Mat4.IDENTITY, Vec3(1, 1, 1), 8)

        root = Node()
        cam = Camera()
        parent = Parent()
        parent.add_child(Child())
        root.add_child(parent)
        root.draw(0, 0, 64, 64, cam)
