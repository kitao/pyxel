from . import canvas_expansion  # noqa: F401
from .app import App


def edit_pyxel_resource_file(filename, palette_file=None):
    if palette_file is not None:
        with open(palette_file) as file:
            palette = [int(line.lstrip("#"), 16) for line in file.read().splitlines()]
    else:
        palette = None

    App(filename, palette)
