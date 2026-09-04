import importlib.util
import json
import shutil
import subprocess
import zipfile
from importlib.machinery import SourceFileLoader
from pathlib import Path

import pytest
import tomllib

ROOT_DIR = Path(__file__).parents[2]
EXPECTED_WHEEL_TAG = "cp311-abi3-pyemscripten_2026_0_wasm32"


def _load_script(name):
    loader = SourceFileLoader(f"{name}_test", str(ROOT_DIR / "scripts" / name))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    assert spec is not None
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


def _write_project_version(root: Path, version: str) -> None:
    pyproject_path = root / "python" / "pyproject.toml"
    pyproject_path.parent.mkdir(parents=True, exist_ok=True)
    pyproject_path.write_text(
        f'[project]\nname = "pyxel"\nversion = "{version}"\n'
        'requires-python = ">=3.11"\n',
        encoding="utf-8",
    )


def _write_wasm_wheel(path: Path, version: str, tag: str = EXPECTED_WHEEL_TAG) -> None:
    with zipfile.ZipFile(path, "w") as wheel_zip:
        wheel_zip.writestr(
            f"pyxel-{version}.dist-info/WHEEL",
            f"Wheel-Version: 1.0\nRoot-Is-Purelib: false\nTag: {tag}\n",
        )


def test_wasm_wheel_check_rejects_generated_metadata_and_host_paths(tmp_path, capsys):
    checker = _load_script("check_wasm_wheel")
    wheel_path = tmp_path / f"pyxel-3.0.0-{EXPECTED_WHEEL_TAG}.whl"
    with zipfile.ZipFile(wheel_path, "w") as wheel_zip:
        wheel_zip.writestr("pyxel/__pycache__/module.cpython-314.pyc", b"bytecode")
        wheel_zip.writestr("pyxel/.ruff_cache/CACHEDIR.TAG", b"cache")
        wheel_zip.writestr("pyxel/module.pyo", b"optimized bytecode")
        wheel_zip.writestr("pyxel/examples/.DS_Store", b"metadata")
        wheel_zip.writestr("pyxel/pyxel_binding.abi3.so", b"/Users/example/src")

    assert checker.find_violations(wheel_path) == [
        ("pyxel/__pycache__/module.cpython-314.pyc", "generated Python bytecode"),
        ("pyxel/.ruff_cache/CACHEDIR.TAG", "generated Ruff cache"),
        ("pyxel/module.pyo", "generated Python bytecode"),
        ("pyxel/examples/.DS_Store", "platform metadata file"),
        ("pyxel/pyxel_binding.abi3.so", "host path b'/Users/'"),
    ]
    checker.DIST_DIR = tmp_path
    _write_project_version(tmp_path, "3.0.0")
    checker.PYPROJECT_PATH = tmp_path / "python" / "pyproject.toml"
    assert checker.main() == 1
    output = capsys.readouterr().out
    assert "error: invalid contents detected" in output


def test_wasm_wheel_check_detects_stale_packaged_sources(tmp_path):
    checker = _load_script("check_wasm_wheel")
    source_dir = tmp_path / "pyxel"
    source_dir.mkdir()
    (source_dir / "cli.py").write_text("current\n", encoding="utf-8")
    wheel_path = tmp_path / "pyxel-test-pyemscripten_2026_0_wasm32.whl"
    with zipfile.ZipFile(wheel_path, "w") as wheel_zip:
        wheel_zip.writestr("pyxel/cli.py", "stale\n")

    assert checker.find_source_mismatches(wheel_path, source_dir) == [
        ("pyxel/cli.py", "differs from python/pyxel/cli.py")
    ]


def test_wasm_wheel_check_detects_removed_packaged_sources(tmp_path):
    checker = _load_script("check_wasm_wheel")
    source_dir = tmp_path / "pyxel"
    source_dir.mkdir()
    (source_dir / "cli.py").write_text("current\n", encoding="utf-8")
    wheel_path = tmp_path / "pyxel-test-pyemscripten_2026_0_wasm32.whl"
    with zipfile.ZipFile(wheel_path, "w") as wheel_zip:
        wheel_zip.writestr("pyxel/cli.py", "current\n")
        wheel_zip.writestr("pyxel/removed.py", "stale\n")

    assert checker.find_source_mismatches(wheel_path, source_dir) == [
        ("pyxel/removed.py", "not present in python/pyxel")
    ]


