import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y


class MusicField(Widget):
    def __init__(self, parent, x, y, ch):
        super().__init__(parent, x, y, 218, 21)

        self._ch = ch

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

        self.data.extend([0, 1, 2, 3, 4] * 6)

    @property
    def data(self):
        music = pyxel.music(self.parent.music)

        if self._ch == 0:
            data = music.ch0
        elif self._ch == 1:
            data = music.ch1
        elif self._ch == 2:
            data = music.ch2
        elif self._ch == 3:
            data = music.ch3

        return data

    def __on_update(self):
        pass

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        pyxel.text(self.x + 5, self.y + 8, "CH{}".format(self._ch), 6)
        pyxel.blt(
            self.x + 20, self.y + 1, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 102, 191, 19, 6
        )

        if self.parent.cursor_y == self._ch:
            x = self.x + (self.parent.cursor_x % 16) * 12 + 22
            y = (
                self.y
                + (self.parent.cursor_y - self._ch + self.parent.cursor_x // 16) * 10
                + 2
            )
            pyxel.rect(x, y, x + 6, y + 6, 8)

        data = self.data

        for i in range(min(len(data), 16)):
            pyxel.text(self.x + 22 + i * 12, self.y + 3, "{:0>2}".format(data[i]), 1)

        for i in range(len(data) - 16):
            pyxel.text(
                self.x + 22 + i * 12, self.y + 13, "{:0>2}".format(data[i + 16]), 1
            )
