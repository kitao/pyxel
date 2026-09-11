import gc
import weakref

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
    Vec3,
)


class TestUpdate:
    def test_update_no_children(self):
        Node().update()


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


class _RoundedBox(Node):
    def __init__(self, transform, *, mass=1.0, velocity=Vec3.ZERO):
        super().__init__()
        self.transform = transform
        self.collider = Collider(
            size=Vec3(2, 2, 2), radius=1.0, mass=mass, velocity=velocity
        )
        self.contacts = []

    def on_collide(self, other, contact):
        del other
        self.contacts.append((contact.depth, tuple(contact.normal)))


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

    def test_static_trigger_notifies_with_zero_depth(self):
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

    @pytest.mark.parametrize("rotated", [False, True])
    @pytest.mark.parametrize(
        "center, depth, normal",
        [
            ((3.5, 3.5, 0), None, None),
            ((3.5, 0, 0), 0.5, (-1, 0, 0)),
            ((3, 3, 3), 2 - 3**0.5, (-1 / 3**0.5,) * 3),
        ],
    )
    def test_rounded_boxes_use_euclidean_corner_distance(
        self, rotated, center, depth, normal
    ):
        transform = (
            Mat4.from_translation(Vec3(3, -2, 1))
            * Mat4.from_axis_angle(Vec3.FORWARD, 37)
            if rotated
            else Mat4.IDENTITY
        )
        root = Node()
        body = _RoundedBox(transform)
        wall = _RoundedBox(transform * Mat4.from_translation(Vec3(*center)), mass=0)
        root.add_child(body)
        root.add_child(wall)

        root.update()

        # Core-box gaps are (1.5, 1.5, 0), (1.5, 0, 0), or (1, 1, 1).
        # Their Euclidean length, not each gap alone, is compared with radius 2.
        if depth is None:
            assert body.contacts == []
            assert wall.contacts == []
        else:
            assert len(body.contacts) == len(wall.contacts) == 1
            # The mass-zero wall leaves the body the full geometric correction.
            # Unit-scale f32 geometry and rigid transforms need a small tolerance.
            assert body.contacts[0][0] == pytest.approx(depth, abs=1e-4)
            assert body.contacts[0][1] == pytest.approx(
                tuple(Vec3(*normal).to_world_dir(transform)), abs=1e-4
            )
            assert wall.contacts[0][0] == 0.0

    @pytest.mark.parametrize(
        "offset, body_speed, wall_speed, hits",
        [(3.5, 10, 0, False), (3, 10, 0, True), (3, 8, -2, True)],
    )
    def test_swept_rounded_boxes_use_corner_distance(
        self, offset, body_speed, wall_speed, hits
    ):
        root = Node()
        body = _RoundedBox(
            Mat4.from_translation(Vec3(-5, offset, offset)),
            velocity=Vec3(body_speed, 0, 0),
        )
        wall = _RoundedBox(Mat4.IDENTITY, mass=0, velocity=Vec3(wall_speed, 0, 0))
        root.add_child(body)
        root.add_child(wall)

        root.update()

        # Both endpoints are separated. Relative x motion is 10 in all cases;
        # the transverse gap is sqrt(2) * (offset - 2), with combined radius 2.
        if not hits:
            assert body.contacts == []
            assert wall.contacts == []
        else:
            assert len(body.contacts) == len(wall.contacts) == 1
            # First contact has core separation (-sqrt(2), 1, 1), of length 2.
            assert body.contacts[0][1] == pytest.approx(
                (-(2**0.5) / 2, 0.5, 0.5), abs=1e-4
            )


