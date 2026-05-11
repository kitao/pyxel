"""Tests for `pyxel.cube.Mesh`.

Mesh is a hierarchical 3D model asset (cube-design.md § 11): parallel
arrays of geometries / transforms / parents, plus shared col_img and
colkey. Topological order (parents[i] < i, with -1 marking roots) is
enforced at construction.
"""

import pytest

from pyxel.cube import Geometry, Mat4, Mesh


def _square_geom() -> Geometry:
    return Geometry(
        positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        indices=[0, 1, 2],
    )


class TestConstruction:
    def test_default_empty(self):
        m = Mesh()
        assert m.geometries == []
        assert m.transforms == []
        assert m.parents == []
        # Default col_img is the flat color 7.
        assert m.col_img == 7
        assert m.colkey is None

    def test_full_kwargs(self):
        g0 = _square_geom()
        g1 = _square_geom()
        m = Mesh(
            geometries=[g0, g1, None],
            transforms=[Mat4(), Mat4(), Mat4()],
            parents=[-1, 0, 1],
            col_img=8,
            colkey=0,
        )
        assert len(m.geometries) == 3
        assert m.geometries[2] is None
        assert m.parents == [-1, 0, 1]
        assert m.col_img == 8
        assert m.colkey == 0

    def test_topological_order_violation_rejected(self):
        g = _square_geom()
        with pytest.raises(ValueError):
            Mesh(
                geometries=[g, g],
                transforms=[Mat4(), Mat4()],
                parents=[1, -1],  # parents[0] = 1 violates parents[i] < i
            )

    def test_parallel_array_length_mismatch_rejected(self):
        g = _square_geom()
        with pytest.raises(ValueError):
            Mesh(
                geometries=[g, g],
                transforms=[Mat4()],  # one short
                parents=[-1, 0],
            )

    def test_invalid_parent_index_rejected(self):
        g = _square_geom()
        with pytest.raises(ValueError):
            Mesh(
                geometries=[g],
                transforms=[Mat4()],
                parents=[-2],  # only -1 is valid for "no parent"
            )

    def test_construct_with_none_geometries(self):
        # geometries[i] = None represents a pure transform group with no
        # geometry of its own; useful as a joint / pivot for descendants.
        m = Mesh(
            geometries=[None, _square_geom()],
            transforms=[Mat4(), Mat4()],
            parents=[-1, 0],
        )
        assert m.geometries[0] is None
        assert m.geometries[1] is not None


class TestAttributes:
    def test_set_col_img_int(self):
        m = Mesh()
        m.col_img = 5
        assert m.col_img == 5

    def test_set_colkey(self):
        m = Mesh()
        m.colkey = 0
        assert m.colkey == 0
        m.colkey = None
        assert m.colkey is None

    def test_set_geometries_revalidates(self):
        g = _square_geom()
        m = Mesh(
            geometries=[g],
            transforms=[Mat4()],
            parents=[-1],
        )
        # Reassigning geometries to a different length without also
        # updating transforms / parents must raise.
        with pytest.raises(ValueError):
            m.geometries = [g, g]


class TestDescendants:
    def test_subtree(self):
        g = _square_geom()
        m = Mesh(
            geometries=[g, g, g, g],
            transforms=[Mat4(), Mat4(), Mat4(), Mat4()],
            parents=[-1, 0, 0, 2],
        )
        # Tree: 0 (root) -> 1, 2; 2 -> 3.
        assert m.descendants(0) == [1, 2, 3]
        assert m.descendants(2) == [3]
        assert m.descendants(3) == []

    def test_out_of_range(self):
        g = _square_geom()
        m = Mesh(geometries=[g], transforms=[Mat4()], parents=[-1])
        assert m.descendants(-1) == []
        assert m.descendants(5) == []


class TestRepr:
    def test_repr_includes_part_count(self):
        g = _square_geom()
        m = Mesh(
            geometries=[g, g],
            transforms=[Mat4(), Mat4()],
            parents=[-1, 0],
        )
        r = repr(m)
        assert "Mesh(" in r
        assert "parts=2" in r
