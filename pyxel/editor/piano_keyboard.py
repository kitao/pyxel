import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y


class PianoKeyboard(Widget):
    def __init__(self, parent):
        super().__init__(parent, 17, 25, 12, 123)

        self._sound = pyxel.sound(64, system=True)
        self._sound.set("g2", "p", "3", "n", 10)
        self.note = None
        self._tone = 0
        self._key_table = [
            pyxel.KEY_Z,
            pyxel.KEY_S,
            pyxel.KEY_X,
            pyxel.KEY_D,
            pyxel.KEY_C,
            pyxel.KEY_V,
            pyxel.KEY_G,
            pyxel.KEY_B,
            pyxel.KEY_H,
            pyxel.KEY_N,
            pyxel.KEY_J,
            pyxel.KEY_M,
            pyxel.KEY_Q,
            pyxel.KEY_2,
            pyxel.KEY_W,
            pyxel.KEY_3,
            pyxel.KEY_E,
            pyxel.KEY_R,
            pyxel.KEY_5,
            pyxel.KEY_T,
            pyxel.KEY_6,
            pyxel.KEY_Y,
            pyxel.KEY_7,
            pyxel.KEY_U,
        ]

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def __on_mouse_down(self, key, x, y):
        pass

    def __on_mouse_hover(self, x, y):
        self.parent.help_message = "PLAY:Z/S/X..Q/2/W..A TONE:1"

    def __on_update(self):
        if (
            self.parent.field_cursor.y > 0
            or self.parent.is_playing
            or pyxel.btn(pyxel.KEY_SHIFT)
            or pyxel.btn(pyxel.KEY_CONTROL)
            or pyxel.btn(pyxel.KEY_ALT)
        ):
            return

        if pyxel.btnp(pyxel.KEY_1):
            self._tone = (self._tone + 1) % 4

        self.note = None
        for i, key in enumerate(self._key_table):
            if pyxel.btn(key):
                self.note = self.parent.octave * 12 + i
                break

        if pyxel.btn(pyxel.KEY_A):
            self.note = -1

        if self.note is not None:
            self._sound.note[0] = self.note
            self._sound.tone[0] = self._tone
            pyxel.play(1, 64)
        else:
            pyxel.stop(1)

    def __on_draw(self):
        pyxel.blt(self.x, self.y, 3, EDITOR_IMAGE_X + 208, EDITOR_IMAGE_Y, 12, 123, 6)

        data = self.parent.get_data(0)

        if (
            self.parent.is_playing
            and not data
            or not self.parent.is_playing
            and self.note is None
        ):
            return

        note = data[self.parent.play_pos] if self.parent.is_playing else self.note
        x = self.x
        y = self.y + (59 - note) * 2
        key = note % 12

        if note == -1:
            pyxel.rect(x, y + 1, x + 11, y + 2, 12)
        elif key == 0 or key == 5:
            pyxel.rect(x, y + 1, x + 6, y + 1, 14)
            pyxel.rect(x + 7, y, x + 11, y + 1, 14)
        elif key == 4 or key == 11:
            pyxel.rect(x, y + 1, x + 6, y + 1, 14)
            pyxel.rect(x + 7, y + 1, x + 11, y + 2, 14)
        elif key == 2 or key == 7 or key == 9:
            pyxel.rect(x, y + 1, x + 6, y + 1, 14)
            pyxel.rect(x + 7, y, x + 11, y + 2, 14)
        else:
            pyxel.rect(x, y + 1, x + 5, y + 1, 14)
