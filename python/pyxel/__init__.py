import os
import sys

from .pyxel_wrapper import *  # type: ignore  # noqa: F403

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


_set_reset_func(_reset)  # type: ignore  #noqa: F405
