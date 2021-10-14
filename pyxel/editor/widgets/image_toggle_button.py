import pyxel

from .settings import BUTTON_DISABLED_COLOR, BUTTON_ENABLED_COLOR, BUTTON_PRESSED_COLOR
from .toggle_button import ToggleButton


class ImageToggleButton(ToggleButton):
    """
    Variables:
        is_visible_var
        is_enabled_var
        is_checked_var

    Events:
        checked
        unchecked
    """

    def __init__(self, parent, x, y, img, u, v, **kwargs):
        super().__init__(parent, x, y, 7, 7, **kwargs)

        self._img = img
        self._u = u
        self._v = v

        self.add_event_listener("draw", self.__on_draw)

    def __on_draw(self):
        col = (
            (BUTTON_PRESSED_COLOR if self._value else BUTTON_ENABLED_COLOR)
            if self.is_enabled
            else BUTTON_DISABLED_COLOR
        )

        pyxel.pal(BUTTON_ENABLED_COLOR, col)
        pyxel.blt(
            self.x, self.y, self._img, self._u, self._v, self.width, self.height, 0
        )
        pyxel.pal()
