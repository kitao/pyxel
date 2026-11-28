import importlib.util
import zipfile
from importlib.machinery import SourceFileLoader
from pathlib import Path

import pytest

MODULE_PATH = Path(__file__).parents[2] / "scripts" / "update_version"
EXPECTED_WHEEL_TAG = "cp311-abi3-pyemscripten_2026_0_wasm32"
WHEEL_SUFFIX = f"-{EXPECTED_WHEEL_TAG}.whl"


def test_update_version_prepares_sources_and_preserves_the_retained_runtime(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)

    web_runtime = tmp_path / "wasm/pyxel.js"
    original = web_runtime.read_bytes()
    update_version.update_version("3.0.0a1", tmp_path)

    assert web_runtime.read_bytes() == original
    assert update_version.version_errors(tmp_path, "v3.0.0a1") == [
        "wasm/pyxel.js: expected 3.0.0a1, found 2.9.7"
    ]
    assert 'pub const VERSION: &str = "3.0.0a1";' in (
        tmp_path / "crates/pyxel-core/src/settings.rs"
    ).read_text(encoding="utf-8")
    assert 'version = "3.0.0a1"' in (tmp_path / "python/pyproject.toml").read_text(
        encoding="utf-8"
    )
    assert 'version = "3.0.0-alpha.1"' in (tmp_path / "crates/Cargo.toml").read_text(
        encoding="utf-8"
    )


@pytest.mark.parametrize(
    "version", ["--help", "typo", "3.0", "3.0.0+LOCAL", "3.0.0a1+local"]
)
def test_invalid_release_version_preserves_all_files(tmp_path, version):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    originals = {p: p.read_bytes() for p in tmp_path.rglob("*") if p.is_file()}

    with pytest.raises(ValueError, match="invalid release version"):
        update_version.update_version(version, tmp_path)

    assert all(path.read_bytes() == data for path, data in originals.items())


def test_update_version_preserves_local_version_spelling(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)

    update_version.update_version("3.0.0+local.2", tmp_path)

    assert 'version = "3.0.0+local.2"' in (tmp_path / "crates/Cargo.toml").read_text()
    assert (
        'version = "3.0.0+local.2"' in (tmp_path / "python/pyproject.toml").read_text()
    )


