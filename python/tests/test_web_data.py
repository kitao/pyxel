import json
import re
from pathlib import Path

ROOT_DIR = Path(__file__).parents[2]


def test_showcase_entries_point_to_existing_files():
    showcase = json.loads(
        (ROOT_DIR / "web/showcase/showcase.json").read_text(encoding="utf-8")
    )
    missing = []

    for category in showcase["categories"]:
        for entry in category["items"]:
            demo = entry.get("demo")
            assert isinstance(demo, str) and demo.strip(), (category["id"], entry)
            if (
                not demo.startswith(("https://", "http://"))
                and not (ROOT_DIR / "web/showcase" / demo).is_file()
            ):
                missing.append(demo)
            name = entry["file"]
            assert isinstance(name, str) and name.strip(), (category["id"], entry)
            if category["id"] in ("examples", "cube-examples"):
                prefix = "https://github.com/kitao/pyxel/blob/main/"
                assert entry["code"].startswith(prefix), entry
                source = ROOT_DIR / entry["code"].removeprefix(prefix)
                assert name == source.name, entry
                if not source.is_file():
                    missing.append(entry["code"])

    assert missing == []


def test_editor_manual_images_exist():
    page = (ROOT_DIR / "web/editor-manual/index.html").read_text(encoding="utf-8")
    data = json.loads(
        (ROOT_DIR / "web/editor-manual/editor-manual.json").read_text(encoding="utf-8")
    )
    local_image = r"(?<![\w/.-])images/[\w.-]+\.(?:png|gif)"
    images = set(re.findall(local_image, page))
    images.update(re.findall(local_image, json.dumps(data)))
    for editor, index in re.findall(r'areaCrop\("(\w+)", (\d+)\)', page):
        images.add(
            f"images/{editor}_{data['annotations'][editor][int(index)]['id']}.png"
        )
    for group in ("menubar_buttons", "image_tools", "playback_buttons"):
        images.update(row["icon"] for row in data[group])

    assert len(images) > 5
    assert [
        name
        for name in sorted(images)
        if not (ROOT_DIR / "web/editor-manual" / name).is_file()
    ] == []
