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
_setup_api("_get_constant_number", c_int32, [c_char_p])
_setup_api("_get_constant_string", None, [c_char_p, c_int32, c_char_p])

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
_setup_api("flip", None, [])

_setup_api("_drop_file_getter", None, [c_char_p, c_int32])
_setup_api("_caption", None, [c_char_p])

#
# Resource
#
_setup_api("save", None, [c_char_p])
_setup_api("load", None, [c_char_p])

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
_setup_api("play_pos", c_int32, [c_int32])
_setup_api("play1", None, [c_int32] * 3)
_setup_api("play", None, [c_int32, POINTER(c_int32)] + [c_int32] * 2)
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
_setup_api("image_load", None, [c_void_p] + [c_int32] * 2 + [c_char_p])
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

#
# Sound class
#
_setup_api("sound_note_getter", POINTER(c_int32), [c_void_p])
_setup_api("sound_note_length_getter", c_int32, [c_void_p])
_setup_api("sound_note_length_setter", None, [c_void_p, c_int32])
_setup_api("sound_tone_getter", POINTER(c_int32), [c_void_p])
_setup_api("sound_tone_length_getter", c_int32, [c_void_p])
_setup_api("sound_tone_length_setter", None, [c_void_p, c_int32])
_setup_api("sound_volume_getter", POINTER(c_int32), [c_void_p])
_setup_api("sound_volume_length_getter", c_int32, [c_void_p])
_setup_api("sound_volume_length_setter", None, [c_void_p, c_int32])
_setup_api("sound_effect_getter", POINTER(c_int32), [c_void_p])
_setup_api("sound_effect_length_getter", c_int32, [c_void_p])
_setup_api("sound_effect_length_setter", None, [c_void_p, c_int32])
_setup_api("sound_speed_getter", c_int32, [c_void_p])
_setup_api("sound_speed_setter", None, [c_void_p, c_int32])

_setup_api("sound_set", None, [c_void_p] + [c_char_p] * 4 + [c_int32])
_setup_api("sound_set_note", None, [c_void_p, c_char_p])
_setup_api("sound_set_tone", None, [c_void_p, c_char_p])
_setup_api("sound_set_volume", None, [c_void_p, c_char_p])
_setup_api("sound_set_effect", None, [c_void_p, c_char_p])

#
# Music class
#
_setup_api("music_ch0_getter", POINTER(c_int32), [c_void_p])
_setup_api("music_ch0_length_getter", c_int32, [c_void_p])
_setup_api("music_ch0_length_setter", None, [c_void_p, c_int32])
_setup_api("music_ch1_getter", POINTER(c_int32), [c_void_p])
_setup_api("music_ch1_length_getter", c_int32, [c_void_p])
_setup_api("music_ch1_length_setter", None, [c_void_p, c_int32])
_setup_api("music_ch2_getter", POINTER(c_int32), [c_void_p])
_setup_api("music_ch2_length_getter", c_int32, [c_void_p])
_setup_api("music_ch2_length_setter", None, [c_void_p, c_int32])
_setup_api("music_ch3_getter", POINTER(c_int32), [c_void_p])
_setup_api("music_ch3_length_getter", c_int32, [c_void_p])
_setup_api("music_ch3_length_setter", None, [c_void_p, c_int32])

_setup_api("music_set", None, [c_void_p] + [POINTER(c_int32), c_int32] * 4)
