import json
import os
import random
import runpy
import sys
import zipfile
from pathlib import Path

import pyxel

# Subprocess entry for example/app/editor regression tests.
# Spawned by test_examples.py / test_apps.py / test_editor.py to give each test
# a fresh Python process (pyxel singleton can be initialized only once per process).
#
# Usage:
#     python _runner.py example      <script_path>  <plan_json> <out_dir>
#     python _runner.py flip_example <script_path>  <plan_json> <out_dir>
#     python _runner.py app          <pyxapp_path>  <plan_json> <out_dir>
#     python _runner.py editor       <editor_name>  <resource_file> <out_dir>


def _patch_init(*, extra=None):
    original_init = pyxel.init

    def patched(*args, **kwargs):
        kwargs["headless"] = True
        kwargs["fps"] = 1_000_000
        cwd = os.getcwd()
        original_init(*args, **kwargs)
        os.chdir(cwd)
        pyxel.rseed(0)
        pyxel.nseed(0)
        if extra is not None:
            extra()

    pyxel.init = patched


def _capture_frames(captured, plan, out_dir):
    current_frame = 0
    for step in plan:
        target = step["frame"]
        if "mouse" in step:
            x, y = step["mouse"]
            pyxel.set_mouse_pos(x, y)
        if "press" in step:
            for key in step["press"]:
                pyxel.set_btn(key, True)
        if "update" in captured:
            while current_frame < target:
                captured["update"]()
                captured["draw"]()
                pyxel.flip()
                current_frame += 1
        if step.get("capture", True):
            pyxel.screenshot(str(out_dir / f"frame_{target}.png"))
        if "press" in step:
            for key in step["press"]:
                pyxel.set_btn(key, False)


# --- example mode ---


def _run_example(script_path, plan, out_dir):
    captured = {}
    _patch_init()
    pyxel.run = lambda update, draw: captured.update(update=update, draw=draw)
    pyxel.show = lambda: None
    os.chdir(Path(script_path).parent)
    runpy.run_path(str(script_path), run_name="__main__")
    _capture_frames(captured, plan, out_dir)


# --- flip_example mode (while+flip() loop) ---


class _FlipCapture(Exception):
    # Raised inside patched flip() to break out of a while+flip() loop at the target frame.
    pass


def _run_flip_example(script_path, plan, out_dir):
    capture_at = {step["frame"] for step in plan}
    max_frame = max(capture_at)
    frame_count = [0]
    original_flip = pyxel.flip

    def patched_flip():
        original_flip()
        frame_count[0] += 1
        if frame_count[0] in capture_at:
            pyxel.screenshot(str(out_dir / f"frame_{frame_count[0]}.png"))
        if frame_count[0] >= max_frame:
            raise _FlipCapture()

    _patch_init()
    pyxel.flip = patched_flip
    os.chdir(Path(script_path).parent)
    try:
        runpy.run_path(str(script_path), run_name="__main__")
    except _FlipCapture:
        pass


# --- app mode ---


def _extract_pyxapp(pyxapp_path, extract_dir):
    with zipfile.ZipFile(pyxapp_path) as zf:
        zf.extractall(extract_dir)
    for setting_file in Path(extract_dir).glob(f"*/{pyxel.APP_STARTUP_SCRIPT_FILE}"):
        return str(
            setting_file.parent / setting_file.read_text(encoding="utf-8").strip()
        )
    sys.exit(f"No startup script found in {pyxapp_path}")


def _run_app(pyxapp_path, plan, out_dir):
    extract_dir = out_dir / "extract"
    extract_dir.mkdir()
    startup = _extract_pyxapp(pyxapp_path, extract_dir)

    captured = {}
    _patch_init(extra=lambda: random.seed(0))
    pyxel.run = lambda update, draw: captured.update(update=update, draw=draw)
    pyxel.show = lambda: None
    app_dir = str(Path(startup).parent)
    sys.path.insert(0, app_dir)
    os.chdir(app_dir)
    runpy.run_path(startup, run_name="__main__")
    _capture_frames(captured, plan, out_dir)


# --- editor mode (manual frame stepping, no pyxel.run loop) ---


def _editor_frame(captured):
    captured["update"]()
    captured["draw"]()
    pyxel.flip()


def _editor_press(captured, *keys):
    for k in keys:
        pyxel.set_btn(k, True)
    _editor_frame(captured)
    for k in keys:
        pyxel.set_btn(k, False)
    _editor_frame(captured)


def _editor_click(captured, x, y, button=None):
    if button is None:
        button = pyxel.MOUSE_BUTTON_LEFT
    pyxel.set_mouse_pos(x, y)
    pyxel.set_btn(button, True)
    _editor_frame(captured)
    pyxel.set_btn(button, False)
    _editor_frame(captured)


def _editor_capture(captured, path):
    _editor_frame(captured)
    pyxel.screenshot(str(path))


def _run_editor(editor, resource_file, out_dir):
    from pyxel import editor as pyxel_editor

    captured = {}
    _patch_init(extra=lambda: pyxel.set_mouse_pos(0, 0))
    pyxel.run = lambda update, draw: captured.update(update=update, draw=draw)
    pyxel_editor.App(resource_file, editor)

    _editor_frame(captured)
    pyxel.screenshot(str(out_dir / "f1.png"))

    if editor == "image":
        _editor_press(captured, pyxel.KEY_B)
        _editor_press(captured, pyxel.KEY_3)
        _editor_click(captured, 76, 81)
    elif editor == "tilemap":
        _editor_click(captured, 15, 136, pyxel.MOUSE_BUTTON_RIGHT)
        _editor_press(captured, pyxel.KEY_B)
        _editor_click(captured, 67, 40)
    elif editor == "sound":
        for key in [pyxel.KEY_Z, pyxel.KEY_X, pyxel.KEY_C, pyxel.KEY_V, pyxel.KEY_B]:
            _editor_press(captured, key, pyxel.KEY_RETURN)
    elif editor == "music":
        for mx in [82, 95, 108]:
            _editor_click(captured, mx, 138)

    _editor_capture(captured, out_dir / "fedit.png")


# --- dispatch ---


def main():
    mode = sys.argv[1]
    out_dir = Path(sys.argv[-1])
    if mode == "example":
        _run_example(sys.argv[2], json.loads(sys.argv[3]), out_dir)
    elif mode == "flip_example":
        _run_flip_example(sys.argv[2], json.loads(sys.argv[3]), out_dir)
    elif mode == "app":
        _run_app(sys.argv[2], json.loads(sys.argv[3]), out_dir)
    elif mode == "editor":
        _run_editor(sys.argv[2], sys.argv[3], out_dir)
    else:
        sys.exit(f"Unknown mode: {mode}")


if __name__ == "__main__":
    main()
