import pyxel
from pyxel.ui import Button


class RadioButton(Button):
    """
    on_change(value)
    """

    def __init__(
        self, parent, x, y, button_w, button_h, margin_x, margin_y, col, row, **kwargs
    ):
        width = button_w * col + margin_x * (col - 1)
        height = button_h * row + margin_y * (row - 1)
        super().__init__(parent, x, y, width, height, **kwargs)

        self.value = 0
        self._button_w = button_w
        self._button_h = button_h
        self._margin_x = margin_x
        self._margin_y = margin_y
        self._col = col
        self._row = row

        self.add_event_handler("press", self.__on_press)

    def __on_press(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        for i in range(self._row):
            for j in range(self._col):
                bx = (self._button_w + self._margin_x) * j
                by = (self._button_h + self._margin_y) * i

                if (
                    x >= bx
                    and x < bx + self._button_w
                    and y >= by
                    and y < by + self._button_h
                ):
                    value = self._col * i + j
                    if self.value != value:
                        self.value = value
                        self.call_event_handler("change", value)
                    return
