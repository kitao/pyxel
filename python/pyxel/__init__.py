import ctypes
import os
import sys

if sys.platform == "linux":
    try:
        ctypes.CDLL("libSDL2-2.0.so.0", mode=ctypes.RTLD_GLOBAL)
    except OSError:
        _sdl2_path = os.path.join(os.path.dirname(__file__), "libs", "libSDL2-2.0.so.0")
        try:
            ctypes.CDLL(_sdl2_path, mode=ctypes.RTLD_GLOBAL)
        except OSError:
            pass

from .pyxel_binding import *  # type: ignore  # noqa: F403

_reset_info = {
    "exec": sys.executable,
    "cwd": os.getcwd(),
    "argv": getattr(sys, "orig_argv", sys.argv[:]),
}


def _reset():
    if WATCH_STATE_FILE_ENV in os.environ:  # type: ignore  # noqa: F405
        os._exit(WATCH_RESET_EXIT_CODE)  # type: ignore  # noqa: F405

    if sys.platform == "darwin":
        try:
            with open(os.devnull, "wb") as f:
                os.dup2(f.fileno(), 2)
        except OSError:
            pass

    import subprocess

    subprocess.Popen(
        [_reset_info["exec"]] + _reset_info["argv"][1:],
        cwd=_reset_info["cwd"],
        env=os.environ.copy(),
    )
    sys.exit(0)


_set_reset_func(_reset)  # type: ignore  # noqa: F405
