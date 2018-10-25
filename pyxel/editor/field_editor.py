import pyxel
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME


class FieldEditor:
    def __init__(
        self,
        data_getter,
        pre_history_setter,
        post_history_setter,
        data_max_length,
        data_view_length,
        data_count,
    ):
        self._get_data = data_getter
        self._add_pre_history = pre_history_setter
        self._add_post_history = post_history_setter
        self._data_max_length = data_max_length
        self._data_view_length = data_view_length
        self._data_count = data_count
        self._cursor_x = 0
        self._cursor_y = 0

    @property
    def cursor_x(self):
        return min(self._cursor_x, len(self.data), self._data_max_length - 1)

    @property
    def _max_cursor_x(self):
        return min(len(self.data), self._data_max_length - 1)

    @property
    def cursor_y(self):
        return self._cursor_y

    @property
    def data(self):
        return self._get_data(self._cursor_y)

    def get_data(self, index):
        return self._get_data(index)

    def move(self, x, y):
        self._cursor_x = x
        self._cursor_y = y

    def move_left(self):
        if self.cursor_x > 0:
            self._cursor_x = self.cursor_x - 1

    def move_right(self):
        if self.cursor_x < self._max_cursor_x:
            self._cursor_x += 1

    def move_up(self):
        cursor_view_y = self._cursor_x // self._data_view_length

        if cursor_view_y > 0:
            self._cursor_x -= self._data_view_length
        elif self._cursor_y > 0:
            self._cursor_y -= 1

            data_view_y = self._max_cursor_x // self._data_view_length
            self._cursor_x = (
                self._data_view_length * data_view_y
                + self._cursor_x % self._data_view_length
            )

    def move_down(self):
        cursor_view_y = self._cursor_x // self._data_view_length
        data_view_y = self._max_cursor_x // self._data_view_length

        if cursor_view_y < data_view_y:
            self._cursor_x += self._data_view_length
        elif self._cursor_y < self._data_count - 1:
            self._cursor_y += 1
            self._cursor_x %= self._data_view_length

    def insert(self, value):
        x = self.cursor_x
        data = self.data

        self._add_pre_history(self.cursor_x, self.cursor_y)

        data.insert(x, value)
        data[:] = data[: self._data_max_length]

        self._cursor_x = x
        self.move_right()

        self._add_post_history(self.cursor_x, self.cursor_y)

    def overwrite(self, value):
        pass

    def backspace(self):
        x = self.cursor_x

        if x == 0:
            return

        data = self.data

        self._add_pre_history(self.cursor_x, self.cursor_y)

        del data[x - 1]
        self.move_left()

        self._add_post_history(self.cursor_x, self.cursor_y)

    def delete(self):
        x = self.cursor_x
        data = self.data

        if x >= len(data):
            return

        self._add_pre_history(self.cursor_x, self.cursor_y)

        del data[x]

        self._add_post_history(self.cursor_x, self.cursor_y)

    def process_input(self):
        if pyxel.btnp(pyxel.KEY_LEFT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.move_left()

        if pyxel.btnp(pyxel.KEY_RIGHT, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.move_right()

        if pyxel.btnp(pyxel.KEY_UP, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.move_up()

        if pyxel.btnp(pyxel.KEY_DOWN, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.move_down()

        if pyxel.btnp(pyxel.KEY_BACKSPACE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.backspace()

        if pyxel.btnp(pyxel.KEY_DELETE, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.delete()
