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
    "c04_mesh_and_motion": [
        {"frame": 1},
        {"frame": 45, "press": [pyxel.KEY_2]},
        {"frame": 91, "press": [pyxel.KEY_RIGHT]},
        {"frame": 92, "press": [pyxel.KEY_3, pyxel.KEY_S]},
        {"frame": 123, "press": [pyxel.KEY_1]},
        {"frame": 124, "press": [pyxel.KEY_S]},
    ],
    "c05_3d_collision": [
        {"frame": 1},
        {"frame": 70, "press": [pyxel.KEY_UP]},
        {"frame": 100, "press": [pyxel.KEY_UP, pyxel.KEY_SPACE]},
    ],
    "c06_3d_physics": [{"frame": 1}, {"frame": 70}, {"frame": 140}],
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

FLIP_EXAMPLES = {"99_flip_animation"}


class TestExamples:
    def test_shooter_consumes_each_collision_once(self):
        code = """
import runpy
import sys

import pyxel

init = pyxel.init
pyxel.init = lambda *args, **kwargs: init(*args, **kwargs, headless=True)
pyxel.run = lambda update, draw: None

namespace = runpy.run_path(sys.argv[1])
pyxel.flip()
app = namespace["App"].__new__(namespace["App"])

# A destroyed enemy cannot score twice or kill the overlapping player.
# A consumed bullet cannot destroy a second overlapping enemy.
for enemy_count, bullet_count, player_x in [(1, 2, 30), (2, 1, 90)]:
    for name in ("enemies", "bullets", "blasts"):
        namespace[name].clear()
    app.player = namespace["Player"](player_x, 30)
    app.score = 0
    app.scene = namespace["SCENE_PLAY"]
    for _ in range(enemy_count):
        namespace["Enemy"](30, 30)
    for _ in range(bullet_count):
        namespace["Bullet"](30, 30)

    app.update_play_scene()

    assert app.score == 10, app.score
    assert app.scene == namespace["SCENE_PLAY"], app.scene
    assert len(namespace["enemies"]) == enemy_count - 1
    assert len(namespace["bullets"]) == bullet_count - 1
    assert len(namespace["blasts"]) == 1
"""

        result = subprocess.run(
            [sys.executable, "-c", code, str(EXAMPLES_DIR / "09_shooter.py")],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
        assert result.returncode == 0, result.stderr

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

    def test_numbered_examples_have_capture_plans(self):
        planned = set(CAPTURE_PLANS)
        examples = {
            script.stem
            for script in EXAMPLES_DIR.glob("*.py")
            if not script.name.startswith("__")
        }
        examples.update(script.stem for script in (EXAMPLES_DIR / "cube").glob("*.py"))
        assert planned == examples

    @pytest.mark.parametrize(
        "name", list(CAPTURE_PLANS.keys()), ids=list(CAPTURE_PLANS.keys())
    )
    def test_example(self, name, tmp_path, compare_screenshots):
        script_dir = EXAMPLES_DIR / "cube" if name.startswith("c") else EXAMPLES_DIR
        script = script_dir / f"{name}.py"
        assert script.exists(), f"Example not found: {script}"

        plan = CAPTURE_PLANS[name]
        if name in FLIP_EXAMPLES:
            run_flip_example_subprocess(script, plan, tmp_path)
        else:
            run_example_subprocess(script, plan, tmp_path)

        results = collect_plan_results(plan, tmp_path)
        compare_screenshots(name, results, EXAMPLE_REFS_DIR)
