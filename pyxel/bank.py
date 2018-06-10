from .glwrapper import GLTexture


class Bank:
    def __init__(self, width, height):
        self._tex = GLTexture(width, height, 1, nearest=True)
        self._data = self._tex.data

    @property
    def data(self):
        self._tex.refresh()
        return self._data

    def save(self):
        pass

    def load(self):
        self._tex.refresh()
