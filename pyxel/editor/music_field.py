import pyxel

from .settings import (
    EDITOR_IMAGE,
    MAX_MUSIC_LENGTH,
    MUSIC_FIELD_BACKGROUND_COLOR,
    MUSIC_FIELD_CURSOR_EDIT_COLOR,
    MUSIC_FIELD_CURSOR_PLAY_COLOR,
    MUSIC_FIELD_SOUND_FOCUS_COLOR,
    MUSIC_FIELD_SOUND_NORMAL_COLOR,
    TEXT_LABEL_COLOR,
)
from .widgets import Widget


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
        if key != pyxel.MOUSE_BUTTON_LEFT or self.parent.is_playing:
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

        pyxel.text(self.x + 5, self.y + 8, "CH{}".format(self._ch), TEXT_LABEL_COLOR)
        pyxel.blt(
            self.x + 20,
            self.y + 1,
            EDITOR_IMAGE,
            0,
            102,
            191,
            19,
            MUSIC_FIELD_BACKGROUND_COLOR,
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
                cursor_col = MUSIC_FIELD_CURSOR_PLAY_COLOR
        else:
            cursor_x = self.parent.field_cursor.x
            cursor_y = self.parent.field_cursor.y
            cursor_col = MUSIC_FIELD_CURSOR_EDIT_COLOR

        if cursor_y == self._ch:
            x = self.x + (cursor_x % 16) * 12 + 21
            y = self.y + (cursor_y - self._ch + cursor_x // 16) * 10 + 2
            pyxel.rect(x, y, 9, 7, cursor_col)

        for i in range(MAX_MUSIC_LENGTH):
            if i >= len(data):
                break

            x = self.x + 22 + (i % 16) * 12
            y = self.y + (i // 16) * 10 + 3
            col = (
                MUSIC_FIELD_SOUND_FOCUS_COLOR
                if cursor_y == self._ch and cursor_x == i
                else MUSIC_FIELD_SOUND_NORMAL_COLOR
            )
            pyxel.text(x, y, "{:0>2}".format(data[i]), col)
