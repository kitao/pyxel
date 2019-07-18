import pyxel
from pyxel.ui import ImageButton, ImageToggleButton, NumberPicker

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y, MAX_SOUND_LENGTH
from .editor import Editor
from .field_cursor import FieldCursor
from .octave_bar import OctaveBar
from .piano_keyboard import PianoKeyboard
from .piano_roll import PianoRoll
from .sound_field import SoundField


class SoundEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self.field_cursor = FieldCursor(
            self.get_data,
            self.add_pre_history,
            self.add_post_history,
            MAX_SOUND_LENGTH,
            MAX_SOUND_LENGTH,
            4,
        )
        self.octave = 2
        self._is_playing = False
        self._play_pos = -1
        self._history_data = None
        self._sound_picker = NumberPicker(
            self, 45, 17, 0, pyxel.SOUND_BANK_COUNT - 1, 0
        )
        self._speed_picker = NumberPicker(self, 105, 17, 1, 99, pyxel.sound(0).speed)
        self._play_button = ImageButton(
            self, 185, 17, 3, EDITOR_IMAGE_X + 126, EDITOR_IMAGE_Y
        )
        self._stop_button = ImageButton(
            self, 195, 17, 3, EDITOR_IMAGE_X + 135, EDITOR_IMAGE_Y, is_enabled=False
        )
        self._loop_button = ImageToggleButton(
            self, 205, 17, 3, EDITOR_IMAGE_X + 144, EDITOR_IMAGE_Y
        )
        self._piano_keyboard = PianoKeyboard(self)
        self._piano_roll = PianoRoll(self)
        self._sound_field = SoundField(self)
        self._left_octave_bar = OctaveBar(self, 12, 25)
        self._right_octave_bar = OctaveBar(self, 224, 25)

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)
        self.add_event_handler("hide", self.__on_hide)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
        self._sound_picker.add_event_handler("change", self.__on_sound_picker_change)
        self._speed_picker.add_event_handler("change", self.__on_speed_picker_change)
        self._play_button.add_event_handler("press", self.__on_play_button_press)
        self._stop_button.add_event_handler("press", self.__on_stop_button_press)
        self._play_button.add_event_handler(
            "mouse_hover", self.__on_play_button_mouse_hover
        )
        self._stop_button.add_event_handler(
            "mouse_hover", self.__on_stop_button_mouse_hover
        )
        self._loop_button.add_event_handler(
            "mouse_hover", self.__on_loop_button_mouse_hover
        )
        self.add_number_picker_help(self._sound_picker)
        self.add_number_picker_help(self._speed_picker)

    @property
    def keyboard_note(self):
        return self._piano_keyboard.note

    @property
    def is_playing(self):
        return self._is_playing

    @property
    def play_pos(self):
        return self._play_pos

    def get_data(self, index):
        sound = pyxel.sound(self._sound_picker.value)

        if index == 0:
            data = sound.note
        elif index == 1:
            data = sound.tone
        elif index == 2:
            data = sound.volume
        elif index == 3:
            data = sound.effect

        return data

    def add_pre_history(self, x, y):
        self._history_data = data = {}
        data["sound"] = self._sound_picker.value
        data["cursor_before"] = (x, y)
        data["before"] = self.field_cursor.data[:]

    def add_post_history(self, x, y):
        data = self._history_data
        data["cursor_after"] = (x, y)
        data["after"] = self.field_cursor.data[:]

        if data["before"] != data["after"]:
            self.add_history(self._history_data)

    def _play(self):
        self._is_playing = True
        self._play_pos = 0
        self._sound_picker.is_enabled = False
        self._speed_picker.is_enabled = False
        self._play_button.is_enabled = False
        self._stop_button.is_enabled = True
        self._loop_button.is_enabled = False

        pyxel.play(0, self._sound_picker.value, loop=self._loop_button.value)

    def _stop(self):
        self._is_playing = None
        self._play_pos = -1
        self._sound_picker.is_enabled = True
        self._speed_picker.is_enabled = True
        self._play_button.is_enabled = True
        self._stop_button.is_enabled = False
        self._loop_button.is_enabled = True

        pyxel.stop(0)

    def __on_undo(self, data):
        self._sound_picker.value = data["sound"]
        self.field_cursor.move(*data["cursor_before"])
        self.field_cursor.data[:] = data["before"]

    def __on_redo(self, data):
        self._sound_picker.value = data["sound"]
        self.field_cursor.move(*data["cursor_after"])
        self.field_cursor.data[:] = data["after"]

    def __on_hide(self):
        self._stop()

    def __on_update(self):
        self._is_playing = pyxel.play_pos(0) >= 0
        self._play_pos = pyxel.play_pos(0)

        sound = pyxel.sound(self._sound_picker.value)
        if self._speed_picker.value != sound.speed:
            self._speed_picker.value = sound.speed

        if pyxel.btnp(pyxel.KEY_SPACE):
            if self._is_playing:
                self._stop_button.press()
            else:
                self._play_button.press()

        if self._is_playing:
            return

        if not self._play_button.is_enabled:
            self._stop()

        if self._loop_button.is_enabled and pyxel.btnp(pyxel.KEY_L):
            self._loop_button.press()

        if pyxel.btnp(pyxel.KEY_PAGE_UP):
            self.octave = min(self.octave + 1, 3)

        if pyxel.btnp(pyxel.KEY_PAGE_DOWN):
            self.octave = max(self.octave - 1, 0)

        self.field_cursor.process_input()

    def __on_draw(self):
        self.draw_panel(11, 16, 218, 157)
        pyxel.text(23, 18, "SOUND", 6)
        pyxel.text(83, 18, "SPEED", 6)

    def __on_sound_picker_change(self, value):
        sound = pyxel.sound(value)
        self._speed_picker.value = sound.speed

    def __on_speed_picker_change(self, value):
        sound = pyxel.sound(self._sound_picker.value)
        sound.speed = value

    def __on_play_button_press(self):
        self._play()

    def __on_stop_button_press(self):
        self._stop()

    def __on_play_button_mouse_hover(self, x, y):
        self.parent.help_message = "PLAY:SPACE"

    def __on_stop_button_mouse_hover(self, x, y):
        self.parent.help_message = "STOP:SPACE"

    def __on_loop_button_mouse_hover(self, x, y):
        self.parent.help_message = "LOOP:L"
