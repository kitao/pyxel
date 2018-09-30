import pyxel

from .button import Button
from .widget import Widget


class NumberPicker(Widget):
    """
    Events:
        __on_change(value)
    """

    def __init__(self, parent, x, y, width, height, min_value, max_value, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self._min_value = min_value
        self._max_value = max_value
        self._value = 0

        self.dec_button = Button(
            self, x, y, self.button_size, self.button_size, is_key_repeat=True
        )
        self.inc_button = Button(
            self,
            x + width - self.button_size,
            y,
            self.button_size,
            self.button_size,
            is_key_repeat=True,
        )

        self.dec_button.add_event_handler("press", self.__on_dec_button_press)
        self.inc_button.add_event_handler("press", self.__on_inc_button_press)

    @property
    def button_size(self):
        return min(self.width, self.height)

    @property
    def value(self):
        return self._value

    @value.setter
    def value(self, value):
        if self._value != value:
            self._value = value
            self.call_event_handler("change", value)

    def __on_dec_button_press(self):
        offset = 10 if pyxel.btn(pyxel.KEY_SHIFT) else 1
        self.value = max(self._value - offset, self._min_value)

    def __on_inc_button_press(self):
        offset = 10 if pyxel.btn(pyxel.KEY_SHIFT) else 1
        self.value = min(self._value + offset, self._max_value)
