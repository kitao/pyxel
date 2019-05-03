def setup_apis(module, lib):
    import ctypes

    def image_wrapper(img, *, system=False):
        return module._image_list[img]

    def tilemap_wrapper(tm):
        return module._tilemap_list[tm]

    def clip_wrapper(x1=None, y1=None, x2=None, y2=None):
        if x1 is None:
            lib.clip0()
        else:
            lib.clip(x1, y1, x2, y2)

    def pal_wrapper(col1=None, col2=None):
        if col1 is None:
            lib.pal0()
        else:
            lib.pal(col1, col2)

    def text_wrapper(x, y, s, col):
        c_s = ctypes.create_string_buffer(s.encode("utf-8"))
        lib.text(x, y, c_s, col)

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
    module.text = text_wrapper
