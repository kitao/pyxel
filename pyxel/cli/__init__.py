import os
import runpy
import sys
import tempfile
import zipfile

import pyxel
from pyxel.editor import edit_pyxel_resource_file
from pyxel.examples import copy_pyxel_examples


def print_usage():
    print("Pyxel {}, a retro game engine for Python".format(pyxel.PYXEL_VERSION))
    print("usage:")
    print("    pyxel PYXEL_APP_FILE(.pyxapp)")
    print("    pyxel -run PYTHON_SCRIPT(.py)")
    print("    pyxel -edit PYXEL_RESOURCE_FILE(.pyxres)")
    print("    pyxel -edit PYXEL_RESOURCE_FILE(.pyxres) palette.hex")
    print("    pyxel -package PYXEL_APP_DIR STARTUP_SCRIPT(.py)")
    print("    pyxel -copy-pyxel-examples COPY_DEST_DIR")


def fileext(filename):
    os.path.splitext(filename)[1].lower()


def launch_pyxel_app_file(filename):
    with tempfile.TemporaryDirectory() as tmpdirname:
        zf = zipfile.ZipFile(filename)
        zf.extractall(tmpdirname)


def package_pyxel_app_file(dirname, filename):
    pass


def main():
    num_args = len(sys.argv)
    command = sys.argv[1].lower() if num_args > 1 else ""

    if num_args == 2:
        launch_pyxel_app_file(sys.argv[1])

    elif num_args == 3 and command == "-run":
        runpy.run_path(sys.argv[2])

    elif num_args in {3, 4} and command == "-edit":
        edit_pyxel_resource_file(*sys.argv[2:])

    elif num_args == 4 and command == "-package":
        # TODO: check dir
        package_pyxel_app_file(sys.argv[2], sys.argv[3])

    elif num_args == 3 and command == "-copy-pyxel-examples":
        # TODO: check dir
        copy_pyxel_examples(sys.argv[2])

    else:
        print_usage()


if __name__ == "__main__":
    main()
