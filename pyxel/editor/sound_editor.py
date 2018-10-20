import pyxel
from pyxel.constants import AUDIO_SOUND_COUNT, SOUND_MAX_LENGTH
from pyxel.ui import ImageButton, NumberPicker
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y
from .editor import Editor
from .octave_bar import OctaveBar
from .piano_keyboard import PianoKeyboard
from .piano_roll import PianoRoll
from .sound_input import SoundInput


class SoundEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self.cursor_x = 0
        self.cursor_y = 0

        self._sound_picker = NumberPicker(self, 45, 17, 0, AUDIO_SOUND_COUNT - 1, 0)
        self._speed_picker = NumberPicker(self, 105, 17, 1, 99, 0)

        self._play_button = ImageButton(
            self, 185, 17, 3, EDITOR_IMAGE_X + 126, EDITOR_IMAGE_Y
        )
        self._stop_button = ImageButton(
            self, 195, 17, 3, EDITOR_IMAGE_X + 135, EDITOR_IMAGE_Y
        )
        self._loop_button = ImageButton(
            self, 205, 17, 3, EDITOR_IMAGE_X + 144, EDITOR_IMAGE_Y
        )

        self._piano_keyboard = PianoKeyboard(self)
        self._piano_roll = PianoRoll(self)
        self._sound_input = SoundInput(self)

        self._left_octave_bar = OctaveBar(self, 12, 25)
        self._right_octave_bar = OctaveBar(self, 224, 25)

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
        self._play_button.add_event_handler("press", self.__on_play_press)
        self._stop_button.add_event_handler("press", self.__on_stop_press)

    @property
    def sound(self):
        return self._sound_picker.value

    @property
    def speed(self):
        return self._speed_picker.value

    @property
    def sound_data(self):
        sound = pyxel.sound(self._sound_picker.value)

        if self.cursor_y == 0:
            data = sound.note
        elif self.cursor_y == 1:
            data = sound.tone
        elif self.cursor_y == 2:
            data = sound.volume
        elif self.cursor_y == 3:
            data = sound.effect

        return data

    @property
    def max_edit_x(self):
        return min(len(self.sound_data), SOUND_MAX_LENGTH - 1)

    @property
    def edit_x(self):
        return min(self.cursor_x, self.max_edit_x)

    def __on_update(self):
        if self.cursor_x > 0 and pyxel.btnp(
            pyxel.KEY_LEFT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self.cursor_x = self.edit_x - 1

        if self.cursor_x < self.max_edit_x and pyxel.btnp(
            pyxel.KEY_RIGHT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self.cursor_x += 1

        if self.cursor_y > 0 and pyxel.btnp(
            pyxel.KEY_UP, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self.cursor_y -= 1

        if self.cursor_y < 3 and pyxel.btnp(
            pyxel.KEY_DOWN, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self.cursor_y += 1

        if pyxel.btnp(pyxel.KEY_SPACE):
            self._play_button.press()

    def __on_draw(self):
        self.draw_frame(11, 16, 218, 157)
        pyxel.text(23, 18, "SOUND", 6)
        pyxel.text(83, 18, "SPEED", 6)
        pyxel.text(17, 150, "TON", 6)
        pyxel.text(17, 158, "VOL", 6)
        pyxel.text(17, 166, "EFX", 6)

    def __on_play_press(self):
        pyxel.play(0, self._sound_picker.value)

    def __on_stop_press(self):
        pyxel.stop(0)
