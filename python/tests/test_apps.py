import pytest

import pyxel

from _capture import (  # type: ignore[reportMissingImports]
    APP_REFS_DIR,
    APPS_DIR,
    collect_plan_results,
    run_app_subprocess,
)

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


class TestApps:
    @pytest.mark.parametrize(
        "name", list(CAPTURE_PLANS.keys()), ids=list(CAPTURE_PLANS.keys())
    )
    def test_app(self, name, tmp_path, compare_screenshots):
        pyxapp = APPS_DIR / f"{name}.pyxapp"
        assert pyxapp.exists(), f"App not found: {pyxapp}"

        plan = CAPTURE_PLANS[name]
        run_app_subprocess(pyxapp, plan, tmp_path)

        results = collect_plan_results(plan, tmp_path)
        compare_screenshots(name, results, APP_REFS_DIR)
