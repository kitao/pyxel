import pyxel
from pyxel.ui import Widget

from .editor_scroll_bar import EditorScrollBar
from .overlay_canvas import OverlayCanvas

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

        # self.parent.color_button.add_event_handler("change", self.on_color_change)
        # self.parent.tool_button.add_event_handler("change", self.on_tool_change)

        self._overlay_canvas = OverlayCanvas()

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

        if self.parent.tool == TOOL_SELECT:
            self._select_x1 = self._select_x2 = x
            self._select_y1 = self._select_y2 = y
        elif self.parent.tool >= TOOL_PENCIL and self.parent.tool <= TOOL_CIRC:
            self._overlay_canvas.pix(x, y, self.parent.color)
        elif self.parent.tool == TOOL_BUCKET:
            dest = pyxel.image(self.parent.image).data[
                self.edit_y : self.edit_y + 16, self.edit_x : self.edit_x + 16
            ]

            data = {}
            data["img"] = self.parent.image
            data["pos"] = (self.edit_x, self.edit_y)
            data["before"] = dest.copy()

            dest = pyxel.image(self.parent.image).data[
                self.edit_y : self.edit_y + 16, self.edit_x : self.edit_x + 16
            ]
            self._overlay_canvas.paint(x, y, self.parent.color, dest)

            data["after"] = dest.copy()
            self.parent.add_edit_history(data)

        self._last_x = x
        self._last_y = y

    def __on_release(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        self._is_dragged = False

        if self.parent.tool >= TOOL_PENCIL and self.parent.tool <= TOOL_CIRC:
            dest = pyxel.image(self.parent.image).data[
                self.edit_y : self.edit_y + 16, self.edit_x : self.edit_x + 16
            ]

            data = {}
            data["img"] = self.parent.image
            data["pos"] = (self.edit_x, self.edit_y)
            data["before"] = dest.copy()

            index = self._overlay_canvas.data != -1
            dest[index] = self._overlay_canvas.data[index]
            self._overlay_canvas.clear()

            data["after"] = dest.copy()
            self.parent.add_edit_history(data)

    def __on_mouse_click(self, key, x, y):
        if key == pyxel.KEY_RIGHT_BUTTON:
            x = self.edit_x + (x - self.x) // 8
            y = self.edit_y + (y - self.y) // 8
            self.parent.color = pyxel.image(self.parent.image).data[y, x]

    def __on_drag(self, key, x, y, dx, dy):
        if key == pyxel.KEY_LEFT_BUTTON:
            x1 = self._press_x
            y1 = self._press_y
            x2 = (x - self.x) // 8
            y2 = (y - self.y) // 8

            if self.parent.tool == TOOL_SELECT:
                x2 = min(max(x2, 0), 15)
                y2 = min(max(y2, 0), 15)
                self._select_x1, self._select_x2 = (x1, x2) if x1 < x2 else (x2, x1)
                self._select_y1, self._select_y2 = (y1, y2) if y1 < y2 else (y2, y1)
            elif self.parent.tool == TOOL_PENCIL:
                if self._is_guide_mode:
                    self._overlay_canvas.clear()
                    self._overlay_canvas.line(x1, y1, x2, y2, self.parent.color)
                else:
                    self._overlay_canvas.line(
                        self._last_x, self._last_y, x2, y2, self.parent.color
                    )
            elif self.parent.tool == TOOL_RECTB:
                self._overlay_canvas.clear()
                self._overlay_canvas.rectb(
                    x1, y1, x2, y2, self.parent.color, self._is_guide_mode
                )
            elif self.parent.tool == TOOL_RECT:
                self._overlay_canvas.clear()
                self._overlay_canvas.rect(
                    x1, y1, x2, y2, self.parent.color, self._is_guide_mode
                )
            elif self.parent.tool == TOOL_CIRCB:
                self._overlay_canvas.clear()
                self._overlay_canvas.circb(
                    x1, y1, x2, y2, self.parent.color, self._is_guide_mode
                )
            elif self.parent.tool == TOOL_CIRC:
                self._overlay_canvas.clear()
                self._overlay_canvas.circ(
                    x1, y1, x2, y2, self.parent.color, self._is_guide_mode
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

            if self.parent.tool == TOOL_PENCIL:
                self._overlay_canvas.clear()
                self._overlay_canvas.line(x1, y1, x2, y2, self.parent.color)
            elif self.parent.tool == TOOL_RECTB:
                self._overlay_canvas.clear()
                self._overlay_canvas.rectb(x1, y1, x2, y2, self.parent.color, True)
            elif self.parent.tool == TOOL_RECT:
                self._overlay_canvas.clear()
                self._overlay_canvas.rect(x1, y1, x2, y2, self.parent.color, True)
            elif self.parent.tool == TOOL_CIRCB:
                self._overlay_canvas.clear()
                self._overlay_canvas.circb(x1, y1, x2, y2, self.parent.color, True)
            elif self.parent.tool == TOOL_CIRC:
                self._overlay_canvas.clear()
                self._overlay_canvas.circ(x1, y1, x2, y2, self.parent.color, True)

        if (
            self.parent.tool == TOOL_SELECT
            and self._select_x1 >= 0
            and pyxel.btn(pyxel.KEY_CONTROL)
        ):
            if pyxel.btnp(pyxel.KEY_C):
                src = pyxel.image(self.parent.image).data[
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

                dest = pyxel.image(self.parent.image).data[
                    y1 : y1 + height, x1 : x1 + width
                ]
                dest[:, :] = self._copy_buffer[:height, :width]

    def on_draw(self):
        for i in range(16):
            y = self.y + i * 8
            for j in range(16):
                x = self.x + j * 8

                if self._overlay_canvas.data[i, j] >= 0:
                    col = self._overlay_canvas.data[i, j]
                else:
                    data = pyxel.image(self.parent.image).data
                    col = data[self.edit_y + i, self.edit_x + j]

                pyxel.rect(x, y, x + 7, y + 7, col)

        pyxel.line(self.x, self.y + 63, self.x + 127, self.y + 63, 1)
        pyxel.line(self.x + 63, self.y, self.x + 63, self.y + 127, 1)

        if self.parent.tool == TOOL_SELECT and self._select_x1 >= 0:
            x1 = self._select_x1 * 8 + 12
            y1 = self._select_y1 * 8 + 17
            x2 = self._select_x2 * 8 + 19
            y2 = self._select_y2 * 8 + 24

            pyxel.rectb(x1, y1, x2, y2, 0)
            pyxel.rectb(x1 + 1, y1 + 1, x2 - 1, y2 - 1, 15)
            pyxel.rectb(x1 + 2, y1 + 2, x2 - 2, y2 - 2, 0)

    def on_color_change(self, value):
        if self.parent.tool == TOOL_SELECT:
            self.parent.tool = TOOL_PENCIL

    def on_tool_change(self, value):
        if self.parent.tool == TOOL_SELECT:
            self._select_x1 = -1
