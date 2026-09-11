import importlib.util
import json
from importlib.machinery import SourceFileLoader
from pathlib import Path, PurePosixPath, PureWindowsPath

import pytest
from _assertions import raises_exact  # type: ignore[reportMissingImports]

MODULE_PATH = Path(__file__).parents[2] / "scripts" / "generate_docs"


@pytest.mark.parametrize("path_type", [PurePosixPath, PureWindowsPath])
def test_generated_header_uses_portable_source_paths(path_type):
    generate_docs = _load_generate_docs()
    generate_docs.ROOT_DIR = path_type("/pyxel")
    sources = [
        generate_docs.ROOT_DIR / "web/user-guide/index.html",
        generate_docs.ROOT_DIR / "web/user-guide/user-guide.json",
    ]

    assert generate_docs.generated_header(*sources) == (
        "<!-- This file is generated from web/user-guide/index.html and "
        "web/user-guide/user-guide.json. -->\n\n"
    )


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


@pytest.mark.parametrize("padding", ["", "\n  "])
@pytest.mark.parametrize("separator", ["", ", "])
def test_link_lists_preserve_pairs_and_separator(padding, separator):
    generate_docs = _load_generate_docs()
    evaluator = generate_docs.JsEval({}, {}, "user-guide")
    expression = (
        f'[{padding}["../api-reference/", "API"], '
        f'["../editor-manual/", "Editor"]{padding}]'
        ".map(([url, name]) => `${link(url, name)}`)"
        f'.join("{separator}")'
    )

    assert evaluator.eval(expression) == separator.join(
        [
            "[API](https://kitao.github.io/pyxel/web/api-reference/)",
            "[Editor](https://kitao.github.io/pyxel/web/editor-manual/)",
        ]
    )


def test_mapped_labels_preserve_separator():
    generate_docs = _load_generate_docs()
    evaluator = generate_docs.JsEval({}, {})
    assert evaluator.eval('["A", "B"].map(k => `${k}`).join(", ")') == "A, B"


def test_keyboard_diagram_renders_rest_as_text():
    generate_docs = _load_generate_docs()
    data = json.loads(generate_docs.EDITOR_MANUAL_JSON.read_text(encoding="utf-8"))
    evaluator = generate_docs.JsEval({}, data)
    assert evaluator.eval("keyboardDiagram()").endswith("\n\n**Rest:** A\n")


@pytest.mark.parametrize(
    ("expression", "expected"),
    [
        ('"Pyxel " + "Editor"', "Pyxel Editor"),
        ('"Ctrl+" + "C"', "Ctrl+C"),
        (r'"A\\\"+B" + "C"', 'A\\"+BC'),
    ],
)
def test_string_concatenation_preserves_literal_contents(expression, expected):
    generate_docs = _load_generate_docs()
    evaluator = generate_docs.JsEval({}, {})
    assert evaluator.eval(expression) == expected


def _load_generate_docs():
    loader = SourceFileLoader("generate_docs_test", str(MODULE_PATH))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    assert spec is not None
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module
