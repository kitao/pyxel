import sys
from pathlib import Path

import pytest
import pyxel.utils


class TestListImportedModules:
    def test_returns_dict_with_system_and_local_keys(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import os\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {"system": ["os"], "local": []}

    def test_extracts_system_imports(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import os\nimport sys\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {"system": ["os", "sys"], "local": []}

    @pytest.mark.parametrize(
        "source",
        [
            "from xml.etree import ElementTree\n",
            "from xml.etree import ElementTree as ET\n",
        ],
    )
    def test_extracts_system_submodule_from_import(self, tmp_path, source):
        script = tmp_path / "main.py"
        script.write_text(source, encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert result["system"] == ["xml.etree", "xml.etree.ElementTree"]

    def test_system_submodule_discovery_does_not_import_package(
        self, tmp_path, monkeypatch
    ):
        module_root = tmp_path / "modules"
        package = module_root / "probe_package"
        package.mkdir(parents=True)
        (package / "__init__.py").write_text(
            "raise RuntimeError('package was imported')\n", encoding="utf-8"
        )
        (package / "child.py").write_text("VALUE = 1\n", encoding="utf-8")

        app_dir = tmp_path / "app"
        app_dir.mkdir()
        script = app_dir / "main.py"
        script.write_text("from probe_package import child\n", encoding="utf-8")
        monkeypatch.syspath_prepend(str(module_root))

        result = pyxel.utils.list_imported_modules(str(script))
        assert result["system"] == ["probe_package", "probe_package.child"]
        assert "probe_package" not in sys.modules
        assert "probe_package.child" not in sys.modules

    def test_extracts_local_imports(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import helper\n", encoding="utf-8")
        helper = tmp_path / "helper.py"
        helper.write_text("import json\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {"system": ["json"], "local": [str(helper)]}

    def test_extracts_local_submodule_imports(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("from pkg import helper\n", encoding="utf-8")
        pkg = tmp_path / "pkg"
        pkg.mkdir()
        (pkg / "__init__.py").write_text("", encoding="utf-8")
        (pkg / "helper.py").write_text("import zlib\n", encoding="utf-8")

        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {
            "system": ["zlib"],
            "local": [str(pkg / "__init__.py"), str(pkg / "helper.py")],
        }

    @pytest.mark.parametrize("source", ["import helper", "from helper import VALUE"])
    def test_nested_absolute_import_uses_startup_directory(self, tmp_path, source):
        script = tmp_path / "main.py"
        script.write_text("import pkg.module\n", encoding="utf-8")
        helper = tmp_path / "helper.py"
        helper.write_text("import decimal\nVALUE = 1\n", encoding="utf-8")
        pkg = tmp_path / "pkg"
        pkg.mkdir()
        (pkg / "__init__.py").write_text("", encoding="utf-8")
        (pkg / "helper.py").write_text("import fractions\n", encoding="utf-8")
        (pkg / "module.py").write_text(
            f"{source}\nfrom . import helper as relative_helper\n", encoding="utf-8"
        )

        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {
            "system": ["decimal", "fractions"],
            "local": sorted(
                str(path)
                for path in (
                    helper,
                    pkg / "__init__.py",
                    pkg / "helper.py",
                    pkg / "module.py",
                )
            ),
        }

    @pytest.mark.parametrize(
        "source",
        [
            "import pkg.sub.leaf\n",
            "from pkg.sub import leaf\n",
            "from pkg.sub.leaf import VALUE\n",
            "from .pkg.sub import leaf\n",
        ],
    )
    @pytest.mark.parametrize("leaf_is_package", [False, True])
    def test_extracts_parent_package_imports(self, tmp_path, source, leaf_is_package):
        script = tmp_path / "main.py"
        script.write_text(source, encoding="utf-8")
        pkg = tmp_path / "pkg"
        sub = pkg / "sub"
        sub.mkdir(parents=True)
        (pkg / "__init__.py").write_text(
            "import sqlite3\nraise RuntimeError('package was imported')\n",
            encoding="utf-8",
        )
        (sub / "__init__.py").write_text("import fractions\n", encoding="utf-8")

        if leaf_is_package:
            (sub / "leaf").mkdir()
            leaf = sub / "leaf" / "__init__.py"
        else:
            leaf = sub / "leaf.py"
        leaf.write_text("import zlib\nVALUE = 1\n", encoding="utf-8")

        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {
            "system": ["fractions", "sqlite3", "zlib"],
            "local": sorted(
                str(path) for path in (pkg / "__init__.py", sub / "__init__.py", leaf)
            ),
        }

    def test_extracts_package_imports_under_namespace(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import namespace.pkg.leaf\n", encoding="utf-8")
        pkg = tmp_path / "namespace" / "pkg"
        pkg.mkdir(parents=True)
        init_file = pkg / "__init__.py"
        init_file.write_text("import sqlite3\n", encoding="utf-8")
        leaf = pkg / "leaf.py"
        leaf.write_text("import zlib\n", encoding="utf-8")

        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {
            "system": ["sqlite3", "zlib"],
            "local": [str(init_file), str(leaf)],
        }

    @pytest.mark.parametrize("source", ["import pkg\n", "import pkg.sub\n"])
    def test_reads_each_file_once_in_relative_cycle(
        self, tmp_path, monkeypatch, source
    ):
        script = tmp_path / "main.py"
        script.write_text(source, encoding="utf-8")
        pkg = tmp_path / "pkg"
        sub = pkg / "sub"
        sub.mkdir(parents=True)
        (pkg / "__init__.py").write_text("from . import sub\n", encoding="utf-8")
        (sub / "__init__.py").write_text(
            "from .. import sub\nimport zlib\n", encoding="utf-8"
        )
        read_files = []
        read_text = Path.read_text

        def record_read(path, *args, **kwargs):
            read_files.append(path.resolve())
            return read_text(path, *args, **kwargs)

        monkeypatch.setattr(Path, "read_text", record_read)

        result = pyxel.utils.list_imported_modules(str(script))
        assert read_files == [script, pkg / "__init__.py", sub / "__init__.py"]
        assert result == {
            "system": ["zlib"],
            "local": sorted(
                str(path)
                for path in (
                    pkg / "__init__.py",
                    sub / "__init__.py",
                    sub / ".." / "sub" / "__init__.py",
                )
            ),
        }

    def test_preserves_relative_import_path_spelling(self, tmp_path, monkeypatch):
        pkg = tmp_path / "pkg"
        pkg.mkdir()
        script = pkg / "main.py"
        script.write_text("from .. import helper\n", encoding="utf-8")
        (tmp_path / "helper.py").write_text("import zlib\n", encoding="utf-8")
        monkeypatch.chdir(tmp_path)

        result = pyxel.utils.list_imported_modules("pkg/main.py")
        assert result == {
            "system": ["zlib"],
            "local": [str(pkg / ".." / "helper.py")],
        }

    @pytest.mark.skipif(sys.platform == "win32", reason="symlink may require elevation")
    def test_resolves_imports_relative_to_symlink_path(self, tmp_path, monkeypatch):
        real = tmp_path / "real"
        alias = tmp_path / "alias"
        real.mkdir()
        alias.mkdir()
        script = real / "main.py"
        script.write_text("from . import helper\n", encoding="utf-8")
        (real / "helper.py").write_text("import sqlite3\n", encoding="utf-8")
        (alias / "helper.py").write_text("import zlib\n", encoding="utf-8")
        (alias / "main.py").symlink_to(script)
        monkeypatch.chdir(tmp_path)

        result = pyxel.utils.list_imported_modules("alias/main.py")
        assert result == {
            "system": ["zlib"],
            "local": [str(alias / "helper.py")],
        }

    @pytest.mark.skipif(sys.platform == "win32", reason="symlink may require elevation")
    def test_stops_symlink_cycle_and_preserves_alias(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import pkg\n", encoding="utf-8")
        pkg = tmp_path / "pkg"
        pkg.mkdir()
        (pkg / "__init__.py").write_text("from . import loop\n", encoding="utf-8")
        (pkg / "loop").symlink_to(pkg, target_is_directory=True)

        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {
            "system": [],
            "local": [str(pkg / "__init__.py"), str(pkg / "loop" / "__init__.py")],
        }

    def test_from_import_attribute_is_not_a_system_module(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("from pathlib import Path\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert result["system"] == ["pathlib"]

    def test_handles_relative_imports(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("from . import helper\n", encoding="utf-8")
        helper = tmp_path / "helper.py"
        helper.write_text("", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {"system": [], "local": [str(helper)]}

    def test_handles_syntax_error_gracefully(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import os\nimport (((bad syntax\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {"system": [], "local": []}

    def test_returns_sorted_lists(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text(
            "import zlib\nimport zebra\nimport abc\nimport alpha\nimport sys\n",
            encoding="utf-8",
        )
        alpha = tmp_path / "alpha.py"
        zebra = tmp_path / "zebra.py"
        alpha.write_text("", encoding="utf-8")
        zebra.write_text("", encoding="utf-8")

        result = pyxel.utils.list_imported_modules(str(script))
        assert result == {
            "system": ["abc", "sys", "zlib"],
            "local": [str(alpha), str(zebra)],
        }
