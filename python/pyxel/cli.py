import base64
import glob
import multiprocessing
import os
import pathlib
import re
import runpy
import shutil
import subprocess
import sys
import tempfile
import time
import uuid
import zipfile

import pyxel
import pyxel.utils

_METADATA_FIELDS = ("title", "author", "desc", "site", "license", "version")
_METADATA_PATTERN = re.compile(r"#\s*(.+?)\s*:\s*(.+)")


def cli():
    commands = [
        (["run", "PYTHON_SCRIPT_FILE(.py)"], run_python_script),
        (
            ["watch", "WATCH_DIR", "PYTHON_SCRIPT_FILE(.py)"],
            watch_and_run_python_script,
        ),
        (["play", f"PYXEL_APP_FILE({pyxel.APP_FILE_EXTENSION})"], play_pyxel_app),
        (
            ["edit", f"[PYXEL_RESOURCE_FILE({pyxel.RESOURCE_FILE_EXTENSION})]"],
            edit_pyxel_resource,
        ),
        (["package", "APP_DIR", "STARTUP_SCRIPT_FILE(.py)"], package_pyxel_app),
        (
            ["app2exe", f"PYXEL_APP_FILE({pyxel.APP_FILE_EXTENSION})"],
            create_executable_from_pyxel_app,
        ),
        (
            ["app2html", f"PYXEL_APP_FILE({pyxel.APP_FILE_EXTENSION})"],
            create_html_from_pyxel_app,
        ),
        (["copy_examples"], copy_pyxel_examples),
    ]

    def print_usage(command_name=None):
        print("usage:")
        for command in commands:
            if command_name is None or command[0] == command_name:
                print(f"    pyxel {' '.join(command[0])}")

    num_args = len(sys.argv)
    if num_args <= 1:
        print(f"Pyxel {pyxel.VERSION}, a retro game engine for Python")
        print_usage()
        return

    for command in commands:
        if sys.argv[1] != command[0][0]:
            continue
        max_args = len(command[0]) + 1
        min_args = max_args - sum(1 for s in command[0] if s.startswith("["))
        if not (min_args <= num_args <= max_args):
            print("invalid number of parameters")
            print_usage(command[0])
            sys.exit(1)
        command[1](*sys.argv[2:])
        return

    print(f"invalid command: '{sys.argv[1]}'")
    print_usage()
    sys.exit(1)


def _exit_with_error(message):
    print(message)
    sys.exit(1)


def _complete_extension(filename, command, valid_ext):
    file_ext = os.path.splitext(filename)[1].lower()
    if not file_ext:
        filename += valid_ext
    elif file_ext != valid_ext:
        _exit_with_error(f"'{command}' command only accepts {valid_ext} files")
    return filename


def _files_in_dir(dirname):
    return sorted(
        os.path.join(root, name)
        for root, _, files in os.walk(dirname)
        for name in files
    )


def _check_file_exists(filename):
    if not os.path.isfile(filename):
        _exit_with_error(f"no such file: '{filename}'")


def _check_dir_exists(dirname):
    if not os.path.isdir(dirname):
        _exit_with_error(f"no such directory: '{dirname}'")


def _check_file_under_dir(filename, dirname):
    if os.path.relpath(
        os.path.realpath(filename), os.path.realpath(dirname)
    ).startswith(".."):
        _exit_with_error("specified file is not under the directory")


def _create_app_dir():
    play_dir = os.path.join(tempfile.gettempdir(), pyxel.BASE_DIR, "play")
    pathlib.Path(play_dir).mkdir(parents=True, exist_ok=True)

    # Clean up stale app dirs from dead processes
    for path in glob.glob(os.path.join(play_dir, "*")):
        try:
            pid = int(os.path.basename(path).split("_")[0])
            if pyxel._pid_exists(pid):
                continue
            if time.time() - os.path.getmtime(path) > 300:
                shutil.rmtree(path)
        except ValueError:
            shutil.rmtree(path)

    app_dir = os.path.join(play_dir, f"{os.getpid()}_{uuid.uuid4()}")
    if os.path.exists(app_dir):
        shutil.rmtree(app_dir)
    os.mkdir(app_dir)
    return app_dir