def test_wasm_wheel_check_detects_stale_project_metadata(tmp_path):
    checker = _load_script("check_wasm_wheel")
    pyproject_path = tmp_path / "pyproject.toml"
    pyproject_path.write_text(
        '[project]\nname = "pyxel"\nversion = "3.0.0"\n'
        'requires-python = ">=3.11"\n'
        '[project.optional-dependencies]\napp2exe = ["pyinstaller>=6.22,<7"]\n',
        encoding="utf-8",
    )
    wheel_path = tmp_path / "pyxel-test-pyemscripten_2026_0_wasm32.whl"
    with zipfile.ZipFile(wheel_path, "w") as wheel_zip:
        wheel_zip.writestr(
            "pyxel-3.0.0.dist-info/METADATA",
            "Metadata-Version: 2.4\n"
            "Name: pyxel\n"
            "Version: 3.0.0\n"
            "Requires-Python: >=3.11\n"
            "Provides-Extra: app2exe\n"
            "Requires-Dist: pyinstaller ; extra == 'app2exe'\n",
        )

    assert checker.find_metadata_mismatches(wheel_path, pyproject_path) == [
        (
            "pyxel-3.0.0.dist-info/METADATA",
            "Requires-Dist fields differ from python/pyproject.toml",
        )
    ]


def test_wasm_wheel_check_rejects_a_mismatched_internal_tag(tmp_path):
    checker = _load_script("check_wasm_wheel")
    wheel_path = tmp_path / f"pyxel-3.0.0-{EXPECTED_WHEEL_TAG}.whl"
    _write_wasm_wheel(
        wheel_path,
        "3.0.0",
        tag="cp311-abi3-pyemscripten_2025_0_wasm32",
    )

    assert checker.find_wheel_tag_mismatches(wheel_path, "3.0.0") == [
        (
            "pyxel-3.0.0.dist-info/WHEEL",
            "Tag fields do not match cp311-abi3-pyemscripten_2026_0_wasm32",
        )
    ]


def test_wasm_wheel_check_requires_the_exact_expected_filename(tmp_path):
    checker = _load_script("check_wasm_wheel")
    dist_dir = tmp_path / "dist"
    dist_dir.mkdir()
    pyproject_path = tmp_path / "pyproject.toml"
    pyproject_path.write_text(
        '[project]\nname = "pyxel"\nversion = "3.0.0"\nrequires-python = ">=3.11"\n',
        encoding="utf-8",
    )
    wrong_path = dist_dir / "pyxel-3.0.0-cp311-abi3-pyemscripten_2025_0_wasm32.whl"
    _write_wasm_wheel(wrong_path, "3.0.0")

    with pytest.raises(ValueError, match="expected WebAssembly wheel.*2026_0"):
        checker.find_wheel(dist_dir, pyproject_path)


def test_tracked_wasm_wheel_matches_packaged_python_sources():
    checker = _load_script("check_wasm_wheel")
    wheel_paths = sorted((ROOT_DIR / "wasm").glob("pyxel-*-pyemscripten_*.whl"))
    version = tomllib.loads(
        (ROOT_DIR / "python" / "pyproject.toml").read_text(encoding="utf-8")
    )["project"]["version"]

    assert len(wheel_paths) == 1
    assert (
        checker.find_source_mismatches(wheel_paths[0], ROOT_DIR / "python" / "pyxel")
        == []
    )
    assert (
        checker.find_metadata_mismatches(
            wheel_paths[0], ROOT_DIR / "python" / "pyproject.toml"
        )
        == []
    )
    assert checker.find_wheel_tag_mismatches(wheel_paths[0], version) == []


