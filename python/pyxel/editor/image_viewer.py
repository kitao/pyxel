import pyxel

from .settings import PANEL_FOCUS_BORDER_COLOR, PANEL_FOCUS_COLOR, clamp
from .widgets import ScrollBar, Widget

_GRID_SIZE = 8
_MAX_TILE_INDEX = 31  # 256 / _GRID_SIZE - 1


class ImageViewer(Widget):
    # Variables:
    #   image_index_var
    #   tilemap_index_var
    #   focus_x_var
    #   focus_y_var
    #   focus_w_var
    #   focus_h_var
    #   viewport_x_var
    #   viewport_y_var
    #   help_message_var
    #
    # Events:
    #   none

    def __init__(self, parent):
        if hasattr(parent, "tilemap_index_var"):
            y = 80
            height = 66
            slider_amount = 8
            self._is_tilemap_mode = True
            self.copy_var("tilemap_index_var", parent)
        else:
            y = 16
            height = 130
            slider_amount = 16
            self._is_tilemap_mode = False

        super().__init__(parent, 157, y, 66, height)
        self._press_x = 0
        self._press_y = 0
        self._drag_offset_x = 0
        self._drag_offset_y = 0
        self.copy_var("image_index_var", parent)
        self.copy_var("help_message_var", parent)

        self.new_var("focus_x_var", 0)
        self.add_var_event_listener("focus_x_var", "set", self.__on_focus_x_set)
        self.add_var_event_listener("focus_x_var", "change", self.__on_focus_x_change)

        self.new_var("focus_y_var", 0)
        self.add_var_event_listener("focus_y_var", "set", self.__on_focus_y_set)
        self.add_var_event_listener("focus_y_var", "change", self.__on_focus_y_change)

        self.new_var("focus_w_var", 1 if self._is_tilemap_mode else 2)

        self.new_var("focus_h_var", 1 if self._is_tilemap_mode else 2)

        # Initialize horizontal scroll bar
        self._h_scroll_bar = ScrollBar(
            self,
            0,
            height - 1,
            width=66,
            scroll_amount=32,
            slider_amount=8,
            value=0,
        )
        self.copy_var("viewport_x_var", self._h_scroll_bar, "value_var")

        # Initialize vertical scroll bar
        self._v_scroll_bar = ScrollBar(
            self,
            65,
            0,
            height=height,
            scroll_amount=32,
            slider_amount=slider_amount,
            value=0,
        )
        self.copy_var("viewport_y_var", self._v_scroll_bar, "value_var")
        
        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("mouse_wheel", self.__on_mouse_wheel)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)
        self.add_var_event_listener("viewport_y_var", "set", self.__on_viewport_y_set)
        self.add_var_event_listener("viewport_x_var", "set", self.__on_viewport_x_set)

    # Helpers

    def _screen_to_focus(self, x, y):
        x = clamp(
            self.viewport_x_var + (x - self.x - 1) // _GRID_SIZE, 0, _MAX_TILE_INDEX
        )
        y = clamp(
            self.viewport_y_var + (y - self.y - 1) // _GRID_SIZE, 0, _MAX_TILE_INDEX
        )
        return x, y

    # Event handlers
    def __on_viewport_y_set(self, value):
        max_v = 24 if self._is_tilemap_mode else 16
        return clamp(value, 0, max_v)

    def __on_viewport_x_set(self, value):
        vw = 8
        return clamp(value, 0, _MAX_TILE_INDEX + 1 - vw)

    def __on_focus_x_set(self, value):
        return clamp(value, 0, _MAX_TILE_INDEX + 1 - self.focus_w_var)

    def __on_focus_x_change(self, value):
        fx = self.focus_x_var
        fw = self.focus_w_var
        vx = self.viewport_x_var
        vw = 8
        self.viewport_x_var += min(fx - vx, 0) + max(fx + fw - vx - vw, 0)

    def __on_focus_y_set(self, value):
        return clamp(value, 0, _MAX_TILE_INDEX + 1 - self.focus_h_var)

    def __on_focus_y_change(self, value):
        fy = self.focus_y_var
        fh = self.focus_h_var
        vy = self.viewport_y_var
        vh = 8 if self._is_tilemap_mode else 16
        self.viewport_y_var += min(fy - vy, 0) + max(fy + fh - vy - vh, 0)

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            self.focus_x_var, self.focus_y_var = self._screen_to_focus(x, y)
            self._press_x = self.focus_x_var
            self._press_y = self.focus_y_var
        elif key == pyxel.MOUSE_BUTTON_MIDDLE or key == pyxel.MOUSE_BUTTON_RIGHT:
            self._drag_offset_x = 0
            self._drag_offset_y = 0

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            if self._is_tilemap_mode:
                new_x, new_y = self._screen_to_focus(x, y)
                self.focus_w_var = min(abs(new_x - self._press_x) + 1, 8)
                self.focus_h_var = min(abs(new_y - self._press_y) + 1, 8)
                self.focus_x_var = min(new_x, self._press_x)
                self.focus_y_var = min(new_y, self._press_y)
            else:
                self.__on_mouse_down(key, x, y)
        elif key == pyxel.MOUSE_BUTTON_MIDDLE or key == pyxel.MOUSE_BUTTON_RIGHT:
            self._drag_offset_x -= dx
            self._drag_offset_y -= dy
            if abs(self._drag_offset_x) >= _GRID_SIZE:
                offset = self._drag_offset_x // _GRID_SIZE
                self.viewport_x_var += offset
                self._drag_offset_x -= offset * _GRID_SIZE
            if abs(self._drag_offset_y) >= _GRID_SIZE:
                offset = self._drag_offset_y // _GRID_SIZE
                self.viewport_y_var += offset
                self._drag_offset_y -= offset * _GRID_SIZE

    def __on_mouse_wheel(self, dy):
        self.viewport_y_var -= dy

    def __on_update(self):
        if self.is_hit(pyxel.mouse_x, pyxel.mouse_y):
            has_ctrl = pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI)
            if has_ctrl and pyxel.btnp(pyxel.KEY_HOME):
                self.focus_x_var = 0
                self.focus_y_var = 0
            elif not has_ctrl and pyxel.btnp(pyxel.KEY_HOME):
                self.focus_x_var = 0
            elif has_ctrl and pyxel.btnp(pyxel.KEY_END):
                self.focus_x_var = _MAX_TILE_INDEX + 1 - self.focus_w_var
                self.focus_y_var = _MAX_TILE_INDEX + 1 - self.focus_h_var
            elif not has_ctrl and pyxel.btnp(pyxel.KEY_END):
                self.focus_x_var = _MAX_TILE_INDEX + 1 - self.focus_w_var

            elif pyxel.btnp(pyxel.KEY_PAGEUP):
                self.focus_y_var -= 8
            elif pyxel.btnp(pyxel.KEY_PAGEDOWN):
                self.focus_y_var += 8

    def __on_mouse_hover(self, x, y):
        x, y = self._screen_to_focus(x, y)
        self.help_message_var = (
            f"TILE:SHIFT+CURSOR ({x},{y})"
            if self._is_tilemap_mode
            else f"TARGET:CURSOR ({x * _GRID_SIZE},{y * _GRID_SIZE})"
        )

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        # Draw image preview.
        pyxel.user_pal()
        pyxel.blt(
            self.x + 1,
            self.y + 1,
            self.image_index_var,
            self.viewport_x_var * _GRID_SIZE,
            self.viewport_y_var * _GRID_SIZE,
            self.width - 2,
            self.height - 2,
        )
        pyxel.pal()

        # Draw focus outline.
        x = self.x + (self.focus_x_var - self.viewport_x_var) * _GRID_SIZE + 1
        y = self.y + (self.focus_y_var - self.viewport_y_var) * _GRID_SIZE + 1
        w = self.focus_w_var * _GRID_SIZE
        h = self.focus_h_var * _GRID_SIZE
        pyxel.clip(self.x + 1, self.y + 1, self.width - 2, self.height - 2)
        self.draw_corners(x, y, w, h)

        pyxel.clip()

    def draw_corners(self, x, y, w, h):
        corners = [
            (x,     y,      1,  1), # ┌
            (x,     y + h,  1, -1), # └
            (x + w, y,     -1,  1), # ┐
            (x + w, y + h, -1, -1)  # ┘
        ]

        for cx, cy, sx, sy in corners:
            # black Border
            pyxel.rectb(cx - (2 if sx > 0 else -1), 
                        cy - (2 if sy > 0 else +1), 
                        2, 4, PANEL_FOCUS_BORDER_COLOR)
            
            pyxel.rectb(cx - (2 if sx > 0 else +1), 
                        cy - (2 if sy > 0 else -1), 
                        4, 2, PANEL_FOCUS_BORDER_COLOR)
            # central pixel
            pyxel.pset(cx, cy, PANEL_FOCUS_BORDER_COLOR)

            # Inner white
            pyxel.pset(cx - sx, cy - sy, PANEL_FOCUS_COLOR)
            pyxel.pset(cx - sx, cy,      PANEL_FOCUS_COLOR)
            pyxel.pset(cx,      cy - sy, PANEL_FOCUS_COLOR)
   