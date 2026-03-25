import pyxel


class TestInputFunctions:
    def test_btn_returns_bool(self):
        result = pyxel.btn(pyxel.KEY_SPACE)
        assert isinstance(result, bool)

    def test_btnp_returns_bool(self):
        result = pyxel.btnp(pyxel.KEY_SPACE)
        assert isinstance(result, bool)

    def test_btnp_with_hold_repeat(self):
        result = pyxel.btnp(pyxel.KEY_SPACE, hold=10, repeat=5)
        assert isinstance(result, bool)

    def test_btnr_returns_bool(self):
        result = pyxel.btnr(pyxel.KEY_SPACE)
        assert isinstance(result, bool)

    def test_btnv_returns_int(self):
        # btnv requires an analog key (mouse position/wheel or gamepad axis)
        result = pyxel.btnv(pyxel.MOUSE_WHEEL_X)
        assert isinstance(result, int)


class TestInputAttributes:
    def test_input_text_accessible(self):
        _ = pyxel.input_text

    def test_input_keys_accessible(self):
        _ = pyxel.input_keys

    def test_dropped_files_accessible(self):
        _ = pyxel.dropped_files

    def test_mouse_x_accessible(self):
        assert isinstance(pyxel.mouse_x, int)

    def test_mouse_y_accessible(self):
        assert isinstance(pyxel.mouse_y, int)
