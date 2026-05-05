import pytest

from pyxel.cube import Ramp


class TestDefault:
    def test_construction(self):
        # Pyxel default palette has 16 colors
        r = Ramp()
        assert repr(r).startswith("Ramp(")

    def test_brightest_level_matches_self(self):
        # Level 15 (brightness factor = 1.0) should map to col itself
        r = Ramp()
        for col in range(16):
            assert r[col, 15] == col


class TestIndexing:
    def test_get_set(self):
        r = Ramp()
        r[0, 0] = 42
        assert r[0, 0] == 42

    def test_out_of_range_col(self):
        r = Ramp()
        with pytest.raises(IndexError):
            _ = r[100, 0]

    def test_out_of_range_level(self):
        r = Ramp()
        with pytest.raises(IndexError):
            _ = r[0, 16]

    def test_negative_col_raises(self):
        r = Ramp()
        with pytest.raises((IndexError, OverflowError)):
            _ = r[-1, 0]

    def test_negative_level_raises(self):
        r = Ramp()
        with pytest.raises((IndexError, OverflowError)):
            _ = r[0, -1]


class TestBuild:
    def test_build_resets_modifications(self):
        r = Ramp()
        r[0, 0] = 99
        r.build()
        # After rebuild, the modified cell should be back to default
        assert r[0, 0] != 99
