import pyxel
from pyxel.editor.editor_constants import CLICK_DIST, CLICK_TIME


class Widget:
    _capture_widget = None
    _capture_key = None
    _capture_time = None
    _capture_press_pos = None
    _capture_last_pos = None

    def __init__(self, parent, x, y, width, height, *, is_visible=True):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.parent = parent
        self.children = []
        self._event_handler = {}
        self._is_visible = False

        if is_visible:
            self.set_visible(True)

        if parent:
            parent.children.append(self)

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

    def set_visible(self, is_visible):
        if self._is_visible == is_visible:
            return

        self._is_visible = is_visible

        if is_visible:
            self.call_event_handler('show')
        else:
            self.call_event_handler('hide')

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

    def process_input(self):
        if not self._is_visible:
            return False

        widget = Widget._capture_widget

        if widget:
            mx = pyxel.mouse_x
            my = pyxel.mouse_y
            last_mx, last_my = Widget._capture_last_pos

            if mx != last_mx or my != last_my:
                widget.call_event_handler('drag', Widget._capture_key,
                                          mx - widget.x, my - widget.y,
                                          mx - last_mx, my - last_my)
                Widget._capture_last_pos = (mx, my)

            if pyxel.btnr(Widget._capture_key):
                widget.call_event_handler('release', Widget._capture_key,
                                          mx - widget.x, my - widget.y)

                press_x, press_y = Widget._capture_press_pos
                if (pyxel.frame_count <= Widget._capture_time + CLICK_TIME
                        and abs(pyxel.mouse_x - press_x) <= CLICK_DIST
                        and abs(pyxel.mouse_y - press_y) <= CLICK_DIST):
                    widget.call_event_handler('click', Widget._capture_key,
                                              mx - widget.x, my - widget.y)

                self._release_mouse()

            return True

        for widget in reversed(self.children):
            if widget.process_input():
                return True

        mx = pyxel.mouse_x
        my = pyxel.mouse_y

        if (mx >= self.x and mx < self.x + self.width and my >= self.y
                and my < self.y + self.height):
            key = None
            if pyxel.btnp(pyxel.KEY_LEFT_BUTTON):
                key = pyxel.KEY_LEFT_BUTTON
            elif pyxel.btnp(pyxel.KEY_RIGHT_BUTTON):
                key = pyxel.KEY_RIGHT_BUTTON

            if key is not None:
                self._capture_mouse(key)
                x = mx - self.x
                y = my - self.y
                self.call_event_handler('press', key, x, y)
                return True

            self.call_event_handler('hover', mx - self.x, my - self.y)

        return False

    def update(self):
        if not self._is_visible:
            return

        self.call_event_handler('update')

        for widget in self.children:
            widget.update()

    def draw(self):
        if not self._is_visible:
            return

        self.call_event_handler('draw')

        for widget in self.children:
            widget.draw()

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
