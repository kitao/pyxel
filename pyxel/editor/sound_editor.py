import pyxel
from pyxel.constants import AUDIO_SOUND_COUNT
from pyxel.ui import NumberPicker, ScrollBar
from pyxel.ui.constants import WIDGET_FRAME_COLOR

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y
from .editor import Editor


class SoundEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self._sound_picker = NumberPicker(self, 45, 17, 0, AUDIO_SOUND_COUNT - 1, 0)
        self._speed_picker = NumberPicker(self, 105, 17, 1, 99, 0)

        self._v_scroll_bar = ScrollBar(
            self, 222, 24, 119, ScrollBar.VERTICAL, 239, 117, 60, with_shadow=False
        )
        self._h_scroll_bar = ScrollBar(
            self, 30, 166, 193, ScrollBar.HORIZONTAL, 8, 4, 0, with_shadow=False
        )

        self.add_event_handler("draw", self.__on_draw)

    @property
    def sound(self):
        return self._sound_picker.value

    @property
    def speed(self):
        return self._speed_picker.value

    def __on_draw(self):
        self.draw_frame(11, 16, 218, 157)
        pyxel.text(23, 18, "SOUND", 6)
        pyxel.text(83, 18, "SPEED", 6)

        offset = (-self._v_scroll_bar.value) % 48 - 48

        pyxel.clip(12, 25, 221, 141)

        pyxel.blt(12, offset + 25, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 8, 115, 96)
        pyxel.blt(127, offset + 25, 3, EDITOR_IMAGE_X + 19, EDITOR_IMAGE_Y + 8, 96, 96)
        pyxel.blt(12, offset + 121, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 8, 115, 96)
        pyxel.blt(127, offset + 121, 3, EDITOR_IMAGE_X + 19, EDITOR_IMAGE_Y + 8, 96, 96)

        sound = pyxel.sound(self.sound)
        offset_x = self._h_scroll_bar.value * 8

        for i, note in enumerate(sound.note[offset_x : offset_x + 32]):
            x = i * 6 + 31
            y = 261 - note * 4 - self._v_scroll_bar.value
            pyxel.rect(x + 1, y, x + 4, y + 2, 2)

        pyxel.clip()

        pyxel.blt(31, 143, 3, EDITOR_IMAGE_X + 19, EDITOR_IMAGE_Y + 104, 96, 23)
        pyxel.blt(127, 143, 3, EDITOR_IMAGE_X + 19, EDITOR_IMAGE_Y + 104, 95, 23)

        pyxel.text(17, 144, "TON", 6)
        pyxel.text(17, 152, "VOL", 6)
        pyxel.text(17, 160, "EFX", 6)

        for i, tone in enumerate(sound.tone[offset_x : offset_x + 32]):
            pyxel.text(32 + i * 6, 144, "TSPN"[tone], 1)

        for i, volume in enumerate(sound.volume[offset_x : offset_x + 32]):
            pyxel.text(32 + i * 6, 152, str(volume), 1)

        for i, effect in enumerate(sound.effect[offset_x : offset_x + 32]):
            pyxel.text(32 + i * 6, 160, "NSVF"[effect], 1)
