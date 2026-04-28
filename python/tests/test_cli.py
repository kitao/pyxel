import base64
import io
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import zipfile
from pathlib import Path

import pytest

import pyxel
import pyxel.cli


def _make_app(root: Path) -> Path:
    app_dir = root / "my_app"
    app_dir.mkdir()
    (app_dir / "main.py").write_text(
        "# title: My App\n# author: Me\nimport pyxel\n", encoding="utf-8"
    )
    (app_dir / "assets").mkdir()
    (app_dir / "assets" / "data.txt").write_text("hello", encoding="utf-8")
    return app_dir


# Public commands


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
        with pytest.raises(SystemExit):
            pyxel.cli.run_python_script(str(missing))
        assert "no such file:" in capsys.readouterr().out

    def test_non_py_file_rejected(self, capsys):
        with pytest.raises(SystemExit):
            pyxel.cli.run_python_script("foo.txt")
        assert "'run' command only accepts .py files" in capsys.readouterr().out


class TestWatchCommand:
    def test_missing_dir_exits_with_error(self, capsys, tmp_path):
        missing_dir = tmp_path / "nodir"
        script = tmp_path / "script.py"
        script.write_text("", encoding="utf-8")
        with pytest.raises(SystemExit):
            pyxel.cli.watch_and_run_python_script(str(missing_dir), str(script))
        assert "no such directory:" in capsys.readouterr().out

    def test_missing_script_exits_with_error(self, capsys, tmp_path):
        watch_dir = tmp_path / "dir"
        watch_dir.mkdir()
        missing = tmp_path / "nope.py"
        with pytest.raises(SystemExit):
            pyxel.cli.watch_and_run_python_script(str(watch_dir), str(missing))
        assert "no such file:" in capsys.readouterr().out

    def test_script_outside_dir_rejected(self, capsys, tmp_path):
        watch_dir = tmp_path / "dir"
        watch_dir.mkdir()
        outside = tmp_path / "outside.py"
        outside.write_text("", encoding="utf-8")
        with pytest.raises(SystemExit):
            pyxel.cli.watch_and_run_python_script(str(watch_dir), str(outside))
        assert "specified file is not under the directory" in capsys.readouterr().out

    def test_non_py_script_rejected(self, capsys, tmp_path):
        watch_dir = tmp_path / "dir"
        watch_dir.mkdir()
        with pytest.raises(SystemExit):
            pyxel.cli.watch_and_run_python_script(str(watch_dir), "foo.txt")
        assert "'watch' command only accepts .py files" in capsys.readouterr().out


class TestPyxelAppMetadata:
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
    def test_invokes_editor_app(self, monkeypatch, tmp_path):
        invocations = []

        class FakeApp:
            def __init__(self, resource_file, starting_editor):
                invocations.append((resource_file, starting_editor))

        monkeypatch.setattr("pyxel.editor.App", FakeApp)

        resource_file = str(tmp_path / "my_resource.pyxres")
        pyxel.cli.edit_pyxel_resource(resource_file)

        assert len(invocations) == 1
        assert invocations[0][0] == resource_file
        assert invocations[0][1] == "image"

    def test_uses_default_resource_when_omitted(self, monkeypatch):
        invocations = []

        class FakeApp:
            def __init__(self, resource_file, starting_editor):
                invocations.append((resource_file, starting_editor))

        monkeypatch.setattr("pyxel.editor.App", FakeApp)

        pyxel.cli.edit_pyxel_resource()

        assert len(invocations) == 1
        assert invocations[0][0].endswith(pyxel.RESOURCE_FILE_EXTENSION)

    def test_with_custom_starting_editor(self, monkeypatch, tmp_path):
        invocations = []

        class FakeApp:
            def __init__(self, resource_file, starting_editor):
                invocations.append((resource_file, starting_editor))

        monkeypatch.setattr("pyxel.editor.App", FakeApp)

        resource_file = str(tmp_path / "my_resource.pyxres")
        pyxel.cli.edit_pyxel_resource(resource_file, "music")

        assert invocations[0][1] == "music"


class TestPlayCommand:
    def test_missing_file_exits_with_error(self, capsys, tmp_path):
        missing = tmp_path / "nope.pyxapp"
        with pytest.raises(SystemExit):
            pyxel.cli.play_pyxel_app(str(missing))
        assert "no such file:" in capsys.readouterr().out

    def test_non_pyxapp_extension_rejected(self, capsys):
        with pytest.raises(SystemExit):
            pyxel.cli.play_pyxel_app("foo.txt")
        assert (
            f"'play' command only accepts {pyxel.APP_FILE_EXTENSION} files"
            in capsys.readouterr().out
        )


