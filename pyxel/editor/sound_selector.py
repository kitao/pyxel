import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y


class SoundSelector(Widget):
    def __init__(self, parent):
        super().__init__(parent, 11, 129, 218, 44)

        self._pressed_sound = None
        self._lighting_time = 0

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.KEY_LEFT_BUTTON:
            return

        x -= self.x + 6
        y -= self.y + 5

        if x < 0 or y < 0 or x > 205 or y > 33 or x % 13 > 10 or y % 9 > 6:
            return

        self._pressed_sound = (y // 9) * 16 + x // 13
        self._lighting_time = 2

        self.parent.field_editor.insert(self._pressed_sound)

    def __on_mouse_hover(self, x, y):
        pass

    def __on_update(self):
        if self._lighting_time > 0:
            self._lighting_time -= 1
            self._pressed_sound = None

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)
        pyxel.blt(
            self.x + 6, self.y + 5, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 121, 206, 34, 6
        )

        if self._pressed_sound is not None:
            x = self.x
            y = self.y
            pyxel.rect(x, y, x + 10, y + 10, 8)
