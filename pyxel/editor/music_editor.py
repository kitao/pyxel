import pyxel
from pyxel.constants import AUDIO_MUSIC_COUNT
from pyxel.ui import ImageButton, NumberPicker
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y
from .editor import Editor
from .music_field import MusicField
from .sound_selector import SoundSelector


class MusicEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self.cursor_x = 0
        self.cursor_y = 0
        self._music_picker = NumberPicker(self, 45, 17, 0, AUDIO_MUSIC_COUNT - 1, 0)
        self._play_button = ImageButton(
            self, 185, 17, 3, EDITOR_IMAGE_X + 126, EDITOR_IMAGE_Y
        )
        self._stop_button = ImageButton(
            self, 195, 17, 3, EDITOR_IMAGE_X + 135, EDITOR_IMAGE_Y
        )
        self._loop_button = ImageButton(
            self, 205, 17, 3, EDITOR_IMAGE_X + 144, EDITOR_IMAGE_Y
        )
        self._sound_selector = SoundSelector(self)
        self._music_field = [MusicField(self, 11, 29 + i * 25, i) for i in range(4)]

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    @property
    def music(self):
        return self._music_picker.value

    @property
    def data(self):
        music = pyxel.music(self._music_picker.value)

        if self.cursor_y == 0:
            data = music.ch0
        elif self.cursor_y == 1:
            data = music.ch1
        elif self.cursor_y == 2:
            data = music.ch2
        elif self.cursor_y == 3:
            data = music.ch3

        return data

    @property
    def max_edit_x(self):
        return min(len(self.data), 31)

    @property
    def edit_x(self):
        return min(self.cursor_x, self.max_edit_x)

    def __on_update(self):
        if self.cursor_x >= 16 and len(self.data) < 15:
            self.cursor_x -= 16

        if self.cursor_x > 0 and pyxel.btnp(
            pyxel.KEY_LEFT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self.cursor_x = self.edit_x - 1

        if self.cursor_x < self.max_edit_x and pyxel.btnp(
            pyxel.KEY_RIGHT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self.cursor_x += 1

        if pyxel.btnp(pyxel.KEY_UP, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            if self.cursor_x >= 16:
                self.cursor_x -= 16
            elif self.cursor_y > 0:
                self.cursor_y -= 1
                if self.cursor_x < 16 and len(self.data) >= 15:
                    self.cursor_x += 16

        if pyxel.btnp(pyxel.KEY_DOWN, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            if self.cursor_x < 16 and len(self.data) >= 15:
                self.cursor_x += 16
            elif self.cursor_y < 3:
                self.cursor_y += 1
                if self.cursor_x >= 16:
                    self.cursor_x -= 16

    def __on_draw(self):
        self.draw_panel(11, 16, 218, 9)
        pyxel.text(23, 18, "MUSIC", 6)
