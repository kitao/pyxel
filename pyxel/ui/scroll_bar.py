import pyxel

from .button import Button
from .constants import BUTTON_PRESSED_COLOR, WIDGET_BACKGROUND_COLOR, WIDGET_PANEL_COLOR
from .widget import Widget


class ScrollBar(Widget):
    """
    Events:
        __on_change(value)
    """

    HORIZONTAL = 0
    VERTICAL = 1

    def __init__(
        self,
        parent,
        x,
        y,
        size,
        direction,
        scroll_range,
        slider_range,
        value,
        *,
        with_shadow=True,
        **kwargs
    ):
        if direction == ScrollBar.HORIZONTAL:
            width = size
            height = 7
        elif direction == ScrollBar.VERTICAL:
            width = 7
            height = size
        else:
            raise ValueError("invalid direction")

        super().__init__(parent, x, y, width, height, **kwargs)

        self._direction = direction
        self.scroll_range = scroll_range
        self.slider_range = slider_range
        self._with_shadow = with_shadow
        self._drag_offset = 0
        self._is_dragged = False
        self._value = None

        if self._direction == ScrollBar.HORIZONTAL:
            self.dec_button = Button(self, x, y, 6, 7)
            self.inc_button = Button(self, x + width - 6, y, 6, 7)
        else:
            self.dec_button = Button(self, x, y, 7, 6)
            self.inc_button = Button(self, x, y + height - 6, 6, 7)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_up", self.__on_mouse_up)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("mouse_repeat", self.__on_mouse_repeat)
        self.add_event_handler("draw", self.__on_draw)
        self.dec_button.add_event_handler("press", self.__on_dec_button_press)
        self.dec_button.add_event_handler("repeat", self.__on_dec_button_press)
        self.inc_button.add_event_handler("press", self.__on_inc_button_press)
        self.inc_button.add_event_handler("repeat", self.__on_inc_button_press)

        self.value = value

    @property
    def scroll_size(self):
        return (
            self.width if self._direction == ScrollBar.HORIZONTAL else self.height
        ) - 14

    @property
    def slider_size(self):
        return round(self.scroll_size * self.slider_range / self.scroll_range)

    @property
    def slider_pos(self):
        return round(7 + self.scroll_size * self._value / self.scroll_range)

    @property
    def value(self):
        return self._value

    @value.setter
    def value(self, value):
        if self._value != value:
            self._value = value
            self.call_event_handler("change", value)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_LEFT_BUTTON:
            return

        x -= self.x
        y -= self.y

        self._drag_offset = (
            x if self._direction == ScrollBar.HORIZONTAL else y
        ) - self.slider_pos

        if self._drag_offset < 0:
            self.__on_dec_button_press()
        elif self._drag_offset >= self.slider_size:
            self.__on_inc_button_press()
        else:
            self._is_dragged = True

    def __on_mouse_up(self, key, x, y):
        self._is_dragged = False

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if not self._is_dragged:
            return

        x -= self.x
        y -= self.y

        drag_pos = x if self._direction == ScrollBar.HORIZONTAL else y
        value = (
            (drag_pos - self._drag_offset - 6) * self.scroll_range / self.scroll_size
        )
        self.value = int(min(max(value, 0), self.scroll_range - self.slider_range))

    def __on_mouse_repeat(self, key, x, y):
        if not self._is_dragged:
            self.__on_mouse_down(key, x, y)

    def __on_draw(self):
        x1 = self.x
        y1 = self.y
        x2 = self.x + self.width - 1
        y2 = self.y + self.height - 1

        self.draw_panel(x1, y1, self.width, self.height, with_shadow=self._with_shadow)

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

        if self._direction == ScrollBar.HORIZONTAL:
            pyxel.rect(x1 + 1, y1 + 1, x1 + 4, y2 - 1, dec_color)
            pyxel.rect(x1 + 6, y1 + 1, x2 - 6, y2 - 1, WIDGET_BACKGROUND_COLOR)
            pyxel.rect(x2 - 1, y1 + 1, x2 - 4, y2 - 1, inc_color)

            pyxel.pix(x1 + 2, y1 + 3, WIDGET_PANEL_COLOR)
            pyxel.line(x1 + 3, y1 + 2, x1 + 3, y2 - 2, WIDGET_PANEL_COLOR)

            pyxel.pix(x2 - 2, y2 - 3, WIDGET_PANEL_COLOR)
            pyxel.line(x2 - 3, y1 + 2, x2 - 3, y2 - 2, WIDGET_PANEL_COLOR)

            x = self.x + self.slider_pos
            y = self.y + 2
            pyxel.rect(x, y, x + self.slider_size - 1, y + 2, WIDGET_PANEL_COLOR)
        else:
            pyxel.rect(x1 + 1, y1 + 1, x2 - 1, y1 + 4, dec_color)
            pyxel.rect(x1 + 1, y1 + 6, x2 - 1, y2 - 6, WIDGET_BACKGROUND_COLOR)
            pyxel.rect(x1 + 1, y2 - 1, x2 - 1, y2 - 4, inc_color)

            pyxel.pix(x1 + 3, y1 + 2, WIDGET_PANEL_COLOR)
            pyxel.line(x1 + 2, y1 + 3, x2 - 2, y1 + 3, WIDGET_PANEL_COLOR)

            pyxel.pix(x1 + 3, y2 - 2, WIDGET_PANEL_COLOR)
            pyxel.line(x1 + 2, y2 - 3, x2 - 2, y2 - 3, WIDGET_PANEL_COLOR)

            x = self.x + 2
            y = self.y + self.slider_pos
            pyxel.rect(x, y, x + 2, y + self.slider_size - 1, WIDGET_PANEL_COLOR)

    def __on_dec_button_press(self):
        self.value = max(self._value - 1, 0)

    def __on_inc_button_press(self):
        self.value = min(self._value + 1, self.scroll_range - self.slider_range)
