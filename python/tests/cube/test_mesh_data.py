import pytest

from pyxel.cube import Mat4, Mesh, Primitive


def _square_prim() -> Primitive:
    return Primitive(
        Primitive.MODE_TRIANGLES,
        [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        [0, 1, 2],
    )


class TestConstruction:
    def test_default_empty(self):
        m = Mesh()
        assert m.primitives == []
        assert m.transforms == []
        assert m.parents == []
        assert m.names == []
        # Default col_img is the flat color 7.
        assert m.col_img == 7
        assert m.colkey is None

    def test_full_kwargs(self):
        p0 = _square_prim()
        p1 = _square_prim()
        m = Mesh(
            primitives=[p0, p1, None],
            transforms=[Mat4(), Mat4(), Mat4()],
            parents=[-1, 0, 1],
            names=["body", "head", "hat"],
            col_img=8,
            colkey=0,
        )
        assert len(m.primitives) == 3
        assert m.primitives[2] is None
        assert m.parents == [-1, 0, 1]
        assert m.names == ["body", "head", "hat"]
        assert m.col_img == 8
        assert m.colkey == 0

    def test_topological_order_violation_rejected(self):
        p = _square_prim()
        with pytest.raises(ValueError):
            Mesh(
                primitives=[p, p],
                transforms=[Mat4(), Mat4()],
                parents=[1, -1],  # parents[0] = 1 violates parents[i] < i
            )

    def test_parallel_array_length_mismatch_rejected(self):
        p = _square_prim()
        with pytest.raises(ValueError):
            Mesh(
                primitives=[p, p],
                transforms=[Mat4()],  # one short
                parents=[-1, 0],
            )

    def test_invalid_parent_index_rejected(self):
        p = _square_prim()
        with pytest.raises(ValueError):
            Mesh(
                primitives=[p],
                transforms=[Mat4()],
                parents=[-2],  # only -1 is valid for "no parent"
            )

    def test_construct_with_none_prims(self):
        # primitives[i] = None represents a pure transform group with no
        # primitive of its own; useful as a joint / pivot for descendants.
        m = Mesh(
            primitives=[None, _square_prim()],
            transforms=[Mat4(), Mat4()],
            parents=[-1, 0],
        )
        assert m.primitives[0] is None
        assert m.primitives[1] is not None

    def test_names_default_to_empty_strings_for_parts(self):
        p = _square_prim()
        m = Mesh(
            primitives=[p, None],
            transforms=[Mat4(), Mat4()],
            parents=[-1, 0],
        )
        assert m.names == ["", ""]

    def test_names_length_mismatch_rejected(self):
        p = _square_prim()
        with pytest.raises(ValueError):
            Mesh(
                primitives=[p, p],
                transforms=[Mat4(), Mat4()],
                parents=[-1, 0],
                names=["root"],
            )


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

    def test_set_prims_revalidates(self):
        p = _square_prim()
        m = Mesh(
            primitives=[p],
            transforms=[Mat4()],
            parents=[-1],
        )
        # Reassigning primitives to a different length without also updating
        # transforms / parents must raise.
        with pytest.raises(ValueError):
            m.primitives = [p, p]

    def test_set_names_revalidates(self):
        p = _square_prim()
        m = Mesh(
            primitives=[p],
            transforms=[Mat4()],
            parents=[-1],
        )
        with pytest.raises(ValueError):
            m.names = ["root", "extra"]


class TestDescendants:
    def test_subtree(self):
        p = _square_prim()
        m = Mesh(
            primitives=[p, p, p, p],
            transforms=[Mat4(), Mat4(), Mat4(), Mat4()],
            parents=[-1, 0, 0, 2],
        )
        # Tree: 0 (root) -> 1, 2; 2 -> 3.
        assert m.descendants(0) == [1, 2, 3]
        assert m.descendants(2) == [3]
        assert m.descendants(3) == []

    def test_out_of_range(self):
        p = _square_prim()
        m = Mesh(primitives=[p], transforms=[Mat4()], parents=[-1])
        assert m.descendants(-1) == []
        assert m.descendants(5) == []


class TestRepr:
    def test_repr_includes_part_count(self):
        p = _square_prim()
        m = Mesh(
            primitives=[p, p],
            transforms=[Mat4(), Mat4()],
            parents=[-1, 0],
        )
        r = repr(m)
        assert "Mesh(" in r
        assert "parts=2" in r
