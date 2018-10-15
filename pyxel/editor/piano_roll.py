import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y


class PianoRoll(Widget):
    def __init__(self, parent):
        super().__init__(parent, 30, 25, 193, 123)

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def __on_update(self):
        pass

    def __on_draw(self):
        pyxel.rect(self.x, self.y, self.x + self.width - 1, self.y + self.height - 1, 6)

        self._cursor_x = 0
        self._cursor_y = 0

        x = self._cursor_x * 4 + 31
        if self._cursor_y == 0:
            pyxel.rect(x, 25, x + 2, 147, 1)

        pyxel.blt(
            self.x, self.y, 3, EDITOR_IMAGE_X + 16, EDITOR_IMAGE_Y + 8, 97, 123, 6
        )
        pyxel.blt(
            self.x + 97, self.y, 3, EDITOR_IMAGE_X + 16, EDITOR_IMAGE_Y + 8, -96, 123, 6
        )

        sound = pyxel.sound(self.parent.sound)

        for i, note in enumerate(sound.note):
            x = i * 4 + 31
            y = 143 - note * 2
            pyxel.rect(x, y, x + 2, y + 2, 8 if note >= 0 else 12)
