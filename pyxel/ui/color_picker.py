import pyxel

from .widget import Widget


class ColorPicker(Widget):
    """
    Events:
        __on_change(value)
    """

    def __init__(self, parent, x, y, value, *, with_shadow=False, **kwargs):
        super().__init__(parent, x, y, 65, 17, **kwargs)

        self._with_shadow = with_shadow
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
        x -= self.x + 1
        y -= self.y + 1

        index_x = min(max(x // 8, 0), 7)
        index_y = min(max(y // 8, 0), 1)

        x1 = index_x * 8
        y1 = index_y * 8
        x2 = x1 + 6
        y2 = x2 + 6

        if x1 <= x <= x2 and y1 <= y <= y2:
            return index_y * 8 + index_x

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
        self.draw_panel(
            self.x, self.y, self.width, self.height, with_shadow=self._with_shadow
        )

        for i in range(2):
            for j in range(8):
                x = self.x + j * 8 + 1
                y = self.y + i * 8 + 1
                col = i * 8 + j
                pyxel.rect(x, y, 7, 7, col)

        x = self.x + (self._value % 8) * 8 + 3
        y = self.y + (self._value // 8) * 8 + 2
        col = 7 if self._value < 6 else 0
        pyxel.text(x, y, "+", col)
