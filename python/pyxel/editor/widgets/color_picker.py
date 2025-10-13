import pyxel

from .widget import Widget


class ColorPicker(Widget):
    """
    Variables:
        value_var

    Events:
        change (value)
    """

    def __init__(self, parent, x, y, value, *, with_shadow=False, **kwargs):
        super().__init__(parent, x, y, 65, 17, **kwargs)
        self._with_shadow = with_shadow
        self._color_width = 4 if pyxel.num_user_colors > 16 else 8
        self._color_height = 4 if pyxel.num_user_colors > 32 else 8
        self._num_cols = 64 // self._color_width
        self._num_rows = 16 // self._color_height

        # Initialize value_var
        self.new_var("value_var", value)
        self.add_var_event_listener("value_var", "set", self.__on_value_set)
        self.add_var_event_listener("value_var", "change", self.__on_value_change)

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("draw", self.__on_draw)

    def check_value(self, x, y):
        x -= self.x + 1
        y -= self.y + 1
        cw = self._color_width
        ch = self._color_height
        if 0 <= x <= self.width - 2 and 0 <= y <= self.height - 2:
            col = (y // ch) * self._num_cols + x // cw
            return col if col < pyxel.num_user_colors else None
        else:
            return None

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return
        value = self.check_value(x, y)
        if value is not None:
            self.value_var = value

    def __on_mouse_drag(self, key, x, y, dx, dy):
        self.__on_mouse_down(key, x, y)

    def __on_draw(self):
        self.draw_panel(
            self.x, self.y, self.width, self.height, with_shadow=self._with_shadow
        )

        # Draw colors
        cw = self._color_width
        ch = self._color_height
        pyxel.user_pal()
        for yi in range(self._num_rows):
            for xi in range(self._num_cols):
                color = yi * self._num_cols + xi
                if color < pyxel.num_user_colors:
                    pyxel.rect(
                        self.x + xi * cw + 1,
                        self.y + yi * ch + 1,
                        cw - 1,
                        ch - 1,
                        color,
                    )
        pyxel.pal()

        # Draw cursor
        col = self.value_var
        if col >= pyxel.num_user_colors:
            return
        x = self.x + cw * (col % self._num_cols) + cw // 2
        y = self.y + ch * (col // self._num_cols) + ch // 2
        rgb = pyxel.colors[pyxel.NUM_COLORS + col]
        brightness = int(
            ((rgb >> 16) & 0xFF) * 0.299
            + ((rgb >> 8) & 0xFF) * 0.587
            + (rgb & 0xFF) * 0.114
        )
        pyxel.elli(
            x - cw // 8,
            y - ch // 8,
            1 + cw // 8 * 1.5,
            1 + ch // 8 * 1.5,
            7 if brightness < 140 else 0,
        )

    def __on_value_set(self, value):
        return min(value, pyxel.num_user_colors - 1)

    def __on_value_change(self, value):
        self.trigger_event("change", value)
