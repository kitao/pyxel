import pyxel

from .constants import BUTTON_DISABLED_COLOR, BUTTON_ENABLED_COLOR, BUTTON_PRESSED_COLOR
from .toggle_button import ToggleButton


class ImageToggleButton(ToggleButton):
    """
    Events:
        __on_change()
    """

    def __init__(self, parent, x, y, img, sx, sy, **kwargs):
        super().__init__(parent, x, y, 7, 7, **kwargs)

        self._img = img
        self._sx = sx
        self._sy = sy

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        col = (
            (BUTTON_PRESSED_COLOR if self._value else BUTTON_ENABLED_COLOR)
            if self.is_enabled
            else BUTTON_DISABLED_COLOR
        )

        pyxel.pal(BUTTON_ENABLED_COLOR, col)
        pyxel.blt(
            self.x, self.y, self._img, self._sx, self._sy, self.width, self.height, 0
        )
        pyxel.pal()
