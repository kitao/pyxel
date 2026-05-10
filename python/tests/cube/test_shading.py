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


# Algorithm invariants. The pickers in compute() select per-cell
# (primary, secondary) entries under three rules every palette must
# satisfy: ramp brightness is monotone, the dither pair (when not flat)
# reads as one color, and lv 0 / lv 1 don't share luma with the source.

GB_PALETTE = [0x000000, 0x555555, 0xAAAAAA, 0xFFFFFF]


def _srgb_to_linear(c: float) -> float:
    if c <= 0.04045:
        return c / 12.92
    return ((c + 0.055) / 1.055) ** 2.4


def _linear_luma(rgb24: int) -> float:
    r = _srgb_to_linear(((rgb24 >> 16) & 0xFF) / 255)
    g = _srgb_to_linear(((rgb24 >> 8) & 0xFF) / 255)
    b = _srgb_to_linear((rgb24 & 0xFF) / 255)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def _entry_luma(pal: list[int], primary: int, secondary: int) -> float:
    if primary == secondary:
        return _linear_luma(pal[primary])
    return (_linear_luma(pal[primary]) + _linear_luma(pal[secondary])) / 2


def _pal(name: str) -> list[int]:
    if name == "pyxel_default":
        return palette()
    if name == "gb_monochrome":
        return GB_PALETTE
    raise KeyError(name)


@pytest.mark.parametrize("pal_name", ["pyxel_default", "gb_monochrome"])
class TestRampInvariants:
    def test_ramp_is_monotone(self, pal_name):
        pal = _pal(pal_name)
        s = Shading(pal)
        for col in range(len(pal)):
            cells = [s[col, lv] for lv in range(4)]
            ls = [_entry_luma(pal, p, q) for p, q in cells]
            assert ls[0] <= ls[1] + 1e-6, f"{pal_name} col {col} lv0>lv1"
            assert ls[1] <= ls[2] + 1e-6, f"{pal_name} col {col} lv1>lv2"
            assert ls[2] <= ls[3] + 1e-6, f"{pal_name} col {col} lv2>lv3"

    def test_shade_levels_below_base(self, pal_name):
        # lv 0 / lv 1 may collapse onto the base flat as a fallback,
        # but they must never overshoot it (= read as brighter).
        pal = _pal(pal_name)
        s = Shading(pal)
        for col in range(len(pal)):
            base = _linear_luma(pal[col])
            for lv in (0, 1):
                p, q = s[col, lv]
                if (p, q) == (col, col):
                    continue  # base-flat fallback is allowed
                assert _entry_luma(pal, p, q) < base + 1e-6, (
                    f"{pal_name} col {col} lv{lv} brighter than base"
                )

    def test_highlight_above_or_equal_base(self, pal_name):
        # lv 3 may collapse onto the base flat (palette has no usable
        # highlight) but must never read as darker than the base.
        pal = _pal(pal_name)
        s = Shading(pal)
        for col in range(len(pal)):
            base = _linear_luma(pal[col])
            p, q = s[col, 3]
            if (p, q) == (col, col):
                continue
            assert _entry_luma(pal, p, q) >= base - 1e-6, (
                f"{pal_name} col {col} lv3 darker than base"
            )