class TestMeshColliderRobustness:
    @staticmethod
    def _make_triangle_mesh_scene():
        primitive = Primitive(
            Primitive.MODE_TRIANGLES,
            [-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0],
            [0, 1, 2],
        )
        mesh = Mesh(
            primitives=[primitive],
            transforms=[Mat4.IDENTITY],
            parents=[-1],
        )

        terrain = Node()
        terrain.collider = Collider(mesh=mesh, mass=0.0)
        root = Node()
        root.add_child(terrain)
        return primitive, root

    def test_bad_primitive_indices_do_not_crash_collision(self):
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
        hit = root.raycast(Vec3(0, 5, 0), Vec3(0, -1, 0))
        assert hit is not None
        assert hit.node is ball

    def test_primitive_position_mutation_invalidates_mesh_collision_cache(self):
        primitive, root = self._make_triangle_mesh_scene()
        origin = Vec3(0.0, 0.0, 1.0)
        direction = Vec3(0.0, 0.0, -1.0)
        assert root.raycast(origin, direction) is not None

        primitive.positions[::3] = [value + 100.0 for value in primitive.positions[::3]]
        assert root.raycast(origin, direction) is None

    def test_primitive_index_mutation_invalidates_mesh_collision_cache(self):
        primitive, root = self._make_triangle_mesh_scene()
        origin = Vec3(0.0, 0.0, 1.0)
        direction = Vec3(0.0, 0.0, -1.0)
        assert root.raycast(origin, direction) is not None

        primitive.indices[:] = []
        assert root.raycast(origin, direction) is not None

        primitive.indices[:] = [0, 1, 99]
        assert root.raycast(origin, direction) is None

    def test_primitive_mode_mutation_invalidates_mesh_collision_cache(self):
        primitive, root = self._make_triangle_mesh_scene()
        origin = Vec3(0.0, 0.0, 1.0)
        direction = Vec3(0.0, 0.0, -1.0)
        assert root.raycast(origin, direction) is not None

        primitive.mode = Primitive.MODE_LINES
        assert root.raycast(origin, direction) is None

    @pytest.mark.parametrize("method", ["raycast", "raycast_all"])
    @pytest.mark.parametrize("y", [-1.0, 1.0])
    @pytest.mark.parametrize("zero", [0.0, -0.0])
    def test_raycast_reaches_mesh_boundary(self, method, y, zero):
        _, root = self._make_triangle_mesh_scene()
        query = getattr(root, method)
        origin = Vec3(0.0, y, 1.0)
        direction = Vec3(zero, zero, -1.0)

        result = query(origin, direction, max_distance=1.0)
        if method == "raycast_all":
            assert len(result) == 1
            hit = result[0]
        else:
            hit = result
            assert hit is not None
        assert hit.node is root.children[0]
        assert hit.distance == 1.0
        assert (hit.point.x, hit.point.y, hit.point.z) == (0.0, y, 0.0)

        assert query(origin, direction, max_distance=0.5) == (
            [] if method == "raycast_all" else None
        )


class TestRaycast:
    def test_raycast_hits_nearer_sphere(self):
        root = Node()
        near = _ball(Vec3(0, 0, 0))
        far = _ball(Vec3(0, 0, -5))
        root.add_child(far)
        root.add_child(near)

        hit = root.raycast(Vec3(0, 0, 5), Vec3(0, 0, -1))
        assert hit is not None
        # The near sphere sits at z=0 with radius 0.5; the ray enters
        # its surface at z=0.5, so distance = 5 - 0.5 = 4.5.
        assert hit.distance == 4.5
        assert hit.node is near

    def test_raycast_distance_uses_world_units_for_non_unit_direction(self):
        root = Node()
        root.add_child(_ball(Vec3(0, 0, 0)))
        hit = root.raycast(Vec3(0, 0, 5), Vec3(0, 0, -2))
        assert hit is not None
        assert hit.distance == 4.5
        assert root.raycast(Vec3(0, 0, 5), Vec3(0, 0, -2), max_distance=3.0) is None

    @pytest.mark.parametrize("method", ["raycast", "raycast_all"])
    @pytest.mark.parametrize("rotated", [False, True])
    @pytest.mark.parametrize(
        "size, origin, direction, point, normal",
        [
            ((0, 2, 0), (0, 0, 0), (0, 1, 0), (0, 1.5, 0), (0, 1, 0)),
            ((0, 2, 0), (0, 1, 0), (0, -1, 0), (0, -1.5, 0), (0, -1, 0)),
            ((0, 2, 0), (0, 0, 0), (1, 0, 0), (0.5, 0, 0), (1, 0, 0)),
            ((0, 2, 0), (0, 3, 0), (0, -1, 0), (0, 1.5, 0), (0, 1, 0)),
            ((2, 2, 2), (0, 1, 0), (1, 0, 0), (1.5, 1, 0), (1, 0, 0)),
            ((2, 2, 2), (0, 0, 0), (1, 0, 0), (1.5, 0, 0), (1, 0, 0)),
            (
                (2, 2, 2),
                (0, 0, 0),
                (1, 1, 1),
                (1 + 0.5 / 3**0.5,) * 3,
                (1 / 3**0.5,) * 3,
            ),
            ((2, 2, 2), (3, 0, 0), (-1, 0, 0), (1.5, 0, 0), (1, 0, 0)),
        ],
    )
    def test_raycast_uses_outer_capsule_and_rounded_box_surfaces(
        self, method, rotated, size, origin, direction, point, normal
    ):
        root = Node()
        body = Node()
        body.collider = Collider(size=Vec3(*size), radius=0.5)
        transform = (
            Mat4.from_translation(Vec3(3, -2, 1))
            * Mat4.from_axis_angle(Vec3.FORWARD, 37)
            if rotated
            else Mat4.IDENTITY
        )
        body.transform = transform
        root.add_child(body)
        origin = Vec3(*origin)
        point = Vec3(*point)
        distance = (point - origin).length()
        direction = Vec3(*direction).to_world_dir(transform)
        origin = transform * origin

        query = getattr(root, method)
        result = query(origin, direction)
        hit = result[0] if method == "raycast_all" else result
        assert hit is not None
        assert hit.node is body
        if method == "raycast_all":
            assert len(result) == 1
        # Rigid transforms and the diagonal corner introduce unit-scale f32 rounding.
        assert hit.distance == pytest.approx(distance, abs=1e-5)
        assert tuple(hit.point) == pytest.approx(tuple(transform * point), abs=1e-5)
        assert tuple(hit.normal) == pytest.approx(
            tuple(Vec3(*normal).to_world_dir(transform)), abs=1e-5
        )
        assert query(origin, direction, max_distance=distance - 0.01) == (
            [] if method == "raycast_all" else None
        )

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
        assert [hit.distance for hit in hits] == [5.5, 6.5, 7.5]

    def test_raycast_hit_node_cycle_is_collectable(self):
        class Ball(Node):
            pass

        root = Node()
        ball = Ball()
        ball.transform = Mat4.from_translation(Vec3.ZERO)
        ball.collider = Collider(radius=0.5)
        root.add_child(ball)

        hit = root.raycast(Vec3(0, 0, 5), Vec3(0, 0, -1))
        assert hit is not None
        assert gc.is_tracked(hit)

        root.remove_child(ball)
        ball.hit = hit
        ball_ref = weakref.ref(ball)
        del ball, hit, root

        gc.collect()
        assert ball_ref() is None

    def test_raycasthit_not_user_constructible(self):
        with raises_exact(TypeError, "cannot create 'pyxel.cube.RaycastHit' instances"):
            RaycastHit()


