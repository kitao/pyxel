import pyxel

from .button import Button
from .widget import Widget


class ScrollBar(Widget):
    """
    Events:
        __on_change(value)
    """

    def __init__(
        self, parent, x, y, width, height, slider_range, scroll_range, **kwargs
    ):
        super().__init__(parent, x, y, width, height, **kwargs)

        self.slider_range = slider_range
        self.scroll_range = scroll_range
        self.is_horizontal = width > height
        self.button_size = min(width, height)
        self.scroll_size = (
            width if self.is_horizontal else height
        ) - self.button_size * 2
        self.slider_size = self.scroll_size * slider_range / scroll_range
        self.slider_pos = 0
        self._press_offset = 0
        self._is_dragged = True
        self.value = 0

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
        self.add_event_handler("update", self.__on_update)

        self.dec_button.add_event_handler("press", self.__on_dec_button_press)
        self.inc_button.add_event_handler("press", self.__on_inc_button_press)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        x -= self.x
        y -= self.y

        slider_pos = (
            self.button_size + self.scroll_size * self.value / self.scroll_range
        )
        self._press_offset = (x if self.is_horizontal else y) - slider_pos

        if self._press_offset < 0:
            self.dec_button.press()
        elif self._press_offset >= self.slider_size:
            self.inc_button.press()
        else:
            self._is_dragged = True

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if not self._is_dragged:
            return

        x -= self.x
        y -= self.y

        drag_pos = x if self.is_horizontal else y
        self.value = (
            (drag_pos - self._press_offset - self.button_size)
            * self.scroll_range
            / self.scroll_size
        )
        self.value = int(min(max(self.value, 0), self.scroll_range - self.slider_range))

        self.call_event_handler("change", self.value)

    def __on_update(self):
        self.slider_pos = round(
            self.button_size + self.scroll_size * self.value / self.scroll_range
        )

    def __on_dec_button_press(self):
        self.value = max(self.value - 1, 0)
        self.call_event_handler("change", self.value)

    def __on_inc_button_press(self):
        self.value = min(self.value + 1, self.scroll_range - self.slider_range)
        self.call_event_handler("change", self.value)
