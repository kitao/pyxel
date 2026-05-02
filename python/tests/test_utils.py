import pyxel.utils


class TestListImportedModules:
    def test_returns_dict_with_system_and_local_keys(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import os\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert "system" in result
        assert "local" in result

    def test_extracts_system_imports(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import os\nimport sys\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert "os" in result["system"]
        assert "sys" in result["system"]

    def test_extracts_from_imports(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("from pathlib import Path\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert "pathlib" in result["system"]

    def test_extracts_local_imports(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import helper\n", encoding="utf-8")
        helper = tmp_path / "helper.py"
        helper.write_text("import json\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert any("helper.py" in p for p in result["local"])
        # json from nested helper should be tracked as system
        assert "json" in result["system"]

    def test_handles_relative_imports(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("from . import helper\n", encoding="utf-8")
        helper = tmp_path / "helper.py"
        helper.write_text("", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        assert any("helper.py" in p for p in result["local"])

    def test_handles_syntax_error_gracefully(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import os\nimport (((bad syntax\n", encoding="utf-8")
        # Syntax error: should not raise, just return empty/partial result
        result = pyxel.utils.list_imported_modules(str(script))
        assert "system" in result
        assert "local" in result

    def test_returns_sorted_lists(self, tmp_path):
        script = tmp_path / "main.py"
        script.write_text("import zlib\nimport abc\nimport sys\n", encoding="utf-8")
        result = pyxel.utils.list_imported_modules(str(script))
        system_list = result["system"]
        assert system_list == sorted(system_list)
        local_list = result["local"]
        assert local_list == sorted(local_list)