class TestOverlapQueries:
    def test_overlap_sphere_finds_overlapping_node(self):
        root = Node()
        inside = _ball(Vec3(0, 0, 0))
        outside = _ball(Vec3(10, 0, 0))
        root.add_child(inside)
        root.add_child(outside)

        nodes = root.overlap_sphere(Vec3.ZERO, 1.0)
        assert nodes == [inside]

    def test_overlap_box_finds_overlapping_node(self):
        root = Node()
        inside = _ball(Vec3(0, 0, 0))
        outside = _ball(Vec3(10, 0, 0))
        root.add_child(inside)
        root.add_child(outside)

        nodes = root.overlap_box(Mat4.IDENTITY, Vec3(2, 2, 2))
        assert nodes == [inside]

    @pytest.mark.parametrize("rotated", [False, True])
    @pytest.mark.parametrize(
        "center, overlaps",
        [((2.75, 2.75, 0), False), ((2.5, 0, 0), True), ((2.5, 2.5, 2.5), True)],
    )
    def test_overlap_box_uses_rounded_target_corner_distance(
        self, rotated, center, overlaps
    ):
        transform = (
            Mat4.from_translation(Vec3(3, -2, 1))
            * Mat4.from_axis_angle(Vec3.FORWARD, 37)
            if rotated
            else Mat4.IDENTITY
        )
        root = Node()
        target = _RoundedBox(transform * Mat4.from_translation(Vec3(*center)))
        root.add_child(target)

        # The sharp query contributes no radius. Its core distance to the target
        # is sqrt(2) * 0.75, 0.5, or sqrt(3) * 0.5; the target radius is 1.
        assert root.overlap_box(transform, Vec3(2, 2, 2)) == (
            [target] if overlaps else []
        )

    def test_overlap_sphere_filters_by_tag(self):
        root = Node()
        enemy = _ball(Vec3(0, 0, 0))
        enemy.tags = ["enemy"]
        friend = _ball(Vec3(0.5, 0, 0))
        friend.tags = ["friend"]
        root.add_child(enemy)
        root.add_child(friend)

        nodes = root.overlap_sphere(Vec3.ZERO, 1.0, tags=["enemy"])
        assert nodes == [enemy]

    def test_trigger_skipped_by_default(self):
        root = Node()
        trigger = _ball(Vec3(0, 0, 0))
        trigger.collider = Collider(radius=0.5, trigger=True)
        root.add_child(trigger)
        nodes = root.overlap_sphere(Vec3.ZERO, 1.0)
        assert nodes == []

        nodes_with_triggers = root.overlap_sphere(Vec3.ZERO, 1.0, hit_triggers=True)
        assert nodes_with_triggers == [trigger]


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


class TestNestedDraw:
    def test_draw_inside_on_draw_raises_and_keeps_camera_usable(self):
        inner = Node()
        inner.camera = Camera()

        class Hud(Node):
            def on_draw(self):
                inner.draw(0, 0, 8, 8)

        root = Node()
        root.camera = Camera()
        root.camera.clear_color = 0
        hud = Hud()
        root.add_child(hud)

        with raises_exact(ValueError, "draw cannot be called from inside on_draw"):
            root.draw(0, 0, 32, 24)

        root.remove_child(hud)
        root.draw(0, 0, 32, 24)


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


def _ball(pos: Vec3, *, radius: float = 0.5, mass: float = 1.0) -> Node:
    n = Node()
    n.transform = Mat4.from_translation(pos)
    n.collider = Collider(radius=radius, mass=mass)
    return n
