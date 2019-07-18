import pyxel
from pyxel.ui import Widget
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y, MAX_SOUND_LENGTH

tone_key_table = [pyxel.KEY_T, pyxel.KEY_S, pyxel.KEY_P, pyxel.KEY_N]
effect_key_table = [pyxel.KEY_N, pyxel.KEY_S, pyxel.KEY_V, pyxel.KEY_F]


class SoundField(Widget):
    def __init__(self, parent):
        super().__init__(parent, 30, 149, 193, 23)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def _screen_to_view(self, x, y):
        x = min(max((x - self.x - 1) // 4, 0), MAX_SOUND_LENGTH - 1)
        y = min(max((y - self.y) // 8, 0), 2)
        return x, y

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_LEFT_BUTTON or self.parent.is_playing:
            return

        x, y = self._screen_to_view(x, y)
        self.parent.field_cursor.move(x, y + 1)

    def __on_mouse_hover(self, x, y):
        x, y = self._screen_to_view(x, y)
        if y == 0:
            self.parent.help_message = "TONE:T/S/P/N/BS/DEL"
        elif y == 1:
            self.parent.help_message = "VOLUME:0-7/BS/DEL"
        elif y == 2:
            self.parent.help_message = "EFFECT:N/S/V/F/BS/DEL"

    def __on_update(self):
        cursor_y = self.parent.field_cursor.y

        if (
            cursor_y < 1
            or self.parent.is_playing
            or pyxel.btn(pyxel.KEY_SHIFT)
            or pyxel.btn(pyxel.KEY_CONTROL)
            or pyxel.btn(pyxel.KEY_ALT)
        ):
            return

        value = None
        if cursor_y == 1:
            for i in range(4):
                if pyxel.btnp(tone_key_table[i], WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
                    value = i
                    break
        elif cursor_y == 2:
            for i in range(8):
                if pyxel.btnp(
                    pyxel.KEY_0 + i, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
                ) or pyxel.btnp(
                    pyxel.KEY_KP_0 + i, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
                ):
                    value = i
                    break
        elif cursor_y == 3:
            for i in range(4):
                if pyxel.btnp(
                    effect_key_table[i], WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
                ):
                    value = i
                    break

        if value is None:
            return

        self.parent.field_cursor.insert(value)

    def __on_draw(self):
        pyxel.text(self.x - 13, self.y + 1, "TON", 6)
        pyxel.text(self.x - 13, self.y + 9, "VOL", 6)
        pyxel.text(self.x - 13, self.y + 17, "EFX", 6)
        pyxel.blt(
            self.x,
            self.y,
            pyxel.IMAGE_BANK_FOR_SYSTEM,
            EDITOR_IMAGE_X,
            EDITOR_IMAGE_Y + 79,
            193,
            23,
        )

        data_str = []
        data_str.append("".join(["TSPN"[v] for v in self.parent.get_data(1)]))
        data_str.append("".join([str(v) for v in self.parent.get_data(2)]))
        data_str.append("".join(["NSVF"[v] for v in self.parent.get_data(3)]))

        for i in range(3):
            pyxel.text(31, 150 + i * 8, data_str[i], 1)

        cursor_y = self.parent.field_cursor.y
        cursor_x = self.parent.field_cursor.x

        if self.parent.is_playing or cursor_y == 0:
            return

        x = cursor_x * 4 + 31
        y = cursor_y * 8 + 142

        pyxel.rect(x, y - 1, 3, 6, 1)

        if cursor_x < len(data_str[cursor_y - 1]):
            pyxel.text(x, y, data_str[cursor_y - 1][cursor_x], 7)
