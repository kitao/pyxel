import gc
import math
import subprocess
import sys
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
    def test_small_parent_scales_preserve_rigid_world_velocity(self):
        root, parent, body = Node(), Node(), Node()
        root.transform = parent.transform = Mat4.from_scale(Vec3(0.01, 0.01, 0.01))
        body.transform = Mat4.from_scale(Vec3(10000, 10000, 10000))
        root.add_child(parent)
        parent.add_child(body)
        body.collider = Collider(
            radius=1, velocity=Vec3.RIGHT, gravity=0, linear_damp=0
        )
        assert body.world_transform == Mat4.IDENTITY
        root.update()
        assert tuple(body.world_transform.pos) == pytest.approx((1, 0, 0), abs=1e-6)

    def test_unrepresentable_parent_local_displacement_does_not_corrupt_pose(self):
        root, body = Node(), Node()
        root.transform = Mat4.from_scale(Vec3(1e-38, 1, 1))
        body.transform = Mat4.from_scale(Vec3(1e38, 1, 1))
        root.add_child(body)
        body.collider = Collider(
            radius=1, velocity=Vec3(10, 0, 0), gravity=0, linear_damp=0
        )
        before = body.transform
        root.update()
        # The inverse fits f32, but the requested local displacement (~1e39) does not.
        assert body.transform == before
        assert body.collider.velocity == Vec3(10, 0, 0)

    @pytest.mark.parametrize("amount", [90, 1e20])
    @pytest.mark.parametrize("axis", [0, 1, 2])
    @pytest.mark.parametrize("sign", [1, -1])
    @pytest.mark.parametrize("mass", [0, 1])
    def test_update_preserves_finite_prescribed_spin(self, amount, axis, sign, mass):
        unit = [0, 0, 0]
        unit[axis] = sign
        spin = Vec3(*(value * amount for value in unit))
        body = Node()
        initial = Mat4.from_translation(Vec3(1, 2, 3)).rotate_z(45)
        body.transform = initial
        body.collider = Collider(
            mass=mass, gravity=0, linear_damp=0, angular_damp=0, angular_velocity=spin
        )
        rotation = Mat4.from_axis_angle(Vec3(*unit), amount)
        assert rotation != Mat4.IDENTITY
        expected = initial * rotation

        body.update()
        actual = body.transform
        for row in range(4):
            for col in range(4):
                assert actual[row, col] == pytest.approx(expected[row, col], abs=2e-6)
        assert actual.pos == Vec3(1, 2, 3)
        assert body.collider.angular_velocity == spin

    def test_update_no_children(self):
        Node().update()

    def test_direct_subtree_update_respects_inactive_ancestor(self):
        class Body(Node):
            calls = 0

            def on_update(self):
                self.calls += 1

        parent, branch, body = Node(), Node(), Body()
        parent.add_child(branch)
        branch.add_child(body)
        body.collider = Collider(velocity=Vec3.RIGHT, gravity=0, linear_damp=0)
        parent.active = False
        branch.update()
        assert body.calls == 0
        assert body.transform.pos == Vec3.ZERO

        parent.active = True
        branch.update()
        assert body.calls == 1
        assert body.transform.pos == Vec3.RIGHT

    @pytest.mark.parametrize("scale", [(-1, 1, 1), (1, -1, 1), (1, 1, -1)])
    def test_canceled_parent_reflections_preserve_contact_response(self, scale):
        class Body(Node):
            def __init__(self):
                super().__init__()
                self.impulses = []

            def on_collide(self, other, contact):
                self.impulses.append(
                    (*contact.delta_velocity, *contact.delta_angular_velocity)
                )

        def simulate(reflect):
            root, parent, floor, body = Node(), Node(), Node(), Body()
            root.add_child(parent)
            root.add_child(floor)
            parent.add_child(body)
            parent.transform = reflect

            world = Mat4.from_translation(Vec3(0, 1.499, 0)) * Mat4.from_euler(
                Vec3(90, 0, 0)
            )
            body.transform = reflect * world
            body.collider = Collider(
                radius=0.5,
                mass=1,
                rolls=True,
                friction=1,
                velocity=Vec3(0, -1, 0),
                angular_velocity=Vec3(0, 10, 0),
                gravity=0,
                linear_damp=0,
                angular_damp=0,
            )

            floor.transform = Mat4.from_translation(Vec3(0, -1, 0))
            floor.collider = Collider(size=Vec3(100, 2, 100), mass=0, friction=1)

            root.update()
            return body

        plain = simulate(Mat4.IDENTITY)
        mirrored = simulate(Mat4.from_scale(Vec3(*scale)))
        assert plain.impulses and any(plain.impulses[0])
        assert len(mirrored.impulses) == len(plain.impulses)
        for actual, expected in zip(mirrored.impulses, plain.impulses):
            assert actual == pytest.approx(expected, abs=1e-6)
        for row in range(4):
            for col in range(4):
                assert mirrored.world_transform[row, col] == pytest.approx(
                    plain.world_transform[row, col], abs=1e-6
                )
        assert tuple(mirrored.collider.angular_velocity) == pytest.approx(
            tuple(plain.collider.angular_velocity), abs=1e-6
        )


