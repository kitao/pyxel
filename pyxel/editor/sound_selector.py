import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y


class SoundSelector(Widget):
    def __init__(self, parent):
        super().__init__(parent, 11, 129, 218, 44)

        self._pressed_sound = None
        self._preview_sound = None
        self._last_preview_sound = None

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_up", self.__on_mouse_up)
        self.add_event_handler("mouse_repeat", self.__on_mouse_down)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def _hit_sound_button(self, x, y):
        x -= self.x + 6
        y -= self.y + 5

        if x < 0 or y < 0 or x > 205 or y > 33 or x % 13 > 10 or y % 9 > 6:
            return None

        return (y // 9) * 16 + x // 13

    def _draw_sound_button(self, snd, col):
        pyxel.pal(13, col)
        x = (snd % 16) * 13
        y = (snd // 16) * 9
        pyxel.blt(
            self.x + x + 6,
            self.y + y + 5,
            pyxel.IMAGE_BANK_FOR_SYSTEM,
            EDITOR_IMAGE_X + x,
            EDITOR_IMAGE_Y + y + 121,
            11,
            7,
        )
        pyxel.pal()

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_LEFT_BUTTON or self.parent.is_playing:
            return

        self._pressed_sound = self._hit_sound_button(x, y)

        if self._pressed_sound is not None:
            self.parent.field_cursor.insert(self._pressed_sound)

    def __on_mouse_up(self, key, x, y):
        if key != pyxel.MOUSE_LEFT_BUTTON:
            return

        self._pressed_sound = None

    def __on_mouse_hover(self, x, y):
        self.parent.help_message = "PREVIEW:HOVER INSERT:CLICK"

    def __on_update(self):
        if self.parent.is_playing:
            return

        mx = pyxel.mouse_x
        my = pyxel.mouse_y

        if self.is_hit(mx, my):
            self._preview_sound = self._hit_sound_button(mx, my)

            if (
                self._preview_sound is not None
                and self._preview_sound != self._last_preview_sound
            ):
                pyxel.play(0, self._preview_sound, loop=True)
        else:
            self._preview_sound = None

        if self._preview_sound is None and pyxel.play_pos(0) >= 0:
            pyxel.stop(0)

        self._last_preview_sound = self._preview_sound

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)
        pyxel.blt(
            self.x + 6, self.y + 5, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 121, 206, 34, 6
        )

        for i in range(pyxel.SOUND_BANK_COUNT):
            if pyxel.sound(i).note:
                self._draw_sound_button(i, 12)

        if self._pressed_sound is not None:
            self._draw_sound_button(self._pressed_sound, 7)
