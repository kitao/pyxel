import pyxel

from .settings import (
    WIDGET_CLICK_DIST,
    WIDGET_CLICK_TIME,
    WIDGET_HOLD_TIME,
    WIDGET_PANEL_COLOR,
    WIDGET_REPEAT_TIME,
    WIDGET_SHADOW_COLOR,
)


class WidgetVariable:
    def __init__(self, value):
        self._value = value
        self.on_get = None
        self.on_set = None
        self.on_change = None

    @property
    def v(self):
        if self.on_get:
            return self.on_get(self._value)
        else:
            return self._value

    @v.setter
    def v(self, value):
        if self.on_get:
            value = self.on_get(value)

        if self._value != value:
            self._value = value

            if self.on_change:
                self.on_change(value)


class MouseCaptureInfo:
    widget = None
    key = None
    time = None
    press_pos = None
    last_pos = None


class Widget:
    """
    Variables:
        is_visible_var
        is_enabled_var

    Events:
        show
        hide
        enabled
        disabled
        mouse_down (key, x, y)
        mouse_up (key, x, y)
        mouse_drag (key, x, y, dx, dy)
        mouse_repeat (key, x, y)
        mouse_click (key, x, y)
        mouse_hover (x, y)
        update
        draw
    """

    _mouse_capture_info = MouseCaptureInfo()

    def __init__(
        self, parent, x, y, width, height, *, is_visible=True, is_enabled=True
    ):
        if parent:
            parent._children.append(self)

        self._parent = parent
        self._children = []
        self._x = x
        self._y = y
        self._width = width
        self._height = height
        self._is_visible = is_visible
        self._is_enabled = is_enabled
        self._event_listeners = {}

        def on_visible_get(value):
            if self._parent:
                return self._parent.is_visible_var.v and value
            else:
                return value

        def on_visible_change(value):
            self._trigger_visible_event(value)

        self.is_visible_var = WidgetVariable(is_visible)
        self.is_visible_var.on_get = on_visible_get
        self.is_visible_var.on_change = on_visible_change

        def on_enabled_get(value):
            if self._parent:
                return self._parent.is_enabled_var.v and value
            else:
                return value

        def on_enabled_change(value):
            self._trigger_enabled_event(value)

        self.is_enabled_var = WidgetVariable(is_enabled)
        self.is_enabled_var.on_get = on_enabled_get
        self.is_enabled_var.on_change = on_enabled_change

    @property
    def x(self):
        if self._parent:
            return self._parent.x + self._x
        else:
            return self._x

    @property
    def y(self):
        if self._parent:
            return self._parent.y + self._y
        else:
            return self._y

    @property
    def x2(self):
        return self.x + self.width - 1

    @property
    def y2(self):
        return self.y + self.height - 1

    @property
    def width(self):
        return self._width

    @property
    def height(self):
        return self._height

    def is_hit(self, x, y):
        return self.x <= x <= self.x2 and self.y <= y <= self.y2

    def set_pos(self, x, y):
        self._x = x
        self._y = y

    def set_size(self, width, height):
        self._width = width
        self._height = height

    def add_event_listener(self, event, listener):
        self._event_listeners.setdefault(event, [])
        self._event_listeners[event].append(listener)

    def remove_event_listener(self, event, listener):
        self._event_listeners.setdefault(event, [])
        self._event_listeners[event].remove(listener)

    def trigger_event(self, event, *args):
        self._event_listeners.setdefault(event, [])

        for listener in self._event_listeners[event]:
            listener(*args)

    def _trigger_visible_event(self, is_visible):
        self.trigger_event("show" if is_visible else "hide")

        for child in self._children:
            if child.is_visible_var.v == is_visible:
                child._trigger_visible_event(is_visible)

    def _trigger_enabled_event(self, is_enabled):
        self.trigger_event("enabled" if is_enabled else "disabled")

        for child in self._children:
            if child.is_enabled_var.v == is_enabled:
                child._trigger_enabled_event(is_enabled)

    def update_all(self):
        capture_widget = Widget._mouse_capture_info.widget

        if capture_widget:
            capture_widget._process_capture()
        else:
            self._process_input()

        self._update()

    def _process_input(self):
        if not self.is_really_visible or not self.is_really_enabled:
            return False

        for widget in reversed(self._children):
            if widget._process_input():
                return True

        x = pyxel.mouse_x
        y = pyxel.mouse_y

        if self.is_hit(x, y):
            key = None

            if pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT):
                key = pyxel.MOUSE_BUTTON_LEFT
            elif pyxel.btnp(pyxel.MOUSE_BUTTON_RIGHT):
                key = pyxel.MOUSE_BUTTON_RIGHT
            elif pyxel.btnp(pyxel.MOUSE_BUTTON_MIDDLE):
                key = pyxel.MOUSE_BUTTON_MIDDLE

            if key is not None:
                self._start_capture(key)
                self.trigger_event("mouse_down", key, x, y)

            self.trigger_event("mouse_hover", x, y)

            return True

        return False

    def _start_capture(self, key):
        capture_info = Widget._mouse_capture_info
        capture_info.widget = self
        capture_info.key = key
        capture_info.time = pyxel.frame_count
        capture_info.press_pos = (pyxel.mouse_x, pyxel.mouse_y)
        capture_info.last_pos = capture_info.press_pos

    def _end_capture(self):
        capture_info = Widget._mouse_capture_info
        capture_info.widget = None
        capture_info.key = None
        capture_info.time = None
        capture_info.press_pos = None
        capture_info.last_pos = None

    def _process_capture(self):
        capture_info = Widget._mouse_capture_info
        last_x, last_y = capture_info.last_pos

        x = pyxel.mouse_x
        y = pyxel.mouse_y

        if x != last_x or y != last_y:
            self.trigger_event(
                "mouse_drag",
                capture_info.key,
                x,
                y,
                x - last_x,
                y - last_y,
            )
            capture_info.last_pos = (x, y)

        if self.is_hit(x, y):
            self.trigger_event("mouse_hover", x, y)

        if pyxel.btnp(capture_info.key, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME):
            self.trigger_event("mouse_repeat", capture_info.key, x, y)

        if pyxel.btnr(capture_info.key):
            self.trigger_event("mouse_up", capture_info.key, x, y)

            press_x, press_y = capture_info.press_pos

            if (
                pyxel.frame_count <= capture_info.time + WIDGET_CLICK_TIME
                and abs(x - press_x) <= WIDGET_CLICK_DIST
                and abs(y - press_y) <= WIDGET_CLICK_DIST
            ):
                self.trigger_event("mouse_click", capture_info.key, x, y)

            self._end_capture()

    def _update(self):
        if not self.is_visible_var.v:
            return

        self.trigger_event("update")

        for child in self._children:
            child._update()

    def draw_all(self):
        if not self.is_visible_var.v:
            return

        self.trigger_event("draw")

        for child in self._children:
            child.draw_all()

    @staticmethod
    def draw_panel(x, y, width, height, *, with_shadow=True):
        w = width
        h = height

        pyxel.line(x + 1, y, x + w - 2, y, WIDGET_PANEL_COLOR)
        pyxel.rect(x, y + 1, w, h - 2, WIDGET_PANEL_COLOR)
        pyxel.line(x + 1, y + h - 1, x + w - 2, y + h - 1, WIDGET_PANEL_COLOR)

        if with_shadow:
            pyxel.line(x + 2, y + h, x + w - 1, y + h, WIDGET_SHADOW_COLOR)
            pyxel.line(x + w, y + 2, x + w, y + h - 1, WIDGET_SHADOW_COLOR)
            pyxel.pset(x + w - 1, y + h - 1, WIDGET_SHADOW_COLOR)
