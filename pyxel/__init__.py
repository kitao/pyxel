import os
import platform

_system = platform.system()

if _system == "Darwin":
    from .lib.macos.pyxel_wrapper import *  # type: ignore  # noqa F403

elif _system == "Windows":
    sdl2_dir = os.path.join(os.path.dirname(__file__), "lib/windows")
    if hasattr(os, "add_dll_directory"):  # for Python 3.8 or later
        os.add_dll_directory(sdl2_dir)
    os.environ["PATH"] = os.pathsep.join([sdl2_dir, os.environ["PATH"]])
    from .lib.windows.pyxel_wrapper import *  # type: ignore  # noqa F403

elif _system == "Linux":
    from .lib.linux.pyxel_wrapper import *  # type: ignore  # noqa F403

else:
    raise Exception("unsupported platform")


from .cli import cli  # noqa E402

if __name__ == "__main__":
    cli()
