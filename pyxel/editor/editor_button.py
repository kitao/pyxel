import pyxel
from pyxel.ui import Button


class EditorButton(Button):
    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        if self.is_blinking:
            pyxel.pal(13, 7 if self.is_enabled else 5)

            x = self.x
            y = self.y
            pyxel.blt(x, y, 3, x, y + 16, self.width, self.height)

            pyxel.pal()
