import pyxel
from pyxel.ui import Widget


class OctaveBar(Widget):
    def __init__(self, parent, x, y):
        super().__init__(parent, x, y, 4, 123)

        self.add_event_handler("mouse_down", self.__on_update)
        self.add_event_handler("mouse_drag", self.__on_update)
        self.add_event_handler("mouse_hover", self.__on_update)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

    def __on_update(self):
        pass

    def __on_draw(self):
        pyxel.rect(self.x, self.y, self.x + self.width - 1, self.y + self.height - 1, 6)

        x = self.x + 1
        y = self.y + 1
        pyxel.rect(x, y, x + 1, y + 46, 13)
