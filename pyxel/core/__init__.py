def init_module():
    import ctypes
    import platform
    import sys

    lib_name = "libpyxelcore_"
    system = platform.system()

    if system == "Darwin":
        lib_name += "darwin"
        lib_ext = ".dylib"
    elif system == "Windows":
        lib_name += "windows"
        lib_ext = ".dll"
    elif system == "Linux":
        lib_name += "linux"
        lib_ext = ".so"
    else:
        raise RuntimeError("unsupported platform: {}".format(system))

    lib_name += "_amd64" if platform.architecture()[0] == "64bit" else "_386"
    lib_name += lib_ext

    lib = ctypes.cdll.LoadLibrary("./libpyxelcore_darwin_amd64.dylib")

    module = sys.modules[__name__]
    module.test = lib.test


init_module()

if __name__ == "__main__":
    test(400, 300)
