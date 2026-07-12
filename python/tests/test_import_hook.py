import importlib.util
import os
import sys
from importlib.machinery import SourceFileLoader
from pathlib import Path
from types import SimpleNamespace

import pytest


MODULE_PATH = Path(__file__).parents[2] / "wasm" / "import_hook.py"


@pytest.fixture
def import_hook():
    original_meta_path = sys.meta_path[:]
    loader = SourceFileLoader("import_hook_test", str(MODULE_PATH))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    assert spec is not None
    module = importlib.util.module_from_spec(spec)
    try:
        loader.exec_module(module)
    finally:
        sys.meta_path[:] = original_meta_path
    return module


def _frame(filename: str):
    return SimpleNamespace(
        f_code=SimpleNamespace(co_filename=filename),
        f_back=None,
    )


@pytest.mark.parametrize("module_name", ["javascript", "js"])
def test_pyodide_pseudo_module_is_skipped(import_hook, monkeypatch, module_name):
    hook = import_hook.ImportHook()

    def unexpected_find_spec(_fullname):
        pytest.fail("skipped Pyodide module reached standard module lookup")

    monkeypatch.setattr(import_hook.importlib.util, "find_spec", unexpected_find_spec)

    assert hook.find_spec(module_name, None) is None


def test_bare_cwd_collision_does_not_set_main_dir(import_hook, monkeypatch):
    hook = import_hook.ImportHook()
    probes = []
    monkeypatch.setattr(import_hook.importlib.util, "find_spec", lambda name: None)
    monkeypatch.setattr(
        import_hook.sys, "_getframe", lambda depth: _frame("/virtual/caller/main.py")
    )

    def exists(path):
        probes.append(path)
        return path == "pkg"

    monkeypatch.setattr(import_hook.os.path, "exists", exists)
    hook.find_spec("pkg", None)

    assert probes == [
        os.path.join("/virtual/caller", "pkg.py"),
        os.path.join("/virtual/caller", "pkg", "__init__.py"),
    ]
    assert hook.main_dir is None


def test_caller_relative_module_sets_main_dir(import_hook, monkeypatch, tmp_path):
    caller = tmp_path / "caller"
    caller.mkdir()
    (caller / "main.py").write_text("", encoding="utf-8")
    (caller / "pkg.py").write_text("", encoding="utf-8")
    hook = import_hook.ImportHook()
    monkeypatch.setattr(import_hook.importlib.util, "find_spec", lambda name: None)
    monkeypatch.setattr(
        import_hook.sys,
        "_getframe",
        lambda depth: _frame(str(caller / "main.py")),
    )

    hook.find_spec("pkg", None)

    assert hook.main_dir == str(caller.resolve())
