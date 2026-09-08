import subprocess
import sys

import pytest
import pyxel
from _capture import (  # type: ignore[reportMissingImports]
    EXAMPLE_REFS_DIR,
    EXAMPLES_DIR,
    collect_plan_results,
    run_example_subprocess,
    run_flip_example_subprocess,
)

CAPTURE_PLANS = {
    # pyxel.run()-based examples
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
    # Asset-loading examples
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
    # Cube examples
    "c01_hello_cube": [{"frame": 8}],
    "c02_basic_shapes": [
        {"frame": 1},
        {"frame": 30, "press": [pyxel.KEY_SPACE]},
    ],
    "c03_custom_shapes": [
        {"frame": 1},
        {
            "frame": 20,
            "mouse": (135, 115),
            "press": [pyxel.MOUSE_BUTTON_LEFT],
            "capture": False,
        },
        {"frame": 32},
        {"frame": 40},
        {"frame": 48},
    ],
    "c04_mesh_and_motion": [{"frame": 1}, {"frame": 45}],
    "c05_3d_collision": [
        {"frame": 1},
        {"frame": 70, "press": [pyxel.KEY_UP]},
        {"frame": 100, "press": [pyxel.KEY_UP, pyxel.KEY_SPACE]},
    ],
    "c06_3d_physics": [{"frame": 1}, {"frame": 70}, {"frame": 140}],
    "cube_physics_character": [{"frame": 1}, {"frame": 60}],
    "cube_physics_shoot": [
        {"frame": 1},
        {"frame": 2, "press": [pyxel.KEY_SPACE], "capture": False},
        {"frame": 60},
    ],
    "cube_physics_stack": [{"frame": 1}, {"frame": 60}],
    "cube_physics_terrain": [{"frame": 1}, {"frame": 60}],
    # Static-screen and launcher captures
    "05_color_palette": [{"frame": 0}],
    "13_custom_font": [{"frame": 0}],
    "17_app_launcher": [{"frame": 1}],
    # pyxel.run() with SPACE held for clipping
    "03_draw_api": [
        {"frame": 1},
        {"frame": 155, "press": [pyxel.KEY_SPACE]},
    ],
    # while+flip() loop
    "99_flip_animation": [{"frame": 1}, {"frame": 30}],
}

# Examples living in the examples/cube subdirectory rather than the top level
CUBE_DIR_EXAMPLES = {
    "cube_physics_character",
    "cube_physics_shoot",
    "cube_physics_stack",
    "cube_physics_terrain",
}

FLIP_EXAMPLES = {"99_flip_animation"}


class TestExamples:
    def test_wavetable_strokes_start_at_click_and_fill_dragged_columns(self):
        code = """
import runpy
import sys

import pyxel

init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
pyxel.run = lambda update, draw: None

namespace = runpy.run_path(sys.argv[1])
editor = namespace["WavetableEditor"](8, 8, 0, "Test")
wave = pyxel.tones[0].wavetable
wave[:] = [8] * 32


def frame(col, row, pressed=None):
    pyxel.set_mouse_pos(editor.x + 1 + col * 5, editor.y + 8 + (15 - row) * 3)
    if pressed is not None:
        pyxel.set_btn(pyxel.MOUSE_BUTTON_LEFT, pressed)
    editor.update()
    pyxel.flip()


frame(2, 8, False)
frame(20, 15, True)
expected = [8] * 20 + [15] + [8] * 11
assert list(wave) == expected, list(wave)

frame(24, 1)
expected[20:25] = [1] * 5
assert list(wave) == expected, list(wave)

frame(7, 9, False)
assert list(wave) == expected, list(wave)

frame(5, 0, True)
expected[5] = 0
assert list(wave) == expected, list(wave)
"""

        result = subprocess.run(
            [sys.executable, "-c", code, str(EXAMPLES_DIR / "14_synthesizer.py")],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

    def test_top_level_examples_have_capture_plans(self):
        planned = set(CAPTURE_PLANS) - CUBE_DIR_EXAMPLES
        examples = {
            script.stem
            for script in EXAMPLES_DIR.glob("*.py")
            if not script.name.startswith("__")
        }
        assert planned == examples

    @pytest.mark.parametrize(
        "name", list(CAPTURE_PLANS.keys()), ids=list(CAPTURE_PLANS.keys())
    )
    def test_example(self, name, tmp_path, compare_screenshots):
        script_dir = (
            EXAMPLES_DIR / "cube" if name in CUBE_DIR_EXAMPLES else EXAMPLES_DIR
        )
        script = script_dir / f"{name}.py"
        assert script.exists(), f"Example not found: {script}"

        plan = CAPTURE_PLANS[name]
        if name in FLIP_EXAMPLES:
            run_flip_example_subprocess(script, plan, tmp_path)
        else:
            run_example_subprocess(script, plan, tmp_path)

        results = collect_plan_results(plan, tmp_path)
        compare_screenshots(name, results, EXAMPLE_REFS_DIR)
