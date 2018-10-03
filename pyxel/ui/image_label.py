import pyxel

from .widget import Widget


class ImageLabel(Widget):
    def __init__(self, parent, x, y, img, sx, sy, w, h, colkey=None, **kwargs):
        super().__init__(parent, x, y, w, h, **kwargs)

        self._img = img
        self._sx = sx
        self._sy = sy
        self._colkey = colkey

        self.add_event_handler("draw", self.__on_draw)

    def __on_draw(self):
        pyxel.blt(
            self.x,
            self.y,
            self._img,
            self._sx,
            self._sy,
            self.width,
            self.height,
            self._colkey,
        )
