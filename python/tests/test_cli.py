import base64
import multiprocessing
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import zipfile
from pathlib import Path
from types import SimpleNamespace

import pytest
import pyxel
import pyxel.cli
from _assertions import raises_exact  # type: ignore[reportMissingImports]

# Public CLI command tests


class TestCliDispatcher:
    def test_no_args_prints_version_and_usage(self, capsys, monkeypatch):
        monkeypatch.setattr(sys, "argv", ["pyxel"])
        pyxel.cli.cli()
        out = capsys.readouterr().out
        assert f"Pyxel {pyxel.VERSION}" in out
        assert "usage:" in out
        assert "pyxel run" in out
        assert "pyxel package" in out

    def test_unknown_command_exits_with_error(self, capsys, monkeypatch):
        monkeypatch.setattr(sys, "argv", ["pyxel", "nonexistent"])
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.cli()
        assert exc_info.value.code == 1
        out = capsys.readouterr().out
        assert "invalid command: 'nonexistent'" in out
        assert "usage:" in out

    def test_too_few_arguments_exits_with_error(self, capsys, monkeypatch):
        monkeypatch.setattr(sys, "argv", ["pyxel", "run"])
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.cli()
        assert exc_info.value.code == 1
        out = capsys.readouterr().out
        assert "invalid number of parameters" in out
        assert "pyxel run" in out

    def test_too_many_arguments_exits_with_error(self, capsys, monkeypatch):
        monkeypatch.setattr(sys, "argv", ["pyxel", "run", "a.py", "b.py"])
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.cli()
        assert exc_info.value.code == 1
        out = capsys.readouterr().out
        assert "invalid number of parameters" in out


class TestRunCommand:
    def test_missing_file_exits_with_error(self, capsys, tmp_path):
        missing = tmp_path / "nope.py"
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.run_python_script(str(missing))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == f"no such file: '{missing}'\n"

    def test_non_py_file_rejected(self, capsys):
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.run_python_script("foo.txt")
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == "'run' command only accepts .py files\n"


class TestWatchCommand:
    def test_file_changes_restart_after_a_syntax_error(
        self, tmp_path, monkeypatch, capfd
    ):
        app_dir = tmp_path / "app"
        app_dir.mkdir()
        script = app_dir / "main.py"
        result_file = tmp_path / "result.txt"
        source = (
            "from pathlib import Path\n"
            f"Path({str(result_file)!r}).write_text('started')\n"
        )
        script.write_text(source + "import time\ntime.sleep(30)\n", encoding="utf-8")
        monkeypatch.setenv(pyxel.WATCH_STATE_FILE_ENV, "")
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        children_before = set(multiprocessing.active_children())
        deadline = time.monotonic() + 10
        stage = 0
        errors = ""

        def advance(_interval):
            nonlocal stage, errors
            time.sleep(0.05)
            errors += capfd.readouterr().err
            assert time.monotonic() < deadline, (stage, errors)
            value = result_file.read_text() if result_file.exists() else ""
            if stage == 0 and value == "started":
                script.write_text("syntax error!!!\n", encoding="utf-8")
                os.utime(script, (1_000_000_000, 1_000_000_000))
                stage = 1
            elif stage == 1 and "SyntaxError" in errors:
                script.write_text(
                    source.replace("'started'", "'recovered'"), encoding="utf-8"
                )
                os.utime(script, (1_000_000_002, 1_000_000_002))
                stage = 2
            elif stage == 2 and value == "recovered":
                raise KeyboardInterrupt

        # Keep real file watching and child processes; shorten only the polling wait.
        monkeypatch.setattr(
            pyxel.cli, "time", SimpleNamespace(time=time.time, sleep=advance)
        )
        try:
            pyxel.cli.watch_and_run_python_script(str(app_dir), str(script))
        finally:
            for child in set(multiprocessing.active_children()) - children_before:
                child.terminate()
                child.join(timeout=5)

        assert stage == 2
        assert result_file.read_text() == "recovered"
        assert "stopped watching" in capfd.readouterr().out

    def test_missing_dir_exits_with_error(self, capsys, tmp_path):
        missing_dir = tmp_path / "nodir"
        script = tmp_path / "script.py"
        script.write_text("", encoding="utf-8")
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.watch_and_run_python_script(str(missing_dir), str(script))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == f"no such directory: '{missing_dir}'\n"

    def test_missing_script_exits_with_error(self, capsys, tmp_path):
        watch_dir = tmp_path / "dir"
        watch_dir.mkdir()
        missing = tmp_path / "nope.py"
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.watch_and_run_python_script(str(watch_dir), str(missing))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == f"no such file: '{missing}'\n"

    def test_script_outside_dir_rejected(self, capsys, tmp_path):
        watch_dir = tmp_path / "dir"
        watch_dir.mkdir()
        outside = tmp_path / "outside.py"
        outside.write_text("", encoding="utf-8")
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.watch_and_run_python_script(str(watch_dir), str(outside))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == "specified file is not under the directory\n"

    def test_non_py_script_rejected(self, capsys, tmp_path):
        watch_dir = tmp_path / "dir"
        watch_dir.mkdir()
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.watch_and_run_python_script(str(watch_dir), "foo.txt")
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == "'watch' command only accepts .py files\n"


