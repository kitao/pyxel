import os
from pathlib import Path

import pytest

import pyxel
import pyxel.editor

from test_examples import (
    _reset_pyxel,
    _restore_pyxel,
    compare_or_update_all,
)

REFERENCES_DIR = Path(__file__).parent / "references"
EDITOR_REFS_DIR = REFERENCES_DIR / "editor"
RESOURCE_FILE = str(
    Path(__file__).parent.parent / "pyxel" / "examples" / "assets" / "sample.pyxres"
)

EDITORS = ["image", "tilemap", "sound", "music"]


def run_editor(starting_editor):
    captured = {}
    original_init = pyxel.init
    original_run = pyxel.run

    def patched_init(*args, **kwargs):
        kwargs["headless"] = True
        kwargs["fps"] = 1_000_000
        cwd = os.getcwd()
        original_init(*args, **kwargs)
        os.chdir(cwd)
        pyxel.rseed(0)
        pyxel.nseed(0)
        pyxel.set_mouse_pos(0, 0)

    def patched_run(update, draw):
        captured["update"] = update
        captured["draw"] = draw

    pyxel.init = patched_init
    pyxel.run = patched_run
    try:
        pyxel.editor.App(RESOURCE_FILE, starting_editor)
    finally:
        pyxel.init = original_init
        pyxel.run = original_run
    return captured


def _frame(captured):
    captured["update"]()
    captured["draw"]()
    pyxel.flip()


def _press(captured, *keys):
    for k in keys:
        pyxel.set_btn(k, True)
    _frame(captured)
    for k in keys:
        pyxel.set_btn(k, False)
    _frame(captured)


def _click(captured, x, y, button=None):
    if button is None:
        button = pyxel.MOUSE_BUTTON_LEFT
    pyxel.set_mouse_pos(x, y)
    pyxel.set_btn(button, True)
    _frame(captured)
    pyxel.set_btn(button, False)
    _frame(captured)


def _capture(captured, path):
    _frame(captured)
    pyxel.screenshot(str(path))


def run_editor_edit(editor, captured, tmp_path):
    results = []

    # Capture initial state
    _frame(captured)
    p = tmp_path / "f1.png"
    pyxel.screenshot(str(p))
    results.append((1, p))

    if editor == "image":
        _press(captured, pyxel.KEY_B)  # Bucket tool
        _press(captured, pyxel.KEY_3)  # Color 3
        _click(captured, 76, 81)  # Click canvas center
        p = tmp_path / "f_edit.png"
        _capture(captured, p)
        results.append(("edit", p))

    elif editor == "tilemap":
        _click(captured, 15, 136, pyxel.MOUSE_BUTTON_RIGHT)  # Spoit brick tile
        _press(captured, pyxel.KEY_B)  # Bucket tool
        _click(captured, 67, 40)  # Fill empty area
        p = tmp_path / "f_edit.png"
        _capture(captured, p)
        results.append(("edit", p))

    elif editor == "sound":
        for key in [pyxel.KEY_Z, pyxel.KEY_X, pyxel.KEY_C, pyxel.KEY_V, pyxel.KEY_B]:
            _press(captured, key, pyxel.KEY_RETURN)
        p = tmp_path / "f_edit.png"
        _capture(captured, p)
        results.append(("edit", p))

    elif editor == "music":
        for mx in [82, 95, 108]:
            _click(captured, mx, 138)
        p = tmp_path / "f_edit.png"
        _capture(captured, p)
        results.append(("edit", p))

    return results


class TestEditor:
    @pytest.mark.parametrize("editor", EDITORS, ids=EDITORS)
    def test_editor(self, editor, tmp_path, update_references):
        _reset_pyxel()
        try:
            captured = run_editor(editor)
            results = run_editor_edit(editor, captured, tmp_path)
            compare_or_update_all(
                f"editor_{editor}", results, EDITOR_REFS_DIR, update_references
            )
        finally:
            _restore_pyxel()
