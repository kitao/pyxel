import platform
import os

_system = platform.system()

def load(filename, *, image = True, tilemap = True, sound = True, music = True):
    load_(os.fspath(filename), image, tilemap, sound, music)

if _system == "Darwin":
    from .lib.macos.pyxel_wrapper import *  # type: ignore  # noqa F403
elif _system == "Windows":
    from .lib.windows.pyxel_wrapper import *  # type: ignore  # noqa F403
elif _system == "Linux":
    from .lib.linux.pyxel_wrapper import *  # type: ignore  # noqa F403
else:
    raise Exception("unsupported platform")