class TestPyxelAppMetadata:
    @pytest.mark.parametrize(
        "metadata",
        [{}, {"title": "My App"}, {"author": "Me"}, {"desc": "Example"}],
    )
    def test_missing_metadata_does_not_block_packaging_or_playback(
        self, tmp_path, monkeypatch, capsys, metadata
    ):
        app_dir = _make_app(tmp_path)
        headers = "".join(f"# {key}: {value}\n" for key, value in metadata.items())
        (app_dir / "main.py").write_text(
            headers + 'print("app ran")\n', encoding="utf-8"
        )
        monkeypatch.chdir(tmp_path)
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        monkeypatch.setattr(sys, "path", sys.path.copy())

        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        app_file = str(tmp_path / "my_app.pyxapp")
        assert pyxel.cli.get_pyxel_app_metadata(app_file) == metadata
        capsys.readouterr()

        pyxel.cli.play_pyxel_app(app_file)

        assert "app ran" in capsys.readouterr().out.splitlines()

    def test_get_metadata_returns_expected_fields(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        metadata = pyxel.cli.get_pyxel_app_metadata(str(tmp_path / "my_app.pyxapp"))
        assert metadata == {"title": "My App", "author": "Me"}

    def test_print_metadata_outputs_zip_comment(self, capsys, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        capsys.readouterr()  # discard package output
        pyxel.cli.print_pyxel_app_metadata(str(tmp_path / "my_app.pyxapp"))
        out = capsys.readouterr().out
        assert "My App" in out
        assert "Me" in out


class TestEditCommand:
    @pytest.mark.parametrize(
        ("args", "expected"),
        [
            ((), ("my_resource.pyxres", "image")),
            (("custom.pyxres",), ("custom.pyxres", "image")),
            (("custom.pyxres", "music"), ("custom.pyxres", "music")),
        ],
    )
    def test_invokes_editor_app(self, monkeypatch, args, expected):
        invocations = []

        class FakeApp:
            def __init__(self, resource_file, starting_editor):
                invocations.append((resource_file, starting_editor))

        monkeypatch.setattr("pyxel.editor.App", FakeApp)
        pyxel.cli.edit_pyxel_resource(*args)
        assert invocations == [expected]


class TestPlayCommand:
    def test_missing_file_exits_with_error(self, capsys, tmp_path):
        missing = tmp_path / "nope.pyxapp"
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.play_pyxel_app(str(missing))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == f"no such file: '{missing}'\n"

    def test_non_pyxapp_extension_rejected(self, capsys):
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.play_pyxel_app("foo.txt")
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == (
            f"'play' command only accepts {pyxel.APP_FILE_EXTENSION} files\n"
        )

    def test_missing_startup_marker_exits_with_exact_error(
        self, capsys, tmp_path, monkeypatch
    ):
        app_file = tmp_path / "empty.pyxapp"
        with zipfile.ZipFile(app_file, "w") as zf:
            zf.writestr("data.txt", "data")
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))

        with pytest.raises(SystemExit) as exc:
            pyxel.cli.play_pyxel_app(str(app_file))

        assert exc.value.code == 1
        assert (
            capsys.readouterr().out
            == f"no such file: '{pyxel.APP_STARTUP_SCRIPT_FILE}'\n"
        )


