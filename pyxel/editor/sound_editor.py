import pyxel
from pyxel.constants import AUDIO_SOUND_COUNT
from pyxel.ui import NumberPicker, ScrollBar

from .editor import Editor


class SoundEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self._sound_picker = NumberPicker(self, 45, 17, 0, AUDIO_SOUND_COUNT - 1, 0)
        self._speed_picker = NumberPicker(self, 105, 17, 0, 99, 0)

        self._scroll_var = ScrollBar(
            self, 222, 24, 125, ScrollBar.VERTICAL, 100, 10, 0, with_shadow=False
        )

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        self.draw_frame(11, 16, 218, 157)
        pyxel.text(23, 18, "SOUND", 6)
        pyxel.text(83, 18, "SPEED", 6)

        pyxel.blt(12, 25, 3, 0, 24, 19, 123)

        for i in range(4):
            pyxel.blt(31 + i * 48, 25, 3, 19, 24, 48, 147)

        pyxel.text(17, 150, "TON", 6)
        pyxel.text(17, 158, "VOL", 6)
        pyxel.text(17, 166, "EFX", 6)

        self.draw_not_implemented_message()
