import pyxel
from pyxel.ui import Widget

from .constants import (
    TILEMAP_IMAGE_HEIGHT,
    TILEMAP_IMAGE_WIDTH,
    TILEMAP_IMAGE_X,
    TILEMAP_IMAGE_Y,
)


class TilemapFrame(Widget):
    def __init__(self, parent):
        super().__init__(parent, 157, 16, 66, 65)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("draw", self.__on_draw)

    def _screen_to_view(self, x, y):
        x = min(max((x - self.x - 1) // 2, 0), 30) * 8
        y = min(max((y - self.y - 1) // 2, 0), 30) * 8
        return x, y

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            self.parent.edit_x, self.parent.edit_y = self._screen_to_view(x, y)

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.KEY_LEFT_BUTTON:
            self.__on_mouse_down(key, x, y)

    def __on_mouse_hover(self, x, y):
        x, y = self._screen_to_view(x, y)
        self.parent.help_message = "TARGET:CURSOR ({},{})".format(x, y)

    def __on_draw(self):
        self.draw_frame(self.x, self.y, self.width, self.height)

        pyxel.blt(
            self.x + 1,
            self.y + 1,
            3,
            TILEMAP_IMAGE_X,
            TILEMAP_IMAGE_Y,
            TILEMAP_IMAGE_WIDTH,
            TILEMAP_IMAGE_HEIGHT,
        )

        pyxel.clip(
            self.x + 1, self.y + 1, self.x + self.width - 2, self.y + self.height - 2
        )

        x = self.x + self.parent.edit_x // 4
        y = self.y + self.parent.edit_y // 4
        pyxel.rectb(x, y, x + 5, y + 5, 0)
        pyxel.rectb(x - 1, y - 1, x + 6, y + 6, 7)
        pyxel.rectb(x - 2, y - 2, x + 7, y + 7, 0)

        pyxel.clip()
