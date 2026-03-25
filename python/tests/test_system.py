import pyxel


class TestSystemAttributes:
    def test_width(self):
        assert pyxel.width == 160

    def test_height(self):
        assert pyxel.height == 120

    def test_frame_count_is_int(self):
        assert isinstance(pyxel.frame_count, int)


class TestConstants:
    def test_font_width(self):
        assert pyxel.FONT_WIDTH == 4

    def test_font_height(self):
        assert pyxel.FONT_HEIGHT == 6

    def test_num_colors(self):
        assert pyxel.NUM_COLORS == 16

    def test_num_images(self):
        assert pyxel.NUM_IMAGES == 3

    def test_num_tilemaps(self):
        assert pyxel.NUM_TILEMAPS == 8

    def test_num_sounds(self):
        assert pyxel.NUM_SOUNDS == 64

    def test_num_musics(self):
        assert pyxel.NUM_MUSICS == 8


class TestSystemFunctions:
    def test_title_no_error(self):
        pyxel.title("test")
