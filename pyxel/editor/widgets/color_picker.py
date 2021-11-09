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

        # valur_var
        self.new_var("value_var", value)
        self.add_var_event_listener("value_var", "change", self.__on_value_change)

        # event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("draw", self.__on_draw)

    def check_value(self, x, y):
        x -= self.x + 1
        y -= self.y + 1
        if (
            0 <= x <= self.width - 2
            and 0 <= y <= self.height - 2
            and x % 8 != 7
            and y != 7
        ):
            return (y // 8) * 8 + x // 8
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

        # colors
        for i in range(2):
            for j in range(8):
                pyxel.rect(self.x + j * 8 + 1, self.y + i * 8 + 1, 7, 7, i * 8 + j)

        # cursor
        col = self.value_var
        pyxel.text(
            self.x + (col % 8) * 8 + 3,
            self.y + (col // 8) * 8 + 2,
            "+",
            7 if col < 6 else 0,
        )

    def __on_value_change(self, value):
        self.trigger_event("change", value)
