import math
from collections import Counter

import pytest
from _assertions import raises_exact  # type: ignore[reportMissingImports]
from pyxel.cube import Primitive, Vec3


def test_required_args():
    with raises_exact(
        TypeError, "Primitive.__new__() missing 1 required positional argument: 'mode'"
    ):
        Primitive(positions=[0.0, 0.0, 0.0], indices=[0])


def test_default_cull_is_back():
    p = Primitive(Primitive.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    assert p.cull == Primitive.CULL_BACK


def test_positions_proxy_in_place_write():
    p = Primitive(Primitive.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    p.positions[0] = 9.0
    assert p.positions[0] == 9.0

    p.positions.append(1.0)
    assert len(p.positions) == 10


def test_positions_proxy_extended_slice_assignment():
    p = Primitive(
        Primitive.MODE_TRIANGLES,
        [0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        [0, 1, 0],
    )
    p.positions[::2] = [10.0, 20.0, 30.0]
    assert list(p.positions) == [10.0, 1.0, 20.0, 3.0, 30.0, 5.0]

    with raises_exact(
        ValueError,
        "attempt to assign sequence of size 1 to extended slice of size 3",
    ):
        p.positions[::2] = [1.0]


def test_positions_whole_assign_rejected():
    p = Primitive(Primitive.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    with raises_exact(
        AttributeError,
        "attribute 'positions' of 'pyxel.cube.Primitive' objects is not writable",
    ):
        p.positions = [1.0, 2.0, 3.0]


def test_normals_default_empty_not_none():
    p = Primitive(Primitive.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    assert list(p.normals) == []


def test_plane_factory_builds_textured_quad():
    p = Primitive.plane(2.0, 4.0)
    assert p.mode == Primitive.MODE_TRIANGLES
    assert p.cull == Primitive.CULL_NONE
    assert list(p.positions) == [-1, 2, 0, 1, 2, 0, -1, -2, 0, 1, -2, 0]
    assert list(p.indices) == [0, 1, 2, 1, 3, 2]
    assert list(p.uvs) == [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0]
    assert list(p.normals) == [0.0, 0.0, -1.0, 0.0, 0.0, -1.0]


def test_box_factory_builds_textured_box():
    p = Primitive.box(Vec3(2.0, 4.0, 6.0))
    assert p.mode == Primitive.MODE_TRIANGLES
    assert p.cull == Primitive.CULL_BACK
    assert len(p.positions) == 72
    assert len(p.indices) == 36
    assert len(p.uvs) == 48
    assert len(p.normals) == 36
    assert list(p.positions)[:6] == [-1.0, -2.0, -3.0, 1.0, -2.0, -3.0]
    assert list(p.indices)[:6] == [0, 2, 1, 0, 3, 2]


def test_sphere_factory_builds_low_poly_sphere():
    p = Primitive.sphere(2.0)
    assert p.mode == Primitive.MODE_TRIANGLES
    assert p.cull == Primitive.CULL_BACK
    assert len(p.indices) == 240
    assert len(p.normals) == 240
    assert len(p.uvs) == len(p.positions) // 3 * 2
    assert max(abs(v) for v in p.positions) == pytest.approx(2.0, abs=1e-6)


@pytest.mark.parametrize("height,radius", [(1, 0.5), (3, 2), (0, 2), (-3, 2)])
def test_capsule_surface_normals_and_uvs(height, radius):
    p = Primitive.capsule(height, radius)
    points = list(zip(*[iter(p.positions)] * 3))
    half_height = abs(height) / 2
    assert p.mode == Primitive.MODE_TRIANGLES
    assert p.cull == Primitive.CULL_BACK
    assert min(y for x, y, z in points) == pytest.approx(-half_height - radius)
    assert max(y for x, y, z in points) == pytest.approx(half_height + radius)
    assert len(p.uvs) == len(points) * 2
    assert len(p.normals) == len(p.indices)
    assert p.indices and len(p.indices) % 3 == 0

    for i, (x, y, z) in enumerate(points):
        axis_y = max(-half_height, min(half_height, y))
        assert math.sqrt(x * x + (y - axis_y) ** 2 + z * z) == pytest.approx(radius)
        u, v = p.uvs[i * 2 : i * 2 + 2]
        assert 0 <= u <= 1
        assert v == pytest.approx(
            (half_height + radius - y) / (abs(height) + 2 * radius), abs=1e-6
        )
        if u == 0:
            assert any(
                points[j] == (x, y, z) and p.uvs[j * 2] == 1 for j in range(len(points))
            )

    edges = Counter()
    for i in range(0, len(p.indices), 3):
        triangle = [points[j] for j in p.indices[i : i + 3]]
        for a, b in zip(triangle, triangle[1:] + triangle[:1]):
            edges[tuple(sorted((a, b)))] += 1
        a, b, c = [Vec3(*points[j]) for j in p.indices[i : i + 3]]
        normal = Vec3(*p.normals[i : i + 3])
        center = (a + b + c) / 3
        outward = center - Vec3(0, max(-half_height, min(half_height, center.y)), 0)
        assert (b - a).cross(c - a).dot(outward) > 0
        assert normal.length() == pytest.approx(1, abs=1e-6)
        assert normal.dot(outward) > 0

    # Welding coincident seam/pole vertices leaves a closed surface.
    assert set(edges.values()) == {2}


def test_capsule_defaults_and_independent_geometry():
    first = Primitive.capsule()
    second = Primitive.capsule(height=1, radius=0.5)
    assert list(first.positions) == list(second.positions)
    first.positions[0] = 100
    assert second.positions[0] == 0


@pytest.mark.parametrize("height,radius", [(0, 0), (2, 0), (2, -1)])
def test_capsule_zero_radius_remains_finite(height, radius):
    p = Primitive.capsule(height, radius)
    assert all(math.isfinite(v) for v in [*p.positions, *p.uvs, *p.normals])
    assert all(v == 0 for v in p.positions[0::3])
    assert all(v == 0 for v in p.positions[2::3])
