import pyxel
from pyxel.ui import Widget

from .editor_scroll_bar import EditorScrollBar


class ImageWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 158, 17, 64, 128)

        self.image_x = 0
        self.image_y = 0

        self._drag_offset_x = 0
        self._drag_offset_y = 0

        self._h_scroll_bar = EditorScrollBar(self, 157, 145, 66, 7, 32, 8)
        self._h_scroll_bar.add_event_handler("change", self.__on_change_x)

        self._v_scroll_bar = EditorScrollBar(self, 222, 16, 7, 130, 32, 16)
        self._v_scroll_bar.add_event_handler("change", self.__on_change_y)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("draw", self.__on_draw)

    def __on_change_x(self, value):
        self.image_x = value * 8

    def __on_change_y(self, value):
        self.image_y = value * 8

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            x -= self.x
            y -= self.y

            self.parent.edit_x = self.image_x + ((x - 4) // 8) * 8
            self.parent.edit_y = self.image_y + ((y - 4) // 8) * 8

            self.parent.edit_x = min(max(self.parent.edit_x, 0), 240)
            self.parent.edit_y = min(max(self.parent.edit_y, 0), 240)

        if key == pyxel.KEY_RIGHT_BUTTON:
            self._drag_offset_x = 0
            self._drag_offset_y = 0

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.KEY_RIGHT_BUTTON:
            self._drag_offset_x -= dx * 2
            self._drag_offset_y -= dy * 2

            if abs(self._drag_offset_x) >= 8:
                offset = (self._drag_offset_x // 8) * 8
                self.image_x += offset
                self._drag_offset_x -= offset

            if abs(self._drag_offset_y) >= 8:
                offset = (self._drag_offset_y // 8) * 8
                self.image_y += offset
                self._drag_offset_y -= offset

            self.image_x = min(max(self.image_x, 0), 192)
            self.image_y = min(max(self.image_y, 0), 128)

    def __on_draw(self):
        pyxel.blt(
            self.x, self.y, self.parent.image, self.image_x, self.image_y, 64, 128
        )

        pyxel.clip(self.x - 1, self.y - 1, self.x + self.width, self.y + self.height)
        x = self.x + self.parent.edit_x - self.image_x
        y = self.y + self.parent.edit_y - self.image_y
        pyxel.rectb(x - 1, y - 1, x + 16, y + 16, 7)
        pyxel.clip()