class TestGravity:
    def test_defaults_and_independent_body_settings(self):
        a, b = Collider(), Collider()
        assert a.gravity == pytest.approx(9.8)
        assert a.gravity_direction == Vec3.DOWN
        assert a.linear_damp == a.angular_damp == pytest.approx(0.1)

        a.gravity, a.gravity_direction = 1.62, Vec3.RIGHT
        a.linear_damp, a.angular_damp = 0.3, 0.4
        assert a.gravity == pytest.approx(1.62)
        assert a.gravity_direction == Vec3.RIGHT
        assert a.linear_damp == pytest.approx(0.3)
        assert a.angular_damp == pytest.approx(0.4)
        assert b.gravity == pytest.approx(9.8)
        assert b.gravity_direction == Vec3.DOWN
        assert b.linear_damp == b.angular_damp == pytest.approx(0.1)

    def test_update_preserves_shared_vector_values(self):
        velocity, spin = Vec3(3, 0, 0), Vec3(0, 3, 0)
        body = Node()
        body.collider = Collider(velocity=velocity, angular_velocity=spin)
        other = Collider(velocity=velocity, angular_velocity=spin)
        saved = body.collider.velocity
        body.update()
        assert velocity == saved == other.velocity == Vec3(3, 0, 0)
        assert spin == other.angular_velocity == Vec3(0, 3, 0)
        assert body.collider.velocity != saved

        body.collider.velocity = body.collider.angular_velocity = Vec3.ZERO
        body.update()
        assert Vec3.ZERO == Vec3(0, 0, 0)

    @pytest.mark.parametrize("fps", [0, 30, 60])
    def test_free_fall_and_damping_use_configured_time_step(self, fps):
        code = """import sys

import pyxel
from pyxel.cube import Collider, Node, Vec3

fps = int(sys.argv[1])
if fps:
    pyxel.init(16, 16, fps=fps, headless=True)
else:
    fps = 30  # Scene updates without init retain the default time step.

# One second in free fall reaches 9.8 m/s regardless of mass or update rate.
for mass in (1, 100):
    body = Node()
    body.collider = Collider(mass=mass, linear_damp=0)

    for _ in range(fps):
        body.update()

    assert abs(body.collider.velocity.y * fps / 100 + 9.8) < 1e-4

    # Semi-implicit Euler falls an extra half-step compared with g*t*t/2.
    expected = -490 * (fps + 1) / fps
    assert abs(body.transform.pos.y - expected) < 0.002

# About 9.5 percent of both speeds is lost in one second at damping 0.1.
body = Node()
body.collider = Collider(
    gravity=0, velocity=Vec3(3, 0, 0), angular_velocity=Vec3(0, 3, 0)
)

for _ in range(fps):
    body.update()

assert 0.903 < body.collider.velocity.x / 3 < 0.906
assert 0.903 < body.collider.angular_velocity.y / 3 < 0.906
"""
        result = subprocess.run(
            [sys.executable, "-c", code, str(fps)],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_direction_is_world_space_and_normalized(self):
        parent, body = Node(), Node()
        parent.transform = Mat4.from_euler(Vec3(0, 0, 90))
        body.collider = Collider(gravity_direction=Vec3(7, 0, 0), linear_damp=0)
        parent.add_child(body)
        parent.update()
        assert tuple(body.collider.velocity) == pytest.approx((980 / 900, 0, 0))
        assert tuple(body.transform.pos) == pytest.approx((0, -980 / 900, 0), abs=1e-6)

        body.collider.gravity_direction = Vec3.ZERO
        parent.update()
        assert tuple(body.collider.velocity) == pytest.approx((980 / 900, 0, 0))

    def test_static_and_inactive_bodies_ignore_gravity_and_damping(self):
        body = Node()
        body.collider = Collider(
            mass=0, velocity=Vec3(1, 0, 0), angular_velocity=Vec3(0, 1, 0)
        )
        body.update()
        assert body.transform.pos == Vec3.RIGHT
        assert body.collider.velocity == Vec3.RIGHT
        assert body.collider.angular_velocity == Vec3.UP

        body.collider.mass = 1
        body.active = False
        body.update()
        assert body.transform.pos == Vec3.RIGHT
        assert body.collider.velocity == Vec3.RIGHT


class TestFiniteGravityDirection:
    @pytest.mark.parametrize("scale", [1e-30, 1e-20, 1, 1e10, 1e20, 1e30])
    def test_strength_is_independent_of_direction_magnitude(self, scale):
        direction = Vec3(3 * scale, 4 * scale, 0)
        assert all(math.isfinite(value) for value in direction)
        body = Node()
        body.collider = Collider(
            gravity=9.8, gravity_direction=direction, linear_damp=0
        )
        before = tuple(direction)
        body.update()
        # World centimeters per frame at the default 30 fps; direction is a 3-4-5 triangle.
        expected = (0.6 * 980 / 900, 0.8 * 980 / 900, 0)
        assert tuple(body.collider.velocity) == pytest.approx(expected, abs=1e-6)
        assert tuple(body.transform.pos) == pytest.approx(expected, abs=1e-6)
        assert tuple(direction) == before


class _CollisionCounter(Node):
    def __init__(self, pos: Vec3):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.collider = Collider(radius=0.5, gravity=0, linear_damp=0, angular_damp=0)
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
            size=Vec3(2, 2, 2),
            radius=1.0,
            mass=mass,
            velocity=velocity,
            gravity=0,
            linear_damp=0,
            angular_damp=0,
        )
        self.contacts = []

    def on_collide(self, other, contact):
        del other
        self.contacts.append((contact.depth, tuple(contact.normal)))


class _ContactRecorder(Node):
    def __init__(self):
        super().__init__()
        self.contacts = []

    def on_collide(self, other, contact):
        del other
        self.contacts.append(
            (contact.depth, tuple(contact.normal), tuple(contact.point))
        )


