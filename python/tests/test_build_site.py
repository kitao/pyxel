import hashlib
import importlib.util
import io
import json
import os
import subprocess
import sys
import zipfile
from importlib.machinery import SourceFileLoader
from pathlib import Path

import pytest

ROOT_DIR = Path(__file__).parents[2]


@pytest.fixture(autouse=True)
def isolated_git(monkeypatch):
    # Inherited variables such as GIT_DIR must not point the fixtures' git elsewhere.
    for name in list(os.environ):
        if name.upper().startswith("GIT_"):
            monkeypatch.delenv(name)


@pytest.fixture
def builder():
    loader = SourceFileLoader("build_site", str(ROOT_DIR / "scripts" / "build_site"))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


def test_release_uses_tagged_bytes_and_excludes_legacy_pages(builder, tmp_path):
    repository = tmp_path / "repository"
    runtime = repository / "wasm"
    runtime.mkdir(parents=True)
    wheel = "pyxel-2.9.9-cp311-abi3-emscripten_5_0_3_wasm32.whl"
    (runtime / "pyxel.js").write_text(f'const PYXEL_WHEEL_PATH = "{wheel}";\n')
    (runtime / "pyxel.css").write_text("body { color: white; }\n")
    (runtime / "import_hook.py").write_text("# Import hook\n")
    (runtime / "images").mkdir()
    (runtime / "images/logo.png").write_bytes(b"tagged image")
    (runtime / "launcher").mkdir()
    (runtime / "launcher/index.html").write_text("legacy redirect")
    with zipfile.ZipFile(runtime / wheel, "w") as archive:
        archive.writestr(
            "pyxel-2.9.9.dist-info/METADATA", "Name: pyxel\nVersion: 2.9.9\n"
        )
    subprocess.run(["git", "init", "-q", str(repository)], check=True)
    subprocess.run(["git", "-C", str(repository), "add", "wasm"], check=True)
    subprocess.run(
        [
            "git",
            "-C",
            str(repository),
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "-qm",
            "Release fixture",
        ],
        check=True,
    )
    subprocess.run(["git", "-C", str(repository), "tag", "v2.9.9"], check=True)
    (runtime / "pyxel.js").write_text("unreleased working copy")
    output = tmp_path / "site"
    assert builder.copy_release(repository, "v2.9.9", output) == "2.9.9"
    assert (
        output / "2.9.9/pyxel.js"
    ).read_text() == f'const PYXEL_WHEEL_PATH = "{wheel}";\n'
    assert (output / "2.9.9" / wheel).read_bytes() == (runtime / wheel).read_bytes()
    assert (output / "2.9.9/images/logo.png").read_bytes() == b"tagged image"
    assert not (output / "2.9.9/launcher").exists()


def test_rewrite_separates_live_runtime_from_copyable_versioned_snippets(builder):
    source = (
        '<head><script src="https://cdn.jsdelivr.net/gh/kitao/pyxel/wasm/pyxel.js"></script></head>\n'
        "<body><script>const snippet = '<script src=\"https://cdn.jsdelivr.net/gh/kitao/pyxel@v2.9.9/wasm/pyxel.js\"><\\/script>';</script>\n"
        '<pyxel-run root="../../../../python/pyxel/examples/cube"></pyxel-run>\n'
        '<pyxel-run root="../../../python/pyxel/examples"></pyxel-run>\n'
        '<img src="../../../docs/images/demo.gif"><img src="../../wasm/images/logo.png"></body>'
        '<script>const palette = "../../docs/pyxel.gpl";</script>'
        '<iframe src="pyxel-screen.html"></iframe>'
        '<script>resourceEditor.src = "pyxel-editor.html";</script>'
        '<a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Launcher input</a>'
    )
    result = builder.rewrite_urls(source, "2.9.9", html=True)
    assert '<head><script src="/pyxel/3.0.0/pyxel.js">' in result
    assert 'src="https://kitao.dev/pyxel/2.9.9/pyxel.js"' in result
    assert 'root="/pyxel/assets/examples/cube"' in result
    assert 'root="/pyxel/assets/examples"' in result
    assert 'src="/pyxel/assets/images/demo.gif"' in result
    assert 'src="/pyxel/3.0.0/images/logo.png"' in result
    assert '"/pyxel/assets/pyxel.gpl"' in result
    assert '<iframe src="pyxel-screen">' in result
    assert 'resourceEditor.src = "pyxel-editor"' in result
    assert (
        'href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py"'
        in result
    )


def test_link_check_covers_json_rendered_images_and_directory_escapes(
    builder, tmp_path
):
    page = tmp_path / "pyxel/user-guide"
    page.mkdir(parents=True)
    (page / "index.html").write_text(
        '<a href="#section">OK</a><script src="https://example.org/a.js"></script>'
    )
    (page / "user-guide.json").write_text(
        '{"text": "<img src=\\"images/missing.png\\">"}'
    )
    with pytest.raises(ValueError, match="images/missing.png"):
        builder.verify_links(tmp_path)
    (page / "images").mkdir()
    (page / "images/missing.png").write_bytes(b"image")
    builder.verify_links(tmp_path)
    (page / "index.html").write_text('<a href="../../../outside.html">outside</a>')
    with pytest.raises(ValueError, match="outside.html"):
        builder.verify_links(tmp_path)