def _create_watch_state_file():
    watch_dir = os.path.join(tempfile.gettempdir(), pyxel.BASE_DIR, "watch")
    pathlib.Path(watch_dir).mkdir(parents=True, exist_ok=True)

    # Clean up state files from dead watcher processes
    for path in glob.glob(os.path.join(watch_dir, "*")):
        try:
            pid = int(os.path.basename(path))
        except ValueError:
            continue
        if not pyxel._pid_exists(pid):
            os.remove(path)

    watch_state_file = os.path.join(watch_dir, str(os.getpid()))
    pathlib.Path(watch_state_file).touch()
    return watch_state_file


def _timestamps_in_dir(dirname):
    return {
        os.path.join(root, name): os.path.getmtime(os.path.join(root, name))
        for root, _, files in os.walk(dirname)
        for name in files
    }


def _run_python_script_in_separate_process(python_script_file):
    python_script_file = os.path.abspath(python_script_file)
    worker = multiprocessing.Process(
        target=run_python_script, args=(python_script_file,)
    )
    worker.daemon = True
    worker.start()
    return worker


def _extract_pyxel_app(pyxel_app_file):
    _check_file_exists(pyxel_app_file)
    app_dir = _create_app_dir()

    with zipfile.ZipFile(pyxel_app_file) as zf:
        zf.extractall(app_dir)

    pattern = os.path.join(app_dir, "*", pyxel.APP_STARTUP_SCRIPT_FILE)
    for setting_file in glob.glob(pattern):
        with open(setting_file, "r") as f:
            return os.path.join(os.path.dirname(setting_file), f.read())
    return None


def _make_metadata_comment(startup_script_file):
    metadata = {}

    with open(startup_script_file, "r", encoding="utf8") as f:
        for line in f:
            match = _METADATA_PATTERN.match(line)
            if match:
                key, value = match.groups()
                key = key.strip().lower()
                if key in _METADATA_FIELDS:
                    metadata[key] = value.strip()

    if not metadata:
        return ""

    max_key_len = max(len(key) for key in metadata)
    max_value_len = max(len(value) for value in metadata.values())
    border = "-" * min((max_key_len + max_value_len + 3), 80)

    metadata_comment = border + "\n"
    for key in _METADATA_FIELDS:
        if key in metadata:
            value = metadata[key]
            metadata_comment += f"{key.ljust(max_key_len)} : {value}\n"
    metadata_comment += border

    return metadata_comment


def run_python_script(python_script_file):
    python_script_file = _complete_extension(python_script_file, "run", ".py")
    _check_file_exists(python_script_file)

    sys.path.insert(0, os.path.abspath(os.path.dirname(python_script_file)))
    runpy.run_path(python_script_file, run_name="__main__")


def watch_and_run_python_script(watch_dir, python_script_file):
    python_script_file = _complete_extension(python_script_file, "watch", ".py")
    _check_dir_exists(watch_dir)
    _check_file_exists(python_script_file)
    _check_file_under_dir(python_script_file, watch_dir)

    os.environ[pyxel.WATCH_STATE_FILE_ENV] = _create_watch_state_file()

    try:
        print(f"start watching '{watch_dir}' (Ctrl+C to stop)")
        cur_time = last_time = time.time()
        timestamps = _timestamps_in_dir(watch_dir)
        worker = _run_python_script_in_separate_process(python_script_file)

        while True:
            time.sleep(0.5)

            cur_time = time.time()
            if cur_time - last_time >= 10:
                last_time = cur_time
                print(f"watching '{watch_dir}' (Ctrl+C to stop)")

            last_timestamps = timestamps
            timestamps = _timestamps_in_dir(watch_dir)
            if (
                timestamps != last_timestamps
                or worker.exitcode == pyxel.WATCH_RESET_EXIT_CODE
            ):
                print(f"rerun {python_script_file}")
                if worker.is_alive():
                    worker.terminate()
                worker = _run_python_script_in_separate_process(python_script_file)

    except KeyboardInterrupt:
        print("\r", end="")
        print("stopped watching")


