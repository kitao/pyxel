import pyxel
from pyxel.ui import NumberPicker

BUTTON_SIZE = 7


class EditorNumberPicker(NumberPicker):
    """
    Events:
        __on_change(value)
    """

    def __init__(self, parent, x, y, width, height, min_value, max_value, **kwargs):
        super().__init__(parent, x, y, width, height, min_value, max_value, **kwargs)

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        pyxel.text(self.x + 11, self.y + 1, str(self.value), 1)

        if self.inc_button.is_pressed or self.dec_button.is_pressed:
            x = (
                self.x
                if self.dec_button.is_pressed
                else self.x + self.width - BUTTON_SIZE
            )
            y = self.y

            pyxel.pal(13, 7)
            pyxel.blt(x, y, 3, x, y + 16, BUTTON_SIZE, BUTTON_SIZE)
            pyxel.pal()
