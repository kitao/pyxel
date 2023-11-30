import pyxel

from .settings import EDITOR_IMAGE
from .widgets import Widget
from .widgets.settings import BUTTON_ENABLED_COLOR, BUTTON_PRESSED_COLOR


class SoundSelector(Widget):
    """
    Variables:
        is_playing_var
        help_message_var
    """

    def __init__(self, parent):
        super().__init__(parent, 11, 129, 218, 44)
        self._pressed_sound = None
        self._preview_sound = None
        self._last_preview_sound = None
        self.field_cursor = parent.field_cursor
        self.copy_var("is_playing_var", parent)
        self.copy_var("help_message_var", parent)

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_up", self.__on_mouse_up)
        self.add_event_listener("mouse_repeat", self.__on_mouse_down)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    def _hit_sound_button(self, x, y):
        x -= self.x + 6
        y -= self.y + 5
        if x < 0 or y < 0 or x > 205 or y > 33 or x % 13 > 10 or y % 9 > 6:
            return None
        return (y // 9) * 16 + x // 13

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT or self.is_playing_var:
            return
        self._pressed_sound = self._hit_sound_button(x, y)
        if self._pressed_sound is not None:
            self.field_cursor.insert(self._pressed_sound)

    def __on_mouse_up(self, key, x, y):
        if key == pyxel.MOUSE_BUTTON_LEFT:
            self._pressed_sound = None

    def __on_mouse_hover(self, x, y):
        self.help_message_var = "PREVIEW:HOVER INSERT:CLICK"

    def __on_update(self):
        if self.is_playing_var:
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
        if self._preview_sound is None and pyxel.play_pos(0) is not None:
            pyxel.stop(0)
        self._last_preview_sound = self._preview_sound

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)
        pyxel.blt(self.x + 6, self.y + 5, EDITOR_IMAGE, 0, 121, 206, 34)
        for i in range(pyxel.NUM_SOUNDS):
            if pyxel.sounds[i].notes:
                self._draw_sound_button(i, BUTTON_ENABLED_COLOR)
        if self._pressed_sound is not None:
            self._draw_sound_button(self._pressed_sound, BUTTON_PRESSED_COLOR)

    def _draw_sound_button(self, snd, col):
        pyxel.pal(13, col)
        x = (snd % 16) * 13
        y = (snd // 16) * 9
        pyxel.blt(
            self.x + x + 6,
            self.y + y + 5,
            EDITOR_IMAGE,
            x,
            y + 121,
            11,
            7,
        )
        pyxel.pal()
