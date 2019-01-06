import os
import sys

import pyxel
from pyxel.editor.app import App


def run():
    arg = sys.argv[1] if len(sys.argv) >= 2 else ""

    if arg.startswith("-"):
        if arg == "-v" or arg == "--version":
            print("Pyxel Editor {}".format(pyxel.VERSION))
        else:
            print("Usage: pyxeleditor [option] [pyxel_resource_file]")
            print("Options:")
            print(" -h, --help     This help text")
            print(" -v, --version  Show version number and quit")
    else:
        filename = arg or os.path.join(os.getcwd(), "my_resource")
        App(filename)


if __name__ == "__main__":
    sys.argv[1:] = ["assets/test_resource"]
    run()
