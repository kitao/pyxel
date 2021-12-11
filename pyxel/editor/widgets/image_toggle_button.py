import pyxel

from .settings import BUTTON_ENABLED_COLOR
from .toggle_button import ToggleButton


class ImageToggleButton(ToggleButton):
    """
    Variables:
        is_checked_var

    Events:
        checked
        unchecked
    """

    def __init__(self, parent, x, y, *, img, u, v, is_checked, **kwargs):
        super().__init__(parent, x, y, 7, 7, is_checked=is_checked, **kwargs)
        self._img = img
        self._u = u
        self._v = v

        # event listeners
        self.add_event_listener("draw", self.__on_draw)

    def __on_draw(self):
        pyxel.pal(BUTTON_ENABLED_COLOR, self.button_color)
        pyxel.blt(
            self.x, self.y, self._img, self._u, self._v, self.width, self.height, 0
        )
        pyxel.pal()
