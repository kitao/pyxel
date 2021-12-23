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

    def __init__(self, parent, x, y, width, height, *, is_checked, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        # Initialize is_checked_var
        self.new_var("is_checked_var", is_checked)
        self.add_var_event_listener(
            "is_checked_var", "change", self.__on_is_checked_change
        )

        # Initialize event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)

    @property
    def button_color(self):
        if not self.is_enabled_var:
            return BUTTON_DISABLED_COLOR
        elif self.is_checked_var:
            return BUTTON_PRESSED_COLOR
        else:
            return BUTTON_ENABLED_COLOR

    def __on_is_checked_change(self, value):
        self.trigger_event("checked" if value else "unchecked")

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            self.is_checked_var = not self.is_checked_var
