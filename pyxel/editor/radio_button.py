import pyxel

from .widget import Widget

BUTTON_SIZE = 7


class RadioButton(Widget):
    def __init__(self, parent, x, y, col, row, interval):
        width = interval * (col - 1) + BUTTON_SIZE
        height = interval * (row - 1) + BUTTON_SIZE
        super().__init__(parent, x, y, width, height)

        self.value = 0
        self._col = col
        self._row = row
        self._interval = interval

    def on_press(self, key, mx, my):
        super().on_press(key, mx, my)

        if (mx % self._interval < BUTTON_SIZE
                and my % self._interval < BUTTON_SIZE):
            value = (my // self._interval) * self._col + mx // self._interval

            if self.value != value:
                self.value = value
                self.on_value_change(value)

    def on_draw(self):
        super().on_draw()

        pyxel.pal(13, 7)

        x = self.x + self._interval * (self.value % self._col)
        y = self.y + self._interval * (self.value // self._col)
        pyxel.blt(x, y, 3, x, y + 16, BUTTON_SIZE, BUTTON_SIZE)

        pyxel.pal()

    def on_value_change(self, value):
        pass
