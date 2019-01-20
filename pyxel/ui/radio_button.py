import pyxel

from .constants import BUTTON_ENABLED_COLOR, BUTTON_PRESSED_COLOR
from .widget import Widget


class RadioButton(Widget):
    """
    Events:
        __on_change(value)
    """

    def __init__(self, parent, x, y, img, sx, sy, btn_count, value, **kwargs):
        width = btn_count * 9 - 2
        height = 7
        super().__init__(parent, x, y, width, height, **kwargs)

        self._img = img
        self._sx = sx
        self._sy = sy
        self._btn_count = btn_count
        self._value = None

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("draw", self.__on_draw)

        self.value = value

    @property
    def value(self):
        return self._value

    @value.setter
    def value(self, value):
        if self._value != value:
            self._value = value
            self.call_event_handler("change", value)

    def check_value(self, x, y):
        x -= self.x
        y -= self.y

        index = min(max(x // 9, 0), self._btn_count - 1)

        x1 = index * 9
        y1 = 0
        x2 = x1 + 6
        y2 = y1 + 6

        if x1 <= x <= x2 and y1 <= y <= y2:
            return index

        return None

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_LEFT_BUTTON:
            return

        value = self.check_value(x, y)

        if value is not None:
            self.value = value

    def __on_mouse_drag(self, key, x, y, dx, dy):
        self.__on_mouse_down(key, x, y)

    def __on_draw(self):
        pyxel.blt(
            self.x, self.y, self._img, self._sx, self._sy, self.width, self.height
        )

        pyxel.pal(BUTTON_ENABLED_COLOR, BUTTON_PRESSED_COLOR)
        pyxel.blt(
            self.x + self._value * 9,
            self.y,
            self._img,
            self._sx + self._value * 9,
            self._sy,
            7,
            7,
        )
        pyxel.pal()
