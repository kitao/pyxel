def setup_apis(module, lib):
    class Image:
        def __init__(self, img):
            self.img = img

        def get(self, x, y):
            return lib.Tilemap_get(self.img, x, y)

        def set(self, x, y, data):
            pass

        def load(self, x, y, filename):
            pass

        def copy(self, x, y, img, u, v, w, h):
            pass

    module.Image = Image
