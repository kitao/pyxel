import pyxel

from .button import Button
from .widget import Widget


class ScrollBar(Widget):
    """
    Events:
        __on_change(value)
    """

    def __init__(
        self, parent, x, y, size, direction, scroll_range, slider_range, **kwargs
    ):
        if direction == "horizontal":
            width = size
            height = 7
        elif direction == "vertical":
            width = 7
            height = size
        else:
            pass  # todo

        super().__init__(parent, x, y, width, height, **kwargs)

        self._direction = direction
        self.scroll_range = scroll_range
        self.slider_range = slider_range
        self._drag_offset = 0
        self._is_dragged = True
        self._value = 0

        if self.is_horizontal:
            self.dec_button = Button(
                self, x, y, self.button_size, self.button_size, is_key_repeat=True
            )
            self.inc_button = Button(
                self,
                x + width - self.button_size,
                y,
                self.button_size,
                self.button_size,
                is_key_repeat=True,
            )
        else:
            self.dec_button = Button(
                self, x, y, self.button_size, self.button_size, is_key_repeat=True
            )
            self.inc_button = Button(
                self,
                x,
                y + height - self.button_size,
                self.button_size,
                self.button_size,
                is_key_repeat=True,
            )

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("draw", self.__on_draw)

        self.dec_button.add_event_handler("press", self.__on_dec_button_press)
        self.inc_button.add_event_handler("press", self.__on_inc_button_press)

    @property
    def is_horizontal(self):
        return self._direction == "horizontal"

    @property
    def button_size(self):
        return min(self.width, self.height)

    @property
    def scroll_size(self):
        return (
            self.width if self.is_horizontal else self.height
        ) - self.button_size * 2

    @property
    def slider_size(self):
        return round(self.scroll_size * self.slider_range / self.scroll_range)

    @property
    def slider_pos(self):
        return round(
            self.button_size + self.scroll_size * self._value / self.scroll_range
        )

    @property
    def value(self):
        return self._value

    @value.setter
    def value(self, value):
        if self._value != value:
            self._value = value
            self.call_event_handler("change", value)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        x -= self.x
        y -= self.y

        self._drag_offset = (x if self.is_horizontal else y) - self.slider_pos

        if self._drag_offset < 0:
            self.dec_button.press()
        elif self._drag_offset >= self.slider_size:
            self.inc_button.press()
        else:
            self._is_dragged = True

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if not self._is_dragged:
            return

        x -= self.x
        y -= self.y

        drag_pos = x if self.is_horizontal else y
        value = (
            (drag_pos - self._drag_offset - self.button_size)
            * self.scroll_range
            / self.scroll_size
        )
        self.value = int(min(max(value, 0), self.scroll_range - self.slider_range))

    def __on_draw(self):
        if self.is_horizontal:
            x = self.x + self.slider_pos
            y = self.y + 2
            pyxel.rect(x, y, x + self.slider_size - 1, y + 2, 1)
        else:
            x = self.x + 2
            y = self.y + self.slider_pos
            pyxel.rect(x, y, x + 2, y + self.slider_size - 1, 1)

        if self.inc_button.is_pressed or self.dec_button.is_pressed:
            if self.dec_button.is_pressed:
                x = self.x + 1
                y = self.y + 1
            elif self.is_horizontal:
                x = self.x + self.width - 5
                y = self.y + 1
            else:
                x = self.x + 1
                y = self.y + self.height - 5

            w, h = (4, 5) if self.is_horizontal else (5, 4)

            pyxel.pal(6, 7)
            pyxel.blt(x, y, 3, x, y + 12, w, h)
            pyxel.pal()

    def __on_dec_button_press(self):
        self.value = max(self._value - 1, 0)

    def __on_inc_button_press(self):
        self.value = min(self._value + 1, self.scroll_range - self.slider_range)