class TestCollisionPipeline:
    @staticmethod
    def _assert_scaled_contacts(body, scale, depth, normal):
        assert body.contacts
        # Normalizing by shape scale leaves only a few ulps of f32 input/output rounding.
        assert body.contacts[0][0] / scale == pytest.approx(depth, rel=4e-6, abs=4e-6)
        for actual_depth, actual_normal, point in body.contacts:
            assert all(
                math.isfinite(value) for value in (actual_depth, *actual_normal, *point)
            )
            assert actual_normal == pytest.approx(normal, abs=4e-6)

    @staticmethod
    def _plane_mesh(scale, reverse):
        primitive = Primitive(
            Primitive.MODE_TRIANGLES,
            [-10 * scale, 0, -10 * scale, 10 * scale, 0, -10 * scale, 0, 0, 10 * scale],
            [0, 2, 1] if reverse else [0, 1, 2],
        )
        return Mesh([primitive], [Mat4.IDENTITY], [-1])

    @pytest.mark.parametrize("reverse", [False, True])
    @pytest.mark.parametrize("overlap", [0, 0.1])
    @pytest.mark.parametrize("motion", ["approach", "exit", "slide"])
    @pytest.mark.parametrize(
        "size,radius,height",
        [((0, 0, 0), 1, 1), ((0, 2, 0), 1, 2), ((2, 2, 2), 0, 1), ((1, 1, 1), 0.5, 1)],
    )
    def test_mesh_motion_from_contact(
        self, reverse, overlap, motion, size, radius, height
    ):
        class Body(Node):
            def on_collide(self, other, contact):
                self.transform = Mat4.from_translation(
                    self.transform.pos + contact.normal * contact.depth
                )
                self.collider.velocity += contact.delta_velocity

        velocity = {
            "approach": Vec3(0, -10, 0),
            "exit": Vec3(0, 10, 0),
            "slide": Vec3(1, 0, 0),
        }[motion]
        root, body, floor = Node(), Body(), Node()
        floor.collider = Collider(mesh=self._plane_mesh(1, reverse), mass=0, friction=0)
        body.transform = Mat4.from_translation(Vec3(0, height - overlap, 0))
        body.collider = Collider(
            size=Vec3(*size),
            radius=radius,
            velocity=velocity,
            gravity=0,
            linear_damp=0,
            angular_damp=0,
            friction=0,
        )
        root.add_child(floor)
        root.add_child(body)
        root.update()

        expected = (
            (0, height - overlap + 10, 0)
            if motion == "exit"
            else (int(motion == "slide"), height, 0)
        )
        # The analytic plane contact is exact apart from f32 position rounding.
        assert tuple(body.transform.pos) == pytest.approx(expected, rel=0, abs=2e-5)
        expected_velocity = velocity if motion != "approach" else Vec3.ZERO
        assert tuple(body.collider.velocity) == pytest.approx(
            tuple(expected_velocity), rel=0, abs=2e-5
        )

    @pytest.mark.parametrize("scale", [1, 1e10, 1e20])
    @pytest.mark.parametrize(
        "target", ["sphere", "capsule", "box", "plane", "capsule_end"]
    )
    def test_sphere_sweep_preserves_scaled_contact(self, scale, target):
        root, body, wall = Node(), _ContactRecorder(), Node()
        if target == "plane":
            start, velocity = Vec3(0, 1.5 * scale, 0), Vec3(0, -3 * scale, 0)
            wall.collider = Collider(mesh=self._plane_mesh(scale, False), mass=0)
            depth, normal, point = 1.75, (0, 1, 0), (0, 0, 0)
        elif target == "capsule_end":
            start, velocity = Vec3(0, 2.5 * scale, 0), Vec3(0, -5 * scale, 0)
            wall.collider = Collider(
                size=Vec3(0, 2 * scale, 0), radius=0.25 * scale, mass=0
            )
            depth, normal, point = 4, (0, 1, 0), (0, 1.25, 0)
        else:
            start, velocity = Vec3(-1.5 * scale, 0, 0), Vec3(3 * scale, 0, 0)
            if target == "box":
                wall.collider = Collider(
                    size=Vec3(0.5 * scale, 2 * scale, 2 * scale), mass=0
                )
            else:
                size = Vec3(0, 2 * scale, 0) if target == "capsule" else Vec3.ZERO
                wall.collider = Collider(size=size, radius=0.25 * scale, mass=0)
            depth, normal, point = 2, (-1, 0, 0), (-0.25, 0, 0)

        body.transform = Mat4.from_translation(start)
        body.collider = Collider(
            radius=0.25 * scale,
            velocity=velocity,
            gravity=0,
            linear_damp=0,
            angular_damp=0,
            friction=0,
        )
        root.add_child(body)
        root.add_child(wall)

        root.update()

        # Entry needs one shape-scale unit of motion (1.25 for the plane).
        # The remaining normal motion gives depth; point lies on the static surface.
        self._assert_scaled_contacts(body, scale, depth, normal)
        assert tuple(value / scale for value in body.contacts[0][2]) == pytest.approx(
            point, abs=4e-6
        )

    @pytest.mark.parametrize("scale", [1, 1e10, 1e20, 1e30, 1e37])
    @pytest.mark.parametrize("x", [0, 0.25, -0.25])
    def test_capsule_box_core_contact_finite_range(self, scale, x):
        root, body, wall = Node(), _ContactRecorder(), Node()
        body.transform = Mat4.from_translation(Vec3(x * scale, 0, 0))
        body.collider = Collider(
            size=Vec3(0, 8 * scale, 0),
            radius=0.1 * scale,
            mass=1,
            gravity=0,
            linear_damp=0,
            angular_damp=0,
        )
        wall.collider = Collider(
            size=Vec3(2 * scale, 6 * scale, 4 * scale), radius=0.2 * scale, mass=0
        )
        root.add_child(body)
        root.add_child(wall)

        root.update()

        self._assert_scaled_contacts(
            body, scale, 1.3 - abs(x), (1 if x >= 0 else -1, 0, 0)
        )

    @pytest.mark.parametrize("scale", [1, 1e10, 1e20, 1e30, 1e37])
    @pytest.mark.parametrize("y", [0, 0.25, -0.25])
    @pytest.mark.parametrize("reverse", [False, True])
    def test_capsule_triangle_core_contact_finite_range(self, scale, y, reverse):
        root, body, wall = Node(), _ContactRecorder(), Node()
        body.transform = Mat4.from_translation(Vec3(0, y * scale, 0))
        body.collider = Collider(
            size=Vec3(0, 2 * scale, 0),
            radius=0.1 * scale,
            mass=1,
            gravity=0,
            linear_damp=0,
            angular_damp=0,
        )
        wall.collider = Collider(mesh=self._plane_mesh(scale, reverse), mass=0)
        root.add_child(body)
        root.add_child(wall)

        root.update()

        normal_y = (1 if reverse else -1) if y == 0 else (1 if y > 0 else -1)
        self._assert_scaled_contacts(body, scale, 1.1 - abs(y), (0, normal_y, 0))

    @pytest.mark.parametrize("scale", [1, 1e10, 1e20, 1e30, 1e37])
    @pytest.mark.parametrize("height, hits", [(0.5, True), (1.25, False)])
    def test_box_triangle_contact_finite_range(self, scale, height, hits):
        root, body, wall = Node(), _ContactRecorder(), Node()
        body.transform = Mat4.from_translation(Vec3(0, height * scale, 0))
        body.collider = Collider(
            size=Vec3(2 * scale, 2 * scale, 2 * scale),
            mass=1,
            gravity=0,
            linear_damp=0,
            angular_damp=0,
        )
        wall.collider = Collider(mesh=self._plane_mesh(scale, False), mass=0)
        root.add_child(body)
        root.add_child(wall)

        root.update()

        if hits:
            self._assert_scaled_contacts(body, scale, 0.5, (0, 1, 0))
        else:
            assert body.contacts == []

    @pytest.mark.parametrize("scale", [1, 1e10, 1e20, 1e30, 1e37])
    @pytest.mark.parametrize("dimensions", [1, 2, 3])
    @pytest.mark.parametrize("reverse", [False, True])
    def test_rounded_box_contact_finite_range(self, scale, dimensions, reverse):
        root, body, wall = Node(), _ContactRecorder(), Node()
        center = [2.1 * scale if i < dimensions else 0 for i in range(3)]
        target = Mat4.from_translation(Vec3(*center))
        body.transform, wall.transform = (
            (target, Mat4.IDENTITY) if reverse else (Mat4.IDENTITY, target)
        )
        size = Vec3(2 * scale, 2 * scale, 2 * scale)
        body.collider = Collider(
            size=size,
            radius=0.1 * scale,
            mass=1,
            gravity=0,
            linear_damp=0,
            angular_damp=0,
        )
        wall.collider = Collider(size=size, radius=0.1 * scale, mass=0)
        root.add_child(body)
        root.add_child(wall)

        root.update()

        length = math.sqrt(dimensions)
        sign = 1 if reverse else -1
        normal = tuple(sign / length if i < dimensions else 0 for i in range(3))
        self._assert_scaled_contacts(body, scale, 0.2 - 0.1 * length, normal)

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

    @pytest.mark.parametrize("scale", [1, 1e10, 1e20, 1e30, 1e37])
    @pytest.mark.parametrize(
        "offset, body_speed, wall_speed, hits",
        [(3.5, 10, 0, False), (3, 10, 0, True), (3, 8, -2, True)],
    )
    def test_swept_rounded_boxes_use_corner_distance(
        self, scale, offset, body_speed, wall_speed, hits
    ):
        root = Node()
        body = _RoundedBox(
            Mat4.from_translation(Vec3(-5 * scale, offset * scale, offset * scale)),
            velocity=Vec3(body_speed * scale, 0, 0),
        )
        wall = _RoundedBox(
            Mat4.IDENTITY, mass=0, velocity=Vec3(wall_speed * scale, 0, 0)
        )
        body.collider.size = wall.collider.size = Vec3(2 * scale, 2 * scale, 2 * scale)
        body.collider.radius = wall.collider.radius = scale
        root.add_child(body)
        root.add_child(wall)

        root.update()

        # Both endpoints are separated. Relative x motion is 10 * scale;
        # the transverse gap is sqrt(2) * (offset - 2) * scale, with combined radius 2 * scale.
        if not hits:
            assert body.contacts == []
            assert wall.contacts == []
        else:
            assert len(body.contacts) == len(wall.contacts) == 1
            # First contact has core separation (-sqrt(2), 1, 1), of length 2.
            assert body.contacts[0][1] == pytest.approx(
                (-(2**0.5) / 2, 0.5, 0.5), abs=1e-4
            )


class _FiniteContactRecorder(Node):
    def __init__(self):
        super().__init__()
        self.contacts = []

    def on_collide(self, other, contact):
        self.contacts.append(
            (
                contact.depth,
                tuple(contact.normal),
                tuple(contact.point),
                tuple(contact.delta_velocity),
                tuple(contact.delta_angular_velocity),
            )
        )


_FINITE_COLLIDER_SIZES = {
    "sphere": (0, 0, 0),
    "capsule": (0, 0.2, 0),
    "box": (0.2, 0.2, 0.2),
}


def _finite_axis_vector(axis, value):
    values = [0, 0, 0]
    values[axis] = value
    return Vec3(*values)


