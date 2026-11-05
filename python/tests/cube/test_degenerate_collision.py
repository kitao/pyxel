import itertools
import math

import pytest
from pyxel.cube import Collider, Mat4, Mesh, Node, Primitive, Vec3


def make_mesh_node(vertices, order, include_face=False):
    positions = [component for vertex in vertices for component in vertex]
    indices = list(order)
    if include_face:
        positions += [-10, 0, -10, 10, 0, -10, 0, 0, 10]
        indices += [3, 4, 5]
    primitive = Primitive(Primitive.MODE_TRIANGLES, positions, indices)
    node = Node()
    node.collider = Collider(mesh=Mesh([primitive], [Mat4.IDENTITY], [-1]))
    return node


@pytest.mark.parametrize("height", [0, 0.2])
@pytest.mark.parametrize("include_face", [False, True])
@pytest.mark.parametrize("order", list(itertools.permutations(range(3))))
@pytest.mark.parametrize(
    "vertices",
    [
        [(0, 0, 0), (0, 0, 0), (1, 0, 0)],
        [(0, 0, 0), (0.5, 0, 0), (1, 0, 0)],
    ],
)
def test_collapsed_triangle_contacts_have_finite_geometry(
    height, include_face, order, vertices
):
    class Body(Node):
        def __init__(self):
            super().__init__()
            self.contacts = []

        def on_collide(self, other, contact):
            self.contacts.append(contact)
            self.transform = (
                Mat4.from_translation(contact.normal * contact.depth) * self.transform
            )
            self.collider.velocity += contact.delta_velocity

    root, body = Node(), Body()
    floor = make_mesh_node(vertices, order, include_face)
    root.add_child(floor)
    root.add_child(body)
    body.transform = Mat4.from_translation(Vec3(0.25, 0.25, 0))
    body.collider = Collider(
        size=Vec3(0, height, 0), radius=0.5, gravity=0, linear_damp=0
    )

    root.update()

    assert body.contacts
    for contact in body.contacts:
        assert all(
            math.isfinite(value)
            for vector in (
                contact.point,
                contact.normal,
                contact.delta_velocity,
                contact.delta_angular_velocity,
            )
            for value in vector
        )
        assert math.isfinite(contact.depth)
        assert tuple(contact.point) == pytest.approx((0.25, 0, 0), abs=1e-6)
        assert tuple(contact.normal) == pytest.approx((0, 1, 0), abs=1e-6)
    assert tuple(body.transform.pos) == pytest.approx(
        (0.25, 0.5 + height / 2, 0), abs=1e-6
    )


@pytest.mark.parametrize("height", [0, 0.2])
@pytest.mark.parametrize("center", [(0, 0, 0), (0, 0.25, 0)])
def test_point_triangle_contact_remains_finite(height, center):
    class Body(Node):
        def __init__(self):
            super().__init__()
            self.contacts = []

        def on_collide(self, other, contact):
            self.contacts.append(contact)

    root, body = Node(), Body()
    floor = make_mesh_node([(0, 0, 0)] * 3, (0, 1, 2))
    root.add_child(floor)
    root.add_child(body)
    body.transform = Mat4.from_translation(Vec3(*center))
    body.collider = Collider(
        size=Vec3(0, height, 0), radius=0.5, gravity=0, linear_damp=0
    )

    root.update()

    assert body.contacts
    for contact in body.contacts:
        assert tuple(contact.point) == (0, 0, 0)
        assert all(math.isfinite(value) for value in contact.normal)
        assert contact.normal.length() == pytest.approx(1, abs=1e-6)
        assert math.isfinite(contact.depth)
        assert contact.depth <= 0.5 + 1e-6


@pytest.mark.parametrize("order", list(itertools.permutations(range(3))))
def test_collapsed_triangle_does_not_overlap_a_distant_sphere(order):
    floor = make_mesh_node([(0, 0, 0), (0, 0, 0), (1, 0, 0)], order)
    assert floor.overlap_sphere(Vec3(3, 0.25, 0), 0.5) == []
    assert floor.overlap_sphere(Vec3(0.25, 0.25, 0), 0.5) == [floor]


