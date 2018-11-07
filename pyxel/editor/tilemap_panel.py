import pyxel
from pyxel.ui import Widget

from .constants import (
    TILEMAP_IMAGE_HEIGHT,
    TILEMAP_IMAGE_WIDTH,
    TILEMAP_IMAGE_X,
    TILEMAP_IMAGE_Y,
)


class TilemapPanel(Widget):
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
        if key == pyxel.MOUSE_LEFT_BUTTON:
            self.parent.drawing_x, self.parent.drawing_y = self._screen_to_view(x, y)

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_LEFT_BUTTON:
            self.__on_mouse_down(key, x, y)

    def __on_mouse_hover(self, x, y):
        x, y = self._screen_to_view(x, y)
        self.parent.help_message = "TARGET:CURSOR ({},{})".format(x, y)

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

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

        x1 = self.x + self.parent.drawing_x // 4 + 1
        y1 = self.y + self.parent.drawing_y // 4 + 1
        x2 = x1 + 3
        y2 = y1 + 3

        pyxel.rectb(x1, y1, x2, y2, 7)
        pyxel.rectb(x1 - 1, y1 - 1, x2 + 1, y2 + 1, 0)

        pyxel.clip()
