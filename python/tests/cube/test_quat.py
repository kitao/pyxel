from math import isclose, sqrt

import pytest

from pyxel.cube import Quat, Vec3


def approx_v(a, b, tol=1e-4):
    return isclose(a.x, b.x, abs_tol=tol) and isclose(a.y, b.y, abs_tol=tol) and isclose(
        a.z, b.z, abs_tol=tol
    )


def approx_q(a, b, tol=1e-4):
    return (
        isclose(a.x, b.x, abs_tol=tol)
        and isclose(a.y, b.y, abs_tol=tol)
        and isclose(a.z, b.z, abs_tol=tol)
        and isclose(a.w, b.w, abs_tol=tol)
    )


class TestConstructor:
    def test_default_is_identity(self):
        q = Quat()
        assert q.x == 0.0
        assert q.y == 0.0
        assert q.z == 0.0
        assert q.w == 1.0

    def test_explicit(self):
        q = Quat(1, 2, 3, 4)
        assert q.x == 1.0
        assert q.y == 2.0
        assert q.z == 3.0
        assert q.w == 4.0


class TestConstants:
    def test_identity(self):
        assert Quat.IDENTITY == Quat(0, 0, 0, 1)


class TestSequence:
    def test_getitem(self):
        q = Quat(1, 2, 3, 4)
        assert q[0] == 1.0
        assert q[1] == 2.0
        assert q[2] == 3.0
        assert q[3] == 4.0

    def test_getitem_out_of_range(self):
        with pytest.raises(IndexError):
            _ = Quat(1, 2, 3, 4)[4]

    def test_iter(self):
        assert list(Quat(1, 2, 3, 4)) == [1.0, 2.0, 3.0, 4.0]

    def test_len(self):
        assert len(Quat()) == 4


class TestOperators:
    def test_neg(self):
        q = Quat(1, 2, 3, 4)
        assert -q == Quat(-1, -2, -3, -4)

    def test_mul_identity(self):
        q = Quat.from_axis_angle(Vec3.UP, 45)
        assert approx_q(q * Quat.IDENTITY, q)

    def test_mul_vec_y90(self):
        q = Quat.from_axis_angle(Vec3.UP, 90)
        assert approx_v(q * Vec3(1, 0, 0), Vec3(0, 0, -1))

    def test_eq(self):
        assert Quat(1, 2, 3, 4) == Quat(1, 2, 3, 4)
        assert Quat(1, 2, 3, 4) != Quat(1, 2, 3, 5)


class TestFactories:
    def test_from_axis_angle_y90(self):
        q = Quat.from_axis_angle(Vec3.UP, 90)
        s = sqrt(0.5)
        assert approx_q(q, Quat(0, s, 0, s))

    def test_from_euler_y90(self):
        q = Quat.from_euler(Vec3(0, 90, 0))
        assert approx_v(q * Vec3(1, 0, 0), Vec3(0, 0, -1))

    def test_from_two_vectors(self):
        q = Quat.from_two_vectors(Vec3(1, 0, 0), Vec3(0, 1, 0))
        assert approx_v(q * Vec3(1, 0, 0), Vec3(0, 1, 0))

    def test_from_matrix_round_trip(self):
        q1 = Quat.from_axis_angle(Vec3.UP, 30)
        m = q1.to_matrix()
        q2 = Quat.from_matrix(m)
        assert approx_v(q1 * Vec3(1, 0, 0), q2 * Vec3(1, 0, 0))


class TestUnary:
    def test_conjugate(self):
        q = Quat(1, 2, 3, 4)
        assert q.conjugate() == Quat(-1, -2, -3, 4)

    def test_inverse_unit(self):
        q = Quat.from_axis_angle(Vec3.UP, 60)
        assert approx_q(q * q.inverse(), Quat.IDENTITY)

    def test_normalize(self):
        q = Quat(2, 0, 0, 0).normalize()
        assert isclose(q.length(), 1.0, abs_tol=1e-6)

    def test_normalize_zero(self):
        assert Quat(0, 0, 0, 0).normalize() == Quat.IDENTITY

    def test_length(self):
        assert isclose(Quat(1, 2, 2, 4).length(), 5.0, abs_tol=1e-5)

    def test_length_squared(self):
        assert Quat(1, 2, 2, 4).length_squared() == 25.0


class TestBinary:
    def test_dot(self):
        a = Quat(1, 2, 3, 4)
        b = Quat(5, 6, 7, 8)
        assert a.dot(b) == 70.0

    def test_angle_to_identity(self):
        assert isclose(Quat.IDENTITY.angle_to(Quat.IDENTITY), 0.0, abs_tol=1e-3)


class TestConversions:
    def test_to_matrix_round_trip(self):
        q = Quat.from_axis_angle(Vec3.UP, 30)
        m = q.to_matrix()
        assert approx_v(m * Vec3(1, 0, 0), q * Vec3(1, 0, 0))

    def test_to_axis_angle(self):
        q = Quat.from_axis_angle(Vec3.UP, 60)
        axis, deg = q.to_axis_angle()
        assert isclose(deg, 60.0, abs_tol=1e-3)
        assert approx_v(axis, Vec3.UP)

    def test_to_euler_y90(self):
        q = Quat.from_axis_angle(Vec3.UP, 90)
        euler = q.to_euler()
        assert isclose(euler.y, 90.0, abs_tol=1e-3)


class TestInterpolation:
    def test_slerp_endpoints(self):
        a = Quat.from_axis_angle(Vec3.UP, 0)
        b = Quat.from_axis_angle(Vec3.UP, 90)
        assert approx_q(a.slerp(b, 0.0), a)
        assert approx_q(a.slerp(b, 1.0), b)

    def test_slerp_midpoint_rotates_half(self):
        a = Quat.from_axis_angle(Vec3.UP, 0)
        b = Quat.from_axis_angle(Vec3.UP, 90)
        mid = a.slerp(b, 0.5)
        # 45 deg rotation of (1, 0, 0) → (cos45, 0, -sin45)
        s = sqrt(0.5)
        assert approx_v(mid * Vec3(1, 0, 0), Vec3(s, 0, -s))
