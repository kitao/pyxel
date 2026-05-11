from pyxel.cube import Geometry


def test_default_construction():
    geom = Geometry()
    assert geom.positions == []
    assert geom.normals is None
    assert geom.uvs is None
    assert geom.indices is None
    assert geom.prim == Geometry.PRIM_TRIANGLES
    assert geom.cull == Geometry.CULL_BACK


def test_full_construction():
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


def test_attribute_assignment():
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


def test_compute_normals_flat():
    geom = Geometry(positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
    geom.compute_normals()
    assert geom.normals is not None
    assert len(geom.normals) == 9
    for i in range(3):
        nx, ny, nz = geom.normals[i * 3 : i * 3 + 3]
        assert abs(nx) < 1e-5
        assert abs(ny) < 1e-5
        assert abs(nz - 1.0) < 1e-5


def test_set_normals_to_none():
    geom = Geometry(positions=[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
    geom.compute_normals()
    assert geom.normals is not None
    geom.normals = None
    assert geom.normals is None


def test_prim_constants():
    assert Geometry.PRIM_POINTS == 0
    assert Geometry.PRIM_LINES == 1
    assert Geometry.PRIM_TRIANGLES == 2


def test_cull_constants():
    assert Geometry.CULL_NONE == 0
    assert Geometry.CULL_BACK == 1
    assert Geometry.CULL_FRONT == 2


def test_repr():
    geom = Geometry(positions=[0.0] * 9)
    r = repr(geom)
    assert "Geometry(" in r
    assert "positions=9" in r


def test_compute_normals_smooth():
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
    geom.compute_normals(smooth=True)
    assert geom.normals is not None
    # vertex 1 is shared by triangle 0 (+Z normal) and triangle 1 (+X normal),
    # so the smoothed normal at vertex 1 is the normalized average.
    nx, ny, nz = geom.normals[3:6]
    half = 1.0 / (2.0**0.5)
    assert abs(nx - half) < 1e-3
    assert abs(ny) < 1e-3
    assert abs(nz - half) < 1e-3
