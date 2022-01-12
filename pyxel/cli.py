import glob
import os
import runpy
import shutil
import sys
import tempfile
import zipfile

import pyxel


def _print_usage():
    print(f"Pyxel {pyxel.PYXEL_VERSION}, a retro game engine for Python")
    print("usage:")
    print("    pyxel run PYTHON_SCRIPT_FILE(.py)")
    print("    pyxel play PYXEL_APP_FILE(.pyxapp)")
    print("    pyxel edit [PYXEL_RESOURCE_FILE(.pyxres)]")
    print("    pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE(.py)")
    print("    pyxel copy_examples")
    print("    pyxel module_search_path")


def _complete_extension(filename, ext_with_dot):
    base, file_ext = os.path.splitext(filename)
    if file_ext == ext_with_dot:
        return filename
    else:
        return base + ext_with_dot


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


def _run_python_script(python_script_file):
    python_script_file = _complete_extension(python_script_file, ".py")
    _check_file_exists(python_script_file)
    sys.path.append(os.path.dirname(python_script_file))
    runpy.run_path(python_script_file)


def _play_pyxel_app(pyxel_app_file):
    pyxel_app_file = _complete_extension(pyxel_app_file, pyxel.APP_FILE_EXTENSION)
    _check_file_exists(pyxel_app_file)
    with tempfile.TemporaryDirectory() as temp_dir:
        zf = zipfile.ZipFile(pyxel_app_file)
        zf.extractall(temp_dir)
        pattern = os.path.join(temp_dir, "*", pyxel.APP_STARTUP_SCRIPT_FILE)
        for setting_file in glob.glob(pattern):
            with open(setting_file, "r") as f:
                startup_script_file = os.path.join(
                    os.path.dirname(setting_file), f.read()
                )
                sys.path.append(os.path.dirname(startup_script_file))
                runpy.run_path(startup_script_file)
                return
        print(f"file not found: '{pyxel.APP_STARTUP_SCRIPT_FILE}'")
        exit(1)


def _edit_pyxel_resource(pyxel_resource_file):
    import pyxel.editor

    pyxel_resource_file = pyxel_resource_file or "my_resource"
    pyxel_resource_file = _complete_extension(
        pyxel_resource_file, pyxel.RESOURCE_FILE_EXTENSION
    )
    pyxel_palette_file = _complete_extension(pyxel_resource_file, ".pyxpal")
    pyxel.editor.App(pyxel_resource_file, pyxel_palette_file)


def _package_pyxel_app(app_root_dir, startup_script_name):
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


def _copy_pyxel_examples():
    src_dir = os.path.join(os.path.dirname(__file__), "examples")
    dst_dir = "pyxel_examples"
    shutil.rmtree(dst_dir, ignore_errors=True)
    for src_file in _files_in_dir(src_dir):
        dst_file = os.path.join(dst_dir, os.path.relpath(src_file, src_dir))
        os.makedirs(os.path.dirname(dst_file), exist_ok=True)
        shutil.copyfile(src_file, dst_file)
        print(f"copied '{dst_file}'")


def _print_module_search_path():
    module_search_path = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    print(module_search_path)


def cli():
    num_args = len(sys.argv)
    command = sys.argv[1] if num_args > 1 else ""
    if command == "run" and num_args == 3:
        _run_python_script(sys.argv[2])
    elif command == "play" and num_args == 3:
        _play_pyxel_app(sys.argv[2])
    elif command == "edit" and (num_args == 2 or num_args == 3):
        _edit_pyxel_resource(sys.argv[2] if num_args == 3 else None)
    elif command == "package" and num_args == 4:
        _package_pyxel_app(sys.argv[2], sys.argv[3])
    elif command == "copy_examples" and num_args == 2:
        _copy_pyxel_examples()
    elif command == "module_search_path" and num_args == 2:
        _print_module_search_path()
    else:
        _print_usage()
