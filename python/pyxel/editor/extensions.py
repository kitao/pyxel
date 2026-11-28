import pyxel


def _user_pal():
    for i in range(pyxel.num_user_colors):
        pyxel.pal(i, pyxel.NUM_COLORS + i)


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
    region = (
        pyxel.Tilemap(width, height, 0)
        if isinstance(self, pyxel.Tilemap)
        else pyxel.Image(width, height)
    )
    region.blt(0, 0, self, x, y, width, height)
    return region


def _set_slice(self, x, y, data):
    self.blt(x, y, data, 0, 0, data.width, data.height)


def _normalize_rect(x1, y1, x2, y2):
    return min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)


pyxel.user_pal = _user_pal  # type: ignore

# Attach editor-only drawing extensions to Image and Tilemap.
_EXTENSIONS = {
    "rect2": _rect2,
    "rectb2": _rectb2,
    "elli2": _elli2,
    "ellib2": _ellib2,
    "get_slice": _get_slice,
    "set_slice": _set_slice,
}
for _name, _func in _EXTENSIONS.items():
    setattr(pyxel.Image, _name, _func)
    setattr(pyxel.Tilemap, _name, _func)
