import pytest

import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]

from pyxel.cube import (
    Camera,
    Collider,
    Mat4,
    Mesh,
    Node,
    Primitive,
    RaycastHit,
    Shading,
    Vec3,
)

# Frame-level pipeline (update + draw) and spatial queries are tested
# here against the universal Node API. The camera attaches to the tree
# via Node.camera (cascading to descendants); clear_color is a Camera
# attribute.


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


class _StaticCollisionCounter(Node):
    def __init__(self, pos: Vec3, *, trigger: bool = False):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.collider = Collider(radius=0.5, mass=0.0, trigger=trigger)
        self.collide_count = 0
        self.depths = []

    def on_collide(self, other, contact):
        del other
        self.collide_count += 1
        self.depths.append(contact.depth)


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

    def test_static_trigger_still_notifies_without_response(self):
        root = Node()
        sensor = _StaticCollisionCounter(Vec3(0, 0, 0), trigger=True)
        wall = _StaticCollisionCounter(Vec3(0.5, 0, 0))
        root.add_child(sensor)
        root.add_child(wall)
        root.update()
        assert sensor.collide_count == 1
        assert wall.collide_count == 1
        assert sensor.depths == [0.0]
        assert wall.depths == [0.0]


class TestMeshColliderRobustness:
    def test_bad_primitive_indices_do_not_crash_collision(self):
        # Out-of-range and negative indices in a hand-built mesh collider
        # are dropped at the lazy BVH build; the collision pipeline and
        # raycast must run without raising instead of panicking.
        prim = Primitive(
            Primitive.MODE_TRIANGLES,
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            [0, 1, 99, 0, -1, 2],
        )
        mesh = Mesh(primitives=[prim], transforms=[Mat4.IDENTITY], parents=[-1])
        root = Node()
        terrain = Node()
        terrain.collider = Collider(mesh=mesh, mass=0.0)
        root.add_child(terrain)
        ball = _ball(Vec3(0, 0.4, 0))
        root.add_child(ball)

        root.update()

        # Both mesh triangles are invalid, so the ray passes through the
        # terrain and hits the ball beneath it.
        hit = root.raycast(Vec3(0, 5, 0), Vec3(0, -1, 0))
        assert hit is not None
        assert hit.node is ball


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

    def test_raycast_distance_uses_world_units_for_non_unit_direction(self):
        root = Node()
        root.add_child(_ball(Vec3(0, 0, 0)))
        hit = root.raycast(Vec3(0, 0, 5), Vec3(0, 0, -2))
        assert hit is not None
        assert hit.distance == pytest.approx(4.5)
        assert root.raycast(Vec3(0, 0, 5), Vec3(0, 0, -2), max_distance=3.0) is None

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

    def test_raycasthit_not_user_constructible(self):
        # RaycastHit is an engine-built payload with no public constructor.
        with raises_exact(TypeError, "cannot create 'builtins.RaycastHit' instances"):
            RaycastHit()


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


class _ColoredBox(Node):
    def __init__(self, pos: Vec3, col: int):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.col = col

    def on_draw(self):
        self.shaded(False)
        self.box(Mat4.IDENTITY, Vec3(4, 4, 4), self.col)


class TestOrthoCameraClipping:
    def test_geometry_behind_ortho_camera_is_not_drawn(self):
        # The orthographic w row is constant 1, so behind-camera clipping
        # comes from the camera clip row; the box behind the camera must
        # not paint over the one in front.
        scene = Node()
        camera = Camera()
        camera.ortho_size = 10.0
        camera.clear_color = 0
        scene.camera = camera
        scene.add_child(_ColoredBox(Vec3(0, 0, -8), 11))
        scene.add_child(_ColoredBox(Vec3(0, 0, 8), 8))

        scene.draw(0, 0, pyxel.width, pyxel.height)

        assert pyxel.pget(pyxel.width // 2, pyxel.height // 2) == 11


# State set in one Node.on_draw must not leak to siblings or children.
class TestStateSetterIsolation:
    @staticmethod
    def _camera():
        camera = Camera()
        camera.clear_color = 0
        camera.transform = Mat4.look_at(Vec3(0, 0, 4), Vec3.ZERO, Vec3.UP)
        return camera

    def test_sibling_isolation(self):
        class A(Node):
            def on_draw(self):
                self.dither(0.0)

        class B(Node):
            def on_draw(self):
                self.shaded(False)
                self.pset(Vec3.ZERO, 8)

        root = Node()
        root.camera = self._camera()
        root.add_child(A())
        root.add_child(B())
        pyxel.cls(0)
        root.draw(0, 0, 160, 120)

        assert pyxel.pget(80, 60) == 8

    def test_child_isolation(self):
        class Parent(Node):
            def on_draw(self):
                self.dither(0.0)

        class Child(Node):
            def on_draw(self):
                self.shaded(False)
                self.pset(Vec3.ZERO, 8)

        root = Node()
        root.camera = self._camera()
        parent = Parent()
        parent.add_child(Child())
        root.add_child(parent)
        pyxel.cls(0)
        root.draw(0, 0, 160, 120)

        assert pyxel.pget(80, 60) == 8
