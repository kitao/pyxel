import pyxel

from .settings import (
    EDITOR_IMAGE,
    MAX_SOUND_LENGTH,
    SOUND_FIELD_CURSOR_EDIT_COLOR,
    SOUND_FIELD_CURSOR_SELECT_COLOR,
    SOUND_FIELD_DATA_NORMAL_COLOR,
    SOUND_FIELD_DATA_SELECT_COLOR,
    TEXT_LABEL_COLOR,
)
from .widgets import Widget
from .widgets.settings import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME

TONE_KEY_TABLE = [pyxel.KEY_T, pyxel.KEY_S, pyxel.KEY_P, pyxel.KEY_N]
EFFECT_KEY_TABLE = [
    pyxel.KEY_N,
    pyxel.KEY_S,
    pyxel.KEY_V,
    pyxel.KEY_F,
    pyxel.KEY_H,
    pyxel.KEY_Q,
]


class SoundField(Widget):
    """
    Variables:
        is_playing_var
        help_message_var
    """

    def __init__(self, parent):
        super().__init__(parent, 30, 149, 193, 23)
        self.field_cursor = parent.field_cursor
        self.get_field = parent.get_field
        self.get_field_help_message = parent.get_field_help_message
        self.copy_var("is_playing_var", parent)
        self.copy_var("help_message_var", parent)

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    def _screen_to_view(self, x, y):
        x = min(max((x - self.x - 1) // 4, 0), MAX_SOUND_LENGTH - 1)
        y = min(max((y - self.y) // 8, 0), 2)
        return x, y

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT or self.is_playing_var:
            return
        x, y = self._screen_to_view(x, y)
        self.field_cursor.move_to(x, y + 1, pyxel.btn(pyxel.KEY_SHIFT))

    def __on_mouse_hover(self, x, y):
        self.help_message_var = self.get_field_help_message()

    def __on_update(self):
        cursor_y = self.field_cursor.y
        if (
            cursor_y < 1
            or self.is_playing_var
            or pyxel.btn(pyxel.KEY_SHIFT)
            or pyxel.btn(pyxel.KEY_CTRL)
            or pyxel.btn(pyxel.KEY_ALT)
            or pyxel.btn(pyxel.KEY_GUI)
        ):
            return
        value = None
        if cursor_y == 1:
            for i in range(4):
                if pyxel.btnp(
                    TONE_KEY_TABLE[i], hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
                ):
                    value = i
                    break
        elif cursor_y == 2:
            for i in range(8):
                key = pyxel.KEY_0 if i == 0 else pyxel.KEY_1 + i - 1
                if pyxel.btnp(
                    key, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
                ) or pyxel.btnp(key, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME):
                    value = i
                    break
        elif cursor_y == 3:
            for i in range(6):
                if pyxel.btnp(
                    EFFECT_KEY_TABLE[i],
                    hold=WIDGET_HOLD_TIME,
                    repeat=WIDGET_REPEAT_TIME,
                ):
                    value = i
                    break
        if value is None:
            return
        self.field_cursor.insert(value)

    def __on_draw(self):
        # Draw field frame
        pyxel.text(self.x - 13, self.y + 1, "TON", TEXT_LABEL_COLOR)
        pyxel.text(self.x - 13, self.y + 9, "VOL", TEXT_LABEL_COLOR)
        pyxel.text(self.x - 13, self.y + 17, "EFX", TEXT_LABEL_COLOR)
        pyxel.blt(
            self.x,
            self.y,
            EDITOR_IMAGE,
            0,
            79,
            193,
            23,
        )

        # Draw field data
        data_str = []
        data_str.append("".join(["TSPN"[v] for v in self.get_field(1)]))
        data_str.append("".join([str(v) for v in self.get_field(2)]))
        data_str.append("".join(["NSVFHQ"[v] for v in self.get_field(3)]))
        for i in range(3):
            pyxel.text(31, 150 + i * 8, data_str[i], SOUND_FIELD_DATA_NORMAL_COLOR)

        # Draw cursor
        cursor_y = self.field_cursor.y
        cursor_x = self.field_cursor.x
        if self.is_playing_var or cursor_y == 0:
            return
        x = cursor_x * 4 + 31
        y = cursor_y * 8 + 142
        w = self.field_cursor.width * 4
        col = (
            SOUND_FIELD_CURSOR_SELECT_COLOR
            if self.field_cursor.is_selecting
            else SOUND_FIELD_CURSOR_EDIT_COLOR
        )
        pyxel.rect(x, y - 1, w, 7, col)
        if cursor_x < len(data_str[cursor_y - 1]):
            pyxel.text(
                x,
                y,
                data_str[cursor_y - 1][cursor_x : cursor_x + self.field_cursor.width],
                SOUND_FIELD_DATA_SELECT_COLOR,
            )
