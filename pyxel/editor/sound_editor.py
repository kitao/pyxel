import pyxel
from pyxel.constants import AUDIO_SOUND_COUNT
from pyxel.ui import ImageButton, NumberPicker
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y
from .editor import Editor


class SoundEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self._cursor_x = 0
        self._cursor_y = 0

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

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    @property
    def sound(self):
        return self._sound_picker.value

    @property
    def speed(self):
        return self._speed_picker.value

    def __on_update(self):
        if self._cursor_x > 0 and pyxel.btnp(
            pyxel.KEY_LEFT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self._cursor_x -= 1

        if self._cursor_x < 47 and pyxel.btnp(
            pyxel.KEY_RIGHT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self._cursor_x += 1

        if self._cursor_y > 0 and pyxel.btnp(
            pyxel.KEY_UP, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self._cursor_y -= 1

        if self._cursor_y < 3 and pyxel.btnp(
            pyxel.KEY_DOWN, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ):
            self._cursor_y += 1

    def __on_draw(self):
        self.draw_frame(11, 16, 218, 157)
        pyxel.rect(12, 25, 227, 147, 6)

        pyxel.text(23, 18, "SOUND", 6)
        pyxel.text(83, 18, "SPEED", 6)

        x = self._cursor_x * 4 + 31
        if self._cursor_y == 0:
            pyxel.rect(x, 25, x + 2, 147, 1)

        pyxel.blt(16, 25, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 8, 110, 147, 6)
        pyxel.blt(126, 25, 3, EDITOR_IMAGE_X + 13, EDITOR_IMAGE_Y + 8, -98, 147, 6)

        if self._cursor_y > 0:
            y = self._cursor_y * 8 + 142
            pyxel.rect(x, y - 1, x + 2, y + 5, 1)

        sound = pyxel.sound(self.sound)

        for i, note in enumerate(sound.note):
            x = i * 4 + 31
            y = 143 - note * 2
            pyxel.rect(x, y, x + 2, y + 2, 8 if note >= 0 else 12)

        pyxel.text(17, 150, "TON", 6)
        pyxel.text(17, 158, "VOL", 6)
        pyxel.text(17, 166, "EFX", 6)

        pyxel.rect(13, 26, 14, 72, 13)
        pyxel.rect(225, 26, 226, 72, 13)

        for i, tone in enumerate(sound.tone):
            col = 7 if self._cursor_y == 1 and self._cursor_x == i else 1
            pyxel.text(31 + i * 4, 150, "TSPN"[tone], col)

        for i, volume in enumerate(sound.volume):
            col = 7 if self._cursor_y == 2 and self._cursor_x == i else 1
            pyxel.text(31 + i * 4, 158, str(volume), col)

        for i, effect in enumerate(sound.effect):
            col = 7 if self._cursor_y == 3 and self._cursor_x == i else 1
            pyxel.text(31 + i * 4, 166, "NSVF"[effect], col)
