import os
import PIL.Image
from .glwrapper import GLTexture


class Image:
    _palette_image = None

    def __init__(self, width, height):
        self._tex = GLTexture(width, height, 1, nearest=True)
        self._data = self._tex.data

    @staticmethod
    def fromstring(data):
        width = len(data[0])
        height = len(data)

        image = Image(width, height)
        image.set(0, 0, data)

        return image

    @staticmethod
    def fromfile(filename):
        if not Image._palette_image:
            return

        dirname = os.path.dirname(__file__)
        filename = os.path.join(dirname, filename)

        pil_image = PIL.Image.open(filename).convert('RGB')
        pil_image.load()

        im = pil_image.im.convert('P', 0, Image._palette_image.im)
        pil_image = pil_image._new(im)

        width, height = pil_image.size
        image = Image(width, height)
        image._data.reshape(width * height)[:] = pil_image.getdata()

        return image

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

        self._data[y:height + y, x:width + x] = [
            list(map(lambda x: int(x, 16), line)) for line in data
        ]

    @staticmethod
    def set_palette(palette):
        pil_palette = []

        for color in palette:
            r = (color >> 16) & 0xff
            g = (color >> 8) & 0xff
            b = color & 0xff
            pil_palette.extend((r, g, b))

        pil_palette += [0] * 240 * 3

        Image._palette_image = PIL.Image.new('P', (1, 1), 0)
        Image._palette_image.putpalette(pil_palette)
