from math import isclose, isfinite, isinf, sqrt

import pytest
from _assertions import raises_exact  # type: ignore[reportMissingImports]
from pyxel.cube import Mat4, Vec3


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
        assert all(isinstance(component, float) for component in (v.x, v.y, v.z))


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

    def test_singleton_identity(self):
        assert Vec3.ZERO is Vec3.ZERO
        assert Vec3.UP is Vec3.UP
        assert Vec3.FORWARD is Vec3.FORWARD


class TestHash:
    def test_hash_equal_for_equal_values(self):
        assert hash(Vec3(1, 2, 3)) == hash(Vec3(1, 2, 3))

    def test_usable_as_dict_key(self):
        d = {Vec3(1, 2, 3): "a", Vec3(4, 5, 6): "b"}
        assert d[Vec3(1, 2, 3)] == "a"
        assert d[Vec3(4, 5, 6)] == "b"

    def test_usable_in_set(self):
        s = {Vec3(0, 0, 0), Vec3(1, 0, 0), Vec3(0, 0, 0)}
        assert len(s) == 2

    def test_hash_equal_for_signed_zero(self):
        assert Vec3(0.0, 0, 0) == Vec3(-0.0, 0, 0)
        assert hash(Vec3(0.0, 0, 0)) == hash(Vec3(-0.0, 0, 0))

    def test_signed_zero_interchangeable_as_dict_key(self):
        d = {Vec3(0.0, 0, 0): "origin"}
        assert d[Vec3(-0.0, 0, 0)] == "origin"


class TestSequence:
    def test_getitem(self):
        v = Vec3(1, 2, 3)
        assert v[0] == 1.0
        assert v[1] == 2.0
        assert v[2] == 3.0

    def test_getitem_out_of_range(self):
        v = Vec3(1, 2, 3)
        with raises_exact(IndexError, "Vec3 index out of range"):
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
        assert n.length() == 1.0
        assert n == Vec3(0.6, 0.8, 0)

    def test_normalize_zero(self):
        assert Vec3.ZERO.normalize() == Vec3.ZERO

    def test_clamp_length_truncates(self):
        v = Vec3(3, 4, 0).clamp_length(2.5)
        assert v == Vec3(1.5, 2, 0)

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
        assert approx_v(v, Vec3(s, s, 0), tol=1e-3)

    def test_slerp_antiparallel_stays_unit_length(self):
        # Antiparallel interpolation uses a deterministic perpendicular axis.
        v = Vec3.RIGHT.slerp(Vec3.LEFT, 0.5)
        assert isclose(v.length(), 1.0, abs_tol=1e-5)
        assert approx_v(v, Vec3(0, 0, 1))

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


@pytest.mark.parametrize("scale", [1e-30, 1e-20, 1, 1e10, 1e20, 1e30])
class TestFiniteInputs:
    def test_length_and_distance_retain_representable_magnitude(self, scale):
        vector = Vec3(3 * scale, 4 * scale, 0)
        assert all(isfinite(value) for value in vector)
        # The 3-4-5 triangle supplies the expected magnitude independently.
        assert vector.length() == pytest.approx(5 * scale, rel=2e-6, abs=0)
        assert vector.distance_to(Vec3.ZERO) == pytest.approx(
            5 * scale, rel=2e-6, abs=0
        )

    def test_normalize_retains_direction(self, scale):
        vector = Vec3(3 * scale, 4 * scale, 0)
        assert tuple(vector.normalize()) == pytest.approx((0.6, 0.8, 0), abs=1e-6)

    def test_clamp_retains_direction_and_requested_magnitude(self, scale):
        vector = Vec3(3 * scale, 4 * scale, 0)
        assert tuple(vector.clamp_length(scale)) == pytest.approx(
            (0.6 * scale, 0.8 * scale, 0), rel=2e-6, abs=0
        )

    def test_perpendicular_angle_is_independent_of_magnitude(self, scale):
        vector = Vec3(3 * scale, 4 * scale, 0)
        perpendicular = Vec3(-4 * scale, 3 * scale, 0)
        assert vector.angle_to(perpendicular) == pytest.approx(90, abs=1e-5)

    def test_projection_is_independent_of_target_magnitude(self, scale):
        vector = Vec3(3 * scale, 4 * scale, 0)
        assert tuple(vector.project(Vec3(scale, 0, 0))) == pytest.approx(
            (3 * scale, 0, 0), rel=2e-6, abs=0
        )


class TestNormRangeDependencies:
    def test_unrepresentable_length_still_has_representable_unit_direction(self):
        vector = Vec3(2e38, 2e38, 2e38)
        assert all(isfinite(value) for value in vector)
        assert isinf(vector.length())
        expected = (1 / sqrt(3),) * 3
        assert tuple(vector.normalize()) == pytest.approx(expected, abs=1e-6)
        assert tuple(vector.clamp_length(1)) == pytest.approx(expected, abs=1e-6)

    @pytest.mark.parametrize("epsilon", [1e-30, 1e-20])
    def test_unit_slerp_retains_near_opposite_interpolation_plane(self, epsilon):
        target = Vec3(-1, epsilon, 0)
        assert target.length() == 1
        midpoint = Vec3.RIGHT.slerp(target, 0.5)
        assert tuple(midpoint) == pytest.approx((0, 1, 0), abs=1e-6)
        assert midpoint.z == 0


class TestFiniteIntermediates:
    @pytest.mark.parametrize("scale", [1, 1e20, 3e38])
    def test_dot_cancellation_is_zero(self, scale):
        assert Vec3(scale, scale, 0).dot(Vec3(2, -2, 0)) == 0

    @pytest.mark.parametrize("scale", [1, 1e20, 3e38])
    def test_parallel_cross_product_is_zero(self, scale):
        assert Vec3(scale, scale, 0).cross(Vec3(2, 2, 0)) == Vec3.ZERO

    @pytest.mark.parametrize("scale", [1, 1e20, 3e38])
    @pytest.mark.parametrize("t", [0, 0.25, 0.5, 1])
    def test_lerp_retains_endpoints_and_interior(self, scale, t):
        start = Vec3(scale, 0, 0)
        expected = (start.x * (1 - 2 * t), 0, 0)
        assert tuple(start.lerp(-start, t)) == pytest.approx(expected, rel=2e-6, abs=0)

    @pytest.mark.parametrize("scale", [1, 1e20, 3e38])
    def test_reflect_in_unit_axis_retains_magnitude(self, scale):
        vector = Vec3(scale, 0, 0)
        assert vector.reflect(Vec3.RIGHT) == -vector

    @pytest.mark.parametrize("small", [1, 1e-20, 1e-23, 1e-30])
    def test_angle_with_mixed_norms_is_45_degrees(self, small):
        assert Vec3(1e10, 1e10, 0).angle_to(Vec3(small, 0, 0)) == pytest.approx(
            45, abs=1e-5
        )

    @pytest.mark.parametrize("small", [1, 1e-30, 1e-35, 1e-38])
    @pytest.mark.parametrize("target", [1, -1e-10, 1e-10])
    def test_x_projection_with_mixed_norms_retains_stored_value(self, small, target):
        vector = Vec3(small, 0, 0)
        assert tuple(vector.project(Vec3(target, 0, 0))) == pytest.approx(
            tuple(vector), rel=2e-6, abs=0
        )


# 1e-5 exceeds the f32 rounding of unit-scale values and stays below any expected difference
def approx_v(a, b, tol=1e-5):
    return (
        isclose(a.x, b.x, abs_tol=tol)
        and isclose(a.y, b.y, abs_tol=tol)
        and isclose(a.z, b.z, abs_tol=tol)
    )