def test_link_check_resolves_cloudflare_html_paths(builder, tmp_path):
    page = tmp_path / "pyxel/code-maker"
    page.mkdir(parents=True)
    (page / "index.html").write_text('<iframe src="pyxel-screen"></iframe>')
    (page / "pyxel-screen.html").write_text("<!doctype html><title>Runtime</title>")
    builder.verify_links(tmp_path)
    (page / "pyxel-screen.html").unlink()
    with pytest.raises(ValueError, match="pyxel-screen"):
        builder.verify_links(tmp_path)


@pytest.mark.parametrize(
    ("destination", "message"),
    [
        (".", "must not replace the repository"),
        ("..", "must not replace the repository"),
        ("dist", "must be dist/site/"),
        ("web", "must be dist/site/"),
        ("wasm", "must be dist/site/"),
    ],
)
def test_output_cannot_replace_repository_files(
    builder, tmp_path, destination, message
):
    root = tmp_path / "repository"
    root.mkdir()
    with pytest.raises(ValueError, match=message):
        builder.build_site(root / destination, ["not-a-tag"], root=root)


def test_output_guards_match_letter_case_aliases(builder, tmp_path):
    root = tmp_path / "repository"
    root.mkdir()
    alias = tmp_path / "REPOSITORY"
    if not alias.exists():
        pytest.skip("the file system distinguishes letter case")
    with pytest.raises(ValueError, match="must not replace the repository"):
        builder.build_site(alias, ["not-a-tag"], root=root)
    with pytest.raises(ValueError, match="must be dist/site/"):
        builder.build_site(alias / "web", ["not-a-tag"], root=root)


@pytest.mark.parametrize(
    "kept", ["documents", "documents/notes.txt", "documents/pyxel/README.md"]
)
def test_output_must_be_empty_or_a_previous_build(builder, tmp_path, kept):
    # A lone pyxel directory, such as another checkout, is not a previous build.
    (tmp_path / kept).parent.mkdir(parents=True, exist_ok=True)
    (tmp_path / kept).write_text("keep")
    with pytest.raises(ValueError, match="previous site build"):
        builder.build_site(tmp_path / "documents", ["not-a-tag"])
    assert (tmp_path / kept).read_text() == "keep"
    assert sorted(path.name for path in tmp_path.iterdir()) == ["documents"]


@pytest.fixture
def repository(tmp_path):
    root = tmp_path / "repository"
    (root / "web").mkdir(parents=True)
    (root / "web/index.html").write_text("home")
    subprocess.run(["git", "init", "-q", str(root)], check=True)
    subprocess.run(["git", "-C", str(root), "add", "web"], check=True)
    return root


def test_public_sources_must_be_tracked(builder, repository):
    (repository / "web/local.js").write_text("untracked")
    assert builder.tracked_files(repository, "web") == [repository / "web/index.html"]
    with pytest.raises(ValueError, match="No public sources"):
        builder.tracked_files(repository, "web/local.js")


def test_public_sources_ignore_an_inherited_git_dir(
    builder, repository, tmp_path, monkeypatch
):
    subprocess.run(["git", "init", "-q", str(tmp_path / "other")], check=True)
    monkeypatch.setenv("GIT_DIR", str(tmp_path / "other" / ".git"))
    assert builder.tracked_files(repository, "web") == [repository / "web/index.html"]


@pytest.mark.skipif(sys.platform == "win32", reason="symlink may require elevation")
def test_public_sources_must_not_be_symlinks(builder, repository, tmp_path):
    (repository / "web").rename(tmp_path / "elsewhere")
    (repository / "web").symlink_to(tmp_path / "elsewhere")
    with pytest.raises(ValueError, match="symlinks"):
        builder.tracked_files(repository, "web")


def test_previous_release_cannot_change_or_disappear(builder, tmp_path):
    old = tmp_path / "old/pyxel/2.9.9"
    new = tmp_path / "new/pyxel/2.9.9"
    old.mkdir(parents=True)
    new.mkdir(parents=True)
    (old / "pyxel.js").write_text("published")
    (new / "pyxel.js").write_text("changed")
    with pytest.raises(ValueError, match="2.9.9"):
        builder.verify_releases(tmp_path / "new", tmp_path / "old")
    (new / "pyxel.js").write_text("published")
    builder.verify_releases(tmp_path / "new", tmp_path / "old")
    (new / "pyxel.js").unlink()
    with pytest.raises(ValueError, match="2.9.9"):
        builder.verify_releases(tmp_path / "new", tmp_path / "old")


PREVIOUS_BUILD = {"_headers": "previous headers", "pyxel/index.html": "previous site"}


