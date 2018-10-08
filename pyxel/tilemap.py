import numpy as np

import pyxel


class Tilemap:
    def __init__(self, width, height):
        self._width = width
        self._height = height
        self._data = np.zeros((width, height), np.uint8)

    @property
    def width(self):
        return self._width

    @property
    def height(self):
        return self._height

    @property
    def data(self):
        return self._data

    def get(self, x, y):
        return self._data[y, x]

    def set(self, x, y, data):
        if type(data) is int:
            self._data[y, x] = data
            return

        sw = len(data[0]) // 2
        sh = len(data)

        rect = pyxel._app._get_copy_rect(
            0, 0, sw, sh, x, y, self.width, self.height, sw, sh
        )
        if not rect:
            return
        sx, sy, dx, dy, cw, ch = rect

        src_data = np.array(
            [
                list(
                    map(
                        lambda x: int(x, 16),
                        [line[i * 2 : i * 2 + 2] for i in range(sw)],
                    )
                )
                for line in data
            ]
        )
        src_data = src_data[sy : sy + ch, sx : sx + cw]
        self._data[dy : dy + ch, dx : dx + cw] = src_data

    def copy(self, x, y, tm, sx, sy, w, h):
        tilemap = pyxel.tilemap(tm)

        rect = pyxel._app._get_copy_rect(
            sx, sy, tilemap.width, tilemap.height, x, y, self.width, self.height, w, h
        )
        if not rect:
            return
        sx, sy, dx, dy, cw, ch = rect

        src_data = tilemap._data[sy : sy + ch, sx : sx + cw]
        self._data[dy : dy + ch, dx : dx + cw] = src_data
