import pytest

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

_KEY_NAMES = [
    "KEY_UNKNOWN",
    "KEY_BACKSPACE",
    "KEY_TAB",
    "KEY_RETURN",
    "KEY_ESCAPE",
    "KEY_SPACE",
    "KEY_EXCLAIM",
    "KEY_QUOTEDBL",
    "KEY_HASH",
    "KEY_DOLLAR",
    "KEY_PERCENT",
    "KEY_AMPERSAND",
    "KEY_QUOTE",
    "KEY_LEFTPAREN",
    "KEY_RIGHTPAREN",
    "KEY_ASTERISK",
    "KEY_PLUS",
    "KEY_COMMA",
    "KEY_MINUS",
    "KEY_PERIOD",
    "KEY_SLASH",
    *[f"KEY_{n}" for n in range(10)],
    "KEY_COLON",
    "KEY_SEMICOLON",
    "KEY_LESS",
    "KEY_EQUALS",
    "KEY_GREATER",
    "KEY_QUESTION",
    "KEY_AT",
    "KEY_LEFTBRACKET",
    "KEY_BACKSLASH",
    "KEY_RIGHTBRACKET",
    "KEY_CARET",
    "KEY_UNDERSCORE",
    "KEY_BACKQUOTE",
    *[f"KEY_{c}" for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ"],
    "KEY_DELETE",
    "KEY_CAPSLOCK",
    *[f"KEY_F{i}" for i in range(1, 13)],
    "KEY_PRINTSCREEN",
    "KEY_SCROLLLOCK",
    "KEY_PAUSE",
    "KEY_INSERT",
    "KEY_HOME",
    "KEY_PAGEUP",
    "KEY_END",
    "KEY_PAGEDOWN",
    "KEY_RIGHT",
    "KEY_LEFT",
    "KEY_DOWN",
    "KEY_UP",
    "KEY_NUMLOCKCLEAR",
    "KEY_KP_DIVIDE",
    "KEY_KP_MULTIPLY",
    "KEY_KP_MINUS",
    "KEY_KP_PLUS",
    "KEY_KP_ENTER",
    *[f"KEY_KP_{n}" for n in range(10)],
    "KEY_KP_PERIOD",
    "KEY_APPLICATION",
    "KEY_POWER",
    "KEY_KP_EQUALS",
    *[f"KEY_F{i}" for i in range(13, 25)],
    "KEY_EXECUTE",
    "KEY_HELP",
    "KEY_MENU",
    "KEY_SELECT",
    "KEY_STOP",
    "KEY_AGAIN",
    "KEY_UNDO",
    "KEY_CUT",
    "KEY_COPY",
    "KEY_PASTE",
    "KEY_FIND",
    "KEY_MUTE",
    "KEY_VOLUMEUP",
    "KEY_VOLUMEDOWN",
    "KEY_KP_COMMA",
    "KEY_KP_EQUALSAS400",
    "KEY_ALTERASE",
    "KEY_SYSREQ",
    "KEY_CANCEL",
    "KEY_CLEAR",
    "KEY_PRIOR",
    "KEY_RETURN2",
    "KEY_SEPARATOR",
    "KEY_OUT",
    "KEY_OPER",
    "KEY_CLEARAGAIN",
    "KEY_CRSEL",
    "KEY_EXSEL",
    "KEY_KP_00",
    "KEY_KP_000",
    "KEY_THOUSANDSSEPARATOR",
    "KEY_DECIMALSEPARATOR",
    "KEY_CURRENCYUNIT",
    "KEY_CURRENCYSUBUNIT",
    "KEY_KP_LEFTPAREN",
    "KEY_KP_RIGHTPAREN",
    "KEY_KP_LEFTBRACE",
    "KEY_KP_RIGHTBRACE",
    "KEY_KP_TAB",
    "KEY_KP_BACKSPACE",
    *[f"KEY_KP_{c}" for c in "ABCDEF"],
    "KEY_KP_XOR",
    "KEY_KP_POWER",
    "KEY_KP_PERCENT",
    "KEY_KP_LESS",
    "KEY_KP_GREATER",
    "KEY_KP_AMPERSAND",
    "KEY_KP_DBLAMPERSAND",
    "KEY_KP_VERTICALBAR",
    "KEY_KP_DBLVERTICALBAR",
    "KEY_KP_COLON",
    "KEY_KP_HASH",
    "KEY_KP_SPACE",
    "KEY_KP_AT",
    "KEY_KP_EXCLAM",
    "KEY_KP_MEMSTORE",
    "KEY_KP_MEMRECALL",
    "KEY_KP_MEMCLEAR",
    "KEY_KP_MEMADD",
    "KEY_KP_MEMSUBTRACT",
    "KEY_KP_MEMMULTIPLY",
    "KEY_KP_MEMDIVIDE",
    "KEY_KP_PLUSMINUS",
    "KEY_KP_CLEAR",
    "KEY_KP_CLEARENTRY",
    "KEY_KP_BINARY",
    "KEY_KP_OCTAL",
    "KEY_KP_DECIMAL",
    "KEY_KP_HEXADECIMAL",
    "KEY_LCTRL",
    "KEY_LSHIFT",
    "KEY_LALT",
    "KEY_LGUI",
    "KEY_RCTRL",
    "KEY_RSHIFT",
    "KEY_RALT",
    "KEY_RGUI",
    "KEY_NONE",
    "KEY_SHIFT",
    "KEY_CTRL",
    "KEY_ALT",
    "KEY_GUI",
]

_MOUSE_NAMES = [
    "MOUSE_POS_X",
    "MOUSE_POS_Y",
    "MOUSE_WHEEL_X",
    "MOUSE_WHEEL_Y",
    "MOUSE_BUTTON_LEFT",
    "MOUSE_BUTTON_MIDDLE",
    "MOUSE_BUTTON_RIGHT",
    "MOUSE_BUTTON_X1",
    "MOUSE_BUTTON_X2",
]

_GAMEPAD_NAMES = [
    f"GAMEPAD{n}_{suffix}"
    for n in (1, 2, 3, 4)
    for suffix in (
        "AXIS_LEFTX",
        "AXIS_LEFTY",
        "AXIS_RIGHTX",
        "AXIS_RIGHTY",
        "AXIS_TRIGGERLEFT",
        "AXIS_TRIGGERRIGHT",
        "BUTTON_A",
        "BUTTON_B",
        "BUTTON_X",
        "BUTTON_Y",
        "BUTTON_BACK",
        "BUTTON_GUIDE",
        "BUTTON_START",
        "BUTTON_LEFTSTICK",
        "BUTTON_RIGHTSTICK",
        "BUTTON_LEFTSHOULDER",
        "BUTTON_RIGHTSHOULDER",
        "BUTTON_DPAD_UP",
        "BUTTON_DPAD_DOWN",
        "BUTTON_DPAD_LEFT",
        "BUTTON_DPAD_RIGHT",
    )
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

    def test_cursor_writable(self):
        original = pyxel.cursor.pget(0, 0)
        pyxel.cursor.pset(0, 0, 7)
        assert pyxel.cursor.pget(0, 0) == 7
        pyxel.cursor.pset(0, 0, original)

    def test_font_writable(self):
        original = pyxel.font.pget(0, 0)
        pyxel.font.pset(0, 0, 7)
        assert pyxel.font.pget(0, 0) == 7
        pyxel.font.pset(0, 0, original)


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

    def test_base_dir(self):
        assert isinstance(pyxel.BASE_DIR, str)
        assert len(pyxel.BASE_DIR) > 0

    def test_window_state_env(self):
        assert isinstance(pyxel.WINDOW_STATE_ENV, str)
        assert len(pyxel.WINDOW_STATE_ENV) > 0

    def test_watch_state_file_env(self):
        assert isinstance(pyxel.WATCH_STATE_FILE_ENV, str)
        assert len(pyxel.WATCH_STATE_FILE_ENV) > 0

    def test_watch_reset_exit_code(self):
        assert isinstance(pyxel.WATCH_RESET_EXIT_CODE, int)

    def test_app_startup_script_file(self):
        assert isinstance(pyxel.APP_STARTUP_SCRIPT_FILE, str)
        assert len(pyxel.APP_STARTUP_SCRIPT_FILE) > 0

    def test_color_constants(self):
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

    def test_all_key_constants_are_int(self):
        for name in _KEY_NAMES:
            assert isinstance(getattr(pyxel, name), int), name

    def test_all_mouse_constants_are_int(self):
        for name in _MOUSE_NAMES:
            assert isinstance(getattr(pyxel, name), int), name

    def test_all_gamepad_constants_are_int(self):
        for name in _GAMEPAD_NAMES:
            assert isinstance(getattr(pyxel, name), int), name

    def test_file_extension_constants(self):
        assert isinstance(pyxel.APP_FILE_EXTENSION, str)
        assert isinstance(pyxel.RESOURCE_FILE_EXTENSION, str)
        assert isinstance(pyxel.PALETTE_FILE_EXTENSION, str)
        assert pyxel.APP_FILE_EXTENSION.startswith(".")
        assert pyxel.RESOURCE_FILE_EXTENSION.startswith(".")
        assert pyxel.PALETTE_FILE_EXTENSION.startswith(".")


class TestSystemSetters:
    def _capture_state(self):
        return (pyxel.width, pyxel.height, list(pyxel.colors), pyxel.frame_count)

    def _assert_state_unchanged(self, before):
        assert (
            pyxel.width,
            pyxel.height,
            list(pyxel.colors),
            pyxel.frame_count,
        ) == before

    def test_title_preserves_state(self):
        before = self._capture_state()
        pyxel.title("test_title")
        self._assert_state_unchanged(before)

    def test_icon_preserves_state(self):
        before = self._capture_state()
        pyxel.icon(["0000", "0770", "0770", "0000"], 1)
        self._assert_state_unchanged(before)

    def test_icon_with_colkey_preserves_state(self):
        before = self._capture_state()
        pyxel.icon(["0000", "0770", "0770", "0000"], 1, colkey=0)
        self._assert_state_unchanged(before)

    def test_perf_monitor_preserves_state(self):
        before = self._capture_state()
        pyxel.perf_monitor(True)
        pyxel.perf_monitor(False)
        self._assert_state_unchanged(before)

    def test_fullscreen_preserves_state(self):
        before = self._capture_state()
        pyxel.fullscreen(True)
        pyxel.fullscreen(False)
        self._assert_state_unchanged(before)

    def test_screen_mode_preserves_state(self):
        before = self._capture_state()
        for mode in (0, 1, 2):
            pyxel.screen_mode(mode)
        self._assert_state_unchanged(before)

    def test_integer_scale_preserves_state(self):
        before = self._capture_state()
        pyxel.integer_scale(True)
        pyxel.integer_scale(False)
        self._assert_state_unchanged(before)


class TestSystemFlow:
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

    @pytest.mark.skip(
        reason="quit() runs atexit handlers and exits the process; not in-process testable."
    )
    def test_quit(self):
        pyxel.quit()

    @pytest.mark.skip(
        reason="reset() spawns a subprocess and calls sys.exit(); not in-process testable."
    )
    def test_reset(self):
        pyxel.reset()
