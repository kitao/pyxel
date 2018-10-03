import pyxel

from .button import Button
from .constants import BUTTON_PRESSED_COLOR, WIDGET_BACKGROUND_COLOR, WIDGET_FRAME_COLOR
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
            self.dec_button = Button(self, x, y, self.button_size, self.button_size)
            self.inc_button = Button(
                self,
                x + width - self.button_size,
                y,
                self.button_size,
                self.button_size,
            )
        else:
            self.dec_button = Button(self, x, y, self.button_size, self.button_size)
            self.inc_button = Button(
                self,
                x,
                y + height - self.button_size,
                self.button_size,
                self.button_size,
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
        x1 = self.x
        y1 = self.y
        x2 = x1 + self.width - 1
        y2 = y1 + self.height - 1

        self.draw_frame(x1, y1, x2, y2)

        pyxel.rect(x1 + 1, y1, x2 - 1, y2, WIDGET_FRAME_COLOR)
        pyxel.rect(x1, y1 + 1, x2, y2 - 1, WIDGET_FRAME_COLOR)

        inc_color = (
            BUTTON_PRESSED_COLOR
            if self.inc_button.is_pressed
            else WIDGET_BACKGROUND_COLOR
        )
        dec_color = (
            BUTTON_PRESSED_COLOR
            if self.dec_button.is_pressed
            else WIDGET_BACKGROUND_COLOR
        )

        if self.is_horizontal:
            pyxel.rect(x1 + 1, y1 + 1, x1 + 4, y2 - 1, dec_color)
            pyxel.rect(x1 + 6, y1 + 1, x2 - 6, y2 - 1, WIDGET_BACKGROUND_COLOR)
            pyxel.rect(x2 - 1, y1 + 1, x2 - 4, y2 - 1, inc_color)

            pyxel.pix(x1 + 2, y1 + 3, WIDGET_FRAME_COLOR)
            pyxel.line(x1 + 3, y1 + 2, x1 + 3, y2 - 2, WIDGET_FRAME_COLOR)

            pyxel.pix(x2 - 2, y2 - 3, WIDGET_FRAME_COLOR)
            pyxel.line(x2 - 3, y1 + 2, x2 - 3, y2 - 2, WIDGET_FRAME_COLOR)

            x = self.x + self.slider_pos
            y = self.y + 2
            pyxel.rect(x, y, x + self.slider_size - 1, y + 2, WIDGET_FRAME_COLOR)
        else:
            pyxel.rect(x1 + 1, y1 + 1, x2 - 1, y1 + 4, dec_color)
            pyxel.rect(x1 + 1, y1 + 6, x2 - 1, y2 - 6, WIDGET_BACKGROUND_COLOR)
            pyxel.rect(x1 + 1, y2 - 1, x2 - 1, y2 - 4, inc_color)

            pyxel.pix(x1 + 3, y1 + 2, WIDGET_FRAME_COLOR)
            pyxel.line(x1 + 2, y1 + 3, x2 - 2, y1 + 3, WIDGET_FRAME_COLOR)

            pyxel.pix(x1 + 3, y2 - 2, WIDGET_FRAME_COLOR)
            pyxel.line(x1 + 2, y2 - 3, x2 - 2, y2 - 3, WIDGET_FRAME_COLOR)

            x = self.x + 2
            y = self.y + self.slider_pos
            pyxel.rect(x, y, x + 2, y + self.slider_size - 1, WIDGET_FRAME_COLOR)

    def __on_dec_button_press(self):
        self.value = max(self._value - 1, 0)

    def __on_inc_button_press(self):
        self.value = min(self._value + 1, self.scroll_range - self.slider_range)
