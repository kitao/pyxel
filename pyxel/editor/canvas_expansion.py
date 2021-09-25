import pyxel


def _rectb2(self, x1, y1, x2, y2, val):
    self.rectb(min(x1, x2), min(y1, y2), abs(x1 - x2) + 1, abs(y1 - y2) + 1, val)


def _rect2(self, x1, y1, x2, y2, val):
    self.rect(min(x1, x2), min(y1, y2), abs(x1 - x2) + 1, abs(y1 - y2) + 1, val)


def _in_ellipse(x, y, a, b):
    a += 0.41
    b += 0.41
    return x * x * b * b + y * y * a * a < a * a * b * b


def _ellipb(self, x1, y1, x2, y2, val):
    a = abs(x1 - x2) / 2
    b = abs(y1 - y2) / 2

    if a <= 0.5 or b <= 0.5:
        self.rect(min(x1, x2), min(y1, y2), abs(x1 - x2), abs(y1 - y2), val)
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
    a = abs(x1 - x2) / 2
    b = abs(y1 - y2) / 2

    if a <= 0.5 or b <= 0.5:
        self.rect(x1, y1, x2, y2, val, False)
        return

    cx = (x1 + x2) / 2
    cy = (y1 + y2) / 2

    for y in range(max(y1, 0), min(y2 + 1, 16)):
        for x in range(max(x1, 0), min(x2 + 1, 16)):
            if _in_ellipse(x - cx, y - cy, a, b):
                self.psert(x, y, val)


def _fill(self, x, y, val):
    _fill_rec(self, x, y, val)


def _fill_rec(self, x, y, val, dst):
    dst_val = self.pget(x, y)

    if dst_val == val:
        return

    for i in range(x, -1, -1):
        if dst[y][i] != dst_val:
            break

        dst[y][i] = val

        if y > 0 and dst[y - 1][i] == dst_val:
            self._fill_rec(i, y - 1, val, dst)

        if y < 15 and dst[y + 1][i] == dst_val:
            self._fill_rec(i, y + 1, val, dst)

    for i in range(x + 1, 16):
        if dst[y][i] != dst_val:
            return

        dst[y][i] = val

        if y > 0 and dst[y - 1][i] == dst_val:
            self._fill_rec(i, y - 1, val, dst)

        if y < 15 and dst[y + 1][i] == dst_val:
            self._fill_rec(i, y + 1, val, dst)


def _get_slice(self, x, y, width, height):
    data = [[0] * width for _ in range(height)]

    for i in range(height):
        for j in range(width):
            data[i][j] = self.pget(x + j, y + i)

    return data


def _set_slice(self, x, y, width, height, slice):
    for i in range(height):
        for j in range(width):
            self.pset(x + j, y + i, slice[i][j])


pyxel.Image.rectb2 = pyxel.Tilemap.rectb2 = _rectb2
pyxel.Image.rect2 = pyxel.Tilemap.rect2 = _rect2
pyxel.Image.ellipb = pyxel.Tilemap.ellipb = _ellipb
pyxel.Image.ellip = pyxel.Tilemap.ellip = _ellip
pyxel.Image.fill = pyxel.Tilemap.fill = _fill
pyxel.Image.get_slice = pyxel.Tilemap.get_slice = _get_slice
pyxel.Image.set_slice = pyxel.Tilemap.set_slicde = _set_slice
