import pyxel

from .settings import (
    BUTTON_DISABLED_COLOR,
    BUTTON_ENABLED_COLOR,
    BUTTON_PRESSED_COLOR,
    BUTTON_PRESSING_TIME,
)
from .widget import Widget
from .widget_variable import WidgetVariable


class Button(Widget):
    """
    Variables:
        is_pressed_var

    Events:
        press
    """

    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self._pressing_time = 0
        self.is_pressed_var = WidgetVariable(
            False, on_get=self.__on_is_pressed_get, on_set=self.__on_is_pressed_set
        )

        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_repeat", self.__on_mouse_down)
        self.add_event_listener("mouse_up", self.__on_mouse_up)
        self.add_event_listener("update", self.__on_update)

    @property
    def button_color(self):
        return (
            (BUTTON_PRESSED_COLOR if self.is_pressed_var.v else BUTTON_ENABLED_COLOR)
            if self.is_enabled_var.v
            else BUTTON_DISABLED_COLOR
        )

    def __on_is_pressed_get(self, value):
        return self._pressing_time > 0

    def __on_is_pressed_set(self, value):
        if value:
            self._pressing_time = BUTTON_PRESSING_TIME + 1
            self.trigger_event("press")
        else:
            self._pressing_time = 0

        return value

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self.is_pressed_var.v = True

    def __on_mouse_up(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self.is_pressed_var.v = False

    def __on_update(self):
        if self._pressing_time > 0:
            self._pressing_time -= 1
