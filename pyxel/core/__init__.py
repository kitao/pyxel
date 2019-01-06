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
        os.environ["PATH"] += os.pathsep + os.path.join(lib_dir, win_dir)
    elif system == "Linux":
        lib_path = os.path.join(lib_dir, "linux", lib_name) + ".so"
    else:
        raise RuntimeError("unsupported platform: {}".format(system))

    print("load library: {}".format(lib_path))
    lib = ctypes.cdll.LoadLibrary(lib_path)

    module = sys.modules[__name__]
    module.test = lib.test


init_module()

if __name__ == "__main__":
    print(test(400, 300))  # noqa: F821
