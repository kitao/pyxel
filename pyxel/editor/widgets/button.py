import pyxel

from .settings import BUTTON_FLASHING_TIME
from .widget import Widget


class Button(Widget):
    """
    Events:
        press
        repeat
        release
    """

    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self._is_pressed = False
        self._flashing_time = 0

        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_repeat", self.__on_mouse_repeat)
        self.add_event_listener("mouse_up", self.__on_mouse_up)
        self.add_event_listener("update", self.__on_update)

    @property
    def is_pressed(self):
        return self._is_pressed

    def press(self):
        self._is_pressed = True
        self._flashing_time = BUTTON_FLASHING_TIME
        self.trigger_event("press")

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._is_pressed = True
        self._flashing_time = 0
        self.trigger_event("press")

    def __on_mouse_repeat(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._is_pressed = True
        self._flashing_time = 0
        self.trigger_event("repeat")

    def __on_mouse_up(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._is_pressed = False
        self._flashing_time = 0
        self.trigger_event("release")

    def __on_update(self):
        if self._flashing_time > 0:
            self._flashing_time -= 1
        else:
            self._is_pressed = False
