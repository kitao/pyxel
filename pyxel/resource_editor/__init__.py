import argparse
import sys

import pyxel
from pyxel.resource_editor.editor_app import EditorApp


def run():
    parser = argparse.ArgumentParser(
        prog="pyxel",
        usage="pyxel resource_file [-h]",
        description="Pyxel Resource Editor {}".format(pyxel.VERSION),
        add_help=True,
    )
    parser.add_argument("resource_file", help="Pyxel resource file")
    args = parser.parse_args()
    EditorApp(args.resource_file)


if __name__ == "__main__":
    sys.argv[1:] = ["test_resource"]
    run()
