import importlib.util
from importlib.machinery import SourceFileLoader
from pathlib import Path

from _assertions import raises_exact  # type: ignore[reportMissingImports]


MODULE_PATH = Path(__file__).parents[2] / "scripts" / "generate_docs"


def _load_generate_docs():
    loader = SourceFileLoader("generate_docs_test", str(MODULE_PATH))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    assert spec is not None
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


def test_generate_from_html_rejects_missing_update_texts(tmp_path):
    generate_docs = _load_generate_docs()
    generate_docs.DOCS_DIR = tmp_path / "docs"
    generate_docs.DOCS_DIR.mkdir()
    html_path = tmp_path / "broken.html"
    html_path.write_text("<html><body>No updater</body></html>", encoding="utf-8")
    json_path = tmp_path / "broken.json"
    json_path.write_text('{"ui":{"title":{"en":"Test"}}}', encoding="utf-8")

    with raises_exact(
        ValueError, f"{html_path}: required function updateTexts() not found"
    ):
        generate_docs.generate_from_html(html_path, json_path, "broken")

    assert not (generate_docs.DOCS_DIR / "broken.md").exists()
