import pytest

from pyxel.cube import PrimData


def test_required_args():
    with pytest.raises(TypeError):
        PrimData(positions=[0.0, 0.0, 0.0], indices=[0])


def test_default_cull_is_back():
    p = PrimData(PrimData.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    assert p.cull == PrimData.CULL_BACK


def test_positions_proxy_in_place_write():
    p = PrimData(PrimData.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    p.positions[0] = 9.0
    assert p.positions[0] == 9.0
    p.positions.append(1.0)
    assert len(p.positions) == 10


def test_positions_whole_assign_rejected():
    p = PrimData(PrimData.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    with pytest.raises(AttributeError):
        p.positions = [1.0, 2.0, 3.0]


def test_normals_default_empty_not_none():
    p = PrimData(PrimData.MODE_TRIANGLES, [0.0] * 9, [0, 1, 2])
    assert list(p.normals) == []
