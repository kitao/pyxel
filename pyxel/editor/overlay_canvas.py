import numpy as np


class OverlayCanvas:
    def __init__(self):
        self.data = np.ndarray((16, 16), np.int16)
        self.clear()

    def clear(self):
        self.data[:, :] = -1

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

    def pix(self, x, y, val):
        if x >= 0 and x < 16 and y >= 0 and y < 16:
            self.data[y, x] = val

    def line(self, x1, y1, x2, y2, val):
        if x1 == x2 and y1 == y2:
            self.pix(x1, y1, val)
            return

        dx = x2 - x1
        dy = y2 - y1

        if abs(dx) > abs(dy):
            if dx < 0:
                x1, y1 = x2, y2
                dx, dy = -dx, -dy

            for i in range(dx + 1):
                x = x1 + i
                y = int(y1 + i * dy / dx + 0.5)

                if x >= 0 and x < 16 and y >= 0 and y < 16:
                    self.data[y, x] = val
        else:
            if dy < 0:
                x1, y1 = x2, y2
                dx, dy = -dx, -dy

            for i in range(dy + 1):
                x = int(x1 + i * dx / dy + 0.5)
                y = y1 + i

                if x >= 0 and x < 16 and y >= 0 and y < 16:
                    self.data[y, x] = val

    def rectb(self, x1, y1, x2, y2, val, is_guide_mode):
        x1, y1, x2, y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)

        for y in range(max(y1, 0), min(y2 + 1, 16)):
            for x in range(max(x1, 0), min(x2 + 1, 16)):
                if x == x1 or x == x2 or y == y1 or y == y2:
                    self.data[y, x] = val

    def rect(self, x1, y1, x2, y2, val, is_guide_mode):
        x1, y1, x2, y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)

        for y in range(max(y1, 0), min(y2 + 1, 16)):
            for x in range(max(x1, 0), min(x2 + 1, 16)):
                self.data[y, x] = val

    def circb(self, x1, y1, x2, y2, val, is_guide_mode):
        x1, y1, x2, y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)

        a = (x2 - x1) / 2
        b = (y2 - y1) / 2

        if a <= 0.5 or b <= 0.5:
            self.rect(x1, y1, x2, y2, val, False)
            return

        cx = (x1 + x2) / 2
        cy = (y1 + y2) / 2

        for y in range(max(y1, 0), min(y2 + 1, 16)):
            for x in range(max(x1, 0), min(x2 + 1, 16)):
                dx = x - cx
                dy = y - cy

                if self._inner_ellipse(dx, dy, a, b) and (
                    not self._inner_ellipse(dx - 1, dy, a, b)
                    or not self._inner_ellipse(dx + 1, dy, a, b)
                    or not self._inner_ellipse(dx, dy - 1, a, b)
                    or not self._inner_ellipse(dx, dy + 1, a, b)
                ):
                    self.data[y, x] = val

    def circ(self, x1, y1, x2, y2, val, is_guide_mode):
        x1, y1, x2, y2 = self._adjust_region(x1, y1, x2, y2, is_guide_mode)

        a = (x2 - x1) / 2
        b = (y2 - y1) / 2

        if a <= 0.5 or b <= 0.5:
            self.rect(x1, y1, x2, y2, val, False)
            return

        cx = (x1 + x2) / 2
        cy = (y1 + y2) / 2

        for y in range(max(y1, 0), min(y2 + 1, 16)):
            for x in range(max(x1, 0), min(x2 + 1, 16)):
                if self._inner_ellipse(x - cx, y - cy, a, b):
                    self.data[y, x] = val

    def paint(self, x, y, val, dest):
        dest_val = dest[y, x]

        if dest_val == val:
            return

        for i in range(x, -1, -1):
            if dest[y, i] != dest_val:
                break

            dest[y, i] = val

            if y > 0 and dest[y - 1, i] == dest_val:
                self.paint(i, y - 1, val, dest)

            if y < 15 and dest[y + 1, i] == dest_val:
                self.paint(i, y + 1, val, dest)

        for i in range(x + 1, 16):
            if dest[y, i] != dest_val:
                return

            dest[y, i] = val

            if y > 0 and dest[y - 1, i] == dest_val:
                self.paint(i, y - 1, val, dest)

            if y < 15 and dest[y + 1, i] == dest_val:
                self.paint(i, y + 1, val, dest)
