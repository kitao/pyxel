import os
import sys

import pyxel
from pyxel.editor.app import App


def run():
    palette = None
    filename = "my_resource"

    for arg in sys.argv[1:]:
        if arg.startswith("-"):
            if arg in ("-v", "--version"):
                print("Pyxel Editor {}".format(pyxel.VERSION))
                return
            elif arg in ("-h", "--help"):
                print("Usage: pyxeleditor [option] [pyxel_resource_file]")
                print("Options:")
                print(" -h, --help     This help text")
                print(" -v, --version  Show version number and quit")
                print(" --palette file Load custom palette from file")
                return
            elif arg.startswith("--palette="):
                palette_file = arg[len("--palette="):]
                with open(palette_file) as file:
                    palette = [int(line.lstrip("#"), 16) for line in file.read().splitlines()]
        else:
            filename = arg

    App(filename, palette)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.argv[1:] = [os.path.join(os.path.dirname(__file__), "assets/test_resource")]

    run()
