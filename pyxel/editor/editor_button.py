import pyxel
from pyxel.ui import Button


class EditorButton(Button):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        if not self.is_enabled or self.is_pressed:
            pyxel.pal(13, 7 if self.is_pressed else 5)
            pyxel.blt(self.x, self.y, 3, self.x, self.y + 12, self.width, self.height)
            pyxel.pal()
