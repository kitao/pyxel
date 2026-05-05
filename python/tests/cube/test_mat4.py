from math import isclose

import pytest

from pyxel.cube import Mat4, Quat, Vec3


def approx_v(a, b, tol=1e-4):
    return isclose(a.x, b.x, abs_tol=tol) and isclose(a.y, b.y, abs_tol=tol) and isclose(
        a.z, b.z, abs_tol=tol
    )


def approx_m(a, b, tol=1e-4):
    for i in range(4):
        for j in range(4):
            if not isclose(a[i, j], b[i, j], abs_tol=tol):
                return False
    return True


class TestConstructor:
    def test_default_is_identity(self):
        m = Mat4()
        for i in range(4):
            for j in range(4):
                expected = 1.0 if i == j else 0.0
                assert m[i, j] == expected


class TestConstants:
    def test_identity(self):
        m = Mat4.IDENTITY
        for i in range(4):
            for j in range(4):
                assert m[i, j] == (1.0 if i == j else 0.0)


class TestElementAccess:
    def test_getitem_tuple(self):
        m = Mat4.from_translation(Vec3(5, 6, 7))
        assert m[0, 3] == 5.0
        assert m[1, 3] == 6.0
        assert m[2, 3] == 7.0

    def test_getitem_out_of_range(self):
        with pytest.raises(IndexError):
            _ = Mat4.IDENTITY[4, 0]


class TestDecomposed:
    def test_pos_after_translation(self):
        m = Mat4.from_translation(Vec3(1, 2, 3))
        assert m.pos == Vec3(1, 2, 3)

    def test_scale_after_from_scale(self):
        m = Mat4.from_scale(Vec3(2, 3, 4))
        assert approx_v(m.scale, Vec3(2, 3, 4))

    def test_rot_zero_after_translation(self):
        m = Mat4.from_translation(Vec3(1, 2, 3))
        assert approx_v(m.rot, Vec3(0, 0, 0))

    def test_compose_round_trip(self):
        pos = Vec3(1, 2, 3)
        rot = Vec3(0, 45, 0)
        scale = Vec3(2, 2, 2)
        m = Mat4.compose(pos, rot, scale)
        assert approx_v(m.pos, pos)
        assert approx_v(m.scale, scale)
        assert approx_v(m.rot, rot)


class TestOperators:
    def test_mul_identity(self):
        m = Mat4.from_translation(Vec3(1, 2, 3))
        assert approx_m(m * Mat4.IDENTITY, m)

    def test_mul_vec_translation(self):
        m = Mat4.from_translation(Vec3(10, 20, 30))
        assert m * Vec3(1, 2, 3) == Vec3(11, 22, 33)

    def test_mul_vec_rotation_y90(self):
        m = Mat4.IDENTITY.rotate_y(90)
        assert approx_v(m * Vec3(1, 0, 0), Vec3(0, 0, -1))

    def test_eq(self):
        assert Mat4.IDENTITY == Mat4.IDENTITY
        assert Mat4.IDENTITY != Mat4.from_translation(Vec3(1, 0, 0))


class TestFactories:
    def test_from_translation(self):
        m = Mat4.from_translation(Vec3(1, 2, 3))
        assert m.pos == Vec3(1, 2, 3)

    def test_from_rotation_y90(self):
        m = Mat4.from_rotation(Vec3(0, 90, 0))
        assert approx_v(m * Vec3(1, 0, 0), Vec3(0, 0, -1))

    def test_from_scale(self):
        m = Mat4.from_scale(Vec3(2, 3, 4))
        assert approx_v(m.scale, Vec3(2, 3, 4))

    def test_from_quat_round_trip(self):
        q = Quat.from_axis_angle(Vec3.UP, 90)
        m = Mat4.from_quat(q)
        assert approx_v(m * Vec3(1, 0, 0), q * Vec3(1, 0, 0))

    def test_compose(self):
        pos = Vec3(1, 2, 3)
        rot = Vec3(0, 0, 0)
        scale = Vec3(2, 2, 2)
        m = Mat4.compose(pos, rot, scale)
        assert m.pos == pos

    def test_look_at_translation(self):
        eye = Vec3(0, 0, 5)
        m = Mat4.look_at(eye, Vec3(0, 0, 0))
        assert approx_v(m.pos, eye)


class TestMutate:
    def test_translate(self):
        m = Mat4().translate(Vec3(1, 2, 3))
        assert m.pos == Vec3(1, 2, 3)

    def test_rotate_x(self):
        m = Mat4().rotate_x(90)
        assert approx_v(m * Vec3(0, 1, 0), Vec3(0, 0, 1))

    def test_rotate_y(self):
        m = Mat4().rotate_y(90)
        assert approx_v(m * Vec3(1, 0, 0), Vec3(0, 0, -1))

    def test_rotate_z(self):
        m = Mat4().rotate_z(90)
        assert approx_v(m * Vec3(1, 0, 0), Vec3(0, 1, 0))

    def test_rotate_arbitrary_axis(self):
        m = Mat4().rotate(Vec3.UP, 90)
        assert approx_v(m * Vec3(1, 0, 0), Vec3(0, 0, -1))

    def test_scale_by(self):
        m = Mat4().scale_by(Vec3(2, 3, 4))
        assert approx_v(m.scale, Vec3(2, 3, 4))


class TestMatrixOps:
    def test_inverse_round_trip(self):
        m = Mat4.compose(Vec3(1, 2, 3), Vec3(30, 45, 60), Vec3(1.5, 2, 0.5))
        identity_back = m * m.inverse()
        assert approx_m(identity_back, Mat4.IDENTITY)

    def test_transpose(self):
        m = Mat4.from_translation(Vec3(1, 2, 3))
        t = m.transpose()
        for i in range(4):
            for j in range(4):
                assert t[i, j] == m[j, i]

    def test_determinant_identity(self):
        assert isclose(Mat4.IDENTITY.determinant(), 1.0, abs_tol=1e-6)

    def test_determinant_scale(self):
        assert isclose(Mat4.from_scale(Vec3(2, 3, 4)).determinant(), 24.0, abs_tol=1e-4)


class TestCoordinateConversions:
    def test_mat_to_world(self):
        inner = Mat4.from_translation(Vec3(5, 0, 0))
        outer = Mat4.from_translation(Vec3(10, 0, 0))
        assert inner.to_world(outer).pos == Vec3(15, 0, 0)

    def test_mat_to_local_round_trip(self):
        outer = Mat4.from_translation(Vec3(10, 0, 0))
        inner = Mat4.from_translation(Vec3(15, 0, 0))
        local = inner.to_local(outer)
        assert approx_v(local.pos, Vec3(5, 0, 0))
