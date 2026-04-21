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


class TestApp2html:
    def test_creates_html(self, tmp_path, monkeypatch):
        _make_app(tmp_path)
        monkeypatch.chdir(tmp_path)
        pyxel.cli.package_pyxel_app("my_app", "my_app/main.py")
        pyxel.cli.create_html_from_pyxel_app(str(tmp_path / "my_app.pyxapp"))
        assert (tmp_path / "my_app.html").is_file()
