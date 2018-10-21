import pyxel
from pyxel.ui import Widget
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y, SOUND_MAX_LENGTH


class PianoRoll(Widget):
    def __init__(self, parent):
        super().__init__(parent, 30, 25, 193, 123)

        self._press_x = 0
        self._press_y = 0

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_up", self.__on_mouse_up)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("mouse_click", self.__on_mouse_click)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def _screen_to_view(self, x, y):
        x = min(max((x - self.x - 1) // 4, 0), SOUND_MAX_LENGTH - 1)
        y = min(max(59 - (y - self.y - 1) // 2, -1), 59)
        return x, y

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON or self.parent.play_pos > -1:
            return

        x, y = self._screen_to_view(x, y)

        self._press_x = x
        self._press_y = y

        self.parent.cursor_x = x
        self.parent.cursor_y = 0

    def __on_mouse_up(self, key, x, y):
        pass

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key != pyxel.KEY_LEFT_BUTTON or self.parent.play_pos > -1:
            return

        x, y = self._screen_to_view(x, y)

        if x > self._press_x:
            x1 = self._press_x
            y1 = self._press_y
            x2 = x
            y2 = y
        elif x < self._press_x:
            x1 = x
            y1 = y
            x2 = self._press_x
            y2 = self._press_y
        else:
            return

        self.parent.add_edit_history_before()

        data = self.parent.sound_data
        padding_length = self._press_x + 1 - len(data)
        if padding_length > 0:
            data.extend([-1] * padding_length)

        self._press_x = x
        self._press_y = y

        self.parent.cursor_x = x
        self.parent.cursor_y = 0

        dx = x2 - x1
        dy = y2 - y1

        for i in range(dx + 1):
            value = round(y1 + (dy / dx) * i)

            if x1 + i >= len(data):
                data.append(value)
            else:
                data[x1 + i] = value

        self.parent.add_edit_history_after()

    def __on_mouse_click(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON or self.parent.play_pos > -1:
            return

        x, y = self._screen_to_view(x, y)

        data = self.parent.sound_data

        self.parent.add_edit_history_before()

        padding_length = x + 1 - len(data)
        if padding_length > 0:
            data.extend([-1] * padding_length)

        data[x] = y

        self.parent.add_edit_history_after()

    def __on_mouse_hover(self, x, y):
        self.parent.help_message = "NOTE:PIANO+ENTER EDIT:ENTER/BS/DEL"

    def __on_update(self):
        cursor_y = self.parent.cursor_y

        if cursor_y > 0 or self.parent.play_pos > -1:
            return

        edit_x = self.parent.edit_x
        data = self.parent.sound_data

        if pyxel.btnp(pyxel.KEY_BACKSPACE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            if edit_x > 0:
                self.parent.add_edit_history_before()
                del data[edit_x - 1]
                self.parent.cursor_x = edit_x - 1
                self.parent.add_edit_history_after()
            return

        if pyxel.btnp(pyxel.KEY_DELETE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            if edit_x < len(data):
                self.parent.add_edit_history_before()
                del data[edit_x]
                self.parent.add_edit_history_after()
            return

        if pyxel.btnp(
            pyxel.KEY_ENTER, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ) or pyxel.btnp(pyxel.KEY_KP_ENTER, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.parent.add_edit_history_before()

            note = self.parent.keyboard_note
            if note is None:
                data.insert(edit_x, -1)
                data[:] = data[:SOUND_MAX_LENGTH]
            elif edit_x >= len(data):
                data.append(note)
            else:
                data[edit_x] = note

            self.parent.cursor_x = edit_x
            if edit_x < SOUND_MAX_LENGTH - 1:
                self.parent.cursor_x += 1

            self.parent.add_edit_history_after()

    def __on_draw(self):
        pyxel.rect(self.x, self.y, self.x + self.width - 1, self.y + self.height - 1, 6)

        play_pos = self.parent.play_pos

        if play_pos > -1:
            x = play_pos * 4 + 31
            pyxel.rect(x, 25, x + 2, 147, 2)
        else:
            if self.parent.cursor_y == 0:
                x = self.parent.edit_x * 4 + 31
                pyxel.rect(x, 25, x + 2, 147, 1)

        pyxel.blt(
            self.x, self.y, 3, EDITOR_IMAGE_X + 16, EDITOR_IMAGE_Y + 8, 97, 123, 6
        )
        pyxel.blt(
            self.x + 97, self.y, 3, EDITOR_IMAGE_X + 16, EDITOR_IMAGE_Y + 8, -96, 123, 6
        )

        sound = pyxel.sound(self.parent.sound)
        for i, note in enumerate(sound.note):
            x = i * 4 + 31
            y = 143 - note * 2
            pyxel.rect(x, y, x + 2, y + 2, 8 if note >= 0 else 12)