def test_tracked_wasm_wheel_has_reproducible_sbom():
    wheel_paths = sorted((ROOT_DIR / "wasm").glob("pyxel-*-pyemscripten_*.whl"))
    assert len(wheel_paths) == 1

    with zipfile.ZipFile(wheel_paths[0]) as wheel_zip:
        sbom_paths = [
            name
            for name in wheel_zip.namelist()
            if ".dist-info/sboms/" in name and name.endswith(".json")
        ]
        assert len(sbom_paths) == 1
        sbom = json.loads(wheel_zip.read(sbom_paths[0]))

    assert "serialNumber" not in sbom
    assert sbom["metadata"]["timestamp"] == "1980-01-01T00:00:00.000000000Z"


def test_install_wasm_wheel_updates_prettier_formatted_path(tmp_path):
    scripts_dir = tmp_path / "scripts"
    dist_dir = tmp_path / "dist"
    wasm_dir = tmp_path / "wasm"
    scripts_dir.mkdir()
    dist_dir.mkdir()
    wasm_dir.mkdir()
    _write_project_version(tmp_path, "3.1.0")

    installer = scripts_dir / "install_wasm_wheel"
    shutil.copy2(ROOT_DIR / "scripts" / "install_wasm_wheel", installer)
    wheel_name = "pyxel-3.1.0-cp311-abi3-pyemscripten_2026_0_wasm32.whl"
    _write_wasm_wheel(dist_dir / wheel_name, "3.1.0")
    (wasm_dir / "pyxel-old-cp311-abi3-emscripten_5_0_3_wasm32.whl").write_bytes(b"old")
    (wasm_dir / "pyxel.js").write_text(
        'const PYXEL_WHEEL_PATH =\n  "pyxel-old.whl";\n',
        encoding="utf-8",
    )

    subprocess.run([installer], check=True)

    assert sorted(path.name for path in wasm_dir.glob("*.whl")) == [wheel_name]
    assert (wasm_dir / "pyxel.js").read_text(encoding="utf-8") == (
        f'const PYXEL_WHEEL_PATH =\n  "{wheel_name}";\n'
    )


def test_install_wasm_wheel_preserves_current_wheel_on_ambiguous_input(tmp_path):
    scripts_dir = tmp_path / "scripts"
    dist_dir = tmp_path / "dist"
    wasm_dir = tmp_path / "wasm"
    scripts_dir.mkdir()
    dist_dir.mkdir()
    wasm_dir.mkdir()
    _write_project_version(tmp_path, "3.1.0")

    installer = scripts_dir / "install_wasm_wheel"
    shutil.copy2(ROOT_DIR / "scripts" / "install_wasm_wheel", installer)
    for version in ("3.1.0", "3.1.1"):
        _write_wasm_wheel(
            dist_dir / f"pyxel-{version}-cp311-abi3-pyemscripten_2026_0_wasm32.whl",
            version,
        )
    current_wheel = wasm_dir / ("pyxel-3.0.0-cp311-abi3-pyemscripten_2026_0_wasm32.whl")
    current_wheel.write_bytes(b"current")
    script = 'const PYXEL_WHEEL_PATH = "' + current_wheel.name + '";\n'
    (wasm_dir / "pyxel.js").write_text(script, encoding="utf-8")

    result = subprocess.run([installer], check=False)

    assert result.returncode == 1
    assert current_wheel.read_bytes() == b"current"
    assert (wasm_dir / "pyxel.js").read_text(encoding="utf-8") == script


def test_install_wasm_wheel_preserves_current_wheel_on_invalid_loader(tmp_path):
    scripts_dir = tmp_path / "scripts"
    dist_dir = tmp_path / "dist"
    wasm_dir = tmp_path / "wasm"
    scripts_dir.mkdir()
    dist_dir.mkdir()
    wasm_dir.mkdir()
    _write_project_version(tmp_path, "3.1.0")

    installer = scripts_dir / "install_wasm_wheel"
    shutil.copy2(ROOT_DIR / "scripts" / "install_wasm_wheel", installer)
    new_wheel = dist_dir / "pyxel-3.1.0-cp311-abi3-pyemscripten_2026_0_wasm32.whl"
    _write_wasm_wheel(new_wheel, "3.1.0")
    current_wheel = wasm_dir / ("pyxel-3.0.0-cp311-abi3-pyemscripten_2026_0_wasm32.whl")
    current_wheel.write_bytes(b"current")
    script = 'const INVALID_WHEEL_PATH = "pyxel-old.whl";\n'
    (wasm_dir / "pyxel.js").write_text(script, encoding="utf-8")

    result = subprocess.run([installer], check=False)

    assert result.returncode == 1
    assert current_wheel.read_bytes() == b"current"
    assert not (wasm_dir / new_wheel.name).exists()
    assert (wasm_dir / "pyxel.js").read_text(encoding="utf-8") == script


