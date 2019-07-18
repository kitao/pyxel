import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y

key_table = [
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


class PianoKeyboard(Widget):
    def __init__(self, parent):
        super().__init__(parent, 17, 25, 12, 123)

        self._sound = pyxel.sound(pyxel.SOUND_BANK_FOR_SYSTEM, system=True)
        self._sound.set("g2", "p", "3", "n", 30)
        self._mouse_note = None
        self.note = None
        self._tone = 0

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_up", self.__on_mouse_up)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def _screen_to_note(self, x, y):
        x -= self.x
        y -= self.y

        octave = (4 - y // 24) * 12
        y %= 24

        if octave > 59:
            return 59
        if octave < 0:
            return -1

        if x <= 6:
            if 2 <= y <= 4:
                return octave + 10
            elif 6 <= y <= 8:
                return octave + 8
            elif 10 <= y <= 12:
                return octave + 6
            elif 16 <= y <= 18:
                return octave + 3
            elif 20 <= y <= 22:
                return octave + 1

        if y <= 2:
            return octave + 11
        elif y <= 6:
            return octave + 9
        elif y <= 10:
            return octave + 7
        elif y <= 13:
            return octave + 5
        elif y <= 16:
            return octave + 4
        elif y <= 20:
            return octave + 2
        else:
            return octave

    def __on_mouse_down(self, key, x, y):
        if key == pyxel.MOUSE_LEFT_BUTTON:
            self._mouse_note = self._screen_to_note(x, y)

    def __on_mouse_up(self, key, x, y):
        self._mouse_note = None

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key == pyxel.MOUSE_LEFT_BUTTON:
            self._mouse_note = self._screen_to_note(x, y)

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

        self.note = self._mouse_note
        for i, key in enumerate(key_table):
            if pyxel.btn(key):
                self.note = self.parent.octave * 12 + i
                break

        if pyxel.btn(pyxel.KEY_A):
            self.note = -1

        if self.note is not None:
            self._sound.note[0] = self.note
            self._sound.tone[0] = self._tone
            pyxel.play(1, pyxel.SOUND_BANK_FOR_SYSTEM)
        else:
            pyxel.stop(1)

    def __on_draw(self):
        pyxel.blt(
            self.x,
            self.y,
            pyxel.IMAGE_BANK_FOR_SYSTEM,
            EDITOR_IMAGE_X + 208,
            EDITOR_IMAGE_Y,
            12,
            123,
            6,
        )

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
            pyxel.rect(x, y + 1, 12, 2, 12)
        elif key == 0 or key == 5:
            pyxel.rect(x, y + 1, 7, 1, 14)
            pyxel.rect(x + 7, y, 5, 2, 14)
        elif key == 4 or key == 11:
            pyxel.rect(x, y + 1, 7, 1, 14)
            pyxel.rect(x + 7, y + 1, 5, 2, 14)
        elif key == 2 or key == 7 or key == 9:
            pyxel.rect(x, y + 1, 7, 1, 14)
            pyxel.rect(x + 7, y, 5, 3, 14)
        else:
            pyxel.rect(x, y + 1, 6, 1, 14)
