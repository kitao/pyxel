import pyxel

from .ui_constants import BUTTON_BLINK_TIME
from .widget import Widget


class Button(Widget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self._blink_time = 0
        self.is_blinking = True

        self.add_event_handler("press", self.__on_press)
        self.add_event_handler("update", self.__on_update)

    def __on_press(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            self._blink_time = BUTTON_BLINK_TIME

    def __on_update(self):
        if self._blink_time > 0:
            self._blink_time -= 1
            self.is_blinking = True
        else:
            self.is_blinking = False
