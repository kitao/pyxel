import numpy as np

from .utilities import copy_ndarray, get_pyxel_tilemap


class Tilemap:
    def __init__(self, width, height):
        self._width = width
        self._height = height
        self._data = np.zeros((width, height), np.uint16)
        self.refimg = 0

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

    def set(self, x, y, data, refimg=None):
        if refimg is not None:
            self.refimg = refimg

        if type(data) is int:
            self._data[y, x] = data
            return

        w = len(data[0]) // 2
        src = np.array(
            [
                list(
                    map(
                        lambda x: int(x, 16),
                        [line[i * 2 : i * 2 + 2] for i in range(w)],
                    )
                )
                for line in data
            ]
        )
        copy_ndarray(self._data, x, y, src)

    def copy(self, x, y, tm, u, v, w, h):
        tilemap = get_pyxel_tilemap(tm)
        copy_ndarray(self._data, x, y, tilemap._data, u, v, w, h)
