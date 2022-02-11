import pyxel

from .settings import (
    EDITOR_IMAGE,
    MUSIC_FIELD_BACKGROUND_COLOR,
    MUSIC_FIELD_CURSOR_EDIT_COLOR,
    MUSIC_FIELD_CURSOR_PLAY_COLOR,
    MUSIC_FIELD_CURSOR_SELECT_COLOR,
    MUSIC_FIELD_SOUND_FOCUS_COLOR,
    MUSIC_FIELD_SOUND_NORMAL_COLOR,
    TEXT_LABEL_COLOR,
)
from .widgets import Widget


class MusicField(Widget):
    """
    Variables:
        is_playing_var
        help_message_var
    """

    def __init__(self, parent, x, y, ch):
        super().__init__(parent, x, y, 218, 21)
        self._ch = ch
        self.field_cursor = parent.field_cursor
        self.get_field = parent.get_field
        self.copy_var("is_playing_var", parent)
        self.copy_var("help_message_var", parent)

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("draw", self.__on_draw)

    @property
    def data(self):
        return self.get_field(self._ch)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT or self.is_playing_var:
            return
        x -= self.x + 21
        y -= self.y + 2
        if x < 0 or y < 0 or x > 188 or y > 16 or x % 12 > 8 or y % 10 > 6:
            return
        self.field_cursor.move_to(
            x // 12 + (y // 10) * 16, self._ch, pyxel.btn(pyxel.KEY_SHIFT)
        )

    def __on_mouse_hover(self, x, y):
        self.help_message_var = "SOUND:SOUND_BUTTON/BS/DEL"

    def __on_draw(self):
        # Draw frame
        self.draw_panel(self.x, self.y, self.width, self.height)
        pyxel.text(self.x + 5, self.y + 8, f"CH{self._ch}", TEXT_LABEL_COLOR)
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

        # Draw cursor
        if self.is_playing_var:
            play_pos = pyxel.play_pos(self._ch)
            if play_pos is None:
                cursor_x = -1
                cursor_y = -1
            else:
                cursor_x = play_pos[0]
                cursor_y = self._ch
                cursor_width = 1
                cursor_col = MUSIC_FIELD_CURSOR_PLAY_COLOR
        else:
            cursor_x = self.field_cursor.x
            cursor_y = self.field_cursor.y
            cursor_width = self.field_cursor.width
            cursor_col = (
                MUSIC_FIELD_CURSOR_SELECT_COLOR
                if self.field_cursor.is_selecting
                else MUSIC_FIELD_CURSOR_EDIT_COLOR
            )
        if cursor_y == self._ch:
            for i in range(len(self.data) + 1):
                if cursor_x <= i < cursor_x + cursor_width:
                    x = self.x + (i % 16) * 12 + 21
                    y = self.y + (cursor_y - self._ch + i // 16) * 10 + 2
                    pyxel.rect(x, y, 9, 7, cursor_col)

        # Draw sounds
        for i in range(len(self.data)):
            x = self.x + 22 + (i % 16) * 12
            y = self.y + (i // 16) * 10 + 3
            col = (
                MUSIC_FIELD_SOUND_FOCUS_COLOR
                if cursor_y == self._ch and cursor_x <= i < cursor_x + cursor_width
                else MUSIC_FIELD_SOUND_NORMAL_COLOR
            )
            pyxel.text(x, y, f"{self.data[i]:0>2}", col)
