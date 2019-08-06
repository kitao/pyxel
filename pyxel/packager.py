import glob
import os
import platform
import shutil
import sys

import pyxel


def run():
    arg = sys.argv[1] if len(sys.argv) >= 2 else ""
    name = ""

    if not arg or arg.startswith("-"):
        if arg == "-v" or arg == "--version":
            print("Pyxel Packager {}".format(pyxel.VERSION))
            return
        else:
            print("Usage: pyxelpackager python_file")
            print("Options:")
            print(" -h, --help     This help text")
            print(" -v, --version  Show version number and quit")
            return

    dirname = os.path.dirname(arg) or "."
    filename = os.path.basename(arg)
    name = name or os.path.splitext(filename)[0]
    separator = ";" if platform.system() == "Windows" else ":"

    os.chdir(dirname)

    options = [
        "--clean",
        "--noconfirm",
        "--name={}".format(name),
        "--onefile",
        "--noconsole",
        "--add-data={}{}{}".format(
            pyxel.core._get_absolute_libpath(),
            separator,
            os.path.dirname(pyxel.core._get_relative_libpath()),
        ),
    ]

    assets = filter(os.path.isfile, glob.glob("assets/**", recursive=True))

    for asset in assets:
        options.append(
            "--add-data={}{}{}".format(
                os.path.abspath(asset), separator, os.path.dirname(asset)
            )
        )

    try:
        shutil.rmtree("dist", ignore_errors=True)
        _run_pyinstaller(options + [filename])
    finally:
        shutil.rmtree("build", ignore_errors=True)
        shutil.rmtree("__pycache__", ignore_errors=True)

        spec_file = "{}.spec".format(name)
        if os.path.exists(spec_file):
            os.remove(spec_file)


def _run_pyinstaller(args):
    import PyInstaller.__main__

    print("pyinstaller {}".format(" ".join(args)))
    PyInstaller.__main__.run(args)
