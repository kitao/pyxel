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
    clamp,
    is_modifier_pressed,
)
from .widgets import ScrollBar, Widget
from .widgets.settings import WIDGET_HOLD_TIME, WIDGET_PANEL_COLOR, WIDGET_REPEAT_TIME

# Sentinel tile that marks cells touched by tilemap-mode drawing primitives.
_EMPTY_TILE = (255, 255)

class CanvasPanel(Widget):
    # Variables:
    #   color_var
    #   tool_var
    #   image_index_var
    #   canvas_var
    #   focus_x_var
    #   focus_y_var
    #   help_message_var
    #
    #   tilemap_index_var
    #   tile_x_var
    #   tile_y_var
    #   tile_w_var
    #   tile_h_var
    #
    # Events:
    #   none

    def __init__(self, parent):
        super().__init__(parent, 11, 16, 130, 130)

        if hasattr(parent, "tilemap_index_var"):
            self._is_tilemap_mode = True
            self.copy_var("tilemap_index_var", parent)
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
        self._canvas_buffer = None
        self._bank_buffer = None
        self._is_dragged = False
        self._is_assist_mode = False
        self._edit_canvas = (
            pyxel.Tilemap(16, 16, 0) if self._is_tilemap_mode else pyxel.Image(16, 16)
        )
        self.add_history = parent.add_history
        self.copy_var("color_var", parent)
        self.copy_var("tool_var", parent)
        self.copy_var("image_index_var", parent)
        self.copy_var("canvas_var", parent)
        self.copy_var("focus_x_var", parent)
        self.copy_var("focus_y_var", parent)
        self.copy_var("help_message_var", parent)
        self.copy_var("secondary_color_var", parent)
        self._drag_button = None

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

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_up", self.__on_mouse_up)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("mouse_wheel", self.__on_mouse_wheel)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    # Helpers

    def _screen_to_focus(self, x, y):
        x = clamp((x - self.x - 1) // 8, 0, 15)
        y = clamp((y - self.y - 1) // 8, 0, 15)
        return x, y

    def _selection_rect(self):
        x = self.focus_x_var * 8 + self._select_x1
        y = self.focus_y_var * 8 + self._select_y1
        w = self._select_x2 - self._select_x1 + 1
        h = self._select_y2 - self._select_y1 + 1
        return x, y, w, h

    def _add_pre_history(self, *, bank_copy=False):
        data = {}
        self._history_data = data

        if bank_copy:
            if self._is_tilemap_mode:
                data["tilemap_index"] = self.tilemap_index_var
                data["old_imgsrc"] = self.canvas_var.imgsrc
            else:
                data["image_index"] = self.image_index_var
            data["old_data"] = self.canvas_var.get_slice(0, 0, 256, 256)
        else:
            if self._is_tilemap_mode:
                data["tilemap_index"] = self.tilemap_index_var
            else:
                data["image_index"] = self.image_index_var

            data["focus_pos"] = (self.focus_x_var, self.focus_y_var)
            data["old_canvas"] = self.canvas_var.get_slice(
                self.focus_x_var * 8, self.focus_y_var * 8, 16, 16
            )

    def _add_post_history(self, *, bank_copy=False):
        data = self._history_data

        if bank_copy:
            data["new_data"] = self.canvas_var.get_slice(0, 0, 256, 256)
            if self._is_tilemap_mode:
                data["new_imgsrc"] = self.canvas_var.imgsrc
                changed = (
                    data["new_data"] != data["old_data"]
                    or data["new_imgsrc"] != data["old_imgsrc"]
                )
            else:
                changed = data["new_data"] != data["old_data"]
            if changed:
                self.add_history(data)
        else:
            data["new_canvas"] = self.canvas_var.get_slice(
                self.focus_x_var * 8, self.focus_y_var * 8, 16, 16
            )

            if data["new_canvas"] != data["old_canvas"]:
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
            self._edit_canvas.imgsrc = self.canvas_var.imgsrc

    def _finish_edit_canvas(self):
        if not self._is_tilemap_mode:
            return

        for y in range(16):
            for x in range(16):
                if self._edit_canvas.pget(x, y) != _EMPTY_TILE:
                    continue
                tile = (
                    self.tile_x_var + (x - self._press_x) % self.tile_w_var,
                    self.tile_y_var + (y - self._press_y) % self.tile_h_var,
                )
                self._edit_canvas.pset(x, y, tile)

    def copy_canvas(self):
        x, y, w, h = self._selection_rect()
        self._canvas_buffer = self.canvas_var.get_slice(x, y, w, h)

    def cut_canvas(self):
        self.copy_canvas()
        x, y, w, h = self._selection_rect()
        self._add_pre_history()
        self.canvas_var.rect(x, y, w, h, (0, 0) if self._is_tilemap_mode else 0)
        self._add_post_history()

    def paste_canvas(self):
        if self._canvas_buffer is None:
            return
        self._add_pre_history()
        width = len(self._canvas_buffer[0])
        height = len(self._canvas_buffer)
        width -= max(self._select_x1 + width - 16, 0)
        height -= max(self._select_y1 + height - 16, 0)
        clipped = [row[:width] for row in self._canvas_buffer[:height]]
        self.canvas_var.set_slice(
            self.focus_x_var * 8 + self._select_x1,
            self.focus_y_var * 8 + self._select_y1,
            clipped,
            self.secondary_color_var
        )
        self._add_post_history()

    def clear_canvas(self, x, y, w, h):
        self.canvas_var.rect(
            x,
            y,
            w,
            h,
            (self.secondary_color_var, self.secondary_color_var)
            if self._is_tilemap_mode
            else self.secondary_color_var,
        )

    # Event handlers

    def __on_h_scroll_bar_change(self, value):
        self.focus_x_var = value

    def __on_v_scroll_bar_change(self, value):
        self.focus_y_var = value

    def __on_focus_x_change(self, value):
        self._h_scroll_bar.value_var = value

    def __on_focus_y_change(self, value):
        self._v_scroll_bar.value_var = value

    def __on_mouse_down(self, key, x, y):
        # Right click picks the current color or tile.
        if key == pyxel.MOUSE_BUTTON_MIDDLE:
            x, y = self._screen_to_focus(x, y)
            x += self.focus_x_var * 8
            y += self.focus_y_var * 8
            if self._is_tilemap_mode:
                (self.tile_x_var, self.tile_y_var) = self.canvas_var.pget(x, y)
            else:
                color = self.canvas_var.pget(x, y)
                if pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI):
                    self.secondary_color_var = color
                else:
                    self.color_var = color
                    
            self._drag_offset_x = 0
            self._drag_offset_y = 0
            self._drag_button = key
            return

        if key not in (pyxel.MOUSE_BUTTON_LEFT, pyxel.MOUSE_BUTTON_RIGHT):
            return

        x, y = self._screen_to_focus(x, y)
        self._press_x = self._last_x = x
        self._press_y = self._last_y = y
        self._is_dragged = True
        self._is_assist_mode = False
        self._drag_button = key

        drawing_color = (
            self.secondary_color_var
            if key == pyxel.MOUSE_BUTTON_RIGHT
            else self.color_var
        )

        # SELECT: begin selection
        if self.tool_var == TOOL_SELECT:
            if key == pyxel.MOUSE_BUTTON_LEFT:
                self._reset_edit_canvas()
                self._select_x1 = self._select_x2 = x
                self._select_y1 = self._select_y2 = y
            else:
                self._is_dragged = False
                self._drag_button = None
            return

        # PENCIL/RECTB/RECT/CIRCB/CIRC: place initial dot
        elif TOOL_PENCIL <= self.tool_var <= TOOL_CIRC:
            self._reset_edit_canvas()
            self._edit_canvas.pset(x, y, drawing_color)
            self._finish_edit_canvas()

        # BUCKET: flood fill and commit immediately
        elif self.tool_var == TOOL_BUCKET:
            self._add_pre_history()
            self._reset_edit_canvas()
            self._edit_canvas.fill(x, y, drawing_color)
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
        if key != self._drag_button:
            return

        self._drag_button = None
        if key == pyxel.MOUSE_BUTTON_MIDDLE:
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
                16
            )
            self._add_post_history()

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_BUTTON_MIDDLE:
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
            return

        if key == self._drag_button:
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

            drawing_color = (
                self.secondary_color_var
                if key == pyxel.MOUSE_BUTTON_RIGHT
                else self.color_var
            )

            # SELECT: update selection rectangle
            if self.tool_var == TOOL_SELECT:
                x2 = clamp(x2, 0, 15)
                y2 = clamp(y2, 0, 15)
                self._select_x1, self._select_x2 = (x1, x2) if x1 < x2 else (x2, x1)
                self._select_y1, self._select_y2 = (y1, y2) if y1 < y2 else (y2, y1)

            # PENCIL: freehand or assisted straight line
            elif self.tool_var == TOOL_PENCIL:
                if self._is_assist_mode:
                    self._reset_edit_canvas()
                    self._edit_canvas.line(x1, y1, x2, y2, drawing_color)
                    self._finish_edit_canvas()
                else:
                    self._edit_canvas.line(
                        self._last_x, self._last_y, x2, y2, drawing_color
                    )
                    self._finish_edit_canvas()

            # RECTB: outlined rectangle
            elif self.tool_var == TOOL_RECTB:
                self._reset_edit_canvas()
                self._edit_canvas.rectb2(x1, y1, x2, y2, drawing_color)
                self._finish_edit_canvas()

            # RECT: filled rectangle
            elif self.tool_var == TOOL_RECT:
                self._reset_edit_canvas()
                self._edit_canvas.rect2(x1, y1, x2, y2, drawing_color)
                self._finish_edit_canvas()

            # CIRCB: outlined ellipse
            elif self.tool_var == TOOL_CIRCB:
                self._reset_edit_canvas()
                self._edit_canvas.ellib2(x1, y1, x2, y2, drawing_color)
                self._finish_edit_canvas()

            # CIRC: filled ellipse
            elif self.tool_var == TOOL_CIRC:
                self._reset_edit_canvas()
                self._edit_canvas.elli2(x1, y1, x2, y2, drawing_color)
                self._finish_edit_canvas()

            self._last_x = x2
            self._last_y = y2

    def __on_mouse_hover(self, x, y):
        if self.tool_var == TOOL_SELECT:
            s = "COPY:CTRL+C/X/V FLIP:H/V"
        elif self._is_dragged:
            s = "ASSIST:SHIFT"
        else:
            s = "PICK:M-CLICK VIEW:M-DRAG"

        x, y = self._screen_to_focus(x, y)
        x += self.focus_x_var * 8
        y += self.focus_y_var * 8
        self.help_message_var = s + f" ({x},{y})"

    def __on_mouse_wheel(self, dy):
        self.tool_var = (self.tool_var + dy) % 7

    def __on_update(self):
        if self._is_dragged and not self._is_assist_mode and pyxel.btn(pyxel.KEY_SHIFT):
            self._is_assist_mode = True
            self.__on_mouse_drag(
                pyxel.MOUSE_BUTTON_LEFT, pyxel.mouse_x, pyxel.mouse_y, 0, 0
            )

        # Copy/cut/paste bank (Ctrl+Shift or Cmd+Shift)
        has_cmd_or_ctrl = pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI)
        if pyxel.btn(pyxel.KEY_SHIFT) and has_cmd_or_ctrl:
            # Ctrl+Shift+C/Ctrl+Shift+X: Copy bank
            if pyxel.btnp(pyxel.KEY_C) or pyxel.btnp(pyxel.KEY_X):
                self._bank_buffer = {}
                if self._is_tilemap_mode:
                    tilemap = pyxel.tilemaps[self.tilemap_index_var]
                    self._bank_buffer["data"] = tilemap.get_slice(0, 0, 256, 256)
                    self._bank_buffer["imgsrc"] = tilemap.imgsrc
                else:
                    self._bank_buffer["data"] = pyxel.images[
                        self.image_index_var
                    ].get_slice(0, 0, 256, 256)

            # Ctrl+Shift+X: Cut bank
            if pyxel.btnp(pyxel.KEY_X):
                self._add_pre_history(bank_copy=True)
                if self._is_tilemap_mode:
                    pyxel.tilemaps[self.tilemap_index_var].rect(0, 0, 256, 256, (0, 0))
                else:
                    pyxel.images[self.image_index_var].rect(0, 0, 256, 256, 0)
                self._add_post_history(bank_copy=True)

            # Ctrl+Shift+V: Paste bank
            if pyxel.btnp(pyxel.KEY_V) and self._bank_buffer is not None:
                self._add_pre_history(bank_copy=True)
                if self._is_tilemap_mode:
                    pyxel.tilemaps[self.tilemap_index_var].set_slice(
                        0, 0, self._bank_buffer["data"], self.secondary_color_var
                    )
                    self.image_index_var = self._bank_buffer["imgsrc"]
                else:
                    pyxel.images[self.image_index_var].set_slice(
                        0, 0, self._bank_buffer["data"], self.secondary_color_var
                    )
                self._add_post_history(bank_copy=True)

        # Ctrl+A: Select all
        if pyxel.btnp(pyxel.KEY_A):
            self._select_x1 = self._select_y1 = 0
            self._select_x2 = self._select_y2 = 15
            self.tool_var = TOOL_SELECT

        # Shift + Arrows: Change selection
        if self.tool_var == TOOL_SELECT and pyxel.btn(pyxel.KEY_SHIFT):
            # CTRL/CMD inverts selection movement direction
            if has_cmd_or_ctrl:
                if pyxel.btnp(pyxel.KEY_LEFT):
                    self._select_x2 = clamp(self._select_x2 - 1 , self._select_x1, 15)
                if pyxel.btnp(pyxel.KEY_RIGHT):
                    self._select_x1 = clamp(self._select_x1 + 1, 0, self._select_x2)
                if pyxel.btnp(pyxel.KEY_UP):
                    self._select_y2 = clamp(self._select_y2 - 1, self._select_y1, 15)
                if pyxel.btnp(pyxel.KEY_DOWN):
                    self._select_y1 = clamp(self._select_y1 + 1, 0, self._select_y2)
            else:
                if pyxel.btnp(pyxel.KEY_LEFT):
                    self._select_x1 = clamp(self._select_x1 - 1 , 0, 15)
                if pyxel.btnp(pyxel.KEY_RIGHT):
                    self._select_x2 = clamp(self._select_x2 + 1, 0, 15)
                if pyxel.btnp(pyxel.KEY_UP):
                    self._select_y1 = clamp(self._select_y1 - 1, 0, 15)
                if pyxel.btnp(pyxel.KEY_DOWN):
                    self._select_y2 = clamp(self._select_y2 + 1, 0, 15)

        # Copy/cut/paste canvas (Ctrl/Cmd without Shift)
        if (
            self.tool_var == TOOL_SELECT
            and not pyxel.btn(pyxel.KEY_SHIFT)
            and has_cmd_or_ctrl
        ):
            # Ctrl+C / Ctrl+Insert: Copy
            if pyxel.btnp(pyxel.KEY_C) or pyxel.btnp(pyxel.KEY_INSERT):
                self.copy_canvas()

            # Ctrl+X: Cut
            if pyxel.btnp(pyxel.KEY_X):
                self.cut_canvas()

            # Ctrl+V: Paste
            if pyxel.btnp(pyxel.KEY_V):
                self.paste_canvas()

        # Copy/cut/paste canvas (Shift+Delete / Shift+Insert)
        if (
            self.tool_var == TOOL_SELECT
            and pyxel.btn(pyxel.KEY_SHIFT)
            and not has_cmd_or_ctrl
        ):
            # Shift+Delete: Cut
            if pyxel.btnp(pyxel.KEY_DELETE):
                self.cut_canvas()

            # Shift+Insert: Paste
            if pyxel.btnp(pyxel.KEY_INSERT):
                self.paste_canvas()

        # Move selection (Ctrl+Arrows)
        if (
            self.tool_var == TOOL_SELECT
            and has_cmd_or_ctrl
            and not pyxel.btn(pyxel.KEY_SHIFT)
        ):
            dx = dy = 0
            if pyxel.btnp(pyxel.KEY_LEFT, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME):
                dx = -1
            elif pyxel.btnp(pyxel.KEY_RIGHT, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME):
                dx = 1
            elif pyxel.btnp(pyxel.KEY_UP, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME):
                dy = -1
            elif pyxel.btnp(pyxel.KEY_DOWN, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME):
                dy = 1

            if dx != 0 or dy != 0:
                self._add_pre_history()
                x, y, w, h = self._selection_rect()
                buffer = (
                    pyxel.Tilemap(w, h, 0)
                    if self._is_tilemap_mode
                    else pyxel.Image(w, h)
                )
                buffer.blt(0, 0, self.canvas_var, x, y, w, h)
                self.clear_canvas(x, y, w, h)
                self.canvas_var.blt(x + dx, y + dy, buffer, 0, 0, w, h, colkey=self.secondary_color_var)
                self._select_x1 += dx
                self._select_x2 += dx
                self._select_y1 += dy
                self._select_y2 += dy
                self._add_post_history()

        # Selection tool operations (no Ctrl/Cmd)
        if self.tool_var == TOOL_SELECT and not has_cmd_or_ctrl:
            # DELETE: Clear selection
            if not pyxel.btn(pyxel.KEY_SHIFT) and pyxel.btnp(pyxel.KEY_DELETE):
                x, y, w, h = self._selection_rect()
                self._add_pre_history()
                self.clear_canvas(x, y, w, h)
                self._add_post_history()

            # H: Flip horizontal
            if pyxel.btnp(pyxel.KEY_H):
                x, y, w, h = self._selection_rect()
                self._add_pre_history()
                self.canvas_var.blt(x, y, self.canvas_var, x, y, -w, h)
                self._add_post_history()

            # V: Flip vertical
            if pyxel.btnp(pyxel.KEY_V):
                x, y, w, h = self._selection_rect()
                self._add_pre_history()
                self.canvas_var.blt(x, y, self.canvas_var, x, y, w, -h)
                self._add_post_history()
            
            # T: Turn sprite in a rotating angle, default 90 degrees
            if pyxel.btnp(pyxel.KEY_T):
                # Hold Shift for Counter Clockwise
                # Hold Alt for 45 degrees
                ccw = pyxel.btn(pyxel.KEY_SHIFT)
                rotating_angle = (-90 if ccw else 90)
                if pyxel.btn(pyxel.KEY_ALT):
                    rotating_angle /= 2

                x, y, w, h = self._selection_rect()

                # copy and rotate
                self._add_pre_history()
                new_canvas = (
                    pyxel.Tilemap(w, h, 0)
                    if self._is_tilemap_mode
                    else pyxel.Image(w, h)
                )
                new_canvas.blt(0, 0, self.canvas_var, x, y, w, h)
                self.clear_canvas(x, y, w, h)
                self.canvas_var.blt(x, y, new_canvas, 0, 0, w, h, rotate=rotating_angle, colkey=self.secondary_color_var)

               # Update selection bounds
                cx = (self._select_x1 + self._select_x2) / 2
                cy = (self._select_y1 + self._select_y2) / 2
                new_x1 = int(cx - (h - 1) / 2)
                new_y1 = int(cy - (w - 1) / 2)
                self._select_x1 = clamp(new_x1, 0, 15)
                self._select_y1 = clamp(new_y1, 0, 15)
                self._select_x2 = clamp(new_x1 + h - 1, 0, 15)
                self._select_y2 = clamp(new_y1 + w - 1, 0, 15)
  
                self._add_post_history()

        # Move tile focus
        if self._is_tilemap_mode and pyxel.btn(pyxel.KEY_SHIFT):
            if pyxel.btnp(
                pyxel.KEY_LEFT, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
                self.tile_x_var -= 1
            if pyxel.btnp(
                pyxel.KEY_RIGHT, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
                self.tile_x_var += 1
            if pyxel.btnp(
                pyxel.KEY_UP, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
                self.tile_y_var -= 1
            if pyxel.btnp(
                pyxel.KEY_DOWN, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
                self.tile_y_var += 1

        # Move target focus (only when no modifiers held)
        if not is_modifier_pressed():
            if pyxel.btnp(
                pyxel.KEY_LEFT, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
                self.focus_x_var -= 1
            if pyxel.btnp(
                pyxel.KEY_RIGHT, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
                self.focus_x_var += 1
            if pyxel.btnp(
                pyxel.KEY_UP, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
                self.focus_y_var -= 1
            if pyxel.btnp(
                pyxel.KEY_DOWN, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
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
            pyxel.user_pal()
            pyxel.bltm(
                self.x + 1,
                self.y + 1,
                canvas,
                offset_x * 8,
                offset_y * 8,
                128,
                128,
            )
            pyxel.pal()
        else:
            pyxel.user_pal()
            # blt scales centered on (x + (w-1)/2, y + (h-1)/2); shift dest by
            # (w * (scale - 1) + 1) / 2 = 56.5 so the 128x128 output aligns to
            # (self.x + 1, self.y + 1). Integer 57 rounds the center identically.
            pyxel.blt(
                self.x + 57,
                self.y + 57,
                canvas,
                offset_x,
                offset_y,
                16,
                16,
                scale=8,
            )
            pyxel.pal()

        pyxel.line(
            self.x + 1, self.y + 64, self.x + 128, self.y + 64, WIDGET_PANEL_COLOR
        )
        pyxel.line(
            self.x + 64, self.y + 1, self.x + 64, self.y + 128, WIDGET_PANEL_COLOR
        )

        # Draw selection area
        if self.tool_var == TOOL_SELECT:
            x = self.x + 1 + self._select_x1 * 8
            y = self.y + 1 + self._select_y1 * 8
            w = (self._select_x2 - self._select_x1 + 1) * 8
            h = (self._select_y2 - self._select_y1 + 1) * 8
            pyxel.clip(self.x + 1, self.y + 1, 128, 128)
            pyxel.rectb(x, y, w, h, PANEL_SELECT_FRAME_COLOR)
            pyxel.rectb(x + 1, y + 1, w - 2, h - 2, PANEL_SELECT_BORDER_COLOR)
            pyxel.rectb(x - 1, y - 1, w + 2, h + 2, PANEL_SELECT_BORDER_COLOR)
            pyxel.clip()
