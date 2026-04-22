import base64
import importlib.util
import multiprocessing
import os
import re
import runpy
import shutil
import subprocess
import sys
import tempfile
import time
import uuid
import zipfile
from pathlib import Path

import pyxel
import pyxel.utils

_METADATA_FIELDS = ("title", "author", "desc", "site", "license", "version")
_METADATA_PATTERN = re.compile(r"#\s*(.+?)\s*:\s*(.+)")
_PACKAGE_SKIP_EXTENSIONS = (".gif", ".zip")


def cli() -> None:
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
    file_ext = Path(filename).suffix.lower()
    if not file_ext:
        filename += valid_ext
    elif file_ext != valid_ext:
        _exit_with_error(f"'{command}' command only accepts {valid_ext} files")
    return filename


def _files_in_dir(dirname):
    # Exclude dotfiles and dot-directories under dirname to avoid picking up
    # OS/tool artifacts (e.g. .DS_Store, .git, .vscode), but keep the pyxapp
    # startup-script file which intentionally starts with '.'
    base = Path(dirname)
    return sorted(
        str(p)
        for p in base.rglob("*")
        if p.is_file()
        and not any(
            part.startswith(".") and part != pyxel.APP_STARTUP_SCRIPT_FILE
            for part in p.relative_to(base).parts
        )
    )


def _check_file_exists(filename):
    if not Path(filename).is_file():
        _exit_with_error(f"no such file: '{filename}'")


def _check_dir_exists(dirname):
    if not Path(dirname).is_dir():
        _exit_with_error(f"no such directory: '{dirname}'")


def _check_file_under_dir(filename, dirname):
    if not Path(filename).resolve().is_relative_to(Path(dirname).resolve()):
        _exit_with_error("specified file is not under the directory")


def _create_app_dir():
    play_dir = Path(tempfile.gettempdir()) / pyxel.BASE_DIR / "play"
    play_dir.mkdir(parents=True, exist_ok=True)

    # Clean up stale app dirs from dead processes
    for path in play_dir.glob("*"):
        try:
            pid = int(path.name.split("_")[0])
        except ValueError:
            shutil.rmtree(path)
            continue
        if pyxel._pid_exists(pid):
            continue
        if time.time() - path.stat().st_mtime > 300:
            shutil.rmtree(path)

    app_dir = play_dir / f"{os.getpid()}_{uuid.uuid4()}"
    if app_dir.exists():
        shutil.rmtree(app_dir)
    app_dir.mkdir()
    return str(app_dir)


def _create_watch_state_file():
    watch_dir = Path(tempfile.gettempdir()) / pyxel.BASE_DIR / "watch"
    watch_dir.mkdir(parents=True, exist_ok=True)

    # Clean up state files from dead watcher processes
    for path in watch_dir.glob("*"):
        try:
            pid = int(path.name)
        except ValueError:
            continue
        if not pyxel._pid_exists(pid):
            path.unlink()

    watch_state_file = watch_dir / str(os.getpid())
    watch_state_file.touch()
    return str(watch_state_file)


def _timestamps_in_dir(dirname):
    return {str(p): p.stat().st_mtime for p in Path(dirname).rglob("*") if p.is_file()}


def _run_python_script_in_separate_process(python_script_file):
    python_script_file = str(Path(python_script_file).absolute())
    worker = multiprocessing.Process(
        target=run_python_script, args=(python_script_file,), daemon=True
    )
    worker.start()
    return worker


def _extract_pyxel_app(pyxel_app_file):
    _check_file_exists(pyxel_app_file)
    app_dir = Path(_create_app_dir())

    with zipfile.ZipFile(pyxel_app_file) as zf:
        app_dir_abs = app_dir.resolve()
        for name in zf.namelist():
            target = (app_dir / name).resolve()
            if target != app_dir_abs and not target.is_relative_to(app_dir_abs):
                _exit_with_error(f"unsafe path in Pyxel app: '{name}'")
        zf.extractall(app_dir)

    for setting_file in app_dir.glob(f"*/{pyxel.APP_STARTUP_SCRIPT_FILE}"):
        return str(
            setting_file.parent / setting_file.read_text(encoding="utf-8").strip()
        )
    return None


def _make_metadata_comment(startup_script_file):
    metadata = {}

    with Path(startup_script_file).open(encoding="utf-8") as f:
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


def run_python_script(python_script_file: str) -> None:
    python_script_file = _complete_extension(python_script_file, "run", ".py")
    _check_file_exists(python_script_file)

    sys.path.insert(0, str(Path(python_script_file).absolute().parent))
    runpy.run_path(python_script_file, run_name="__main__")


def watch_and_run_python_script(watch_dir: str, python_script_file: str) -> None:
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


def get_pyxel_app_metadata(pyxel_app_file: str) -> dict[str, str]:
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


def print_pyxel_app_metadata(pyxel_app_file: str) -> None:
    _check_file_exists(pyxel_app_file)
    with zipfile.ZipFile(pyxel_app_file) as zf:
        if zf.comment:
            print(zf.comment.decode(encoding="utf-8"))


def play_pyxel_app(pyxel_app_file: str) -> None:
    file_ext = Path(pyxel_app_file).suffix.lower()
    if file_ext != ".zip":
        pyxel_app_file = _complete_extension(
            pyxel_app_file, "play", pyxel.APP_FILE_EXTENSION
        )
    _check_file_exists(pyxel_app_file)

    print_pyxel_app_metadata(pyxel_app_file)
    startup_script_file = _extract_pyxel_app(pyxel_app_file)

    if not startup_script_file:
        _exit_with_error(f"file not found: '{pyxel.APP_STARTUP_SCRIPT_FILE}'")

    sys.path.insert(0, str(Path(startup_script_file).absolute().parent))
    runpy.run_path(startup_script_file, run_name="__main__")


