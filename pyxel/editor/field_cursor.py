import pyxel

from .widgets.settings import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME


class FieldCursor:
    def __init__(
        self,
        *,
        max_field_length,
        field_wrap_length,
        get_field,
        add_pre_history,
        add_post_history,
        cross_filed_copying,
    ):
        self._max_field_length = max_field_length
        self._field_wrap_length = (
            field_wrap_length
            if field_wrap_length < max_field_length
            else max_field_length + 1
        )
        self._get_field = get_field
        self._add_pre_history = add_pre_history
        self._add_post_history = add_post_history
        self._cross_field_copying = cross_filed_copying
        self._cursor_x = 0
        self._cursor_y = 0
        self._select_x = None
        self._copy_field = None

    @property
    def x(self):
        return (
            min(self._adjusted_cursor_x, self._adjusted_select_x)
            if self.is_selecting
            else self._adjusted_cursor_x
        )

    @property
    def y(self):
        return self._cursor_y

    @property
    def width(self):
        if self.is_selecting:
            width = abs(self._adjusted_cursor_x - self._adjusted_select_x) + 1
            return min(width, len(self.field) - self.x)
        else:
            return 1

    @property
    def field(self):
        return self._get_field(self._cursor_y)

    @property
    def is_selecting(self):
        return self._select_x is not None and len(self.field) > 0

    @property
    def _max_cursor_x(self):
        return max(min(len(self.field), self._max_field_length - 1), 0)

    @property
    def _max_select_x(self):
        return max(min(len(self.field) - 1, self._max_field_length - 1), 0)

    @property
    def _max_y(self):
        y = 0
        while self._get_field(y + 1) is not None:
            y += 1
        return y

    @property
    def _adjusted_cursor_x(self):
        return min(self._cursor_x, self._max_cursor_x)

    @property
    def _adjusted_select_x(self):
        return min(self._select_x, self._max_select_x)

    def move_to(self, x, y, with_select_key):
        y = max(min(y, self._max_y), 0)
        if self._cursor_y != y:
            self._cursor_x = max(min(x, self._max_cursor_x), 0)
            self._cursor_y = y
            self._select_x = None
        elif with_select_key:
            if self.is_selecting:
                self._cursor_x = max(min(x, self._max_select_x), 0)
            else:
                self._select_x = max(
                    min(self._adjusted_cursor_x, self._max_select_x), 0
                )
                self._cursor_x = max(min(x, self._max_select_x), 0)
        else:
            self._cursor_x = max(min(x, self._max_cursor_x), 0)
            self._select_x = None

    def move_left(self, with_select_key):
        if with_select_key:
            if self.is_selecting:
                self._cursor_x = max(self._adjusted_cursor_x - 1, 0)
            elif len(self.field) > 0:
                self._cursor_x = self._select_x = min(
                    self._adjusted_cursor_x, self._max_select_x
                )
        else:
            self._cursor_x = max(self._adjusted_cursor_x - 1, 0)
            self._select_x = None

    def move_right(self, with_select_key):
        if with_select_key:
            if self.is_selecting:
                self._cursor_x = min(self._adjusted_cursor_x + 1, self._max_select_x)
            elif self._adjusted_cursor_x <= self._max_select_x:
                self._cursor_x = self._select_x = min(
                    self._adjusted_cursor_x, self._max_select_x
                )
        else:
            self._cursor_x = min(self._adjusted_cursor_x + 1, self._max_cursor_x)
            self._select_x = None

    def move_up(self, with_select_key):
        if self._adjusted_cursor_x >= self._field_wrap_length:
            if not with_select_key:
                self._select_x = None
            elif not self.is_selecting:
                self._select_x = self._adjusted_cursor_x
            self._cursor_x -= self._field_wrap_length
        elif self._cursor_y > 0:
            self._cursor_y -= 1
            self._cursor_x = (
                self._field_wrap_length * (len(self.field) // self._field_wrap_length)
                + self._cursor_x % self._field_wrap_length
            )
            self._select_x = None

    def move_down(self, with_select_key):
        if (
            self._adjusted_cursor_x // self._field_wrap_length
            < len(self.field) // self._field_wrap_length
        ):
            if not with_select_key:
                self._select_x = None
            elif not self.is_selecting:
                self._select_x = self._adjusted_cursor_x
            self._cursor_x += self._field_wrap_length
        elif self._cursor_y < self._max_y:
            self._cursor_y += 1
            self._cursor_x %= self._field_wrap_length
            self._select_x = None

    def insert(self, value):
        self._add_pre_history(self.x, self.y)
        lst = self.field.to_list()
        x = self.x
        if self.is_selecting:
            lst[x : x + self.width] = []
        if not isinstance(value, list):
            value = [value]
        lst[x:x] = value
        self.field.from_list(lst[: self._max_field_length])
        self.move_to(x + len(value), self.y, False)
        self._add_post_history(self.x, self.y)

    def backspace(self):
        if not self.is_selecting and self.x == 0:
            return
        self._add_pre_history(self.x, self.y)
        lst = self.field.to_list()
        if self.is_selecting:
            x = self.x
            width = self.width
        else:
            x = self.x - 1
            width = 1
        lst[x : x + width] = []
        self.field.from_list(lst)
        self.move_to(x, self.y, False)
        self._add_post_history(self.x, self.y)

    def delete(self):
        if self.x >= len(self.field):
            return
        self._add_pre_history(self.x, self.y)
        lst = self.field.to_list()
        x = self.x
        width = self.width
        lst[x : x + width] = []
        self.field.from_list(lst)
        self.move_to(x, self.y, False)
        self._add_post_history(self.x, self.y)

    def select_all(self):
        if len(self.field) == 0:
            return
        self._cursor_x = 0
        self._select_x = len(self.field) - 1

    def copy(self):
        if not self.is_selecting:
            return
        lst = self.field.to_list()
        self._copy_field = (self.y, lst[self.x : self.x + self.width])

    def cut(self):
        if not self.is_selecting:
            return
        self.copy()
        self.delete()

    def paste(self):
        if self._copy_field is None:
            return
        (y, field) = self._copy_field
        if not self._cross_field_copying and self.y != y:
            return
        self.insert(field)

    def process_input(self):
        if pyxel.btn(pyxel.KEY_ALT):
            return
        if pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI):
            # Ctrl+A: Select all
            if pyxel.btnp(pyxel.KEY_A):
                self.select_all()

            # Ctrl+C: Copy
            if pyxel.btnp(pyxel.KEY_C):
                self.copy()

            # Ctrl+X: Cut
            if pyxel.btnp(pyxel.KEY_X):
                self.cut()

            # Ctrl+V: Paste
            if pyxel.btnp(pyxel.KEY_V):
                self.paste()
            return
        with_select_key = pyxel.btn(pyxel.KEY_SHIFT)
        if pyxel.btnp(pyxel.KEY_LEFT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.move_left(with_select_key)
        if pyxel.btnp(pyxel.KEY_RIGHT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.move_right(with_select_key)
        if pyxel.btnp(pyxel.KEY_UP, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.move_up(with_select_key)
        if pyxel.btnp(pyxel.KEY_DOWN, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.move_down(with_select_key)
        if pyxel.btnp(pyxel.KEY_BACKSPACE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.backspace()
        if pyxel.btnp(pyxel.KEY_DELETE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.delete()
