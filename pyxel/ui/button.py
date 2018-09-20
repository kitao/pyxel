import pyxel

from .ui_constants import BUTTON_LIGHTING_TIME
from .widget import Widget


class Button(Widget):
    """
    Events:
        __on_press()
    """

    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self._lighting_time = 0
        self._is_lighting = False
        self._is_pressed = False

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_up", self.__on_mouse_up)
        self.add_event_handler("update", self.__on_update)

    @property
    def is_lighting(self):
        return self._is_pressed or self._is_lighting

    def press(self):
        self._is_lighting = True
        self._lighting_time = BUTTON_LIGHTING_TIME
        self.call_event_handler("press")

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        self._is_pressed = True
        self.call_event_handler("press")

    def __on_mouse_up(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        self._is_pressed = False

    def __on_update(self):
        if self._lighting_time > 0:
            self._lighting_time -= 1
        else:
            self._is_lighting = False
