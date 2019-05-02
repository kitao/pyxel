import ctypes
import os
import platform


def load_library():
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

    return ctypes.cdll.LoadLibrary(lib_path)


_lib = load_library()


def get_constant_number(name):
    c_name = ctypes.create_string_buffer(name.encode("utf-8"))
    return _lib.get_constant_number(c_name)


def get_constant_string(name):
    c_name = ctypes.create_string_buffer(name.encode("utf-8"))
    return _lib.get_constant_string(c_name)


if __name__ == "__main__":
    import pyxel

    class App:
        def __init__(self):
            pyxel.init(256, 256, caption="HOGE")  # noqa: F821

            self.x = 0

            pyxel.image(0).load(0, 0, "../examples/assets/cat_16x16.png")

            pyxel.run(self.update, self.draw)  # noqa: F821

        def update(self):
            self.x += 1

            if pyxel.btnp(pyxel.MOUSE_LEFT_BUTTON):
                print("left button pressed")

            if pyxel.btnr(pyxel.MOUSE_LEFT_BUTTON):
                print("left button released")

            if pyxel.btnp(pyxel.KEY_Q):
                pyxel.quit()

        def draw(self):
            pyxel.cls(0)  # noqa: F821
            pyxel.rect(self.x, 30, 300, 50, 8)  # noqa: F821
            pyxel.rectb(10, 70, 300, 100, pyxel.frame_count % 16)  # noqa: F821

            pyxel.blt(0, 100, 3, 0, 0, 256, 64)
            pyxel.pix(self.x, 10, 7)  # noqa: F821
            pyxel.pix(pyxel.mouse_x, pyxel.mouse_y, 7)  # noqa: F821

            pyxel.blt(100, 150, 0, 0, 0, 16, 16)
            pyxel.blt(120, 150, 0, 0, 0, 16, 16, 5)

            pyxel.line(10, 15, 50, 30, 7)  # noqa: F821
            pyxel.line(10, 15, 50, 100, 8)  # noqa: F821
            pyxel.circ(40, 40, 20, 9)  # noqa: F821
            pyxel.circb(50, 80, 10, 9)  # noqa: F821

            pyxel.text(50, 120, "abcdABCD", 7)

    App()
