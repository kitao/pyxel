import pyxel

from .text_button import TextButton
from .ui_constants import INPUT_FIELD_COLOR, INPUT_TEXT_COLOR
from .widget import Widget


class NumberPicker(Widget):
    """
    Events:
        __on_change(value)
    """

    def __init__(self, parent, x, y, min_value, max_value, **kwargs):
        width = max(len(str(min_value)), len(str(max_value))) * 4 + 21
        height = 7
        super().__init__(parent, x, y, width, height, **kwargs)

        self._min_value = min_value
        self._max_value = max_value
        self._value = None

        self.dec_button = TextButton(self, x, y, "-")
        self.inc_button = TextButton(self, x + width - 7, y, "+")

        self.add_event_handler("draw", self.__on_draw)

        self.dec_button.add_event_handler("press", self.__on_dec_button_press)
        self.inc_button.add_event_handler("press", self.__on_inc_button_press)

        self.value = 0

    @property
    def value(self):
        return self._value

    @value.setter
    def value(self, value):
        if self._value != value:
            self._value = value
            self.call_event_handler("change", value)

            self.dec_button.is_enabled = self._value != self._min_value
            self.inc_button.is_enabled = self._value != self._max_value

    def __on_draw(self):
        pyxel.rect(
            self.x + 9,
            self.y,
            self.x + self.width - 10,
            self.y + self.height - 1,
            INPUT_FIELD_COLOR,
        )
        pyxel.text(self.x + 11, self.y + 1, str(self._value), INPUT_TEXT_COLOR)

    def __on_dec_button_press(self):
        offset = 10 if pyxel.btn(pyxel.KEY_SHIFT) else 1
        self.value = max(self._value - offset, self._min_value)

    def __on_inc_button_press(self):
        offset = 10 if pyxel.btn(pyxel.KEY_SHIFT) else 1
        self.value = min(self._value + offset, self._max_value)
