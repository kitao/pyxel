import pyxel

from .settings import PANEL_FOCUS_BORDER_COLOR, PANEL_FOCUS_COLOR
from .widgets import ScrollBar, Widget


class ImageViewer(Widget):
    def __init__(self, parent):
        """
        Variables:
            image_index_var
            tilemap_index_var
            focus_x_var
            focus_y_var
            viewport_x_var
            viewport_y_var
            help_message_var
        """

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

        # Initialize focus_x_var
        self.new_var("focus_x_var", 0)
        self.add_var_event_listener("focus_x_var", "set", self.__on_focus_x_set)
        self.add_var_event_listener("focus_x_var", "change", self.__on_focus_x_change)

        # Initialize focus_y_var
        self.new_var("focus_y_var", 0)
        self.add_var_event_listener("focus_y_var", "set", self.__on_focus_y_set)
        self.add_var_event_listener("focus_y_var", "change", self.__on_focus_y_change)

        # Initialize focus_w_var
        self.new_var("focus_w_var", 1 if self._is_tilemap_mode else 2)

        # Initialize focus_h_var
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
        self.add_event_listener("draw", self.__on_draw)

    def _screen_to_focus(self, x, y):
        x = min(max(self.viewport_x_var + (x - self.x - 1) // 8, 0), 31)
        y = min(max(self.viewport_y_var + (y - self.y - 1) // 8, 0), 31)
        return x, y

    def __on_focus_x_set(self, value):
        return min(max(value, 0), 32 - self.focus_w_var)

    def __on_focus_x_change(self, value):
        fx = self.focus_x_var
        fw = self.focus_w_var
        vx = self.viewport_x_var
        vw = 8
        self.viewport_x_var += min(fx - vx, 0) + max(fx + fw - vx - vw, 0)

    def __on_focus_y_set(self, value):
        return min(max(value, 0), 32 - self.focus_h_var)

    def __on_focus_y_change(self, value):
        fy = self.focus_y_var
        fh = self.focus_h_var
        vy = self.viewport_y_var
        vh = 8 if self._is_tilemap_mode else 16
        self.viewport_y_var += min(fy - vy, 0) + max(fy + fh - vy - vh, 0)

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            # Toggle tile flag with Alt+click
            if pyxel.btn(pyxel.KEY_ALT) and not self._is_tilemap_mode:
                px = x - self.x - 1
                py = y - self.y - 1
                tx = self.viewport_x_var + px // 8
                ty = self.viewport_y_var + py // 8
                if 0 <= tx < 32 and 0 <= ty < 32:
                    lx = px % 8
                    ly = py % 8
                    bit = (0 if lx < 4 else 1) if ly < 4 else (3 if lx < 4 else 2)
                    img = pyxel.images[self.image_index_var]
                    img.fset(tx, ty, bit, not img.fget(tx, ty, bit))
                return
            self.focus_x_var, self.focus_y_var = self._screen_to_focus(x, y)
            self._press_x = self.focus_x_var
            self._press_y = self.focus_y_var
        elif key == pyxel.MOUSE_BUTTON_RIGHT:
            self._drag_offset_x = 0
            self._drag_offset_y = 0

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            if self._is_tilemap_mode:
                last_focus_x = self.focus_x_var
                last_focus_y = self.focus_y_var
                self._focus_x_var, self._focus_y_var = self._screen_to_focus(x, y)
                self.focus_w_var = min(abs(self._focus_x_var - self._press_x) + 1, 8)
                self.focus_h_var = min(abs(self._focus_y_var - self._press_y) + 1, 8)
                self.focus_x_var = min(self._focus_x_var, last_focus_x)
                self.focus_y_var = min(self._focus_y_var, last_focus_y)
            else:
                self.__on_mouse_down(key, x, y)
        elif key == pyxel.MOUSE_BUTTON_RIGHT:
            self._drag_offset_x -= dx
            self._drag_offset_y -= dy
            if abs(self._drag_offset_x) >= 8:
                offset = self._drag_offset_x // 8
                self.viewport_x_var += offset
                self._drag_offset_x -= offset * 8
            if abs(self._drag_offset_y) >= 8:
                offset = self._drag_offset_y // 8
                self.viewport_y_var += offset
                self._drag_offset_y -= offset * 8

    def __on_mouse_hover(self, x, y):
        if pyxel.btn(pyxel.KEY_ALT) and not self._is_tilemap_mode:
            tx, ty = self._screen_to_focus(x, y)
            self.help_message_var = f"FLAG:ALT+CLICK ({tx},{ty})"
            return

        x, y = self._screen_to_focus(x, y)
        self.help_message_var = (
            f"TILE:SHIFT+CURSOR ({x},{y})"
            if self._is_tilemap_mode
            else f"TARGET:CURSOR ({x * 8},{y * 8})"
        )

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        # Draw image
        pyxel.user_pal()
        pyxel.blt(
            self.x + 1,
            self.y + 1,
            self.image_index_var,
            self.viewport_x_var * 8,
            self.viewport_y_var * 8,
            self.width - 2,
            self.height - 2,
        )
        pyxel.pal()

        # Draw flag indicators
        if pyxel.btn(pyxel.KEY_ALT) and not self._is_tilemap_mode:
            flag_colors = (8, 10, 11, 12)
            petal_rects = (
                ((2, 2, 2, 2),),  # bit 0: top-left
                ((4, 2, 2, 2),),  # bit 1: top-right
                ((4, 4, 2, 2),),  # bit 2: bottom-right
                ((2, 4, 2, 2),),  # bit 3: bottom-left
            )
            vx = self.viewport_x_var
            vy = self.viewport_y_var
            cols = (self.width - 2) // 8
            rows = (self.height - 2) // 8
            pyxel.clip(self.x + 1, self.y + 1, self.width - 2, self.height - 2)
            for row in range(rows):
                for col in range(cols):
                    tx = vx + col
                    ty = vy + row
                    if tx >= 32 or ty >= 32:
                        continue
                    img = pyxel.images[self.image_index_var]
                    bx = self.x + 1 + col * 8
                    by = self.y + 1 + row * 8
                    pyxel.rect(bx + 2, by + 1, 4, 1, 0)
                    pyxel.rect(bx + 2, by + 6, 4, 1, 0)
                    pyxel.rect(bx + 1, by + 2, 1, 4, 0)
                    pyxel.rect(bx + 6, by + 2, 1, 4, 0)
                    for bit in range(4):
                        c = flag_colors[bit] if img.fget(tx, ty, bit) else 1
                        for ox, oy, pw, ph in petal_rects[bit]:
                            pyxel.rect(bx + ox, by + oy, pw, ph, c)
            pyxel.clip()

        # Draw focus
        x = self.x + (self.focus_x_var - self.viewport_x_var) * 8 + 1
        y = self.y + (self.focus_y_var - self.viewport_y_var) * 8 + 1
        w = self.focus_w_var * 8
        h = self.focus_h_var * 8
        pyxel.clip(self.x + 1, self.y + 1, self.width - 2, self.height - 2)
        pyxel.rectb(x, y, w, h, PANEL_FOCUS_COLOR)
        pyxel.rectb(x + 1, y + 1, w - 2, h - 2, PANEL_FOCUS_BORDER_COLOR)
        pyxel.rectb(x - 1, y - 1, w + 2, h + 2, PANEL_FOCUS_BORDER_COLOR)
        pyxel.clip()
