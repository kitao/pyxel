def setup_apis(module, lib):
    def clip_wrapper():
        pass

    module.clip = lib.clip

    def pal_wrapper():
        pass

    module.image = lib.image
    module.tilemap = lib.tilemap
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
