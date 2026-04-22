import pyxel


def _user_pal():
    for i in range(pyxel.NUM_COLORS):
        pyxel.pal(i, pyxel.NUM_COLORS + i)


def _normalize_rect(x1, y1, x2, y2):
    return min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)


def _rect2(self, x1, y1, x2, y2, val):
    x1, y1, x2, y2 = _normalize_rect(x1, y1, x2, y2)
    self.rect(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _rectb2(self, x1, y1, x2, y2, val):
    x1, y1, x2, y2 = _normalize_rect(x1, y1, x2, y2)
    self.rectb(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _elli2(self, x1, y1, x2, y2, val):
    x1, y1, x2, y2 = _normalize_rect(x1, y1, x2, y2)
    self.elli(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _ellib2(self, x1, y1, x2, y2, val):
    x1, y1, x2, y2 = _normalize_rect(x1, y1, x2, y2)
    self.ellib(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _get_slice(self, x, y, width, height):
    return [[self.pget(x + xi, y + yi) for xi in range(width)] for yi in range(height)]


def _set_slice(self, x, y, data):
    width = len(data[0])
    height = len(data)

    for yi in range(height):
        for xi in range(width):
            self.pset(x + xi, y + yi, data[yi][xi])


pyxel.user_pal = _user_pal  # type: ignore

# Attach editor-only drawing extensions to Image and Tilemap
_EXTENSIONS = {
    "rect2": _rect2,
    "rectb2": _rectb2,
    "elli2": _elli2,
    "ellib2": _ellib2,
    "get_slice": _get_slice,
    "set_slice": _set_slice,
}
for _name, _func in _EXTENSIONS.items():
    setattr(pyxel.Image, _name, _func)  # type: ignore
    setattr(pyxel.Tilemap, _name, _func)  # type: ignore
