from .constants import (
    DEFAULT_BORDER_COLOR,
    DEFAULT_BORDER_WIDTH,
    DEFAULT_CAPTION,
    DEFAULT_FPS,
    DEFAULT_PALETTE,
    DEFAULT_SCALE,
    VERSION,
)


def init(
    width,
    height,
    *,
    caption=DEFAULT_CAPTION,
    scale=DEFAULT_SCALE,
    palette=DEFAULT_PALETTE,
    fps=DEFAULT_FPS,
    border_width=DEFAULT_BORDER_WIDTH,
    border_color=DEFAULT_BORDER_COLOR
):
    import sys
    from .app import App
    from . import constants

    module = sys.modules[__name__]
    module.VERSION = VERSION  # to avoid 'unused' warning

    for k, v in constants.__dict__.items():
        if k.startswith("KEY_"):
            module.__dict__[k] = v

    module._app = App(
        module, width, height, caption, scale, palette, fps, border_width, border_color
    )
