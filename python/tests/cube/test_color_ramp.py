import pytest

from pyxel.cube import ColorRamp


class TestDefault:
    def test_construction(self):
        r = ColorRamp()
        assert repr(r).startswith("ColorRamp(")

    def test_brightest_level_matches_self(self):
        # Level 15 (factor 1.0) → target == col self → primary == col,
        # ratio == 0 (flat fill; secondary is irrelevant in that case).
        r = ColorRamp()
        for col in range(16):
            primary, _, ratio = r[col, 15]
            assert primary == col
            assert ratio == 0


class TestIndexing:
    def test_get_set(self):
        r = ColorRamp()
        r[0, 0] = (5, 7, 8)
        assert r[0, 0] == (5, 7, 8)

    def test_out_of_range_col(self):
        r = ColorRamp()
        with pytest.raises(IndexError):
            _ = r[100, 0]

    def test_out_of_range_level(self):
        r = ColorRamp()
        with pytest.raises(IndexError):
            _ = r[0, 16]

    def test_negative_col_raises(self):
        r = ColorRamp()
        with pytest.raises((IndexError, OverflowError)):
            _ = r[-1, 0]

    def test_negative_level_raises(self):
        r = ColorRamp()
        with pytest.raises((IndexError, OverflowError)):
            _ = r[0, -1]

    def test_ratio_within_bounds(self):
        # Default-built ramp must stay within the 4x4 Bayer range [0, 16).
        r = ColorRamp()
        for col in range(16):
            for level in range(16):
                _, _, ratio = r[col, level]
                assert 0 <= ratio < 16


class TestBuild:
    def test_build_resets_modifications(self):
        r = ColorRamp()
        r[0, 0] = (99, 99, 0)
        r.build()
        primary, secondary, _ = r[0, 0]
        # Default value will not be (99, 99, 0) — at minimum one of
        # primary/secondary/ratio should change.
        assert (primary, secondary) != (99, 99)
