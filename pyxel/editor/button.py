import pyxel

from .editor_constants import BUTTON_BLINK_TIME
from .widget import Widget


class Button(Widget):
    def __init__(self, parent, x, y, width, height):
        super().__init__(parent, x, y, width, height)

        self._blink_time = 0

        self.add_event_handler('press', self.on_press)
        self.add_event_handler('update', self.on_update)
        self.add_event_handler('draw', self.on_draw)

    def on_press(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            self._blink_time = BUTTON_BLINK_TIME + 1

    def on_update(self):
        if self._blink_time > 0:
            self._blink_time -= 1

    def on_draw(self):
        if not self.is_enabled or self._blink_time > 0:
            pyxel.pal(13, 7 if self.is_enabled else 5)

            x = self.x
            y = self.y
            pyxel.blt(x, y, 3, x, y + 16, self.width, self.height)

            pyxel.pal()
