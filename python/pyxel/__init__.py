import os
import sys

from .pyxel_wrapper import *  # type: ignore  # noqa: F403

_reset_info = {
    "exec": sys.executable,
    "env": os.environ.copy(),
    "cwd": os.getcwd(),
    "argv": getattr(sys, "orig_argv", sys.argv[:]),
}


def reset():
    try:
        import pyodide  # type: ignore  # noqa: F401
    except ImportError:
        pass
    else:  # Pyodide
        raise Exception("PYXEL_RESET")

    if WATCH_STATE_FILE_ENV in os.environ:  # type: ignore  # noqa: F405
        os._exit(WATCH_RESET_EXIT_CODE)  # type: ignore  # noqa: F405

    if sys.platform == "darwin":
        try:
            with open(os.devnull, "wb") as f:
                os.dup2(f.fileno(), 2)
        except OSError:
            pass

    _reset_info["env"][RESET_STATE_ENV] = window_state()  # type: ignore  #noqa: F405

    try:
        os.chdir(_reset_info["cwd"])
    except Exception:
        pass

    os.execvpe(
        _reset_info["exec"],
        [_reset_info["exec"]] + _reset_info["argv"][1:],
        _reset_info["env"],
    )
