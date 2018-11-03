import numpy as np

from . import utilities


class Tilemap:
    def __init__(self, width, height):
        self._width = width
        self._height = height
        self._data = np.zeros((width, height), np.uint16)

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
        utilities.copy_ndarray(self._data, x, y, src)

    def copy(self, x, y, tm, u, v, w, h):
        tilemap = utilities.tilemap(tm)
        utilities.copy_ndarray(self._data, x, y, tilemap._data, u, v, w, h)
