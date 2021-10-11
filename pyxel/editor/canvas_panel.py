import pyxel

from .settings import (
    PANEL_SELECT_BORDER_COLOR,
    PANEL_SELECT_FRAME_COLOR,
    TOOL_BUCKET,
    TOOL_CIRC,
    TOOL_CIRCB,
    TOOL_PENCIL,
    TOOL_RECT,
    TOOL_RECTB,
    TOOL_SELECT,
)
from .widgets import ScrollBar, Widget
from .widgets.settings import WIDGET_HOLD_TIME, WIDGET_PANEL_COLOR, WIDGET_REPEAT_TIME


class CanvasPanel(Widget):
    def __init__(self, parent, *, is_tilemap_mode):
        super().__init__(parent, 11, 16, 130, 130)

        self._is_tilemap_mode = is_tilemap_mode
        self._history_data = None
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
        self._is_assist_mode = False
        self._temp_canvas = (
            pyxel.Tilemap(16, 16, 0) if is_tilemap_mode else pyxel.Image(16, 16)
        )
        self._h_scroll_bar = ScrollBar(
            self, 11, 145, 130, ScrollBar.HORIZONTAL, 32, 2, 0
        )
        self._v_scroll_bar = ScrollBar(self, 140, 16, 130, ScrollBar.VERTICAL, 32, 2, 0)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_up", self.__on_mouse_up)
        self.add_event_handler("mouse_click", self.__on_mouse_click)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
        self._h_scroll_bar.add_event_handler("change", self.__on_h_scroll_bar_change)
        self._v_scroll_bar.add_event_handler("change", self.__on_v_scroll_bar_change)

    def _add_pre_history(self, canvas):
        self._history_data = data = {}

        if self._is_tilemap_mode:
            data["tilemap"] = self.parent.tilemap_no
        else:
            data["image"] = self.parent.image_no

        data["pos"] = (self.parent.edit_x, self.parent.edit_y)
        data["before"] = canvas.copy()

    def _add_post_history(self, canvas):
        data = self._history_data
        data["after"] = canvas.copy()

        if data["before"] != data["after"]:
            self.parent.add_history(data)

    def _screen_to_view(self, x, y):
        x = min(max((x - self.x - 1) // 8, 0), 15)
        y = min(max((y - self.y - 1) // 8, 0), 15)
        return x, y

    def _reset_temp_canvas(self):
        self._temp_canvas.blt(
            0,
            0,
            self.parent.canvas,
            self.parent.edit_x,
            self.parent.edit_y,
            16,
            16,
        )

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        x, y = self._screen_to_view(x, y)

        self._press_x = x
        self._press_y = y

        self._is_dragged = True
        self._is_assist_mode = False

        if self.parent.tool == TOOL_SELECT:
            self._select_x1 = self._select_x2 = x
            self._select_y1 = self._select_y2 = y
        elif self.parent.tool >= TOOL_PENCIL and self.parent.tool <= TOOL_CIRC:
            self._reset_temp_canvas()
            self._temp_canvas.pset(x, y, self.parent.color)
        elif self.parent.tool == TOOL_BUCKET:
            self._reset_temp_canvas()

            self._add_pre_history(
                self.parent.canvas.get_slice(
                    self.parent.edit_x, self.parent.edit_y, 16, 16
                )
            )

            self.canvas.fill(x, y, self.parent.color)

            self.canvas.blt(
                self.parent.edit_x,
                self.parent.edit_y,
                self._temp_canvas,
                0,
                0,
                16,
                16,
            )

            self._add_post_history(
                self.parent.canvas.get_slice(
                    self.parent.edit_x, self.parent.edit_y, 16, 16
                )
            )

        self._last_x = x
        self._last_y = y

    def __on_mouse_up(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return

        self._is_dragged = False

        if TOOL_PENCIL <= self.parent.tool <= TOOL_CIRC:
            self._add_pre_history(
                self.parent.canvas.get_slice(
                    self.parent.edit_x, self.parent.edit_y, 16, 16
                )
            )

            self.parent.canvas.blt(
                self.parent.edit_x,
                self.parent.edit_y,
                self._temp_canvas,
                0,
                0,
                16,
                16,
            )

            self._add_post_history(
                self.parent.canvas.get_slice(
                    self.parent.edit_x, self.parent.edit_y, 16, 16
                )
            )

    def __on_mouse_click(self, key, x, y):
        if key == pyxel.MOUSE_BUTTON_RIGHT:
            x = self.parent.edit_x + (x - self.x) // 8
            y = self.parent.edit_y + (y - self.y) // 8

            if self._is_tilemap_mode:
                self.parent.color = pyxel.tilemap(self.parent.tilemap).data[y][x]
            else:
                self.parent.color = pyxel.image(self.parent.image).data[y][x]

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            x1 = self._press_x
            y1 = self._press_y
            x2 = (x - self.x - 1) // 8
            y2 = (y - self.y - 1) // 8

            if self.parent.tool == TOOL_SELECT:
                x2 = min(max(x2, 0), 15)
                y2 = min(max(y2, 0), 15)
                self._select_x1, self._select_x2 = (x1, x2) if x1 < x2 else (x2, x1)
                self._select_y1, self._select_y2 = (y1, y2) if y1 < y2 else (y2, y1)
            elif self.parent.tool == TOOL_PENCIL:
                if self._is_assist_mode:
                    self._overlay_canvas.clear()
                    self._overlay_canvas.line(x1, y1, x2, y2, self.parent.color)
                else:
                    self._temp_canvas.line(
                        self._last_x, self._last_y, x2, y2, self.parent.color
                    )
            elif self.parent.tool == TOOL_RECTB:
                self._reset_temp_canvas()
                self._temp_canvas.rectb2(
                    x1,
                    y1,
                    x2,
                    y2,
                    self.parent.color,
                )
            elif self.parent.tool == TOOL_RECT:
                self._reset_temp_canvas()
                self._temp_canvas.rect2(
                    x1,
                    y1,
                    x2,
                    y2,
                    self.parent.color,
                )
            elif self.parent.tool == TOOL_CIRCB:
                self._reset_temp_canvas()
                self._temp_canvas.ellipb(x1, y1, x2, y2, self.parent.color)
            elif self.parent.tool == TOOL_CIRC:
                self._reset_temp_canvas()
                self._temp_canvas.ellip(x1, y1, x2, y2, self.parent.color)

            self._last_x = x2
            self._last_y = y2

        elif key == pyxel.MOUSE_BUTTON_RIGHT:
            self._drag_offset_x -= dx
            self._drag_offset_y -= dy

            if abs(self._drag_offset_x) >= 16:
                offset = self._drag_offset_x // 16
                self.parent.edit_x += offset * 8
                self._drag_offset_x -= offset * 16

            if abs(self._drag_offset_y) >= 16:
                offset = self._drag_offset_y // 16
                self.parent.edit_y += offset * 8
                self._drag_offset_y -= offset * 16

            self.parent.edit_x = min(max(self.parent.edit_x, 0), 240)
            self.parent.edit_y = min(max(self.parent.edit_y, 0), 240)

    def __on_mouse_hover(self, x, y):
        if self.parent.tool == TOOL_SELECT:
            s = "COPY:CTRL+C PASTE:CTRL+V"
        elif self._is_dragged:
            s = "ASSIST:SHIFT"
        else:
            s = "PICK:R-CLICK VIEW:R-DRAG"

        x, y = self._screen_to_view(x, y)
        x += self.parent.edit_x
        y += self.parent.edit_y
        self.parent.help_message = s + " ({},{})".format(x, y)

    def __on_update(self):
        if self._is_dragged and not self._is_assist_mode and pyxel.btn(pyxel.KEY_SHIFT):
            self._is_assist_mode = True

        if (
            self.parent.tool == TOOL_SELECT
            and self._select_x1 >= 0
            and (pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI))
        ):
            if pyxel.btnp(pyxel.KEY_C):
                self._copy_buffer = self.parent.canvas.get_slice(
                    self.parent.edit_x + self._select_x1,
                    self.parent.edit_y + self._select_y1,
                    self._select_x2 - self._select_x1 + 1,
                    self._select_y2 - self._select_y1 + 1,
                )
            elif self._copy_buffer is not None and pyxel.btnp(pyxel.KEY_V):
                width, height = len(self._copy_buffer[0]), len(self._copy_buffer)
                width -= max(self._select_x1 + width - 16, 0)
                height -= max(self._select_y1 + height - 16, 0)

                self._add_pre_history(
                    self.parent.canvas.get_slice(
                        self.parent.edit_x, self.parent.edit_y, 16, 16
                    )
                )

                self.parent.canvas.set_slice(
                    self.parent.edit_x + self._select_x1,
                    self.parent.edit_y + self._select_y1,
                    width,
                    height,
                    self._copy_buffer,
                )

                self._add_post_history(
                    self.parent.canvas.get_slice(
                        self.parent.edit_x, self.parent.edit_y, 16, 16
                    )
                )

        if (
            pyxel.btn(pyxel.KEY_SHIFT)
            or pyxel.btn(pyxel.KEY_CTRL)
            or pyxel.btn(pyxel.KEY_ALT)
            or pyxel.btn(pyxel.KEY_GUI)
        ):
            return

        if pyxel.btnp(pyxel.KEY_LEFT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.parent.edit_x -= 8

        if pyxel.btnp(pyxel.KEY_RIGHT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.parent.edit_x += 8

        if pyxel.btnp(pyxel.KEY_UP, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.parent.edit_y -= 8

        if pyxel.btnp(pyxel.KEY_DOWN, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.parent.edit_y += 8

        self.parent.edit_x = min(max(self.parent.edit_x, 0), 240)
        self.parent.edit_y = min(max(self.parent.edit_y, 0), 240)

        self._h_scroll_bar.value = self.parent.edit_x // 8
        self._v_scroll_bar.value = self.parent.edit_y // 8

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        if self._is_tilemap_mode:
            pass
            """
            pyxel.bltm(
                self.x + 1,
                self.y + 1,
                self.parent.tilemap,
                self.parent.edit_x,
                self.parent.edit_y,
                16,
                16,
            )

            for i in range(16):
                y = self.y + i * 8 + 1
                for j in range(16):
                    x = self.x + j * 8 + 1

                    val = self._overlay_canvas.data[i][j]
                    if val != OverlayCanvas.COLOR_NONE:
                        sx = (val % 32) * 8
                        sy = (val // 32) * 8
                        pyxel.blt(x, y, self.parent.image, sx, sy, 8, 8)
            """
        else:
            canvas, offset_x, offset_y = (
                (self._temp_canvas, 0, 0)
                if self._is_dragged
                else (self.parent.canvas, self.parent.edit_x, self.parent.edit_y)
            )

            for i in range(16):
                y = self.y + i * 8 + 1
                for j in range(16):
                    x = self.x + j * 8 + 1
                    pyxel.rect(x, y, 8, 8, canvas.pget(offset_x + j, offset_y + i))

        pyxel.line(
            self.x + 1, self.y + 64, self.x + 128, self.y + 64, WIDGET_PANEL_COLOR
        )
        pyxel.line(
            self.x + 64, self.y + 1, self.x + 64, self.y + 128, WIDGET_PANEL_COLOR
        )

        if self.parent.tool == TOOL_SELECT and self._select_x1 >= 0:
            pyxel.clip(self.x + 1, self.y + 1, self.x + 128, self.y + 128)

            x = self._select_x1 * 8 + 12
            y = self._select_y1 * 8 + 17
            w = self._select_x2 * 8 - x + 20
            h = self._select_y2 * 8 - y + 25

            pyxel.rectb(x, y, w, h, PANEL_SELECT_FRAME_COLOR)
            pyxel.rectb(x + 1, y + 1, w - 2, h - 2, PANEL_SELECT_BORDER_COLOR)
            pyxel.rectb(x - 1, y - 1, w + 2, h + 2, PANEL_SELECT_BORDER_COLOR)

            pyxel.clip()

    def __on_h_scroll_bar_change(self, value):
        self.parent.edit_x = value * 8

    def __on_v_scroll_bar_change(self, value):
        self.parent.edit_y = value * 8

    @staticmethod
    def _adjust_region(x1, y1, x2, y2, is_assist_mode):
        if is_assist_mode:
            dx = x2 - x1
            dy = y2 - y1

            if abs(dx) > abs(dy):
                y2 = y1 + abs(dx) * (1 if dy > 0 else -1)
            else:
                x2 = x1 + abs(dy) * (1 if dx > 0 else -1)

        x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
        y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)

        return x1, y1, x2, y2
