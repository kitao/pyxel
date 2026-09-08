import sys

if sys.platform == "linux":
    import ctypes
    from pathlib import Path

    # Preload SDL2 with RTLD_GLOBAL so the binding can resolve its symbols.
    # Fall back to the bundled copy and let the binding surface import failures.
    try:
        ctypes.CDLL("libSDL2-2.0.so.0", mode=ctypes.RTLD_GLOBAL)
    except OSError:
        _sdl2_path = Path(__file__).parent / "libs" / "libSDL2-2.0.so.0"
        try:
            ctypes.CDLL(str(_sdl2_path), mode=ctypes.RTLD_GLOBAL)
        except OSError:
            pass

from . import cube as _cube
from .pyxel_binding import *  # type: ignore

# Restore the Python cube package overwritten by the native star import.
cube = _cube
del _cube
