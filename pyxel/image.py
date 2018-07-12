import os
import inspect
import numpy as np
import PIL.Image
from .glwrapper import GLTexture
import pyxel


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

    def set(self, x, y, data):
        width = len(data[0])
        height = len(data)

        self._data[y:y + height, x:x + width] = [
            list(map(lambda x: int(x, 16), line)) for line in data
        ]

    def load(self, x, y, filename):
        palette = []
        for color in pyxel._app._palette:
            r = (color >> 16) & 0xff
            g = (color >> 8) & 0xff
            b = color & 0xff
            palette.extend((r, g, b))
        palette += [0] * 240 * 3

        palette_image = PIL.Image.new('P', (1, 1), 0)
        palette_image.putpalette(palette)

        dirname = os.path.dirname(inspect.stack()[-1].filename)
        filename = os.path.join(dirname, filename)

        pil_image = PIL.Image.open(filename).convert('RGB')
        pil_image.load()

        im = pil_image.im.convert('P', 0, palette_image.im)
        pil_image = pil_image._new(im)

        width, height = pil_image.size
        width = min(width, self._tex.width - x)
        height = min(height, self._tex.height - y)
        pil_image = pil_image.crop((0, 0, width, height))

        self._data[y:y + height, x:x + width] = np.array(
            pil_image.getdata()).reshape(height, width)

    def copy(self, x, y, img):
        pass
