import pyxel
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME


class FieldCursor:
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
        self._x = 0
        self._y = 0

    @property
    def x(self):
        return min(self._x, len(self.data), self._data_max_length - 1)

    @property
    def _max_x(self):
        return min(len(self.data), self._data_max_length - 1)

    @property
    def y(self):
        return self._y

    @property
    def data(self):
        return self._get_data(self._y)

    def move(self, x, y):
        self._x = x
        self._y = y

    def move_left(self):
        if self.x > 0:
            self._x = self.x - 1

    def move_right(self):
        if self.x < self._max_x:
            self._x += 1

    def move_up(self):
        cursor_view_y = self._x // self._data_view_length

        if cursor_view_y > 0:
            self._x -= self._data_view_length
        elif self._y > 0:
            self._y -= 1

            data_view_y = self._max_x // self._data_view_length
            self._x = (
                self._data_view_length * data_view_y + self._x % self._data_view_length
            )

    def move_down(self):
        cursor_view_y = self._x // self._data_view_length
        data_view_y = self._max_x // self._data_view_length

        if cursor_view_y < data_view_y:
            self._x += self._data_view_length
        elif self._y < self._data_count - 1:
            self._y += 1
            self._x %= self._data_view_length

    def insert(self, value):
        x = self.x
        data = self.data

        self._add_pre_history(self.x, self.y)

        data.insert(x, value)
        data[:] = data[: self._data_max_length]

        self._x = x
        self.move_right()

        self._add_post_history(self.x, self.y)

    def backspace(self):
        x = self.x
        data = self.data

        if x == 0:
            return

        self._add_pre_history(self.x, self.y)

        del data[x - 1]
        if self._x <= self._max_x:
            self.move_left()

        self._add_post_history(self.x, self.y)

    def delete(self):
        x = self.x
        data = self.data

        if x >= len(data):
            return

        self._add_pre_history(self.x, self.y)

        del data[x]

        self._add_post_history(self.x, self.y)

    def process_input(self):
        if (
            pyxel.btn(pyxel.KEY_SHIFT)
            or pyxel.btn(pyxel.KEY_CONTROL)
            or pyxel.btn(pyxel.KEY_ALT)
        ):
            return

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
