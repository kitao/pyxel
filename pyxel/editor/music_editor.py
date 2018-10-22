import pyxel
from pyxel.constants import AUDIO_MUSIC_COUNT
from pyxel.ui import ImageButton, NumberPicker

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y
from .editor import Editor


class MusicEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

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

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        self.draw_frame(11, 16, 218, 9)
        pyxel.text(23, 18, "MUSIC", 6)

        for i in range(4):
            self.draw_frame(11, 29 + i * 25, 218, 21)
            pyxel.blt(
                32, 30 + i * 25, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 102, 190, 19, 6
            )
            pyxel.text(16, 37 + i * 25, "CH{}".format(i), 6)

        self.draw_frame(11, 129, 218, 44)

        pyxel.blt(17, 134, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 121, 206, 34, 6)
