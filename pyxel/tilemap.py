import numpy as np


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

    def set(self, x, y, data):
        pass

    def copy(self, x, y, tm, sx, sy, width, height):
        pass
