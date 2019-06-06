import pyxel
from pyxel.ui import Widget
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y, MAX_SOUND_LENGTH


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
        x = min(max((x - self.x - 1) // 4, 0), MAX_SOUND_LENGTH - 1)
        y = min(max(59 - (y - self.y - 1) // 2, -1), 59)
        return x, y

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_LEFT_BUTTON or self.parent.is_playing:
            return

        x, y = self._screen_to_view(x, y)

        self._press_x = x
        self._press_y = y

        self.parent.field_cursor.move(x, 0)

    def __on_mouse_up(self, key, x, y):
        pass

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key != pyxel.MOUSE_LEFT_BUTTON or self.parent.is_playing:
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

        self.parent.add_pre_history(x, 0)

        data = self.parent.field_cursor.data
        padding_length = self._press_x + 1 - len(data)
        if padding_length > 0:
            data.extend([-1] * padding_length)

        self._press_x = x
        self._press_y = y

        self.parent.field_cursor.move(x, 0)

        dx = x2 - x1
        dy = y2 - y1

        for i in range(dx + 1):
            value = round(y1 + (dy / dx) * i)

            if x1 + i >= len(data):
                data.append(value)
            else:
                data[x1 + i] = value

        self.parent.add_post_history(x, 0)

    def __on_mouse_click(self, key, x, y):
        if key != pyxel.MOUSE_LEFT_BUTTON or self.parent.is_playing:
            return

        x, y = self._screen_to_view(x, y)

        self.parent.field_cursor.move(x, 0)

        data = self.parent.field_cursor.data

        self.parent.add_pre_history(x, 0)

        padding_length = x + 1 - len(data)
        if padding_length > 0:
            data.extend([-1] * padding_length)

        data[x] = y

        self.parent.add_post_history(x, 0)

    def __on_mouse_hover(self, x, y):
        self.parent.help_message = "NOTE:CLICK/PIANO_KEY+ENTER/BS/DEL"

    def __on_update(self):
        cursor_y = self.parent.field_cursor.y

        if cursor_y > 0 or self.parent.is_playing:
            return

        if (
            pyxel.btnp(pyxel.KEY_ENTER, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME)
            or pyxel.btnp(pyxel.KEY_KP_ENTER, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME)
        ) and self.parent.keyboard_note is not None:
            self.parent.field_cursor.insert(self.parent.keyboard_note)

    def __on_draw(self):
        pyxel.rect(self.x, self.y, self.width, self.height, 6)

        if self.parent.is_playing:
            x = (self.parent.play_pos % 100) * 4 + 31
            pyxel.rect(x, 25, 3, 123, 2)
        else:
            if self.parent.field_cursor.y == 0:
                x = self.parent.field_cursor.x * 4 + 31
                pyxel.rect(x, 25, 3, 123, 1)

        pyxel.blt(self.x, self.y, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 7, 193, 72, 6)
        pyxel.blt(
            self.x, self.y + 72, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 7, 193, 51, 6
        )

        for i, note in enumerate(self.parent.get_data(0)):
            x = i * 4 + 31
            y = 143 - note * 2
            pyxel.rect(x, y, 3, 3, 8 if note >= 0 else 12)
