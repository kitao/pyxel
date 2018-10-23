import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y


class SoundSelector(Widget):
    def __init__(self, parent):
        super().__init__(parent, 11, 129, 218, 44)

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def __on_update(self):
        pass

    def __on_draw(self):
        self.draw_panel(11, 129, 218, 44)
        pyxel.blt(17, 134, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 121, 206, 34, 6)

        # for i in range(4):
        #    for j in range(16):
        #        pyxel.text(19 + j * 13, 135 + i * 9, "{:0>2}".format(i * 16 + j), 1)
