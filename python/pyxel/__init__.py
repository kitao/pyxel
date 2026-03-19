import sys

if sys.platform == "linux":
    import ctypes
    import os

    try:
        ctypes.CDLL("libSDL2-2.0.so.0", mode=ctypes.RTLD_GLOBAL)
    except OSError:
        _sdl2_path = os.path.join(os.path.dirname(__file__), "libs", "libSDL2-2.0.so.0")
        try:
            ctypes.CDLL(_sdl2_path, mode=ctypes.RTLD_GLOBAL)
        except OSError:
            pass

from .pyxel_binding import *  # type: ignore  # noqa: F403
