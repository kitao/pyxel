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
    module.get_width = lib.get_width
    module.get_height = lib.get_height
    module.get_frame_count = lib.get_frame_count

    module.init = lib.init
    module.run = lib.run
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
    run()  # noqa: F821