class TestPackage:
    def test_with_relative_paths_from_parent(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        assert (tmp_path / "my_app.pyxapp").is_file()

    def test_with_relative_paths_from_app_dir(self, tmp_path, monkeypatch):
        # Regression for `pyxel package . main.py` run from inside app_dir
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

    def test_pyxapp_has_no_duplicate_entries(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            names = zf.namelist()
        assert len(names) == len(set(names)), f"duplicate entries in pyxapp: {names}"

    def test_startup_script_pointer_is_relative(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        with zipfile.ZipFile(tmp_path / "my_app.pyxapp") as zf:
            pointer = zf.read(f"my_app/{pyxel.APP_STARTUP_SCRIPT_FILE}").decode("utf-8")
        assert pointer == "main.py"

    def test_metadata_embedded_in_pyxapp(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        metadata = pyxel.cli.get_pyxel_app_metadata(str(tmp_path / "my_app.pyxapp"))
        assert metadata["title"] == "My App"
        assert metadata["author"] == "Me"

    def test_rejects_non_py_startup_script(self, tmp_path, monkeypatch):
        app_dir = _make_app(tmp_path)
        (app_dir / "main.txt").write_text("", encoding="utf-8")
        monkeypatch.chdir(tmp_path)
        with pytest.raises(SystemExit):
            pyxel.cli.package_pyxel_app("my_app", "my_app/main.txt")

    def test_rejects_startup_script_outside_app_dir(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        outside = tmp_path / "outside.py"
        outside.write_text("", encoding="utf-8")
        monkeypatch.chdir(tmp_path)
        with pytest.raises(SystemExit):
            pyxel.cli.package_pyxel_app("my_app", str(outside))

    def test_stdout_shows_added_files(self, capsys, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        out = capsys.readouterr().out
        assert "added 'my_app/main.py'" in out
        assert "added 'my_app/assets/data.txt'" in out
        assert f"added 'my_app/{pyxel.APP_STARTUP_SCRIPT_FILE}'" in out


class TestApp2exe:
    def test_exe_runs_with_resource(self, tmp_path, monkeypatch):
        pytest.importorskip("PyInstaller")
        app_dir = tmp_path / "my_app"
        (app_dir / "assets").mkdir(parents=True)
        shutil.copy(
            Path(pyxel.__file__).parent / "examples" / "assets" / "sample.pyxres",
            app_dir / "assets" / "sample.pyxres",
        )
        (app_dir / "main.py").write_text(
            "import pyxel\n"
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
        result = subprocess.run([str(exe)], capture_output=True, timeout=60)
        assert result.returncode == 0, (
            f"exe failed rc={result.returncode}\n"
            f"stdout={result.stdout.decode(errors='replace')}\n"
            f"stderr={result.stderr.decode(errors='replace')}"
        )

    def test_missing_pyxapp_exits_with_error(self, capsys, tmp_path):
        with pytest.raises(SystemExit):
            pyxel.cli.create_executable_from_pyxel_app(str(tmp_path / "nope.pyxapp"))
        assert "no such file:" in capsys.readouterr().out

    def test_non_pyxapp_extension_rejected(self, capsys):
        with pytest.raises(SystemExit):
            pyxel.cli.create_executable_from_pyxel_app("foo.txt")
        assert (
            f"'app2exe' command only accepts {pyxel.APP_FILE_EXTENSION} files"
            in capsys.readouterr().out
        )


class TestApp2html:
    def test_creates_html(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        pyxel.cli.create_html_from_pyxel_app(str(tmp_path / "my_app.pyxapp"))
        assert (tmp_path / "my_app.html").is_file()

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

        with zipfile.ZipFile(io.BytesIO(payload)) as zf:
            names = set(zf.namelist())
        assert "my_app/main.py" in names
        assert "my_app/assets/data.txt" in names
        assert f"my_app/{pyxel.APP_STARTUP_SCRIPT_FILE}" in names

    def test_missing_pyxapp_exits_with_error(self, capsys, tmp_path):
        with pytest.raises(SystemExit):
            pyxel.cli.create_html_from_pyxel_app(str(tmp_path / "nope.pyxapp"))
        assert "no such file:" in capsys.readouterr().out

    def test_non_pyxapp_extension_rejected(self, capsys):
        with pytest.raises(SystemExit):
            pyxel.cli.create_html_from_pyxel_app("foo.txt")
        assert (
            f"'app2html' command only accepts {pyxel.APP_FILE_EXTENSION} files"
            in capsys.readouterr().out
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
        monkeypatch.chdir(tmp_path)
        pyxel.cli.copy_pyxel_examples()
        dst = tmp_path / "pyxel_examples"
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


# Private helpers


class TestErrorHelpers:
    def test_exit_with_error_prints_message_and_exits(self, capsys):
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli._exit_with_error("boom")
        assert exc_info.value.code == 1
        assert "boom" in capsys.readouterr().out

    def test_check_file_exists_missing(self, capsys, tmp_path):
        missing = tmp_path / "nope.py"
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli._check_file_exists(str(missing))
        assert exc_info.value.code == 1
        assert f"no such file: '{missing}'" in capsys.readouterr().out

    def test_check_dir_exists_missing(self, capsys, tmp_path):
        missing = tmp_path / "nodir"
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli._check_dir_exists(str(missing))
        assert exc_info.value.code == 1
        assert f"no such directory: '{missing}'" in capsys.readouterr().out

    def test_check_file_under_dir_outside(self, capsys, tmp_path):
        inside_dir = tmp_path / "dir"
        inside_dir.mkdir()
        outside = tmp_path / "outside.py"
        outside.write_text("", encoding="utf-8")
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli._check_file_under_dir(str(outside), str(inside_dir))
        assert exc_info.value.code == 1
        assert "specified file is not under the directory" in capsys.readouterr().out

    def test_complete_extension_rejects_wrong_ext(self, capsys):
        with pytest.raises(SystemExit) as exc_info:
            pyxel.cli._complete_extension("foo.txt", "run", ".py")
        assert exc_info.value.code == 1
        assert "'run' command only accepts .py files" in capsys.readouterr().out

    def test_complete_extension_appends_missing_ext(self):
        assert pyxel.cli._complete_extension("foo", "run", ".py") == "foo.py"


class TestWatchHelpers:
    def test_timestamps_in_dir_lists_nested_files(self, tmp_path):
        (tmp_path / "a.py").write_text("a", encoding="utf-8")
        sub = tmp_path / "sub"
        sub.mkdir()
        (sub / "b.py").write_text("b", encoding="utf-8")
        result = pyxel.cli._timestamps_in_dir(str(tmp_path))
        assert set(result.keys()) == {
            str(tmp_path / "a.py"),
            str(sub / "b.py"),
        }

    def test_timestamps_in_dir_detects_modification(self, tmp_path):
        f = tmp_path / "a.py"
        f.write_text("a", encoding="utf-8")
        before = pyxel.cli._timestamps_in_dir(str(tmp_path))
        time.sleep(0.01)
        f.write_text("b", encoding="utf-8")
        after = pyxel.cli._timestamps_in_dir(str(tmp_path))
        assert before != after

    def test_create_watch_state_file_touches_pid_file(self, tmp_path, monkeypatch):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        state_file = pyxel.cli._create_watch_state_file()
        assert Path(state_file).is_file()
        assert Path(state_file).name == str(os.getpid())

    def test_create_watch_state_file_cleans_dead_process_files(
        self, tmp_path, monkeypatch
    ):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        watch_dir = tmp_path / pyxel.BASE_DIR / "watch"
        watch_dir.mkdir(parents=True)
        # Use a PID that almost certainly doesn't exist
        dead_file = watch_dir / "999999"
        dead_file.touch()
        pyxel.cli._create_watch_state_file()
        assert not dead_file.exists()


class TestExtractPyxelAppSafety:
    @staticmethod
    def _build_zip_with_entry(zip_path, entry_name):
        with zipfile.ZipFile(zip_path, "w") as zf:
            zf.writestr(entry_name, b"malicious content")

    def test_rejects_path_traversal_entry(self, capsys, tmp_path, monkeypatch):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        zip_path = tmp_path / "evil.pyxapp"
        self._build_zip_with_entry(zip_path, "../evil.txt")
        with pytest.raises(SystemExit):
            pyxel.cli._extract_pyxel_app(str(zip_path))
        assert "unsafe path in Pyxel app:" in capsys.readouterr().out

    def test_rejects_absolute_path_entry(self, capsys, tmp_path, monkeypatch):
        monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
        zip_path = tmp_path / "evil.pyxapp"
        self._build_zip_with_entry(zip_path, "/etc/passwd")
        with pytest.raises(SystemExit):
            pyxel.cli._extract_pyxel_app(str(zip_path))
        assert "unsafe path in Pyxel app:" in capsys.readouterr().out
