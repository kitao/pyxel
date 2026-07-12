import importlib.util
import re
from importlib.machinery import SourceFileLoader
from pathlib import Path


MODULE_PATH = Path(__file__).parents[2] / "scripts" / "update_version"
WORKFLOW_DIR = Path(__file__).parents[2] / ".github/workflows"
CORE_DIR = Path(__file__).parents[2] / "crates/pyxel-core"


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
            '"pyxel-2.9.7-cp311-abi3-emscripten_5_0_3_wasm32.whl";\n'
        ),
    }
    for relative_path, text in files.items():
        path = root / relative_path
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")


def test_update_version_updates_and_verifies_every_runtime_surface(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)

    update_version.update_version("3.0.0a1", tmp_path)

    assert update_version.version_errors(tmp_path, "v3.0.0a1") == []
    assert 'version = "3.0.0-alpha.1"' in (tmp_path / "crates/Cargo.toml").read_text(
        encoding="utf-8"
    )
    assert "pyxel-3.0.0a1-cp311" in (tmp_path / "wasm/pyxel.js").read_text(
        encoding="utf-8"
    )


def test_version_errors_names_the_mismatched_surface(tmp_path):
    update_version = _load_update_version()
    _write_version_files(tmp_path)
    wasm_path = tmp_path / "wasm/pyxel.js"
    wasm_path.write_text(
        wasm_path.read_text(encoding="utf-8").replace("2.9.7", "2.9.6"),
        encoding="utf-8",
    )

    errors = update_version.version_errors(tmp_path, "v2.9.7")

    assert errors == ["wasm/pyxel.js: expected 2.9.7, found 2.9.6"]


def test_release_build_depends_on_version_check():
    release_workflow = (WORKFLOW_DIR / "release.yml").read_text(encoding="utf-8")
    verify_workflow = (WORKFLOW_DIR / "verify.yml").read_text(encoding="utf-8")

    assert "release_tag: ${{ github.ref_name }}" in release_workflow
    assert "build:\n    needs: verify\n" in release_workflow
    assert 'scripts/update_version --check "$RELEASE_TAG"' in verify_workflow
    assert release_workflow.index(
        "twine upload --skip-existing"
    ) < release_workflow.index("uses: softprops/action-gh-release@v2")


def test_aarch64_wheel_is_imported_on_native_arm_runner():
    build_workflow = (WORKFLOW_DIR / "build.yml").read_text(encoding="utf-8")

    assert "target: aarch64-unknown-linux-gnu\n            os: ubuntu-24.04-arm" in (
        build_workflow
    )
    assert "os: ubuntu-24.04-arm\n            skip_import: true" not in build_workflow


def test_windows_arm_host_verifies_gui_quit_with_x64_wheel():
    build_workflow = (WORKFLOW_DIR / "build.yml").read_text(encoding="utf-8")
    build_jobs, verify_wheels = build_workflow.split("  verify-wheels:", maxsplit=1)

    assert (
        "target: x86_64-pc-windows-msvc\n"
        "            os: windows-11-arm\n"
        "            arch: x64\n"
        "            verify_gui_quit: true"
    ) in verify_wheels
    assert "windows-11-arm" not in build_jobs
    assert "name: Verify GUI quit on Windows ARM64" in build_workflow
    assert "if: ${{ matrix.verify_gui_quit }}" in build_workflow
    assert "timeout-minutes: 1" in build_workflow
    assert "platform.machine()" in build_workflow
    assert "pyxel.init(64, 64" in build_workflow
    assert "pyxel.quit()" in build_workflow


def test_sdl2_archive_is_verified_from_one_version_and_digest_owner():
    build_workflow = (WORKFLOW_DIR / "build.yml").read_text(encoding="utf-8")
    build_rs = (CORE_DIR / "build.rs").read_text(encoding="utf-8")
    cargo_toml = (CORE_DIR / "Cargo.toml").read_text(encoding="utf-8")
    build_winmac, build_linux = build_workflow.split("  build-linux:", maxsplit=1)
    build_linux, _ = build_linux.split("  verify-wheels:", maxsplit=1)

    assert re.search(r'const SDL2_SHA256: &str = "[0-9a-f]{64}";', build_rs)
    assert "sha2 = " in cargo_toml
    assert 'SDL2_VERSION: "' not in build_workflow
    assert "name: Resolve SDL2 source" not in build_winmac
    assert "name: Resolve SDL2 source" in build_linux
    assert "SDL2_VERSION: &str" in build_linux
    assert "SDL2_SHA256: &str" in build_linux
    assert "curl -qfsSLO" in build_linux
    assert "sha256sum --check --strict" in build_linux
    assert build_linux.index("sha256sum --check --strict") < build_linux.index(
        "tar xzf SDL2-"
    )
    assert "verify_sdl2_archive(&sdl2_archive_path)" in build_rs
    assert "assert_eq!(actual, SDL2_SHA256" in build_rs
    assert build_rs.index("verify_sdl2_archive(&sdl2_archive_path)") < build_rs.index(
        "Archive::new"
    )
