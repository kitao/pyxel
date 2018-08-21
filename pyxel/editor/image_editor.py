import numpy as np

import pyxel

from .radio_button import RadioButton
from .screen import Screen
from .scroll_bar import ScrollBar
from .widget import Widget


class EditWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 12, 17, 128, 128)

        self.edit_x = 0
        self.edit_y = 0

        self._last_x = 0
        self._last_y = 0

        self._drag_offset_x = 0
        self._drag_offset_y = 0

        self._canvas = np.ndarray((16, 16), np.int8)
        self._canvas[:, :] = -1

        self._h_scroll_bar = ScrollBar(self, 11, 145, 130, 7, 2, 32)
        self._h_scroll_bar.add_event_handler('change', self.on_change_x)

        self._v_scroll_bar = ScrollBar(self, 140, 16, 7, 130, 2, 32)
        self._v_scroll_bar.add_event_handler('change', self.on_change_y)

        self.add_event_handler('press', self.on_press)
        self.add_event_handler('release', self.on_release)
        self.add_event_handler('click', self.on_click)
        self.add_event_handler('drag', self.on_drag)
        self.add_event_handler('draw', self.on_draw)

    def _draw_line(self, x1, y1, x2, y2, col):
        if x1 == x2 and y1 == y2:
            if x1 >= 0 and x1 < 16 and y1 >= 0 and y1 < 16:
                self._canvas[y1, x1] = col
            return

        dx = x2 - x1
        dy = y2 - y1

        if abs(dx) > abs(dy):
            if dx < 0:
                x1, y1 = x2, y2
                dx, dy = -dx, -dy

            for i in range(dx + 1):
                x = x1 + i
                y = int(y1 + i * dy / dx + 0.5)

                if x >= 0 and x < 16 and y >= 0 and y < 16:
                    self._canvas[y, x] = col
        else:
            if dy < 0:
                x1, y1 = x2, y2
                dx, dy = -dx, -dy

            for i in range(dy + 1):
                x = int(x1 + i * dx / dy + 0.5)
                y = y1 + i

                if x >= 0 and x < 16 and y >= 0 and y < 16:
                    self._canvas[y, x] = col

    def _copy_canvas(self):
        img = self.parent.image_button.value
        dest = pyxel.image(img).data[self.edit_y:self.edit_y +
                                     16, self.edit_x:self.edit_x + 16]
        index = self._canvas != -1
        dest[index] = self._canvas[index]
        self._canvas[:, :] = -1

    def on_change_x(self, value):
        self.edit_x = value * 8

    def on_change_y(self, value):
        self.edit_y = value * 8

    def on_press(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            x //= 8
            y //= 8
            self._canvas[y, x] = self.parent.color_button.value
            self._copy_canvas()
            self._last_x = x
            self._last_y = y

    def on_release(self, key, x, y):
        self._copy_canvas()

    def on_click(self, key, x, y):
        if key == pyxel.KEY_RIGHT_BUTTON:
            img = self.parent.image_button.value
            x = self.edit_x + x // 8
            y = self.edit_y + y // 8
            self.parent.color_button.value = pyxel.image(img).data[y, x]

    def on_drag(self, key, x, y, dx, dy):
        if key == pyxel.KEY_LEFT_BUTTON:
            x //= 8
            y //= 8
            self._draw_line(self._last_x, self._last_y, x, y,
                            self.parent.color_button.value)
            self._copy_canvas()
            self._last_x = x
            self._last_y = y

        elif key == pyxel.KEY_RIGHT_BUTTON:
            self._drag_offset_x -= dx
            self._drag_offset_y -= dy

            if abs(self._drag_offset_x) >= 8:
                offset = (self._drag_offset_x // 8) * 8
                self.edit_x += offset
                self._drag_offset_x -= offset

            if abs(self._drag_offset_y) >= 8:
                offset = (self._drag_offset_y // 8) * 8
                self.edit_y += offset
                self._drag_offset_y -= offset

            self.edit_x = min(max(self.edit_x, 0), 240)
            self.edit_y = min(max(self.edit_y, 0), 240)

            self._h_scroll_bar.value = self.edit_x // 8
            self._v_scroll_bar.value = self.edit_y // 8

    def on_draw(self):
        for i in range(16):
            y = self.y + i * 8
            for j in range(16):
                x = self.x + j * 8

                if self._canvas[i, j] >= 0:
                    col = self._canvas[i, j]
                else:
                    data = pyxel.image(self.parent.image_button.value).data
                    col = data[self.edit_y + i, self.edit_x + j]

                pyxel.rect(x, y, x + 7, y + 7, col)

        pyxel.line(self.x, self.y + 63, self.x + 127, self.y + 63, 1)
        pyxel.line(self.x + 63, self.y, self.x + 63, self.y + 127, 1)


class PreviewWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 158, 17, 64, 128)

        self.preview_x = 0
        self.preview_y = 0

        self._drag_offset_x = 0
        self._drag_offset_y = 0

        self._h_scroll_bar = ScrollBar(self, 157, 145, 66, 7, 8, 32)
        self._h_scroll_bar.add_event_handler('change', self.on_change_x)

        self._v_scroll_bar = ScrollBar(self, 222, 16, 7, 130, 16, 32)
        self._v_scroll_bar.add_event_handler('change', self.on_change_y)

        self.add_event_handler('press', self.on_press)
        self.add_event_handler('drag', self.on_drag)
        self.add_event_handler('draw', self.on_draw)

    def on_change_x(self, value):
        self.preview_x = value * 8

    def on_change_y(self, value):
        self.preview_y = value * 8

    def on_press(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            self.parent.edit_window.edit_x = self.preview_x + (
                (x - 4) // 8) * 8
            self.parent.edit_window.edit_y = self.preview_y + (
                (y - 4) // 8) * 8

            self.parent.edit_window.edit_x = min(
                max(self.parent.edit_window.edit_x, 0), 240)
            self.parent.edit_window.edit_y = min(
                max(self.parent.edit_window.edit_y, 0), 240)

        if key == pyxel.KEY_RIGHT_BUTTON:
            self._drag_offset_x = 0
            self._drag_offset_y = 0

    def on_drag(self, key, x, y, dx, dy):
        if key == pyxel.KEY_RIGHT_BUTTON:
            self._drag_offset_x -= dx * 2
            self._drag_offset_y -= dy * 2

            if abs(self._drag_offset_x) >= 8:
                offset = (self._drag_offset_x // 8) * 8
                self.preview_x += offset
                self._drag_offset_x -= offset

            if abs(self._drag_offset_y) >= 8:
                offset = (self._drag_offset_y // 8) * 8
                self.preview_y += offset
                self._drag_offset_y -= offset

            self.preview_x = min(max(self.preview_x, 0), 192)
            self.preview_y = min(max(self.preview_y, 0), 128)

    def on_draw(self):
        img = self.parent.image_button.value
        pyxel.blt(self.x, self.y, img, self.preview_x, self.preview_y, 64, 128)

        pyxel.clip(self.x - 1, self.y - 1, self.x + self.width,
                   self.y + self.height)
        x = self.x + self.parent.edit_window.edit_x - self.preview_x
        y = self.y + self.parent.edit_window.edit_y - self.preview_y
        pyxel.rectb(x - 1, y - 1, x + 16, y + 16, 7)
        pyxel.clip()


class ImageEditor(Screen):
    def __init__(self, parent):
        super().__init__(parent, 'image_editor.png')

        def on_draw():
            widget = self.color_button
            x = widget.x + (widget.value % 8) * 8
            y = widget.y + (widget.value // 8) * 8
            col = 7 if widget.value < 6 else 0
            pyxel.text(x + 2, y + 1, '+', col)

        self.color_button = RadioButton(self, 12, 157, 8, 2, 8)
        self.color_button.remove_event_handler('draw',
                                               self.color_button.on_draw)
        self.color_button.add_event_handler('draw', on_draw)
        self.color_button.value = 7

        self.tool_button = RadioButton(self, 81, 161, 7, 1, 9)
        self.image_button = RadioButton(self, 191, 161, 3, 1, 10)
        self.edit_window = EditWindow(self)
        self.preview_window = PreviewWindow(self)
