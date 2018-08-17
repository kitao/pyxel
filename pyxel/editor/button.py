import pyxel

from .widget import Widget


class RadioButton(Widget):
    def __init__(self, parent, x, y, width, height):
        super().__init__(parent, x, y, width, height)

        self.add_event_handler('draw', self.on_draw)

    def on_draw(self):
        pyxel.pal(13, 7)

        x = self.x + self._interval * (self.value % self._col)
        y = self.y + self._interval * (self.value // self._col)
        pyxel.blt(x, y, 3, x, y + 16, self.width, self.height)

        pyxel.pal()
