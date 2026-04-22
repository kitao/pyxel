import pyxel

from .settings import (
    EDITOR_IMAGE,
    PIANO_KEYBOARD_PLAY_COLOR,
    PIANO_KEYBOARD_REST_COLOR,
    is_modifier_pressed,
)
from .widgets import Widget

_KEY_TABLE = [
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

# (y_start, y_end, note_offset) within each 24px octave block
_BLACK_KEY_RANGES = [
    (2, 4, 10),
    (6, 8, 8),
    (10, 12, 6),
    (16, 18, 3),
    (20, 22, 1),
]
_WHITE_KEY_RANGES = [
    (0, 2, 11),
    (2, 6, 9),
    (6, 10, 7),
    (10, 13, 5),
    (13, 16, 4),
    (16, 20, 2),
    (20, 24, 0),
]

# Classification of white keys for the playback highlight shape
_BOTTOM_WHITE_KEYS = {0, 5}  # C, F — below a black key
_TOP_WHITE_KEYS = {4, 11}  # E, B — above a black key
_FULL_WHITE_KEYS = {2, 7, 9}  # D, G, A — between two black keys


class PianoKeyboard(Widget):
    # Variables:
    #   note_var
    #   octave_var
    #   is_playing_var
    #   help_message_var

    def __init__(self, parent):
        super().__init__(parent, 17, 25, 12, 123)
        self._preview_sound = pyxel.Sound()
        self._preview_sound.set("g2", "p", "3", "n", 30)
        self._preview_tone = 0
        self._mouse_note = None
        self.field_cursor = parent.field_cursor
        self.get_field = parent.get_field
        self.copy_var("speed_var", parent)
        self.copy_var("octave_var", parent)
        self.copy_var("is_playing_var", parent)
        self.copy_var("help_message_var", parent)

        self.new_var("note_var", None)

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_up", self.__on_mouse_up)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    # Helpers

    def _screen_to_note(self, x, y):
        x -= self.x
        y -= self.y
        octave = (4 - y // 24) * 12
        y %= 24
        if octave > 59:
            return 59
        if octave < 0:
            return -1

        # Check black keys first (narrower region, x <= 6)
        if x <= 6:
            for y_start, y_end, offset in _BLACK_KEY_RANGES:
                if y_start <= y <= y_end:
                    return octave + offset

        # Then white keys
        for y_start, y_end, offset in _WHITE_KEY_RANGES:
            if y_start <= y < y_end:
                return octave + offset
        return octave

    # Event handlers

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT:
            return
        if self.field_cursor.y > 0:
            self.field_cursor.move_to(self.field_cursor.x, 0, False)
        self._mouse_note = self._screen_to_note(x, y)

    def __on_mouse_up(self, key, x, y):
        self._mouse_note = None

    def __on_mouse_drag(self, key, x, y, dx, dy):
        self.__on_mouse_down(key, x, y)

    def __on_mouse_hover(self, x, y):
        self.help_message_var = "NOTE:Z/S/X..Q/2/W..A+ENTER TONE:1"

    def __on_update(self):
        if self.field_cursor.y > 0 or self.is_playing_var or is_modifier_pressed():
            return

        if pyxel.btnp(pyxel.KEY_1):
            self._preview_tone = (self._preview_tone + 1) % 4

        self.note_var = self._mouse_note
        for i, key in enumerate(_KEY_TABLE):
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
        pyxel.blt(self.x, self.y, EDITOR_IMAGE, 208, 0, 12, 123)

        play_pos = pyxel.play_pos(0)
        notes = self.get_field(0)

        if play_pos is not None and notes:
            note = notes[min(round(play_pos[1] * 120 / self.speed_var), len(notes) - 1)]
        elif play_pos is None and self.note_var is not None:
            note = self.note_var
        else:
            return

        key = note % 12
        x = self.x
        y = self.y + (59 - note) * 2

        if note == -1:
            pyxel.rect(x, y + 1, 12, 2, PIANO_KEYBOARD_REST_COLOR)
        elif key in _BOTTOM_WHITE_KEYS:
            pyxel.rect(x, y + 1, 7, 1, PIANO_KEYBOARD_PLAY_COLOR)
            pyxel.rect(x + 7, y, 5, 2, PIANO_KEYBOARD_PLAY_COLOR)
        elif key in _TOP_WHITE_KEYS:
            pyxel.rect(x, y + 1, 7, 1, PIANO_KEYBOARD_PLAY_COLOR)
            pyxel.rect(x + 7, y + 1, 5, 2, PIANO_KEYBOARD_PLAY_COLOR)
        elif key in _FULL_WHITE_KEYS:
            pyxel.rect(x, y + 1, 7, 1, PIANO_KEYBOARD_PLAY_COLOR)
            pyxel.rect(x + 7, y, 5, 3, PIANO_KEYBOARD_PLAY_COLOR)
        else:
            pyxel.rect(x, y + 1, 6, 1, PIANO_KEYBOARD_PLAY_COLOR)
