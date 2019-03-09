def init_module():
    import ctypes
    import os
    import platform
    import sys

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

    setup_apis(sys.modules[__name__], lib)


def setup_apis(module, lib):
    #
    # System
    #
    module.width_getter = lib.width_getter
    module.height_getter = lib.height_getter
    module.frame_count_getter = lib.frame_count_getter

    module.init = lib.init

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


init_module()


if __name__ == "__main__":
    import ctypes

    colors = (ctypes.c_int * 16)()

    colors[0] = 0x000000
    colors[1] = 0x1D2B53
    colors[2] = 0x7E2553
    colors[3] = 0x008751
    colors[4] = 0xAB5236
    colors[5] = 0x5F574F
    colors[6] = 0xC2C3C7
    colors[7] = 0xFFF1E8
    colors[8] = 0xFF004D
    colors[9] = 0xFFA300
    colors[10] = 0xFFEC27
    colors[11] = 0x00E436
    colors[12] = 0x29ADFF
    colors[13] = 0x83769C
    colors[14] = 0xFF77A8
    colors[15] = 0xFFCCAA

    init(  # noqa: F821
        400,
        300,
        ctypes.create_string_buffer("This is caption".encode("utf-8")),
        1,
        colors,
        2,
        3,
        4,
    )

    global x
    x = 0

    def update():
        global x
        x += 1

    def draw():
        cls(0)  # noqa: F821
        rect(x, 30, 300, 50, 8)  # noqa: F821
        rectb(10, 70, 300, 100, 10)  # noqa: F821
        pix(x, 10, 7)  # noqa: F821

    run(update, draw)  # noqa: F821
