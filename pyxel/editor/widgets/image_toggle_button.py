import pyxel

from .settings import BUTTON_ENABLED_COLOR
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
        show
        hide
        enabled
        disabled
        mouse_down (key, x, y)
        mouse_up (key, x, y)
        mouse_drag (key, x, y, dx, dy)
        mouse_repeat (key, x, y)
        mouse_click (key, x, y)
        mouse_hover (x, y)
        update
        draw
    """

    def __init__(self, parent, x, y, img, u, v, **kwargs):
        super().__init__(parent, x, y, 7, 7, **kwargs)

        self._img = img
        self._u = u
        self._v = v

        self.add_event_listener("draw", self.__on_draw)

    def __on_draw(self):
        pyxel.pal(BUTTON_ENABLED_COLOR, self.button_color)
        pyxel.blt(
            self.x, self.y, self._img, self._u, self._v, self.width, self.height, 0
        )
        pyxel.pal()
