from pyxel.constants import (
    DEFAULT_BORDER_COLOR,
    DEFAULT_BORDER_WIDTH,
    DEFAULT_CAPTION,
    DEFAULT_FPS,
    DEFAULT_PALETTE,
    DEFAULT_SCALE,
)


def setup_apis(module):
    import ctypes
    import os
    import platform

    lib_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "bin")
    lib_name = "libpyxelcore"
    system = platform.system()

    if system == "Darwin":
        lib_path = os.path.join(lib_dir, "macos", lib_name) + ".dylib"
    elif system == "Windows":
        win_dir = "win64" if platform.architecture()[0] == "64bit" else "win32"
        lib_path = os.path.join(lib_dir, win_dir, lib_name) + ".dll"
        dll_path = os.path.join(lib_dir, win_dir)
        os.environ["PATH"] = dll_path + os.pathsep + os.environ["PATH"]
    elif system == "Linux":
        lib_path = os.path.join(lib_dir, "linux", lib_name) + ".so"
    else:
        raise RuntimeError("unsupported platform: {}".format(system))

    print("load library: {}".format(lib_path))
    lib = ctypes.cdll.LoadLibrary(lib_path)

    #
    # System
    #
    module.width_getter = lib.width_getter
    module.height_getter = lib.height_getter
    module.frame_count_getter = lib.frame_count_getter

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
        c_caption = ctypes.create_string_buffer("This is caption".encode("utf-8"))

        c_palette = (ctypes.c_int * 16)()
        for i in range(16):
            c_palette[i] = palette[i]

        lib.init(
            width, height, c_caption, scale, c_palette, fps, border_width, border_color
        )

    module.init = init

    def run(update, draw):
        lib.run(ctypes.CFUNCTYPE(None)(update), ctypes.CFUNCTYPE(None)(draw))

    module.run = run

    module.quit = lib.quit

    #
    # Resource
    #

    #
    # Input
    #

    #
    # Graphics
    #
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
    # Audio
    #

    #
    # Image class
    #

    #
    # Tilemap class
    #

    #
    # Sound class
    #

    #
    # Music class
    #


if __name__ == "__main__":
    import sys

    setup_apis(sys.modules[__name__])

    class App:
        def __init__(self):
            init(400, 300)  # noqa: F821

            self.x = 0

            run(self.update, self.draw)  # noqa: F821

        def update(self):
            self.x += 1

        def draw(self):
            cls(0)  # noqa: F821
            rect(self.x, 30, 300, 50, 8)  # noqa: F821
            rectb(10, 70, 300, 100, 10)  # noqa: F821
            pix(self.x, 10, 7)  # noqa: F821

    App()
