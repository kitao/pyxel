import pyxel

from .widgets.settings import WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME


class FieldCursor:
    def __init__(
        self,
        seq_getter,
        seq_count,
        max_seq_length,
        view_length,
        add_pre_history,
        add_post_history,
    ):
        self._seq_getter = seq_getter
        self._seq_count = seq_count
        self._max_seq_length = max_seq_length
        self._view_length = view_length
        self._add_pre_history = add_pre_history
        self._add_post_history = add_post_history
        self._x = 0
        self._y = 0

    @property
    def x(self):
        return min(self._x, len(self.seq), self._max_seq_length - 1)

    @property
    def rightmost_x(self):
        return min(len(self.seq), self._max_seq_length - 1)

    @property
    def y(self):
        return self._y

    @property
    def seq(self):
        return self._seq_getter(self._y)

    def move(self, x, y):
        self._x = x
        self._y = y

    def move_left(self):
        if self.x > 0:
            self._x = self.x - 1

    def move_right(self):
        if self.x < self.rightmost_x:
            self._x += 1

    def move_up(self):
        cursor_view_y = self._x // self._view_length

        if cursor_view_y > 0:
            self._x -= self._view_length
        elif self._y > 0:
            self._x = (
                self._view_length * (self.rightmost_x // self._view_length)
                + self._x % self._view_length
            )
            self._y -= 1

    def move_down(self):
        cursor_view_y = self._x // self._view_length
        seq_view_y = self.rightmost_x // self._view_length

        if cursor_view_y < seq_view_y:
            self._x += self._view_length
        elif self._y < self._seq_count - 1:
            self._y += 1
            self._x %= self._view_length

    def insert(self, value):
        x = self.x
        seq = self.sequence

        self._add_pre_history(self.x, self.y)

        seq.insert(x, value)
        seq[:] = seq[: self._max_seq_length]

        self._x = x
        self.move_right()

        self._add_post_history(self.x, self.y)

    def backspace(self):
        x = self.x
        seq = self.seq

        if x == 0:
            return

        self._add_pre_history(self.x, self.y)

        del seq[x - 1]
        if self._x <= self.rightmost_x:
            self.move_left()

        self._add_post_history(self.x, self.y)

    def delete(self):
        x = self.x
        seq = self.seq

        if x >= len(seq):
            return

        self._add_pre_history(self.x, self.y)

        del seq[x]

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
