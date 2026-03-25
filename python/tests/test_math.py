import pytest
import pyxel


# clamp — Union[int, float] type preservation
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

    def test_mixed_int_float(self):
        result = pyxel.clamp(5, 0.0, 10.0)
        assert result == 5


# sgn — Union[int, float] type preservation
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
        assert pyxel.sgn(0) == 0

    def test_zero_float(self):
        result = pyxel.sgn(0.0)
        assert result == 0.0
        assert isinstance(result, float)


# Trigonometric functions — degrees, not radians
class TestTrig:
    def test_sin_90(self):
        assert pyxel.sin(90) == pytest.approx(1.0)

    def test_sin_0(self):
        assert pyxel.sin(0) == pytest.approx(0.0)

    def test_cos_0(self):
        assert pyxel.cos(0) == pytest.approx(1.0)

    def test_cos_90(self):
        assert pyxel.cos(90) == pytest.approx(0.0, abs=1e-6)

    def test_atan2(self):
        assert pyxel.atan2(1, 0) == pytest.approx(90.0)


# ceil / floor / sqrt
class TestBasicMath:
    def test_ceil(self):
        assert pyxel.ceil(1.2) == 2

    def test_ceil_negative(self):
        assert pyxel.ceil(-1.2) == -1

    def test_floor(self):
        assert pyxel.floor(1.8) == 1

    def test_floor_negative(self):
        assert pyxel.floor(-1.8) == -2

    def test_sqrt(self):
        assert pyxel.sqrt(4.0) == pytest.approx(2.0)


# Random number generation
class TestRandom:
    def test_rndi_in_range(self):
        for _ in range(100):
            val = pyxel.rndi(0, 10)
            assert 0 <= val <= 10
            assert isinstance(val, int)

    def test_rndf_in_range(self):
        for _ in range(100):
            val = pyxel.rndf(0.0, 1.0)
            assert 0.0 <= val <= 1.0
            assert isinstance(val, float)


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
