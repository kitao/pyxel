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
        "--log-level=WARN",
        "--onefile",
        "--noconsole",
        "--name={}".format(name),
        "--hidden-import=numpy.random.bounded_integers",
        "--hidden-import=numpy.random.common",
        "--hidden-import=numpy.random.entropy",
    ]

    src_lib_dir = os.path.dirname(pyxel.core._get_absolute_libpath())
    dst_lib_dir = os.path.dirname(pyxel.core._get_relative_libpath())
    libs = filter(os.path.isfile, glob.glob(os.path.join(src_lib_dir, "*")))

    for lib in libs:
        libname = os.path.basename(lib)

        options.append(
            "--add-data={}{}{}".format(
                os.path.join(src_lib_dir, libname), separator, dst_lib_dir
            )
        )

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


if __name__ == "__main__":
    run()
