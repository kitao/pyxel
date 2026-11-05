import ast
import importlib.util
import os
import posixpath
import re
import runpy
import sys
import textwrap
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


def test_top_level_import_uses_sys_path_in_order(import_hook, monkeypatch, tmp_path):
    hook = import_hook.ImportHook()
    probes = []
    monkeypatch.setattr(import_hook.importlib.util, "find_spec", lambda name: None)
    first, second = str(tmp_path / "first"), str(tmp_path / "second")
    monkeypatch.setattr(sys, "path", [first, 123, first, second])

    def exists(path):
        probes.append(path)
        return path == os.path.join(second, "pkg.py")

    monkeypatch.setattr(os.path, "exists", exists)
    hook.find_spec("pkg", None)
    assert probes == [
        os.path.join(first, "pkg", "__init__.py"),
        os.path.join(first, "pkg.py"),
        os.path.join(second, "pkg", "__init__.py"),
        os.path.join(second, "pkg.py"),
    ]


@pytest.mark.parametrize("empty", [True, False])
def test_submodule_uses_package_path_even_when_empty(
    import_hook, monkeypatch, tmp_path, empty
):
    package = str(tmp_path / "package")
    search = [] if empty else [package]
    hook = import_hook.ImportHook()
    probes = []
    monkeypatch.setattr(import_hook.importlib.util, "find_spec", lambda name: None)
    monkeypatch.setattr(sys, "path", ["/not-the-package"])
    monkeypatch.setattr(os.path, "exists", lambda p: probes.append(p) or False)

    hook.find_spec("parent.child", search)
    assert probes == (
        [
            os.path.join(package, "child", "__init__.py"),
            os.path.join(package, "child.py"),
        ]
        if search
        else []
    )


def test_namespace_package_gets_a_chance_to_load_its_init(
    import_hook, monkeypatch, tmp_path
):
    hook = import_hook.ImportHook()
    probes = []
    monkeypatch.setattr(
        import_hook.importlib.util,
        "find_spec",
        lambda name: SimpleNamespace(origin=None, loader=None),
    )
    app = str(tmp_path / "app")
    monkeypatch.setattr(sys, "path", [app])
    monkeypatch.setattr(os.path, "exists", lambda p: probes.append(p) or True)
    hook.find_spec("pkg", None)
    assert probes == [os.path.join(app, "pkg", "__init__.py")]


def test_find_spec_suppresses_reentrant_lookup(import_hook, monkeypatch):
    hook = import_hook.ImportHook()
    calls = []

    def find_spec(name):
        calls.append(name)
        assert hook.find_spec(name, None) is None
        return SimpleNamespace(origin="built-in", loader=object())

    monkeypatch.setattr(import_hook.importlib.util, "find_spec", find_spec)
    assert hook.find_spec("recursive_pkg", None) is None
    assert calls == ["recursive_pkg"]


@pytest.mark.parametrize(
    "origin", ["built-in", "frozen", "/app/pkg.py", "/lib/site-packages/pkg.py"]
)
def test_resolved_modules_need_no_remote_lookup(import_hook, monkeypatch, origin):
    hook = import_hook.ImportHook()
    monkeypatch.setattr(
        import_hook.importlib.util,
        "find_spec",
        lambda name: SimpleNamespace(origin=origin, loader=object()),
    )
    monkeypatch.setattr(
        os.path, "exists", lambda p: pytest.fail("unexpected remote lookup")
    )
    hook.find_spec("pkg", None)


def test_runtime_reset_selects_only_modules_inside_application_directories():
    runtime_path = MODULE_PATH.with_name("pyxel.js")
    source = runtime_path.read_text(encoding="utf-8")
    block = next(
        block
        for block in re.findall(r"runPython\(`(.*?)`\)", source, re.DOTALL)
        if "mods = [" in block
    )

    selection = next(
        node.value
        for node in ast.walk(ast.parse(textwrap.dedent(block)))
        if isinstance(node, ast.Assign)
        and any(
            isinstance(target, ast.Name) and target.id == "mods"
            for target in node.targets
        )
    )

    modules = {
        "app": SimpleNamespace(__file__="/work/app.py"),
        "temp": SimpleNamespace(__file__="/tmp/package/__init__.py"),
        "work_sibling": SimpleNamespace(__file__="/work-other/app.py"),
        "temp_sibling": SimpleNamespace(__file__="/tmp-other/app.py"),
        "root": SimpleNamespace(__file__="/work"),
        "escaped": SimpleNamespace(__file__="/work/../outside/app.py"),
        "relative": SimpleNamespace(__file__="package/helper.py"),
        "builtin": SimpleNamespace(),
        "empty": SimpleNamespace(__file__=""),
    }
    # Evaluate only module selection, using Pyodide's POSIX paths on every host.
    namespace = {
        "sys": SimpleNamespace(modules=modules),
        "os": SimpleNamespace(
            path=SimpleNamespace(
                abspath=lambda path: posixpath.normpath(posixpath.join("/work", path))
            )
        ),
        "work_dir": "/work",
        "temp_dir": "/tmp",
    }

    selected = eval(
        compile(ast.Expression(selection), str(runtime_path), "eval"), namespace
    )
    assert selected == ["app", "temp", "relative", "__main__"]


@pytest.mark.parametrize(
    "statement",
    [
        "import {name} as plugin",
        "from importlib import import_module\nplugin = import_module('{name}')",
    ],
)
def test_cache_invalidation_redownloads_removed_application_modules(
    import_hook, monkeypatch, tmp_path, statement
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
        statement.format(name=module_name) + "\nvalue = plugin.value",
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


def test_invalidation_uses_the_new_search_directory(import_hook, monkeypatch, tmp_path):
    hook = import_hook.ImportHook()
    probes = []
    monkeypatch.setattr(import_hook.importlib.util, "find_spec", lambda name: None)
    monkeypatch.setattr(sys, "path", [str(tmp_path / "old")])

    def exists(path):
        probes.append(path)
        return path.endswith("helper.py")

    monkeypatch.setattr(os.path, "exists", exists)
    hook.find_spec("helper", None)
    monkeypatch.setattr(sys, "path", [str(tmp_path / "new")])
    hook.invalidate_caches()
    probes.clear()
    hook.find_spec("helper", None)
    assert probes == [
        str(tmp_path / "new" / "helper" / "__init__.py"),
        str(tmp_path / "new" / "helper.py"),
    ]


def test_relative_entry_imports_after_changing_its_working_directory(
    import_hook, monkeypatch, tmp_path
):
    app = tmp_path / "game"
    app.mkdir()
    name = "pyxel_relative_remote_test"
    module = app / (name + ".py")
    source = b"\xef\xbb\xbf" + "# 日本語\nvalue = 42\n".encode()
    (app / "main.py").write_text(
        "import os, importlib\nos.chdir('game')\n"
        f"result = importlib.import_module('{name}').value\n"
    )
    monkeypatch.chdir(tmp_path)
    monkeypatch.syspath_prepend(str(app))
    hook = import_hook.ImportHook()
    monkeypatch.setattr(sys, "meta_path", [hook, *sys.meta_path])
    original_exists = os.path.exists

    def exists(path):
        if path == str(module) and not original_exists(path):
            module.write_bytes(source)
        return original_exists(path)

    monkeypatch.setattr(os.path, "exists", exists)
    try:
        result = runpy.run_path("game/main.py")
        assert result["result"] == 42
        assert result["__file__"] == "game/main.py"
        assert module.read_bytes() == source
    finally:
        sys.modules.pop(name, None)
