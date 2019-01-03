def init_module():
    import ctypes
    import os
    import platform
    import sys

    lib_name = "libpyxelcore"
    system = platform.system()

    if system == "Darwin":
        lib_name += "_darwin"
        lib_ext = ".dylib"
    elif system == "Windows":
        lib_name += "_windows"
        lib_ext = ".dll"
    elif system == "Linux":
        lib_name += "_linux"
        lib_ext = ".so"
    else:
        raise RuntimeError("unsupported platform: {}".format(system))

    lib_name = os.path.join(os.path.dirname(__file__), lib_name)
    lib_name += "_amd64" if platform.architecture()[0] == "64bit" else "_386"
    lib_name += lib_ext

    lib = ctypes.cdll.LoadLibrary(lib_name)

    module = sys.modules[__name__]
    module.test = lib.test


init_module()

if __name__ == "__main__":
    test(400, 300)  # noqa: F821
