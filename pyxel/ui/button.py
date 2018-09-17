import pyxel

from .ui_constants import BUTTON_BLINK_TIME
from .widget import Widget


class Button(Widget):
    """
    Events:
        __on_press(key, x, y)
    """

    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self._is_down = False
        self._is_blinking = False
        self._blink_time = 0

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("mouse_up", self.__on_mouse_up)
        self.add_event_handler("update", self.__on_update)

    @property
    def is_down(self):
        return self._is_down or self._is_blinking

    def press(self, key, x, y):
        self._blink_time = BUTTON_BLINK_TIME
        self._is_blinking = True

        self.call_event_handler("Press", key, x, y)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        self._is_down = True

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        self._is_down = self.is_hit(x, y)

    def __on_mouse_up(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        self._is_down = False

        if self.is_hit(x, y):
            self.call_event_handler("Press", key, x, y)

    def __on_update(self):
        if self._blink_time > 0:
            self._blink_time -= 1
        else:
            self._is_blinking = False
