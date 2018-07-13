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

        rect = self._get_copy_rect(x, y, width, height)
        if not rect:
            return
        src_x, src_y, dest_x, dest_y, width, height = rect

        src_data = np.array(
            [list(map(lambda x: int(x, 16), line)) for line in data])
        src_data = src_data[src_y:src_y + height, src_x:src_x + width]
        self._data[dest_y:dest_y + height, dest_x:dest_x + width] = src_data

    def load(self, x, y, filename):
        dirname = os.path.dirname(inspect.stack()[-1].filename)
        filename = os.path.join(dirname, filename)

        pil_image = PIL.Image.open(filename).convert('RGB')
        pil_image.load()

        width, height = pil_image.size

        rect = self._get_copy_rect(x, y, width, height)
        if not rect:
            return
        src_x, src_y, dest_x, dest_y, width, height = rect

        palette = []
        for color in pyxel._app._palette:
            r = (color >> 16) & 0xff
            g = (color >> 8) & 0xff
            b = color & 0xff
            palette.extend((r, g, b))
        palette += [0] * 240 * 3

        palette_image = PIL.Image.new('P', (1, 1), 0)
        palette_image.putpalette(palette)

        im = pil_image.im.convert('P', 0, palette_image.im)
        pil_image = pil_image._new(im)
        pil_image = pil_image.crop((src_x, src_y, src_x + width,
                                    src_y + height))

        src_data = np.array(pil_image.getdata()).reshape(height, width)
        self._data[dest_y:dest_y + height, dest_x:dest_x + width] = src_data

    def copy(self, x, y, no, width, height):
        image = pyxel.image(no)

        rect = self._get_copy_rect(x, y, width, height)
        if not rect:
            return
        src_x, src_y, dest_x, dest_y, width, height = rect

        src_data = image._data[src_y:src_y + height, src_x:src_x + width]
        self._data[dest_y:dest_y + height, dest_x:dest_x + width] = src_data

    def _get_copy_rect(self, dest_x, dest_y, width, height):
        dest_width = self._tex.width
        dest_height = self._tex.height

        if (dest_x >= dest_width or dest_y >= dest_height or dest_x + width < 0
                or dest_y + height < 0):
            return None

        src_x = 0
        src_y = 0

        if dest_x < 0:
            src_x = -dest_x
            width += dest_x
            dest_x = 0

        if dest_y < 0:
            src_y = -dest_y
            height += dest_y
            dest_y = 0

        if dest_x + width > dest_width:
            width -= dest_x + width - dest_width

        if dest_y + height > dest_height:
            height -= dest_y + height - dest_height

        return (src_x, src_y, dest_x, dest_y, width, height)
