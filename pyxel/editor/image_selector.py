import pyxel

from .settings import PANEL_FOCUS_BORDER_COLOR, PANEL_FOCUS_COLOR
from .widgets import ScrollBar, Widget


class ImageSelector(Widget):
    def __init__(self, parent, *, is_image_editor):
        y, height = (16, 130) if is_image_editor else (80, 66)
        super().__init__(parent, 157, y, 66, height)

        self._is_image_editor = is_image_editor
        self._viewport_x = 0
        self._viewport_y = 0
        self._press_x = 0
        self._press_y = 0
        self._drag_offset_x = 0
        self._drag_offset_y = 0
        # self._tile_table = [list(range(x, x + 32)) for x in range(0, 1024, 32)]
        self._h_scroll_bar = ScrollBar(
            self, 157, 145, 66, ScrollBar.HORIZONTAL, 32, 8, 0
        )
        self._v_scroll_bar = (
            ScrollBar(self, 222, 16, 130, ScrollBar.VERTICAL, 32, 16, 0)
            if is_image_editor
            else ScrollBar(self, 222, 80, 66, ScrollBar.VERTICAL, 32, 8, 0)
        )

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
        self._h_scroll_bar.add_event_handler("change", self.__on_h_scroll_bar_change)
        self._v_scroll_bar.add_event_handler("change", self.__on_v_scroll_bar_change)

    """
    @select_x.setter
    def select_x(self, value):
        self.parent.edit_x = min(max(value, 0), 256 - self.parent.edit_width)

        offset_left = self._viewport_x - self.parent.edit_x
        if offset_left > 0:
            self._viewport_x -= offset_left

        offset_right = (
            self.parent.edit_x + self.parent.edit_width - self._viewport_x - 64
        )
        if offset_right > 0:
            self._viewport_x += offset_right

    @select_y.setter
    def select_y(self, value):
        self.parent.edit_y = min(max(value, 0), 256 - self.parent.edit_height)

        offset_top = self._viewport_y - self.parent.edit_y
        if offset_top > 0:
            self._viewport_y -= offset_top

        offset_bottom = (
            self.parent.edit_y + self.parent.edit_height - self._viewport_y - 64
        )
        if offset_bottom > 0:
            self._viewport_y += offset_bottom

    @property
    def focused_tiles(self):
        x = self.parent.edit_x // 8
        y = self.parent.edit_y // 8
        width = self.parent.edit_width // 8
        height = self.parent.edit_height // 8

        return slice_array2d(self._tile_table, x, y, width, height)
    """

    def _screen_to_viewport(self, x, y):
        x = (x + self._viewport_x - self.x - 1) // 8 * 8
        y = (y + self._viewport_y - self.y - 1) // 8 * 8
        return x, y

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            x, y = self._screen_to_viewport(x, y)

            if self._is_image_editor:
                self.parent.edit_x = min(max(x, 0), 240)
                self.parent.edit_y = min(max(y, 0), 240)
            else:
                self.parent.edit_x = self._press_x = min(max(x, 0), 248)
                self.parent.edit_y = self._press_y = min(max(y, 0), 248)

        if key == pyxel.MOUSE_BUTTON_RIGHT:
            self._drag_offset_x = 0
            self._drag_offset_y = 0

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            if self._is_image_editor:
                self.__on_mouse_down(key, x, y)
            else:
                x, y = self._screen_to_viewport(x, y)

                x = min(max(x, 0), 248)
                y = min(max(y, 0), 248)

                self.parent.edit_x = min(self._press_x, x)
                self.parent.edit_y = min(self._press_y, y)

                self.parent.edit_width = min(abs(self._press_x - x) + 8, 64)
                self.parent.edit_height = min(abs(self._press_y - y) + 8, 64)
        elif key == pyxel.MOUSE_BUTTON_RIGHT:
            self._drag_offset_x -= dx
            self._drag_offset_y -= dy

            if abs(self._drag_offset_x) >= 8:
                offset = (self._drag_offset_x // 8) * 8
                self._drag_offset_x -= offset
                self._viewport_x += offset

            if abs(self._drag_offset_y) >= 8:
                offset = (self._drag_offset_y // 8) * 8
                self._drag_offset_y -= offset
                self._viewport_y += offset

            self._viewport_x = min(max(self._viewport_x, 0), 192)
            self._viewport_y = min(
                max(self._viewport_y, 0), 128 if self._is_image_editor else 192
            )

    def __on_mouse_hover(self, x, y):
        x, y = self._screen_to_viewport(x, y)
        s = "TARGET:CURSOR IMPORT:DROP" if self._is_image_editor else "VIEW:R-DRAG"
        self.parent.help_message = s + " ({},{})".format(x, y)

    def __on_update(self):
        pass

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        pyxel.blt(
            self.x + 1,
            self.y + 1,
            self.parent.image,
            self._viewport_x,
            self._viewport_y,
            self.width - 2,
            self.height - 2,
        )

        pyxel.clip(self.x + 1, self.y + 1, self.width - 2, self.height - 2)

        x = self.x + self.parent.edit_x - self._viewport_x + 1
        y = self.y + self.parent.edit_y - self._viewport_y + 1
        w = self.parent.edit_width
        h = self.parent.edit_height

        pyxel.rectb(x, y, w, h, PANEL_FOCUS_COLOR)
        pyxel.rectb(x + 1, y + 1, w - 2, h - 2, PANEL_FOCUS_BORDER_COLOR)
        pyxel.rectb(x - 1, y - 1, w + 2, h + 2, PANEL_FOCUS_BORDER_COLOR)

        pyxel.clip()

    def __on_h_scroll_bar_change(self, value):
        self._viewport_x = value * 8

    def __on_v_scroll_bar_change(self, value):
        self._viewport_y = value * 8
