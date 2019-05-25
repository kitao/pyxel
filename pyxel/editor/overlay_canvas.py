import numpy as np


class OverlayCanvas:
    COLOR_NONE = 0x7FFF
    COLOR_MARK = 0x7FFE

    def __init__(self):
        self.data = np.ndarray((16, 16), np.uint16)
        self.clear()

    def clear(self):
        self.data[:, :] = OverlayCanvas.COLOR_NONE

    @staticmethod
    def _adjust_region(x1, y1, x2, y2, is_guide_mode):
        if is_guide_mode:
            dx = x2 - x1
            dy = y2 - y1

            if abs(dx) > abs(dy):
                y2 = y1 + abs(dx) * (1 if dy > 0 else -1)
            else:
                x2 = x1 + abs(dy) * (1 if dx > 0 else -1)

        x1, x2 = (x1, x2) if x1 < x2 else (x2, x1)
        y1, y2 = (y1, y2) if y1 < y2 else (y2, y1)

        return x1, y1, x2, y2

    @staticmethod
    def _inner_ellipse(x, y, a, b):
        a += 0.41
        b += 0.41
        return x * x * b * b + y * y * a * a < a * a * b * b

    @staticmethod
    def _replace_with_tiles(dest, x, y, tiles):
        tiles_height, tiles_width = tiles.shape

        for i in range(16):
            for j in range(16):
                if dest[i, j] == OverlayCanvas.COLOR_MARK:
                    dest[i, j] = tiles[(i - y) % tiles_height, (j - x) % tiles_width]

    def pix(self, x, y, col):
        if x < 0 or x > 15 or y < 0 or y > 15:
            return

        if type(col) is int:
            self.data[y, x] = col
        else:
            copy_ndarray(self.data, x, y, col)

    def line(self, x1, y1, x2, y2, col):
        if x1 == x2 and y1 == y2:
            self.pix(x1, y1, col)
            return

        dx = x2 - x1
        dy = y2 - y1

        if abs(dx) > abs(dy):
            sign = 1 if dx > 0 else -1
            rate = sign * dy / dx

            for i in range(abs(dx) + 1):
                x = x1 + sign * i
                y = int(y1 + rate * i + 0.5)

                if x < 0 or x > 15 or y < 0 or y > 15:
                    continue

                if type(col) is int:
                    self.data[y, x] = col
                else:
                    copy_ndarray(self.data, x, y, col)
        else:
            sign = 1 if dy > 0 else -1
            rate = sign * dx / dy

            for i in range(abs(dy) + 1):
                x = int(x1 + rate * i + 0.5)
                y = y1 + sign * i

                if x < 0 or x > 15 or y < 0 or y > 15:
                    continue

                if type(col) is int:
                    self.data[y, x] = col
                else:
                    copy_ndarray(self.data, x, y, col)

    def rectb(self, x1, y1, x2, y2, col, is_guide_mode):
        _x1, _y1, _x2, _y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)
        _col = col if type(col) is int else OverlayCanvas.COLOR_MARK

        for y in range(max(_y1, 0), min(_y2 + 1, 16)):
            for x in range(max(_x1, 0), min(_x2 + 1, 16)):
                if x == _x1 or x == _x2 or y == _y1 or y == _y2:
                    self.data[y, x] = _col

        if type(col) is not int:
            self._replace_with_tiles(self.data, x1, y1, col)

    def rect(self, x1, y1, x2, y2, col, is_guide_mode):
        _x1, _y1, _x2, _y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)
        _col = col if type(col) is int else OverlayCanvas.COLOR_MARK

        for y in range(max(_y1, 0), min(_y2 + 1, 16)):
            for x in range(max(_x1, 0), min(_x2 + 1, 16)):
                self.data[y, x] = _col

        if type(col) is not int:
            self._replace_with_tiles(self.data, x1, y1, col)

    def circb(self, x1, y1, x2, y2, col, is_guide_mode):
        _x1, _y1, _x2, _y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)
        _col = col if type(col) is int else OverlayCanvas.COLOR_MARK

        a = (_x2 - _x1) / 2
        b = (_y2 - _y1) / 2

        if a <= 0.5 or b <= 0.5:
            self.rect(_x1, _y1, _x2, _y2, col, False)
            return

        cx = (_x1 + _x2) / 2
        cy = (_y1 + _y2) / 2

        for y in range(max(_y1, 0), min(_y2 + 1, 16)):
            for x in range(max(_x1, 0), min(_x2 + 1, 16)):
                dx = x - cx
                dy = y - cy

                if self._inner_ellipse(dx, dy, a, b) and (
                    not self._inner_ellipse(dx - 1, dy, a, b)
                    or not self._inner_ellipse(dx + 1, dy, a, b)
                    or not self._inner_ellipse(dx, dy - 1, a, b)
                    or not self._inner_ellipse(dx, dy + 1, a, b)
                ):
                    self.data[y, x] = _col

        if type(col) is not int:
            self._replace_with_tiles(self.data, x1, y1, col)

    def circ(self, x1, y1, x2, y2, col, is_guide_mode):
        _x1, _y1, _x2, _y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)
        _col = col if type(col) is int else OverlayCanvas.COLOR_MARK

        a = (_x2 - _x1) / 2
        b = (_y2 - _y1) / 2

        if a <= 0.5 or b <= 0.5:
            self.rect(_x1, _y1, _x2, _y2, col, False)
            return

        cx = (_x1 + _x2) / 2
        cy = (_y1 + _y2) / 2

        for y in range(max(_y1, 0), min(_y2 + 1, 16)):
            for x in range(max(_x1, 0), min(_x2 + 1, 16)):
                if self._inner_ellipse(x - cx, y - cy, a, b):
                    self.data[y, x] = _col

        if type(col) is not int:
            self._replace_with_tiles(self.data, x1, y1, col)

    def fill(self, x, y, col, dest):
        _col = col if type(col) is int else OverlayCanvas.COLOR_MARK

        self._fill_recursively(x, y, _col, dest)

        if type(col) is not int:
            self._replace_with_tiles(dest, x, y, col)

    def _fill_recursively(self, x, y, col, dest):
        dest_col = dest[y, x]

        if dest_col == col:
            return

        for i in range(x, -1, -1):
            if dest[y, i] != dest_col:
                break

            dest[y, i] = col

            if y > 0 and dest[y - 1, i] == dest_col:
                self._fill_recursively(i, y - 1, col, dest)

            if y < 15 and dest[y + 1, i] == dest_col:
                self._fill_recursively(i, y + 1, col, dest)

        for i in range(x + 1, 16):
            if dest[y, i] != dest_col:
                return

            dest[y, i] = col

            if y > 0 and dest[y - 1, i] == dest_col:
                self._fill_recursively(i, y - 1, col, dest)

            if y < 15 and dest[y + 1, i] == dest_col:
                self._fill_recursively(i, y + 1, col, dest)


def copy_ndarray(dest, dx, dy, src, sx=0, sy=0, cw=None, ch=None):
    dh, dw = dest.shape
    sh, sw = src.shape
    cw = cw or sw
    ch = ch or sh

    rx1 = max(max(-dx, 0), max(-sx, 0))
    ry1 = max(max(-dy, 0), max(-sy, 0))
    rx2 = max(max(dx + cw - dw, 0), max(sx + cw - sw, 0))
    ry2 = max(max(dy + ch - dh, 0), max(sy + ch - sh, 0))

    cw -= rx1 + rx2
    ch -= ry1 + ry2

    if cw <= 0 or ch <= 0:
        return False

    dx += rx1
    dy += ry1
    sx += rx1
    sy += ry1

    dest[dy : dy + ch, dx : dx + cw] = src[sy : sy + ch, sx : sx + cw]

    return True
