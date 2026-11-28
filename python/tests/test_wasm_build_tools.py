import importlib.util
import json
import os
import shutil
import subprocess
import zipfile
from importlib.machinery import SourceFileLoader
from pathlib import Path

import pytest

ROOT_DIR = Path(__file__).parents[2]
EXPECTED_WHEEL_TAG = "cp311-abi3-pyemscripten_2026_0_wasm32"
WASM_HEADER = b"\x00asm\x01\x00\x00\x00"


def test_wasm_wheel_check_rejects_generated_metadata_and_host_paths(tmp_path, capsys):
    checker = _load_script("check_wasm_wheel")
    wheel_path = tmp_path / f"pyxel-3.0.0-{EXPECTED_WHEEL_TAG}.whl"
    with zipfile.ZipFile(wheel_path, "w") as wheel_zip:
        wheel_zip.writestr("pyxel/__pycache__/module.cpython-314.pyc", b"bytecode")
        wheel_zip.writestr("pyxel/.ruff_cache/CACHEDIR.TAG", b"cache")
        wheel_zip.writestr("pyxel/module.pyo", b"optimized bytecode")
        wheel_zip.writestr("pyxel/examples/.DS_Store", b"metadata")
        wheel_zip.writestr(
            "pyxel/pyxel_binding.abi3.so", WASM_HEADER + b"/Users/example/src"
        )
        wheel_zip.writestr("pyxel/examples/tool.js", b"/home/example/src")

    assert checker.find_violations(wheel_path) == [
        ("pyxel/__pycache__/module.cpython-314.pyc", "generated Python bytecode"),
        ("pyxel/.ruff_cache/CACHEDIR.TAG", "generated Ruff cache"),
        ("pyxel/module.pyo", "generated Python bytecode"),
        ("pyxel/examples/.DS_Store", "platform metadata file"),
        ("pyxel/pyxel_binding.abi3.so", "host path b'/Users/'"),
        ("pyxel/examples/tool.js", "host path b'/home/'"),
    ]

    checker.DIST_DIR = tmp_path
    _write_project_version(tmp_path, "3.0.0")
    checker.PYPROJECT_PATH = tmp_path / "python" / "pyproject.toml"
    assert checker.main([]) == 1
    output = capsys.readouterr()
    assert output.out == ""
    assert "error: invalid contents detected" in output.err


@pytest.mark.parametrize("filename", ["cli.py", "tool.js"])
def test_wasm_wheel_check_detects_stale_packaged_sources(tmp_path, filename):
    checker = _load_script("check_wasm_wheel")
    source_dir = tmp_path / "pyxel"
    source_dir.mkdir()
    (source_dir / filename).write_text("current\n", encoding="utf-8")

    wheel_path = tmp_path / "pyxel-test-pyemscripten_2026_0_wasm32.whl"
    with zipfile.ZipFile(wheel_path, "w") as wheel_zip:
        wheel_zip.writestr(f"pyxel/{filename}", "stale\n")

    assert checker.find_source_mismatches(wheel_path, source_dir) == [
        (f"pyxel/{filename}", f"differs from python/pyxel/{filename}")
    ]


@pytest.mark.parametrize("filename", ["cli.py", "tool.js"])
def test_wasm_wheel_check_detects_missing_packaged_sources(tmp_path, filename):
    checker = _load_script("check_wasm_wheel")
    source_dir = tmp_path / "pyxel"
    source_dir.mkdir()
    (source_dir / filename).write_text("current\n", encoding="utf-8")

    wheel_path = tmp_path / "pyxel-test-pyemscripten_2026_0_wasm32.whl"
    with zipfile.ZipFile(wheel_path, "w"):
        pass

    assert checker.find_source_mismatches(wheel_path, source_dir) == [
        (f"pyxel/{filename}", "missing packaged source")
    ]


