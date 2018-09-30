import sys

import pyxel
from pyxel.editor.editor_app import EditorApp


def run():
    if len(sys.argv) == 2:
        EditorApp(sys.argv[1])
    else:
        print("usage: pyxeleditor pyxe_resource_file")
        print("\n")
        print("Pyxel Editor {}".format(pyxel.VERSION))
        print("Please specify a Pyxel resource file (.pyxel)")
        print("e.g. pyxeleditor my_resource.pyxel")


if __name__ == "__main__":
    sys.argv[1:] = ["test_resource"]
    run()
