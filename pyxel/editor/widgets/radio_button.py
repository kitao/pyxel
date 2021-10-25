import pyxel

from .settings import BUTTON_ENABLED_COLOR, BUTTON_PRESSED_COLOR
from .widget import Widget


class RadioButton(Widget):
    """
    Variables:
        value_var

    Events:
        change (value)
    """

    def __init__(self, parent, x, y, *, img, u, v, btn_count, value, **kwargs):
        super().__init__(parent, x, y, btn_count * 9 - 2, 7, **kwargs)
        self._img = img
        self._u = u
        self._v = v
        self._btn_count = btn_count

        # value_var
        self.new_var("value_var", value)
        self.add_var_event_listener("value_var", "change", self.__on_value_change)

        # event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("draw", self.__on_draw)

    def check_value(self, x, y):
        x -= self.x
        y -= self.y
        index = min(max(x // 9, 0), self._btn_count - 1)
        x1 = index * 9
        y1 = 0
        x2 = x1 + 6
        y2 = y1 + 6
        return index if x1 <= x <= x2 and y1 <= y <= y2 else None

    def __on_value_change(self, value):
        self.trigger_event("change", value)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        value = self.check_value(x, y)
        if value is not None:
            self.value_var = value

    def __on_mouse_drag(self, key, x, y, dx, dy):
        self.__on_mouse_down(key, x, y)

    def __on_draw(self):
        pyxel.blt(
            self.x,
            self.y,
            self._img,
            self._u,
            self._v,
            self.width,
            self.height,
        )
        pyxel.pal(BUTTON_ENABLED_COLOR, BUTTON_PRESSED_COLOR)
        pyxel.blt(
            self.x + self.value_var * 9,
            self.y,
            self._img,
            self._u + self.value_var * 9,
            self._v,
            7,
            7,
        )
        pyxel.pal()
