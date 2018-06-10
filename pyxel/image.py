from .glwrapper import GLTexture


class Image:
    def __init__(self, width, height):
        self._width = width
        self._height = height
        self._tex = GLTexture(width, height, 1, nearest=True)
        self._data = self._tex.data

    @property
    def width(self):
        return self._width

    @property
    def height(self):
        return self._height

    @property
    def data(self):
        self._tex.refresh()
        return self._data

    def save(self):
        # todo
        pass

    def load(self):
        self._tex.refresh()
        # todo
