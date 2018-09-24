import argparse
import sys

import pyxel
from pyxel.editor.editor_app import EditorApp


def run():
    parser = argparse.ArgumentParser(
        prog="pyxeleditor",
        usage="pyxeleditor resource_file [-h]",
        description="Pyxel Editor {}".format(pyxel.VERSION),
        add_help=True,
    )
    parser.add_argument("resource_file", help="Pyxel resource file (.pyxel)")

    if len(sys.argv) == 1:
        parser.print_help()
        sys.exit()

    args = parser.parse_args()
    EditorApp(args.resource_file)


if __name__ == "__main__":
    sys.argv[1:] = ["test_resource"]
    run()
