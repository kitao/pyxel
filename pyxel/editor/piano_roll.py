import pyxel

from .settings import (
    EDITOR_IMAGE,
    MAX_SOUND_LENGTH,
    PIANO_ROLL_BACKGROUND_COLOR,
    PIANO_ROLL_CURSOR_EDIT_COLOR,
    PIANO_ROLL_CURSOR_PLAY_COLOR,
    PIANO_ROLL_NOTE_COLOR,
    PIANO_ROLL_REST_COLOR,
)
from .widgets import Widget
from .widgets.settings import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME


class PianoRoll(Widget):
    """
    Variables:
        note_var
        is_playing_var
        help_message_var
    """

    def __init__(self, parent):
        super().__init__(parent, 30, 25, 193, 123)
        self._press_x = 0
        self._press_y = 0
        self.field_cursor = parent.field_cursor
        self.get_field = parent.get_field
        self.add_pre_history = parent.add_pre_history
        self.add_post_history = parent.add_post_history
        self.copy_var("note_var", parent)
        self.copy_var("is_playing_var", parent)
        self.copy_var("help_message_var", parent)

        # Set event listeners
        self.add_event_listener("mouse_down", self.__on_mouse_down)
        self.add_event_listener("mouse_drag", self.__on_mouse_drag)
        self.add_event_listener("mouse_click", self.__on_mouse_click)
        self.add_event_listener("mouse_hover", self.__on_mouse_hover)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    def _screen_to_view(self, x, y):
        x = min(max((x - self.x - 1) // 4, 0), MAX_SOUND_LENGTH - 1)
        y = min(max(59 - (y - self.y - 1) // 2, -1), 59)
        return x, y

    def _set_note(self, x, y):
        self.add_pre_history(x, 0)
        self.field_cursor.move_to(x, 0)
        field = self.field_cursor.field
        field_len = len(field)
        if x < field_len:
            field[x] = y
        else:
            lst = field.to_list()
            lst.extend([-1] * (x - field_len) + [y])
            field.from_list(lst)
        self.add_post_history(x, 0)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT or self.is_playing_var:
            return
        x, y = self._screen_to_view(x, y)
        self._press_x = x
        self._press_y = y
        self._set_note(x, y)

    def __on_mouse_drag(self, key, x, y, dx, dy):
        if key != pyxel.MOUSE_BUTTON_LEFT or self.is_playing_var:
            return
        x, y = self._screen_to_view(x, y)
        if x > self._press_x:
            step = 1
        elif x < self._press_x:
            step = -1
        else:
            self._set_note(x, y)
            return
        dx = x - self._press_x
        dy = y - self._press_y
        alpha = dy / dx
        for i in range(step, dx + step, step):
            self._set_note(self._press_x + i, round(self._press_y + alpha * i))
        self._press_x = x
        self._press_y = y

    def __on_mouse_click(self, key, x, y):
        if key != pyxel.MOUSE_BUTTON_LEFT or self.is_playing_var:
            return
        x, y = self._screen_to_view(x, y)
        self.field_cursor.move_to(x, 0)
        field = self.field_cursor.field
        self.add_pre_history(x, 0)
        padding_length = x + 1 - len(field)
        if padding_length > 0:
            list = field.to_list()
            list.extend([-1] * padding_length)
            field.from_list(list)
        field[x] = y
        self.add_post_history(x, 0)

    def __on_mouse_hover(self, x, y):
        self.help_message_var = "NOTE:CLICK/PIANO_KEY+ENTER/BS/DEL"

    def __on_update(self):
        if self.field_cursor.y > 0 or self.is_playing_var:
            return
        if (
            pyxel.btnp(pyxel.KEY_RETURN, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME)
            or pyxel.btnp(pyxel.KEY_KP_ENTER, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME)
        ) and self.note_var is not None:
            self.field_cursor.insert(self.note_var)

    def __on_draw(self):
        # Draw frame
        pyxel.rect(self.x, self.y, self.width, self.height, 7)
        if self.is_playing_var:
            x = pyxel.play_pos(0)[1] * 4 + 31
            pyxel.rect(x, 25, 3, 123, PIANO_ROLL_CURSOR_PLAY_COLOR)
        elif self.field_cursor.y == 0:
            x = self.field_cursor.x * 4 + 31
            pyxel.rect(x, 25, 3, 123, PIANO_ROLL_CURSOR_EDIT_COLOR)
        pyxel.blt(
            self.x,
            self.y,
            EDITOR_IMAGE,
            0,
            7,
            193,
            72,
            PIANO_ROLL_BACKGROUND_COLOR,
        )
        pyxel.blt(
            self.x,
            self.y + 72,
            EDITOR_IMAGE,
            0,
            7,
            193,
            51,
            PIANO_ROLL_BACKGROUND_COLOR,
        )

        # Draw notes
        notes = self.get_field(0)
        for i, note in enumerate(notes):
            pyxel.rect(
                i * 4 + 31,
                143 - note * 2,
                3,
                3,
                PIANO_ROLL_NOTE_COLOR if note >= 0 else PIANO_ROLL_REST_COLOR,
            )
