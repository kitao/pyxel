import json
import re
from pathlib import Path

ROOT_DIR = Path(__file__).parents[2]
EXAMPLES_DIR = ROOT_DIR / "python" / "pyxel" / "examples"


def test_showcase_entries_point_to_existing_files():
    showcase = json.loads(
        (ROOT_DIR / "web/showcase/showcase.json").read_text(encoding="utf-8")
    )
    missing = []

    for entry in iter_entries(showcase):
        demo = entry["demo"]
        if (
            not demo.startswith("http")
            and not (ROOT_DIR / "web/showcase" / demo).is_file()
        ):
            missing.append(demo)
        # `file` names the packaged script of an example; for apps and tools it is a label.
        name = entry.get("file", "")
        if name.endswith(".py") and not any(
            (directory / name).is_file()
            for directory in (EXAMPLES_DIR, EXAMPLES_DIR / "cube")
        ):
            missing.append(name)

    assert missing == []


def test_editor_manual_images_exist():
    page = (ROOT_DIR / "web/editor-manual/index.html").read_text(encoding="utf-8")
    data = json.loads(
        (ROOT_DIR / "web/editor-manual/editor-manual.json").read_text(encoding="utf-8")
    )
    images = set(re.findall(r"images/[\w.-]+\.(?:png|gif)", page))
    images.update(re.findall(r"images/[\w.-]+\.(?:png|gif)", json.dumps(data)))
    for editor, index in re.findall(r'areaCrop\("(\w+)", (\d+)\)', page):
        images.add(
            f"images/{editor}_{data['annotations'][editor][int(index)]['id']}.png"
        )
    tool_icons = re.search(r"const icons = \[(.*?)\];", page, re.DOTALL).group(1)
    images.update(
        f"images/tool_{name}.png" for name in re.findall(r'"(\w+)"', tool_icons)
    )

    assert len(images) > 5
    assert [
        name
        for name in sorted(images)
        if not (ROOT_DIR / "web/editor-manual" / name).is_file()
    ] == []


def iter_entries(value):
    if isinstance(value, dict):
        if isinstance(value.get("demo"), str):
            yield value
        for child in value.values():
            yield from iter_entries(child)
    elif isinstance(value, list):
        for child in value:
            yield from iter_entries(child)
