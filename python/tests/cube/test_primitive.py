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
