import pyxel

from .widget import Widget

BUTTON_SIZE = 7


class ScrollBar(Widget):
    def __init__(self, parent, x, y, width, height, numerator, denominator):
        super().__init__(parent, x, y, width, height)

        self._numerator = numerator
        self._denominator = denominator
        self._is_horizontal = width > height
        self.value = 0

        self._bar_size = (width
                          if self._is_horizontal else height) - BUTTON_SIZE * 2
        self._slider_size = self._bar_size * numerator / denominator

        self._press_offset = 0
        self._is_dragging = False
        self._inc_blink_time = 0
        self._dec_blink_time = 0

        self.add_event_handler('press', self.on_press)
        self.add_event_handler('release', self.on_release)
        self.add_event_handler('drag', self.on_drag)
        self.add_event_handler('update', self.on_update)
        self.add_event_handler('draw', self.on_draw)

    def on_press(self, key, x, y):
        slider_pos = (
            BUTTON_SIZE + self._bar_size * self.value / self._denominator)
        self._press_offset = (x if self._is_horizontal else y) - slider_pos

        if self._press_offset < 0:
            self.value = max(self.value - 1, 0)
            self._dec_blink_time = 4
            self.call_event_handler('change', self.value)
        elif self._press_offset >= self._slider_size:
            self.value = min(self.value + 1,
                             self._denominator - self._numerator)
            self._inc_blink_time = 4
            self.call_event_handler('change', self.value)
        else:
            self._is_dragging = True

    def on_release(self, key, x, y):
        self._is_dragging = False

    def on_drag(self, key, x, y, dx, dy):
        if not self._is_dragging:
            return

        drag_pos = x if self._is_horizontal else y
        self.value = (drag_pos - self._press_offset -
                      BUTTON_SIZE) * self._denominator / self._bar_size
        self.value = int(
            min(max(self.value, 0), self._denominator - self._numerator))

        self.call_event_handler('change', self.value)

    def on_update(self):
        if self._inc_blink_time > 0:
            self._inc_blink_time -= 1

        if self._dec_blink_time > 0:
            self._dec_blink_time -= 1

    def on_draw(self):
        slider_pos = int(BUTTON_SIZE +
                         self._bar_size * self.value / self._denominator + 0.5)

        if self._is_horizontal:
            x = self.x + slider_pos
            y = self.y + 2
            pyxel.rect(x, y, x + self._slider_size - 1, y + 2, 1)
        else:
            x = self.x + 2
            y = self.y + slider_pos
            pyxel.rect(x, y, x + 2, y + self._slider_size - 1, 1)

        if self._inc_blink_time > 0 or self._dec_blink_time > 0:
            if self._dec_blink_time > 0:
                x = self.x + 1
                y = self.y + 1
            elif self._is_horizontal:
                x = self.x + self.width - 5
                y = self.y + 1
            else:
                x = self.x + 1
                y = self.y + self.height - 5

            w, h = (4, 5) if self._is_horizontal else (5, 4)

            pyxel.pal(6, 7)
            pyxel.blt(x, y, 3, x, y + 16, w, h)
            pyxel.pal()

    def on_change(self, value):
        pass
