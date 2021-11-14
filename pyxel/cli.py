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
    print(f"Pyxel {pyxel.PYXEL_VERSION}, a retro game engine for Python")
    print("usage:")
    print("    pyxel PYXEL_APP_FILE(.pyxapp)")
    print("    pyxel -run PYTHON_SCRIPT(.py)")
    print("    pyxel -edit PYXEL_RESOURCE_FILE(.pyxres)")
    print("    pyxel -package PYXEL_APP_DIR STARTUP_SCRIPT(.py)")
    print("    pyxel -copy-examples")


def _complete_extension(filename, ext_with_dot):
    file_ext = os.path.splitext(filename)[1]
    if file_ext.lower() == ext_with_dot.lower():
        return filename
    else:
        return filename + ext_with_dot


def _files_in_dir(dirname):
    paths = glob.glob(os.path.join(dirname, "**/*"), recursive=True)
    return sorted(list(filter(os.path.isfile, paths)))


def _check_file_exists(filename):
    if not os.path.isfile(filename):
        print(f"no such file: '{filename}'")
        exit(1)


def _check_dir_exists(dirname):
    if not os.path.isdir(dirname):
        print(f"no such directory: '{dirname}'")
        exit(1)


def _launch_pyxel_app(filename):
    filename = _complete_extension(filename, pyxel.APP_FILE_EXTENSION)
    _check_file_exists(filename)

    with tempfile.TemporaryDirectory() as dirname:
        zf = zipfile.ZipFile(filename)
        zf.extractall(dirname)

        pattern = os.path.join(dirname, "*/" + pyxel.APP_STARTUP_SCRIPT_FILE)
        for startup_file in glob.glob(pattern):
            with open(startup_file, "r") as f:
                startup_script = os.path.join(os.path.dirname(startup_file), f.read())
                runpy.run_path(startup_script)
                return

        print(f"file not found: '{pyxel.APP_STARTUP_SCRIPT_FILE}'")
        exit(1)


def _run_python_script(filename):
    filename = _complete_extension(filename, ".py")
    _check_file_exists(filename)
    runpy.run_path(filename)


def _edit_pyxel_resource_file(filename):
    filename = filename or "my_resource"
    filename = _complete_extension(filename, pyxel.RESOURCE_FILE_EXTENSION)
    pyxel.editor.App(filename)


def _package_pyxel_app_file(dirname, filename):
    _check_dir_exists(dirname)
    startup_script = os.path.basename(_complete_extension(filename, ".py"))
    _check_file_exists(os.path.join(dirname, startup_script))

    startup_file = os.path.join(dirname, pyxel.APP_STARTUP_SCRIPT_FILE)
    with open(startup_file, "w") as f:
        f.write(startup_script)

    package_file = os.path.basename(dirname) + pyxel.APP_FILE_EXTENSION
    package_dir = os.path.dirname(dirname)

    with zipfile.ZipFile(
        package_file,
        "w",
        compression=zipfile.ZIP_DEFLATED,
    ) as zf:
        files = [startup_file] + _files_in_dir(dirname)
        for file in files:
            arcname = os.path.relpath(file, package_dir)
            zf.write(file, arcname)
            print(f"added '{arcname}'")

    os.remove(startup_file)


def _copy_pyxel_examples():
    src_dir = os.path.join(os.path.dirname(__file__), "examples")
    dst_dir = "pyxel_examples"

    shutil.rmtree(dst_dir, ignore_errors=True)

    for src_file in _files_in_dir(src_dir):
        dst_file = os.path.join(dst_dir, os.path.relpath(src_file, src_dir))
        os.makedirs(os.path.dirname(dst_file), exist_ok=True)
        shutil.copyfile(src_file, dst_file)
        print(f"copied '{dst_file}'")


def cli():
    num_args = len(sys.argv)
    command = sys.argv[1].lower() if num_args > 1 else ""

    if not command.startswith("-"):
        if num_args == 2:
            _launch_pyxel_app(sys.argv[1])

    elif command == "-run":
        if num_args == 3:
            _run_python_script(sys.argv[2])

    elif command == "-edit":
        if num_args == 2:
            _edit_pyxel_resource_file(None)
        elif num_args == 3:
            _edit_pyxel_resource_file(sys.argv[2])

    elif command == "-package":
        if num_args == 4:
            _package_pyxel_app_file(sys.argv[2], sys.argv[3])

    elif command == "-copy-examples":
        if num_args == 2:
            _copy_pyxel_examples()

    else:
        _print_usage()
