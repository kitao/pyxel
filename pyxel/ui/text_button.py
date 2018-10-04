import pyxel
from pyxel.constants import FONT_HEIGHT, FONT_WIDTH

from .button import Button
from .constants import (
    BUTTON_DISABLED_COLOR,
    BUTTON_ENABLED_COLOR,
    BUTTON_PRESSED_COLOR,
    BUTTON_TEXT_COLOR,
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
        x1 = self.x
        y1 = self.y
        x2 = self.x + self.width - 1
        y2 = self.y + self.height - 1
        col = (
            BUTTON_PRESSED_COLOR
            if self.is_pressed
            else (BUTTON_ENABLED_COLOR if self.is_enabled else BUTTON_DISABLED_COLOR)
        )

        pyxel.rect(x1 + 1, y1, x2 - 1, y2, col)
        pyxel.rect(x1, y1 + 1, x2, y2 - 1, col)
        pyxel.text(x1 + 2, y1 + 1, self._text, BUTTON_TEXT_COLOR)
