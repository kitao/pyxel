import pyxel

from .button import Button
from .settings import WIDGET_BACKGROUND_COLOR, WIDGET_PANEL_COLOR
from .widget import Widget


class ScrollBar(Widget):
    """
    Variables:
        value_var

    Events:
        change (value)
    """

    def __init__(
        self,
        parent,
        x,
        y,
        *,
        width=None,
        height=None,
        scroll_amount,
        slider_amount,
        value,
        with_shadow=True,
        **kwargs,
    ):
        if width is None and height is None or width is not None and height is not None:
            raise ValueError("Either width or height should be specified")
        if height is not None:
            width = 7
            self._is_vertical = True
        else:
            height = 7
            self._is_vertical = False
        super().__init__(parent, x, y, width, height, **kwargs)
        self.scroll_amount = scroll_amount
        self.slider_amount = slider_amount
        self._with_shadow = with_shadow
        self._drag_offset = 0
        self._is_dragged = False

        # Initialize value_var
        self.new_var("value_var", value)
        self.add_var_event_listener("value_var", "set", self.__on_value_set)
        self.add_var_event_listener("value_var", "change", self.__on_value_change)

        # Initialize dec button
        if self._is_vertical:
            btn_w = 7
            btn_h = 6
        else:
            btn_w = 6
            btn_h = 7
        self.dec_button = Button(self, 0, 0, btn_w, btn_h)
        self.dec_button.add_event_listener("press", self.__on_dec_button_press)

        # Initialize inc button
        if self._is_vertical:
            inc_x = 0
            inc_y = height - 6
        else:
            inc_x = width - 6
            inc_y = 0
        self.inc_button = Button(self, inc_x, inc_y, btn_w, btn_h)
        self.inc_button.add_event_listener("press", self.__on_inc_button_press)

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_up", self.__on_mouse_up)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_repeat", self.__on_mouse_repeat)
        self.add_event_listener("draw", self.__on_draw)

    @property
    def _scroll_size(self):
        return (self.height if self._is_vertical else self.width) - 14

    @property
    def _slider_size(self):
        return round(self._scroll_size * self.slider_amount / self.scroll_amount)

    @property
    def _slider_pos(self):
        return round(7 + self._scroll_size * self.value_var / self.scroll_amount)

    def __on_value_set(self, value):
        return min(max(value, 0), self.scroll_amount)

    def __on_value_change(self, value):
        self.trigger_event("change", value)

    def __on_dec_button_press(self):
        self.value_var = max(self.value_var - 1, 0)

    def __on_inc_button_press(self):
        self.value_var = min(
            self.value_var + 1, self.scroll_amount - self.slider_amount
        )

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return
        x -= self.x
        y -= self.y
        self._drag_offset = (y if self._is_vertical else x) - self._slider_pos
        if self._drag_offset < 0:
            self.__on_dec_button_press()
        elif self._drag_offset >= self._slider_size:
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
        drag_pos = y if self._is_vertical else x
        value = (
            (drag_pos - self._drag_offset - 6) * self.scroll_amount / self._scroll_size
        )
        self.value_var = int(
            min(max(value, 0), self.scroll_amount - self.slider_amount)
        )

    def __on_mouse_repeat(self, key, x, y):
        if not self._is_dragged:
            self.__on_mouse_down(key, x, y)

    def __on_draw(self):
        x = self.x
        y = self.y
        w = self.width
        h = self.height
        self.draw_panel(x, y, w, h, with_shadow=self._with_shadow)
        inc_col = 6 if self.inc_button.is_pressed_var else WIDGET_BACKGROUND_COLOR
        dec_col = 6 if self.dec_button.is_pressed_var else WIDGET_BACKGROUND_COLOR
        if self._is_vertical:
            # Draw border
            pyxel.rect(x + 1, y + 1, w - 2, 4, dec_col)
            pyxel.rect(x + 1, y + 6, w - 2, h - 12, WIDGET_BACKGROUND_COLOR)
            pyxel.rect(x + 1, y + h - 5, w - 2, 4, inc_col)

            # Draw up arrow
            pyxel.pset(x + 3, y + 2, WIDGET_PANEL_COLOR)
            pyxel.line(x + 2, y + 3, x + w - 3, y + 3, WIDGET_PANEL_COLOR)

            # Draw down arrow
            pyxel.pset(x + 3, y + h - 3, WIDGET_PANEL_COLOR)
            pyxel.line(x + 2, y + h - 4, x + w - 3, y + h - 4, WIDGET_PANEL_COLOR)

            # Draw slider
            pyxel.rect(
                self.x + 2,
                self.y + self._slider_pos,
                3,
                self._slider_size,
                WIDGET_PANEL_COLOR,
            )
        else:
            # Draw border
            pyxel.rect(x + 1, y + 1, 4, h - 2, dec_col)
            pyxel.rect(x + 6, y + 1, w - 12, h - 2, WIDGET_BACKGROUND_COLOR)
            pyxel.rect(x + w - 5, y + 1, 4, h - 2, inc_col)

            # Draw left arrow
            pyxel.pset(x + 2, y + 3, WIDGET_PANEL_COLOR)
            pyxel.line(x + 3, y + 2, x + 3, y + h - 3, WIDGET_PANEL_COLOR)

            # Draw right arrow
            pyxel.pset(x + w - 3, y + h - 4, WIDGET_PANEL_COLOR)
            pyxel.line(x + w - 4, y + 2, x + w - 4, y + h - 3, WIDGET_PANEL_COLOR)

            # Draw slider
            pyxel.rect(
                self.x + self._slider_pos,
                self.y + 2,
                self._slider_size,
                3,
                WIDGET_PANEL_COLOR,
            )
