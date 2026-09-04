import importlib.util
from importlib.machinery import SourceFileLoader
from pathlib import Path

import pytest

MODULE_PATH = Path(__file__).parents[2] / "scripts" / "generate_pyi_docstrings"


def _load_generate_pyi_docstrings():
    loader = SourceFileLoader("generate_pyi_docstrings_test", str(MODULE_PATH))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    assert spec is not None
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


def test_add_docstrings_normalizes_overload_groups():
    generator = _load_generate_pyi_docstrings()
    content = """\
from typing import overload

@overload
def clip() -> None: ...
@overload
def clip(x: int) -> None: ...
def clip(x: int | None = None) -> None: ...

"""
    doc_map = {
        (None, "clip"): {
            "description": {"en": "Set the clipping area."},
            "params": [
                {
                    "name": "x",
                    "description": {"en": "X coordinate"},
                }
            ],
        }
    }

    result = generator._add_docstrings(content, doc_map)

    assert result.count("def clip(") == 2
    assert result.count("Set the clipping area.") == 1
    assert "def clip(x: int | None = None)" not in result
    assert 'def clip() -> None:\n    """Set the clipping area.' in result
    assert "def clip(x: int) -> None: ...\n\n" not in result


@pytest.mark.parametrize("class_name", ["Image", "Tilemap"])
def test_class_fallback_adapts_destination_without_mutating_module(class_name):
    generator = _load_generate_pyi_docstrings()
    source = {
        "description": {"en": "Clear the screen with color col."},
        "params": [{"name": "col", "description": {"en": "Drawing color"}}],
    }
    item = generator._fallback_item(class_name, "cls", {(None, "cls"): source})

    assert source == {
        "description": {"en": "Clear the screen with color col."},
        "params": [{"name": "col", "description": {"en": "Drawing color"}}],
    }
    if class_name == "Image":
        assert item == {
            "description": {"en": "Clear the image with color col."},
            "params": [{"name": "col", "description": {"en": "Drawing color"}}],
        }
    else:
        assert item == {
            "description": {"en": "Clear the tilemap with tile."},
            "params": [
                {"name": "tile", "description": {"en": "Tile (image_tx, image_ty)"}}
            ],
        }
