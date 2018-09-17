import pyxel
from pyxel.ui import Button


class EditorButton(Button):
    def __init__(self, parent, x, y, width, height, **kwargs):
        super().__init__(parent, x, y, width, height, **kwargs)

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        if self.is_down:
            pyxel.pal(13, 7 if self.is_enabled else 5)
            pyxel.blt(self.x, self.y, 3, self.x, self.y + 16, self.width, self.height)
            pyxel.pal()
