import pyxel
from pyxel.ui import Widget


class OctaveBar(Widget):
    def __init__(self, parent, x, y):
        super().__init__(parent, x, y, 4, 123)

        self.add_event_handler("mouse_down", self.__on_mouse_down)
        self.add_event_handler("mouse_drag", self.__on_mouse_drag)
        self.add_event_handler("mouse_hover", self.__on_mouse_hover)
        self.add_event_handler("draw", self.__on_draw)

    def __on_mouse_down(self, key, x, y):
        if key != pyxel.MOUSE_LEFT_BUTTON:
            return

        x -= self.x
        y -= self.y

        self.parent.octave = min(max(3 - ((y - 12) // 24), 0), 3)

    def __on_mouse_drag(self, key, x, y, dx, dy):
        self.__on_mouse_down(key, x, y)

    def __on_mouse_hover(self, x, y):
        self.parent.help_message = "OCTAVE:PAGEUP/PAGEDOWN"

    def __on_draw(self):
        pyxel.rect(self.x, self.y, self.width, self.height, 6)

        x = self.x + 1
        y = self.y + 1 + (3 - self.parent.octave) * 24
        pyxel.rect(x, y, 2, 47, 13)