def edit_pyxel_resource(
    pyxel_resource_file: str | None = None, starting_editor: str = "image"
) -> None:
    import pyxel.editor

    if not pyxel_resource_file:
        pyxel_resource_file = "my_resource"

    pyxel_resource_file = _complete_extension(
        pyxel_resource_file, "edit", pyxel.RESOURCE_FILE_EXTENSION
    )
    pyxel.editor.App(pyxel_resource_file, starting_editor)


def package_pyxel_app(app_dir: str, startup_script_file: str) -> None:
    startup_script_file = _complete_extension(startup_script_file, "package", ".py")
    _check_dir_exists(app_dir)
    _check_file_exists(startup_script_file)
    _check_file_under_dir(startup_script_file, app_dir)

    metadata_comment = _make_metadata_comment(startup_script_file)
    if metadata_comment:
        print(metadata_comment)

    app_dir = Path(app_dir).resolve()
    setting_file = app_dir / pyxel.APP_STARTUP_SCRIPT_FILE
    setting_file.write_text(
        str(Path(startup_script_file).resolve().relative_to(app_dir)),
        encoding="utf-8",
    )

    pyxel_app_file = app_dir.name + pyxel.APP_FILE_EXTENSION
    app_parent_dir = app_dir.parent

    with zipfile.ZipFile(
        pyxel_app_file,
        "w",
        compression=zipfile.ZIP_DEFLATED,
    ) as zf:
        zf.comment = metadata_comment.encode(encoding="utf-8")
        for file in _files_in_dir(app_dir):
            if (
                Path(file).name == pyxel_app_file
                or "__pycache__" in file
                or file.lower().endswith(_PACKAGE_SKIP_EXTENSIONS)
            ):
                continue
            arcname = str(Path(file).relative_to(app_parent_dir))
            zf.write(file, arcname)
            print(f"added '{arcname}'")

    setting_file.unlink()


def create_executable_from_pyxel_app(pyxel_app_file: str) -> None:
    pyxel_app_file = _complete_extension(
        pyxel_app_file, "app2exe", pyxel.APP_FILE_EXTENSION
    )
    _check_file_exists(pyxel_app_file)

    app2exe_dir = Path(tempfile.gettempdir()) / pyxel.BASE_DIR / "app2exe"
    if app2exe_dir.is_dir():
        shutil.rmtree(app2exe_dir)
    app2exe_dir.mkdir(parents=True, exist_ok=True)

    pyxel_app_name = Path(pyxel_app_file).stem
    bootstrap_script_file = str(app2exe_dir / f"{pyxel_app_name}.py")
    app_filename = f"{pyxel_app_name}{pyxel.APP_FILE_EXTENSION}"
    Path(bootstrap_script_file).write_text(
        "import pyxel.cli; from pathlib import Path; pyxel.cli.play_pyxel_app("
        f"str(Path(__file__).parent / {repr(app_filename)}))",
        encoding="utf-8",
    )

    if importlib.util.find_spec("PyInstaller") is None:
        _exit_with_error("PyInstaller is not found. Please install it.")

    startup_script_file = _extract_pyxel_app(pyxel_app_file)
    if startup_script_file is None:
        _exit_with_error("Failed to extract startup script from Pyxel app.")

    modules = pyxel.utils.list_imported_modules(startup_script_file)["system"]
    hidden_imports = [arg for m in modules for arg in ("--hidden-import", m)]
    command = [
        sys.executable,
        "-m",
        "PyInstaller",
        "--windowed",
        "--onedir",
        "--add-data",
        f"{pyxel_app_file}{os.pathsep}.",
        *hidden_imports,
        bootstrap_script_file,
    ]
    print(" ".join(command))
    result = subprocess.run(command)
    if result.returncode != 0:
        _exit_with_error(f"PyInstaller build failed with exit code {result.returncode}")

    # Clean up temporary build artifacts
    shutil.rmtree(app2exe_dir, ignore_errors=True)
    spec_file = Path(pyxel_app_file).with_suffix(".spec")
    if spec_file.is_file():
        spec_file.unlink()
    shutil.rmtree(Path.cwd() / "build", ignore_errors=True)


def create_html_from_pyxel_app(pyxel_app_file: str) -> None:
    pyxel_app_file = _complete_extension(
        pyxel_app_file, "app2html", pyxel.APP_FILE_EXTENSION
    )
    _check_file_exists(pyxel_app_file)

    base64_string = base64.b64encode(Path(pyxel_app_file).read_bytes()).decode()

    pyxel_app_name = Path(pyxel_app_file).stem
    Path(f"{pyxel_app_name}.html").write_text(
        "<!doctype html>\n"
        f'<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel@{pyxel.VERSION}/wasm/pyxel.js">'
        "</script>\n"
        "<script>\n"
        f'launchPyxel({{ command: "play", name: "{pyxel_app_name}{pyxel.APP_FILE_EXTENSION}", '
        f'gamepad: "enabled", base64: "{base64_string}" }});\n'
        "</script>\n",
        encoding="utf-8",
    )


def copy_pyxel_examples() -> None:
    src_dir = Path(__file__).parent / "examples"
    dst_dir = Path("pyxel_examples")
    shutil.rmtree(dst_dir, ignore_errors=True)

    for src_file in _files_in_dir(src_dir):
        if "__pycache__" in src_file:
            continue
        dst_file = dst_dir / Path(src_file).relative_to(src_dir)
        dst_file.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(src_file, dst_file)
        print(f"copied '{dst_file}'")
