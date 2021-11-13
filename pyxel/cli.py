import glob
import os
import runpy
import shutil
import sys
import tempfile
import zipfile

import pyxel
import pyxel.editor


def _print_usage():
    print("Pyxel {}, a retro game engine for Python".format(pyxel.PYXEL_VERSION))
    print("usage:")
    print("    pyxel PYXEL_APP_FILE(.pyxapp)")
    print("    pyxel -run PYTHON_SCRIPT(.py)")
    print("    pyxel -edit PYXEL_RESOURCE_FILE(.pyxres)")
    print("    pyxel -package PYXEL_APP_DIR STARTUP_SCRIPT(.py)")
    print("    pyxel -examples COPY_DEST_DIR")


def _check_file_exists(filename):
    if not os.path.isfile(filename):
        print("no such file: '{}'".format(filename))
        exit(1)


def _check_dir_exists(dirname):
    if not os.path.isdir(dirname):
        print("no such directory: '{}'".format(dirname))
        exit(1)


def _complete_extension(filename, ext):
    file_ext = os.path.splitext(filename)[1]

    if file_ext.lower() == ext.lower():
        return filename
    else:
        return filename + "." + ext


def _launch_pyxel_app_file(filename):
    filename = _complete_extension(filename, pyxel.APPLICATION_FILE_EXTENSION)
    _check_file_exists(filename)

    with tempfile.TemporaryDirectory() as dirname:
        zf = zipfile.ZipFile(filename)
        zf.extractall(dirname)

        pattern = os.path.join(dirname, "*/.pyxapp-startup-script")
        for name in glob.glob(pattern):
            with open(name, "r") as f:
                filename = os.path.join(os.path.dirname(name), f.read())
                _run_python_script(filename)
                exit(0)


def _run_python_script(filename):
    filename = _complete_extension(filename, ".py")
    _check_file_exists(filename)
    runpy.run_path(filename)


def _edit_pyxel_resource_file(filename):
    filename = _complete_extension(filename, pyxel.RESOURCE_FILE_EXTENSION)
    pyxel.editor.App(filename)


def _package_pyxel_app_file(dirname, filename):
    _check_dir_exists(dirname)

    script_name = os.path.basename(_complete_extension(filename, ".py"))
    filename = os.path.join(dirname, script_name)

    _check_file_exists(filename)

    with open(os.path.join(dirname, ".pyxapp-startup-script"), "w") as f:
        f.write(script_name)

    pyxapp_name = os.path.basename(dirname)
    shutil.make_archive(pyxapp_name, "zip", os.path.join(dirname, ".."), pyxapp_name)
    os.rename(pyxapp_name + ".zip", pyxapp_name + pyxel.APPLICATION_FILE_EXTENSION)


def _copy_pyxel_examples(dirname):
    _check_dir_exists(dirname)

    src_dir = os.path.join(os.path.dirname(__file__), "examples")
    dst_dir = os.path.join(dirname, "pyxel_examples")

    shutil.rmtree(dst_dir, ignore_errors=True)
    os.makedirs(os.path.join(dst_dir, "assets"))

    patterns = ["[0-9]*.py", "assets/*.pyxres", "assets/*.png", "assets/*.gif"]

    for pattern in patterns:
        srcs = glob.glob(os.path.join(src_dir, pattern))

        for src in srcs:
            relpath = os.path.relpath(src, src_dir)
            dst = os.path.join(dst_dir, relpath)
            shutil.copyfile(src, dst)


def cli():
    num_args = len(sys.argv)
    command = sys.argv[1].lower() if num_args > 1 else ""

    if num_args == 2:
        _launch_pyxel_app_file(sys.argv[1])

    elif num_args == 3 and command == "-run":
        _run_python_script(sys.argv[2])

    elif num_args == 3 and command == "-edit":
        _edit_pyxel_resource_file(sys.argv[2])

    elif num_args == 4 and command == "-package":
        _package_pyxel_app_file(sys.argv[2], sys.argv[3])

    elif num_args == 3 and command == "-examples":
        _copy_pyxel_examples(sys.argv[2])

    else:
        _print_usage()
