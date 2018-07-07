from .glwrapper import GLTexture


class Image:
    def __init__(self, *args):
        if len(args) == 1:
            data = args[0]
            width = len(data[0])
            height = len(data)
        elif len(args) == 2:
            width, height = args
            data = None
        else:
            raise ValueError('invalid image argument')

        self._tex = GLTexture(width, height, 1, nearest=True)
        self._data = self._tex.data

        if data:
            self._data[0:height, 0:width] = [
                list(map(lambda x: int(x, 16), line)) for line in data
            ]

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

    def save(self):
        # todo
        pass

    def load(self):
        self._tex.update()
        # todo
