import pyxel


class TestInputFunctions:
    def test_btn_returns_bool(self):
        result = pyxel.btn(pyxel.KEY_SPACE)
        assert isinstance(result, bool)

    def test_btnp_returns_bool(self):
        result = pyxel.btnp(pyxel.KEY_SPACE)
        assert isinstance(result, bool)

    def test_btnp_with_hold_repeat_returns_bool(self):
        result = pyxel.btnp(pyxel.KEY_SPACE, hold=10, repeat=5)
        assert isinstance(result, bool)

    def test_btnr_returns_bool(self):
        result = pyxel.btnr(pyxel.KEY_SPACE)
        assert isinstance(result, bool)

    def test_btnv_returns_int(self):
        # btnv requires an analog key (mouse position/wheel or gamepad axis)
        result = pyxel.btnv(pyxel.MOUSE_WHEEL_X)
        assert isinstance(result, int)

    def test_mouse_visible(self):
        pyxel.mouse(True)
        pyxel.mouse(False)


class TestInputAttributes:
    def test_input_text_is_string(self):
        assert isinstance(pyxel.input_text, str)

    def test_input_keys_is_list(self):
        keys = pyxel.input_keys
        assert isinstance(keys, list)

    def test_dropped_files_is_list(self):
        files = pyxel.dropped_files
        assert isinstance(files, list)

    def test_mouse_x_is_int(self):
        assert isinstance(pyxel.mouse_x, int)

    def test_mouse_y_is_int(self):
        assert isinstance(pyxel.mouse_y, int)


class TestSetButtonState:
    def test_press_makes_btn_true(self):
        pyxel.set_btn(pyxel.KEY_A, True)
        assert pyxel.btn(pyxel.KEY_A) is True

    def test_press_makes_btnp_true(self):
        pyxel.set_btn(pyxel.KEY_B, True)
        assert pyxel.btnp(pyxel.KEY_B) is True

    def test_release_after_flip_makes_btn_false(self):
        pyxel.set_btn(pyxel.KEY_C, True)
        pyxel.flip()
        pyxel.set_btn(pyxel.KEY_C, False)
        assert pyxel.btn(pyxel.KEY_C) is False

    def test_release_makes_btnr_true(self):
        pyxel.set_btn(pyxel.KEY_D, True)
        pyxel.set_btn(pyxel.KEY_D, False)
        assert pyxel.btnr(pyxel.KEY_D) is True

    def test_btnp_false_after_flip(self):
        pyxel.set_btn(pyxel.KEY_E, True)
        assert pyxel.btnp(pyxel.KEY_E) is True
        pyxel.flip()
        # btnp is false on next frame (no new press)
        assert pyxel.btnp(pyxel.KEY_E) is False
        # But btn is still true (key held)
        assert pyxel.btn(pyxel.KEY_E) is True

    def test_btnr_false_without_release(self):
        pyxel.set_btn(pyxel.KEY_F, True)
        assert pyxel.btnr(pyxel.KEY_F) is False

    def test_multiple_keys_independent(self):
        pyxel.set_btn(pyxel.KEY_G, True)
        pyxel.set_btn(pyxel.KEY_H, False)
        assert pyxel.btn(pyxel.KEY_G) is True
        assert pyxel.btn(pyxel.KEY_H) is False

    def test_input_keys_reflects_pressed(self):
        pyxel.set_btn(pyxel.KEY_I, True)
        assert pyxel.KEY_I in pyxel.input_keys
        pyxel.set_btn(pyxel.KEY_I, False)
        pyxel.flip()


