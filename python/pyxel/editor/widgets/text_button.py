import pyxel

from .button import Button
from .settings import BUTTON_TEXT_COLOR


class TextButton(Button):
    """
    Variables:
        is_pressed_var

    Events:
        press
    """

    def __init__(self, parent, x, y, *, text, **kwargs):
        super().__init__(
            parent,
            x,
            y,
            len(text) * pyxel.FONT_WIDTH + 3,
            pyxel.FONT_HEIGHT + 1,
            **kwargs,
        )
        self._text = text

        # Set event listeners
        self.add_event_listener("draw", self.__on_draw)

    def __on_draw(self):
        x = self.x
        y = self.y
        w = self.width
        h = self.height
        col = self.button_color
        pyxel.line(x + 1, y, x + w - 2, y, col)
        pyxel.rect(x, y + 1, w, h - 2, col)
        pyxel.line(x + 1, y + h - 1, x + w - 2, y + h - 1, col)
        pyxel.text(x + 2, y + 1, self._text, BUTTON_TEXT_COLOR)
