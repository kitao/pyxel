import pytest

import pyxel


@pytest.fixture(autouse=True)
def restore_dimensions():
    yield
    pyxel.resize(160, 120)


class TestResize:
    def test_updates_width_height(self):
        pyxel.resize(320, 240)
        assert pyxel.width == 320
        assert pyxel.height == 240

    def test_updates_screen_canvas(self):
        pyxel.resize(320, 240)
        assert pyxel.screen.width == 320
        assert pyxel.screen.height == 240

    def test_multiple_resizes(self):
        pyxel.resize(320, 240)
        assert (pyxel.width, pyxel.height) == (320, 240)
        pyxel.resize(64, 64)
        assert (pyxel.width, pyxel.height) == (64, 64)
        pyxel.resize(128, 96)
        assert (pyxel.width, pyxel.height) == (128, 96)

    def test_draw_at_new_bounds(self):
        pyxel.resize(320, 240)
        pyxel.pset(319, 239, 7)
        pyxel.resize(64, 48)
        pyxel.pset(63, 47, 7)

    def test_clears_screen_contents(self):
        pyxel.cls(7)
        assert pyxel.pget(0, 0) == 7
        pyxel.resize(80, 60)
        assert pyxel.pget(0, 0) == 0

    def test_resets_clip_rect(self):
        pyxel.clip(10, 10, 20, 20)
        pyxel.resize(80, 60)
        # After resize, clip is reset to full screen, so pset at (0,0) takes effect
        pyxel.pset(0, 0, 7)
        assert pyxel.pget(0, 0) == 7

    def test_resets_camera(self):
        pyxel.camera(50, 50)
        pyxel.resize(80, 60)
        # After resize, camera offset is reset, so pset(0,0) draws at (0,0)
        pyxel.pset(0, 0, 7)
        assert pyxel.pget(0, 0) == 7

    def test_zero_width_raises(self):
        with pytest.raises(ValueError):
            pyxel.resize(0, 120)

    def test_zero_height_raises(self):
        with pytest.raises(ValueError):
            pyxel.resize(160, 0)

    def test_negative_raises_overflow(self):
        with pytest.raises(OverflowError):
            pyxel.resize(-1, 120)
        with pytest.raises(OverflowError):
            pyxel.resize(160, -1)
