import pytest
import pyxel


class TestTypeErrors:
    def test_sin_with_string(self):
        with pytest.raises(TypeError):
            pyxel.sin("abc")

    def test_pset_with_string_x(self):
        with pytest.raises(TypeError):
            pyxel.pset("a", 0, 0)

    def test_clamp_with_string(self):
        with pytest.raises(TypeError):
            pyxel.clamp("a", 0, 10)

    def test_rect_wrong_types(self):
        with pytest.raises(TypeError):
            pyxel.rect("a", "b", "c", "d", "e")


class TestIndexErrors:
    def test_images_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.images[999]

    def test_sounds_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.sounds[999]

    def test_tilemaps_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.tilemaps[999]

    def test_colors_negative_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.colors[-9999]
