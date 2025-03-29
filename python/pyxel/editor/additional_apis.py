import pyxel


def _user_pal():
    num_user_colors = len(pyxel.colors) - pyxel.NUM_COLORS
    for i in range(pyxel.NUM_COLORS):
        if i < num_user_colors:
            pyxel.pal(i, pyxel.NUM_COLORS + i)
        else:
            pyxel.pal(i, 0)


def _rect2(self, x1, y1, x2, y2, val):
    x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
    y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)
    self.rect(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _rectb2(self, x1, y1, x2, y2, val):
    x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
    y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)
    self.rectb(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _elli2(self, x1, y1, x2, y2, val):
    x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
    y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)
    self.elli(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _ellib2(self, x1, y1, x2, y2, val):
    x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
    y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)
    self.ellib(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _get_slice(self, x, y, width, height):
    data = [[0] * width for _ in range(height)]

    for yi in range(height):
        for xi in range(width):
            data[yi][xi] = self.pget(x + xi, y + yi)

    return data


def _set_slice(self, x, y, slice):
    width = len(slice[0])
    height = len(slice)

    for yi in range(height):
        for xi in range(width):
            self.pset(x + xi, y + yi, slice[yi][xi])


pyxel.user_pal = _user_pal  # type: ignore

pyxel.Image.rect2 = pyxel.Tilemap.rect2 = _rect2  # type: ignore
pyxel.Image.rectb2 = pyxel.Tilemap.rectb2 = _rectb2  # type: ignore
pyxel.Image.elli2 = pyxel.Tilemap.elli2 = _elli2  # type: ignore
pyxel.Image.ellib2 = pyxel.Tilemap.ellib2 = _ellib2  # type: ignore
pyxel.Image.get_slice = pyxel.Tilemap.get_slice = _get_slice  # type: ignore
pyxel.Image.set_slice = pyxel.Tilemap.set_slice = _set_slice  # type: ignore
