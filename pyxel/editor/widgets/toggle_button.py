import pyxel

from .widget import Widget


class ToggleButton(Widget):
    """
    Events:
        __on_change(value)
    """

    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self._value = False

        self.add_event_handler("mouse_down", self.__on_mouse_down)

    @property
    def value(self):
        return self._value

    def press(self):
        self._value = not self._value
        self.call_event_handler("change", self._value)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self.press()