def get_pyxel_app_metadata(pyxel_app_file):
    _check_file_exists(pyxel_app_file)
    metadata = {}

    with zipfile.ZipFile(pyxel_app_file) as zf:
        if not zf.comment:
            return metadata
        comment = zf.comment.decode(encoding="utf-8")

    for line in comment.splitlines():
        if line.startswith("-"):
            continue
        if ":" in line:
            key, value = line.split(":", 1)
            metadata[key.strip()] = value.strip()

    return metadata


def print_pyxel_app_metadata(pyxel_app_file):
    _check_file_exists(pyxel_app_file)
    with zipfile.ZipFile(pyxel_app_file) as zf:
        if zf.comment:
            print(zf.comment.decode(encoding="utf-8"))


def play_pyxel_app(pyxel_app_file):
    file_ext = os.path.splitext(pyxel_app_file)[1].lower()
    if file_ext != ".zip":
        pyxel_app_file = _complete_extension(
            pyxel_app_file, "play", pyxel.APP_FILE_EXTENSION
        )
    _check_file_exists(pyxel_app_file)

    print_pyxel_app_metadata(pyxel_app_file)
    startup_script_file = _extract_pyxel_app(pyxel_app_file)

    if not startup_script_file:
        _exit_with_error(f"file not found: '{pyxel.APP_STARTUP_SCRIPT_FILE}'")

    sys.path.insert(0, os.path.abspath(os.path.dirname(startup_script_file)))
    runpy.run_path(startup_script_file, run_name="__main__")


def edit_pyxel_resource(pyxel_resource_file=None, starting_editor="image"):
    import pyxel.editor

    if not pyxel_resource_file:
        pyxel_resource_file = "my_resource"

    pyxel_resource_file = _complete_extension(
        pyxel_resource_file, "edit", pyxel.RESOURCE_FILE_EXTENSION
    )
    pyxel.editor.App(pyxel_resource_file, starting_editor)


def package_pyxel_app(app_dir, startup_script_file):
    startup_script_file = _complete_extension(startup_script_file, "package", ".py")
    _check_dir_exists(app_dir)
    _check_file_exists(startup_script_file)
    _check_file_under_dir(startup_script_file, app_dir)

    metadata_comment = _make_metadata_comment(startup_script_file)
    if metadata_comment:
        print(metadata_comment)

    app_dir = os.path.abspath(app_dir)
    setting_file = os.path.join(app_dir, pyxel.APP_STARTUP_SCRIPT_FILE)
    with open(setting_file, "w") as f:
        f.write(os.path.relpath(startup_script_file, app_dir))

    pyxel_app_file = os.path.basename(app_dir) + pyxel.APP_FILE_EXTENSION
    app_parent_dir = os.path.dirname(app_dir)

    with zipfile.ZipFile(
        pyxel_app_file,
        "w",
        compression=zipfile.ZIP_DEFLATED,
    ) as zf:
        zf.comment = metadata_comment.encode(encoding="utf-8")
        _SKIP_EXTENSIONS = (".gif", ".zip")
        files = [setting_file] + _files_in_dir(app_dir)
        for file in files:
            if (
                os.path.basename(file) == pyxel_app_file
                or "__pycache__" in file
                or file.lower().endswith(_SKIP_EXTENSIONS)
            ):
                continue
            arcname = os.path.relpath(file, app_parent_dir)
            zf.write(file, arcname)
            print(f"added '{arcname}'")

    os.remove(setting_file)


