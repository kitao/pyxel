from pyxel.p3d import Mat4, Vec3


class TestVec3:
    def test_new(self):
        v = Vec3(1.0, 2.0, 3.0)
        assert v.x == 1.0
        assert v.y == 2.0
        assert v.z == 3.0

    def test_add(self):
        a = Vec3(1.0, 2.0, 3.0)
        b = Vec3(4.0, 5.0, 6.0)
        c = a + b
        assert c.x == 5.0
        assert c.y == 7.0
        assert c.z == 9.0

    def test_sub(self):
        a = Vec3(4.0, 5.0, 6.0)
        b = Vec3(1.0, 2.0, 3.0)
        c = a - b
        assert c.x == 3.0
        assert c.y == 3.0
        assert c.z == 3.0

    def test_mul_scalar(self):
        v = Vec3(1.0, 2.0, 3.0)
        r = v * 2.0
        assert r.x == 2.0
        assert r.y == 4.0
        assert r.z == 6.0

    def test_neg(self):
        v = Vec3(1.0, -2.0, 3.0)
        r = -v
        assert r.x == -1.0
        assert r.y == 2.0
        assert r.z == -3.0

    def test_dot(self):
        a = Vec3(1.0, 0.0, 0.0)
        b = Vec3(0.0, 1.0, 0.0)
        assert a.dot(b) == 0.0
        assert a.dot(a) == 1.0

    def test_cross(self):
        x = Vec3(1.0, 0.0, 0.0)
        y = Vec3(0.0, 1.0, 0.0)
        z = x.cross(y)
        assert z.x == 0.0
        assert z.y == 0.0
        assert z.z == 1.0

    def test_length(self):
        v = Vec3(3.0, 4.0, 0.0)
        assert v.length() == 5.0

    def test_normalize(self):
        v = Vec3(0.0, 0.0, 5.0)
        n = v.normalize()
        assert abs(n.z - 1.0) < 1e-6
        assert abs(n.length() - 1.0) < 1e-6

    def test_normalize_zero(self):
        v = Vec3(0.0, 0.0, 0.0)
        n = v.normalize()
        assert n.x == 0.0
        assert n.y == 0.0
        assert n.z == 0.0


class TestMat4:
    def test_identity(self):
        m = Mat4.identity()
        v = Vec3(1.0, 2.0, 3.0)
        r = m.transform_point(v)
        assert abs(r.x - 1.0) < 1e-6
        assert abs(r.y - 2.0) < 1e-6
        assert abs(r.z - 3.0) < 1e-6

    def test_translation(self):
        m = Mat4.translation(10.0, 20.0, 30.0)
        v = Vec3(0.0, 0.0, 0.0)
        r = m.transform_point(v)
        assert abs(r.x - 10.0) < 1e-6
        assert abs(r.y - 20.0) < 1e-6
        assert abs(r.z - 30.0) < 1e-6

    def test_scale(self):
        m = Mat4.scale(2.0, 3.0, 4.0)
        v = Vec3(1.0, 1.0, 1.0)
        r = m.transform_point(v)
        assert abs(r.x - 2.0) < 1e-6
        assert abs(r.y - 3.0) < 1e-6
        assert abs(r.z - 4.0) < 1e-6

    def test_rotation_z_90(self):
        m = Mat4.rotation_z(90.0)
        v = Vec3(1.0, 0.0, 0.0)
        r = m.transform_point(v)
        assert abs(r.x) < 1e-5
        assert abs(r.y - 1.0) < 1e-5

    def test_multiply(self):
        t = Mat4.translation(5.0, 0.0, 0.0)
        s = Mat4.scale(2.0, 2.0, 2.0)
        m = t * s  # first scale, then translate
        v = Vec3(1.0, 0.0, 0.0)
        r = m.transform_point(v)
        assert abs(r.x - 7.0) < 1e-5  # 1*2 + 5 = 7
