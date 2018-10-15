import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y


class SoundInput(Widget):
    def __init__(self, parent):
        super().__init__(parent, 30, 149, 193, 23)

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def __on_update(self):
        pass

    def __on_draw(self):
        pyxel.blt(self.x, self.y, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 132, 97, 23)
        pyxel.blt(self.x + 97, self.y, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 132, -96, 23)

        self._cursor_x = 0
        self._cursor_y = 0

        # if self._cursor_y > 0:
        #    y = self._cursor_y * 8 + 142
        #    pyxel.rect(x, y - 1, x + 2, y + 5, 1)

        sound = pyxel.sound(self.parent.sound)

        for i, tone in enumerate(sound.tone):
            col = 7 if self._cursor_y == 1 and self._cursor_x == i else 1
            pyxel.text(31 + i * 4, 150, "TSPN"[tone], col)

        for i, volume in enumerate(sound.volume):
            col = 7 if self._cursor_y == 2 and self._cursor_x == i else 1
            pyxel.text(31 + i * 4, 158, str(volume), col)

        for i, effect in enumerate(sound.effect):
            col = 7 if self._cursor_y == 3 and self._cursor_x == i else 1
            pyxel.text(31 + i * 4, 166, "NSVF"[effect], col)