class TestFiniteRelativeFrames:
    @pytest.mark.parametrize("shape_a", _FINITE_COLLIDER_SIZES)
    @pytest.mark.parametrize("shape_b", _FINITE_COLLIDER_SIZES)
    @pytest.mark.parametrize("axis", range(3))
    @pytest.mark.parametrize("sign", [-1, 1])
    @pytest.mark.parametrize("scale", [1, 1e20, 1e38])
    def test_sweep_relative_range(self, shape_a, shape_b, axis, sign, scale):
        root, a, b = Node(), _FiniteContactRecorder(), _FiniteContactRecorder()
        for node, order, shape in [(a, sign, shape_a), (b, -sign, shape_b)]:
            node.transform = Mat4.from_translation(
                _finite_axis_vector(axis, -order * 0.5 * scale)
            )
            node.collider = Collider(
                size=Vec3(*(v * scale for v in _FINITE_COLLIDER_SIZES[shape])),
                radius=0.1 * scale,
                velocity=_finite_axis_vector(axis, order * 1.8 * scale),
                gravity=0,
                linear_damp=0,
                angular_damp=0,
                friction=0,
            )
            root.add_child(node)
        root.update()
        ha, hb = (
            _FINITE_COLLIDER_SIZES[shape_a][axis] / 2,
            _FINITE_COLLIDER_SIZES[shape_b][axis] / 2,
        )
        assert len(a.contacts) == len(b.contacts) == 1
        for node, order in [(a, sign), (b, -sign)]:
            depth, normal, point, velocity, spin = node.contacts[0]
            assert all(
                math.isfinite(v) for v in [depth, *normal, *point, *velocity, *spin]
            )
            assert depth / scale == pytest.approx((2.8 + ha + hb) / 2, abs=3e-5)
            assert normal == pytest.approx(
                tuple(_finite_axis_vector(axis, -order)), abs=3e-5
            )
            assert tuple(v / scale for v in velocity) == pytest.approx(
                tuple(_finite_axis_vector(axis, -order * 1.8)), abs=3e-5
            )
            assert spin == pytest.approx((0, 0, 0), abs=1e-7)
            assert point[axis] / scale == pytest.approx(sign * (ha - hb) / 2, abs=3e-5)
            # Flat/axial contacts may choose any point in their common face/segment.
            for other in range(3):
                if other != axis:
                    assert (
                        abs(point[other] / scale)
                        <= min(
                            _FINITE_COLLIDER_SIZES[shape_a][other],
                            _FINITE_COLLIDER_SIZES[shape_b][other],
                        )
                        / 2
                        + 3e-5
                    )

    @pytest.mark.parametrize("shape", ["sphere", "capsule", "box", "rounded_box"])
    @pytest.mark.parametrize("axis", range(3))
    @pytest.mark.parametrize("sign", [-1, 1])
    @pytest.mark.parametrize("scale", [1, 1e20, 1e38])
    def test_ray_relative_range(self, shape, axis, sign, scale):
        sizes = {
            "sphere": (0, 0, 0),
            "capsule": (0, 0.2, 0),
            "box": (3, 3, 3),
            "rounded_box": (2.6, 2.6, 2.6),
        }
        radii = {"sphere": 1.5, "capsule": 1.5, "box": 0, "rounded_box": 0.2}
        root, target = Node(), Node()
        target.transform = Mat4.from_translation(
            _finite_axis_vector(axis, -sign * 1.8 * scale)
        )
        target.collider = Collider(
            size=Vec3(*(v * scale for v in sizes[shape])),
            radius=radii[shape] * scale,
            mass=0,
        )
        root.add_child(target)
        extent = sizes[shape][axis] / 2 + radii[shape]
        for method in ["raycast", "raycast_all"]:
            capped = getattr(root, method)(
                _finite_axis_vector(axis, sign * 1.8 * scale),
                _finite_axis_vector(axis, -sign),
                max_distance=(3.5 - extent) * scale,
            )
            assert capped == ([] if method == "raycast_all" else None)
            result = getattr(root, method)(
                _finite_axis_vector(axis, sign * 1.8 * scale),
                _finite_axis_vector(axis, -sign),
            )
            if method == "raycast_all":
                assert len(result) == 1
                hit = result[0]
            else:
                assert result is not None
                hit = result
            assert hit.node is target
            assert all(
                math.isfinite(v) for v in [hit.distance, *hit.point, *hit.normal]
            )
            assert hit.distance / scale == pytest.approx(3.6 - extent, abs=3e-5)
            assert tuple(v / scale for v in hit.point) == pytest.approx(
                tuple(_finite_axis_vector(axis, sign * (-1.8 + extent))), abs=3e-5
            )
            assert tuple(hit.normal) == pytest.approx(
                tuple(_finite_axis_vector(axis, sign)), abs=3e-5
            )

    @pytest.mark.parametrize("shape", ["capsule", "rounded_box"])
    @pytest.mark.parametrize("sign", [-1, 1])
    def test_ray_relative_inside_exit(self, shape, sign):
        scale = 1e38
        root, target = Node(), Node()
        axis = 1
        size = (0, 2, 0) if shape == "capsule" else (3, 3, 3)
        radius = 2.7 if shape == "capsule" else 2.2
        target.transform = Mat4.from_translation(
            _finite_axis_vector(axis, -sign * 1.8 * scale)
        )
        target.collider = Collider(
            size=Vec3(*(v * scale for v in size)), radius=radius * scale, mass=0
        )
        root.add_child(target)
        hit = root.raycast(
            _finite_axis_vector(axis, sign * 1.8 * scale),
            _finite_axis_vector(axis, sign),
        )
        assert hit is not None
        assert hit.distance / scale == pytest.approx(0.1, abs=3e-5)
        assert tuple(v / scale for v in hit.point) == pytest.approx(
            tuple(_finite_axis_vector(axis, sign * 1.9)), abs=3e-5
        )
        assert tuple(hit.normal) == pytest.approx(
            tuple(_finite_axis_vector(axis, sign)), abs=3e-5
        )

    @pytest.mark.parametrize("shape", ["capsule", "rounded_box"])
    @pytest.mark.parametrize("sign", [-1, 1])
    @pytest.mark.parametrize("scale", [1, 1e20, 1e38])
    def test_ray_relative_rejects_aabb_corner(self, shape, sign, scale):
        root, target = Node(), Node()
        capsule = shape == "capsule"
        size = (0, 0.2, 0) if capsule else (1, 1, 1)
        radius = 0.5 if capsule else 0.2
        target.transform = Mat4.from_translation(Vec3(-sign * 1.8 * scale, 0, 0))
        target.collider = Collider(
            size=Vec3(*(v * scale for v in size)), radius=radius * scale, mass=0
        )
        root.add_child(target)
        # Both rays enter the AABB but miss the curved edge/cap surface.
        origin = Vec3(
            sign * 1.8 * scale,
            (0.55 if capsule else 0.65) * scale,
            (0.45 if capsule else 0.65) * scale,
        )
        direction = Vec3(-sign, 0, 0)
        assert root.raycast(origin, direction) is None
        assert root.raycast_all(origin, direction) == []


