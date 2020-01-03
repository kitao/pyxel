import pyxel

from .button import Button
from .constants import (
    BUTTON_DISABLED_COLOR,
    BUTTON_ENABLED_COLOR,
    BUTTON_PRESSED_COLOR,
    BUTTON_TEXT_COLOR,
    FONT_HEIGHT,
    FONT_WIDTH,
)


class TextButton(Button):
    """
    Events:
        __on_press()
        __on_release()
    """

    def __init__(self, parent, x, y, text, **kwargs):
        super().__init__(
            parent, x, y, len(text) * FONT_WIDTH + 3, FONT_HEIGHT + 1, **kwargs
        )

        self._text = text

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        x = self.x
        y = self.y
        w = self.width
        h = self.height
        col = (
            BUTTON_PRESSED_COLOR
            if self.is_pressed
            else (BUTTON_ENABLED_COLOR if self.is_enabled else BUTTON_DISABLED_COLOR)
        )

        pyxel.line(x + 1, y, x + w - 2, y, col)
        pyxel.rect(x, y + 1, w, h - 2, col)
        pyxel.line(x + 1, y + h - 1, x + w - 2, y + h - 1, col)
        pyxel.text(x + 2, y + 1, self._text, BUTTON_TEXT_COLOR)
