import json
import re
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


def iter_json_objects(value):
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from iter_json_objects(child)
    elif isinstance(value, list):
        for child in value:
            yield from iter_json_objects(child)
