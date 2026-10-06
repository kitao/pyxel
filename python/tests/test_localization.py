import json
import re
from html.parser import HTMLParser
from pathlib import Path

ROOT_DIR = Path(__file__).parents[2]


def test_localized_text_entries_are_complete_and_keep_placeholders():
    source_paths = (
        "web/api-reference/api-reference.json",
        "web/api-reference/cube/api-reference.json",
        "web/code-maker/manual.json",
        "web/editor-manual/editor-manual.json",
        "web/launcher/launcher.json",
        "web/mml-studio/manual.json",
        "web/mml-studio/mml-commands.json",
        "web/showcase/showcase.json",
        "web/user-guide/cube/user-guide.json",
        "web/user-guide/user-guide.json",
        "web/web-usage/web-usage.json",
    )

    for relative_path in source_paths:
        path = ROOT_DIR / relative_path
        document = json.loads(path.read_text(encoding="utf-8"))
        language_codes = {language["code"] for language in document["languages"]}

        for value in iter_json_objects(document):
            keys = set(value)
            if not keys & language_codes or not any(
                isinstance(text, str) for text in value.values()
            ):
                continue

            assert keys == language_codes, (path, keys)
            assert all(isinstance(text, str) for text in value.values()), (path, value)
            placeholder_sets = {
                tuple(sorted(re.findall(r"\{\d+\}", text))) for text in value.values()
            }
            assert len(placeholder_sets) == 1, (path, value)


def test_cube_guides_keep_sections_examples_and_navigation_in_sync():
    path = ROOT_DIR / "web/user-guide/cube/user-guide.json"
    document = json.loads(path.read_text(encoding="utf-8"))
    guides = document["guides"]
    assert [guide["language"] for guide in guides] == [
        language["code"] for language in document["languages"]
    ]
    source = next(guide for guide in guides if guide["language"] == "ja")
    section_ids = [section["id"] for section in source["sections"]]

    for guide in guides:
        assert [section["id"] for section in guide["sections"]] == section_ids
        for section, source_section in zip(guide["sections"], source["sections"]):
            context = (guide["language"], section["id"])
            examples = [
                block["code"]
                for block in section["blocks"]
                if block.get("syntax") == "python"
            ]
            source_examples = [
                block["code"]
                for block in source_section["blocks"]
                if block.get("syntax") == "python"
            ]
            assert examples == source_examples, context
            navigation = GuideNavigation()
            source_navigation = GuideNavigation()
            for block in section["blocks"]:
                navigation.feed(block.get("text", ""))
            for block in source_section["blocks"]:
                source_navigation.feed(block.get("text", ""))
            assert navigation.targets == source_navigation.targets, context
            assert navigation.ordered_steps == source_navigation.ordered_steps, context


class GuideNavigation(HTMLParser):
    def __init__(self):
        super().__init__()
        self.targets = []
        self.ordered_steps = []
        self.in_ordered_list = False

    def handle_starttag(self, tag, attrs):
        for name, value in attrs:
            if name in ("id", "href", "src"):
                self.targets.append((tag, name, value))
        if tag == "ol":
            self.in_ordered_list = True
            self.ordered_steps.append(0)
        elif tag == "li" and self.in_ordered_list:
            self.ordered_steps[-1] += 1

    def handle_endtag(self, tag):
        if tag == "ol":
            self.in_ordered_list = False


def iter_json_objects(value):
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from iter_json_objects(child)
    elif isinstance(value, list):
        for child in value:
            yield from iter_json_objects(child)
