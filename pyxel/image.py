import inspect
import os

import numpy as np
import PIL.Image

from .gl_wrapper import GLTexture
from .utilities import copy_ndarray, get_pyxel_image, palettize_pil_image


class Image:
    def __init__(self, width, height):
        self._tex = GLTexture(width, height, 1, nearest=True)
        self._data = self._tex.data

    @property
    def width(self):
        return self._tex.width

    @property
    def height(self):
        return self._tex.height

    @property
    def data(self):
        self._tex.update()
        return self._data

    def get(self, x, y):
        return self._data[y, x]

    def set(self, x, y, data):
        if type(data) is int:
            self._data[y, x] = data
            self._tex.update()
            return

        src = np.array([list(map(lambda x: int(x, 16), line)) for line in data])

        if copy_ndarray(self._data, x, y, src):
            self._tex.update()

    def load(self, x, y, filename):
        dirname = os.path.dirname(inspect.stack()[-1].filename)
        filename = os.path.join(dirname, filename)

        pil_image = PIL.Image.open(filename).convert("RGB")
        pil_image.load()
        pil_image = palettize_pil_image(pil_image)

        sw, sh = pil_image.size
        src = np.array(pil_image.getdata()).reshape(sh, sw)

        if copy_ndarray(self._data, x, y, src):
            self._tex.update()

    def copy(self, x, y, img, u, v, w, h):
        image = get_pyxel_image(img)

        if copy_ndarray(self._data, x, y, image._data, u, v, w, h):
            self._tex.update()
