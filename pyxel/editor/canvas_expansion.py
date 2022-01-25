import pyxel


def _rect2(self, x1, y1, x2, y2, val):
    x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
    y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)
    self.rect(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _rectb2(self, x1, y1, x2, y2, val):
    x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
    y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)
    self.rectb(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _elp2(self, x1, y1, x2, y2, val):
    x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
    y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)
    self.elp(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _elpb2(self, x1, y1, x2, y2, val):
    x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
    y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)
    self.elpb(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


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


pyxel.Image.rect2 = pyxel.Tilemap.rect2 = _rect2
pyxel.Image.rectb2 = pyxel.Tilemap.rectb2 = _rectb2
pyxel.Image.ellip2 = pyxel.Tilemap.ellip2 = _ellip2
pyxel.Image.ellipb2 = pyxel.Tilemap.ellipb2 = _ellipb2
pyxel.Image.get_slice = pyxel.Tilemap.get_slice = _get_slice
pyxel.Image.set_slice = pyxel.Tilemap.set_slice = _set_slice
