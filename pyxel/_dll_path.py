import os
import platform

if platform.system() == "Windows":
    lib_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "core", "bin")
    win_dir = "win64" if platform.architecture()[0] == "64bit" else "win32"
    dll_path = os.path.join(lib_dir, win_dir)
    os.environ["PATH"] = dll_path + os.pathsep + os.environ["PATH"]
