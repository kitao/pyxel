import pyxel
from pyxel.ui.constants import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME


class FieldCursor:
    def __init__(
        self,
        data_getter,
        data_length_getter,
        data_length_setter,
        pre_history_setter,
        post_history_setter,
        data_max_length,
        data_view_length,
        data_count,
    ):
        self._get_data = data_getter
        self._get_data_length = data_length_getter
        self._set_data_length = data_length_setter
        self._add_pre_history = pre_history_setter
        self._add_post_history = post_history_setter
        self._data_max_length = data_max_length
        self._data_view_length = data_view_length
        self._data_count = data_count
        self._x = 0
        self._y = 0

    @property
    def x(self):
        return min(self._x, self.data_length, self._data_max_length - 1)

    @property
    def _max_x(self):
        return min(self.data_length, self._data_max_length - 1)

    @property
    def y(self):
        return self._y

    @property
    def data(self):
        return self._get_data(self._y)

    @property
    def data_length(self):
        return self._get_data_length(self._y)

    @data_length.setter
    def data_length(self, length):
        self._set_data_length(self._y, length)

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

        self._add_pre_history(self.x, self.y)

        self.data[x + 1 : self._data_max_length] = self.data[
            x : self._data_max_length - 1
        ]
        self.data[x] = value

        if self.data_length < self._data_max_length:
            self.data_length += 1

        self._x = x
        self.move_right()

        self._add_post_history(self.x, self.y)

    def overwrite(self, value):
        pass

    def backspace(self):
        x = self.x

        if x == 0:
            return

        self._add_pre_history(self.x, self.y)

        self.data[x - 1 : self._data_max_length - 1] = self.data[
            x : self._data_max_length
        ]

        if self.data_length > 0:
            self.data_length -= 1

        if self._x <= self._max_x:
            self.move_left()

        self._add_post_history(self.x, self.y)

    def delete(self):
        x = self.x

        if x >= self.data_length:
            return

        self._add_pre_history(self.x, self.y)

        self.data[x : self._data_max_length - 1] = self.data[
            x + 1 : self._data_max_length
        ]
        self.data_length -= 1

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
