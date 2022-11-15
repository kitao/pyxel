import glob
import multiprocessing
import os
import pathlib
import re
import runpy
import shutil
import sys
import tempfile
import time
import urllib.request
import zipfile

import pyxel


def cli():
    num_args = len(sys.argv)
    command = sys.argv[1] if num_args > 1 else ""
    if command == "run" and num_args == 3:
        run_python_script(sys.argv[2])
    if command == "watch" and num_args == 4:
        watch_and_run_python_script(sys.argv[2], sys.argv[3])
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
    print(f"Pyxel {pyxel.VERSION}, a retro game engine for Python")
    print("usage:")
    print("    pyxel run PYTHON_SCRIPT_FILE(.py)")
    print("    pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE(.py)")
    print("    pyxel play PYXEL_APP_FILE(.pyxapp)")
    print("    pyxel edit [PYXEL_RESOURCE_FILE(.pyxres)]")
    print("    pyxel package APP_DIR STARTUP_SCRIPT_FILE(.py)")
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

    if parse_version(latest_version) > parse_version(pyxel.VERSION):
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


def _check_file_under_dir(filename, dirname):
    if os.path.relpath(filename, dirname).startswith(".."):
        print("specified file is not under the directory")
        exit(1)


def _make_app_dir():
    play_dir = os.path.join(tempfile.gettempdir(), pyxel.WORKING_DIR, "play")
    pathlib.Path(play_dir).mkdir(parents=True, exist_ok=True)
    for path in glob.glob(os.path.join(play_dir, "*")):
        pid = int(os.path.basename(path))
        if not pyxel.process_exists(pid):
            shutil.rmtree(path)
    app_dir = os.path.join(play_dir, str(os.getpid()))
    os.mkdir(app_dir)
    return app_dir


def _timestamps_in_dir(dirname):
    paths = glob.glob(os.path.join(dirname, "*"))
    paths += glob.glob(os.path.join(dirname, "*/*"))
    paths += glob.glob(os.path.join(dirname, "*/*/*"))
    files = filter(os.path.isfile, paths)
    timestamps = {}
    for file in files:
        timestamps[file] = os.path.getmtime(file)
    return timestamps


def _run_python_script_in_separate_process(python_script_file):
    worker = multiprocessing.Process(
        target=run_python_script, args=(python_script_file,)
    )
    worker.daemon = True
    worker.start()
    return worker


def run_python_script(python_script_file):
    python_script_file = _complete_extension(python_script_file, ".py")
    _check_file_exists(python_script_file)
    sys.path.append(os.path.dirname(python_script_file))
    runpy.run_path(python_script_file, run_name="__main__")


def watch_and_run_python_script(watch_dir, python_script_file):
    _check_dir_exists(watch_dir)
    python_script_file = _complete_extension(python_script_file, ".py")
    _check_file_exists(python_script_file)
    _check_file_under_dir(python_script_file, watch_dir)
    watch_info_file = os.path.join(
        os.path.dirname(os.path.abspath(python_script_file)),
        pyxel.WATCH_INFO_FILE,
    )
    try:
        print(f"start watching '{watch_dir}' (Ctrl+C to stop)")
        with open(watch_info_file, "w") as f:
            f.write("valid\n")
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
            if timestamps != last_timestamps:
                print(f"rerun {python_script_file}")
                worker.terminate()
                watch_info = ""
                if os.path.isfile(watch_info_file):
                    with open(watch_info_file) as f:
                        watch_info = f.read()
                if not watch_info.startswith("valid\n"):
                    watch_info = "valid\n" + watch_info
                with open(watch_info_file, "w") as f:
                    f.write(watch_info)
                worker = _run_python_script_in_separate_process(python_script_file)
    except KeyboardInterrupt:
        print("\r", end="")
        print("stopped watching")
    finally:
        if os.path.isfile(watch_info_file):
            os.remove(watch_info_file)


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
            runpy.run_path(startup_script_file, run_name="__main__")
            return
    print(f"file not found: '{pyxel.APP_STARTUP_SCRIPT_FILE}'")
    exit(1)


def edit_pyxel_resource(pyxel_resource_file, starting_editor="image"):
    import pyxel.editor

    pyxel_resource_file = pyxel_resource_file or "my_resource"
    pyxel_resource_file = _complete_extension(
        pyxel_resource_file, pyxel.RESOURCE_FILE_EXTENSION
    )
    pyxel.editor.App(pyxel_resource_file, starting_editor)


def package_pyxel_app(app_dir, startup_script_file):
    _check_dir_exists(app_dir)
    startup_script_file = _complete_extension(startup_script_file, ".py")
    _check_file_exists(startup_script_file)
    _check_file_under_dir(startup_script_file, app_dir)
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
        files = [setting_file] + _files_in_dir(app_dir)
        for file in files:
            if os.path.basename(file) == pyxel_app_file or "/__pycache__/" in file:
                continue
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
