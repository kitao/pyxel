import pyxel

from .settings import INPUT_FIELD_COLOR, INPUT_TEXT_COLOR
from .text_button import TextButton
from .widget import Widget


class NumberPicker(Widget):
    """
    Variables:
        value_var

    Events:
        change (value)
    """

    def __init__(self, parent, x, y, min_value, max_value, value, **kwargs):
        self._number_len = max(len(str(min_value)), len(str(max_value)))
        width = self._number_len * 4 + 21
        height = 7

        super().__init__(parent, x, y, width, height, **kwargs)

        self._min_value = min_value
        self._max_value = max_value

        self.add_event_listener("draw", self.__on_draw)

        # value_var
        self.make_variable(
            "value_var",
            value,
            on_set=self.__on_value_set,
            on_change=self.__on_value_change,
        )

        # dec button
        self.dec_button = TextButton(self, 0, 0, "-")
        self.dec_button.add_event_listener("press", self.__on_dec_button_press)

        # inc button
        self.inc_button = TextButton(self, width - 7, 0, "+")
        self.inc_button.add_event_listener("press", self.__on_inc_button_press)

    def __on_draw(self):
        pyxel.rect(self.x + 9, self.y, self.width - 18, self.height, INPUT_FIELD_COLOR)
        pyxel.text(
            self.x + 11,
            self.y + 1,
            ("{:>" + str(self._number_len) + "}").format(self.value_var),
            INPUT_TEXT_COLOR,
        )

    def __on_value_set(self, value):
        return min(max(value, self._min_value), self._max_value)

    def __on_value_change(self, value):
        self.dec_button.is_enabled_var = self.value_var > self._min_value
        self.inc_button.is_enabled_var = self.value_var < self._max_value

        self.trigger_event("change", value)

    def __on_dec_button_press(self):
        self.value_var -= 10 if pyxel.btn(pyxel.KEY_SHIFT) else 1

    def __on_inc_button_press(self):
        self.value_var += 10 if pyxel.btn(pyxel.KEY_SHIFT) else 1
