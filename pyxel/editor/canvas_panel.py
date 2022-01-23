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
    """
    Variables:
        color_var
        tool_var
        image_no_var
        canvas_var
        focus_x_var
        focus_y_var
        help_message_var

        tilemap_no_var
        tile_x_var
        tile_y_var
        tile_w_var
        tile_h_var
    """

    def __init__(self, parent):
        super().__init__(parent, 11, 16, 130, 130)
        if hasattr(parent, "tilemap_no_var"):
            self._is_tilemap_mode = True
            self.copy_var("tilemap_no_var", parent)
            self.copy_var("tile_x_var", parent)
            self.copy_var("tile_y_var", parent)
            self.copy_var("tile_w_var", parent)
            self.copy_var("tile_h_var", parent)
        else:
            self._is_tilemap_mode = False
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
        self._edit_canvas = (
            pyxel.Tilemap(16, 16, 0) if self._is_tilemap_mode else pyxel.Image(16, 16)
        )
        self.add_history = parent.add_history
        self.copy_var("color_var", parent)
        self.copy_var("tool_var", parent)
        self.copy_var("image_no_var", parent)
        self.copy_var("canvas_var", parent)
        self.copy_var("focus_x_var", parent)
        self.copy_var("focus_y_var", parent)
        self.copy_var("help_message_var", parent)

        # Initialize horizontal scroll bar
        self._h_scroll_bar = ScrollBar(
            self,
            0,
            129,
            width=130,
            scroll_amount=32,
            slider_amount=2,
            value=0,
        )
        self._h_scroll_bar.add_event_listener("change", self.__on_h_scroll_bar_change)
        self.add_var_event_listener("focus_x_var", "change", self.__on_focus_x_change)

        # Initialize vertical scroll bar
        self._v_scroll_bar = ScrollBar(
            self,
            129,
            0,
            height=130,
            scroll_amount=32,
            slider_amount=2,
            value=0,
        )
        self._v_scroll_bar.add_event_listener("change", self.__on_v_scroll_bar_change)
        self.add_var_event_listener("focus_y_var", "change", self.__on_focus_y_change)

        # Initialize event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_up", self.__on_mouse_up)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    def _screen_to_focus(self, x, y):
        x = min(max((x - self.x - 1) // 8, 0), 15)
        y = min(max((y - self.y - 1) // 8, 0), 15)
        return x, y

    def _add_pre_history(self):
        self._history_data = data = {}
        if self._is_tilemap_mode:
            data["tilemap_no"] = self.tilemap_no_var
        else:
            data["image_no"] = self.image_no_var
        data["focus_pos"] = (self.focus_x_var, self.focus_y_var)
        data["old_canvas"] = self.canvas_var.get_slice(
            self.focus_x_var * 8, self.focus_y_var * 8, 16, 16
        )

    def _add_post_history(self):
        data = self._history_data
        data["new_canvas"] = self.canvas_var.get_slice(
            self.focus_x_var * 8, self.focus_y_var * 8, 16, 16
        )
        if data["old_canvas"] != data["new_canvas"]:
            self.add_history(data)

    def _reset_edit_canvas(self):
        self._edit_canvas.blt(
            0,
            0,
            self.canvas_var,
            self.focus_x_var * 8,
            self.focus_y_var * 8,
            16,
            16,
        )
        if self._is_tilemap_mode:
            self._edit_canvas.refimg = self.canvas_var.refimg

    def _finish_edit_canvas(self):
        if not self._is_tilemap_mode:
            return
        for y in range(16):
            for x in range(16):
                if self._edit_canvas.pget(x, y) != (255, 255):
                    continue
                tile = (
                    self.tile_x_var + (x - self._press_x) % self.tile_w_var,
                    self.tile_y_var + (y - self._press_y) % self.tile_h_var,
                )
                self._edit_canvas.pset(x, y, tile)

    def __on_h_scroll_bar_change(self, value):
        self.focus_x_var = value

    def __on_v_scroll_bar_change(self, value):
        self.focus_y_var = value

    def __on_focus_x_change(self, value):
        self._h_scroll_bar.value_var = value

    def __on_focus_y_change(self, value):
        self._v_scroll_bar.value_var = value

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.MOUSE_BUTTON_RIGHT:
            x = self.focus_x_var * 8 + (x - self.x) // 8
            y = self.focus_y_var * 8 + (y - self.y) // 8
            if self._is_tilemap_mode:
                (self.tile_x_var, self.tile_y_var) = self.canvas_var.pget(x, y)
            else:
                self.color_var = self.canvas_var.pget(x, y)
            return
        elif key != pyxel.MOUSE_BUTTON_LEFT:
            return
        x, y = self._screen_to_focus(x, y)
        self._press_x = self._last_x = x
        self._press_y = self._last_y = y
        self._is_dragged = True
        self._is_assist_mode = False
        if self.tool_var == TOOL_SELECT:
            self._reset_edit_canvas()
            self._select_x1 = self._select_x2 = x
            self._select_y1 = self._select_y2 = y
        elif self.tool_var >= TOOL_PENCIL and self.tool_var <= TOOL_CIRC:
            self._reset_edit_canvas()
            self._edit_canvas.pset(x, y, self.color_var)
            self._finish_edit_canvas()
        elif self.tool_var == TOOL_BUCKET:
            self._add_pre_history()
            self._reset_edit_canvas()
            self._edit_canvas.fill(x, y, self.color_var)
            self._finish_edit_canvas()
            self.canvas_var.blt(
                self.focus_x_var * 8,
                self.focus_y_var * 8,
                self._edit_canvas,
                0,
                0,
                16,
                16,
            )
            self._add_post_history()

    def __on_mouse_up(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return
        self._is_dragged = False
        if TOOL_PENCIL <= self.tool_var <= TOOL_CIRC:
            self._add_pre_history()
            self.canvas_var.blt(
                self.focus_x_var * 8,
                self.focus_y_var * 8,
                self._edit_canvas,
                0,
                0,
                16,
                16,
            )
            self._add_post_history()

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            x1 = self._press_x
            y1 = self._press_y
            x2 = (x - self.x - 1) // 8
            y2 = (y - self.y - 1) // 8

            if TOOL_RECTB <= self.tool_var <= TOOL_CIRC and self._is_assist_mode:
                dx = x2 - x1
                dy = y2 - y1
                if abs(dx) > abs(dy):
                    y2 = y1 + abs(dx) * (1 if dy > 0 else -1)
                else:
                    x2 = x1 + abs(dy) * (1 if dx > 0 else -1)

            if self.tool_var == TOOL_SELECT:
                x2 = min(max(x2, 0), 15)
                y2 = min(max(y2, 0), 15)
                self._select_x1, self._select_x2 = (x1, x2) if x1 < x2 else (x2, x1)
                self._select_y1, self._select_y2 = (y1, y2) if y1 < y2 else (y2, y1)

            elif self.tool_var == TOOL_PENCIL:
                if self._is_assist_mode:
                    self._reset_edit_canvas()
                    self._edit_canvas.line(x1, y1, x2, y2, self.color_var)
                    self._finish_edit_canvas()
                else:
                    self._edit_canvas.line(
                        self._last_x, self._last_y, x2, y2, self.color_var
                    )
                    self._finish_edit_canvas()

            elif self.tool_var == TOOL_RECTB:
                self._reset_edit_canvas()
                self._edit_canvas.rectb2(
                    x1,
                    y1,
                    x2,
                    y2,
                    self.color_var,
                )
                self._finish_edit_canvas()

            elif self.tool_var == TOOL_RECT:
                self._reset_edit_canvas()
                self._edit_canvas.rect2(
                    x1,
                    y1,
                    x2,
                    y2,
                    self.color_var,
                )
                self._finish_edit_canvas()

            elif self.tool_var == TOOL_CIRCB:
                self._reset_edit_canvas()
                self._edit_canvas.ellipb2(x1, y1, x2, y2, self.color_var)
                self._finish_edit_canvas()

            elif self.tool_var == TOOL_CIRC:
                self._reset_edit_canvas()
                self._edit_canvas.ellip2(x1, y1, x2, y2, self.color_var)
                self._finish_edit_canvas()

            self._last_x = x2
            self._last_y = y2

        elif key == pyxel.MOUSE_BUTTON_RIGHT:
            self._drag_offset_x -= dx
            self._drag_offset_y -= dy
            if abs(self._drag_offset_x) >= 16:
                offset = self._drag_offset_x // 16
                self.focus_x_var += offset
                self._drag_offset_x -= offset * 16
            if abs(self._drag_offset_y) >= 16:
                offset = self._drag_offset_y // 16
                self.focus_y_var += offset
                self._drag_offset_y -= offset * 16

    def __on_mouse_hover(self, x, y):
        if self.tool_var == TOOL_SELECT:
            s = "COPY:CTRL+C PASTE:CTRL+V"
        elif self._is_dragged:
            s = "ASSIST:SHIFT"
        else:
            s = "PICK:R-CLICK VIEW:R-DRAG"
        x, y = self._screen_to_focus(x, y)
        x += self.focus_x_var * 8
        y += self.focus_y_var * 8
        self.help_message_var = s + f" ({x},{y})"

    def __on_update(self):
        if self._is_dragged and not self._is_assist_mode and pyxel.btn(pyxel.KEY_SHIFT):
            self._is_assist_mode = True
            self.__on_mouse_drag(
                pyxel.MOUSE_BUTTON_LEFT, pyxel.mouse_x, pyxel.mouse_y, 0, 0
            )

        if self.tool_var == TOOL_SELECT and (
            pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI)
        ):
            # Ctrl+C: Copy
            if pyxel.btnp(pyxel.KEY_C):
                self._copy_buffer = self.canvas_var.get_slice(
                    self.focus_x_var * 8 + self._select_x1,
                    self.focus_y_var * 8 + self._select_y1,
                    self._select_x2 - self._select_x1 + 1,
                    self._select_y2 - self._select_y1 + 1,
                )

            # Ctrl+V: Paste
            if self._copy_buffer is not None and pyxel.btnp(pyxel.KEY_V):
                self._add_pre_history()
                width = len(self._copy_buffer[0])
                height = len(self._copy_buffer)
                width -= max(self._select_x1 + width - 16, 0)
                height -= max(self._select_y1 + height - 16, 0)
                self.canvas_var.set_slice(
                    self.focus_x_var * 8 + self._select_x1,
                    self.focus_y_var * 8 + self._select_y1,
                    self._copy_buffer,
                )
                self._add_post_history()

        # Move tile focus
        if self._is_tilemap_mode and pyxel.btn(pyxel.KEY_SHIFT):
            if pyxel.btnp(pyxel.KEY_LEFT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
                self.tile_x_var -= 1
            if pyxel.btnp(pyxel.KEY_RIGHT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
                self.tile_x_var += 1
            if pyxel.btnp(pyxel.KEY_UP, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
                self.tile_y_var -= 1
            if pyxel.btnp(pyxel.KEY_DOWN, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
                self.tile_y_var += 1

        # Move target focus
        if not (
            pyxel.btn(pyxel.KEY_SHIFT)
            or pyxel.btn(pyxel.KEY_CTRL)
            or pyxel.btn(pyxel.KEY_ALT)
            or pyxel.btn(pyxel.KEY_GUI)
        ):
            if pyxel.btnp(pyxel.KEY_LEFT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
                self.focus_x_var -= 1
            if pyxel.btnp(pyxel.KEY_RIGHT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
                self.focus_x_var += 1
            if pyxel.btnp(pyxel.KEY_UP, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
                self.focus_y_var -= 1
            if pyxel.btnp(pyxel.KEY_DOWN, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
                self.focus_y_var += 1

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        # Draw edit panel
        canvas, offset_x, offset_y = (
            (self._edit_canvas, 0, 0)
            if self._is_dragged
            else (self.canvas_var, self.focus_x_var * 8, self.focus_y_var * 8)
        )
        if self._is_tilemap_mode:
            pyxel.bltm(
                self.x + 1,
                self.y + 1,
                canvas,
                offset_x * 8,
                offset_y * 8,
                128,
                128,
            )
        else:
            for yi in range(16):
                for xi in range(16):
                    pyxel.rect(
                        self.x + xi * 8 + 1,
                        self.y + yi * 8 + 1,
                        8,
                        8,
                        canvas.pget(offset_x + xi, offset_y + yi),
                    )
        pyxel.line(
            self.x + 1, self.y + 64, self.x + 128, self.y + 64, WIDGET_PANEL_COLOR
        )
        pyxel.line(
            self.x + 64, self.y + 1, self.x + 64, self.y + 128, WIDGET_PANEL_COLOR
        )

        # Draw selection area
        if self.tool_var == TOOL_SELECT and self._select_x1 >= 0:
            x = self._select_x1 * 8 + 12
            y = self._select_y1 * 8 + 17
            w = self._select_x2 * 8 - x + 20
            h = self._select_y2 * 8 - y + 25
            pyxel.clip(self.x + 1, self.y + 1, self.x + 128, self.y + 128)
            pyxel.rectb(x, y, w, h, PANEL_SELECT_FRAME_COLOR)
            pyxel.rectb(x + 1, y + 1, w - 2, h - 2, PANEL_SELECT_BORDER_COLOR)
            pyxel.rectb(x - 1, y - 1, w + 2, h + 2, PANEL_SELECT_BORDER_COLOR)
            pyxel.clip()
