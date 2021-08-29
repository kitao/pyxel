import platform

system = platform.system()

if system == "Darwin":
    from .bin.mac.pyxel_extension import *  # type: ignore  # noqa F403
elif system == "Windows":
    from .bin.windows.pyxel_extension import *  # type: ignore  # noqa F403
elif system == "Linux":
    from .bin.linux.pyxel_extension import *  # type: ignore  # noqa F403
else:
    raise Exception("unsupported platform")
