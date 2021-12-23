import pyxel

from .settings import EDITOR_IMAGE, PIANO_KEYBOARD_PLAY_COLOR, PIANO_KEYBOARD_REST_COLOR
from .widgets import Widget

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
    """
    Variables:
        note_var
        octave_var
        is_playing_var
        help_message_var
    """

    def __init__(self, parent):
        super().__init__(parent, 17, 25, 12, 123)
        self._preview_sound = pyxel.Sound()
        self._preview_sound.set("g2", "p", "3", "n", 30)
        self._preview_tone = 0
        self._mouse_note = None
        self.field_cursor = parent.field_cursor
        self.get_field = parent.get_field
        self.copy_var("octave_var", parent)
        self.copy_var("is_playing_var", parent)
        self.copy_var("help_message_var", parent)

        # Initialize note_var
        self.new_var("note_var", None)

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_up", self.__on_mouse_up)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

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
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return
        self.field_cursor.move_to(self.field_cursor.x, 0)
        self._mouse_note = self._screen_to_note(x, y)

    def __on_mouse_up(self, key, x, y):
        self._mouse_note = None

    def __on_mouse_drag(self, key, x, y, dx, dy):
        self.__on_mouse_down(key, x, y)

    def __on_mouse_hover(self, x, y):
        self.help_message_var = "NOTE:Z/S/X..Q/2/W..A+ENTER TONE:1"

    def __on_update(self):
        if (
            self.field_cursor.y > 0
            or self.is_playing_var
            or pyxel.btn(pyxel.KEY_SHIFT)
            or pyxel.btn(pyxel.KEY_CTRL)
            or pyxel.btn(pyxel.KEY_ALT)
            or pyxel.btn(pyxel.KEY_GUI)
        ):
            return
        if pyxel.btnp(pyxel.KEY_1):
            self._preview_tone = (self._preview_tone + 1) % 4
        self.note_var = self._mouse_note
        for i, key in enumerate(key_table):
            if pyxel.btn(key):
                self.note_var = self.octave_var * 12 + i
                break
        if pyxel.btn(pyxel.KEY_A):
            self.note_var = -1
        if self.note_var is not None:
            self._preview_sound.notes[0] = self.note_var
            self._preview_sound.tones[0] = self._preview_tone
            pyxel.play(1, self._preview_sound)
        else:
            pyxel.stop(1)

    def __on_draw(self):
        pyxel.blt(
            self.x,
            self.y,
            EDITOR_IMAGE,
            208,
            0,
            12,
            123,
        )
        notes = self.get_field(0)
        if (
            self.is_playing_var
            and not notes
            or not self.is_playing_var
            and self.note_var is None
        ):
            return
        note = notes[pyxel.play_pos(0)[1]] if self.is_playing_var else self.note_var
        key = note % 12
        x = self.x
        y = self.y + (59 - note) * 2
        if note == -1:
            pyxel.rect(x, y + 1, 12, 2, PIANO_KEYBOARD_REST_COLOR)
        elif key == 0 or key == 5:
            pyxel.rect(x, y + 1, 7, 1, PIANO_KEYBOARD_PLAY_COLOR)
            pyxel.rect(x + 7, y, 5, 2, PIANO_KEYBOARD_PLAY_COLOR)
        elif key == 4 or key == 11:
            pyxel.rect(x, y + 1, 7, 1, PIANO_KEYBOARD_PLAY_COLOR)
            pyxel.rect(x + 7, y + 1, 5, 2, PIANO_KEYBOARD_PLAY_COLOR)
        elif key == 2 or key == 7 or key == 9:
            pyxel.rect(x, y + 1, 7, 1, PIANO_KEYBOARD_PLAY_COLOR)
            pyxel.rect(x + 7, y, 5, 3, PIANO_KEYBOARD_PLAY_COLOR)
        else:
            pyxel.rect(x, y + 1, 6, 1, PIANO_KEYBOARD_PLAY_COLOR)
