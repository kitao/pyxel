import pyxel

from .widget import Widget, WidgetVariable


class ToggleButton(Widget):
    """
    Events:
        checked
        unchecked
    """

    def __init__(self, parent, left, top, width, height, **kwargs):
        super().__init__(parent, left, top, width, height, **kwargs)

        def on_checked_change(value):
            if value:
                self.trigger_event("checked")
            else:
                self.trigger_event("unchecked")

        self.is_checked_var = WidgetVariable(False, on_checked_change)

        self.add_event_listener("mouse_down", self.__on_mouse_down)

    def press(self):
        self.is_checked_var.v = not self.is_checked_var.v

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self.press()
