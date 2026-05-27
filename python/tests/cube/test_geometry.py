from pyxel.cube import Geometry


class TestConstruction:
    def test_default_construction(self):
        geom = Geometry()
        assert geom.positions == []
        assert geom.normals is None
        assert geom.uvs is None
        assert geom.indices is None
        assert geom.prim == Geometry.PRIM_TRIANGLES
        assert geom.cull == Geometry.CULL_BACK

    def test_full_construction(self):
        geom = Geometry(
            positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            indices=[0, 1, 2],
            uvs=[0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
            prim=Geometry.PRIM_TRIANGLES,
            cull=Geometry.CULL_NONE,
        )
        assert len(geom.positions) == 9
        assert geom.indices == [0, 1, 2]
        assert geom.uvs == [0.0, 0.0, 1.0, 0.0, 0.0, 1.0]
        assert geom.cull == Geometry.CULL_NONE


class TestAttributes:
    def test_attribute_assignment(self):
        geom = Geometry()
        geom.positions = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        geom.indices = [0, 1, 2]
        geom.uvs = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0]
        geom.prim = Geometry.PRIM_LINES
        geom.cull = Geometry.CULL_NONE
        assert len(geom.positions) == 9
        assert geom.indices == [0, 1, 2]
        assert geom.prim == Geometry.PRIM_LINES
        assert geom.cull == Geometry.CULL_NONE


class TestComputeNormals:
    def test_flat(self):
        # Per-face layout (cube-design.md § 9.4): one (nx, ny, nz) per
        # triangle. A single +Z-facing triangle yields 3 floats.
        geom = Geometry(positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
        geom.compute_normals()
        assert geom.normals is not None
        assert len(geom.normals) == 3
        nx, ny, nz = geom.normals
        assert abs(nx) < 1e-5
        assert abs(ny) < 1e-5
        assert abs(nz - 1.0) < 1e-5

    def test_set_to_none_clears_cache(self):
        geom = Geometry(positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
        geom.compute_normals()
        assert geom.normals is not None
        geom.normals = None
        assert geom.normals is None

    def test_two_faces_have_distinct_normals(self):
        # Triangle 0 lies in the z=0 plane (normal +Z); triangle 1 lies
        # in the x=1 plane (normal +X). Per-face normals keep them in
        # separate output slots, unlike a per-vertex layout that would
        # blend at the shared vertex 1.
        geom = Geometry(
            positions=[
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                1.0,
                0.0,
                1.0,
                1.0,
                1.0,
                0.0,
            ],
            indices=[0, 1, 2, 1, 4, 3],
        )
        geom.compute_normals()
        assert geom.normals is not None
        assert len(geom.normals) == 6
        # Face 0 = +Z.
        assert abs(geom.normals[0]) < 1e-5
        assert abs(geom.normals[1]) < 1e-5
        assert abs(geom.normals[2] - 1.0) < 1e-5
        # Face 1 = +X.
        assert abs(geom.normals[3] - 1.0) < 1e-5
        assert abs(geom.normals[4]) < 1e-5
        assert abs(geom.normals[5]) < 1e-5


class TestConstants:
    def test_prim_constants(self):
        assert Geometry.PRIM_POINTS == 0
        assert Geometry.PRIM_LINES == 1
        assert Geometry.PRIM_TRIANGLES == 2

    def test_cull_constants(self):
        assert Geometry.CULL_NONE == 0
        assert Geometry.CULL_BACK == 1
        assert Geometry.CULL_FRONT == 2


class TestRepr:
    def test_repr_includes_positions_length(self):
        geom = Geometry(positions=[0.0] * 9)
        r = repr(geom)
        assert "Geometry(" in r
        assert "positions=9" in r
