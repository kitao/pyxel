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

        self.add_event_handler('press', self.on_press)
        self.add_event_handler('draw', self.on_draw)

    def on_press(self, key, x, y):
        if (x % self._interval < BUTTON_SIZE
                and y % self._interval < BUTTON_SIZE):
            value = (y // self._interval) * self._col + x // self._interval

            if self.value != value:
                self.value = value
                self.call_event_handler('change', value)

    def on_draw(self):
        x = self.x + self._interval * (self.value % self._col)
        y = self.y + self._interval * (self.value // self._col)

        pyxel.pal(13, 7)
        pyxel.blt(x, y, 3, x, y + 16, BUTTON_SIZE, BUTTON_SIZE)
        pyxel.pal()

    def on_change(self, value):
        pass