@pytest.mark.parametrize("filename", ["removed.py", "tool.js"])
def test_wasm_wheel_check_detects_removed_packaged_sources(tmp_path, filename):
    checker = _load_script("check_wasm_wheel")
    source_dir = tmp_path / "pyxel"
    source_dir.mkdir()
    (source_dir / "cli.py").write_text("current\n", encoding="utf-8")

    wheel_path = tmp_path / "pyxel-test-pyemscripten_2026_0_wasm32.whl"
    with zipfile.ZipFile(wheel_path, "w") as wheel_zip:
        wheel_zip.writestr("pyxel/cli.py", "current\n")
        wheel_zip.writestr(f"pyxel/{filename}", "stale\n")

    assert checker.find_source_mismatches(wheel_path, source_dir) == [
        (f"pyxel/{filename}", "not present in python/pyxel")
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


@pytest.fixture
def release_checkout(tmp_path, monkeypatch):
    for key in os.environ.copy():
        if key.upper().startswith("GIT_"):
            monkeypatch.delenv(key)
    root = tmp_path / "checkout"
    _write_project_version(root, "3.0.0")
    package = root / "python" / "pyxel"
    package.mkdir(exist_ok=True)
    (package / "__init__.py").write_bytes(b"")
    for name in ("LICENSE", "README.md"):
        (root / name).write_text(name, encoding="utf-8")
    subprocess.run(["git", "init", "-q"], cwd=root, check=True)
    subprocess.run(
        ["git", "add", "python", "LICENSE", "README.md"], cwd=root, check=True
    )
    subprocess.run(
        [
            "git",
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "-qm",
            "Package sources",
        ],
        cwd=root,
        check=True,
    )
    return root


def test_release_wheel_uses_committed_inputs_and_checks_sbom_paths(
    release_checkout, release_members, tmp_path
):
    checker = _load_script("check_wasm_wheel")
    package = release_checkout / "python/pyxel"
    (package / "notes.txt").write_text("local note", encoding="utf-8")
    wheel = tmp_path / f"pyxel-3.0.0-{EXPECTED_WHEEL_TAG}.whl"
    sbom = "pyxel-3.0.0.dist-info/sboms/pyxel-binding.cyclonedx.json"
    for contaminated in (False, True):
        location = (
            "file://%2Fhome%2Fbuilder%2Fsrc" if contaminated else "file:///src/pyxel"
        )
        members = release_members | {sbom: json.dumps({"ref": location})}
        if contaminated:
            members["pyxel/notes.txt"] = (package / "notes.txt").read_bytes()
        _write_wasm_wheel(wheel, "3.0.0", members=members)
        assert checker.find_release_violations(wheel, release_checkout) == (
            [
                ("pyxel/notes.txt", "not present in python/pyxel"),
                (sbom, "build-host path in the SBOM"),
            ]
            if contaminated
            else []
        )


def test_tracked_wasm_wheel_matches_its_source(tmp_path, monkeypatch):
    for name in list(os.environ):
        if name.upper().startswith("GIT_"):
            monkeypatch.delenv(name)
    checker = _load_script("check_wasm_wheel")
    wheel_paths = sorted((ROOT_DIR / "wasm").glob("pyxel-*-pyemscripten_*.whl"))
    assert len(wheel_paths) == 1
    version = wheel_paths[0].name.split("-")[1]
    source_root = ROOT_DIR
    release_ref = f"refs/tags/v{version}"
    tag_check = subprocess.run(
        ["git", "show-ref", "--verify", "--quiet", release_ref],
        cwd=ROOT_DIR,
        check=False,
    )
    assert tag_check.returncode in (0, 1)
    if tag_check.returncode == 0:
        commit = subprocess.check_output(
            ["git", "rev-parse", f"{release_ref}^{{commit}}"],
            cwd=ROOT_DIR,
            text=True,
        ).strip()
        entry = subprocess.check_output(
            ["git", "ls-tree", commit, "--", f"wasm/{wheel_paths[0].name}"],
            cwd=ROOT_DIR,
            text=True,
        )
        if entry:
            release_wheel = subprocess.check_output(
                ["git", "cat-file", "blob", entry.split()[2]], cwd=ROOT_DIR
            )
            # Retained releases belong to their tagged source; local rebuilds
            # must still match the current working tree.
            if wheel_paths[0].read_bytes() == release_wheel:
                checker.extract_package_sources(ROOT_DIR, commit, tmp_path)
                source_root = tmp_path

    assert (
        checker.find_wheel(ROOT_DIR / "wasm", source_root / "python" / "pyproject.toml")
        == wheel_paths[0]
    )
    assert (
        checker.find_wheel_mismatches(
            wheel_paths[0],
            source_root / "python" / "pyproject.toml",
            source_root / "python" / "pyxel",
        )
        == []
    )


def test_tracked_wasm_wheel_omits_serial_number_and_fixes_timestamp():
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


@pytest.fixture
def release_installer(release_checkout):
    installer = _load_script("install_wasm_wheel")
    installer.ROOT_DIR = release_checkout
    installer.PYPROJECT_PATH = release_checkout / "python/pyproject.toml"
    installer.WASM_DIR = release_checkout / "wasm"
    installer.WASM_DIR.mkdir()
    old = installer.WASM_DIR / "pyxel-old-cp311-abi3-emscripten_5_0_3_wasm32.whl"
    old.write_bytes(b"previous")
    (installer.WASM_DIR / "pyxel.js").write_text(
        f'const PYXEL_WHEEL_PATH =\n  "{old.name}";\n'
    )
    return installer


@pytest.fixture
def release_members(release_checkout):
    return {
        "pyxel/__init__.py": b"",
        "pyxel/LICENSE": (release_checkout / "LICENSE").read_bytes(),
        "pyxel/README.md": (release_checkout / "README.md").read_bytes(),
        "pyxel-3.0.0.dist-info/sboms/pyxel-binding.cyclonedx.json": "{}",
    }


@pytest.fixture
def release_artifact(tmp_path, release_members):
    wheel = tmp_path / f"pyxel-3.0.0-{EXPECTED_WHEEL_TAG}.whl"
    _write_wasm_wheel(wheel, "3.0.0", members=release_members)
    return wheel


def _contents(directory):
    return {path.name: path.read_bytes() for path in directory.iterdir()}


def test_installer_without_a_run_id_keeps_the_retained_wheel(
    release_installer, release_artifact
):
    root = release_installer.ROOT_DIR
    scripts = root / "scripts"
    scripts.mkdir()
    for name in ("install_wasm_wheel", "check_wasm_wheel"):
        shutil.copy2(ROOT_DIR / "scripts" / name, scripts)
    dist = root / "dist"
    dist.mkdir()
    shutil.copyfile(release_artifact, dist / release_artifact.name)
    for name in ("LICENSE", "README.md"):
        shutil.copyfile(root / name, root / "python/pyxel" / name)
    before = _contents(release_installer.WASM_DIR)

    result = subprocess.run(
        [scripts / "install_wasm_wheel"], capture_output=True, text=True, check=False
    )

    assert result.returncode == 2
    assert "--github-run" in result.stderr
    assert _contents(release_installer.WASM_DIR) == before


def test_install_wasm_wheel_updates_prettier_formatted_path(
    release_installer, release_artifact
):
    assert release_installer.install_wheel(release_artifact.parent) == 0
    assert _contents(release_installer.WASM_DIR) == {
        release_artifact.name: release_artifact.read_bytes(),
        "pyxel.js": f'const PYXEL_WHEEL_PATH =\n  "{release_artifact.name}";\n'.encode(),
    }


@pytest.mark.parametrize("defect", ["ambiguous input", "invalid loader"])
def test_install_wasm_wheel_preserves_current_wheel(
    release_installer, release_artifact, defect
):
    if defect == "ambiguous input":
        (release_artifact.parent / f"pyxel-3.1.0-{EXPECTED_WHEEL_TAG}.whl").touch()
    else:
        (release_installer.WASM_DIR / "pyxel.js").write_text(
            'const OTHER = "old.whl";\n'
        )
    before = _contents(release_installer.WASM_DIR)

    assert release_installer.install_wheel(release_artifact.parent) == 1
    assert _contents(release_installer.WASM_DIR) == before


def test_install_wasm_wheel_preserves_loader_target_when_script_replace_fails(
    release_installer, release_artifact, monkeypatch
):
    before = _contents(release_installer.WASM_DIR)
    original_replace = Path.replace

    def replace(source, target):
        if source.name == "pyxel.js.tmp":
            raise OSError("injected script replacement failure")
        return original_replace(source, target)

    monkeypatch.setattr(Path, "replace", replace)
    with pytest.raises(OSError, match="injected script replacement failure"):
        release_installer.install_wheel(release_artifact.parent)
    assert _contents(release_installer.WASM_DIR) == {
        **before,
        release_artifact.name: release_artifact.read_bytes(),
    }


@pytest.mark.parametrize(
    ("member", "value", "reason"),
    [
        ("pyxel/__init__.py", "stale", "differs from python/pyxel"),
        (
            "pyxel-3.0.0.dist-info/METADATA",
            "Name: pyxel",
            "Requires-Python fields differ",
        ),
        (
            "pyxel-3.0.0.dist-info/WHEEL",
            "Tag: cp311-abi3-pyemscripten_2025_0_wasm32",
            "Tag fields do not match",
        ),
        ("pyxel/__pycache__/cli.pyc", b"cache", "generated Python bytecode"),
        ("pyxel/binding.so", b"/Users/example/source", "host path"),
        (
            "pyxel/pyxel_binding.abi3.so",
            None,
            "expected exactly one WebAssembly binding",
        ),
        ("pyxel/pyxel_binding.abi3.so", b"", "invalid WebAssembly module header"),
    ],
)
def test_installer_rejects_invalid_contents_before_changes(
    release_installer, release_artifact, release_members, capsys, member, value, reason
):
    _write_wasm_wheel(
        release_artifact, "3.0.0", members=release_members | {member: value}
    )
    before = _contents(release_installer.WASM_DIR)

    assert release_installer.install_wheel(release_artifact.parent) == 1
    assert reason in capsys.readouterr().err
    assert _contents(release_installer.WASM_DIR) == before


def test_installer_rejects_wrong_wheel_filename_before_changes(
    release_installer, release_artifact
):
    release_artifact.rename(
        release_artifact.with_name(release_artifact.name.replace("2026_0", "2025_0"))
    )
    before = _contents(release_installer.WASM_DIR)

    assert release_installer.install_wheel(release_artifact.parent) == 1
    assert _contents(release_installer.WASM_DIR) == before


@pytest.mark.parametrize(
    "defect",
    [
        "different workflow",
        "different commit",
        "failed run",
        "extra wheel",
        "unsigned",
        None,
    ],
)
def test_installing_an_official_wheel(release_checkout, monkeypatch, defect):
    installer = _load_script("install_wasm_wheel")
    monkeypatch.setattr(installer, "ROOT_DIR", release_checkout)
    commit = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=release_checkout, text=True
    ).strip()
    installed = []

    def github(command, *args):
        if command == "api":
            assert args[-1] == "repos/kitao/pyxel/actions/runs/42"
            return json.dumps(
                {
                    "path": "other.yml"
                    if defect == "different workflow"
                    else ".github/workflows/build-wasm.yml",
                    "event": "workflow_dispatch",
                    "status": "completed",
                    "conclusion": "failure" if defect == "failed run" else "success",
                    "head_sha": "another" if defect == "different commit" else commit,
                    "head_repository": {"full_name": "kitao/pyxel"},
                }
            )
        assert args[args.index("--repo") + 1] == "kitao/pyxel"
        if command == "run":
            assert args[:2] == ("download", "42")
            assert args[args.index("--name") + 1] == "pyxel-wasm"
            directory = Path(args[args.index("--dir") + 1])
            (directory / "official.whl").write_bytes(b"downloaded")
            if defect == "extra wheel":
                (directory / "other.whl").touch()
        else:
            assert command == "attestation" and args[0] == "verify"
            assert args[args.index("--source-digest") + 1] == commit
            assert (
                args[args.index("--signer-workflow") + 1]
                == "kitao/pyxel/.github/workflows/build-wasm.yml"
            )
            assert "--deny-self-hosted-runners" in args
            if defect == "unsigned":
                raise subprocess.CalledProcessError(1, ["gh", command])

    def install(directory):
        assert directory != release_checkout / "dist"
        installed.append((directory / "official.whl").read_bytes())
        return 0

    monkeypatch.setattr(installer, "github", github)
    monkeypatch.setattr(installer, "install_wheel", install)
    assert installer.main("42") == (0 if defect is None else 1)
    assert installed == ([b"downloaded"] if defect is None else [])


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


def _write_wasm_wheel(
    path: Path, version: str, tag: str = EXPECTED_WHEEL_TAG, *, members=None
) -> None:
    files = {
        "pyxel/pyxel_binding.abi3.so": WASM_HEADER,
        f"pyxel-{version}.dist-info/METADATA": f"Metadata-Version: 2.4\nName: pyxel\nVersion: {version}\nRequires-Python: >=3.11\n",
        f"pyxel-{version}.dist-info/WHEEL": f"Wheel-Version: 1.0\nRoot-Is-Purelib: false\nTag: {tag}\n",
    }
    with zipfile.ZipFile(path, "w") as wheel_zip:
        for name, data in (files | (members or {})).items():
            if data is not None:
                wheel_zip.writestr(name, data)
