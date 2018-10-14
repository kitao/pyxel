import pyxel
from pyxel.constants import AUDIO_SOUND_COUNT
from pyxel.ui import ImageButton, NumberPicker

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y
from .editor import Editor


class SoundEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

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

        pyxel.blt(17, 25, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 8, 109, 147)
        pyxel.blt(126, 25, 3, EDITOR_IMAGE_X + 13, EDITOR_IMAGE_Y + 8, -97, 147)
        pyxel.rect(12, 25, 15, 147, 6)
        pyxel.rect(224, 25, 227, 147, 6)

        sound = pyxel.sound(self.sound)

        for i, note in enumerate(sound.note):
            x = i * 4 + 31
            y = 143 - note * 2
            pyxel.rect(x, y, x + 2, y + 2, 8 if note >= 0 else 12)

        pyxel.text(17, 150, "TON", 6)
        pyxel.text(17, 158, "VOL", 6)
        pyxel.text(17, 166, "EFX", 6)

        for i, tone in enumerate(sound.tone):
            pyxel.text(31 + i * 4, 150, "TSPN"[tone], 1)

        for i, volume in enumerate(sound.volume):
            pyxel.text(31 + i * 4, 158, str(volume), 1)

        for i, effect in enumerate(sound.effect):
            pyxel.text(31 + i * 4, 166, "NSVF"[effect], 1)
