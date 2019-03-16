def setup_apis(module, lib):
    import ctypes
    from pyxel.constants import (
        DEFAULT_BORDER_COLOR,
        DEFAULT_BORDER_WIDTH,
        DEFAULT_CAPTION,
        DEFAULT_FPS,
        DEFAULT_PALETTE,
        DEFAULT_SCALE,
    )

    module.width_getter = lib.width_getter
    module.height_getter = lib.height_getter
    module.frame_count_getter = lib.frame_count_getter

    def init_wrapper(
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
        c_caption = ctypes.create_string_buffer("This is caption".encode("utf-8"))

        c_palette = (ctypes.c_int * 16)()
        for i in range(16):
            c_palette[i] = palette[i]

        lib.init(
            width, height, c_caption, scale, c_palette, fps, border_width, border_color
        )

    module.init = init_wrapper

    def run_wrapper(update, draw):
        lib.run(ctypes.CFUNCTYPE(None)(update), ctypes.CFUNCTYPE(None)(draw))

    module.run = run_wrapper

    module.quit = lib.quit