class TestMeshColliderRobustness:
    @pytest.mark.parametrize(
        "edit",
        ["positions", "indices", "primitives", "transforms", "parents", "mesh", "none"],
    )
    @pytest.mark.parametrize("terrain_mass", [0, 1])
    def test_sleeping_bodies_wake_when_mesh_geometry_changes(self, edit, terrain_mass):
        class Ball(Node):
            def __init__(self):
                super().__init__()
                self.depths = []
                self.transform = Mat4.from_translation(Vec3(0, 8, 0))
                self.collider = Collider(
                    radius=8, rolls=True, gravity=0, linear_damp=0, angular_damp=0
                )

            def on_update(self):
                self.collider.velocity += Vec3(0, -0.16, 0)

            def on_collide(self, other, contact):
                self.depths.append(contact.depth)
                self.transform = Mat4.from_translation(
                    self.transform.pos + contact.normal * contact.depth
                )
                self.collider.velocity += contact.delta_velocity
                self.collider.angular_velocity += contact.delta_angular_velocity

        positions = [
            value
            for y in (0, 2)
            for point in (
                (-100, y, -100),
                (-100, y, 100),
                (100, y, 100),
                (100, y, -100),
            )
            for value in point
        ]
        indices = [0, 1, 2, 0, 2, 3]
        primitive = Primitive(Primitive.MODE_TRIANGLES, positions, indices)
        scenes = []
        # Two independent mesh caches share the same editable primitive.
        for _ in range(2):
            mesh = Mesh(
                [None, primitive],
                [Mat4.from_translation(Vec3(0, 2, 0)), Mat4.IDENTITY],
                [-1, -1],
            )
            root, terrain, ball = Node(), Node(), Ball()
            terrain.collider = Collider(mesh=mesh, mass=terrain_mass)
            root.add_child(terrain)
            root.add_child(ball)
            for _ in range(150):
                root.update()
            assert ball.transform.pos == Vec3(0, 8, 0)
            assert ball.collider.velocity == Vec3.ZERO
            scenes.append((root, terrain, ball, mesh))

        if edit == "positions":
            primitive.positions[1::3] = [y + 2 for y in primitive.positions[1::3]]
        elif edit == "indices":
            primitive.indices[:] = [i + 4 for i in indices]
        else:
            for _, terrain, _, mesh in scenes:
                if edit == "primitives":
                    mesh.primitives = [
                        None,
                        Primitive(Primitive.MODE_TRIANGLES, positions[12:], indices),
                    ]
                elif edit == "transforms":
                    mesh.transforms = [
                        Mat4.IDENTITY,
                        Mat4.from_translation(Vec3(0, 2, 0)),
                    ]
                elif edit == "parents":
                    mesh.parents = [-1, 0]
                elif edit == "mesh":
                    terrain.collider.mesh = Mesh(
                        [Primitive(Primitive.MODE_TRIANGLES, positions[12:], indices)],
                        [Mat4.IDENTITY],
                        [-1],
                    )

        for root, _, ball, _ in scenes:
            ball.depths.clear()
            root.update()
            if edit == "none":
                assert ball.depths == [0]
                assert ball.transform.pos == Vec3(0, 8, 0)
            else:
                # Radius8 resting on the raised y2 plane requires a y10 center.
                assert ball.depths[0] == pytest.approx(2, abs=1e-5)
                assert ball.transform.pos.y == pytest.approx(10, abs=1e-5)
            for _ in range(25):
                root.update()
            assert ball.transform.pos.y == pytest.approx(
                8 if edit == "none" else 10, abs=1e-5
            )

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
    @pytest.mark.parametrize("method", ["raycast", "raycast_all"])
    @pytest.mark.parametrize("scale", [0.5, 2**-128])
    def test_mesh_unavailable_inverse_keeps_world_ray_and_cache(self, method, scale):
        primitive, target, root = _inverse_fallback_mesh_scene(scale)
        query = getattr(root, method)
        result = query(Vec3(-1, 0.25, 0.25), Vec3.RIGHT, max_distance=1.25)
        hit = result[0] if method == "raycast_all" else result
        assert hit is not None and hit.node is target
        plane = (target.world_transform * Vec3(1, 0, 0)).x
        assert hit.distance == pytest.approx(plane + 1, abs=1e-6)
        assert tuple(hit.point) == pytest.approx((plane, 0.25, 0.25), abs=1e-6)

        assert query(Vec3(-1, 0.25, 0.25), Vec3.RIGHT, max_distance=0.25) == (
            [] if method == "raycast_all" else None
        )

        primitive.positions[1::3] = [y + 10 for y in primitive.positions[1::3]]
        assert query(Vec3(-1, 0.25, 0.25), Vec3.RIGHT, max_distance=1.25) == (
            [] if method == "raycast_all" else None
        )

    @pytest.mark.parametrize("scale", [0.5, 2**-128])
    def test_mesh_unavailable_inverse_keeps_sphere_geometry_and_cache(self, scale):
        primitive, target, root = _inverse_fallback_mesh_scene(scale)
        plane = (target.world_transform * Vec3(1, 0, 0)).x
        for center in [
            (plane + 0.125, 0.25, 0.25),
            (plane + 0.125, -0.1, 0.25),
            (plane + 0.125, -0.1, -0.1),
        ]:
            assert root.overlap_sphere(Vec3(*center), 0.25) == [target]

        # This point is in the triangle AABB but beyond its diagonal edge by more than the radius.
        assert root.overlap_sphere(Vec3(plane + 0.125, 0.9, 0.9), 0.25) == []

        primitive.positions[1::3] = [y + 10 for y in primitive.positions[1::3]]
        assert root.overlap_sphere(Vec3(plane + 0.125, 0.25, 0.25), 0.25) == []

    @pytest.mark.parametrize("method", ["raycast", "raycast_all"])
    @pytest.mark.parametrize("scale", [1, 1e35, 1e38])
    @pytest.mark.parametrize("axis", [0, 1, 2])
    @pytest.mark.parametrize("sign", [1, -1])
    def test_raycast_preserves_mesh_hit_when_local_reciprocal_overflows(
        self, method, scale, axis, sign
    ):
        vertices = []
        for values in [(0, -10, 0), (0, 10, 0), (0, 0, -2000)]:
            point = [0.0] * 3
            for i, value in enumerate(values):
                point[(axis + i) % 3] = value
            vertices.append(Vec3(*point))
        primitive = Primitive(
            Primitive.MODE_TRIANGLES,
            [value for vertex in vertices for value in vertex],
            [0, 1, 2],
        )
        root, target = Node(), Node()
        diagonal = [1.0] * 3
        diagonal[axis] = scale
        target.transform = Mat4.from_scale(Vec3(*diagonal))
        target.collider = Collider(mesh=Mesh([primitive], [Mat4.IDENTITY], [-1]))
        root.add_child(target)
        # Scaling the identically zero normal-axis coordinates leaves every world vertex exact.
        assert all(target.world_transform * vertex == vertex for vertex in vertices)
        origin, direction, point = ([0.0] * 3 for _ in range(3))
        origin[axis], direction[axis] = -sign, 0.001 * sign
        direction[(axis + 2) % 3], point[(axis + 2) % 3] = -1, -1000
        query = getattr(root, method)

        for max_distance in [1000.001, 1000.01, 1000.1, 2000]:
            result = query(Vec3(*origin), Vec3(*direction), max_distance=max_distance)
            if method == "raycast_all":
                assert len(result) == 1
                hit = result[0]
            else:
                hit = result
                assert hit is not None
            assert hit.node is target
            # The interior point has barycentric weights (1/4, 1/4, 1/2); allow f32 ulps at 1000.
            assert hit.distance == pytest.approx(math.sqrt(1000001), abs=0.001)
            assert tuple(hit.point) == pytest.approx(point, abs=0.001)

        for max_distance in [900, 1000.0001]:
            assert query(
                Vec3(*origin), Vec3(*direction), max_distance=max_distance
            ) == ([] if method == "raycast_all" else None)

    @pytest.mark.parametrize("method", ["raycast", "raycast_all"])
    def test_raycast_empty_mesh_with_subnormal_local_direction(self, method):
        target = Node()
        target.transform = Mat4.from_scale(Vec3(1e38, 1, 1))
        target.collider = Collider(mesh=Mesh([], [], []))
        assert getattr(target, method)(Vec3(-1, 0, 0), Vec3(0.001, 0, -1)) == (
            [] if method == "raycast_all" else None
        )

    @pytest.mark.parametrize("method", ["raycast", "raycast_all"])
    @pytest.mark.parametrize("scale", [1, 1e-38])
    @pytest.mark.parametrize("axis", [0, 1, 2])
    @pytest.mark.parametrize("sign", [1, -1])
    def test_raycast_scaled_mesh_preserves_world_distance(
        self, method, scale, axis, sign
    ):
        positions = []
        for a, b in [(-1, -1), (1, -1), (0, 1)]:
            point = [0.0] * 3
            point[axis] = 2 * sign / scale
            point[(axis + 1) % 3] = a / scale
            point[(axis + 2) % 3] = b / scale
            positions.extend(point)
        primitive = Primitive(Primitive.MODE_TRIANGLES, positions, [0, 1, 2])
        root, target = Node(), Node()
        target.transform = Mat4.from_scale(Vec3(scale, scale, scale))
        target.collider = Collider(mesh=Mesh([primitive], [Mat4.IDENTITY], [-1]))
        root.add_child(target)
        origin, direction, point, normal = ([0.0] * 3 for _ in range(4))
        origin[axis], direction[axis] = -2 * sign, sign
        point[axis], normal[axis] = 2 * sign, -sign
        query = getattr(root, method)
        result = query(Vec3(*origin), Vec3(*direction), max_distance=5)
        if method == "raycast_all":
            assert len(result) == 1
            hit = result[0]
        else:
            hit = result
            assert hit is not None
        assert hit.node is target
        # The world plane is four units away; scaling allows a few stored f32 ulps.
        assert hit.distance == pytest.approx(4, rel=2e-6)
        assert tuple(hit.point) == pytest.approx(point, abs=2e-6)
        assert tuple(hit.normal) == pytest.approx(normal, abs=2e-6)
        assert query(Vec3(*origin), Vec3(*direction), max_distance=3) == (
            [] if method == "raycast_all" else None
        )

    @pytest.mark.parametrize("method", ["raycast", "raycast_all"])
    @pytest.mark.parametrize("scale", [1, 1e10, 1e20, 1e37])
    @pytest.mark.parametrize(
        "surface",
        ["sphere", "capsule_body", "capsule_cap", "capsule_hemisphere", "mesh"],
    )
    def test_raycast_finite_geometry_range(self, method, scale, surface):
        root, target = Node(), Node()
        if surface == "mesh":
            primitive = Primitive(
                Primitive.MODE_TRIANGLES,
                [-scale, 0, -scale, scale, 0, -scale, 0, 0, scale],
                [0, 1, 2],
            )
            target.collider = Collider(mesh=Mesh([primitive], [Mat4.IDENTITY], [-1]))
            origin, direction = Vec3(0, 5 * scale, 0), Vec3.DOWN
            distance, point, normal = 5 * scale, (0, 0, 0), (0, 1, 0)
        else:
            target.collider = Collider(
                size=Vec3(0, 2 * scale, 0)
                if surface.startswith("capsule")
                else Vec3.ZERO,
                radius=scale,
            )
            if surface == "capsule_cap":
                origin, direction = Vec3(0, 5 * scale, 0), Vec3.DOWN
                distance, point, normal = 3 * scale, (0, 2 * scale, 0), (0, 1, 0)
            elif surface == "capsule_hemisphere":
                chord = math.sqrt(0.75)
                origin, direction = Vec3(0, 1.5 * scale, 5 * scale), Vec3(0, 0, -1)
                distance, point, normal = (
                    (5 - chord) * scale,
                    (0, 1.5 * scale, chord * scale),
                    (0, 0.5, chord),
                )
            else:
                origin, direction = Vec3(5 * scale, 0, 0), Vec3.LEFT
                distance, point, normal = 4 * scale, (scale, 0, 0), (1, 0, 0)
        root.add_child(target)

        query = getattr(root, method)
        result = query(origin, direction)
        if method == "raycast_all":
            assert len(result) == 1
            hit = result[0]
        else:
            hit = result
            assert hit is not None
        assert hit.node is target
        # Analytic surface coordinates allow a few ulps of the stored scale.
        assert hit.distance == pytest.approx(distance, rel=2e-6)
        assert all(math.isfinite(x) for x in (*hit.point, *hit.normal))
        assert tuple(hit.point) == pytest.approx(point, rel=2e-6, abs=2e-6 * scale)
        assert tuple(hit.normal) == pytest.approx(normal, rel=2e-6, abs=2e-6)

        assert query(origin, direction, max_distance=0.9 * distance) == (
            [] if method == "raycast_all" else None
        )

    @pytest.mark.parametrize("method", ["raycast", "raycast_all"])
    @pytest.mark.parametrize("scale", [1, 1e20, 1e37])
    @pytest.mark.parametrize("capsule", [False, True])
    def test_raycast_inside_finite_geometry_range(self, method, scale, capsule):
        target = Node()
        target.collider = Collider(
            size=Vec3(0, 2 * scale, 0) if capsule else Vec3.ZERO, radius=scale
        )
        result = getattr(target, method)(Vec3.ZERO, Vec3.RIGHT)
        if method == "raycast_all":
            assert len(result) == 1
            hit = result[0]
        else:
            hit = result
            assert hit is not None
        assert hit.node is target
        assert all(math.isfinite(x) for x in (*hit.point, *hit.normal))
        assert hit.distance == pytest.approx(scale, rel=2e-6)
        assert tuple(hit.point) == pytest.approx((scale, 0, 0), rel=2e-6)
        assert tuple(hit.normal) == (1, 0, 0)

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
    @pytest.mark.parametrize("origin", [(0, 0, 5), (3, 0, 4)])
    @pytest.mark.parametrize("magnitude", [1, 2, 1e20, 1e30, 6e37])
    def test_raycast_preserves_finite_direction_scale(self, method, origin, magnitude):
        root, target = Node(), _ball(Vec3.ZERO)
        root.add_child(target)
        origin = Vec3(*origin)
        direction = origin * -magnitude
        assert all(math.isfinite(value) for value in direction)
        query = getattr(root, method)

        result = query(origin, direction, max_distance=5)
        if method == "raycast_all":
            assert len(result) == 1
            hit = result[0]
        else:
            hit = result
            assert hit is not None
        assert hit.node is target
        # Both origins have length 5; the radius .5 sphere is entered at 4.5.
        # Diagonal normalization and intersection use unit-scale f32 geometry.
        assert hit.distance == pytest.approx(4.5, abs=1e-5)
        assert tuple(hit.point) == pytest.approx(tuple(origin * 0.1), abs=1e-5)
        assert tuple(hit.normal) == pytest.approx(tuple(origin * 0.2), abs=1e-5)
        assert query(origin, direction, max_distance=4) == (
            [] if method == "raycast_all" else None
        )

    @pytest.mark.parametrize("method", ["raycast", "raycast_all"])
    @pytest.mark.parametrize("rotated", [False, True])
    @pytest.mark.parametrize(
        "size, radius, origin, direction, point, normal",
        [
            ((0, 2, 0), 0.5, (0, 0, 0), (0, 1, 0), (0, 1.5, 0), (0, 1, 0)),
            ((0, 2, 0), 0.5, (0, 1, 0), (0, -1, 0), (0, -1.5, 0), (0, -1, 0)),
            ((0, 2, 0), 0.5, (0, 0, 0), (1, 0, 0), (0.5, 0, 0), (1, 0, 0)),
            ((0, 2, 0), 0.5, (0, 3, 0), (0, -1, 0), (0, 1.5, 0), (0, 1, 0)),
            ((2, 2, 2), 0.5, (0, 1, 0), (1, 0, 0), (1.5, 1, 0), (1, 0, 0)),
            ((2, 2, 2), 0.5, (0, 0, 0), (1, 0, 0), (1.5, 0, 0), (1, 0, 0)),
            (
                (2, 2, 2),
                0.5,
                (0, 0, 0),
                (1, 1, 1),
                (1 + 0.5 / 3**0.5,) * 3,
                (1 / 3**0.5,) * 3,
            ),
            ((2, 2, 2), 0.5, (3, 0, 0), (-1, 0, 0), (1.5, 0, 0), (1, 0, 0)),
            ((2, 2, 2), 0, (0, 0, 0), (1, 0, 0), (1, 0, 0), (1, 0, 0)),
            ((2, 2, 2), 0, (0, 0, 0), (0, 1, 0), (0, 1, 0), (0, 1, 0)),
            ((2, 2, 2), 0, (0, 0, 0), (0, 0, 1), (0, 0, 1), (0, 0, 1)),
            ((2, 2, 2), 0, (3, 0, 0), (-1, 0, 0), (1, 0, 0), (1, 0, 0)),
            ((2, 2, 2), 0, (0, -3, 0), (0, 1, 0), (0, -1, 0), (0, -1, 0)),
            ((2, 2, 2), 0, (0, 0, 3), (0, 0, -1), (0, 0, 1), (0, 0, 1)),
        ],
    )
    def test_raycast_uses_outer_collider_surfaces(
        self, method, rotated, size, radius, origin, direction, point, normal
    ):
        root = Node()
        body = Node()
        body.collider = Collider(size=Vec3(*size), radius=radius)
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

    @pytest.mark.parametrize("axis", [Vec3.RIGHT, Vec3.UP, Vec3.FORWARD])
    @pytest.mark.parametrize("sign", [-1, 1])
    def test_raycast_from_box_surface_returns_outward_normal(self, axis, sign):
        body = Node()
        body.collider = Collider(size=Vec3(2, 2, 2), radius=0)
        point = axis * sign
        hit = body.raycast(point, -point, max_distance=0)
        assert hit is not None
        assert hit.distance == 0
        assert hit.point == point
        assert hit.normal == point

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
        ball.collider = Collider(radius=0.5, gravity=0, linear_damp=0, angular_damp=0)
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
    @pytest.mark.parametrize("scale", [1e-30, 1e-20, 1, 1e10, 1e20, 1e30, 1e37])
    @pytest.mark.parametrize("height, overlaps", [(0.5, True), (1.25, False)])
    def test_sphere_triangle_finite_geometry_range(self, scale, height, overlaps):
        root, target = Node(), Node()
        primitive = Primitive(
            Primitive.MODE_TRIANGLES,
            [-scale, 0, -scale, scale, 0, -scale, 0, 0, scale],
            [0, 1, 2],
        )
        target.collider = Collider(mesh=Mesh([primitive], [Mat4.IDENTITY], [-1]))
        root.add_child(target)
        # A radius scale sphere crosses the plane at height .5 scale, but not 1.25 scale.
        assert root.overlap_sphere(Vec3(0, height * scale, 0), scale) == (
            [target] if overlaps else []
        )

    @pytest.mark.parametrize("scale", [1, 1e20, 1e30])
    @pytest.mark.parametrize("axis", [0, 1, 2])
    @pytest.mark.parametrize("rounded", [False, True])
    @pytest.mark.parametrize(
        "center, sharp_hit, rounded_hit",
        [
            ((1.5, 0, 0), True, True),
            ((2.5, 0, 0), False, False),
            ((2, 0, 0), False, True),
            ((1.75, 1.75, 0), False, True),
            ((2, 2, 0), False, False),
            ((1.5, 1.5, 1.5), True, True),
            ((1.75, 1.75, 1.75), False, False),
            ((0, 0, 0), True, True),
        ],
    )
    def test_sphere_box_finite_range(
        self, scale, axis, rounded, center, sharp_hit, rounded_hit
    ):
        root, target = Node(), Node()
        target.collider = Collider(
            size=Vec3(2 * scale, 2 * scale, 2 * scale),
            radius=0.25 * scale if rounded else 0,
        )
        root.add_child(target)
        center = center[axis:] + center[:axis]
        # Distance is the Euclidean excess past each core face; reach is 1 or 1.25.
        overlaps = rounded_hit if rounded else sharp_hit
        assert root.overlap_sphere(Vec3(*(x * scale for x in center)), scale) == (
            [target] if overlaps else []
        )

    @pytest.mark.parametrize("scale", [1, 1e20, 1e30])
    @pytest.mark.parametrize("radius", [0.5, 0.75])
    @pytest.mark.parametrize(
        "center, overlaps",
        [
            ((1.25, 0, 0), True),
            ((1.9, 0, 0), False),
            ((1.25, 2.25, 0), True),
            ((1.6, 2.6, 0), False),
            ((0, 2.25, 0), True),
            ((0, 0, 0), True),
            ((1.5, 2.5, 1.5), False),
            ((1.25, 2.25, 1.25), True),
        ],
    )
    def test_box_capsule_finite_range(self, scale, radius, center, overlaps):
        root, target = Node(), Node()
        target.transform = Mat4.from_translation(Vec3(*(x * scale for x in center)))
        target.collider = Collider(size=Vec3(0, 2 * scale, 0), radius=radius * scale)
        root.add_child(target)
        # Axis y spans +/-scale; the nearest endpoint extends face y=scale to 2*scale.
        assert root.overlap_box(
            Mat4.IDENTITY, Vec3(2 * scale, 2 * scale, 2 * scale)
        ) == ([target] if overlaps else [])

    @pytest.mark.parametrize("scale", [1, 1e20, 1e38])
    @pytest.mark.parametrize("axis", [0, 1, 2])
    @pytest.mark.parametrize(
        "distance, overlaps", [(1.5, True), (2, False), (2.5, False)]
    )
    def test_sphere_query_finite_range(self, scale, axis, distance, overlaps):
        root, target = Node(), Node()
        target.collider = Collider(radius=scale)
        root.add_child(target)
        center = [0, 0, 0]
        center[axis] = distance * scale
        assert root.overlap_sphere(Vec3(*center), scale) == (
            [target] if overlaps else []
        )

    @pytest.mark.parametrize("scale", [1, 1e20, 1e38])
    @pytest.mark.parametrize(
        "x, y, overlaps",
        [
            (1.25, 0.5, True),
            (1.25, 2.25, False),
            (0, 2.25, True),
            (0, -2.25, True),
            (1.25, -2.25, False),
        ],
    )
    def test_capsule_query_finite_projection(self, scale, x, y, overlaps):
        root, target = Node(), Node()
        target.collider = Collider(size=Vec3(0, 2 * scale, 0), radius=0.5 * scale)
        root.add_child(target)
        # Axis ends at y=+/-scale; diagonal cap gap is hypot(1.25,1.25)*scale,
        # greater than the total radius 1.5*scale although the AABBs overlap.
        assert root.overlap_sphere(Vec3(x * scale, y * scale, 0), scale) == (
            [target] if overlaps else []
        )

    @pytest.mark.parametrize(
        "scale, center, overlaps",
        [(0.5, (0.65, 0, 0), True), (2, (2.15, 2.15, 0), False)],
    )
    def test_overlap_box_absorbs_query_scale(self, scale, center, overlaps):
        root, target = Node(), Node()
        target.transform = Mat4.from_translation(Vec3(*center))
        target.collider = Collider(radius=0.2)
        root.add_child(target)
        # Face distance .15 is inside radius .2; corner distance sqrt(2)*.15 is outside.
        assert root.overlap_box(
            Mat4.from_scale(Vec3(scale, scale, scale)), Vec3(2, 2, 2)
        ) == ([target] if overlaps else [])

    @pytest.mark.parametrize("shape", ["sphere", "capsule", "box", "mesh"])
    @pytest.mark.parametrize(
        "reflection", [(1, 1, 1), (-1, 1, 1), (1, -1, 1), (1, 1, -1)]
    )
    @pytest.mark.parametrize("distance, overlaps", [(2.1, True), (2.3, False)])
    def test_overlap_box_rotated_nonuniform_scale_for_every_shape(
        self, shape, reflection, distance, overlaps
    ):
        pose = Mat4.from_translation(Vec3(7, 8, 9)) * Mat4.from_euler(Vec3(23, 41, -17))
        query = pose.scale_by(
            Vec3(2 * reflection[0], 3 * reflection[1], 0.5 * reflection[2])
        )
        root, target = Node(), Node()
        target.transform = pose * Mat4.from_translation(Vec3(distance, 0, 0))
        if shape == "sphere":
            target.collider = Collider(radius=0.2)
        elif shape == "capsule":
            target.collider = Collider(size=Vec3(0, 0.4, 0), radius=0.2)
        elif shape == "box":
            target.collider = Collider(size=Vec3(0.4, 0.4, 0.4))
        else:
            target.collider = Collider(
                mesh=Mesh(
                    primitives=[Primitive.box(Vec3(0.4, 0.4, 0.4))],
                    transforms=[Mat4.IDENTITY],
                    parents=[-1],
                )
            )
        root.add_child(target)
        # Each target extends .2 along the rigid-frame X axis; query ends at X=2.
        assert root.overlap_box(query, Vec3(2, 2, 2)) == ([target] if overlaps else [])

    @pytest.mark.parametrize(
        "mat",
        [
            Mat4.from_scale(Vec3(1, 0, 1)),
            Mat4.from_scale(Vec3(2, 1, 1)).rotate_z(30),
            Mat4.from_translation(Vec3(1, 0, 0)).transpose(),
            Mat4.from_scale(Vec3(float("inf"), 1, 1)),
        ],
    )
    def test_overlap_box_rejects_unsupported_query_transforms(self, mat):
        with raises_exact(
            ValueError,
            "mat must be a finite affine transform with nonzero, orthogonal axes",
        ):
            Node().overlap_box(mat, Vec3.ONE)

    def test_overlap_box_accepts_composed_rotation_rounding(self):
        mat = Mat4.IDENTITY
        for _ in range(10000):
            mat = mat.rotate_x(3).rotate_y(5).rotate_z(7)
        root, target = Node(), Node()
        target.collider = Collider(radius=0.2)
        root.add_child(target)
        assert root.overlap_box(mat.scale_by(Vec3(0.5, 2, 3)), Vec3.ONE) == [target]

    def test_overlap_box_keeps_signed_and_zero_dimensions(self):
        root, target = Node(), Node()
        target.collider = Collider(radius=0.2)
        root.add_child(target)
        assert root.overlap_box(Mat4.IDENTITY, Vec3(-1, 0, 1)) == [target]

    @pytest.mark.parametrize("size", [Vec3(float("nan"), 1, 1), Vec3(1e30, 1, 1)])
    def test_overlap_box_rejects_nonfinite_scaled_dimensions(self, size):
        with raises_exact(
            ValueError, "size and mat scale must produce finite box dimensions"
        ):
            Node().overlap_box(Mat4.from_scale(Vec3(1e30, 1, 1)), size)

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
        enemy.tags = {"enemy"}
        friend = _ball(Vec3(0.5, 0, 0))
        friend.tags = {"friend"}
        root.add_child(enemy)
        root.add_child(friend)

        nodes = root.overlap_sphere(Vec3.ZERO, 1.0, tags={"enemy"})
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


