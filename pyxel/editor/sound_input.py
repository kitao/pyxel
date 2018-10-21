import pyxel
from pyxel.constants import SOUND_MAX_LENGTH
from pyxel.ui import Widget
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y


class SoundInput(Widget):
    def __init__(self, parent):
        super().__init__(parent, 30, 149, 193, 23)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def _screen_to_view(self, x, y):
        x = min(max((x - self.x - 1) // 4, 0), SOUND_MAX_LENGTH - 1)
        y = min(max((y - self.y) // 8, 0), 2)
        return x, y

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON or self.parent.play_pos > -1:
            return

        x, y = self._screen_to_view(x, y)
        self.parent.cursor_x = x
        self.parent.cursor_y = y + 1

    def __on_mouse_hover(self, x, y):
        x, y = self._screen_to_view(x, y)
        if y == 0:
            self.parent.help_message = "TONE:1-4 EDIT:ENTER/BS/DEL"
        elif y == 1:
            self.parent.help_message = "VOLUME:0-7 EDIT:ENTER/BS/DEL"
        elif y == 2:
            self.parent.help_message = "EFFECT:1-4 EDIT:ENTER/BS/DEL"

    def __on_update(self):
        cursor_y = self.parent.cursor_y

        if cursor_y < 1 or self.parent.play_pos > -1:
            return

        edit_x = self.parent.edit_x
        data = self.parent.sound_data

        if pyxel.btnp(pyxel.KEY_BACKSPACE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            if edit_x > 0:
                del data[edit_x - 1]
                self.parent.cursor_x = edit_x - 1
            return

        if pyxel.btnp(pyxel.KEY_DELETE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            if edit_x < len(data):
                del data[edit_x]
            return

        if pyxel.btnp(
            pyxel.KEY_ENTER, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ) or pyxel.btnp(pyxel.KEY_KP_ENTER, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            data.insert(edit_x, 0)
            data[:] = data[:SOUND_MAX_LENGTH]

            self.parent.cursor_x = edit_x
            if edit_x < SOUND_MAX_LENGTH - 1:
                self.parent.cursor_x += 1
            return

        value = None
        if cursor_y == 1:
            for i in range(4):
                if pyxel.btnp(
                    pyxel.KEY_1 + i, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
                ) or pyxel.btnp(
                    pyxel.KEY_KP_1 + i, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
                ):
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
                    pyxel.KEY_1 + i, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
                ) or pyxel.btnp(
                    pyxel.KEY_KP_1 + i, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
                ):
                    value = i
                    break

        if value is None:
            return

        edit_x = self.parent.edit_x
        data = self.parent.sound_data

        if edit_x >= len(data):
            data.append(value)
        else:
            data[edit_x] = value

        self.parent.cursor_x = edit_x
        if edit_x < SOUND_MAX_LENGTH - 1:
            self.parent.cursor_x += 1

    def __on_draw(self):
        pyxel.blt(self.x, self.y, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 132, 97, 23)
        pyxel.blt(self.x + 97, self.y, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 132, -96, 23)

        sound = pyxel.sound(self.parent.sound)
        cursor_x = self.parent.cursor_x
        cursor_y = self.parent.cursor_y if self.parent.play_pos < 0 else 0

        if cursor_y > 0:
            x = self.parent.edit_x * 4 + 31
            y = cursor_y * 8 + 142
            pyxel.rect(x, y - 1, x + 2, y + 5, 1)

        for i, tone in enumerate(sound.tone):
            col = 7 if cursor_y == 1 and cursor_x == i else 1
            pyxel.text(31 + i * 4, 150, "TSPN"[tone], col)

        for i, volume in enumerate(sound.volume):
            col = 7 if cursor_y == 2 and cursor_x == i else 1
            pyxel.text(31 + i * 4, 158, str(volume), col)

        for i, effect in enumerate(sound.effect):
            col = 7 if cursor_y == 3 and cursor_x == i else 1
            pyxel.text(31 + i * 4, 166, "NSVF"[effect], col)
