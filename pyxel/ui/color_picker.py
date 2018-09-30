import pyxel

from .ui_constants import WIDGET_FRAME_COLOR
from .widget import Widget


class ColorPicker(Widget):
    """
    Events:
        __on_change(value)
    """

    def __init__(self, parent, x, y, **kwargs):
        super().__init__(parent, x, y, 65, 17, **kwargs)

        self._value = 0

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("draw", self.__on_draw)

    @property
    def value(self):
        return self._value

    @value.setter
    def value(self, value):
        if self._value != value:
            self._value = value
            self.call_event_handler("change", value)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        x -= self.x
        y -= self.y

        index_x = min(max((x + 1) // 8, 0), 7)
        index_y = min(max((y + 1) // 8, 0), 1)

        x1 = index_x * 8 + 1
        y1 = index_y * 8 + 1
        x2 = x1 + 6
        y2 = x2 + 6

        if x >= x1 and x < x2 and y >= y1 and y < y2:
            self.value = index_y * 8 + index_x

    def __on_mouse_drag(self, key, x, y, dx, dy):
        self.__on_mouse_down(key, x, y)

    def __on_draw(self):
        x1 = self.x
        y1 = self.y
        x2 = x1 + self.width - 1
        y2 = y1 + self.height - 1
        pyxel.rect(x1 + 1, y1, x2 - 1, y2, WIDGET_FRAME_COLOR)
        pyxel.rect(x1, y1 + 1, x2, y2 - 1, WIDGET_FRAME_COLOR)

        for i in range(2):
            for j in range(8):
                x1 = self.x + j * 8 + 1
                y1 = self.y + i * 8 + 1
                x2 = x1 + 6
                y2 = y1 + 6
                col = i * 8 + j
                pyxel.rect(x1, y1, x2, y2, col)

        x = self.x + (self.value % 8) * 8 + 3
        y = self.y + (self.value // 8) * 8 + 2
        col = 7 if self.value < 6 else 0
        pyxel.text(x, y, "+", col)
