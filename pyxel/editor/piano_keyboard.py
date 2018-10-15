import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y


class PianoKeyboard(Widget):
    def __init__(self, parent):
        super().__init__(parent, 17, 25, 12, 123)

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def __on_update(self):
        pass

    def __on_draw(self):
        pyxel.blt(self.x, self.y, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 8, 12, 123, 6)
