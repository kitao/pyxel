import pyxel

from .settings import (
    BUTTON_DISABLED_COLOR,
    BUTTON_ENABLED_COLOR,
    BUTTON_PRESSED_COLOR,
    BUTTON_PRESSING_TIME,
)
from .widget import Widget


class Button(Widget):
    """
    Variables:
        is_visible_var
        is_enabled_var

    Events:
        press
        repeat
        release
        show
        hide
        enabled
        disabled
        mouse_down (key, x, y)
        mouse_up (key, x, y)
        mouse_drag (key, x, y, dx, dy)
        mouse_repeat (key, x, y)
        mouse_click (key, x, y)
        mouse_hover (x, y)
        update
        draw
    """

    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self._pressing_time = 0

        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_repeat", self.__on_mouse_repeat)
        self.add_event_listener("mouse_up", self.__on_mouse_up)
        self.add_event_listener("update", self.__on_update)

    @property
    def button_color(self):
        return (
            (BUTTON_PRESSED_COLOR if self._pressing_time > 0 else BUTTON_ENABLED_COLOR)
            if self.is_enabled_var.v
            else BUTTON_DISABLED_COLOR
        )

    def press(self):
        self._pressing_time = BUTTON_PRESSING_TIME + 1
        self.trigger_event("press")

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._pressing_time = 2
        self.trigger_event("press")

    def __on_mouse_repeat(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._pressing_time = 2
        self.trigger_event("repeat")

    def __on_mouse_up(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._pressing_time = 0
        self.trigger_event("release")

    def __on_update(self):
        if self._pressing_time > 0:
            self._pressing_time -= 1
