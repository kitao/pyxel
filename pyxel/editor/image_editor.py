import numpy as np

import pyxel
from pyxel.ui import Widget

from .editor import Editor
from .editor_radio_button import EditorRadioButton
from .editor_scroll_bar import EditorScrollBar

TOOL_SELECT = 0
TOOL_PENCIL = 1
TOOL_RECTB = 2
TOOL_RECT = 3
TOOL_CIRCB = 4
TOOL_CIRC = 5
TOOL_BUCKET = 6


class OverlayCanvas:
    def __init__(self):
        self.data = np.ndarray((16, 16), np.int8)
        self.clear()

    def clear(self):
        self.data[:, :] = -1

    @staticmethod
    def _adjust_region(x1, y1, x2, y2, is_guide_mode):
        if is_guide_mode:
            dx = x2 - x1
            dy = y2 - y1

            if abs(dx) > abs(dy):
                y2 = y1 + abs(dx) * (1 if dy > 0 else -1)
            else:
                x2 = x1 + abs(dy) * (1 if dx > 0 else -1)

        x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
        y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)

        return x1, y1, x2, y2

    @staticmethod
    def _inner_ellipse(x, y, a, b):
        a += 0.41
        b += 0.41
        return x * x * b * b + y * y * a * a < a * a * b * b

    def pix(self, x, y, val):
        if x >= 0 and x < 16 and y >= 0 and y < 16:
            self.data[y, x] = val

    def line(self, x1, y1, x2, y2, val):
        if x1 == x2 and y1 == y2:
            self.pix(x1, y1, val)
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
                    self.data[y, x] = val
        else:
            if dy < 0:
                x1, y1 = x2, y2
                dx, dy = -dx, -dy

            for i in range(dy + 1):
                x = int(x1 + i * dx / dy + 0.5)
                y = y1 + i

                if x >= 0 and x < 16 and y >= 0 and y < 16:
                    self.data[y, x] = val

    def rectb(self, x1, y1, x2, y2, val, is_guide_mode):
        x1, y1, x2, y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)

        for y in range(max(y1, 0), min(y2 + 1, 16)):
            for x in range(max(x1, 0), min(x2 + 1, 16)):
                if x == x1 or x == x2 or y == y1 or y == y2:
                    self.data[y, x] = val

    def rect(self, x1, y1, x2, y2, val, is_guide_mode):
        x1, y1, x2, y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)

        for y in range(max(y1, 0), min(y2 + 1, 16)):
            for x in range(max(x1, 0), min(x2 + 1, 16)):
                self.data[y, x] = val

    def circb(self, x1, y1, x2, y2, val, is_guide_mode):
        x1, y1, x2, y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)

        a = (x2 - x1) / 2
        b = (y2 - y1) / 2

        if a <= 0.5 or b <= 0.5:
            self.rect(x1, y1, x2, y2, val, False)
            return

        cx = (x1 + x2) / 2
        cy = (y1 + y2) / 2

        for y in range(max(y1, 0), min(y2 + 1, 16)):
            for x in range(max(x1, 0), min(x2 + 1, 16)):
                dx = x - cx
                dy = y - cy

                if self._inner_ellipse(dx, dy, a, b) and (
                    not self._inner_ellipse(dx - 1, dy, a, b)
                    or not self._inner_ellipse(dx + 1, dy, a, b)
                    or not self._inner_ellipse(dx, dy - 1, a, b)
                    or not self._inner_ellipse(dx, dy + 1, a, b)
                ):
                    self.data[y, x] = val

    def circ(self, x1, y1, x2, y2, val, is_guide_mode):
        x1, y1, x2, y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)

        a = (x2 - x1) / 2
        b = (y2 - y1) / 2

        if a <= 0.5 or b <= 0.5:
            self.rect(x1, y1, x2, y2, val, False)
            return

        cx = (x1 + x2) / 2
        cy = (y1 + y2) / 2

        for y in range(max(y1, 0), min(y2 + 1, 16)):
            for x in range(max(x1, 0), min(x2 + 1, 16)):
                if self._inner_ellipse(x - cx, y - cy, a, b):
                    self.data[y, x] = val

    def paint(self, x, y, val, dest):
        dest_val = dest[y, x]

        if dest_val == val:
            return

        for i in range(x, -1, -1):
            if dest[y, i] != dest_val:
                break

            dest[y, i] = val

            if y > 0 and dest[y - 1, i] == dest_val:
                self.paint(i, y - 1, val, dest)

            if y < 15 and dest[y + 1, i] == dest_val:
                self.paint(i, y + 1, val, dest)

        for i in range(x + 1, 16):
            if dest[y, i] != dest_val:
                return

            dest[y, i] = val

            if y > 0 and dest[y - 1, i] == dest_val:
                self.paint(i, y - 1, val, dest)

            if y < 15 and dest[y + 1, i] == dest_val:
                self.paint(i, y + 1, val, dest)


class EditWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 12, 17, 128, 128)

        self.edit_x = 0
        self.edit_y = 0

        self._press_x = 0
        self._press_y = 0

        self._last_x = 0
        self._last_y = 0

        self._drag_offset_x = 0
        self._drag_offset_y = 0

        self._select_x1 = 0
        self._select_y1 = 0
        self._select_x2 = 0
        self._select_y2 = 0
        self._copy_buffer = None

        self._is_dragged = False
        self._is_guide_mode = False

        self._h_scroll_bar = EditorScrollBar(self, 11, 145, 130, 7, 32, 2)
        self._h_scroll_bar.add_event_handler("change", self.__on_change_x)

        self._v_scroll_bar = EditorScrollBar(self, 140, 16, 7, 130, 32, 2)
        self._v_scroll_bar.add_event_handler("change", self.__on_change_y)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_up", self.__on_release)
        self.add_event_handler("mouse_click", self.__on_mouse_click)
        self.add_event_handler("mouse_drag", self.__on_drag)
        self.add_event_handler("update", self.on_update)
        self.add_event_handler("draw", self.on_draw)

        self.parent.color_button.add_event_handler("change", self.on_color_change)
        self.parent.tool_button.add_event_handler("change", self.on_tool_change)

        self._overlay_canvas = OverlayCanvas()

    @property
    def tool(self):
        return self.parent.tool_button.value

    @tool.setter
    def tool(self, value):
        self.parent.tool_button.value = value

    @property
    def color(self):
        return self.parent.color_button.value

    @color.setter
    def color(self, value):
        self.parent.color_button.value = value

    def __on_change_x(self, value):
        self.edit_x = value * 8

    def __on_change_y(self, value):
        self.edit_y = value * 8

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        x = (x - self.x) // 8
        y = (y - self.y) // 8

        self._press_x = x
        self._press_y = y

        self._is_dragged = True
        self._is_guide_mode = False

        if self.tool == TOOL_SELECT:
            self._select_x1 = self._select_x2 = x
            self._select_y1 = self._select_y2 = y
        elif self.tool >= TOOL_PENCIL and self.tool <= TOOL_CIRC:
            self._overlay_canvas.pix(x, y, self.color)
        elif self.tool == TOOL_BUCKET:
            img = self.parent.image_button.value
            dest = pyxel.image(img).data[
                self.edit_y : self.edit_y + 16, self.edit_x : self.edit_x + 16
            ]

            data = {}
            data["img"] = img
            data["pos"] = (self.edit_x, self.edit_y)
            data["before"] = dest.copy()

            img = self.parent.image_button.value
            dest = pyxel.image(img).data[
                self.edit_y : self.edit_y + 16, self.edit_x : self.edit_x + 16
            ]
            self._overlay_canvas.paint(x, y, self.color, dest)

            data["after"] = dest.copy()
            self.parent.add_edit_history(data)

        self._last_x = x
        self._last_y = y

    def __on_release(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        self._is_dragged = False

        if self.tool >= TOOL_PENCIL and self.tool <= TOOL_CIRC:
            img = self.parent.image_button.value
            dest = pyxel.image(img).data[
                self.edit_y : self.edit_y + 16, self.edit_x : self.edit_x + 16
            ]

            data = {}
            data["img"] = img
            data["pos"] = (self.edit_x, self.edit_y)
            data["before"] = dest.copy()

            index = self._overlay_canvas.data != -1
            dest[index] = self._overlay_canvas.data[index]
            self._overlay_canvas.clear()

            data["after"] = dest.copy()
            self.parent.add_edit_history(data)

    def __on_mouse_click(self, key, x, y):
        if key == pyxel.KEY_RIGHT_BUTTON:
            img = self.parent.image_button.value
            x = self.edit_x + (x - self.x) // 8
            y = self.edit_y + (y - self.y) // 8
            self.color = pyxel.image(img).data[y, x]

    def __on_drag(self, key, x, y, dx, dy):
        if key == pyxel.KEY_LEFT_BUTTON:
            x1 = self._press_x
            y1 = self._press_y
            x2 = (x - self.x) // 8
            y2 = (y - self.y) // 8

            if self.tool == TOOL_SELECT:
                x2 = min(max(x2, 0), 15)
                y2 = min(max(y2, 0), 15)
                self._select_x1, self._select_x2 = (x1, x2) if x1 < x2 else (x2, x1)
                self._select_y1, self._select_y2 = (y1, y2) if y1 < y2 else (y2, y1)
            elif self.tool == TOOL_PENCIL:
                if self._is_guide_mode:
                    self._overlay_canvas.clear()
                    self._overlay_canvas.line(x1, y1, x2, y2, self.color)
                else:
                    self._overlay_canvas.line(
                        self._last_x, self._last_y, x2, y2, self.color
                    )
            elif self.tool == TOOL_RECTB:
                self._overlay_canvas.clear()
                self._overlay_canvas.rectb(
                    x1, y1, x2, y2, self.color, self._is_guide_mode
                )
            elif self.tool == TOOL_RECT:
                self._overlay_canvas.clear()
                self._overlay_canvas.rect(
                    x1, y1, x2, y2, self.color, self._is_guide_mode
                )
            elif self.tool == TOOL_CIRCB:
                self._overlay_canvas.clear()
                self._overlay_canvas.circb(
                    x1, y1, x2, y2, self.color, self._is_guide_mode
                )
            elif self.tool == TOOL_CIRC:
                self._overlay_canvas.clear()
                self._overlay_canvas.circ(
                    x1, y1, x2, y2, self.color, self._is_guide_mode
                )

            self._last_x = x2
            self._last_y = y2

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

    def on_update(self):
        if self._is_dragged and not self._is_guide_mode and pyxel.btn(pyxel.KEY_SHIFT):
            self._is_guide_mode = True

            x1 = self._press_x
            y1 = self._press_y
            x2 = self._last_x
            y2 = self._last_y

            if self.tool == TOOL_PENCIL:
                self._overlay_canvas.clear()
                self._overlay_canvas.line(x1, y1, x2, y2, self.color)
            elif self.tool == TOOL_RECTB:
                self._overlay_canvas.clear()
                self._overlay_canvas.rectb(x1, y1, x2, y2, self.color, True)
            elif self.tool == TOOL_RECT:
                self._overlay_canvas.clear()
                self._overlay_canvas.rect(x1, y1, x2, y2, self.color, True)
            elif self.tool == TOOL_CIRCB:
                self._overlay_canvas.clear()
                self._overlay_canvas.circb(x1, y1, x2, y2, self.color, True)
            elif self.tool == TOOL_CIRC:
                self._overlay_canvas.clear()
                self._overlay_canvas.circ(x1, y1, x2, y2, self.color, True)

        if (
            self.tool == TOOL_SELECT
            and self._select_x1 >= 0
            and pyxel.btn(pyxel.KEY_CONTROL)
        ):
            if pyxel.btnp(pyxel.KEY_C):
                img = self.parent.image_button.value
                src = pyxel.image(img).data[
                    self.edit_y + self._select_y1 : self.edit_y + self._select_y2 + 1,
                    self.edit_x + self._select_x1 : self.edit_x + self._select_x2 + 1,
                ]

                self._copy_buffer = src.copy()
            elif self._copy_buffer is not None and pyxel.btnp(pyxel.KEY_V):
                x1 = self.edit_x + self._select_x1
                y1 = self.edit_y + self._select_y1

                height, width = self._copy_buffer.shape
                width -= max(self._select_x1 + width - 16, 0)
                height -= max(self._select_y1 + height - 16, 0)

                img = self.parent.image_button.value
                dest = pyxel.image(img).data[y1 : y1 + height, x1 : x1 + width]
                dest[:, :] = self._copy_buffer[:height, :width]

    def on_draw(self):
        for i in range(16):
            y = self.y + i * 8
            for j in range(16):
                x = self.x + j * 8

                if self._overlay_canvas.data[i, j] >= 0:
                    col = self._overlay_canvas.data[i, j]
                else:
                    data = pyxel.image(self.parent.image_button.value).data
                    col = data[self.edit_y + i, self.edit_x + j]

                pyxel.rect(x, y, x + 7, y + 7, col)

        pyxel.line(self.x, self.y + 63, self.x + 127, self.y + 63, 1)
        pyxel.line(self.x + 63, self.y, self.x + 63, self.y + 127, 1)

        if self.tool == TOOL_SELECT and self._select_x1 >= 0:
            x1 = self._select_x1 * 8 + 12
            y1 = self._select_y1 * 8 + 17
            x2 = self._select_x2 * 8 + 19
            y2 = self._select_y2 * 8 + 24

            pyxel.rectb(x1, y1, x2, y2, 0)
            pyxel.rectb(x1 + 1, y1 + 1, x2 - 1, y2 - 1, 15)
            pyxel.rectb(x1 + 2, y1 + 2, x2 - 2, y2 - 2, 0)

    def on_color_change(self, value):
        if self.tool == TOOL_SELECT:
            self.tool = TOOL_PENCIL

    def on_tool_change(self, value):
        if self.tool == TOOL_SELECT:
            self._select_x1 = -1


class PreviewWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 158, 17, 64, 128)

        self.preview_x = 0
        self.preview_y = 0

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
        self.preview_x = value * 8

    def __on_change_y(self, value):
        self.preview_y = value * 8

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            x -= self.x
            y -= self.y

            self.parent.edit_window.edit_x = self.preview_x + ((x - 4) // 8) * 8
            self.parent.edit_window.edit_y = self.preview_y + ((y - 4) // 8) * 8

            self.parent.edit_window.edit_x = min(
                max(self.parent.edit_window.edit_x, 0), 240
            )
            self.parent.edit_window.edit_y = min(
                max(self.parent.edit_window.edit_y, 0), 240
            )

        if key == pyxel.KEY_RIGHT_BUTTON:
            self._drag_offset_x = 0
            self._drag_offset_y = 0

    def __on_mouse_drag(self, key, x, y, dx, dy):
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

    def __on_draw(self):
        img = self.parent.image_button.value
        pyxel.blt(self.x, self.y, img, self.preview_x, self.preview_y, 64, 128)

        pyxel.clip(self.x - 1, self.y - 1, self.x + self.width, self.y + self.height)
        x = self.x + self.parent.edit_window.edit_x - self.preview_x
        y = self.y + self.parent.edit_window.edit_y - self.preview_y
        pyxel.rectb(x - 1, y - 1, x + 16, y + 16, 7)
        pyxel.clip()


class ImageEditor(Editor):
    def __init__(self, parent, is_tilemap_mode=False):
        super().__init__(parent, "image_editor.png")

        self._is_tilemap_mode = is_tilemap_mode

        self.color_button = EditorRadioButton(
            self, 12, 157, 8, 2, 1, is_color_button=True
        )
        self.color_button.value = 7

        self.tool_button = EditorRadioButton(self, 81, 161, 7, 1, 2)
        self.tool_button.value = TOOL_PENCIL

        self.image_button = EditorRadioButton(self, 191, 161, 3, 1, 3)
        self.edit_window = EditWindow(self)
        self.preview_window = PreviewWindow(self)

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)

    def __on_undo(self, data):
        img = data["img"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["before"]

        self.edit_window.edit_x = x
        self.edit_window.edit_y = y
        self.image_button.value = img

    def __on_redo(self, data):
        img = data["img"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["after"]

        self.edit_window.edit_x = x
        self.edit_window.edit_y = y
        self.image_button.value = img
