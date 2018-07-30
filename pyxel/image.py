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
        sw = len(data[0])
        sh = len(data)

        rect = self._get_copy_rect(0, 0, sw, sh, x, y, self.width, self.height,
                                   sw, sh)
        if not rect:
            return
        sx, sy, dx, dy, cw, ch = rect

        src_data = np.array(
            [list(map(lambda x: int(x, 16), line)) for line in data])
        src_data = src_data[sy:sy + ch, sx:sx + cw]
        self._data[dy:dy + ch, dx:dx + cw] = src_data

    def load(self, x, y, filename):
        dirname = os.path.dirname(inspect.stack()[-1].filename)
        filename = os.path.join(dirname, filename)

        pil_image = PIL.Image.open(filename).convert('RGB')
        pil_image.load()

        sw, sh = pil_image.size

        rect = self._get_copy_rect(0, 0, sw, sh, x, y, self.width, self.height,
                                   sw, sh)
        if not rect:
            return
        sx, sy, dx, dy, cw, ch = rect

        pil_image = pyxel._app.palettize_pil_image(pil_image)
        pil_image = pil_image.crop((sx, sy, sx + cw, sy + ch))

        src_data = np.array(pil_image.getdata()).reshape(ch, cw)
        self._data[dy:dy + ch, dx:dx + cw] = src_data

    def copy(self, x, y, img, sx, sy, width, height):
        image = pyxel.image(img)

        rect = self._get_copy_rect(sx, sy, image.width, image.height, x, y,
                                   self.width, self.height, width, height)
        if not rect:
            return
        sx, sy, dx, dy, cw, ch = rect

        src_data = image._data[sy:sy + ch, sx:sx + cw]
        self._data[dy:dy + ch, dx:dx + cw] = src_data

    @staticmethod
    def _get_copy_rect(sx, sy, sw, sh, dx, dy, dw, dh, cw, ch):
        over_sx = max(-sx, 0)
        over_sy = max(-sy, 0)
        over_dx = max(-dx, 0)
        over_dy = max(-dy, 0)

        if over_sx > 0 or over_dx > 0:
            cw -= max(over_sx, over_dx)
            if over_sx > 0:
                sx = 0
            if over_dx > 0:
                dx = 0

        if over_sy > 0 or over_dy > 0:
            ch -= max(over_sy, over_dy)
            if over_sy > 0:
                sy = 0
            if over_dy > 0:
                dy = 0

        over_sx = max(sx + cw - sw, 0)
        over_sy = max(sx + ch - sh, 0)
        over_dx = max(dx + cw - dw, 0)
        over_dy = max(dx + ch - dh, 0)

        if over_sx > 0 or over_dx > 0:
            cw -= max(over_sx, over_dx)

        if over_sy > 0 or over_dy > 0:
            ch -= max(over_sy, over_dy)

        if cw > 0 and ch > 0:
            return sx, sy, dx, dy, cw, ch
        else:
            return None
