import platform

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


from .cli import cli  # noqa E402

if __name__ == "__main__":
    cli()
