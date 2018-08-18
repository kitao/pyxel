import pyxel

from .widget import Widget

BUTTON_SIZE = 7


class RadioButton(Widget):
    def __init__(self, parent, x, y, col, row, interval):
        width = interval * (col - 1) + BUTTON_SIZE
        height = interval * (row - 1) + BUTTON_SIZE
        super().__init__(parent, x, y, width, height)

        self.index = 0
        self._col = col
        self._row = row
        self._interval = interval

        self.add_event_handler('press', self.on_press)
        self.add_event_handler('draw', self.on_draw)

    def on_press(self, key, x, y):
        if (x % self._interval < BUTTON_SIZE
                and y % self._interval < BUTTON_SIZE):
            index = (y // self._interval) * self._col + x // self._interval

            if self.index != index:
                self.index = index
                self.call_event_handler('change', index)

    def on_draw(self):
        pyxel.pal(13, 7)

        x = self.x + self._interval * (self.index % self._col)
        y = self.y + self._interval * (self.index // self._col)
        pyxel.blt(x, y, 3, x, y + 16, BUTTON_SIZE, BUTTON_SIZE)

        pyxel.pal()

    def on_change(self, index):
        pass
