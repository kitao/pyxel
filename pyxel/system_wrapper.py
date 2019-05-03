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

    from . import core

    COLOR_COUNT = core.get_constant_number("COLOR_COUNT")
    IMAGE_BANK_COUNT = core.get_constant_number("IMAGE_BANK_COUNT")
    TILEMAP_BANK_COUNT = core.get_constant_number("TILEMAP_BANK_COUNT")
    SOUND_BANK_COUNT = core.get_constant_number("SOUND_BANK_COUNT")
    MUSIC_BANK_COUNT = core.get_constant_number("MUSIC_BANK_COUNT")

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

        c_palette = (ctypes.c_int * COLOR_COUNT)()
        for i in range(COLOR_COUNT):
            c_palette[i] = palette[i]

        lib.init(
            width, height, c_caption, scale, c_palette, fps, border_width, border_color
        )

        lib.image.restype = ctypes.c_void_p
        lib.tilemap.restype = ctypes.c_void_p
        lib.sound.restype = ctypes.c_void_p
        lib.music.restype = ctypes.c_void_p

        module._image_list = []
        for i in range(IMAGE_BANK_COUNT):
            module._image_list.append(module.Image(ctypes.c_void_p(lib.image(i, True))))

        module._tilemap_list = []
        for i in range(TILEMAP_BANK_COUNT):
            module._tilemap_list.append(module.Tilemap(ctypes.c_void_p(lib.tilemap(i))))

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
