import pyxel

from .settings import BUTTON_LIGHTING_TIME
from .widget import Widget


class Button(Widget):
    """
    Events:
        __on_press()
        __on_repeat()
        __on_release()
    """

    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self._is_pressed = False
        self._is_lighting = False
        self._lighting_time = 0

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_repeat", self.__on_mouse_repeat)
        self.add_event_handler("mouse_up", self.__on_mouse_up)
        self.add_event_handler("update", self.__on_update)

    @property
    def is_pressed(self):
        return self._is_pressed or self._is_lighting

    def press(self):
        self._is_lighting = True
        self._lighting_time = BUTTON_LIGHTING_TIME
        self.call_event_handler("press")

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._is_pressed = True
        self.call_event_handler("press")

    def __on_mouse_repeat(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._is_pressed = True
        self.call_event_handler("repeat")

    def __on_mouse_up(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._is_pressed = False
        self.call_event_handler("release")

    def __on_update(self):
        if self._lighting_time > 0:
            self._lighting_time -= 1
        else:
            self._is_lighting = False
