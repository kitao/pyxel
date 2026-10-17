import json
import os
import subprocess
import sys
from html.parser import HTMLParser
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
SAMPLES = {
    "c02_basic_shapes": ["SPACE"],
    "c04_mesh_and_motion": ["2", "RIGHT", "UP", "3", "S", "1"],
    "c05_3d_collision": ["RIGHT", "UP", "SPACE", "SPACE", "S"],
    "c06_3d_physics": ["RIGHT", "UP", "SPACE", "R", "SPACE"],
}


class LauncherParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.launchers = []

    def handle_starttag(self, tag, attrs):
        if tag == "pyxel-run":
            self.launchers.append(dict(attrs))


@pytest.mark.parametrize("name", SAMPLES)
def test_interactive_cube_page_enables_touch_gamepad(name):
    page = ROOT / "web/showcase/examples" / (name.replace("_", "-") + ".html")
    parser = LauncherParser()
    parser.feed(page.read_text())
    assert len(parser.launchers) == 1
    assert parser.launchers[0]["name"] == f"cube/{name}.py"
    assert parser.launchers[0].get("gamepad") == "enabled"


@pytest.mark.parametrize("name,actions", SAMPLES.items())
def test_cube_gamepad_matches_keyboard_actions(name, actions):
    code = """
import hashlib
import json
import os
import runpy
import sys
from pathlib import Path

import pyxel

script, mode, actions = sys.argv[1:]
script = Path(script)
os.chdir(script.parent)
pyxel.rseed(0)
pyxel.nseed(0)
init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(
    *args, **{**kwargs, "headless": True, "fps": 1_000_000}
)
callbacks = {}
pyxel.run = lambda update, draw: callbacks.update(update=update, draw=draw)
runpy.run_path(str(script))
app = callbacks["update"].__self__
buttons = {"RIGHT": "DPAD_RIGHT", "UP": "DPAD_UP", "SPACE": "A"}
if script.stem == "c04_mesh_and_motion":
    buttons.update({"1": "A", "2": "B", "3": "X", "S": "Y"})
else:
    buttons.update({"S": "B", "R": "B"})

# Gamepad fire must aim at the center even when the last pointer was elsewhere.
if script.stem == "c06_3d_physics":
    pyxel.set_mouse_pos(*((7, 9) if mode == "gamepad" else (160, 120)))
    app.mouse_pos = (pyxel.mouse_x, pyxel.mouse_y)
    app.gamepad = mode == "gamepad"

captures = []
for action in json.loads(actions):
    key = getattr(pyxel, "KEY_" + action)
    button = getattr(pyxel, "GAMEPAD1_BUTTON_" + buttons[action])
    pressed = button if mode == "gamepad" else key
    pyxel.set_btn(pressed, True)
    for frame in range(12):
        callbacks["update"]()
        callbacks["draw"]()
        pyxel.flip()
    pixels = bytes(pyxel.pget(x, y) for y in range(pyxel.height) for x in range(pyxel.width))
    captures.append(hashlib.sha256(pixels).hexdigest())
    pyxel.set_btn(pressed, False)
    callbacks["update"]()
    pyxel.flip()

if script.stem == "c02_basic_shapes":
    pyxel.set_btn(pyxel.GAMEPAD1_BUTTON_DPAD_LEFT, True)
    pyxel.set_btn(pyxel.GAMEPAD1_BUTTON_DPAD_UP, True)
    callbacks["update"]()
    assert app.scene.yaw > 0 and app.scene.pitch < 0
elif script.stem == "c05_3d_collision":
    assert app.player.transform.pos != app.player.start
elif script.stem == "c06_3d_physics":
    assert app.scene.shot_count == 2  # Holding A/Space repeats at the same interval.
    if mode == "gamepad":
        assert app.gamepad
        pyxel.set_mouse_pos(20, 30)
        callbacks["update"]()
        assert not app.gamepad

print(json.dumps(captures))
"""
    results = []
    for mode in ("keyboard", "gamepad"):
        process = subprocess.run(
            [
                sys.executable,
                "-c",
                code,
                str(ROOT / "python/pyxel/examples/cube" / f"{name}.py"),
                mode,
                json.dumps(actions),
            ],
            env={**os.environ, "SDL_AUDIODRIVER": "dummy"},
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
        assert process.returncode == 0, process.stdout + process.stderr
        results.append(json.loads(process.stdout.strip().splitlines()[-1]))
    assert results[0] == results[1]
