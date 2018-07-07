from .glwrapper import GLTexture


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
        self._tex.update()

    def save(self):
        # todo
        pass

    def load(self):
        self._tex.update()
        # todo