def test_update_version_leaves_files_unchanged_when_last_pattern_is_missing(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    cargo_path = tmp_path / "crates/Cargo.toml"
    cargo_path.write_text("[workspace.package]\n", encoding="utf-8")
    original_files = {
        path: path.read_bytes() for path in tmp_path.rglob("*") if path.is_file()
    }

    with pytest.raises(ValueError, match="version pattern not found"):
        update_version.update_version("3.0.0a1", tmp_path)

    assert {path: path.read_bytes() for path in original_files} == original_files


def test_version_errors_names_the_mismatched_surface(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    wasm_path = tmp_path / "wasm/pyxel.js"
    wasm_path.write_text(
        wasm_path.read_text(encoding="utf-8").replace("2.9.7", "2.9.6"),
        encoding="utf-8",
    )
    (tmp_path / "wasm" / f"pyxel-2.9.7{WHEEL_SUFFIX}").unlink()
    _write_wasm_wheel(
        tmp_path / "wasm" / f"pyxel-2.9.6{WHEEL_SUFFIX}",
        "2.9.6",
    )

    errors = update_version.version_errors(tmp_path, "v2.9.7")
    assert errors == ["wasm/pyxel.js: expected 2.9.7, found 2.9.6"]


def test_untagged_check_uses_runtime_settings(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    (tmp_path / "python/pyproject.toml").write_text('[project]\nversion = "2.9.8"\n')

    assert update_version.version_errors(tmp_path) == [
        "python/pyproject.toml: expected 2.9.7, found 2.9.8"
    ]


@pytest.mark.parametrize(
    ("path", "other_table"),
    [
        ("python/pyproject.toml", "[tool.example]"),
        ("crates/Cargo.toml", "[workspace.dependencies.example]"),
    ],
)
def test_version_update_only_uses_the_package_table(tmp_path, path, other_table):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    manifest = tmp_path / path
    other = f'{other_table}\nversion = "1.2.3"\n\n'
    original = manifest.read_text()
    manifest.write_text(other + original)

    assert update_version.version_errors(tmp_path, build_only=True) == []
    update_version.update_version("2.9.8", tmp_path)
    assert manifest.read_text() == other + original.replace("2.9.7", "2.9.8")


def test_build_check_precedes_wheel_generation(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    update_version.update_version("3.0.0a1", tmp_path)
    assert update_version.version_errors(tmp_path, build_only=True) == []

    (tmp_path / "crates/pyxel-core/src/settings.rs").write_text(
        'pub const VERSION: &str = "3.0.0";\n'
    )
    assert update_version.version_errors(tmp_path, build_only=True) == [
        "python/pyproject.toml: expected 3.0.0, found 3.0.0a1",
        "crates/Cargo.toml: expected 3.0.0, found 3.0.0-alpha.1",
    ]


def test_version_errors_accepts_prettier_formatted_wasm_constant(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    wasm_path = tmp_path / "wasm" / "pyxel.js"
    wasm_path.write_text(
        wasm_path.read_text(encoding="utf-8").replace(
            'const PYXEL_WHEEL_PATH = "',
            'const PYXEL_WHEEL_PATH =\n  "',
        ),
        encoding="utf-8",
    )

    assert update_version.version_errors(tmp_path, "v2.9.7") == []


def test_version_errors_reports_a_missing_referenced_wasm_wheel(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    (tmp_path / "wasm" / f"pyxel-2.9.7{WHEEL_SUFFIX}").rename(
        tmp_path / "wasm" / f"pyxel-2.9.6{WHEEL_SUFFIX}"
    )

    errors = update_version.version_errors(tmp_path, "v2.9.7")
    assert errors == [
        f"wasm/pyxel.js: referenced wheel not found: wasm/pyxel-2.9.7{WHEEL_SUFFIX}"
    ]


def test_version_errors_rejects_multiple_wasm_wheels(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    _write_wasm_wheel(
        tmp_path / "wasm" / f"pyxel-2.9.6{WHEEL_SUFFIX}",
        "2.9.6",
    )

    errors = update_version.version_errors(tmp_path, "v2.9.7")
    assert errors == [
        (
            "wasm: expected exactly one Pyxel wheel, found 2: "
            f"pyxel-2.9.6{WHEEL_SUFFIX}, pyxel-2.9.7{WHEEL_SUFFIX}"
        )
    ]


def test_version_errors_rejects_wrong_wasm_wheel_filename_contract(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    expected_path = tmp_path / "wasm" / f"pyxel-2.9.7{WHEEL_SUFFIX}"
    wrong_name = "pyxel-2.9.7-cp311-abi3-pyemscripten_2025_0_wasm32.whl"
    expected_path.rename(tmp_path / "wasm" / wrong_name)
    wasm_script = tmp_path / "wasm" / "pyxel.js"
    wasm_script.write_text(
        wasm_script.read_text(encoding="utf-8").replace(
            expected_path.name,
            wrong_name,
        ),
        encoding="utf-8",
    )

    errors = update_version.version_errors(tmp_path, "v2.9.7")
    assert errors == [
        (f"wasm/pyxel.js: expected wheel pyxel-2.9.7{WHEEL_SUFFIX}, found {wrong_name}")
    ]


def test_version_errors_rejects_wrong_internal_wasm_wheel_tag(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    wheel_path = tmp_path / "wasm" / f"pyxel-2.9.7{WHEEL_SUFFIX}"
    _write_wasm_wheel(
        wheel_path,
        "2.9.7",
        tag="cp311-abi3-pyemscripten_2025_0_wasm32",
    )

    errors = update_version.version_errors(tmp_path, "v2.9.7")
    assert errors == [
        (f"wasm/{wheel_path.name}: WHEEL Tag fields do not match {EXPECTED_WHEEL_TAG}")
    ]


def _load_update_version():
    loader = SourceFileLoader("update_version_test", str(MODULE_PATH))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    assert spec is not None
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


def _write_version_files(root: Path) -> None:
    files = {
        "crates/Cargo.toml": '[workspace.package]\nversion = "2.9.7"\n',
        "crates/pyxel-core/src/settings.rs": 'pub const VERSION: &str = "2.9.7";\n',
        "python/pyproject.toml": '[project]\nversion = "2.9.7"\n',
        "wasm/pyxel.js": (
            "const PYXEL_WHEEL_PATH = "
            '"pyxel-2.9.7-cp311-abi3-pyemscripten_2026_0_wasm32.whl";\n'
        ),
    }

    for relative_path, text in files.items():
        path = root / relative_path
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")

    _write_wasm_wheel(root / "wasm" / f"pyxel-2.9.7{WHEEL_SUFFIX}", "2.9.7")


def _write_wasm_wheel(path: Path, version: str, tag: str = EXPECTED_WHEEL_TAG) -> None:
    with zipfile.ZipFile(path, "w") as wheel_zip:
        wheel_zip.writestr(
            f"pyxel-{version}.dist-info/WHEEL",
            f"Wheel-Version: 1.0\nRoot-Is-Purelib: false\nTag: {tag}\n",
        )
