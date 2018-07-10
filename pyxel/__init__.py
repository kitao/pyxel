VERSION = '0.5.0'

DEFAULT_CAPTION = 'Pyxel'
DEFAULT_SCALE = 4
DEFAULT_PALETTE = [
    0x000000, 0x1d2b53, 0x7e2553, 0x008751, 0xab5236, 0x5f574f, 0xc2c3c7,
    0xfff1e8, 0xff004d, 0xffa300, 0xffec27, 0x00e436, 0x29adff, 0x83769c,
    0xff77a8, 0xffccaa
]
DEFAULT_FPS = 30
DEFAULT_BORDER_WIDTH = 0
DEFAULT_BORDER_COLOR = 0x101018


def init(width,
         height,
         *,
         caption=DEFAULT_CAPTION,
         scale=DEFAULT_SCALE,
         palette=DEFAULT_PALETTE,
         fps=DEFAULT_FPS,
         border_width=DEFAULT_BORDER_WIDTH,
         border_color=DEFAULT_BORDER_COLOR):
    import sys
    from .image import Image
    from .sound import Sound
    from . import key
    from .app import App

    module = sys.modules[__name__]
    module.Image = Image
    module.Sound = Sound

    for k, v in key.__dict__.items():
        if k.startswith('KEY_'):
            module.__dict__[k] = v

    Image.set_palette(palette)

    App(module, width, height, caption, scale, palette, fps, border_width,
        border_color)
