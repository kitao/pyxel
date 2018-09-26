import pyxel
from pyxel.ui import Widget

from .editor_scroll_bar import EditorScrollBar


class SelectWindow(Widget):
    def __init__(self, parent, *, is_tilemap_mode):
        super().__init__(parent, 158, 17, 64, 128)

        self._is_tilemap_mode = is_tilemap_mode

        self.select_x = 0
        self.select_y = 0

        self.cursor_x = 0
        self.cursor_y = 0

        self._drag_offset_x = 0
        self._drag_offset_y = 0

        self._h_scroll_bar = EditorScrollBar(self, 157, 145, 66, 7, 32, 8)
        self._h_scroll_bar.add_event_handler("change", self.__on_change_x)

        self._v_scroll_bar = EditorScrollBar(self, 222, 16, 7, 130, 32, 16)
        self._v_scroll_bar.add_event_handler("change", self.__on_change_y)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def __on_change_x(self, value):
        self.select_x = value * 8

    def __on_change_y(self, value):
        self.select_y = value * 8

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            x -= self.x
            y -= self.y

            if self._is_tilemap_mode:
                self.cursor_x = min(max(self.select_x + (x // 8) * 8, 0), 248)
                self.cursor_y = min(max(self.select_y + (y // 8) * 8, 0), 248)
            else:
                self.cursor_x = min(max(self.select_x + ((x - 4) // 8) * 8, 0), 240)
                self.cursor_y = min(max(self.select_y + ((y - 4) // 8) * 8, 0), 240)

                self.parent.edit_x = self.cursor_x
                self.parent.edit_y = self.cursor_y

        if key == pyxel.KEY_RIGHT_BUTTON:
            self._drag_offset_x = 0
            self._drag_offset_y = 0

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.KEY_RIGHT_BUTTON:
            self._drag_offset_x -= dx * 2
            self._drag_offset_y -= dy * 2

            if abs(self._drag_offset_x) >= 8:
                offset = (self._drag_offset_x // 8) * 8
                self.select_x += offset
                self._drag_offset_x -= offset

            if abs(self._drag_offset_y) >= 8:
                offset = (self._drag_offset_y // 8) * 8
                self.select_y += offset
                self._drag_offset_y -= offset

            self.select_x = min(max(self.select_x, 0), 192)
            self.select_y = min(max(self.select_y, 0), 128)

    def __on_update(self):
        self._h_scroll_bar.value = self.select_x // 8
        self._v_scroll_bar.value = self.select_y // 8

    def __on_draw(self):
        pyxel.blt(
            self.x, self.y, self.parent.image, self.select_x, self.select_y, 64, 128
        )

        pyxel.clip(self.x - 1, self.y - 1, self.x + self.width, self.y + self.height)
        x = self.x + self.cursor_x - self.select_x
        y = self.y + self.cursor_y - self.select_y

        if self._is_tilemap_mode:
            pyxel.rectb(x - 1, y - 1, x + 8, y + 8, 7)
        else:
            pyxel.rectb(x - 1, y - 1, x + 16, y + 16, 7)

        pyxel.clip()
