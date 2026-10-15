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
            "[API](https://kitao.dev/pyxel/api-reference/)",
            "[Editor](https://kitao.dev/pyxel/editor-manual/)",
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


@pytest.mark.parametrize(
    ("web_path", "url", "expected"),
    [
        (
            "user-guide/cube",
            "../#s-dist",
            "https://kitao.dev/pyxel/user-guide/#s-dist",
        ),
        (
            "user-guide/cube",
            "../../showcase/",
            "https://kitao.dev/pyxel/showcase/",
        ),
        (
            "user-guide/cube",
            "images/coordinates-en.svg",
            "https://kitao.dev/pyxel/user-guide/cube/images/coordinates-en.svg",
        ),
        (
            "user-guide/cube",
            "#s-models",
            "https://kitao.dev/pyxel/user-guide/cube/#s-models",
        ),
        ("user-guide/cube", "../../../docs/pyxel.gpl", "pyxel.gpl"),
        ("user-guide", "../../docs/images/palette.png", "images/palette.png"),
        (
            "editor-manual",
            "../api-reference/",
            "https://kitao.dev/pyxel/api-reference/",
        ),
        (
            "user-guide/cube",
            "https://example.com/model.glb",
            "https://example.com/model.glb",
        ),
    ],
)
def test_relative_links_resolve_from_the_containing_page(web_path, url, expected):
    generate_docs = _load_generate_docs()
    assert generate_docs._transform_url(url, web_path) == expected


def test_detailed_guide_selection_preserves_content(tmp_path):
    generate_docs = _load_generate_docs()
    generate_docs.ROOT_DIR = tmp_path
    generate_docs.DOCS_DIR = tmp_path / "docs"
    generate_docs.DOCS_DIR.mkdir()
    html_path = tmp_path / "page.html"
    html_path.write_text(
        """function updateTexts() {
          document.getElementById("overview").innerHTML = `<p>${t("intro")}</p>`;
          const guide = data.guides.find((guide) => guide.language === lang);
          document.getElementById("guide").innerHTML = renderGuide(guide);
        }""",
        encoding="utf-8",
    )
    program = 'def draw():\n    text = "<p>literal</p>"\n    return text'
    guide = {
        "language": "en",
        "sections": [
            {
                "title": "Detailed guide",
                "blocks": [
                    {"text": "<h3>Subtopic</h3><h4>Detail</h4>"},
                    {
                        "text": '<ul><li><a href="../#s-dist">Package</a> <code>pyxel package</code></li></ul>'
                    },
                    {
                        "text": "<table><tr><th>Asset</th><th>Setting</th></tr><tr><td>Motion</td><td>Enabled when animated</td></tr></table>"
                    },
                    {"text": '<img src="images/mesh.svg" alt="Mesh hierarchy">'},
                    {"code": program, "syntax": "python"},
                ],
            }
        ],
    }
    json_path = tmp_path / "page.json"
    json_path.write_text(
        json.dumps(
            {
                "ui": {"title": {"en": "Example"}, "intro": {"en": "Overview"}},
                "guides": [guide],
            }
        ),
        encoding="utf-8",
    )

    generate_docs.generate_from_html(html_path, json_path, "guide", "user-guide/cube")
    result = (generate_docs.DOCS_DIR / "guide.md").read_text()
    assert "Overview" in result
    assert "## Detailed guide" in result
    assert "### Subtopic\n" in result
    assert "#### Detail\n" in result
    assert (
        "- [Package](https://kitao.dev/pyxel/user-guide/#s-dist) `pyxel package`"
        in result
    )
    assert (
        "| Asset | Setting |\n| --- | --- |\n| Motion | Enabled when animated |"
        in result
    )
    assert (
        "![Mesh hierarchy](https://kitao.dev/pyxel/user-guide/cube/images/mesh.svg)"
        in result
    )
    assert f"```python\n{program}\n```" in result


def test_linked_images_follow_the_existing_illustration_selection():
    generate_docs = _load_generate_docs()
    source = (
        '<a href="https://example.com/book"><img src="../../docs/images/pyxel_book.png" alt="Book"></a>'
        '<a href="../api-reference/"><img src="../../docs/images/palette.png" alt="Palette"></a>'
        '<a href="../editor-manual/">Editor</a>'
    )
    result = generate_docs._html_to_markdown(source, "user-guide", "user-guide")
    assert "example.com/book" not in result
    assert "Book" not in result
    assert (
        "[![Palette](images/palette.png)](https://kitao.dev/pyxel/api-reference/)"
        in result
    )
    assert "[Editor](https://kitao.dev/pyxel/editor-manual/)" in result


def test_ordered_instructions_preserve_steps_and_inline_code():
    generate_docs = _load_generate_docs()
    result = generate_docs._html_to_markdown(
        '<ol class="steps"><li>Open <code>model.bbmodel</code>.</li>'
        "<li>Choose <strong>Export</strong>.</li></ol>",
        "user-guide-cube",
        "user-guide/cube",
    )
    assert result == "\n1. Open `model.bbmodel`.\n2. Choose **Export**.\n"


def _load_generate_docs():
    loader = SourceFileLoader("generate_docs_test", str(MODULE_PATH))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    assert spec is not None
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module
