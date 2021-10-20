import pyxel

from .settings import BUTTON_DISABLED_COLOR, BUTTON_ENABLED_COLOR, BUTTON_PRESSED_COLOR
from .widget import Widget


class ToggleButton(Widget):
    """
    Variables:
        is_checked_var

    Events:
        checked
        unchecked
    """

    def __init__(self, parent, left, top, width, height, **kwargs):
        super().__init__(parent, left, top, width, height, **kwargs)

        self.add_event_listener("mouse_down", self.__on_mouse_down)

        # is_checked_var
        self.make_variable("is_checked_var", on_change=self.__on_is_checked_change)

    @property
    def button_color(self):
        return (
            (BUTTON_PRESSED_COLOR if self.is_checked_var else BUTTON_ENABLED_COLOR)
            if self.is_enabled_var
            else BUTTON_DISABLED_COLOR
        )

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self.is_checked_var = not self.is_checked_var

    def __on_is_checked_change(self, value):
        if value:
            self.trigger_event("checked")
        else:
            self.trigger_event("unchecked")
