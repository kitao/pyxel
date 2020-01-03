import pyxel

from .constants import (
    WIDGET_CLICK_DIST,
    WIDGET_CLICK_TIME,
    WIDGET_HOLD_TIME,
    WIDGET_PANEL_COLOR,
    WIDGET_REPEAT_TIME,
    WIDGET_SHADOW_COLOR,
)


class Widget:
    """
    Events:
        __on_show()
        __on_hide()
        __on_enabled()
        __on_disabled()
        __on_move(x, y)
        __on_resize(width, height)
        __on_mouse_down(key, x, y)
        __on_mouse_up(key, x, y)
        __on_mouse_drag(key, x, y, dx, dy)
        __on_mouse_repeat(key, x, y)
        __on_mouse_click(key, x, y)
        __on_mouse_hover(x, y)
        __on_update()
        __on_draw()
    """

    class CaptureInfo:
        widget = None
        key = None
        time = None
        press_pos = None
        last_pos = None

    _capture_info = CaptureInfo()

    def __init__(
        self, parent, x, y, width, height, *, is_visible=True, is_enabled=True
    ):
        self._parent = None
        self._children = []
        self._x = None
        self._y = None
        self._width = None
        self._height = None
        self._is_visible = None
        self._is_enabled = None
        self._event_handler_lists = {}

        self.parent = parent
        self.move(x, y)
        self.resize(width, height)
        self.is_visible = is_visible
        self.is_enabled = is_enabled

    @property
    def parent(self):
        return self._parent

    @parent.setter
    def parent(self, value):
        if self._parent:
            self._parent._children.remove(self)

        self._parent = value

        if value:
            value._children.append(self)

    @property
    def x(self):
        return self._x

    @property
    def y(self):
        return self._y

    @property
    def width(self):
        return self._width

    @property
    def height(self):
        return self._height

    @property
    def is_visible(self):
        return self._is_visible

    @is_visible.setter
    def is_visible(self, value):
        if self._is_visible == value:
            return

        self._is_visible = value

        if value:
            self.call_event_handler("show")
        else:
            self.call_event_handler("hide")

    @property
    def is_enabled(self):
        return self._is_enabled

    @is_enabled.setter
    def is_enabled(self, value):
        if self._is_enabled == value:
            return

        self._is_enabled = value

        if value:
            self.call_event_handler("enabled")
        else:
            self.call_event_handler("disabled")

    def _get_event_handler_list(self, event):
        if event not in self._event_handler_lists:
            self._event_handler_lists[event] = []

        return self._event_handler_lists[event]

    def add_event_handler(self, event, handler):
        self._get_event_handler_list(event).append(handler)

    def remove_event_handler(self, event, handler):
        self._get_event_handler_list(event).remove(handler)

    def call_event_handler(self, event, *args):
        for handler in self._get_event_handler_list(event):
            handler(*args)

    def is_hit(self, x, y):
        return (
            self._x <= x <= self._x + self._width - 1
            and self._y <= y <= self._y + self._height - 1
        )

    def move(self, x, y):
        if self._x == x and self._y == y:
            return

        if self._x is None or self._y is None:
            self._x = x
            self._y = y

        dx = x - self._x
        dy = y - self._y

        self._move_delta(dx, dy)

    def _move_delta(self, dx, dy):
        self._x += dx
        self._y += dy

        self.call_event_handler("move", self._x, self._y)

        for child in self._children:
            child._move_delta(dx, dy)

    def resize(self, width, height):
        if self._width == width and self._height == height:
            return

        self._width = width
        self._height = height
        self.call_event_handler("resize", width, height)

    @staticmethod
    def draw_panel(x, y, width, height, *, with_shadow=True):
        x = x
        y = y
        w = width
        h = height

        pyxel.line(x + 1, y, x + w - 2, y, WIDGET_PANEL_COLOR)
        pyxel.rect(x, y + 1, w, h - 2, WIDGET_PANEL_COLOR)
        pyxel.line(x + 1, y + h - 1, x + w - 2, y + h - 1, WIDGET_PANEL_COLOR)

        if with_shadow:
            pyxel.line(x + 2, y + h, x + w - 1, y + h, WIDGET_SHADOW_COLOR)
            pyxel.line(x + w, y + 2, x + w, y + h - 1, WIDGET_SHADOW_COLOR)
            pyxel.pix(x + w - 1, y + h - 1, WIDGET_SHADOW_COLOR)

    def _capture_mouse(self, key):
        Widget._capture_info.widget = self
        Widget._capture_info.key = key
        Widget._capture_info.time = pyxel.frame_count
        Widget._capture_info.press_pos = (pyxel.mouse_x, pyxel.mouse_y)
        Widget._capture_info.last_pos = Widget._capture_info.press_pos

    @staticmethod
    def _release_mouse():
        Widget._capture_info.widget = None
        Widget._capture_info.key = None
        Widget._capture_info.time = None
        Widget._capture_info.press_pos = None
        Widget._capture_info.last_pos = None

    def update_widgets(self):
        capture_widget = Widget._capture_info.widget

        if capture_widget:
            capture_widget._process_capture()
        else:
            self._process_input()

        self._update()

    def _process_capture(self):
        capture_info = Widget._capture_info
        last_mx, last_my = capture_info.last_pos

        mx = pyxel.mouse_x
        my = pyxel.mouse_y

        if mx != last_mx or my != last_my:
            self.call_event_handler(
                "mouse_drag", capture_info.key, mx, my, mx - last_mx, my - last_my
            )
            capture_info.last_pos = (mx, my)

        if self.is_hit(mx, my):
            self.call_event_handler("mouse_hover", mx, my)

        if pyxel.btnp(capture_info.key, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.call_event_handler("mouse_repeat", capture_info.key, mx, my)

        if pyxel.btnr(capture_info.key):
            self.call_event_handler("mouse_up", capture_info.key, mx, my)

            press_x, press_y = capture_info.press_pos
            if (
                pyxel.frame_count <= capture_info.time + WIDGET_CLICK_TIME
                and abs(pyxel.mouse_x - press_x) <= WIDGET_CLICK_DIST
                and abs(pyxel.mouse_y - press_y) <= WIDGET_CLICK_DIST
            ):
                self.call_event_handler("mouse_click", capture_info.key, mx, my)

            self._release_mouse()

    def _process_input(self):
        if not self._is_visible:
            return False

        if self._is_enabled:
            for widget in reversed(self._children):
                if widget._process_input():
                    return True
        else:
            return False

        mx = pyxel.mouse_x
        my = pyxel.mouse_y

        if self.is_hit(mx, my):
            key = None

            if pyxel.btnp(pyxel.MOUSE_LEFT_BUTTON):
                key = pyxel.MOUSE_LEFT_BUTTON
            elif pyxel.btnp(pyxel.MOUSE_RIGHT_BUTTON):
                key = pyxel.MOUSE_RIGHT_BUTTON
            elif pyxel.btnp(pyxel.MOUSE_MIDDLE_BUTTON):
                key = pyxel.MOUSE_MIDDLE_BUTTON

            if key is not None:
                self._capture_mouse(key)
                self.call_event_handler("mouse_down", key, mx, my)

            self.call_event_handler("mouse_hover", mx, my)

            return True

        return False

    def _update(self):
        if not self._is_visible:
            return

        self.call_event_handler("update")

        for child in self._children:
            child._update()

    def draw_widgets(self):
        if not self._is_visible:
            return

        self.call_event_handler("draw")

        for child in self._children:
            child.draw_widgets()
