"""Tests for `pyxel.cube.Mesh`.

Mesh is a flat-buffer asset (cube-design.md § 11): positions /
indices / normals / uvs / image / colkey, all assignable through the
all-optional `__init__` or directly afterwards. The tests below cover
the round-trip behaviour and the empty-positions raise contract.
"""

import pytest

from pyxel.cube import (
    FloatBuffer,
    IntBuffer,
    Mat4,
    Mesh,
    Scene,
)


class TestConstruction:
    def test_default_empty(self):
        m = Mesh()
        # positions defaults to an empty FloatBuffer; everything else None.
        assert isinstance(m.positions, FloatBuffer)
        assert m.positions.size == 0
        assert m.indices is None
        assert m.normals is None
        assert m.uvs is None
        assert m.image is None
        assert m.colkey is None

    def test_init_all_optional_partial(self):
        # Pass only some kwargs; the rest stay at their defaults.
        positions = FloatBuffer([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
        m = Mesh(positions=positions, colkey=3)
        assert m.positions.size == 9
        assert list(m.positions) == [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        assert m.colkey == 3
        assert m.indices is None
        assert m.normals is None
        assert m.uvs is None

    def test_init_all_kwargs(self):
        positions = FloatBuffer([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
        indices = IntBuffer([0, 1, 2])
        normals = FloatBuffer([0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0])
        uvs = FloatBuffer([0.0, 0.0, 1.0, 0.0, 0.0, 1.0])
        m = Mesh(
            positions=positions,
            indices=indices,
            normals=normals,
            uvs=uvs,
            colkey=5,
        )
        assert m.positions.size == 9
        assert m.indices is not None and m.indices.size == 3
        assert m.normals is not None and m.normals.size == 9
        assert m.uvs is not None and m.uvs.size == 6
        assert m.colkey == 5

    def test_repr(self):
        m = Mesh()
        # No format guarantee beyond "Mesh(" prefix; just smoke-test.
        assert "Mesh(" in repr(m)


class TestMemberAssignment:
    def test_assign_after_construction(self):
        m = Mesh()
        m.positions = FloatBuffer([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
        m.indices = IntBuffer([0, 1, 2])
        m.colkey = 7
        assert m.positions.size == 9
        assert m.indices is not None and list(m.indices) == [0, 1, 2]
        assert m.colkey == 7

    def test_clear_optionals_with_none(self):
        m = Mesh(
            indices=IntBuffer([0, 1, 2]),
            normals=FloatBuffer([0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0]),
            uvs=FloatBuffer([0.0, 0.0, 1.0, 0.0, 0.0, 1.0]),
            colkey=3,
        )
        m.indices = None
        m.normals = None
        m.uvs = None
        m.colkey = None
        assert m.indices is None
        assert m.normals is None
        assert m.uvs is None
        assert m.colkey is None


class TestEmptyPositionsRaises:
    """Drawing an empty Mesh raises at the call site (cube-design.md § 11.2).

    A scene-rooted Node draws the mesh; without a draw context the
    binding still validates the empty-positions case before entering
    `with_draw_context`, so the raise reaches the test.
    """

    def test_default_mesh_raises(self):
        s = Scene()
        m = Mesh()
        with pytest.raises(ValueError):
            s.mesh(Mat4.IDENTITY, m)

    def test_non_empty_does_not_raise(self):
        # 1-triangle mesh: 3 vertices, 9 floats. The draw is a no-op
        # outside `Scene.draw`'s active context, but the binding-side
        # empty-check should pass and not raise.
        s = Scene()
        m = Mesh(positions=FloatBuffer([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]))
        s.mesh(Mat4.IDENTITY, m)


class TestNoLegacyApi:
    """Confirm the new spec dropped the legacy factories / per-element
    helpers / col / create_node so user code that still relies on them
    fails fast (cube-design.md § 11)."""

    def test_no_factories(self):
        for name in ("box", "sphere", "cylinder", "plane", "from_vertices"):
            assert not hasattr(Mesh, name), f"Mesh.{name} should be gone"

    def test_no_per_element_helpers(self):
        m = Mesh()
        for name in (
            "vertex_count",
            "face_count",
            "get_vertex",
            "set_vertex",
            "get_uv",
            "set_uv",
            "get_face",
            "set_face",
            "resize",
            "create_node",
            "col",
        ):
            assert not hasattr(m, name), f"Mesh.{name} should be gone"


class TestNodeMeshSignature:
    """`Node.mesh` accepts `col` (default 7) and no `colkey` (lives on
    the Mesh)."""

    def test_col_is_keyword_only_with_default(self):
        s = Scene()
        m = Mesh(positions=FloatBuffer([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]))
        # Default col=7 — call without specifying col.
        s.mesh(Mat4.IDENTITY, m)
        # Override col per call.
        s.mesh(Mat4.IDENTITY, m, col=12)

    def test_colkey_is_not_a_per_call_argument(self):
        s = Scene()
        m = Mesh(
            positions=FloatBuffer([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
            colkey=3,
        )
        with pytest.raises(TypeError):
            s.mesh(Mat4.IDENTITY, m, colkey=3)
