import importlib.util
import os
import runpy
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


def test_find_spec_suppresses_reentrant_lookup(import_hook, monkeypatch):
    hook = import_hook.ImportHook()
    calls = []

    def find_spec(name):
        calls.append(name)
        assert hook.find_spec(name, None) is None
        return SimpleNamespace(origin="built-in")

    monkeypatch.setattr(import_hook.importlib.util, "find_spec", find_spec)

    assert hook.find_spec("recursive_pkg", None) is None
    assert calls == ["recursive_pkg"]


def test_cached_main_dir_is_probed_from_another_caller(
    import_hook, monkeypatch, tmp_path
):
    hook = import_hook.ImportHook()
    probes = []
    main = tmp_path / "main"
    other = tmp_path / "other"
    caller = str(main / "app.py")
    monkeypatch.setattr(import_hook.importlib.util, "find_spec", lambda name: None)
    monkeypatch.setattr(import_hook.sys, "_getframe", lambda depth: _frame(caller))

    def exists(path):
        probes.append(path)
        return path == str(main / "first.py")

    monkeypatch.setattr(import_hook.os.path, "exists", exists)
    hook.find_spec("first", None)
    caller = str(other / "plugin.py")
    probes.clear()

    hook.find_spec("nested.module", None)

    assert hook.main_dir == str(main)
    assert probes == [
        str(other / "nested" / "module.py"),
        str(other / "nested" / "module" / "__init__.py"),
        str(main / "nested" / "module.py"),
        str(main / "nested" / "module" / "__init__.py"),
    ]


def test_cache_invalidation_redownloads_removed_application_modules(
    import_hook, monkeypatch, tmp_path
):
    hook = import_hook.ImportHook()
    module_name = "pyxel_reset_remote_test"
    module_path = tmp_path / f"{module_name}.py"
    original_exists = os.path.exists
    downloads = []

    def hosted_exists(path):
        if path == str(module_path) and not original_exists(path):
            downloads.append(path)
            module_path.write_text(f"value = {len(downloads)}\n", encoding="utf-8")
        return original_exists(path)

    monkeypatch.syspath_prepend(str(tmp_path))
    monkeypatch.setattr(sys, "meta_path", [hook, *sys.meta_path])
    monkeypatch.setattr(os.path, "exists", hosted_exists)
    # Avoid bytecode retaining the first downloaded module after reset.
    monkeypatch.setattr(sys, "dont_write_bytecode", True)
    script_path = tmp_path / "main.py"
    script_path.write_text(
        f"import {module_name}\nvalue = {module_name}.value",
        encoding="utf-8",
    )

    try:
        namespace = runpy.run_path(str(script_path))
        assert namespace["value"] == 1

        # Match resetPyxel: remove modules, invalidate caches, then remove files.
        sys.modules.pop(module_name)
        import_hook.importlib.invalidate_caches()
        module_path.unlink()

        namespace = runpy.run_path(str(script_path))
        assert namespace["value"] == 2
        assert downloads == [str(module_path), str(module_path)]
    finally:
        sys.modules.pop(module_name, None)


def test_cache_invalidation_forgets_the_previous_main_directory(
    import_hook, monkeypatch, tmp_path
):
    hook = import_hook.ImportHook()
    old_main = tmp_path / "old" / "main.py"
    new_main = tmp_path / "new" / "main.py"
    caller = old_main
    probes = []
    monkeypatch.setattr(import_hook.importlib.util, "find_spec", lambda name: None)
    monkeypatch.setattr(import_hook.sys, "_getframe", lambda depth: _frame(str(caller)))
    monkeypatch.setattr(sys, "meta_path", [hook, *sys.meta_path])

    def exists(path):
        probes.append(path)
        return os.fspath(path).endswith("helper.py")

    monkeypatch.setattr(import_hook.os.path, "exists", exists)
    hook.find_spec("helper", None)
    assert hook.main_dir == str(old_main.parent)

    import_hook.importlib.invalidate_caches()
    caller = new_main
    probes.clear()
    hook.find_spec("helper", None)

    assert hook.main_dir == str(new_main.parent)
    assert probes == [str(new_main.parent / "helper.py")]


def _frame(filename: str):
    return SimpleNamespace(
        f_code=SimpleNamespace(co_filename=filename),
        f_back=None,
    )
