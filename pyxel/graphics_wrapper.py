def setup_apis(module, lib):
    module.clip = lib.clip
    module.pal = lib.pal
    module.cls = lib.cls
    module.pix = lib.pix
    module.line = lib.line
    module.rect = lib.rect
    module.rectb = lib.rectb
    module.circ = lib.circ
    module.circb = lib.circb
    module.blt = lib.blt
    module.bltm = lib.bltm
    module.text = lib.text

    #
    # Image class
    #
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

    #
    # Tilemap class
    #
    class Tilemap:
        width: int = 0
        height: int = 0
        data = None

        def __init__(self, tm):
            self.tm = tm

        def get(self, x: int, y: int) -> int:
            pass

        def set(self, x: int, y: int, data, refimg: int = None) -> None:
            pass

        def copy(self, x: int, y: int, tm: int, u: int, v: int, w: int, h: int) -> None:
            pass

    module.Tilemap = Tilemap
