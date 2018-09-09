import pyxel
from pyxel.editor.editor_constants import (WIDGET_CLICK_DIST,
                                           WIDGET_CLICK_TIME,
                                           WIDGET_KEY_HOLD_TIME,
                                           WIDGET_KEY_REPEAT_TIME)


class Widget:
    _capture_widget = None
    _capture_key = None
    _capture_time = None
    _capture_press_pos = None
    _capture_last_pos = None

    def __init__(self,
                 parent,
                 x,
                 y,
                 width,
                 height,
                 *,
                 is_visible=True,
                 is_repeat=False):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.parent = parent
        self.children = []
        self._event_handler = {}
        self.is_enabled = True
        self._is_visible = False
        self.is_repeat = is_repeat

        if is_visible:
            self.is_visible = True

        if parent:
            parent.children.append(self)

    @property
    def is_visible(self):
        return self._is_visible

    @is_visible.setter
    def is_visible(self, value):
        if self._is_visible == value:
            return

        self._is_visible = value

        if value:
            self.call_event_handler('show')
        else:
            self.call_event_handler('hide')

    def _capture_mouse(self, key):
        Widget._capture_widget = self
        Widget._capture_key = key
        Widget._capture_time = pyxel.frame_count
        Widget._capture_press_pos = (pyxel.mouse_x, pyxel.mouse_y)
        Widget._capture_last_pos = Widget._capture_press_pos

    def _release_mouse(self):
        Widget._capture_widget = None
        Widget._capture_key = None
        Widget._capture_time = None
        Widget._capture_press_pos = None
        Widget._capture_last_pos = None

    def _get_event_handler(self, event):
        if event not in self._event_handler:
            self._event_handler[event] = []

        return self._event_handler[event]

    def add_event_handler(self, event, handler):
        self._get_event_handler(event).append(handler)

    def remove_event_handler(self, event, handler):
        self._get_event_handler(event).remove(handler)

    def call_event_handler(self, event, *args):
        for handler in self._get_event_handler(event):
            handler(*args)

    @staticmethod
    def process_input(root):
        capturer = Widget._capture_widget

        if capturer:
            mx = pyxel.mouse_x
            my = pyxel.mouse_y
            last_mx, last_my = Widget._capture_last_pos

            if mx != last_mx or my != last_my:
                capturer.call_event_handler('drag', Widget._capture_key,
                                            mx - capturer.x, my - capturer.y,
                                            mx - last_mx, my - last_my)
                Widget._capture_last_pos = (mx, my)

            if pyxel.btnr(Widget._capture_key):
                capturer.call_event_handler('release', Widget._capture_key,
                                            mx - capturer.x, my - capturer.y)

                press_x, press_y = Widget._capture_press_pos
                if (pyxel.frame_count <=
                        Widget._capture_time + WIDGET_CLICK_TIME
                        and abs(pyxel.mouse_x - press_x) <= WIDGET_CLICK_DIST
                        and abs(pyxel.mouse_y - press_y) <= WIDGET_CLICK_DIST):
                    capturer.call_event_handler('click', Widget._capture_key,
                                                mx - capturer.x,
                                                my - capturer.y)

                root._release_mouse()

        root._process_input()

    def _process_input(self):
        if not self._is_visible:
            return False

        if self.is_enabled:
            for widget in reversed(self.children):
                if widget._process_input():
                    return True

        mx = pyxel.mouse_x
        my = pyxel.mouse_y

        if (mx >= self.x and mx < self.x + self.width and my >= self.y
                and my < self.y + self.height):
            if self.is_enabled and (not Widget._capture_widget
                                    or Widget._capture_widget == self):
                key = None

                if self.is_repeat:
                    hold_time = WIDGET_KEY_HOLD_TIME
                    repeat_time = WIDGET_KEY_REPEAT_TIME
                else:
                    hold_time = 0
                    repeat_time = 0

                if pyxel.btnp(pyxel.KEY_LEFT_BUTTON, hold_time, repeat_time):
                    key = pyxel.KEY_LEFT_BUTTON
                elif pyxel.btnp(pyxel.KEY_RIGHT_BUTTON, hold_time,
                                repeat_time):
                    key = pyxel.KEY_RIGHT_BUTTON

                if key is not None:
                    self._capture_mouse(key)
                    x = mx - self.x
                    y = my - self.y
                    self.call_event_handler('press', key, x, y)
                    return True

            self.call_event_handler('hover', mx - self.x, my - self.y)

        return False

    @staticmethod
    def update(root):
        if not root._is_visible:
            return

        root.call_event_handler('update')

        for child in root.children:
            Widget.update(child)

    @staticmethod
    def draw(root):
        if not root._is_visible:
            return

        root.call_event_handler('draw')

        for child in root.children:
            Widget.draw(child)

    def on_show(self):
        pass

    def on_hide(self):
        pass

    def on_press(self, key, x, y):
        pass

    def on_release(self, key, x, y):
        pass

    def on_click(self, key, x, y):
        pass

    def on_drag(self, key, x, y, dx, dy):
        pass

    def on_hover(self, x, y):
        pass

    def on_update(self):
        pass

    def on_draw(self):
        pass
