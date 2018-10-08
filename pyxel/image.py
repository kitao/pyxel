import inspect
import os

import numpy as np
import PIL.Image

import pyxel

from .gl_wrapper import GLTexture


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

        sw = len(data[0])
        sh = len(data)

        rect = pyxel._app._get_copy_rect(
            0, 0, sw, sh, x, y, self.width, self.height, sw, sh
        )
        if not rect:
            return
        sx, sy, dx, dy, cw, ch = rect

        src_data = np.array([list(map(lambda x: int(x, 16), line)) for line in data])
        src_data = src_data[sy : sy + ch, sx : sx + cw]
        self._data[dy : dy + ch, dx : dx + cw] = src_data
        self._tex.update()

    def load(self, x, y, filename):
        dirname = os.path.dirname(inspect.stack()[-1].filename)
        filename = os.path.join(dirname, filename)

        pil_image = PIL.Image.open(filename).convert("RGB")
        pil_image.load()

        sw, sh = pil_image.size

        rect = pyxel._app._get_copy_rect(
            0, 0, sw, sh, x, y, self.width, self.height, sw, sh
        )
        if not rect:
            return
        sx, sy, dx, dy, cw, ch = rect

        pil_image = pyxel._app.palettize_pil_image(pil_image)
        pil_image = pil_image.crop((sx, sy, sx + cw, sy + ch))

        src_data = np.array(pil_image.getdata()).reshape(ch, cw)
        self._data[dy : dy + ch, dx : dx + cw] = src_data
        self._tex.update()

    def copy(self, x, y, img, sx, sy, w, h):
        image = pyxel.image(img)

        rect = pyxel._app._get_copy_rect(
            sx, sy, image.width, image.height, x, y, self.width, self.height, w, h
        )
        if not rect:
            return
        sx, sy, dx, dy, cw, ch = rect

        src_data = image._data[sy : sy + ch, sx : sx + cw]
        self._data[dy : dy + ch, dx : dx + cw] = src_data
        self._tex.update()
