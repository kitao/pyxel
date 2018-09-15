import numpy as np

import pyxel
from pyxel.constants import RENDERER_IMAGE_COUNT

from .radio_button import RadioButton
from .screen import Screen
from .scroll_bar import ScrollBar
from .widget import Widget

TOOL_SELECT = 0
TOOL_PENCIL = 1
TOOL_RECTB = 2
TOOL_RECT = 3
TOOL_CIRCB = 4
TOOL_CIRC = 5
TOOL_BUCKET = 6


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

        self._is_dragged = False
        self._is_guide_mode = False

        self._h_scroll_bar = ScrollBar(self, 11, 145, 130, 7, 2, 32)
        self._h_scroll_bar.add_event_handler("change", self.on_change_x)

        self._v_scroll_bar = ScrollBar(self, 140, 16, 7, 130, 2, 32)
        self._v_scroll_bar.add_event_handler("change", self.on_change_y)

        self.add_event_handler("press", self.on_press)
        self.add_event_handler("release", self.on_release)
        self.add_event_handler("click", self.on_click)
        self.add_event_handler("drag", self.on_drag)
        self.add_event_handler("update", self.on_update)
        self.add_event_handler("draw", self.on_draw)

        self.overlay = np.ndarray((16, 16), np.int8)
        self.clear_overlay()

    def clear_overlay(self):
        self.overlay[:, :] = -1

    def draw_line_on_overlay(self, x1, y1, x2, y2, val):
        if x1 == x2 and y1 == y2:
            if x1 >= 0 and x1 < 16 and y1 >= 0 and y1 < 16:
                self.overlay[y1, x1] = val
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
                    self.overlay[y, x] = val
        else:
            if dy < 0:
                x1, y1 = x2, y2
                dx, dy = -dx, -dy

            for i in range(dy + 1):
                x = int(x1 + i * dx / dy + 0.5)
                y = y1 + i

                if x >= 0 and x < 16 and y >= 0 and y < 16:
                    self.overlay[y, x] = val

    def draw_rectb_on_overlay(self, x1, y1, x2, y2, val):
        self.overlay[y1 : y1 + 1, x1 : x2 + 1] = val
        self.overlay[y2 : y2 + 1, x1 : x2 + 1] = val
        self.overlay[y1 : y2 + 1, x1 : x1 + 1] = val
        self.overlay[y1 : y2 + 1, x2 : x2 + 1] = val

    def draw_rect_on_overlay(self, x1, y1, x2, y2, val):
        self.overlay[y1 : y2 + 1, x1 : x2 + 1] = val

    def draw_circb_on_overlay(self, x1, y1, x2, y2, val):
        pass

    def draw_circ_on_overlay(self, x1, y1, x2, y2, val):
        pass

    def on_change_x(self, value):
        self.edit_x = value * 8

    def on_change_y(self, value):
        self.edit_y = value * 8

    def on_press(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        x //= 8
        y //= 8

        self._press_x = x
        self._press_y = y

        self._is_dragged = True
        self._is_guide_mode = False

        tool = self.parent.tool_button.value

        if tool >= TOOL_PENCIL and tool <= TOOL_CIRC:
            self.overlay[y, x] = self.parent.color_button.value
        elif tool == TOOL_BUCKET:
            pass

        self._last_x = x
        self._last_y = y

    def on_release(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        self._is_dragged = False

        img = self.parent.image_button.value
        dest = pyxel.image(img).data[
            self.edit_y : self.edit_y + 16, self.edit_x : self.edit_x + 16
        ]

        data = {}
        data["img"] = img
        data["pos"] = (self.edit_x, self.edit_y)
        data["before"] = dest.copy()

        index = self.overlay != -1
        dest[index] = self.overlay[index]
        self.clear_overlay()

        data["after"] = dest.copy()
        self.parent.add_edit_history(data)

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

            tool = self.parent.tool_button.value
            col = self.parent.color_button.value

            x1, y1 = self._press_x, self._press_y
            x2, y2 = min(max(x, 0), 15), min(max(y, 0), 15)

            if x1 > x2:
                x1, x2 = x2, x1

            if y1 > y2:
                y1, y2 = y2, y1

            if tool == TOOL_SELECT:
                pass
            elif tool == TOOL_PENCIL:
                if self._is_guide_mode:
                    self.clear_overlay()
                    self.draw_line_on_overlay(self._press_x, self._press_y, x, y, col)
                else:
                    self.draw_line_on_overlay(self._last_x, self._last_y, x, y, col)
            elif tool == TOOL_RECTB:
                self.clear_overlay()
                self.draw_rectb_on_overlay(x1, y1, x2, y2, col)
            elif tool == TOOL_RECT:
                self.draw_rect_on_overlay(x1, y1, x2, y2, col)
            elif tool == TOOL_CIRCB:
                self.draw_circb_on_overlay(x1, y1, x2, y2, col)
            elif tool == TOOL_CIRC:
                self.draw_circ_on_overlay(x1, y1, x2, y2, col)

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

    def on_update(self):
        if self._is_dragged and not self._is_guide_mode and pyxel.btn(pyxel.KEY_SHIFT):
            self._is_guide_mode = True
            tool = self.parent.tool_button.value

            if tool == TOOL_PENCIL:
                self.clear_overlay()
                self.draw_line_on_overlay(
                    self._press_x,
                    self._press_y,
                    self._last_x,
                    self._last_y,
                    self.parent.color_button.value,
                )

    def on_draw(self):
        for i in range(16):
            y = self.y + i * 8
            for j in range(16):
                x = self.x + j * 8

                if self.overlay[i, j] >= 0:
                    col = self.overlay[i, j]
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
        self._h_scroll_bar.add_event_handler("change", self.on_change_x)

        self._v_scroll_bar = ScrollBar(self, 222, 16, 7, 130, 16, 32)
        self._v_scroll_bar.add_event_handler("change", self.on_change_y)

        self.add_event_handler("press", self.on_press)
        self.add_event_handler("drag", self.on_drag)
        self.add_event_handler("draw", self.on_draw)

    def on_change_x(self, value):
        self.preview_x = value * 8

    def on_change_y(self, value):
        self.preview_y = value * 8

    def on_press(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
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

        pyxel.clip(self.x - 1, self.y - 1, self.x + self.width, self.y + self.height)
        x = self.x + self.parent.edit_window.edit_x - self.preview_x
        y = self.y + self.parent.edit_window.edit_y - self.preview_y
        pyxel.rectb(x - 1, y - 1, x + 16, y + 16, 7)
        pyxel.clip()


class ImageEditor(Screen):
    def __init__(self, parent, is_tilemap_mode=False):
        super().__init__(parent, "image_editor.png")

        def on_draw():
            widget = self.color_button
            x = widget.x + (widget.value % 8) * 8
            y = widget.y + (widget.value // 8) * 8
            col = 7 if widget.value < 6 else 0
            pyxel.text(x + 2, y + 1, "+", col)

        self.color_button = RadioButton(self, 12, 157, 8, 2, 8)
        self.color_button.remove_event_handler("draw", self.color_button.on_draw)
        self.color_button.add_event_handler("draw", on_draw)
        self.color_button.value = 7

        self.tool_button = RadioButton(self, 81, 161, 7, 1, 9)
        self.tool_button.value = TOOL_PENCIL
        self.image_button = RadioButton(self, 191, 161, 3, 1, 10)
        self.edit_window = EditWindow(self)
        self.preview_window = PreviewWindow(self)

        self.add_event_handler("undo", self.on_undo)
        self.add_event_handler("redo", self.on_redo)

    def on_undo(self, data):
        img = data["img"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["before"]

        self.edit_window.edit_x = x
        self.edit_window.edit_y = y
        self.image_button.value = img

    def on_redo(self, data):
        img = data["img"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["after"]

        self.edit_window.edit_x = x
        self.edit_window.edit_y = y
        self.image_button.value = img