class TestPackage:
    def test_with_relative_paths_from_parent(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        assert (tmp_path / "my_app.pyxapp").is_file()

    def test_with_relative_paths_from_app_dir(self, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        monkeypatch.chdir(app_dir)
        pyxel.cli.package_pyxel_app(".", "main.py")
        assert (app_dir / "my_app.pyxapp").is_file()

    def test_with_absolute_paths(self, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app(str(app_dir), str(app_dir / "main.py"))
        assert (tmp_path / "my_app.pyxapp").is_file()

    def test_pyxapp_contents(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            names = set(zf.namelist())
        assert f"my_app/{pyxel.APP_STARTUP_SCRIPT_FILE}" in names
        assert "my_app/main.py" in names
        assert "my_app/assets/data.txt" in names

    @pytest.mark.skipif(sys.platform == "win32", reason="symlink may require elevation")
    def test_pyxapp_includes_linked_directory_contents(self, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        shared = tmp_path / "shared"
        shared.mkdir()
        (shared / "data.txt").write_bytes(b"shared asset")
        (app_dir / "linked").symlink_to(shared, target_is_directory=True)
        monkeypatch.chdir(tmp_path)

        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")

        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            assert zf.read("my_app/linked/data.txt") == b"shared asset"

    def test_pyxapp_excludes_dotfiles_and_hidden_assets(self, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        (app_dir / ".env").write_text("TOKEN=value\n", encoding="utf-8")
        hidden_dir = app_dir / ".assets"
        hidden_dir.mkdir()
        (hidden_dir / "data.txt").write_text("hidden asset\n", encoding="utf-8")
        monkeypatch.chdir(tmp_path)

        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")

        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            names = set(zf.namelist())
        assert f"my_app/{pyxel.APP_STARTUP_SCRIPT_FILE}" in names
        assert "my_app/.env" not in names
        assert "my_app/.assets/data.txt" not in names

    def test_pyxapp_excludes_gif_and_zip_files(self, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        (app_dir / "preview.GIF").write_bytes(b"gif")
        (app_dir / "source.ZIP").write_bytes(b"zip")
        monkeypatch.chdir(tmp_path)

        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")

        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            names = set(zf.namelist())
        assert "my_app/preview.GIF" not in names
        assert "my_app/source.ZIP" not in names

    @pytest.mark.parametrize("parent_name", ["__pycache__", "build__pycache__data"])
    def test_pyxapp_cache_filter_ignores_ancestor_names(
        self, parent_name, tmp_path, monkeypatch
    ):
        work_dir = tmp_path / parent_name
        work_dir.mkdir()
        _make_app(work_dir)
        monkeypatch.chdir(work_dir)

        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")

        with zipfile.ZipFile(work_dir / "my_app.pyxapp") as zf:
            names = set(zf.namelist())
        assert "my_app/main.py" in names
        assert "my_app/assets/data.txt" in names

    def test_pyxapp_cache_filter_matches_complete_relative_component(
        self, tmp_path, monkeypatch
    ):
        app_dir = _make_app(tmp_path)
        cache_dir = app_dir / "__pycache__"
        cache_dir.mkdir()
        (cache_dir / "ignored.pyc").write_bytes(b"cache")
        named_dir = app_dir / "build__pycache__data"
        named_dir.mkdir()
        (named_dir / "included.txt").write_text("data", encoding="utf-8")
        monkeypatch.chdir(tmp_path)

        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")

        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            names = set(zf.namelist())
        assert "my_app/__pycache__/ignored.pyc" not in names
        assert "my_app/build__pycache__data/included.txt" in names

    def test_pyxapp_has_no_duplicate_entries(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            names = zf.namelist()
        assert len(names) == len(set(names)), f"duplicate entries in pyxapp: {names}"

    def test_startup_script_pointer_uses_archive_separator(self, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        startup_file = app_dir / "src" / "main.py"
        startup_file.parent.mkdir()
        startup_file.write_text("", encoding="utf-8")
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/src/main.py")
        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            pointer = zf.read(f"my_app/{pyxel.APP_STARTUP_SCRIPT_FILE}").decode("utf-8")
        assert pointer == "src/main.py"

    def test_preserves_existing_startup_marker(self, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        setting_file = app_dir / pyxel.APP_STARTUP_SCRIPT_FILE
        setting_file.write_text("preserve-me", encoding="utf-8")
        monkeypatch.chdir(tmp_path)

        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")

        assert setting_file.read_text(encoding="utf-8") == "preserve-me"
        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            pointer = zf.read(f"my_app/{pyxel.APP_STARTUP_SCRIPT_FILE}").decode("utf-8")
        assert pointer == "main.py"

    def test_rejects_non_py_startup_script(self, capsys, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        (app_dir / "main.txt").write_text("", encoding="utf-8")
        monkeypatch.chdir(tmp_path)
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.package_pyxel_app("my_app", "my_app/main.txt")
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == "'package' command only accepts .py files\n"

    def test_rejects_startup_script_outside_app_dir(
        self, capsys, tmp_path, monkeypatch
    ):
        _make_app(tmp_path)
        outside = tmp_path / "outside.py"
        outside.write_text("", encoding="utf-8")
        monkeypatch.chdir(tmp_path)
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.package_pyxel_app("my_app", str(outside))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == "specified file is not under the directory\n"

    @pytest.mark.parametrize(
        "startup", [".main.py", ".src/main.py", "__pycache__/main.py"]
    )
    def test_rejects_excluded_startup_script(
        self, startup, capsys, tmp_path, monkeypatch
    ):
        app_dir = tmp_path / "my_app"
        startup_file = app_dir / startup
        startup_file.parent.mkdir(parents=True)
        startup_file.write_text("", encoding="utf-8")
        monkeypatch.chdir(tmp_path)

        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.package_pyxel_app("my_app", str(Path("my_app") / startup))

        assert exc_info.value.code == 1
        assert (
            capsys.readouterr().out == "startup script is excluded from the Pyxel app\n"
        )
        assert not (tmp_path / "my_app.pyxapp").exists()

    @pytest.mark.skipif(os.name == "nt", reason="paths are not valid on Windows")
    @pytest.mark.parametrize(
        "startup",
        ["C:/main.py", "src\\main.py", "C:main.py", "main\tname.py", "main\nname.py"],
    )
    def test_preserves_posix_startup_path(self, startup, capsys, tmp_path, monkeypatch):
        app_dir = tmp_path / "my_app"
        startup_file = app_dir / startup
        startup_file.parent.mkdir(parents=True)
        startup_file.write_text("print('startup ran')\n", encoding="utf-8")
        monkeypatch.chdir(tmp_path)
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        monkeypatch.setattr(sys, "path", sys.path[:])

        pyxel.cli.package_pyxel_app("my_app", str(Path("my_app") / startup))
        capsys.readouterr()
        pyxel.cli.play_pyxel_app("my_app.pyxapp")

        assert capsys.readouterr().out == "startup ran\n"

    def test_stdout_shows_added_files(self, capsys, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        out = capsys.readouterr().out
        assert "added 'my_app/main.py'" in out
        assert "added 'my_app/assets/data.txt'" in out
        assert f"added 'my_app/{pyxel.APP_STARTUP_SCRIPT_FILE}'" in out

    def test_packaging_failure_preserves_existing_app(self, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        app_file = tmp_path / "my_app.pyxapp"
        app_file.write_bytes(b"existing app")

        def raise_on_write(*_args, **_kwargs):
            raise RuntimeError("zip write failed")

        monkeypatch.setattr(zipfile.ZipFile, "write", raise_on_write)

        with raises_exact(RuntimeError, "zip write failed"):
            pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")

        assert app_file.read_bytes() == b"existing app"
        assert list(tmp_path.glob(".my_app.pyxapp.*.tmp")) == []
        assert not (app_dir / pyxel.APP_STARTUP_SCRIPT_FILE).exists()


class TestApp2exe:
    def test_missing_pyinstaller_names_the_supported_install_extra(
        self, capsys, tmp_path, monkeypatch
    ):
        app_file = tmp_path / "my_app.pyxapp"
        app_file.write_bytes(b"not used")
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        monkeypatch.setattr(pyxel.cli.importlib.util, "find_spec", lambda _name: None)

        with pytest.raises(SystemExit) as exc:
            pyxel.cli.create_executable_from_pyxel_app(str(app_file))

        assert exc.value.code == 1
        assert capsys.readouterr().out == (
            "PyInstaller is not installed. Install app2exe support with: "
            'pip install "pyxel[app2exe]"\n'
        )
        assert list((tmp_path / pyxel.BASE_DIR).glob("app2exe-*")) == []

    def test_build_artifacts_are_isolated_from_user_files(self, tmp_path, monkeypatch):
        app_file = tmp_path / "my_app.pyxapp"
        app_file.write_bytes(b"not used")
        startup_script = tmp_path / "main.py"
        startup_script.write_text("", encoding="utf-8")

        cwd = tmp_path / "work"
        cwd.mkdir()
        build_sentinel = cwd / "build" / "keep.txt"
        build_sentinel.parent.mkdir()
        build_sentinel.write_text("keep", encoding="utf-8")
        spec_sentinel = app_file.with_suffix(".spec")
        spec_sentinel.write_text("keep", encoding="utf-8")

        temp_dir = tmp_path / "temp"
        commands = []

        def run_pyinstaller(args, *, check):
            commands.append(args)
            return subprocess.CompletedProcess(args, 0)

        monkeypatch.chdir(cwd)
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(temp_dir))
        monkeypatch.setattr(
            pyxel.cli.importlib.util, "find_spec", lambda _name: object()
        )
        monkeypatch.setattr(
            pyxel.cli, "_extract_pyxel_app", lambda _path: str(startup_script)
        )
        monkeypatch.setattr(
            pyxel.cli.pyxel.utils,
            "list_imported_modules",
            lambda _path: {"system": []},
        )
        monkeypatch.setattr(pyxel.cli.subprocess, "run", run_pyinstaller)

        pyxel.cli.create_executable_from_pyxel_app(os.path.relpath(app_file, cwd))
        pyxel.cli.create_executable_from_pyxel_app(os.path.relpath(app_file, cwd))

        assert len(commands) == 2
        work_dirs = []
        for command in commands:
            app2exe_dir = Path(command[command.index("--specpath") + 1])
            work_dirs.append(app2exe_dir)
            assert app2exe_dir.parent == temp_dir / pyxel.BASE_DIR
            assert app2exe_dir.name.startswith("app2exe-")
            assert command[command.index("--workpath") + 1] == str(
                app2exe_dir / "build"
            )
            add_data = command[command.index("--add-data") + 1]
            assert add_data == f"{app_file}{os.pathsep}."
            assert not app2exe_dir.exists()
        assert work_dirs[0] != work_dirs[1]
        assert build_sentinel.read_text(encoding="utf-8") == "keep"
        assert spec_sentinel.read_text(encoding="utf-8") == "keep"

    @pytest.mark.skipif(sys.platform == "win32", reason="symlink may require elevation")
    def test_symlink_name_is_preserved_in_pyinstaller_command(
        self, tmp_path, monkeypatch
    ):
        app_file = tmp_path / "real.pyxapp"
        app_file.write_bytes(b"not used")
        symlink = tmp_path / "alias.pyxapp"
        symlink.symlink_to(app_file)
        startup_script = tmp_path / "main.py"
        startup_script.write_text("", encoding="utf-8")

        command = None
        bootstrap_script = None

        def run_pyinstaller(args, *, check):
            nonlocal command, bootstrap_script
            command = args
            bootstrap_script = Path(args[-1]).read_text(encoding="utf-8")
            return subprocess.CompletedProcess(args, 0)

        monkeypatch.setattr(
            pyxel.cli.importlib.util, "find_spec", lambda _name: object()
        )
        monkeypatch.setattr(
            pyxel.cli, "_extract_pyxel_app", lambda _path: str(startup_script)
        )
        monkeypatch.setattr(
            pyxel.cli.pyxel.utils,
            "list_imported_modules",
            lambda _path: {"system": []},
        )
        monkeypatch.setattr(pyxel.cli.subprocess, "run", run_pyinstaller)

        pyxel.cli.create_executable_from_pyxel_app(str(symlink))

        assert command is not None
        add_data = command[command.index("--add-data") + 1]
        assert add_data == f"{symlink.absolute()}{os.pathsep}."
        assert bootstrap_script is not None
        assert "alias.pyxapp" in bootstrap_script

    def test_uppercase_extension_is_preserved_in_bootstrap(self, tmp_path, monkeypatch):
        app_file = tmp_path / "Demo.PYXAPP"
        app_file.write_bytes(b"not used")
        startup_script = tmp_path / "main.py"
        startup_script.write_text("", encoding="utf-8")
        bootstrap_script = None

        def run_pyinstaller(args, *, check):
            nonlocal bootstrap_script
            bootstrap_script = Path(args[-1]).read_text(encoding="utf-8")
            return subprocess.CompletedProcess(args, 0)

        monkeypatch.setattr(
            pyxel.cli.importlib.util, "find_spec", lambda _name: object()
        )
        monkeypatch.setattr(
            pyxel.cli, "_extract_pyxel_app", lambda _path: str(startup_script)
        )
        monkeypatch.setattr(
            pyxel.cli.pyxel.utils,
            "list_imported_modules",
            lambda _path: {"system": []},
        )
        monkeypatch.setattr(pyxel.cli.subprocess, "run", run_pyinstaller)

        pyxel.cli.create_executable_from_pyxel_app(str(app_file))

        assert bootstrap_script is not None
        assert "Demo.PYXAPP" in bootstrap_script
        assert "Demo.pyxapp" not in bootstrap_script

    def test_exe_runs_with_resource(self, tmp_path, monkeypatch):
        pytest.importorskip("PyInstaller")
        monkeypatch.setenv("PYINSTALLER_CONFIG_DIR", str(tmp_path / "pyinstaller"))
        app_dir = tmp_path / "my_app"
        (app_dir / "assets").mkdir(parents=True)
        shutil.copy(
            Path(pyxel.__file__).parent / "examples" / "assets" / "sample.pyxres",
            app_dir / "assets" / "sample.pyxres",
        )
        (app_dir / "main.py").write_text(
            "from xml.etree import ElementTree\n"
            "import pyxel\n"
            "assert ElementTree.fromstring('<root />').tag == 'root'\n"
            "pyxel.init(64, 64, headless=True)\n"
            'pyxel.load("assets/sample.pyxres")\n'
            "def update():\n"
            "    pyxel.quit()\n"
            "def draw():\n"
            "    pyxel.cls(0)\n"
            "    pyxel.blt(0, 0, 0, 0, 0, 16, 16)\n"
            "pyxel.run(update, draw)\n",
            encoding="utf-8",
        )
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        pyxel.cli.create_executable_from_pyxel_app("my_app.pyxapp")

        exe_name = "my_app.exe" if sys.platform == "win32" else "my_app"
        exe = tmp_path / "dist" / "my_app" / exe_name
        assert exe.is_file()
        result = subprocess.run(
            [str(exe)], capture_output=True, timeout=60, check=False
        )
        assert result.returncode == 0, (
            f"exe failed rc={result.returncode}\n"
            f"stdout={result.stdout.decode(errors='replace')}\n"
            f"stderr={result.stderr.decode(errors='replace')}"
        )

    def test_exe_runs_with_parent_package_dependency(self, tmp_path, monkeypatch):
        pytest.importorskip("PyInstaller")
        monkeypatch.setenv("PYINSTALLER_CONFIG_DIR", str(tmp_path / "pyinstaller"))
        sentinel = tmp_path / "parent-import.success"
        monkeypatch.setenv("PYXEL_PARENT_IMPORT_SENTINEL", str(sentinel))
        app_dir = tmp_path / "my_app"
        pkg = app_dir / "pkg"
        (pkg / "sub").mkdir(parents=True)
        (pkg / "__init__.py").write_text(
            "import sqlite3\n"
            "with sqlite3.connect(':memory:') as connection:\n"
            "    VALUE = connection.execute('select 42').fetchone()[0]\n",
            encoding="utf-8",
        )
        (pkg / "sub" / "__init__.py").write_text("VALUE = 42\n", encoding="utf-8")
        (app_dir / "main.py").write_text(
            "import os\n"
            "from pathlib import Path\n"
            "import pkg.sub\n"
            "assert pkg.VALUE == pkg.sub.VALUE == 42\n"
            "Path(os.environ['PYXEL_PARENT_IMPORT_SENTINEL']).write_text('ok')\n",
            encoding="utf-8",
        )
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")

        pyxel.cli.create_executable_from_pyxel_app("my_app.pyxapp")

        exe_name = "my_app.exe" if sys.platform == "win32" else "my_app"
        executable = tmp_path / "dist" / "my_app" / exe_name
        result = subprocess.run(
            [str(executable)], capture_output=True, timeout=60, check=False
        )
        assert result.returncode == 0, (
            f"exe failed rc={result.returncode}\n"
            f"stdout={result.stdout.decode(errors='replace')}\n"
            f"stderr={result.stderr.decode(errors='replace')}"
        )
        assert sentinel.read_text(encoding="utf-8") == "ok"

    @pytest.mark.skipif(sys.platform == "win32", reason="symlink may require elevation")
    def test_exe_runs_with_symlinked_pyxapp(self, tmp_path, monkeypatch):
        pytest.importorskip("PyInstaller")
        monkeypatch.setenv("PYINSTALLER_CONFIG_DIR", str(tmp_path / "pyinstaller"))

        app_dir = tmp_path / "real"
        app_dir.mkdir()
        (app_dir / "main.py").write_text(
            "import pyxel\npyxel.init(16, 16, headless=True)\npyxel.quit()\n",
            encoding="utf-8",
        )
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("real", "real/main.py")
        Path("alias.pyxapp").symlink_to("real.pyxapp")

        pyxel.cli.create_executable_from_pyxel_app("alias.pyxapp")

        executable = tmp_path / "dist" / "alias" / "alias"
        result = subprocess.run(
            [str(executable)], capture_output=True, timeout=60, check=False
        )
        assert result.returncode == 0, (
            f"exe failed rc={result.returncode}\n"
            f"stdout={result.stdout.decode(errors='replace')}\n"
            f"stderr={result.stderr.decode(errors='replace')}"
        )

    def test_missing_pyxapp_exits_with_error(self, capsys, tmp_path):
        missing = tmp_path / "nope.pyxapp"
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.create_executable_from_pyxel_app(str(missing))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == f"no such file: '{missing}'\n"

    def test_non_pyxapp_extension_rejected(self, capsys):
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.create_executable_from_pyxel_app("foo.txt")
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == (
            f"'app2exe' command only accepts {pyxel.APP_FILE_EXTENSION} files\n"
        )

    def test_missing_startup_marker_exits_with_exact_error(
        self, capsys, tmp_path, monkeypatch
    ):
        app_file = tmp_path / "broken.pyxapp"
        app_file.write_bytes(b"not used")
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        monkeypatch.setattr(
            pyxel.cli.importlib.util, "find_spec", lambda _name: object()
        )
        monkeypatch.setattr(pyxel.cli, "_extract_pyxel_app", lambda _path: None)

        with pytest.raises(SystemExit) as exc:
            pyxel.cli.create_executable_from_pyxel_app(str(app_file))

        assert exc.value.code == 1
        assert (
            capsys.readouterr().out
            == f"no such file: '{pyxel.APP_STARTUP_SCRIPT_FILE}'\n"
        )
        assert list((tmp_path / pyxel.BASE_DIR).glob("app2exe-*")) == []


class TestApp2html:
    def test_html_embeds_pyxapp_as_base64(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        pyxel.cli.create_html_from_pyxel_app("my_app.pyxapp")

        html = (tmp_path / "my_app.html").read_text(encoding="utf-8")
        assert (
            f"https://cdn.jsdelivr.net/gh/kitao/pyxel@{pyxel.VERSION}/wasm/pyxel.js"
            in html
        )
        assert "launchPyxel(" in html
        assert 'command: "play"' in html
        assert 'name: "my_app.pyxapp"' in html
        assert 'gamepad: "enabled"' in html

        match = re.search(r'base64:\s*"([^"]+)"', html)
        assert match is not None, "base64 payload not found in html"
        payload = base64.b64decode(match.group(1))
        assert payload == (tmp_path / "my_app.pyxapp").read_bytes()

    def test_html_escapes_app_name_as_javascript_string(self, tmp_path, monkeypatch):
        pyxel_app_file = tmp_path / 'bad"name\\line.pyxapp'
        pyxel_app_file.write_bytes(b"payload")
        monkeypatch.chdir(tmp_path)

        pyxel.cli.create_html_from_pyxel_app(str(pyxel_app_file))

        html = (tmp_path / 'bad"name\\line.html').read_text(encoding="utf-8")
        assert 'name: "bad\\"name\\\\line.pyxapp"' in html

    def test_documented_gamepad_removal_keeps_valid_javascript(
        self, tmp_path, monkeypatch
    ):
        node = shutil.which("node")
        if node is None:
            pytest.skip("Node.js is required to check the generated JavaScript")
        web_usage = (
            Path(__file__).resolve().parents[2] / "web/web-usage/index.html"
        ).read_text(encoding="utf-8")
        instruction = re.search(
            r"""t\("app2html_gamepad"\)\.replace\("\{0\}", chip\('([^']+)'\)\)""",
            web_usage,
        )
        assert instruction is not None

        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        pyxel.cli.create_html_from_pyxel_app("my_app.pyxapp")
        html = (tmp_path / "my_app.html").read_text(encoding="utf-8")
        script = re.search(r"<script>\s*(.*?)\s*</script>", html, re.DOTALL)
        assert script is not None
        assert script.group(1).count(instruction.group(1)) == 1

        result = subprocess.run(
            [node, "--check"],
            input=script.group(1).replace(instruction.group(1), ""),
            text=True,
            capture_output=True,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_missing_pyxapp_exits_with_error(self, capsys, tmp_path):
        missing = tmp_path / "nope.pyxapp"
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.create_html_from_pyxel_app(str(missing))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == f"no such file: '{missing}'\n"

    def test_non_pyxapp_extension_rejected(self, capsys):
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli.create_html_from_pyxel_app("foo.txt")
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == (
            f"'app2html' command only accepts {pyxel.APP_FILE_EXTENSION} files\n"
        )


class TestCopyExamples:
    def test_creates_examples_dir(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        pyxel.cli.copy_pyxel_examples()
        dst = tmp_path / "pyxel_examples"
        assert dst.is_dir()
        assert (dst / "01_hello_pyxel.py").is_file()
        assert (dst / "assets").is_dir()

    def test_excludes_pycache(self, tmp_path, monkeypatch):
        package_dir = tmp_path / "package"
        examples_dir = package_dir / "examples"
        cache_dir = examples_dir / "__pycache__"
        cache_dir.mkdir(parents=True)
        (cache_dir / "main.pyc").write_bytes(b"cache")
        (examples_dir / "main.py").write_text("import pyxel\n", encoding="utf-8")
        monkeypatch.setattr(pyxel.cli, "__file__", str(package_dir / "cli.py"))
        monkeypatch.chdir(tmp_path)

        pyxel.cli.copy_pyxel_examples()

        dst = tmp_path / "pyxel_examples"
        assert (dst / "main.py").read_text(encoding="utf-8") == "import pyxel\n"
        assert list(dst.rglob("__pycache__")) == []

    def test_prints_copied_paths(self, capsys, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        pyxel.cli.copy_pyxel_examples()
        out = capsys.readouterr().out
        assert "copied 'pyxel_examples/01_hello_pyxel.py'" in out

    def test_overwrites_existing_dir(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        dst = tmp_path / "pyxel_examples"
        dst.mkdir()
        stale = dst / "stale.txt"
        stale.write_text("old", encoding="utf-8")
        pyxel.cli.copy_pyxel_examples()
        assert not stale.exists()
        assert (dst / "01_hello_pyxel.py").is_file()


# Private helper tests


class TestErrorHelpers:
    def test_exit_with_error_prints_message_and_exits(self, capsys):
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli._exit_with_error("boom")
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == "boom\n"

    def test_complete_extension_appends_missing_ext(self):
        assert pyxel.cli._complete_extension("foo", "run", ".py") == "foo.py"


class TestWatchHelpers:
    @pytest.mark.skipif(sys.platform == "win32", reason="symlink may require elevation")
    def test_directory_links_preserve_aliases_without_following_cycles(self, tmp_path):
        app_dir = tmp_path / "app"
        app_dir.mkdir()
        shared = tmp_path / "shared"
        shared.mkdir()
        (shared / "data.txt").write_text("data", encoding="utf-8")
        (shared / "back").symlink_to(app_dir, target_is_directory=True)
        for name in ("first", "second"):
            (app_dir / name).symlink_to(shared, target_is_directory=True)

        expected = [str(app_dir / name / "data.txt") for name in ("first", "second")]
        assert pyxel.cli._files_in_dir(app_dir) == expected
        assert sorted(pyxel.cli._timestamps_in_dir(app_dir)) == expected

    @pytest.mark.skipif(sys.platform == "win32", reason="symlink may require elevation")
    def test_timestamps_in_dir_detects_linked_file_changes(self, tmp_path):
        app_dir = tmp_path / "app"
        app_dir.mkdir()
        shared = tmp_path / "shared"
        shared.mkdir()
        (app_dir / "linked").symlink_to(shared, target_is_directory=True)
        target = shared / "data.txt"
        target.write_text("before", encoding="utf-8")
        os.utime(target, (1_000_000_000, 1_000_000_000))
        before = pyxel.cli._timestamps_in_dir(app_dir)

        target.write_text("after", encoding="utf-8")
        os.utime(target, (1_000_000_002, 1_000_000_002))
        after = pyxel.cli._timestamps_in_dir(app_dir)

        linked_file = str(app_dir / "linked" / "data.txt")
        assert before == {linked_file: 1_000_000_000}
        assert after == {linked_file: 1_000_000_002}
        target.unlink()
        assert pyxel.cli._timestamps_in_dir(app_dir) == {}

    def test_create_app_dir_tolerates_concurrent_stale_cleanup(
        self, tmp_path, monkeypatch
    ):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        monkeypatch.setattr(pyxel.cli.pyxel, "_pid_exists", lambda _pid: False)
        stale_dir = tmp_path / pyxel.BASE_DIR / "play" / "999999_stale"
        stale_dir.mkdir(parents=True)
        original_stat = Path.stat

        def remove_before_stat(path, *args, **kwargs):
            if path == stale_dir:
                shutil.rmtree(stale_dir)
                raise FileNotFoundError(stale_dir)
            return original_stat(path, *args, **kwargs)

        monkeypatch.setattr(Path, "stat", remove_before_stat)

        app_dir = Path(pyxel.cli._create_app_dir())

        assert app_dir.is_dir()

    @pytest.mark.parametrize("disappear", [False, True])
    def test_timestamps_in_dir_lists_nested_files(
        self, disappear, tmp_path, monkeypatch
    ):
        (tmp_path / "a.py").write_text("a", encoding="utf-8")
        sub = tmp_path / "sub"
        sub.mkdir()
        nested_file = sub / "b.py"
        nested_file.write_text("b", encoding="utf-8")
        original_is_file = Path.is_file

        def remove_after_is_file(path):
            is_file = original_is_file(path)
            if disappear and path == nested_file and is_file:
                path.unlink()
            return is_file

        monkeypatch.setattr(Path, "is_file", remove_after_is_file)

        result = pyxel.cli._timestamps_in_dir(str(tmp_path))

        expected = {str(tmp_path / "a.py")}
        if not disappear:
            expected.add(str(nested_file))
        assert set(result.keys()) == expected

    def test_timestamps_in_dir_detects_modification(self, tmp_path):
        f = tmp_path / "a.py"
        f.write_text("a", encoding="utf-8")
        os.utime(f, (1_000_000_000, 1_000_000_000))
        before = pyxel.cli._timestamps_in_dir(str(tmp_path))
        f.write_text("b", encoding="utf-8")
        os.utime(f, (1_000_000_002, 1_000_000_002))
        after = pyxel.cli._timestamps_in_dir(str(tmp_path))
        assert before == {str(f): 1_000_000_000}
        assert after == {str(f): 1_000_000_002}

    def test_create_watch_state_file_touches_pid_file(self, tmp_path, monkeypatch):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        state_file = pyxel.cli._create_watch_state_file()
        assert Path(state_file).is_file()
        assert Path(state_file).name == str(os.getpid())

    def test_create_watch_state_file_cleans_dead_process_files(
        self, tmp_path, monkeypatch
    ):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        monkeypatch.setattr(pyxel.cli.pyxel, "_pid_exists", lambda _pid: False)
        watch_dir = tmp_path / pyxel.BASE_DIR / "watch"
        watch_dir.mkdir(parents=True)
        dead_file = watch_dir / str(os.getpid() + 1)
        dead_file.touch()
        pyxel.cli._create_watch_state_file()
        assert not dead_file.exists()

    def test_create_watch_state_file_tolerates_concurrent_cleanup(
        self, tmp_path, monkeypatch
    ):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        monkeypatch.setattr(pyxel.cli.pyxel, "_pid_exists", lambda _pid: False)
        watch_dir = tmp_path / pyxel.BASE_DIR / "watch"
        watch_dir.mkdir(parents=True)
        dead_file = watch_dir / "999999"
        dead_file.touch()
        original_unlink = Path.unlink

        def remove_before_unlink(path, *args, **kwargs):
            if path == dead_file and path.exists():
                original_unlink(path)
            return original_unlink(path, *args, **kwargs)

        monkeypatch.setattr(Path, "unlink", remove_before_unlink)

        state_file = pyxel.cli._create_watch_state_file()

        assert Path(state_file).is_file()


class TestExtractPyxelAppSafety:
    @staticmethod
    def _build_zip_with_entry(zip_path, entry_name):
        with zipfile.ZipFile(zip_path, "w") as zf:
            zf.writestr(entry_name, b"malicious content")

    def test_rejects_path_traversal_entry(self, capsys, tmp_path, monkeypatch):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        zip_path = tmp_path / "evil.pyxapp"
        self._build_zip_with_entry(zip_path, "../evil.txt")
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli._extract_pyxel_app(str(zip_path))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == "unsafe path in Pyxel app: '../evil.txt'\n"

    def test_rejects_absolute_path_entry(self, capsys, tmp_path, monkeypatch):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        zip_path = tmp_path / "evil.pyxapp"
        self._build_zip_with_entry(zip_path, "/etc/passwd")
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli._extract_pyxel_app(str(zip_path))
        assert exc_info.value.code == 1
        assert capsys.readouterr().out == "unsafe path in Pyxel app: '/etc/passwd'\n"

    @pytest.mark.parametrize(
        "startup_path",
        [
            "",
            "../outside.py",
            "src/../../outside.py",
            "src\\..\\..\\outside.py",
            "/tmp/outside.py",
            "C:\\outside.py",
            "\\\\server\\share\\main.py",
            "main\0.py",
            "main.py\nother.py",
            "missing.py",
        ],
    )
    def test_rejects_invalid_startup_script_pointer(
        self, startup_path, capsys, tmp_path, monkeypatch
    ):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        app_file = tmp_path / "invalid-startup.pyxapp"
        with zipfile.ZipFile(app_file, "w") as zf:
            zf.writestr(f"app/{pyxel.APP_STARTUP_SCRIPT_FILE}", startup_path)
            zf.writestr("outside.py", "VALUE = 1\n")

        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli._extract_pyxel_app(str(app_file))

        assert exc_info.value.code == 1
        assert capsys.readouterr().out == (
            f"invalid startup script path in Pyxel app: {startup_path!r}\n"
        )

    @pytest.mark.skipif(os.name == "nt", reason="paths are not valid on Windows")
    @pytest.mark.parametrize(
        "startup_path", ["main\\file.py", "main\tfile.py", "main\nfile.py"]
    )
    def test_reads_posix_startup_script_pointer(
        self, startup_path, tmp_path, monkeypatch
    ):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        app_file = tmp_path / "posix.pyxapp"
        with zipfile.ZipFile(app_file, "w") as zf:
            zf.writestr(f"app/{pyxel.APP_STARTUP_SCRIPT_FILE}", startup_path)
            zf.writestr(f"app/{startup_path}", "VALUE = 42\n")

        startup_script = pyxel.cli._extract_pyxel_app(str(app_file))

        assert startup_script is not None
        assert Path(startup_script).name == startup_path
        assert Path(startup_script).read_text(encoding="utf-8") == "VALUE = 42\n"

    @pytest.mark.skipif(os.name == "nt", reason="backslash is native on Windows")
    def test_reads_windows_startup_script_pointer(self, tmp_path, monkeypatch):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        app_file = tmp_path / "windows.pyxapp"
        with zipfile.ZipFile(app_file, "w") as zf:
            zf.writestr(f"app/{pyxel.APP_STARTUP_SCRIPT_FILE}", "src\\main.py")
            zf.writestr("app/src/main.py", "VALUE = 42\n")

        startup_script = pyxel.cli._extract_pyxel_app(str(app_file))

        assert startup_script is not None
        assert Path(startup_script).read_text(encoding="utf-8") == "VALUE = 42\n"

    @pytest.mark.parametrize("startup_path", ["src/../main.py", "src\\..\\main.py"])
    def test_reads_normalized_startup_script_inside_application(
        self, startup_path, tmp_path, monkeypatch
    ):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        app_file = tmp_path / "normalized-startup.pyxapp"
        with zipfile.ZipFile(app_file, "w") as zf:
            zf.writestr(f"app/{pyxel.APP_STARTUP_SCRIPT_FILE}", startup_path)
            zf.writestr("app/src/helper.py", "")
            zf.writestr("app/main.py", "VALUE = 42\n")

        startup_script = pyxel.cli._extract_pyxel_app(str(app_file))

        assert startup_script is not None
        assert Path(startup_script).name == "main.py"
        assert Path(startup_script).resolve().parent.name == "app"
        assert Path(startup_script).read_text(encoding="utf-8") == "VALUE = 42\n"


def _make_app(root: Path) -> Path:
    app_dir = root / "my_app"
    app_dir.mkdir()
    (app_dir / "main.py").write_text(
        "# title: My App\n# author: Me\nimport pyxel\n", encoding="utf-8"
    )
    (app_dir / "assets").mkdir()
    (app_dir / "assets" / "data.txt").write_text("hello", encoding="utf-8")
    return app_dir
