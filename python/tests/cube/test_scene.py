import pyxel
import pytest

from pyxel.cube import Collider, Mat4, Node, Scene, Shading, Vec3


def palette() -> list[int]:
    return [pyxel.colors[i] for i in range(16)]


class TestDefault:
    def test_construction(self):
        s = Scene()
        assert s.shading is None
        assert s.clear_color is None
        assert "Scene(" in repr(s)

    def test_is_node_subclass(self):
        # Scene inherits Node so the entire Node API (transform, hierarchy,
        # immediate-mode draw commands, lifecycle hooks) is available.
        assert isinstance(Scene(), Node)


class TestAttributes:
    def test_set_shading(self):
        s = Scene()
        new_shading = Shading(palette())
        s.shading = new_shading
        # Shading.__getitem__ returns (primary, secondary).
        assert s.shading[0, 2] == new_shading[0, 2]

    def test_clear_color_default_none(self):
        assert Scene().clear_color is None

    def test_clear_color_round_trips(self):
        s = Scene()
        s.clear_color = 5
        assert s.clear_color == 5
        s.clear_color = None
        assert s.clear_color is None


class TestUpdate:
    def test_update_no_children(self):
        # Empty scene update must not crash.
        Scene().update()


# Immediate-mode draw commands are no-op outside an active DrawContext
# (i.e., when called outside Scene.draw). The tests confirm they do not
# crash when invoked from outside; functional rendering is exercised by
# integration tests / sample programs.
class TestImmediateDrawSafety:
    def test_pset(self):
        Scene().pset(Vec3.ZERO, 7)

    def test_line(self):
        Scene().line(Vec3.ZERO, Vec3(1, 0, 0), 7)

    def test_tri(self):
        s = Scene()
        s.tri(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 7)
        s.trib(Vec3.ZERO, Vec3(1, 0, 0), Vec3(0, 1, 0), 8)

    def test_circ(self):
        s = Scene()
        s.circ(Vec3.ZERO, 1.0, 7)
        s.circb(Vec3.ZERO, 1.0, 8)

    def test_rect_family(self):
        s = Scene()
        m = Mat4.IDENTITY
        s.rect(m, 1.0, 1.0, 7)
        s.rectb(m, 1.0, 1.0, 8)
        s.elli(m, 1.0, 1.0, 9)
        s.ellib(m, 1.0, 1.0, 10)


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
        scene = Scene()
        a = _CollisionCounter(Vec3(0, 0, 0))
        b = _CollisionCounter(Vec3(0.5, 0, 0))
        scene.add_child(a)
        scene.add_child(b)
        scene.update()
        assert a.collide_count == 1
        assert b.collide_count == 1

    def test_separated_spheres_do_not_collide(self):
        scene = Scene()
        a = _CollisionCounter(Vec3(0, 0, 0))
        b = _CollisionCounter(Vec3(5, 0, 0))
        scene.add_child(a)
        scene.add_child(b)
        scene.update()
        assert a.collide_count == 0
        assert b.collide_count == 0


class TestRaycast:
    def test_raycast_hits_nearer_sphere(self):
        scene = Scene()
        near = _ball(Vec3(0, 0, 0))
        far = _ball(Vec3(0, 0, -5))
        scene.add_child(near)
        scene.add_child(far)
        hit = scene.raycast(Vec3(0, 0, 5), Vec3(0, 0, -1))
        assert hit is not None
        # The near sphere sits at z=0 with radius 0.5; the ray enters
        # its surface at z=0.5, so distance = 5 - 0.5 = 4.5.
        assert hit.distance == pytest.approx(4.5)
        # RaycastHit.node preserves the scene tree's Py<Node> instance
        # (binding mirrors the overlap_* identity path).
        assert hit.node is near
        del far  # silence unused-variable lint

    def test_raycast_returns_none_when_miss(self):
        scene = Scene()
        scene.add_child(_ball(Vec3(0, 0, 0)))
        hit = scene.raycast(Vec3(10, 10, 10), Vec3(1, 0, 0))
        assert hit is None

    def test_raycast_all_sorted_by_distance(self):
        scene = Scene()
        scene.add_child(_ball(Vec3(0, 0, -1)))
        scene.add_child(_ball(Vec3(0, 0, -3)))
        scene.add_child(_ball(Vec3(0, 0, -2)))
        hits = scene.raycast_all(Vec3(0, 0, 5), Vec3(0, 0, -1))
        assert len(hits) == 3
        for i in range(1, len(hits)):
            assert hits[i].distance >= hits[i - 1].distance


class TestOverlapQueries:
    def test_overlap_sphere_finds_overlapping_node(self):
        scene = Scene()
        inside = _ball(Vec3(0, 0, 0))
        outside = _ball(Vec3(10, 0, 0))
        scene.add_child(inside)
        scene.add_child(outside)
        nodes = scene.overlap_sphere(Vec3.ZERO, 1.0)
        assert inside in nodes
        assert outside not in nodes

    def test_overlap_box_finds_overlapping_node(self):
        scene = Scene()
        inside = _ball(Vec3(0, 0, 0))
        outside = _ball(Vec3(10, 0, 0))
        scene.add_child(inside)
        scene.add_child(outside)
        nodes = scene.overlap_box(Mat4.IDENTITY, Vec3(2, 2, 2))
        assert inside in nodes
        assert outside not in nodes

    def test_overlap_sphere_filters_by_tag(self):
        scene = Scene()
        enemy = _ball(Vec3(0, 0, 0))
        enemy.tags = ["enemy"]
        friend = _ball(Vec3(0.5, 0, 0))
        friend.tags = ["friend"]
        scene.add_child(enemy)
        scene.add_child(friend)
        nodes = scene.overlap_sphere(Vec3.ZERO, 1.0, tags=["enemy"])
        assert enemy in nodes
        assert friend not in nodes

    def test_trigger_skipped_by_default(self):
        scene = Scene()
        trigger = _ball(Vec3(0, 0, 0))
        trigger.collider = Collider(radius=0.5, trigger=True)
        scene.add_child(trigger)
        # hit_triggers default is False.
        nodes = scene.overlap_sphere(Vec3.ZERO, 1.0)
        assert trigger not in nodes
        # Opt-in includes the trigger.
        nodes_with_triggers = scene.overlap_sphere(Vec3.ZERO, 1.0, hit_triggers=True)
        assert trigger in nodes_with_triggers
