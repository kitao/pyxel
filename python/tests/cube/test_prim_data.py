import pytest

from pyxel.cube import Primitive, Vec3


def test_required_args():
    with pytest.raises(
        TypeError, match="missing 1 required positional argument: 'mode'"
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


def test_positions_whole_assign_rejected():
    p = Primitive(Primitive.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    with pytest.raises(AttributeError, match="attribute 'positions'.*not writable"):
        p.positions = [1.0, 2.0, 3.0]


def test_normals_default_empty_not_none():
    p = Primitive(Primitive.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    assert list(p.normals) == []


def test_plane_factory_builds_textured_quad():
    p = Primitive.plane(2.0, 4.0)

    assert p.mode == Primitive.MODE_TRIANGLES
    assert p.cull == Primitive.CULL_NONE
    assert list(p.positions) == pytest.approx(
        [-1.0, 2.0, 0.0, 1.0, 2.0, 0.0, -1.0, -2.0, 0.0, 1.0, -2.0, 0.0]
    )
    assert list(p.indices) == [0, 1, 2, 1, 3, 2]
    assert list(p.uvs) == pytest.approx([0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0])
    assert list(p.normals) == pytest.approx([0.0, 0.0, -1.0, 0.0, 0.0, -1.0])


def test_box_factory_builds_textured_box():
    p = Primitive.box(Vec3(2.0, 4.0, 6.0))

    assert p.mode == Primitive.MODE_TRIANGLES
    assert p.cull == Primitive.CULL_BACK
    assert len(p.positions) == 72
    assert len(p.indices) == 36
    assert len(p.uvs) == 48
    assert len(p.normals) == 36
    assert list(p.positions)[:6] == pytest.approx([-1.0, -2.0, -3.0, 1.0, -2.0, -3.0])
    assert list(p.indices)[:6] == [0, 2, 1, 0, 3, 2]


def test_sphere_factory_builds_low_poly_sphere():
    p = Primitive.sphere(2.0)

    assert p.mode == Primitive.MODE_TRIANGLES
    assert p.cull == Primitive.CULL_BACK
    assert len(p.indices) == 240
    assert len(p.normals) == 240
    assert len(p.uvs) == len(p.positions) // 3 * 2
    assert max(abs(v) for v in p.positions) == pytest.approx(2.0)


def test_sphere_factory_has_fixed_shape():
    with pytest.raises(TypeError, match="unexpected keyword argument 'segments'"):
        Primitive.sphere(2.0, segments=4)


def test_cylinder_factory_is_not_part_of_core_primitive_set():
    assert not hasattr(Primitive, "cylinder")
