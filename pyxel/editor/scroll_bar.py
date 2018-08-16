import pyxel

from .widget import Widget


class ScrollBar(Widget):
    def __init__(self, parent, x, y, width, height):
        super().__init__(parent, x, y, width, height)
