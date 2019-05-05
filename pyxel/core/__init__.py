import os
import platform
import sys
from ctypes import CFUNCTYPE, POINTER, c_char_p, c_int32, c_void_p, cdll


def _load_library():
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

    return cdll.LoadLibrary(lib_path)


_lib = _load_library()
_module = sys.modules[__name__]


def _setup_api(name, restype, argtypes):
    api = _module.__dict__[name] = eval("_lib.{0}".format(name))
    api.restype = restype
    api.argtypes = argtypes


#
# Constants
#
_setup_api("get_constant_number", c_int32, [c_char_p])
_setup_api("get_constant_string", c_char_p, [c_char_p])

#
# System
#
_setup_api("width_getter", c_int32, [])
_setup_api("height_getter", c_int32, [])
_setup_api("frame_count_getter", c_int32, [])

_setup_api(
    "init", None, [c_int32] * 2 + [c_char_p, c_int32, c_int32 * 16] + [c_int32] * 3
)
_setup_api("run", None, [CFUNCTYPE(None), CFUNCTYPE(None)])
_setup_api("quit", None, [])

#
# Resource
#
_setup_api("save", c_int32, [c_char_p])
_setup_api("load", c_int32, [c_char_p])

#
# Input
#
_setup_api("mouse_x_getter", c_int32, [])
_setup_api("mouse_y_getter", c_int32, [])

_setup_api("btn", c_int32, [c_int32])
_setup_api("btnp", c_int32, [c_int32] * 3)
_setup_api("btnr", c_int32, [c_int32])
_setup_api("mouse", None, [c_int32])

#
# Graphics
#
_setup_api("image", c_void_p, [c_int32] * 2)
_setup_api("tilemap", c_void_p, [c_int32])
_setup_api("clip0", None, [])
_setup_api("clip", None, [c_int32] * 4)
_setup_api("pal0", None, [])
_setup_api("pal", None, [c_int32] * 2)
_setup_api("cls", None, [c_int32])
_setup_api("pix", None, [c_int32] * 3)
_setup_api("line", None, [c_int32] * 5)
_setup_api("rect", None, [c_int32] * 5)
_setup_api("rectb", None, [c_int32] * 5)
_setup_api("circ", None, [c_int32] * 4)
_setup_api("circb", None, [c_int32] * 4)
_setup_api("blt", None, [c_int32] * 8)
_setup_api("bltm", None, [c_int32] * 8)
_setup_api("text", None, [c_int32, c_int32, c_char_p, c_int32])

#
# Audio
#
_setup_api("sound", c_void_p, [c_int32] * 2)
_setup_api("music", c_void_p, [c_int32])
_setup_api("play", None, [c_int32] * 3)
_setup_api("playm", None, [c_int32] * 2)
_setup_api("stop", None, [c_int32])

#
# Image class
#
_setup_api("image_width_getter", c_int32, [c_void_p])
_setup_api("image_height_getter", c_int32, [c_void_p])
_setup_api("image_data_getter", POINTER(c_int32), [c_void_p])

_setup_api("image_get", c_int32, [c_void_p] + [c_int32] * 2)
_setup_api("image_set1", None, [c_void_p] + [c_int32] * 3)
_setup_api("image_set", None, [c_void_p] + [c_int32] * 2 + [POINTER(c_char_p), c_int32])
_setup_api("image_load", c_int32, [c_void_p] + [c_int32] * 2 + [c_char_p])
_setup_api("image_copy", None, [c_void_p] + [c_int32] * 7)

#
# Tilemap class
#
_setup_api("tilemap_width_getter", c_int32, [c_void_p])
_setup_api("tilemap_height_getter", c_int32, [c_void_p])
_setup_api("tilemap_data_getter", POINTER(c_int32), [c_void_p])
_setup_api("tilemap_refimg_getter", c_int32, [c_void_p])
_setup_api("tilemap_refimg_setter", c_int32, [c_void_p, c_int32])

_setup_api("tilemap_get", c_int32, [c_void_p] + [c_int32] * 2)
_setup_api("tilemap_set1", None, [c_void_p] + [c_int32] * 3)
_setup_api(
    "tilemap_set", None, [c_void_p] + [c_int32] * 2 + [POINTER(c_char_p), c_int32]
)
_setup_api("tilemap_copy", None, [c_void_p] + [c_int32] * 7)


if __name__ == "__main__":
    import pyxel

    class App:
        def __init__(self):
            pyxel.init(256, 256, caption="HOGE")  # noqa: F821
            pyxel.mouse(True)

            self.x = 0

            pyxel.image(0).load(0, 0, "../examples/assets/cat_16x16.png")
            pyxel.image(1).load(0, 0, "../examples/assets/tileset_24x32.png")

            pyxel.image(3, system=True).set(0, 0, ["7777", "7777"])

            print("tilemap[0,0]=", pyxel.tilemap(0).get(0, 0))
            pyxel.tilemap(0).set(
                0,
                0,
                [
                    "022000002004001000060061062000040",
                    "042003020021022003000001002003060",
                ],
            )
            pyxel.tilemap(0).refimg = 1

            pyxel.image(3, system=True).data[0] = 8
            pyxel.tilemap(0).data[0] = 1

            pyxel.run(self.update, self.draw)  # noqa: F821

        def update(self):
            self.x += 1

            if pyxel.btnp(pyxel.KEY_Q):
                pyxel.quit()

        def draw(self):
            pyxel.cls(0)  # noqa: F821
            pyxel.rect(self.x, 30, 300, 50, 8)  # noqa: F821
            pyxel.rectb(10, 70, 300, 100, pyxel.frame_count % 16)  # noqa: F821

            pyxel.blt(0, 100, 3, 0, 0, 256, 64)
            pyxel.pix(self.x, 10, 7)  # noqa: F821
            pyxel.pix(pyxel.mouse_x, pyxel.mouse_y, 7)  # noqa: F821

            pyxel.blt(100.9, 150, 0, 0, 0, 16, 16)
            pyxel.blt(120, 150, 0, 0, 0, 16, 16, 5)

            pyxel.line(10, 15, 50, 30, 7)  # noqa: F821
            pyxel.line(10, 15, 50, 100, 8)  # noqa: F821
            pyxel.circ(40, 40, 20, 9)  # noqa: F821
            pyxel.circb(50, 80, 10, 9)  # noqa: F821

            pyxel.bltm(0, 150, 0, 0, 0, 11, 2, 2)

            pyxel.text(50, 120, "abcdABCD", 7)

    App()
