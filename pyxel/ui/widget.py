import pyxel

from .ui_constants import (
    WIDGET_CLICK_DIST,
    WIDGET_CLICK_TIME,
    WIDGET_HOLD_TIME,
    WIDGET_REPEAT_TIME,
)


class Widget:
    """
    Events:
        __on_show()
        __on_hide()
        __on_enabled()
        __on_disables()
        __on_mouse_down(key, x, y)
        __on_mouse_up(key, x, y)
        __on_mouse_drag(key, x, y, dx, dy)
        __on_mouse_hover(x, y)
        __on_mouse_click(key, x, y)
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
        self,
        parent,
        x,
        y,
        width,
        height,
        *,
        is_visible=True,
        is_enabled=True,
        is_key_repeat=False
    ):
        self.parent = parent
        self.children = []
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self._is_visible = None
        self._is_enabled = None
        self.is_key_repeat = is_key_repeat
        self._event_handler = {}

        if parent:
            parent.children.append(self)

        self.is_visible = is_visible
        self.is_enabled = is_enabled

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

    def add_event_handler(self, event, handler):
        self._get_event_handler(event).append(handler)

    def remove_event_handler(self, event, handler):
        self._get_event_handler(event).remove(handler)

    def call_event_handler(self, event, *args):
        for handler in self._get_event_handler(event):
            handler(*args)

    def _get_event_handler(self, event):
        if event not in self._event_handler:
            self._event_handler[event] = []

        return self._event_handler[event]

    def is_hit(self, x, y):
        return (
            x >= self.x
            and x < self.x + self.width
            and y >= self.y
            and y < self.y + self.height
        )

    def _capture_mouse(self, key):
        Widget._capture_info.widget = self
        Widget._capture_info.key = key
        Widget._capture_info.time = pyxel.frame_count
        Widget._capture_info.press_pos = (pyxel.mouse_x, pyxel.mouse_y)
        Widget._capture_info.last_pos = Widget._capture_info.press_pos

    def _release_mouse(self):
        Widget._capture_info.widget = None
        Widget._capture_info.key = None
        Widget._capture_info.time = None
        Widget._capture_info.press_pos = None
        Widget._capture_info.last_pos = None

    @staticmethod
    def update(root):
        capture_widget = Widget._capture_info.widget

        if capture_widget:
            capture_widget._process_capture()
        else:
            root._process_input()

        root._update()

    def _process_capture(self):
        capture_info = Widget._capture_info
        mx = pyxel.mouse_x
        my = pyxel.mouse_y
        last_mx, last_my = capture_info.last_pos

        if mx != last_mx or my != last_my:
            self.call_event_handler(
                "mouse_drag", capture_info.key, mx, my, mx - last_mx, my - last_my
            )
            capture_info.last_pos = (mx, my)

        if pyxel.btnr(capture_info.key):
            self.call_event_handler("mouse_up", capture_info.key, mx, my)

            press_x, press_y = capture_info.press_pos
            if (
                pyxel.frame_count <= capture_info.time + WIDGET_CLICK_TIME
                and abs(pyxel.mouse_x - press_x) <= WIDGET_CLICK_DIST
                and abs(pyxel.mouse_y - press_y) <= WIDGET_CLICK_DIST
            ):
                self.call_event_handler("mouse_click", capture_info.key, mx, my)

            widget._release_mouse()

    def _process_input(self):
        if not self._is_visible:
            return False

        if self._is_enabled:
            for widget in reversed(self.children):
                if widget._process_input():
                    return True
        else:
            return False

        mx = pyxel.mouse_x
        my = pyxel.mouse_y

        if self.is_hit(mx, my):
            if self.is_key_repeat:
                hold_time = WIDGET_HOLD_TIME
                repeat_time = WIDGET_REPEAT_TIME
            else:
                hold_time = 0
                repeat_time = 0

            key = None

            if pyxel.btnp(pyxel.KEY_LEFT_BUTTON, hold_time, repeat_time):
                key = pyxel.KEY_LEFT_BUTTON
            elif pyxel.btnp(pyxel.KEY_RIGHT_BUTTON, hold_time, repeat_time):
                key = pyxel.KEY_RIGHT_BUTTON

            if key != None:
                self._capture_mouse(key)
                self.call_event_handler("mouse_down", key, mx, my)
            else:
                self.call_event_handler("mouse_hover", mx, my)

            return True

        return False

    def _update(self):
        if not self._is_visible:
            return

        self.call_event_handler("update")

        for child in root.children:
            child.update()

    @staticmethod
    def draw(root):
        if not root._is_visible:
            return

        root.call_event_handler("draw")

        for child in root.children:
            Widget.draw(child)
