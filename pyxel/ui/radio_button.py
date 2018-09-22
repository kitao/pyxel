import pyxel

from .widget import Widget


class RadioButton(Widget):
    """
    Events:
        __on_change(value)
    """

    def __init__(
        self,
        parent,
        x,
        y,
        button_w,
        button_h,
        margin_x,
        margin_y,
        column,
        row,
        **kwargs
    ):
        width = button_w * column + margin_x * (column - 1)
        height = button_h * row + margin_y * (row - 1)
        super().__init__(parent, x, y, width, height, **kwargs)

        self.button_w = button_w
        self.button_h = button_h
        self.margin_x = margin_x
        self.margin_y = margin_y
        self.column = column
        self.row = row
        self.value = 0

        self.add_event_handler("mouse_down", self.__on_mouse_down)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        x -= self.x
        y -= self.y

        interval_x = self.button_w + self.margin_x
        interval_y = self.button_h + self.margin_y

        index_x = x // interval_x
        index_y = y // interval_y

        button_x = interval_x * index_x
        button_y = interval_y * index_y

        if (
            x >= button_x
            and x < button_x + self.button_w
            and y >= button_y
            and y < button_y + self.button_h
        ):
            value = self.column * index_y + index_x

            if self.value != value:
                self.value = value
                self.call_event_handler("change", value)