def create_executable_from_pyxel_app(pyxel_app_file):
    pyxel_app_file = _complete_extension(
        pyxel_app_file, "app2exe", pyxel.APP_FILE_EXTENSION
    )
    _check_file_exists(pyxel_app_file)

    app2exe_dir = os.path.join(tempfile.gettempdir(), pyxel.BASE_DIR, "app2exe")
    if os.path.isdir(app2exe_dir):
        shutil.rmtree(app2exe_dir)
    pathlib.Path(app2exe_dir).mkdir(parents=True, exist_ok=True)

    pyxel_app_name = os.path.splitext(os.path.basename(pyxel_app_file))[0]
    startup_script_file = os.path.join(app2exe_dir, pyxel_app_name + ".py")
    with open(startup_script_file, "w") as f:
        app_filename = f"{pyxel_app_name}{pyxel.APP_FILE_EXTENSION}"
        f.write(
            "import os, pyxel.cli; pyxel.cli.play_pyxel_app("
            f"os.path.join(os.path.dirname(__file__), {repr(app_filename)}))"
        )

    cp = subprocess.run(
        [sys.executable, "-m", "PyInstaller", "-h"], capture_output=True
    )
    if cp.returncode != 0:
        _exit_with_error("Pyinstaller is not found. Please install it.")

    startup_script = _extract_pyxel_app(pyxel_app_file)
    if startup_script is None:
        _exit_with_error("Failed to extract startup script from pyxel app.")

    modules = pyxel.utils.list_imported_modules(startup_script)["system"]
    hidden_imports = [arg for m in modules for arg in ("--hidden-import", m)]
    command = [
        sys.executable,
        "-m",
        "PyInstaller",
        "--windowed",
        "--onedir",
        "--distpath",
        ".",
        "--add-data",
        f"{pyxel_app_file}{os.pathsep}.",
        *hidden_imports,
        startup_script_file,
    ]
    print(" ".join(command))
    result = subprocess.run(command)
    if result.returncode != 0:
        _exit_with_error(f"PyInstaller build failed with exit code {result.returncode}")

    # Clean up temporary build artifacts
    shutil.rmtree(app2exe_dir, ignore_errors=True)
    spec_file = os.path.splitext(pyxel_app_file)[0] + ".spec"
    if os.path.isfile(spec_file):
        os.remove(spec_file)
    shutil.rmtree(os.path.join(os.getcwd(), "build"), ignore_errors=True)


def create_html_from_pyxel_app(pyxel_app_file):
    pyxel_app_file = _complete_extension(
        pyxel_app_file, "app2html", pyxel.APP_FILE_EXTENSION
    )
    _check_file_exists(pyxel_app_file)

    with open(pyxel_app_file, "rb") as f:
        base64_string = base64.b64encode(f.read()).decode()

    pyxel_app_name = os.path.splitext(os.path.basename(pyxel_app_file))[0]
    with open(pyxel_app_name + ".html", "w") as f:
        f.write(
            "<!doctype html>\n"
            f'<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel@{pyxel.VERSION}/wasm/pyxel.js">'
            "</script>\n"
            "<script>\n"
            f'launchPyxel({{ command: "play", name: "{pyxel_app_name}{pyxel.APP_FILE_EXTENSION}", '
            f'gamepad: "enabled", base64: "{base64_string}" }});\n'
            "</script>\n"
        )


def copy_pyxel_examples():
    src_dir = os.path.join(os.path.dirname(__file__), "examples")
    dst_dir = "pyxel_examples"
    shutil.rmtree(dst_dir, ignore_errors=True)

    for src_file in _files_in_dir(src_dir):
        if "__pycache__" in src_file:
            continue
        dst_file = os.path.join(dst_dir, os.path.relpath(src_file, src_dir))
        os.makedirs(os.path.dirname(dst_file), exist_ok=True)
        shutil.copyfile(src_file, dst_file)
        print(f"copied '{dst_file}'")
