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

    module = sys.modules[__name__]
    module.test = lib.test
    module.Init = lib.Init


init_module()

if __name__ == "__main__":
    import ctypes

    colors = (ctypes.c_int * 16)()
    colors[0] = 111
    colors[1] = 222

    Init(  # noqa: F821
        -100,
        0,
        ctypes.create_string_buffer("This is caption".encode("utf-8")),
        1,
        colors,
        2,
        3,
        4,
    )
    print(test(400, 300))  # noqa: F821