def test_install_wasm_wheel_preserves_loader_target_when_script_replace_fails(
    tmp_path, monkeypatch
):
    installer = _load_script("install_wasm_wheel")
    dist_dir = tmp_path / "dist"
    wasm_dir = tmp_path / "wasm"
    dist_dir.mkdir()
    wasm_dir.mkdir()
    _write_project_version(tmp_path, "3.1.0")

    old_wheel = wasm_dir / "pyxel-3.0.0-cp311-abi3-pyemscripten_2026_0_wasm32.whl"
    new_wheel = dist_dir / "pyxel-3.1.0-cp311-abi3-pyemscripten_2026_0_wasm32.whl"
    old_wheel.write_bytes(b"old")
    _write_wasm_wheel(new_wheel, "3.1.0")
    script_path = wasm_dir / "pyxel.js"
    script = f'const PYXEL_WHEEL_PATH = "{old_wheel.name}";\n'
    script_path.write_text(script, encoding="utf-8")

    real_replace = Path.replace

    def replace_with_script_failure(source, target):
        if source.name == "pyxel.js.tmp":
            raise OSError("injected script replacement failure")
        return real_replace(source, target)

    installer.DIST_DIR = dist_dir
    installer.WASM_DIR = wasm_dir
    installer.PYPROJECT_PATH = tmp_path / "python" / "pyproject.toml"
    monkeypatch.setattr(Path, "replace", replace_with_script_failure)

    with pytest.raises(OSError, match="injected script replacement failure"):
        installer.main()

    assert script_path.read_text(encoding="utf-8") == script
    assert old_wheel.read_bytes() == b"old"
    assert (wasm_dir / new_wheel.name).read_bytes() == new_wheel.read_bytes()


@pytest.mark.parametrize(
    ("wheel_name", "internal_tag"),
    [
        (
            "pyxel-3.1.0-cp311-abi3-pyemscripten_2025_0_wasm32.whl",
            EXPECTED_WHEEL_TAG,
        ),
        (
            f"pyxel-3.1.0-{EXPECTED_WHEEL_TAG}.whl",
            "cp311-abi3-pyemscripten_2025_0_wasm32",
        ),
    ],
    ids=["filename", "internal-tag"],
)
def test_install_wasm_wheel_rejects_wrong_wheel_contract_before_changes(
    tmp_path, wheel_name, internal_tag
):
    scripts_dir = tmp_path / "scripts"
    dist_dir = tmp_path / "dist"
    wasm_dir = tmp_path / "wasm"
    scripts_dir.mkdir()
    dist_dir.mkdir()
    wasm_dir.mkdir()
    _write_project_version(tmp_path, "3.1.0")
    installer = scripts_dir / "install_wasm_wheel"
    shutil.copy2(ROOT_DIR / "scripts" / "install_wasm_wheel", installer)
    _write_wasm_wheel(dist_dir / wheel_name, "3.1.0", internal_tag)
    current_wheel = wasm_dir / f"pyxel-3.0.0-{EXPECTED_WHEEL_TAG}.whl"
    current_wheel.write_bytes(b"current")
    script = f'const PYXEL_WHEEL_PATH = "{current_wheel.name}";\n'
    (wasm_dir / "pyxel.js").write_text(script, encoding="utf-8")

    result = subprocess.run([installer], check=False)

    assert result.returncode == 1
    assert current_wheel.read_bytes() == b"current"
    assert (wasm_dir / "pyxel.js").read_text(encoding="utf-8") == script
