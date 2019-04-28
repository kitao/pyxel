def setup_apis(module, lib):
    import ctypes
    from pyxel import (
        DEFAULT_BORDER_COLOR,
        DEFAULT_BORDER_WIDTH,
        DEFAULT_CAPTION,
        DEFAULT_FPS,
        DEFAULT_PALETTE,
        DEFAULT_SCALE,
    )

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
        c_caption = ctypes.create_string_buffer(caption.encode("utf-8"))

        c_palette = (ctypes.c_int * 16)()
        for i in range(16):
            c_palette[i] = palette[i]

        lib.init(
            width, height, c_caption, scale, c_palette, fps, border_width, border_color
        )

        lib.image.restype = ctypes.c_void_p

        module._image_list = []
        for i in range(4):
            module._image_list.append(module.Image(lib.image(i, True)))

        # init tilemap
        # init sound
        # init music

    def run_wrapper(update, draw):
        def update_wrapper():
            module.width = lib.width_getter()
            module.height = lib.height_getter()
            module.frame_count = lib.frame_count_getter()

            module.mouse_x = lib.mouse_x_getter()
            module.mouse_y = lib.mouse_y_getter()

            update()

        lib.run(ctypes.CFUNCTYPE(None)(update_wrapper), ctypes.CFUNCTYPE(None)(draw))

    module.width_getter = lib.width_getter
    module.height_getter = lib.height_getter
    module.frame_count_getter = lib.frame_count_getter
    module.init = init_wrapper
    module.run = run_wrapper
    module.quit = lib.quit
