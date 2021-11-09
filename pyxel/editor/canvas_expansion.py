import pyxel


def _rectb2(self, x1, y1, x2, y2, val):
    x1, x2 = min(x1, x2), max(x1, x2)
    y1, y2 = min(y1, y2), max(y1, y2)
    self.rectb(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _rect2(self, x1, y1, x2, y2, val):
    x1, x2 = min(x1, x2), max(x1, x2)
    y1, y2 = min(y1, y2), max(y1, y2)
    self.rect(x1, y1, x2 - x1 + 1, y2 - y1 + 1, val)


def _in_ellipse(x, y, a, b):
    a += 0.41
    b += 0.41
    return x * x * b * b + y * y * a * a < a * a * b * b


def _ellipb(self, x1, y1, x2, y2, val):
    x1, x2 = min(x1, x2), max(x1, x2)
    y1, y2 = min(y1, y2), max(y1, y2)
    a = (x2 - x1) / 2
    b = (y2 - y1) / 2
    if a <= 0.5 or b <= 0.5:
        self.rect2(x1, y1, x2, y2, val)
        return

    cx = (x1 + x2) / 2
    cy = (y1 + y2) / 2
    for y in range(max(y1, 0), min(y2 + 1, 16)):
        for x in range(max(x1, 0), min(x2 + 1, 16)):
            dx = x - cx
            dy = y - cy
            if _in_ellipse(dx, dy, a, b) and (
                not _in_ellipse(dx - 1, dy, a, b)
                or not _in_ellipse(dx + 1, dy, a, b)
                or not _in_ellipse(dx, dy - 1, a, b)
                or not _in_ellipse(dx, dy + 1, a, b)
            ):
                self.pset(x, y, val)


def _ellip(self, x1, y1, x2, y2, val):
    x1, x2 = min(x1, x2), max(x1, x2)
    y1, y2 = min(y1, y2), max(y1, y2)
    a = (x2 - x1) / 2
    b = (y2 - y1) / 2
    if a <= 0.5 or b <= 0.5:
        self.rect2(x1, y1, x2, y2, val)
        return

    cx = (x1 + x2) / 2
    cy = (y1 + y2) / 2
    for y in range(max(y1, 0), min(y2 + 1, 16)):
        for x in range(max(x1, 0), min(x2 + 1, 16)):
            if _in_ellipse(x - cx, y - cy, a, b):
                self.pset(x, y, val)


def _fill(self, x, y, val):
    dst_val = self.pget(x, y)
    if dst_val == val:
        return

    for i in range(x, -1, -1):
        if self.pget(i, y) != dst_val:
            break
        self.pset(i, y, val)
        if y > 0 and self.pget(i, y - 1) == dst_val:
            self.fill(i, y - 1, val)
        if y < 15 and self.pget(i, y + 1) == dst_val:
            self.fill(i, y + 1, val)

    for i in range(x + 1, 16):
        if self.pget(i, y) != dst_val:
            break
        self.pset(i, y, val)
        if y > 0 and self.pget(i, y - 1) == dst_val:
            self.fill(i, y - 1, val)
        if y < 15 and self.pget(i, y + 1) == dst_val:
            self.fill(i, y + 1, val)


def _get_slice(self, x, y, width, height):
    data = [[0] * width for _ in range(height)]
    for i in range(height):
        for j in range(width):
            data[i][j] = self.pget(x + j, y + i)
    return data


def _set_slice(self, x, y, slice):
    width = len(slice[0])
    height = len(slice)
    for i in range(height):
        for j in range(width):
            self.pset(x + j, y + i, slice[i][j])


pyxel.Image.rectb2 = pyxel.Tilemap.rectb2 = _rectb2
pyxel.Image.rect2 = pyxel.Tilemap.rect2 = _rect2
pyxel.Image.ellipb = pyxel.Tilemap.ellipb = _ellipb
pyxel.Image.ellip = pyxel.Tilemap.ellip = _ellip
pyxel.Image.fill = pyxel.Tilemap.fill = _fill
pyxel.Image.get_slice = pyxel.Tilemap.get_slice = _get_slice
pyxel.Image.set_slice = pyxel.Tilemap.set_slice = _set_slice
