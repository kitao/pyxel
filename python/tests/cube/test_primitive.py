import pytest

from pyxel.cube import Primitive


def test_required_args():
    with pytest.raises(TypeError):
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
    with pytest.raises(AttributeError):
        p.positions = [1.0, 2.0, 3.0]


def test_normals_default_empty_not_none():
    p = Primitive(Primitive.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    assert list(p.normals) == []
