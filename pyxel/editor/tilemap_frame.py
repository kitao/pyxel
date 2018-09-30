import pyxel
from pyxel.ui import Widget


class TilemapFrame(Widget):
    def __init__(self, parent):
        super().__init__(parent, 158, 17, 64, 63)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("draw", self.__on_draw)

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            x -= self.x
            y -= self.y

            self.parent.edit_x = min(max((x - 1) // 2, 0), 30) * 8
            self.parent.edit_y = min(max((y - 1) // 2, 0), 30) * 8

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.KEY_LEFT_BUTTON:
            self.__on_mouse_down(key, x, y)

    def __on_draw(self):
        pyxel.blt(self.x, self.y, 3, 0, 192, 64, 63)

        x = self.x + self.parent.edit_x // 4
        y = self.y + self.parent.edit_y // 4
        height = 4 if self.parent.edit_y < 240 else 3
        pyxel.rectb(x - 1, y - 1, x + 4, y + height, 7)
