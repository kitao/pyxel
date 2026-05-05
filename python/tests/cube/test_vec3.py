from math import isclose, sqrt

import pytest

from pyxel.cube import Mat4, Vec3


def approx(a, b, tol=1e-5):
    return isclose(a.x, b.x, abs_tol=tol) and isclose(a.y, b.y, abs_tol=tol) and isclose(
        a.z, b.z, abs_tol=tol
    )


class TestConstructor:
    def test_default(self):
        v = Vec3()
        assert v.x == 0.0
        assert v.y == 0.0
        assert v.z == 0.0

    def test_explicit(self):
        v = Vec3(1, 2, 3)
        assert v.x == 1.0
        assert v.y == 2.0
        assert v.z == 3.0

    def test_attributes_are_floats(self):
        v = Vec3(1, 2, 3)
        assert isinstance(v.x, float)


class TestConstants:
    def test_zero(self):
        assert Vec3.ZERO == Vec3(0, 0, 0)

    def test_one(self):
        assert Vec3.ONE == Vec3(1, 1, 1)

    def test_axis_constants(self):
        assert Vec3.RIGHT == Vec3(1, 0, 0)
        assert Vec3.LEFT == Vec3(-1, 0, 0)
        assert Vec3.UP == Vec3(0, 1, 0)
        assert Vec3.DOWN == Vec3(0, -1, 0)
        assert Vec3.FORWARD == Vec3(0, 0, -1)
        assert Vec3.BACK == Vec3(0, 0, 1)


class TestSequence:
    def test_getitem(self):
        v = Vec3(1, 2, 3)
        assert v[0] == 1.0
        assert v[1] == 2.0
        assert v[2] == 3.0

    def test_getitem_out_of_range(self):
        v = Vec3(1, 2, 3)
        with pytest.raises(IndexError):
            _ = v[3]

    def test_iter(self):
        v = Vec3(1, 2, 3)
        assert list(v) == [1.0, 2.0, 3.0]

    def test_len(self):
        assert len(Vec3()) == 3


class TestOperators:
    def test_add(self):
        assert Vec3(1, 2, 3) + Vec3(4, 5, 6) == Vec3(5, 7, 9)

    def test_sub(self):
        assert Vec3(4, 5, 6) - Vec3(1, 2, 3) == Vec3(3, 3, 3)

    def test_mul_scalar(self):
        assert Vec3(1, 2, 3) * 2 == Vec3(2, 4, 6)

    def test_rmul_scalar(self):
        assert 2.0 * Vec3(1, 2, 3) == Vec3(2, 4, 6)

    def test_truediv(self):
        assert Vec3(2, 4, 6) / 2 == Vec3(1, 2, 3)

    def test_neg(self):
        assert -Vec3(1, 2, 3) == Vec3(-1, -2, -3)

    def test_eq(self):
        assert Vec3(1, 2, 3) == Vec3(1, 2, 3)
        assert Vec3(1, 2, 3) != Vec3(1, 2, 4)


class TestMath:
    def test_dot_perpendicular(self):
        assert Vec3.RIGHT.dot(Vec3.UP) == 0.0

    def test_dot_parallel(self):
        assert Vec3.RIGHT.dot(Vec3.RIGHT) == 1.0

    def test_cross(self):
        assert Vec3.RIGHT.cross(Vec3.UP) == Vec3(0, 0, 1)
        assert Vec3.UP.cross(Vec3.RIGHT) == Vec3(0, 0, -1)

    def test_length(self):
        assert Vec3(3, 4, 0).length() == 5.0

    def test_length_squared(self):
        assert Vec3(3, 4, 0).length_squared() == 25.0

    def test_distance_to(self):
        assert Vec3(1, 2, 3).distance_to(Vec3(4, 6, 3)) == 5.0

    def test_distance_squared_to(self):
        assert Vec3(1, 2, 3).distance_squared_to(Vec3(4, 6, 3)) == 25.0

    def test_angle_to_perpendicular(self):
        assert isclose(Vec3.RIGHT.angle_to(Vec3.UP), 90.0, abs_tol=1e-3)

    def test_angle_to_zero(self):
        assert Vec3.ZERO.angle_to(Vec3.UP) == 0.0


class TestTransform:
    def test_normalize_unit(self):
        n = Vec3(3, 4, 0).normalize()
        assert isclose(n.length(), 1.0, abs_tol=1e-6)
        assert approx(n, Vec3(0.6, 0.8, 0))

    def test_normalize_zero(self):
        assert Vec3.ZERO.normalize() == Vec3.ZERO

    def test_clamp_length_truncates(self):
        v = Vec3(3, 4, 0).clamp_length(2.5)
        assert isclose(v.length(), 2.5, abs_tol=1e-6)

    def test_clamp_length_keeps_short(self):
        v = Vec3(1, 0, 0).clamp_length(10.0)
        assert v == Vec3(1, 0, 0)

    def test_min(self):
        assert Vec3(1, 5, 3).min(Vec3(4, 2, 6)) == Vec3(1, 2, 3)

    def test_max(self):
        assert Vec3(1, 5, 3).max(Vec3(4, 2, 6)) == Vec3(4, 5, 6)

    def test_lerp_endpoints(self):
        a = Vec3(0, 0, 0)
        b = Vec3(10, 20, 30)
        assert a.lerp(b, 0.0) == a
        assert a.lerp(b, 1.0) == b

    def test_lerp_midpoint(self):
        assert Vec3(0, 0, 0).lerp(Vec3(10, 20, 30), 0.5) == Vec3(5, 10, 15)

    def test_slerp_unit(self):
        v = Vec3.RIGHT.slerp(Vec3.UP, 0.5)
        s = sqrt(0.5)
        assert approx(v, Vec3(s, s, 0), tol=1e-3)

    def test_reflect(self):
        assert Vec3(1, -1, 0).reflect(Vec3.UP) == Vec3(1, 1, 0)

    def test_project(self):
        assert Vec3(3, 4, 0).project(Vec3.RIGHT) == Vec3(3, 0, 0)

    def test_project_onto_zero(self):
        assert Vec3(3, 4, 0).project(Vec3.ZERO) == Vec3.ZERO


class TestCoordinateConversions:
    def test_to_world_translation(self):
        local_origin = Mat4.from_translation(Vec3(10, 0, 0))
        assert Vec3(1, 0, 0).to_world(local_origin) == Vec3(11, 0, 0)

    def test_to_local_translation(self):
        local_origin = Mat4.from_translation(Vec3(10, 0, 0))
        assert Vec3(11, 0, 0).to_local(local_origin) == Vec3(1, 0, 0)

    def test_to_world_dir_ignores_translation(self):
        local_origin = Mat4.from_translation(Vec3(10, 0, 0))
        assert Vec3(1, 0, 0).to_world_dir(local_origin) == Vec3(1, 0, 0)

    def test_to_local_dir_ignores_translation(self):
        local_origin = Mat4.from_translation(Vec3(10, 0, 0))
        assert Vec3(1, 0, 0).to_local_dir(local_origin) == Vec3(1, 0, 0)