@pytest.mark.parametrize("scale", [1e-5, 1, 1e10, 1e30])
@pytest.mark.parametrize("height", [0, 2])
@pytest.mark.parametrize(
    "direction",
    [(1, 0, 0), (0, 1, 0), (1, 2, 3), (1e-3, 1, 0), (1e-5, 1, 0), (1e-10, 1, 0)],
)
@pytest.mark.parametrize("order", list(itertools.permutations(range(3))))
@pytest.mark.parametrize("middle", [0, 1])
def test_coincident_mesh_cores_separate_without_depth_amplification(
    scale, height, direction, order, middle
):
    class Body(Node):
        def __init__(self):
            super().__init__()
            self.contacts = []

        def on_collide(self, other, contact):
            self.contacts.append(contact)
            self.transform = (
                Mat4.from_translation(contact.normal * contact.depth) * self.transform
            )
            self.collider.velocity += contact.delta_velocity

    start = tuple(-100 * scale * value for value in direction)
    end = tuple(100 * scale * value for value in direction)
    floor = make_mesh_node([start, (start, (0, 0, 0))[middle], end], order)
    root, body = Node(), Body()
    root.add_child(floor)
    root.add_child(body)
    body.collider = Collider(
        size=Vec3(0, height * scale, 0),
        radius=0.5 * scale,
        gravity=0,
        linear_damp=0,
    )

    root.update()

    assert body.contacts
    line = Vec3(*direction).normalize()
    for contact in body.contacts:
        normal = contact.normal
        assert all(math.isfinite(value) for value in (*contact.point, *normal))
        assert normal.length() == pytest.approx(1, abs=1e-6)
        assert normal.dot(line) == pytest.approx(0, abs=1e-6)
        if height:
            assert normal.y == pytest.approx(0, abs=1e-6)
        assert math.isfinite(contact.depth)
        assert contact.depth / scale <= 0.5 + 1e-5
    displacement = body.transform.pos / scale
    assert displacement.length() == pytest.approx(0.5, abs=1e-5)

    # Move just beyond touching, then require the same scene path to be clear.
    padding = body.contacts[0].normal * (1e-4 * scale)
    body.transform = Mat4.from_translation(padding) * body.transform
    body.contacts.clear()
    root.update()
    assert body.contacts == []


@pytest.mark.parametrize("body_height", [0, 2])
@pytest.mark.parametrize("rotation", [Vec3.ZERO, Vec3(0, 0, 90), Vec3(30, 20, 10)])
def test_coincident_analytic_capsules_have_valid_separation(body_height, rotation):
    class Body(Node):
        def __init__(self):
            super().__init__()
            self.contacts = []

        def on_collide(self, other, contact):
            self.contacts.append(contact)
            self.transform = (
                Mat4.from_translation(contact.normal * contact.depth) * self.transform
            )
            self.collider.velocity += contact.delta_velocity

    root, fixed, body = Node(), Node(), Body()
    root.add_child(fixed)
    root.add_child(body)
    fixed.transform = Mat4.from_euler(rotation)
    fixed.collider = Collider(size=Vec3(0, 100, 0), radius=0.5, mass=0)
    body.collider = Collider(
        size=Vec3(0, body_height, 0), radius=0.5, gravity=0, linear_damp=0
    )

    root.update()

    assert body.contacts
    fixed_axis = Vec3.UP.to_world_dir(fixed.transform)
    for contact in body.contacts:
        assert contact.normal.length() == pytest.approx(1, abs=1e-6)
        assert contact.normal.dot(fixed_axis) == pytest.approx(0, abs=1e-6)
        if body_height:
            assert contact.normal.y == pytest.approx(0, abs=1e-6)
        assert contact.depth <= 1 + 1e-5
    assert body.transform.pos.length() == pytest.approx(1, abs=1e-5)

    body.transform = (
        Mat4.from_translation(body.contacts[0].normal * 1e-4) * body.transform
    )
    body.contacts.clear()
    root.update()
    assert body.contacts == []
