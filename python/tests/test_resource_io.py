import os

import pytest

import pyxel


# Resource I/O
class TestResourceIO:
    def test_load_pyxres(self, assets_dir):
        pyxel.load(os.path.join(assets_dir, "sample.pyxres"))

    def test_save_load_roundtrip(self, tmp_path):
        # Set up known data
        img = pyxel.images[0]
        img.cls(0)
        img.pset(0, 0, 7)
        snd = pyxel.sounds[0]
        snd.set("c2e2g2", "sss", "777", "nnn", 10)

        # Save
        path = str(tmp_path / "test.pyxres")
        pyxel.save(path)

        # Modify data
        img.cls(0)
        snd.set("a2", "s", "7", "n", 5)

        # Load and verify restored
        pyxel.load(path)
        assert pyxel.images[0].pget(0, 0) == 7
        assert len(pyxel.sounds[0].notes) == 3

    def test_save_exclude_images(self, tmp_path):
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 9)
        path = str(tmp_path / "test_excl.pyxres")
        pyxel.save(path, exclude_images=True)

        pyxel.images[0].cls(0)
        pyxel.load(path)
        # Image data was excluded, so pixel should still be 0
        assert pyxel.images[0].pget(0, 0) == 0

    def test_load_pal(self, assets_dir):
        pyxel.load_pal(os.path.join(assets_dir, "audio_bgm.pyxpal"))

    def test_save_load_pal_roundtrip(self, tmp_path):
        original_color = pyxel.colors[0]
        path = str(tmp_path / "test.pyxpal")
        pyxel.save_pal(path)

        pyxel.colors[0] = 0xFFFFFF
        pyxel.load_pal(path)
        assert pyxel.colors[0] == original_color

    def test_load_nonexistent_file_raises(self):
        with pytest.raises(Exception):
            pyxel.load("/nonexistent/path/file.pyxres")

    def test_screenshot(self, tmp_path):
        pyxel.cls(7)
        pyxel.flip()
        path = str(tmp_path / "test_screenshot.png")
        pyxel.screenshot(path)
        assert os.path.exists(path)

    def test_screencast(self, tmp_path):
        # In headless mode, flip() doesn't capture frames,
        # so screencast produces no GIF. Verify it doesn't raise.
        pyxel.cls(5)
        pyxel.flip()
        path = str(tmp_path / "test_screencast.gif")
        pyxel.screencast(path)

    def test_reset_screencast(self):
        pyxel.reset_screencast()
        # Should not raise
