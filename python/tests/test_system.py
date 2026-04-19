import pyxel

_COLOR_NAMES = [
    "BLACK",
    "NAVY",
    "PURPLE",
    "GREEN",
    "BROWN",
    "DARK_BLUE",
    "LIGHT_BLUE",
    "WHITE",
    "RED",
    "ORANGE",
    "YELLOW",
    "LIME",
    "CYAN",
    "GRAY",
    "PINK",
    "PEACH",
]


class TestSystemAttributes:
    def test_width(self):
        assert pyxel.width == 160

    def test_height(self):
        assert pyxel.height == 120

    def test_frame_count_is_int(self):
        assert isinstance(pyxel.frame_count, int)

    def test_frame_count_non_negative(self):
        assert pyxel.frame_count >= 0

    def test_mouse_wheel_is_int(self):
        assert isinstance(pyxel.mouse_wheel, int)

    def test_screen_is_image(self):
        assert isinstance(pyxel.screen, pyxel.Image)
        assert pyxel.screen.width == 160
        assert pyxel.screen.height == 120

    def test_cursor_is_image(self):
        assert isinstance(pyxel.cursor, pyxel.Image)
        assert pyxel.cursor.width > 0
        assert pyxel.cursor.height > 0

    def test_font_image_is_image(self):
        assert isinstance(pyxel.font, pyxel.Image)
        assert pyxel.font.width > 0
        assert pyxel.font.height > 0


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

    def test_num_channels(self):
        assert pyxel.NUM_CHANNELS == 4

    def test_num_tones(self):
        assert pyxel.NUM_TONES == 4

    def test_image_size(self):
        assert pyxel.IMAGE_SIZE == 256

    def test_tilemap_size(self):
        assert pyxel.TILEMAP_SIZE == 256

    def test_tile_size(self):
        assert pyxel.TILE_SIZE == 8

    def test_version_is_string(self):
        assert isinstance(pyxel.VERSION, str)
        assert len(pyxel.VERSION) > 0

    def test_color_constants(self):
        assert pyxel.COLOR_BLACK == 0
        assert pyxel.COLOR_NAVY == 1
        assert pyxel.COLOR_PURPLE == 2
        assert pyxel.COLOR_GREEN == 3
        assert pyxel.COLOR_BROWN == 4
        assert pyxel.COLOR_DARK_BLUE == 5
        assert pyxel.COLOR_LIGHT_BLUE == 6
        assert pyxel.COLOR_WHITE == 7
        assert pyxel.COLOR_RED == 8
        assert pyxel.COLOR_ORANGE == 9
        assert pyxel.COLOR_YELLOW == 10
        assert pyxel.COLOR_LIME == 11
        assert pyxel.COLOR_CYAN == 12
        assert pyxel.COLOR_GRAY == 13
        assert pyxel.COLOR_PINK == 14
        assert pyxel.COLOR_PEACH == 15

    def test_color_constants_contiguous(self):
        for i, name in enumerate(_COLOR_NAMES):
            assert getattr(pyxel, f"COLOR_{name}") == i

    def test_tone_constants(self):
        assert pyxel.TONE_TRIANGLE == 0
        assert pyxel.TONE_SQUARE == 1
        assert pyxel.TONE_PULSE == 2
        assert pyxel.TONE_NOISE == 3

    def test_effect_constants(self):
        assert pyxel.EFFECT_NONE == 0
        assert pyxel.EFFECT_SLIDE == 1
        assert pyxel.EFFECT_VIBRATO == 2
        assert pyxel.EFFECT_FADEOUT == 3
        assert pyxel.EFFECT_HALF_FADEOUT == 4
        assert pyxel.EFFECT_QUARTER_FADEOUT == 5

    def test_default_colors_is_list(self):
        assert isinstance(pyxel.DEFAULT_COLORS, list)
        assert len(pyxel.DEFAULT_COLORS) == 16
        assert all(isinstance(c, int) for c in pyxel.DEFAULT_COLORS)

    def test_key_constants_are_int(self):
        assert isinstance(pyxel.KEY_SPACE, int)
        assert isinstance(pyxel.KEY_RETURN, int)
        assert isinstance(pyxel.KEY_A, int)

    def test_mouse_constants_are_int(self):
        assert isinstance(pyxel.MOUSE_BUTTON_LEFT, int)
        assert isinstance(pyxel.MOUSE_BUTTON_RIGHT, int)
        assert isinstance(pyxel.MOUSE_POS_X, int)

    def test_gamepad_constants_are_int(self):
        assert isinstance(pyxel.GAMEPAD1_BUTTON_A, int)
        assert isinstance(pyxel.GAMEPAD1_AXIS_LEFTX, int)

    def test_file_extension_constants(self):
        assert isinstance(pyxel.APP_FILE_EXTENSION, str)
        assert isinstance(pyxel.RESOURCE_FILE_EXTENSION, str)
        assert isinstance(pyxel.PALETTE_FILE_EXTENSION, str)
        # Extensions should start with a dot
        assert pyxel.APP_FILE_EXTENSION.startswith(".")
        assert pyxel.RESOURCE_FILE_EXTENSION.startswith(".")
        assert pyxel.PALETTE_FILE_EXTENSION.startswith(".")


class TestSystemFunctions:
    def test_title(self):
        pyxel.title("test_title")
        # Should not raise

    def test_icon(self):
        pyxel.icon(["0000", "0770", "0770", "0000"], 1)

    def test_icon_with_colkey(self):
        pyxel.icon(["0000", "0770", "0770", "0000"], 1, colkey=0)

    def test_perf_monitor(self):
        pyxel.perf_monitor(True)
        pyxel.perf_monitor(False)

    def test_fullscreen(self):
        pyxel.fullscreen(False)

    def test_screen_mode(self):
        pyxel.screen_mode(0)

    def test_integer_scale(self):
        pyxel.integer_scale(True)
        pyxel.integer_scale(False)

    def test_flip_advances_frame_count(self):
        before = pyxel.frame_count
        pyxel.flip()
        after = pyxel.frame_count
        assert after == before + 1

    def test_flip_multiple(self):
        before = pyxel.frame_count
        pyxel.flip()
        pyxel.flip()
        pyxel.flip()
        assert pyxel.frame_count == before + 3
