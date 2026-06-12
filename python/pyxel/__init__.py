import sys

if sys.platform == "linux":
    import ctypes
    from pathlib import Path

    # Preload system SDL2 with RTLD_GLOBAL so the binding below resolves its
    # symbols, falling back to the bundled copy and deferring failures to import
    try:
        ctypes.CDLL("libSDL2-2.0.so.0", mode=ctypes.RTLD_GLOBAL)
    except OSError:
        _sdl2_path = Path(__file__).parent / "libs" / "libSDL2-2.0.so.0"
        try:
            ctypes.CDLL(str(_sdl2_path), mode=ctypes.RTLD_GLOBAL)
        except OSError:
            pass

from .pyxel_binding import *  # type: ignore  # noqa: F403