@pytest.mark.parametrize(
    "files",
    [
        pytest.param({}, id="empty"),
        pytest.param(PREVIOUS_BUILD, id="build"),
        pytest.param(
            PREVIOUS_BUILD | {".DS_Store": "Finder"},
            id="build with every top-level file and .DS_Store",
        ),
    ],
)
def test_failed_build_preserves_a_replaceable_output(builder, tmp_path, files):
    output = tmp_path / "site"
    output.mkdir()
    for name, text in files.items():
        (output / name).parent.mkdir(parents=True, exist_ok=True)
        (output / name).write_text(text)
    # The runtime tag fails after the output check accepts the output.
    with pytest.raises(ValueError, match="release tag"):
        builder.build_site(output, ["not-a-tag"])
    assert {
        path.relative_to(output).as_posix(): path.read_text()
        for path in output.rglob("*")
        if path.is_file()
    } == files
    assert sorted(path.name for path in tmp_path.iterdir()) == ["site"]


def test_finder_files_do_not_change_a_previous_release(builder, tmp_path):
    old = tmp_path / "old/pyxel/2.9.9"
    new = tmp_path / "new/pyxel/2.9.9"
    old.mkdir(parents=True)
    new.mkdir(parents=True)
    (old / "pyxel.js").write_text("published")
    (old / ".DS_Store").write_text("Finder")
    (new / "pyxel.js").write_text("published")
    builder.verify_releases(tmp_path / "new", tmp_path / "old")


def test_provisional_runtime_can_be_replaced_but_not_removed(builder, tmp_path):
    old = tmp_path / "old/pyxel/3.0.0"
    new = tmp_path / "new/pyxel/3.0.0"
    old.mkdir(parents=True)
    new.mkdir(parents=True)
    (old / "pyxel.js").write_text("provisional runtime")
    (new / "pyxel.js").write_text("replacement runtime")
    builder.verify_releases(tmp_path / "new", tmp_path / "old")

    (new / "pyxel.js").unlink()
    with pytest.raises(ValueError, match="3.0.0"):
        builder.verify_releases(tmp_path / "new", tmp_path / "old")


@pytest.mark.parametrize("cached", [None, b"corrupt cache", b"runtime"])
def test_pyodide_distribution_checks_cache_before_reusing_it(
    builder, tmp_path, monkeypatch, cached
):
    root = tmp_path / "repository"
    (root / "web").mkdir(parents=True)
    (root / "wasm").mkdir()
    (root / "wasm/pyxel.js").write_text('const PYODIDE_VERSION = "314.0.7";\n')
    (root / "web/pyodide-files.json").write_text(
        json.dumps(
            {
                "314.0.7": {
                    "pyodide.js": {
                        "url": "https://example.invalid/pyodide.js",
                        "sha256": hashlib.sha256(b"runtime").hexdigest(),
                    }
                }
            }
        )
    )
    (root / "web/pyodide-NOTICE.txt").write_text("Upstream licenses and sources")
    cache = root / "dist/pyodide/v314.0.7/pyodide.js"
    if cached is not None:
        cache.parent.mkdir(parents=True)
        cache.write_bytes(cached)
    requests = []

    def download(url, timeout):
        requests.append(url)
        return io.BytesIO(b"runtime")

    monkeypatch.setattr(builder, "urlopen", download)
    site = tmp_path / "site"
    builder.copy_pyodide(root, site)
    assert (site / "pyodide/v314.0.7/pyodide.js").read_bytes() == b"runtime"
    assert (
        site / "pyodide/v314.0.7/NOTICE.txt"
    ).read_text() == "Upstream licenses and sources"
    assert len(requests) == (0 if cached == b"runtime" else 1)

    cache.write_bytes(b"corrupt cache")
    monkeypatch.setattr(
        builder, "urlopen", lambda *args, **kwargs: io.BytesIO(b"wrong download")
    )
    with pytest.raises(ValueError, match="checksum"):
        builder.copy_pyodide(root, site)
    assert (site / "pyodide/v314.0.7/pyodide.js").read_bytes() == b"runtime"

    (root / "wasm/pyxel.js").write_text('const PYODIDE_VERSION = "314.0.8";\n')
    with pytest.raises(ValueError, match="pinned distribution"):
        builder.copy_pyodide(root, site)


def test_previous_pyodide_release_cannot_change_or_disappear(builder, tmp_path):
    old = tmp_path / "old/pyxel/pyodide/v314.0.7"
    new = tmp_path / "new/pyxel/pyodide/v314.0.7"
    old.mkdir(parents=True)
    new.mkdir(parents=True)
    (old / "pyodide.js").write_text("published")
    (new / "pyodide.js").write_text("published")
    builder.verify_releases(tmp_path / "new", tmp_path / "old")
    (new / "pyodide.js").write_text("changed")
    with pytest.raises(ValueError, match="Pyodide release"):
        builder.verify_releases(tmp_path / "new", tmp_path / "old")
    (new / "pyodide.js").unlink()
    with pytest.raises(ValueError, match="Pyodide release"):
        builder.verify_releases(tmp_path / "new", tmp_path / "old")
