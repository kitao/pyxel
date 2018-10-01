import sys

import pyxel
from pyxel.editor.editor_app import EditorApp


def run():
    if len(sys.argv) == 2:
        EditorApp(sys.argv[1])
    else:
        print("usage: pyxeleditor pyxel_resource_file")
        print("\n")
        print("Pyxel Editor {}".format(pyxel.VERSION))
        print("Please specify an arbitrary file name")
        print("e.g. pyxeleditor my_resource")


if __name__ == "__main__":
    sys.argv[1:] = ["test_resource"]
    run()
