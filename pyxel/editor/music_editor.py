import pyxel
from pyxel.ui import ImageButton, ImageToggleButton, NumberPicker

from .constants import (
    EDITOR_IMAGE_X,
    EDITOR_IMAGE_Y,
    MAX_MUSIC_LENGTH,
    MUSIC_BANK_COUNT,
    MUSIC_CHANNEL_COUNT,
)
from .editor import Editor
from .field_cursor import FieldCursor
from .music_field import MusicField
from .sound_selector import SoundSelector


class MusicEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self._is_playing = False
        self._play_pos = [0 for _ in range(MUSIC_CHANNEL_COUNT)]
        self.field_cursor = FieldCursor(
            self.get_data,
            self.add_pre_history,
            self.add_post_history,
            MAX_MUSIC_LENGTH,
            16,
            MUSIC_CHANNEL_COUNT,
        )
        self._music_picker = NumberPicker(self, 45, 17, 0, MUSIC_BANK_COUNT - 1, 0)
        self._play_button = ImageButton(
            self, 185, 17, 3, EDITOR_IMAGE_X + 126, EDITOR_IMAGE_Y
        )
        self._stop_button = ImageButton(
            self, 195, 17, 3, EDITOR_IMAGE_X + 135, EDITOR_IMAGE_Y
        )
        self._loop_button = ImageToggleButton(
            self, 205, 17, 3, EDITOR_IMAGE_X + 144, EDITOR_IMAGE_Y
        )
        self._music_field = [MusicField(self, 11, 29 + i * 25, i) for i in range(4)]
        self._sound_selector = SoundSelector(self)

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)
        self.add_event_handler("hide", self.__on_hide)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
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
        self.add_number_picker_help(self._music_picker)

    @property
    def music(self):
        return self._music_picker.value

    @property
    def is_playing(self):
        return self._is_playing

    def play_pos(self, ch):
        return self._play_pos[ch]

    def get_data(self, value):
        music = pyxel.music(self._music_picker.value)

        if value == 0:
            data = music.ch0
        elif value == 1:
            data = music.ch1
        elif value == 2:
            data = music.ch2
        elif value == 3:
            data = music.ch3

        return data

    def add_pre_history(self, x, y):
        self._history_data = data = {}
        data["music"] = self._music_picker.value
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

        for i in range(MUSIC_CHANNEL_COUNT):
            self._play_pos[i] = 0

        self._music_picker.is_enabled = False
        self._play_button.is_enabled = False
        self._stop_button.is_enabled = True
        self._loop_button.is_enabled = False

        pyxel.playm(self._music_picker.value, loop=self._loop_button.value)

    def _stop(self):
        self._is_playing = False

        for i in range(MUSIC_CHANNEL_COUNT):
            self._play_pos[i] = -1

        self._music_picker.is_enabled = True
        self._play_button.is_enabled = True
        self._stop_button.is_enabled = False
        self._loop_button.is_enabled = True

        pyxel.stop()

    def __on_undo(self, data):
        self._music_picker.value = data["music"]
        self.field_cursor.move(*data["cursor_before"])
        self.field_cursor.data[:] = data["before"]

    def __on_redo(self, data):
        dat = data["after"]
        dat_len = len(dat)

        self._music_picker.value = data["music"]
        self.field_cursor.move(*data["cursor_after"])
        self.field_cursor.data[:dat_len] = dat
        self.field_cursor.data_lengh = dat_len

    def __on_hide(self):
        self._stop()

    def __on_update(self):
        if self._is_playing:
            self._is_playing = False

            for i in range(MUSIC_CHANNEL_COUNT):
                if pyxel.play_pos(i) >= 0:
                    self._is_playing = True
                    play_pos = pyxel.play_pos(i)
                    self._play_pos[i] = play_pos // 100 if play_pos >= 0 else -1
                else:
                    self._play_pos[i] = -1

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

        self.field_cursor.process_input()

    def __on_draw(self):
        self.draw_panel(11, 16, 218, 9)
        pyxel.text(23, 18, "MUSIC", 6)

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