class TestBtnpHoldRepeat:
    # Hold = frames the key must be held before repeats begin firing.
    # Repeat = frame interval between subsequent ticks.

    def test_first_press_returns_true(self):
        pyxel.set_btn(pyxel.KEY_J, True)
        assert pyxel.btnp(pyxel.KEY_J, hold=3, repeat=2) is True
        pyxel.set_btn(pyxel.KEY_J, False)
        pyxel.flip()

    def test_silent_during_hold_window(self):
        pyxel.set_btn(pyxel.KEY_K, True)
        assert pyxel.btnp(pyxel.KEY_K, hold=3, repeat=2) is True
        pyxel.flip()
        assert pyxel.btnp(pyxel.KEY_K, hold=3, repeat=2) is False
        pyxel.flip()
        assert pyxel.btnp(pyxel.KEY_K, hold=3, repeat=2) is False
        pyxel.set_btn(pyxel.KEY_K, False)
        pyxel.flip()

    def test_repeat_ticks_after_hold(self):
        pyxel.set_btn(pyxel.KEY_L, True)
        assert pyxel.btnp(pyxel.KEY_L, hold=3, repeat=2) is True
        for _ in range(3):
            pyxel.flip()
        # 3 frames after press: hold complete, first repeat tick
        assert pyxel.btnp(pyxel.KEY_L, hold=3, repeat=2) is True
        pyxel.flip()
        assert pyxel.btnp(pyxel.KEY_L, hold=3, repeat=2) is False
        pyxel.flip()
        # 5 frames after press: next repeat tick
        assert pyxel.btnp(pyxel.KEY_L, hold=3, repeat=2) is True
        pyxel.set_btn(pyxel.KEY_L, False)
        pyxel.flip()

    def test_repeat_zero_disables_repeat(self):
        pyxel.set_btn(pyxel.KEY_M, True)
        assert pyxel.btnp(pyxel.KEY_M, hold=3, repeat=0) is True
        for _ in range(8):
            pyxel.flip()
            assert pyxel.btnp(pyxel.KEY_M, hold=3, repeat=0) is False
        pyxel.set_btn(pyxel.KEY_M, False)
        pyxel.flip()


class TestSetButtonValue:
    def test_set_analog_value(self):
        pyxel.set_btnv(pyxel.MOUSE_WHEEL_Y, 5)
        assert pyxel.btnv(pyxel.MOUSE_WHEEL_Y) == 5

    def test_set_analog_value_negative(self):
        pyxel.set_btnv(pyxel.MOUSE_WHEEL_Y, -3)
        assert pyxel.btnv(pyxel.MOUSE_WHEEL_Y) == -3

    def test_set_analog_value_zero(self):
        pyxel.set_btnv(pyxel.MOUSE_WHEEL_Y, 0)
        assert pyxel.btnv(pyxel.MOUSE_WHEEL_Y) == 0

    def test_mouse_wheel_reflects_btnv(self):
        pyxel.set_btnv(pyxel.MOUSE_WHEEL_Y, 5)
        assert pyxel.mouse_wheel == 5
        pyxel.set_btnv(pyxel.MOUSE_WHEEL_Y, -3)
        assert pyxel.mouse_wheel == -3
        pyxel.set_btnv(pyxel.MOUSE_WHEEL_Y, 0)


class TestSetMousePos:
    def test_updates_mouse_coordinates(self):
        pyxel.set_mouse_pos(80, 60)
        assert pyxel.mouse_x == 80
        assert pyxel.mouse_y == 60

    def test_updates_btnv_values(self):
        pyxel.set_mouse_pos(42, 17)
        assert pyxel.btnv(pyxel.MOUSE_POS_X) == 42
        assert pyxel.btnv(pyxel.MOUSE_POS_Y) == 17

    def test_origin(self):
        pyxel.set_mouse_pos(0, 0)
        assert pyxel.mouse_x == 0
        assert pyxel.mouse_y == 0


class TestSetInputText:
    def test_sets_text(self):
        pyxel.set_input_text("hello")
        assert pyxel.input_text == "hello"

    def test_replaces_existing_text(self):
        pyxel.set_input_text("first")
        pyxel.set_input_text("second")
        assert pyxel.input_text == "second"

    def test_cleared_after_flip(self):
        pyxel.set_input_text("temp")
        pyxel.flip()
        assert pyxel.input_text == ""

    def test_empty_string(self):
        pyxel.set_input_text("")
        assert pyxel.input_text == ""

    def test_multibyte_text(self):
        pyxel.set_input_text("日本語")
        assert pyxel.input_text == "日本語"


class TestSetDroppedFiles:
    def test_sets_files(self):
        pyxel.set_dropped_files(["a.txt", "b.txt"])
        assert list(pyxel.dropped_files) == ["a.txt", "b.txt"]

    def test_replaces_existing_files(self):
        pyxel.set_dropped_files(["old.txt"])
        pyxel.set_dropped_files(["new.txt"])
        assert list(pyxel.dropped_files) == ["new.txt"]

    def test_cleared_after_flip(self):
        pyxel.set_dropped_files(["temp.txt"])
        pyxel.flip()
        assert list(pyxel.dropped_files) == []

    def test_empty_list(self):
        pyxel.set_dropped_files([])
        assert list(pyxel.dropped_files) == []
