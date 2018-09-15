import argparse
import sys

import pyxel
from pyxel.editor.editor_app import EditorApp


def run():
    parser = argparse.ArgumentParser(
        prog="pyxel",
        usage="pyxel resource_file [-h] [-w app_file]",
        description="Pyxel Resource Editor {}".format(pyxel.VERSION),
        add_help=True,
    )
    parser.add_argument("resource_file", help="Pyxel resource file")
    parser.add_argument(
        "-w",
        "--watch",
        help="Pyxel app file to be watched",
        metavar="app_file",
        dest="app_file",
    )

    args = parser.parse_args()

    EditorApp(args.resource_file, args.app_file)


if __name__ == "__main__":
    sys.argv[1:] = ["test_resource", "-w", "test_app"]
    run()
