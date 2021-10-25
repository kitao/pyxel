import pyxel

from .button import Button
from .settings import BUTTON_ENABLED_COLOR


class ImageButton(Button):
    """
    Variables:
        is_pressed_var

    Events:
        press
    """

    def __init__(self, parent, x, y, *, img, u, v, **kwargs):
        super().__init__(parent, x, y, 7, 7, **kwargs)
        self._img = img
        self._u = u
        self._v = v

        # event listeners
        self.add_event_listener("draw", self.__on_draw)

    def __on_draw(self):
        pyxel.pal(BUTTON_ENABLED_COLOR, self.button_color)
        pyxel.blt(
            self.x,
            self.y,
            self._img,
            self._u,
            self._v,
            self.width,
            self.height,
            0,
        )
        pyxel.pal()
