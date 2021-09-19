import os
import platform
import subprocess
import sys

_system = platform.system()

if _system == "Darwin":
    _cli_dir = "bin/macos"
    from .lib.macos.pyxel_wrapper import *  # type: ignore  # noqa F403
elif _system == "Windows":
    _cli_dir = "bin/windows"
    from .lib.windows.pyxel_wrapper import *  # type: ignore  # noqa F403
elif _system == "Linux":
    _cli_dir = "bin/linux"
    from .lib.linux.pyxel_wrapper import *  # type: ignore  # noqa F403
else:
    raise Exception("unsupported platform")


def cli():
    cmd = os.path.join(_cli_dir, os.path.dirname(__file__), "pyxel")
    result = subprocess.run(cmd, *sys.argv)
    exit(result.returncode)
