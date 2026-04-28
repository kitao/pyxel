import colorsys
import shutil
from pathlib import Path

import pytest

import pyxel
from pyxel import editor as _editor  # noqa: F401  # extension load attaches pyxel.user_pal

from _capture import (  # type: ignore[reportMissingImports]
    EDITOR_REFS_DIR,
    collect_editor_results,
    run_editor_subprocess,
)

_ = _editor  # mark as used (extension side-effect)

RESOURCE_FILE = str(
    Path(__file__).parent.parent / "pyxel" / "examples" / "assets" / "sample.pyxres"
)

# (editor, palette_count_or_None)
# palette_count=None: use the resource file's default palette (no extra .pyxpal)
# palette_count=N: write an HSV-spread .pyxpal alongside the .pyxres so that
# pyxel.load() auto-loads it and the user palette becomes N colors
_EDITOR_PALETTE_PARAMS = [
    ("image", None),
    ("tilemap", None),
    ("sound", None),
    ("music", None),
    ("image", 16),
    ("image", 32),
    ("image", 64),
]


def _param_id(editor, palette_count):
    if palette_count is None:
        return editor
    return f"{editor}_{palette_count}colors"


def _hsv_pyxpal_lines(count):
    lines = []
    for i in range(count):
        r, g, b = colorsys.hsv_to_rgb(i / count, 0.8, 1.0)
        lines.append(f"{int(r * 255):02x}{int(g * 255):02x}{int(b * 255):02x}")
    return lines


class TestEditor:
    @pytest.mark.parametrize(
        "editor,palette_count",
        _EDITOR_PALETTE_PARAMS,
        ids=[_param_id(e, p) for e, p in _EDITOR_PALETTE_PARAMS],
    )
    def test_editor(self, editor, palette_count, tmp_path, compare_screenshots):
        if palette_count is None:
            resource = RESOURCE_FILE
            ref_name = f"editor_{editor}"
        else:
            pyxres = tmp_path / "test.pyxres"
            shutil.copy(RESOURCE_FILE, pyxres)
            pyxpal = tmp_path / "test.pyxpal"
            pyxpal.write_text(
                "\n".join(_hsv_pyxpal_lines(palette_count)) + "\n", encoding="utf-8"
            )
            resource = str(pyxres)
            ref_name = f"editor_{editor}_{palette_count}colors"

        run_editor_subprocess(editor, resource, tmp_path)
        results = collect_editor_results(tmp_path)
        compare_screenshots(ref_name, results, EDITOR_REFS_DIR)


class TestUserPal:
    def test_user_pal_maps_all_user_colors(self):
        saved_colors = list(pyxel.colors)
        saved_num_user = getattr(pyxel, "num_user_colors", None)
        try:
            user_colors = [0x100000 + i for i in range(64)]
            pyxel.colors[:] = saved_colors + user_colors
            pyxel.num_user_colors = len(user_colors)  # type: ignore[attr-defined]

            pyxel.user_pal()  # type: ignore[attr-defined]
            try:
                for i in range(pyxel.num_user_colors):  # type: ignore[attr-defined]
                    pyxel.cls(0)
                    pyxel.rect(0, 0, 1, 1, i)
                    assert pyxel.pget(0, 0) == pyxel.NUM_COLORS + i
            finally:
                pyxel.pal()
        finally:
            pyxel.colors[:] = saved_colors
            if saved_num_user is None:
                if hasattr(pyxel, "num_user_colors"):
                    delattr(pyxel, "num_user_colors")
            else:
                pyxel.num_user_colors = saved_num_user  # type: ignore[attr-defined]
