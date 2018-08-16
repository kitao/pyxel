import pyxel
from pyxel.editor.editor_constants import CLICK_DIST, CLICK_TIME


class Widget:
    _capture_widget = None
    _capture_key = None
    _capture_time = None
    _capture_press_pos = None
    _capture_last_pos = None

    def __init__(self, parent, x, y, width, height, *, is_visible=True):
        self.parent = parent
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.children = []
        self._is_visible = False

        if is_visible:
            self.set_visible(True)

        if parent:
            parent.children.append(self)

    def set_visible(self, is_visible):
        if self._is_visible == is_visible:
            return

        self._is_visible = is_visible

        if is_visible:
            self.on_show()
        else:
            self.on_hide()

    def process_mouse_event(self):
        if not self._is_visible:
            return False

        if Widget._capture_widget:
            widget = Widget._capture_widget
            mx = pyxel.mouse_x
            my = pyxel.mouse_y
            last_mx, last_my = Widget._capture_last_pos

            if mx != last_mx or my != last_my:
                widget.on_drag(Widget._capture_key, mx - widget.x,
                               my - widget.y, mx - last_mx, my - last_my)
                Widget._capture_last_pos = (mx, my)

            if pyxel.btnr(Widget._capture_key):
                widget.on_release(Widget._capture_key, mx - widget.x,
                                  my - widget.y)

                press_x, press_y = Widget._capture_press_pos
                if (pyxel.frame_count <= Widget._capture_time + CLICK_TIME
                        and abs(pyxel.mouse_x - press_x) <= CLICK_DIST
                        and abs(pyxel.mouse_y - press_y) <= CLICK_DIST):
                    widget.on_click(Widget._capture_key, mx - widget.x,
                                    my - widget.y)

                Widget._capture_widget = None

            return True

        for widget in reversed(self.children):
            if widget.process_mouse_event():
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
                Widget._capture_widget = self
                Widget._capture_key = key
                Widget._capture_time = pyxel.frame_count
                Widget._capture_press_pos = (mx, my)
                Widget._capture_last_pos = Widget._capture_press_pos

                x = mx - self.x
                y = my - self.y
                self.on_press(key, x, y)
                self.on_drag(key, x, y, 0, 0)
                return True

            self.on_hover()

        return False

    def update(self):
        if not self._is_visible:
            return

        self.on_update()

        for widget in self.children:
            widget.update()

    def draw(self):
        if not self._is_visible:
            return

        self.on_draw()

        for widget in self.children:
            widget.draw()

    def on_show(self):
        pass

    def on_hide(self):
        pass

    def on_press(self, key, mx, my):
        pass

    def on_release(self, key, mx, my):
        pass

    def on_click(self, key, mx, my):
        pass

    def on_drag(self, key, mx, my, dx, dy):
        pass

    def on_hover(self):
        pass

    def on_update(self):
        pass

    def on_draw(self):
        pass