def _inverse_fallback_mesh_scene(scale):
    primitive = Primitive(
        Primitive.MODE_TRIANGLES, [1, 0, 0, 1, 1, 0, 1, 0, 1], [0, 1, 2]
    )
    target = Node()
    target.transform = Mat4.from_translation(Vec3(-0.5, 0, 0)) * Mat4.from_scale(
        Vec3(scale, 1, 1)
    )
    target.collider = Collider(mesh=Mesh([primitive], [Mat4.IDENTITY], [-1]))
    root = Node()
    root.add_child(target)
    return primitive, target, root


class _ColoredBox(Node):
    def __init__(self, pos: Vec3, col: int):
        super().__init__()
        self.transform = Mat4.from_translation(pos)
        self.col = col

    def on_draw(self):
        self.shaded(False)
        self.box(Mat4.IDENTITY, Vec3(4, 4, 4), self.col)


class TestViewportClear:
    def test_builtin_text_can_draw_on_its_font_image(self):
        class Label(Node):
            def on_draw(self):
                self.text(Vec3(0, 0, -5), "AA", 0)

        font = pyxel.font
        original = list(font.data_ptr())
        expected = pyxel.Image(font.width, font.height)
        expected.blt(0, 0, font, 0, 0, font.width, font.height)
        root = Label()
        root.camera = Camera()
        root.draw(-24, -3, 64, 36, expected)
        assert list(expected.data_ptr()) != original
        try:
            root.draw(-24, -3, 64, 36, font)
            assert list(font.data_ptr()) == list(expected.data_ptr())
        finally:
            font.data_ptr()[:] = original

    @pytest.mark.parametrize("offscreen", [False, True])
    @pytest.mark.parametrize("color", [None, 3])
    @pytest.mark.parametrize(
        "viewport",
        [
            (3, 2, 6, 5),
            (-2, -1, 6, 4),
            (14, 10, 6, 5),
            (20, 20, 2, 2),
            (1, 1, 0, 4),
            (1, 1, -2, 4),
            (1, 1, 4, 0),
            (1, 1, 4, -2),
            (0, 0, 160, 120),
        ],
    )
    def test_clear_color_only_fills_viewport(self, offscreen, color, viewport):
        scene = Node()
        scene.camera = Camera()
        scene.camera.clear_color = color
        target = pyxel.Image(16, 12) if offscreen else pyxel.screen
        target.cls(7)

        if offscreen:
            scene.draw(*viewport, target)
        else:
            scene.draw(*viewport)

        left, top, width, height = viewport
        for y in range(target.height):
            for x in range(target.width):
                inside = left <= x < left + width and top <= y < top + height
                expected = color if inside and color is not None else 7
                assert target.pget(x, y) == expected, (x, y, viewport)

    @pytest.mark.parametrize("offscreen", [False, True])
    def test_later_viewport_keeps_earlier_viewport(self, offscreen):
        scene = Node()
        scene.camera = Camera()
        target = pyxel.Image(16, 12) if offscreen else pyxel.screen
        target.cls(7)
        args = (target,) if offscreen else ()

        scene.camera.clear_color = 3
        scene.draw(0, 0, 8, 12, *args)
        scene.camera.clear_color = 6
        scene.draw(8, 0, 8, 12, *args)

        for y in range(target.height):
            for x in range(target.width):
                expected = (3 if x < 8 else 6) if x < 16 and y < 12 else 7
                assert target.pget(x, y) == expected

    def test_clear_preserves_palette_and_other_2d_drawing_state(self):
        scene = Node()
        scene.camera = Camera()
        scene.camera.clear_color = 3
        target = pyxel.Image(16, 12)
        target.cls(7)
        target.pal(3, 11)
        target.clip(1, 1, 1, 1)
        target.camera(2, 3)
        target.dither(0)

        scene.draw(4, 3, 5, 4, target)
        pixels = target.data_ptr()

        # Background clearing uses viewport coordinates, independent of 2D state.
        for y in range(12):
            for x in range(16):
                expected = 11 if 4 <= x < 9 and 3 <= y < 7 else 7
                assert pixels[y * 16 + x] == expected

        # Subsequent 2D drawing still uses the original dither, offset and clip.
        target.pset(3, 4, 8)
        assert pixels[1 * 16 + 1] == 7
        target.dither(1)
        target.pset(3, 4, 3)
        target.pset(4, 4, 8)
        assert pixels[1 * 16 + 1] == 11
        assert pixels[1 * 16 + 2] == 7


class TestVisibility:
    def test_direct_subtree_draw_respects_hidden_ancestor_and_clears(self):
        parent, branch = Node(), Node()
        parent.add_child(branch)
        branch.add_child(_ColoredBox(Vec3(0, 0, -8), 11))
        parent.camera = Camera()
        parent.camera.ortho_size = 10
        parent.camera.clear_color = 0
        target = pyxel.Image(16, 16)
        target.cls(7)

        parent.visible = False
        branch.draw(0, 0, 16, 16, target)
        assert target.pget(8, 8) == 0

        parent.visible = True
        branch.draw(0, 0, 16, 16, target)
        assert target.pget(8, 8) == 11


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
