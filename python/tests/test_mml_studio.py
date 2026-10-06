import os
import subprocess
import sys
from pathlib import Path

import pytest


@pytest.mark.parametrize("play,stop", [(True, False), (True, True), (False, False)])
def test_mml_studio_native_request_consumption(play, stop):
    code = """
import runpy
import sys
from types import SimpleNamespace

import pyxel

play, stop = sys.argv[2] == "1", sys.argv[3] == "1"
window = SimpleNamespace(js_play=play, js_stop=stop, js_loop=False)
# Keep playback active throughout the child timeout.
for i in range(1, 5):
    setattr(window, f"js_ch{i}_mml", "T10 L1 C")
sys.modules["js"] = SimpleNamespace(window=window)
init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
callbacks = []
pyxel.run = lambda update, draw: callbacks.append(update)

runpy.run_path(sys.argv[1])
assert len(callbacks) == 1
expected = play and not stop
assert all((pyxel.play_pos(i) is not None) == expected for i in range(4))
assert window.js_stop is False

if expected:
    window.js_loop = True
    callbacks[0]()
    assert callbacks[0].__self__.loop_enabled is True

    window.js_solo2 = True
    window.js_mute2 = True
    callbacks[0]()
    assert all(pyxel.channels[i].gain == 0 for i in range(4))

window.js_stop = True
callbacks[0]()
assert window.js_stop is False
assert all(pyxel.play_pos(i) is None for i in range(4))
"""
    source = Path(__file__).parents[2] / "web" / "mml-studio" / "mml_studio.py"
    result = subprocess.run(
        [
            sys.executable,
            "-B",
            "-c",
            code,
            str(source),
            "1" if play else "0",
            "1" if stop else "0",
        ],
        env={**os.environ, "SDL_AUDIODRIVER": "dummy"},
        capture_output=True,
        text=True,
        timeout=10,
        check=False,
    )
    assert result.returncode == 0, result.stderr
    assert "Failed to initialize audio device" not in result.stdout
