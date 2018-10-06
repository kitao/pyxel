import sys

import pyxel
from pyxel.editor.app import App


def run():
    if len(sys.argv) == 2:
        App(sys.argv[1])
    else:
        print("usage: pyxeleditor pyxel_resource_file")
        print("\n")
        print("Pyxel Editor {}".format(pyxel.VERSION))
        print("Please specify an arbitrary file name to run Pyxel Editor")
        print("e.g. pyxeleditor my_resource")


if __name__ == "__main__":
    sys.argv[1:] = ["test_resource"]
    run()
