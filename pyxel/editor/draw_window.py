import numpy as np

from .widget import Widget


class DrawWindow(Widget):
    def __init__(self, parent):
        super().__init__(parent, 12, 17, 128, 128)

        self.overlay = np.ndarray((16, 16), np.int8)
        self.clear_overlay()

    def clear_overlay(self):
        self.overlay[:, :] = -1

    def draw_line_on_overlay(self, x1, y1, x2, y2, val):
        if x1 == x2 and y1 == y2:
            if x1 >= 0 and x1 < 16 and y1 >= 0 and y1 < 16:
                self.overlay[y1, x1] = val
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
                    self.overlay[y, x] = val
        else:
            if dy < 0:
                x1, y1 = x2, y2
                dx, dy = -dx, -dy

            for i in range(dy + 1):
                x = int(x1 + i * dx / dy + 0.5)
                y = y1 + i

                if x >= 0 and x < 16 and y >= 0 and y < 16:
                    self.overlay[y, x] = val

    def draw_rectb_on_overlay(self, x1, y1, x2, y2, val):
        self.overlay[y1:y1 + 1, x1:x2 + 1] = val
        self.overlay[y2:y2 + 1, x1:x2 + 1] = val
        self.overlay[y1:y2 + 1, x1:x1 + 1] = val
        self.overlay[y1:y2 + 1, x2:x2 + 1] = val

    def draw_rect_on_overlay(self, x1, y1, x2, y2, val):
        self.overlay[y1:y2 + 1, x1:x2 + 1] = val

    def draw_circb_on_overlay(self, x1, y1, x2, y2, val):
        pass

    def draw_circ_on_overlay(self, x1, y1, x2, y2, val):
        pass
