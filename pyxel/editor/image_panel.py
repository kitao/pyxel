import numpy as np

import pyxel
from pyxel.ui import ScrollBar, Widget


class ImagePanel(Widget):
    def __init__(self, parent, *, is_tilemap_mode):
        y, height = (80, 66) if is_tilemap_mode else (16, 130)
        super().__init__(parent, 157, y, 66, height)

        self._is_tilemap_mode = is_tilemap_mode
        self.viewport_x = 0
        self.viewport_y = 0
        self._focus_x = 0
        self._focus_y = 0
        self._focus_width = 8 if is_tilemap_mode else 16
        self._focus_height = 8 if is_tilemap_mode else 16
        self._press_x = 0
        self._press_y = 0
        self._drag_offset_x = 0
        self._drag_offset_y = 0
        self._tile_table = np.arange(1024).reshape(32, 32)
        self._h_scroll_bar = ScrollBar(
            self, 157, 145, 66, ScrollBar.HORIZONTAL, 32, 8, 0
        )
        self._v_scroll_bar = (
            ScrollBar(self, 222, 80, 66, ScrollBar.VERTICAL, 32, 8, 0)
            if is_tilemap_mode
            else ScrollBar(self, 222, 16, 130, ScrollBar.VERTICAL, 32, 16, 0)
        )

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
        self._h_scroll_bar.add_event_handler("change", self.__on_h_scroll_bar_change)
        self._v_scroll_bar.add_event_handler("change", self.__on_v_scroll_bar_change)

    @property
    def focused_tiles(self):
        x = self._focus_x // 8
        y = self._focus_y // 8
        width = self._focus_width // 8
        height = self._focus_height // 8

        return self._tile_table[y : y + height, x : x + width]

    def set_focus(self, x, y):
        self._focus_x = min(max(x, 0), 256 - self._focus_width)
        self._focus_y = min(max(y, 0), 256 - self._focus_height)

        offset_left = self.viewport_x - self._focus_x
        if offset_left > 0:
            self.viewport_x -= offset_left

        offset_right = self._focus_x + self._focus_width - self.viewport_x - 64
        if offset_right > 0:
            self.viewport_x += offset_right

        offset_top = self.viewport_y - self._focus_y
        if offset_top > 0:
            self.viewport_y -= offset_top

        offset_bottom = self._focus_y + self._focus_height - self.viewport_y - 64
        if offset_bottom > 0:
            self.viewport_y += offset_bottom

    def _screen_to_view(self, x, y):
        x = (x + self.viewport_x - self.x - 1) // 8 * 8
        y = (y + self.viewport_y - self.y - 1) // 8 * 8
        return x, y

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.MOUSE_LEFT_BUTTON:
            x, y = self._screen_to_view(x, y)

            if self._is_tilemap_mode:
                x = min(max(x, 0), 248)
                y = min(max(y, 0), 248)

                self._press_x = x
                self._press_y = y

                self.set_focus(x, y)
            else:
                self._focus_x = min(max(x, 0), 240)
                self._focus_y = min(max(y, 0), 240)

                self.parent.drawing_x = self._focus_x
                self.parent.drawing_y = self._focus_y

        if key == pyxel.MOUSE_RIGHT_BUTTON:
            self._drag_offset_x = 0
            self._drag_offset_y = 0

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_LEFT_BUTTON:
            if self._is_tilemap_mode:
                x, y = self._screen_to_view(x, y)

                x = min(max(x, 0), 248)
                y = min(max(y, 0), 248)

                self._focus_x = min(self._press_x, x)
                self._focus_y = min(self._press_y, y)

                self._focus_width = min(abs(self._press_x - x) + 8, 64)
                self._focus_height = min(abs(self._press_y - y) + 8, 64)
            else:
                self.__on_mouse_down(key, x, y)
        elif key == pyxel.MOUSE_RIGHT_BUTTON:
            self._drag_offset_x -= dx
            self._drag_offset_y -= dy

            if abs(self._drag_offset_x) >= 8:
                offset = (self._drag_offset_x // 8) * 8
                self._drag_offset_x -= offset
                self.viewport_x += offset

            if abs(self._drag_offset_y) >= 8:
                offset = (self._drag_offset_y // 8) * 8
                self._drag_offset_y -= offset
                self.viewport_y += offset

            self.viewport_x = min(max(self.viewport_x, 0), 192)
            self.viewport_y = min(
                max(self.viewport_y, 0), 192 if self._is_tilemap_mode else 128
            )

    def __on_mouse_hover(self, x, y):
        x, y = self._screen_to_view(x, y)
        s = "VIEW:R-DRAG" if self._is_tilemap_mode else "TARGET:CURSOR IMPORT:DROP"
        self.parent.help_message = s + " ({},{})".format(x, y)

    def __on_update(self):
        if not self._is_tilemap_mode:
            self._focus_x = self.parent.drawing_x
            self._focus_y = self.parent.drawing_y

        self._h_scroll_bar.value = self.viewport_x // 8
        self._v_scroll_bar.value = self.viewport_y // 8

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        pyxel.blt(
            self.x + 1,
            self.y + 1,
            self.parent.image,
            self.viewport_x,
            self.viewport_y,
            self.width - 2,
            self.height - 2,
        )

        pyxel.clip(self.x + 1, self.y + 1, self.width - 2, self.height - 2)

        x = self.x + self._focus_x - self.viewport_x + 1
        y = self.y + self._focus_y - self.viewport_y + 1
        w = self._focus_width
        h = self._focus_height

        pyxel.rectb(x, y, w, h, 7)
        pyxel.rectb(x + 1, y + 1, w - 2, h - 2, 0)
        pyxel.rectb(x - 1, y - 1, w + 2, h + 2, 0)

        pyxel.clip()

    def __on_h_scroll_bar_change(self, value):
        self.viewport_x = value * 8

    def __on_v_scroll_bar_change(self, value):
        self.viewport_y = value * 8
