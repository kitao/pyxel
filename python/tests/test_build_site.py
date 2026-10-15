import importlib.util
import subprocess
import zipfile
from importlib.machinery import SourceFileLoader
from pathlib import Path

import pytest

ROOT_DIR = Path(__file__).parents[2]


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


@pytest.mark.parametrize("destination", [".", "dist", "web", "wasm"])
def test_output_cannot_replace_repository_files(builder, tmp_path, destination):
    root = tmp_path / "repository"
    root.mkdir()
    with pytest.raises(ValueError, match="output|Build output"):
        builder.build_site(root / destination, root=root)


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


def test_failed_build_preserves_existing_site(builder, tmp_path):
    output = tmp_path / "site"
    output.mkdir()
    (output / "index.html").write_text("previous site")
    with pytest.raises(ValueError, match="release tag"):
        builder.build_site(output, ["not-a-tag"])
    assert (output / "index.html").read_text() == "previous site"
    assert sorted(path.name for path in tmp_path.iterdir()) == ["site"]


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
