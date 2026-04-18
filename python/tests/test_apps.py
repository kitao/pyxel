import os
import random
import runpy
import sys
import zipfile
from pathlib import Path

import pytest
import pyxel

from test_examples import (
    _reinit_pyxel,
    _restore_pyxel,
    capture_frames,
    compare_or_update_all,
)

REFERENCES_DIR = Path(__file__).parent / "references"
APPS_DIR = Path(__file__).parent.parent / "pyxel" / "examples" / "apps"
APP_REFS_DIR = REFERENCES_DIR / "apps"

CAPTURE_PLANS = {
    # ENTER opens menu, second ENTER starts game
    "megaball": [
        {"frame": 30},
        {"frame": 31, "press": [pyxel.KEY_RETURN], "capture": False},
        {"frame": 35, "press": [pyxel.KEY_RETURN], "capture": False},
        {"frame": 90},
    ],
    # ENTER starts game, capture before game over with enemies visible
    "mega_wing": [
        {"frame": 30},
        {"frame": 31, "press": [pyxel.KEY_RETURN], "capture": False},
        {"frame": 150},
    ],
    "space_rescue": [
        {"frame": 30},
        {"frame": 31, "press": [pyxel.KEY_RETURN], "capture": False},
        {"frame": 60},
    ],
    # Title fade-in ~67 frames (alpha += 0.015)
    "cursed_caverns": [
        {"frame": 70},
        {"frame": 71, "press": [pyxel.KEY_RETURN], "capture": False},
        {"frame": 100},
    ],
    "30sec_of_daylight": [
        {"frame": 30},
        {"frame": 31, "press": [pyxel.KEY_RETURN], "capture": False},
        {"frame": 60},
    ],
    # Title animation: TOP(40) + TITLE(70) + BOTTOM(95) = 205 frames
    "laser-jetman": [
        {"frame": 210},
        {"frame": 211, "press": [pyxel.KEY_RETURN], "capture": False},
        {"frame": 270},
    ],
    # KEY_Z starts game, wait longer for enemies
    "vortexion": [
        {"frame": 30},
        {"frame": 31, "press": [pyxel.KEY_Z], "capture": False},
        {"frame": 200},
    ],
}


def extract_pyxapp(pyxapp_path, extract_dir):
    """Extract .pyxapp ZIP and return path to startup script."""
    with zipfile.ZipFile(pyxapp_path) as zf:
        zf.extractall(extract_dir)

    for setting_file in Path(extract_dir).glob(f"*/{pyxel.APP_STARTUP_SCRIPT_FILE}"):
        with open(setting_file) as f:
            return str(setting_file.parent / f.read().strip())
    pytest.fail(f"No startup script found in {pyxapp_path}")


def run_pyxapp(startup_path):
    """Execute .pyxapp startup script, capturing update/draw callbacks."""
    captured = {}
    original_init = pyxel.init
    original_run = pyxel.run
    original_show = pyxel.show

    def patched_init(*args, **kwargs):
        kwargs["headless"] = True
        kwargs["fps"] = 1_000_000
        cwd = os.getcwd()
        original_init(*args, **kwargs)
        os.chdir(cwd)
        pyxel.rseed(0)
        pyxel.nseed(0)
        random.seed(0)

    def patched_run(update, draw):
        captured["update"] = update
        captured["draw"] = draw

    pyxel.init = patched_init
    pyxel.run = patched_run
    pyxel.show = lambda: None
    app_dir = str(Path(startup_path).parent)
    saved_modules = set(sys.modules.keys())
    sys.path.insert(0, app_dir)
    try:
        os.chdir(app_dir)
        runpy.run_path(startup_path, run_name="__main__")
    finally:
        # Keep CWD at app_dir so capture_frames can resolve relative asset paths
        if app_dir in sys.path:
            sys.path.remove(app_dir)
        # Clean up app-specific modules to avoid cross-contamination
        for mod_name in list(sys.modules.keys()):
            if mod_name not in saved_modules:
                del sys.modules[mod_name]
        pyxel.init = original_init
        pyxel.run = original_run
        pyxel.show = original_show
    return captured, app_dir


class TestApps:
    @pytest.mark.parametrize(
        "name", list(CAPTURE_PLANS.keys()), ids=list(CAPTURE_PLANS.keys())
    )
    def test_app(self, name, tmp_path, update_references):
        pyxapp = APPS_DIR / f"{name}.pyxapp"
        assert pyxapp.exists(), f"App not found: {pyxapp}"

        extract_dir = tmp_path / "extract"
        extract_dir.mkdir()
        startup = extract_pyxapp(pyxapp, extract_dir)

        _reinit_pyxel()
        original_dir = os.getcwd()
        try:
            captured, app_dir = run_pyxapp(startup)
            os.chdir(app_dir)
            plan = CAPTURE_PLANS[name]
            results = capture_frames(captured, plan, tmp_path)
            compare_or_update_all(name, results, APP_REFS_DIR, update_references)
        finally:
            os.chdir(original_dir)
            _restore_pyxel()
