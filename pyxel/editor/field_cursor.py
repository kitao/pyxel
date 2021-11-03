import pyxel

from .widgets.settings import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME


class FieldCursor:
    def __init__(
        self,
        *,
        field_getter,
        max_field_length,
        field_wrap_length,
        add_pre_history,
        add_post_history,
    ):
        self._field_getter = field_getter
        self._max_field_length = max_field_length
        self._field_wrap_length = field_wrap_length
        self._add_pre_history = add_pre_history
        self._add_post_history = add_post_history
        self._x = 0
        self._y = 0

    @property
    def x(self):
        return min(self._x, len(self.field), self._max_field_length - 1)

    @property
    def y(self):
        return self._y

    @property
    def field(self):
        return self._field_getter(self._y)

    def move_to(self, x, y):
        while not self._field_getter(y):
            y -= 1

        self._x = min(x, self._max_field_length - 1)
        self._y = y

    def move_left(self):
        if self._x > 0:
            self._x = self.x - 1

    def move_right(self):
        if self._x < min(len(self.field), self._max_field_length - 1):
            self._x += 1

    def move_up(self):
        if self._x >= self._field_wrap_length:
            self._x -= self._field_wrap_length

        elif self._y > 0:
            self._x = (
                self._field_wrap_length * (self._x // self._field_wrap_length) + self._x
            )
            self._y -= 1

    def move_down(self):
        if len(self.field) >= self._field_wrap_length:
            self._x += self._field_wrap_length
            return

        if not self._field_getter(self._y + 1):
            return

        self._x %= self._field_wrap_length
        self._y += 1

    def insert(self, value):
        self._add_pre_history(self.x, self.y)

        lst = self.field.to_list()
        lst.insert(self.x, value)
        self.field.from_list(lst[: self._max_field_length])
        self.move_right()

        self._add_post_history(self.x, self.y)

    def backspace(self):
        if self.x == 0:
            return

        self._add_pre_history(self.x, self.y)

        lst = self.field.to_list()
        del lst[self.x - 1]
        self.field.from_list(lst)
        self.move_left()

        self._add_post_history(self.x, self.y)

    def delete(self):
        if self.x >= len(self.field):
            return

        self._add_pre_history(self.x, self.y)

        lst = self.field.to_list()
        del lst[self.x]
        self.field.from_list(lst)

        self._add_post_history(self.x, self.y)

    def process_input(self):
        if (
            pyxel.btn(pyxel.KEY_SHIFT)
            or pyxel.btn(pyxel.KEY_CTRL)
            or pyxel.btn(pyxel.KEY_ALT)
            or pyxel.btn(pyxel.KEY_GUI)
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
