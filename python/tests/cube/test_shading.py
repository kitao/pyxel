import pyxel
import pytest

from pyxel.cube import Shading, Vec3


def palette() -> list[int]:
    return [pyxel.colors[i] for i in range(16)]


class TestDefault:
    def test_construction(self):
        s = Shading(palette())
        assert repr(s).startswith("Shading(")

    def test_direction_default(self):
        # Default direction points downward: Vec3(0, -1, 0).
        s = Shading(palette())
        assert s.direction.y == pytest.approx(-1.0)


class TestIndexing:
    def test_get_set(self):
        s = Shading(palette())
        s[0, 0] = (5, 7)
        assert s[0, 0] == (5, 7)

    def test_out_of_range_col(self):
        s = Shading(palette())
        with pytest.raises(IndexError):
            _ = s[100, 0]

    def test_out_of_range_level(self):
        s = Shading(palette())
        with pytest.raises(IndexError):
            _ = s[0, 4]

    def test_negative_col_raises(self):
        s = Shading(palette())
        with pytest.raises((IndexError, OverflowError)):
            _ = s[-1, 0]

    def test_negative_level_raises(self):
        s = Shading(palette())
        with pytest.raises((IndexError, OverflowError)):
            _ = s[0, -1]


class TestDirectionMutate:
    def test_set_direction(self):
        s = Shading(palette())
        s.direction = Vec3(0.5, -0.5, 0.0)
        assert s.direction.x == pytest.approx(0.5)
        assert s.direction.y == pytest.approx(-0.5)


class TestBuild:
    def test_build_resets_modifications(self):
        s = Shading(palette())
        s[0, 0] = (99, 99)
        s.build(palette())
        primary, secondary = s[0, 0]
        assert (primary, secondary) != (99, 99)

    def test_build_callable(self):
        s = Shading(palette())
        s.build(palette())
        _ = s[0, 0]
