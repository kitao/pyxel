import pytest
import pyxel


# clamp -- Union[int, float] type preservation
class TestClamp:
    def test_int_returns_int(self):
        result = pyxel.clamp(5, 0, 10)
        assert result == 5
        assert isinstance(result, int)

    def test_float_returns_float(self):
        result = pyxel.clamp(5.0, 0.0, 10.0)
        assert result == 5.0
        assert isinstance(result, float)

    def test_clamps_below(self):
        assert pyxel.clamp(-5, 0, 10) == 0

    def test_clamps_above(self):
        assert pyxel.clamp(15, 0, 10) == 10

    def test_at_lower_boundary(self):
        assert pyxel.clamp(0, 0, 10) == 0

    def test_at_upper_boundary(self):
        assert pyxel.clamp(10, 0, 10) == 10

    def test_mixed_int_float(self):
        result = pyxel.clamp(5, 0.0, 10.0)
        assert result == 5

    def test_negative_range(self):
        assert pyxel.clamp(-5, -10, -1) == -5
        assert pyxel.clamp(0, -10, -1) == -1
        assert pyxel.clamp(-20, -10, -1) == -10


# sgn -- Union[int, float] type preservation
class TestSgn:
    def test_positive_int(self):
        result = pyxel.sgn(3)
        assert result == 1
        assert isinstance(result, int)

    def test_negative_float(self):
        result = pyxel.sgn(-3.0)
        assert result == -1.0
        assert isinstance(result, float)

    def test_zero_int(self):
        result = pyxel.sgn(0)
        assert result == 0
        assert isinstance(result, int)

    def test_zero_float(self):
        result = pyxel.sgn(0.0)
        assert result == 0.0
        assert isinstance(result, float)

    def test_large_positive(self):
        assert pyxel.sgn(999999) == 1

    def test_large_negative(self):
        assert pyxel.sgn(-999999) == -1


# Trigonometric functions -- degrees, not radians
class TestTrig:
    def test_sin_0(self):
        assert pyxel.sin(0) == pytest.approx(0.0, abs=1e-6)

    def test_sin_90(self):
        assert pyxel.sin(90) == pytest.approx(1.0, abs=1e-6)

    def test_sin_180(self):
        assert pyxel.sin(180) == pytest.approx(0.0, abs=1e-6)

    def test_sin_270(self):
        assert pyxel.sin(270) == pytest.approx(-1.0, abs=1e-6)

    def test_sin_360(self):
        assert pyxel.sin(360) == pytest.approx(0.0, abs=1e-6)

    def test_cos_0(self):
        assert pyxel.cos(0) == pytest.approx(1.0, abs=1e-6)

    def test_cos_90(self):
        assert pyxel.cos(90) == pytest.approx(0.0, abs=1e-6)

    def test_cos_180(self):
        assert pyxel.cos(180) == pytest.approx(-1.0, abs=1e-6)

    def test_cos_360(self):
        assert pyxel.cos(360) == pytest.approx(1.0, abs=1e-6)

    def test_sin_negative(self):
        assert pyxel.sin(-90) == pytest.approx(-1.0, abs=1e-6)

    def test_atan2_quadrants(self):
        assert pyxel.atan2(1, 0) == pytest.approx(90.0, abs=1e-3)
        assert pyxel.atan2(0, 1) == pytest.approx(0.0, abs=1e-3)
        assert pyxel.atan2(-1, 0) == pytest.approx(-90.0, abs=1e-3)

    def test_sin_cos_identity(self):
        # sin^2 + cos^2 = 1 for any angle
        for deg in [0, 30, 45, 60, 90, 120, 180, 270]:
            s = pyxel.sin(deg)
            c = pyxel.cos(deg)
            assert s * s + c * c == pytest.approx(1.0, abs=1e-5)


# ceil / floor / sqrt
class TestBasicMath:
    def test_ceil(self):
        assert pyxel.ceil(1.2) == 2

    def test_ceil_negative(self):
        assert pyxel.ceil(-1.2) == -1

    def test_ceil_integer(self):
        assert pyxel.ceil(3.0) == 3

    def test_floor(self):
        assert pyxel.floor(1.8) == 1

    def test_floor_negative(self):
        assert pyxel.floor(-1.8) == -2

    def test_floor_integer(self):
        assert pyxel.floor(3.0) == 3

    def test_sqrt(self):
        assert pyxel.sqrt(4.0) == pytest.approx(2.0)

    def test_sqrt_zero(self):
        assert pyxel.sqrt(0.0) == pytest.approx(0.0)

    def test_sqrt_one(self):
        assert pyxel.sqrt(1.0) == pytest.approx(1.0)


# Random number generation
class TestRandom:
    def test_rndi_in_range(self):
        for _ in range(100):
            val = pyxel.rndi(0, 10)
            assert 0 <= val <= 10
            assert isinstance(val, int)

    def test_rndi_single_value(self):
        # When min == max, always returns that value
        for _ in range(10):
            assert pyxel.rndi(5, 5) == 5

    def test_rndi_includes_boundaries(self):
        pyxel.rseed(0)
        values = {pyxel.rndi(0, 1) for _ in range(100)}
        assert 0 in values
        assert 1 in values

    def test_rndf_in_range(self):
        for _ in range(100):
            val = pyxel.rndf(0.0, 1.0)
            assert 0.0 <= val <= 1.0
            assert isinstance(val, float)

    def test_rseed_reproducible(self):
        pyxel.rseed(99)
        seq1 = [pyxel.rndi(0, 1000) for _ in range(10)]
        pyxel.rseed(99)
        seq2 = [pyxel.rndi(0, 1000) for _ in range(10)]
        assert seq1 == seq2

    def test_rseed_different_seeds_differ(self):
        pyxel.rseed(1)
        seq1 = [pyxel.rndi(0, 1000) for _ in range(10)]
        pyxel.rseed(2)
        seq2 = [pyxel.rndi(0, 1000) for _ in range(10)]
        assert seq1 != seq2

    def test_rndf_reproducible(self):
        pyxel.rseed(42)
        seq1 = [pyxel.rndf(0.0, 1.0) for _ in range(10)]
        pyxel.rseed(42)
        seq2 = [pyxel.rndf(0.0, 1.0) for _ in range(10)]
        assert seq1 == seq2


# Noise
class TestNoise:
    def test_noise_range(self):
        for x in [0.0, 0.5, 1.0, 10.0]:
            val = pyxel.noise(x, 0.0, 0.0)
            assert -1.0 <= val <= 1.0

    def test_noise_seed_reproducible(self):
        pyxel.nseed(42)
        val1 = pyxel.noise(1.0, 2.0, 3.0)
        pyxel.nseed(42)
        val2 = pyxel.noise(1.0, 2.0, 3.0)
        assert val1 == val2

    def test_noise_different_seeds_differ(self):
        pyxel.nseed(100)
        val1 = pyxel.noise(1.5, 2.5, 3.5)
        pyxel.nseed(999)
        val2 = pyxel.noise(1.5, 2.5, 3.5)
        assert val1 != val2

    def test_noise_1d(self):
        val = pyxel.noise(0.5)
        assert -1.0 <= val <= 1.0

    def test_noise_2d(self):
        val = pyxel.noise(0.5, 0.3)
        assert -1.0 <= val <= 1.0

    def test_noise_continuity(self):
        # Nearby inputs should produce nearby outputs (Perlin noise is smooth)
        pyxel.nseed(0)
        v1 = pyxel.noise(1.0, 0.0, 0.0)
        v2 = pyxel.noise(1.001, 0.0, 0.0)
        assert abs(v1 - v2) < 0.1
