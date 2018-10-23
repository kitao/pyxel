import pyxel
from pyxel.ui import Widget
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y, MUSIC_MAX_LENGTH


class MusicField(Widget):
    def __init__(self, parent, x, y, ch):
        super().__init__(parent, x, y, 218, 21)

        self._ch = ch

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

        self.data.extend([0, 1, 2, 3, 4] * 6)

    @property
    def data(self):
        music = pyxel.music(self.parent.music)

        if self._ch == 0:
            data = music.ch0
        elif self._ch == 1:
            data = music.ch1
        elif self._ch == 2:
            data = music.ch2
        elif self._ch == 3:
            data = music.ch3

        return data

    def __on_update(self):
        cursor_y = self.parent.cursor_y

        if cursor_y != self._ch:
            return

        edit_x = self.parent.edit_x
        data = self.data

        if pyxel.btnp(pyxel.KEY_BACKSPACE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            if edit_x > 0:
                # self.parent.add_edit_history_before()
                del data[edit_x - 1]
                self.parent.cursor_x = edit_x - 1
                # self.parent.add_edit_history_after()
            return

        if pyxel.btnp(pyxel.KEY_DELETE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            if edit_x < len(data):
                # self.parent.add_edit_history_before()
                del data[edit_x]
                # self.parent.add_edit_history_after()
            return

        if pyxel.btnp(
            pyxel.KEY_ENTER, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
        ) or pyxel.btnp(pyxel.KEY_KP_ENTER, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            # self.parent.add_edit_history_before()

            data.insert(edit_x, 0)
            data[:] = data[:MUSIC_MAX_LENGTH]

            self.parent.cursor_x = edit_x
            if edit_x < MUSIC_MAX_LENGTH - 1:
                self.parent.cursor_x += 1

            # self.parent.add_edit_history_after()
            return

    def __on_draw(self):
        self.draw_panel(self.x, self.y, self.width, self.height)

        pyxel.text(self.x + 5, self.y + 8, "CH{}".format(self._ch), 6)
        pyxel.blt(
            self.x + 20, self.y + 1, 3, EDITOR_IMAGE_X, EDITOR_IMAGE_Y + 102, 191, 19, 6
        )

        cursor_x = self.parent.cursor_x
        cursor_y = self.parent.cursor_y

        if cursor_y == self._ch:
            x = self.x + (self.parent.edit_x % 16) * 12 + 21
            y = self.y + (cursor_y - self._ch + self.parent.edit_x // 16) * 10 + 2
            pyxel.rect(x, y, x + 8, y + 6, 1)

        data = self.data

        for i in range(min(len(data), 16)):
            col = 7 if cursor_y == self._ch and cursor_x == i else 1
            pyxel.text(self.x + 22 + i * 12, self.y + 3, "{:0>2}".format(data[i]), col)

        for i in range(len(data) - 16):
            col = 7 if cursor_y == self._ch and cursor_x == i + 16 else 1
            pyxel.text(
                self.x + 22 + i * 12, self.y + 13, "{:0>2}".format(data[i + 16]), col
            )
