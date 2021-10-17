import pyxel

from .settings import BUTTON_DISABLED_COLOR, BUTTON_ENABLED_COLOR, BUTTON_PRESSED_COLOR
from .widget import Widget, WidgetVariable


class ToggleButton(Widget):
    """
    Variables:
        is_visible_var
        is_enabled_var
        is_checked_var

    Events:
        checked
        unchecked
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

    def __init__(self, parent, left, top, width, height, **kwargs):
        super().__init__(parent, left, top, width, height, **kwargs)

        self.is_checked_var = WidgetVariable(False)
        self.is_checked_var.add_event_listener("change", self.__on_checked_change)

        self.add_event_listener("mouse_down", self.__on_mouse_down)

    @property
    def button_color(self):
        return (
            (BUTTON_PRESSED_COLOR if self.is_checked_var.v else BUTTON_ENABLED_COLOR)
            if self.is_enabled_var.v
            else BUTTON_DISABLED_COLOR
        )

    def press(self):
        self.is_checked_var.v = not self.is_checked_var.v

    def __on_checked_change(self, value):
        if value:
            self.trigger_event("checked")
        else:
            self.trigger_event("unchecked")

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self.press()
