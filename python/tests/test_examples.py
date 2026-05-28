import pytest

import pyxel

from _capture import (  # type: ignore[reportMissingImports]
    EXAMPLES_DIR,
    EXAMPLE_REFS_DIR,
    collect_plan_results,
    run_example_subprocess,
    run_flip_example_subprocess,
)

CAPTURE_PLANS = {
    # pyxel.run() without asset loading
    "01_hello_pyxel": [{"frame": 8}],
    "04_sound_api": [{"frame": 1}],
    "06_click_game": [
        {"frame": 1},
        {"frame": 10, "mouse": (110, 146), "press": [pyxel.MOUSE_BUTTON_LEFT]},
    ],
    "07_snake": [{"frame": 1}],
    "08_triangle_api": [{"frame": 1}, {"frame": 200}],
    "09_shooter": [
        {"frame": 1, "press": [pyxel.KEY_RETURN]},
        {"frame": 120},
    ],
    "12_perlin_noise": [{"frame": 1}, {"frame": 40}],
    "14_synthesizer": [{"frame": 1}],
    # pyxel.run() with asset loading
    "02_jump_game": [{"frame": 10}],
    "10_platformer": [
        {"frame": 1},
        *[
            {"frame": i, "press": [pyxel.KEY_RIGHT, pyxel.KEY_SPACE], "capture": False}
            for i in range(2, 80, 2)
        ],
        {"frame": 80},
    ],
    "11_offscreen": [{"frame": 1}, {"frame": 121}],
    "15_tiled_map_file": [{"frame": 1}],
    "16_transform": [{"frame": 1}, {"frame": 45}],
    "18_audio_playback": [
        {"frame": 1},
        {"frame": 3, "press": [pyxel.KEY_RETURN]},
    ],
    "19_perspective": [
        {"frame": 1},
        {"frame": 20, "press": [pyxel.KEY_RIGHT, pyxel.KEY_W]},
    ],
    # pyxel.show()-based (no update/draw loop)
    "05_color_palette": [{"frame": 0}],
    "13_custom_font": [{"frame": 0}],
    "17_app_launcher": [{"frame": 1}],
    # pyxel.run() with SPACE toggling clip
    "03_draw_api": [
        {"frame": 1},
        {"frame": 155, "press": [pyxel.KEY_SPACE]},
    ],
    # while+flip() loop
    "99_flip_animation": [{"frame": 1}, {"frame": 30}],
}

FLIP_EXAMPLES = {"99_flip_animation"}


class TestExamples:
    @pytest.mark.parametrize(
        "name", list(CAPTURE_PLANS.keys()), ids=list(CAPTURE_PLANS.keys())
    )
    def test_example(self, name, tmp_path, compare_screenshots):
        script = EXAMPLES_DIR / f"{name}.py"
        assert script.exists(), f"Example not found: {script}"

        plan = CAPTURE_PLANS[name]
        if name in FLIP_EXAMPLES:
            run_flip_example_subprocess(script, plan, tmp_path)
        else:
            run_example_subprocess(script, plan, tmp_path)

        results = collect_plan_results(plan, tmp_path)
        compare_screenshots(name, results, EXAMPLE_REFS_DIR)
