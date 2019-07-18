import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y, MAX_MUSIC_LENGTH


class MusicField(Widget):
    def __init__(self, parent, x, y, ch):
        super().__init__(parent, x, y, 218, 21)

        self._ch = ch

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("draw", self.__on_draw)

    @property
    def data(self):
        return self.parent.get_data(self._ch)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_LEFT_BUTTON or self.parent.is_playing:
            return

        x -= self.x + 21
        y -= self.y + 2

        if x < 0 or y < 0 or x > 188 or y > 16 or x % 12 > 8 or y % 10 > 6:
            return

        self.parent.field_cursor.move(x // 12 + (y // 10) * 16, self._ch)

    def __on_mouse_hover(self, x, y):
        self.parent.help_message = "SOUND:SOUND_BUTTON/BS/DEL"

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        pyxel.text(self.x + 5, self.y + 8, "CH{}".format(self._ch), 6)
        pyxel.blt(
            self.x + 20,
            self.y + 1,
            pyxel.IMAGE_BANK_FOR_SYSTEM,
            EDITOR_IMAGE_X,
            EDITOR_IMAGE_Y + 102,
            191,
            19,
            6,
        )

        data = self.data

        if self.parent.is_playing:
            play_pos = self.parent.play_pos(self._ch)

            if play_pos < 0 or not data:
                cursor_x = -1
                cursor_y = -1
            else:
                cursor_x = play_pos
                cursor_y = self._ch
                cursor_col = 8
        else:
            cursor_x = self.parent.field_cursor.x
            cursor_y = self.parent.field_cursor.y
            cursor_col = 1

        if cursor_y == self._ch:
            x = self.x + (cursor_x % 16) * 12 + 21
            y = self.y + (cursor_y - self._ch + cursor_x // 16) * 10 + 2
            pyxel.rect(x, y, 9, 7, cursor_col)

        for i in range(MAX_MUSIC_LENGTH):
            if i >= len(data):
                break

            x = self.x + 22 + (i % 16) * 12
            y = self.y + (i // 16) * 10 + 3
            col = 7 if cursor_y == self._ch and cursor_x == i else 1
            pyxel.text(x, y, "{:0>2}".format(data[i]), col)
