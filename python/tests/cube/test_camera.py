from math import isclose

from pyxel.cube import Camera, Mat4, Vec3


class TestDefault:
    def test_fov(self):
        assert Camera().fov == 60.0

    def test_near_far(self):
        c = Camera()
        assert isclose(c.near, 0.1, abs_tol=1e-6)
        assert c.far == 1000.0

    def test_ortho_size_none(self):
        assert Camera().ortho_size is None

    def test_transform_identity(self):
        m = Camera().transform
        for i in range(4):
            for j in range(4):
                assert m[i, j] == (1.0 if i == j else 0.0)


class TestMutation:
    def test_set_fov(self):
        c = Camera()
        c.fov = 90.0
        assert c.fov == 90.0

    def test_set_near_far(self):
        c = Camera()
        c.near = 0.5
        c.far = 500.0
        assert c.near == 0.5
        assert c.far == 500.0

    def test_set_ortho_size(self):
        c = Camera()
        c.ortho_size = 10.0
        assert c.ortho_size == 10.0
        c.ortho_size = None
        assert c.ortho_size is None

    def test_set_transform(self):
        c = Camera()
        c.transform = Mat4.from_translation(Vec3(1, 2, 3))
        assert c.transform.pos == Vec3(1, 2, 3)


class TestRepr:
    def test_includes_attributes(self):
        c = Camera()
        c.fov = 75.0
        c.ortho_size = 5.0
        r = repr(c)
        assert "75" in r
        assert "5" in r
        assert "ortho_size=" in r

    def test_none_ortho_size(self):
        r = repr(Camera())
        assert "ortho_size=None" in r
