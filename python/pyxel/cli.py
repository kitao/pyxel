import glob
import os
import pathlib
import re
import runpy
import shutil
import sys
import tempfile
import urllib.request
import zipfile

import pyxel


def cli():
    num_args = len(sys.argv)
    command = sys.argv[1] if num_args > 1 else ""
    if command == "run" and num_args == 3:
        run_python_script(sys.argv[2])
    elif command == "play" and num_args == 3:
        play_pyxel_app(sys.argv[2])
    elif command == "edit" and (num_args == 2 or num_args == 3):
        edit_pyxel_resource(sys.argv[2] if num_args == 3 else None)
    elif command == "package" and num_args == 4:
        package_pyxel_app(sys.argv[2], sys.argv[3])
    elif command == "copy_examples" and num_args == 2:
        copy_pyxel_examples()
    else:
        _print_usage()


def _print_usage():
    print(f"Pyxel {pyxel.PYXEL_VERSION}, a retro game engine for Python")
    print("usage:")
    print("    pyxel run PYTHON_SCRIPT_FILE(.py)")
    print("    pyxel play PYXEL_APP_FILE(.pyxapp)")
    print("    pyxel edit [PYXEL_RESOURCE_FILE(.pyxres)]")
    print("    pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE(.py)")
    print("    pyxel copy_examples")
    _check_newer_version()


def _check_newer_version():
    url = "https://www.github.com/kitao/pyxel"
    req = urllib.request.Request(url)
    latest_version = None
    try:
        with urllib.request.urlopen(req, timeout=3) as res:
            pattern = r"/kitao/pyxel/releases/tag/v(\d+\.\d+\.\d+)"
            text = res.read().decode("utf-8")
            result = re.search(pattern, text)
            if result:
                latest_version = result.group(1)
    except urllib.error.URLError:
        return
    if not latest_version:
        return

    def parse_version(version):
        return list(map(int, version.split(".")))

    if parse_version(latest_version) > parse_version(pyxel.PYXEL_VERSION):
        print(f"A new version, Pyxel {latest_version}, is available.")


def _complete_extension(filename, ext_with_dot):
    file_ext = os.path.splitext(filename)[1]
    if file_ext == ext_with_dot:
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


def _make_app_dir():
    play_dir = os.path.join(tempfile.gettempdir(), pyxel.PYXEL_WORKING_DIR, "play")
    pathlib.Path(play_dir).mkdir(parents=True, exist_ok=True)
    for path in glob.glob(os.path.join(play_dir, "*")):
        pid = int(os.path.basename(path))
        if not pyxel.process_exists(pid):
            shutil.rmtree(path)
    app_dir = os.path.join(play_dir, str(os.getpid()))
    os.mkdir(app_dir)
    return app_dir


def run_python_script(python_script_file):
    python_script_file = _complete_extension(python_script_file, ".py")
    _check_file_exists(python_script_file)
    sys.path.append(os.path.dirname(python_script_file))
    runpy.run_path(python_script_file)


def play_pyxel_app(pyxel_app_file):
    pyxel_app_file = _complete_extension(pyxel_app_file, pyxel.APP_FILE_EXTENSION)
    _check_file_exists(pyxel_app_file)
    app_dir = _make_app_dir()
    zf = zipfile.ZipFile(pyxel_app_file)
    zf.extractall(app_dir)
    pattern = os.path.join(app_dir, "*", pyxel.APP_STARTUP_SCRIPT_FILE)
    for setting_file in glob.glob(pattern):
        with open(setting_file, "r") as f:
            startup_script_file = os.path.join(os.path.dirname(setting_file), f.read())
            sys.path.append(os.path.dirname(startup_script_file))
            runpy.run_path(startup_script_file)
            return
    print(f"file not found: '{pyxel.APP_STARTUP_SCRIPT_FILE}'")
    exit(1)


def edit_pyxel_resource(pyxel_resource_file):
    import pyxel.editor

    pyxel_resource_file = pyxel_resource_file or "my_resource"
    pyxel_resource_file = _complete_extension(
        pyxel_resource_file, pyxel.RESOURCE_FILE_EXTENSION
    )
    pyxel.editor.App(pyxel_resource_file)


def package_pyxel_app(app_root_dir, startup_script_name):
    _check_dir_exists(app_root_dir)
    startup_script_name = _complete_extension(startup_script_name, ".py")
    _check_file_exists(os.path.join(app_root_dir, startup_script_name))
    setting_file = os.path.join(app_root_dir, pyxel.APP_STARTUP_SCRIPT_FILE)
    with open(setting_file, "w") as f:
        f.write(startup_script_name)
    pyxel_app_file = os.path.basename(app_root_dir) + pyxel.APP_FILE_EXTENSION
    app_parent_dir = os.path.dirname(os.path.abspath(app_root_dir))
    with zipfile.ZipFile(
        pyxel_app_file,
        "w",
        compression=zipfile.ZIP_DEFLATED,
    ) as zf:
        files = [setting_file] + _files_in_dir(app_root_dir)
        for file in files:
            arcname = os.path.relpath(file, app_parent_dir)
            zf.write(file, arcname)
            print(f"added '{arcname}'")
    os.remove(setting_file)


def copy_pyxel_examples():
    src_dir = os.path.join(os.path.dirname(__file__), "examples")
    dst_dir = "pyxel_examples"
    shutil.rmtree(dst_dir, ignore_errors=True)
    for src_file in _files_in_dir(src_dir):
        dst_file = os.path.join(dst_dir, os.path.relpath(src_file, src_dir))
        os.makedirs(os.path.dirname(dst_file), exist_ok=True)
        shutil.copyfile(src_file, dst_file)
        print(f"copied '{dst_file}'")
