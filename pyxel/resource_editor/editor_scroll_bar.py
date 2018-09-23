import pyxel
from pyxel.ui import ScrollBar


class EditorScrollBar(ScrollBar):
    def __init__(
        self, parent, x, y, width, height, scroll_range, slider_range, **kwargs
    ):
        super().__init__(
            parent, x, y, width, height, scroll_range, slider_range, **kwargs
        )

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        if self.is_horizontal:
            x = self.x + self.slider_pos
            y = self.y + 2
            pyxel.rect(x, y, x + self.slider_size - 1, y + 2, 1)
        else:
            x = self.x + 2
            y = self.y + self.slider_pos
            pyxel.rect(x, y, x + 2, y + self.slider_size - 1, 1)

        if self.inc_button.is_pressed or self.dec_button.is_pressed:
            if self.dec_button.is_pressed:
                x = self.x + 1
                y = self.y + 1
            elif self.is_horizontal:
                x = self.x + self.width - 5
                y = self.y + 1
            else:
                x = self.x + 1
                y = self.y + self.height - 5

            w, h = (4, 5) if self.is_horizontal else (5, 4)

            pyxel.pal(6, 7)
            pyxel.blt(x, y, 3, x, y + 16, w, h)
            pyxel.pal()
