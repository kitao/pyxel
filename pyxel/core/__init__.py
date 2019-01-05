def init_module():
    import ctypes
    import os
    import platform
    import sys

    lib_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "libpyxelcore")

    system = platform.system()
    if system == "Darwin":
        lib_path += "_darwin"
        lib_ext = ".dylib"
    elif system == "Windows":
        lib_path += "_windows"
        lib_ext = ".dll"
    elif system == "Linux":
        lib_path += "_linux"
        lib_ext = ".so"
    else:
        raise RuntimeError("unsupported platform: {}".format(system))

    lib_path += "_amd64" if platform.architecture()[0] == "64bit" else "_386"
    lib_path += lib_ext

    print("load library: {}".format(os.path.basename(lib_path)))
    lib = ctypes.cdll.LoadLibrary(lib_path)

    module = sys.modules[__name__]
    module.test = lib.test


init_module()

if __name__ == "__main__":
    print(test(400, 300))  # noqa: F821
