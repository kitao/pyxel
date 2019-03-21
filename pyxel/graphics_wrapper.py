def setup_apis(module, lib):
    import ctypes

    def image_wrapper(img, *, system=False):
        # if not system and img == RENDERER_IMAGE_COUNT - 1:
        #    raise ValueError("image bank {} is reserved for system".format(img))

        return module._image_list[img]

    def tilemap_wrapper():
        pass

    def clip_wrapper(self, x1=None, y1=None, x2=None, y2=None):
        if x1 is None:
            lib.clip0()
        else:
            lib.clip(x1, y1, x2, y2)

    def pal_wrapper(self, col1=None, col2=None):
        if col1 is None:
            lib.pal0()
        else:
            lib.pal(col1, col2)

    lib.image.restype = ctypes.c_void_p
    module.image = image_wrapper

    module.tilemap = tilemap_wrapper
    module.clip = clip_wrapper
    module.pal = pal_wrapper
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
