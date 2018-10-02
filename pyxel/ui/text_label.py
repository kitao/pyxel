import pyxel
from pyxel.constants import FONT_HEIGHT, FONT_WIDTH

from .widget import Widget


class TextLabel(Widget):
    def __init__(self, parent, x, y, s, col, **kwargs):
        super().__init__(parent, x, y, len(s) * FONT_WIDTH, FONT_HEIGHT, **kwargs)

        self._s = s
        self._col = col

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        pyxel.text(self._x, self._y, self._s, self._col)
