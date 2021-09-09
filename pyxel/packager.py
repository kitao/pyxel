import argparse
import glob
import os
import platform
import shutil

import pyxel


def run():
    parser = argparse.ArgumentParser(description="Pyxel Packager")
    parser.add_argument(
        "python_file", type=argparse.FileType(), help="Pyxel program file entry point"
    )
    parser.add_argument(
        "-v",
        "--version",
        action="version",
        version="Pyxel Packager {}".format(pyxel.VERSION),
        help="Show version number and quit",
    )
    parser.add_argument(
        "-i", "--icon", type=argparse.FileType(), help="Pyxel program icon file"
    )

    args = parser.parse_args()

    dirname = os.path.dirname(args.python_file.name) or "."
    filename = os.path.basename(args.python_file.name)
    name = os.path.splitext(filename)[0]
    separator = ";" if platform.system() == "Windows" else ":"

    os.chdir(dirname)

    options = [
        "--clean",
        "--noconfirm",
        "--log-level=WARN",
        "--onefile",
        "--noconsole",
        "--name={}".format(name),
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

    assets = filter(os.path.isfile, glob.glob("**/assets/**", recursive=True))

    for asset in assets:
        options.append(
            "--add-data={}{}{}".format(
                os.path.abspath(asset), separator, os.path.dirname(asset)
            )
        )

    if args.icon:
        options.append("--icon={}".format(os.path.abspath(args.icon.name)))

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
    try:
        import PyInstaller.__main__

        print("pyinstaller {}".format(" ".join(args)))
        PyInstaller.__main__.run(args)
    except ModuleNotFoundError:
        print("pyxel error: PyInstaller is not installed")


if __name__ == "__main__":
    run()
