import os
import runpy
import shutil
from pathlib import Path

import pytest
import pyxel

REFERENCES_DIR = Path(__file__).parent / "references"
EXAMPLES_DIR = Path(__file__).parent.parent / "pyxel" / "examples"
EXAMPLE_REFS_DIR = REFERENCES_DIR / "examples"


def reinit_pyxel():
    """Reset Pyxel state so init() can be called again."""
    pyxel._reset_statics()


def restore_pyxel():
    """Restore Pyxel to conftest's default session state."""
    pyxel._reset_statics()
    pyxel.init(160, 120, headless=True)


def run_example(script_path):
    """Execute example script, capturing update/draw callbacks."""
    captured = {}
    original_init = pyxel.init
    original_run = pyxel.run
    original_show = pyxel.show

    def patched_init(*args, **kwargs):
        kwargs["headless"] = True
        kwargs["fps"] = 1_000_000  # Bypass SDL_Delay throttle
        cwd = os.getcwd()
        original_init(*args, **kwargs)
        os.chdir(cwd)
        # Fix random seeds for deterministic screenshots
        pyxel.rseed(0)
        pyxel.nseed(0)

    def patched_run(update, draw):
        captured["update"] = update
        captured["draw"] = draw

    def patched_show():
        pass

    pyxel.init = patched_init
    pyxel.run = patched_run
    pyxel.show = patched_show
    original_dir = os.getcwd()
    try:
        os.chdir(Path(script_path).parent)
        runpy.run_path(str(script_path), run_name="__main__")
    finally:
        os.chdir(original_dir)
        pyxel.init = original_init
        pyxel.run = original_run
        pyxel.show = original_show
    return captured


class _FlipCapture(Exception):
    """Raised to stop a while+flip() loop at the target frame."""

    pass


def run_flip_example(script_path, plan, tmp_dir):
    """Handle while+flip() examples by patching flip() to capture at plan frames."""
    original_init = pyxel.init
    original_flip = pyxel.flip
    frame_count = [0]
    capture_at = {step["frame"] for step in plan}
    max_frame = max(capture_at)
    results = []

    def patched_init(*args, **kwargs):
        kwargs["headless"] = True
        kwargs["fps"] = 1_000_000
        cwd = os.getcwd()
        original_init(*args, **kwargs)
        os.chdir(cwd)
        pyxel.rseed(0)
        pyxel.nseed(0)

    def patched_flip():
        original_flip()
        frame_count[0] += 1
        if frame_count[0] in capture_at:
            path = tmp_dir / f"frame_{frame_count[0]}.png"
            pyxel.screenshot(str(path))
            results.append((frame_count[0], path))
        if frame_count[0] >= max_frame:
            raise _FlipCapture()

    pyxel.init = patched_init
    pyxel.flip = patched_flip
    original_dir = os.getcwd()
    try:
        os.chdir(Path(script_path).parent)
        runpy.run_path(str(script_path), run_name="__main__")
    except _FlipCapture:
        pass
    finally:
        os.chdir(original_dir)
        pyxel.init = original_init
        pyxel.flip = original_flip
    return results


def capture_frames(captured, plan, tmp_dir):
    """Execute capture plan, returning list of (frame, png_path) pairs."""
    results = []
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
            path = tmp_dir / f"frame_{target}.png"
            pyxel.screenshot(str(path))
            results.append((target, path))
        if "press" in step:
            for key in step["press"]:
                pyxel.set_btn(key, False)
    return results


def compare_or_update_all(name, results, refs_dir, update_references):
    """Compare all captured frames to references, or update them."""
    updated = []
    failures = []
    for frame, actual_path in results:
        ref_path = refs_dir / f"{name}_f{frame}.png"
        if update_references:
            refs_dir.mkdir(parents=True, exist_ok=True)
            shutil.copy(actual_path, ref_path)
            updated.append(ref_path.name)
            continue

        if not ref_path.exists():
            failures.append(
                f"Reference missing: {ref_path.name}. Run with --update-references"
            )
            continue

        actual_bytes = Path(actual_path).read_bytes()
        ref_bytes = ref_path.read_bytes()
        if actual_bytes != ref_bytes:
            diff_path = ref_path.with_name(f"{name}_f{frame}_actual.png")
            shutil.copy(actual_path, diff_path)
            failures.append(
                f"Screenshot mismatch: {ref_path.name}. "
                f"Actual saved to {diff_path.name}"
            )

    if updated:
        pytest.skip(f"References updated: {', '.join(updated)}")
    if failures:
        pytest.fail("\n".join(failures))


# Capture plans — frame numbers and optional input injection
CAPTURE_PLANS = {
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
    # Group 2: pyxel.run() with asset loading
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
    # Group 3: pyxel.show()-based (no update/draw loop)
    "05_color_palette": [{"frame": 0}],
    "13_custom_font": [{"frame": 0}],
    "17_app_launcher": [{"frame": 1}],
    # Group 5: draw API with clipping test (SPACE toggles clip)
    "03_draw_api": [
        {"frame": 1},
        {"frame": 155, "press": [pyxel.KEY_SPACE]},
    ],
    # Group 4: while+flip loop
    "99_flip_animation": [{"frame": 1}, {"frame": 30}],
}

FLIP_EXAMPLES = {"99_flip_animation"}


class TestExamples:
    @pytest.mark.parametrize(
        "name", list(CAPTURE_PLANS.keys()), ids=list(CAPTURE_PLANS.keys())
    )
    def test_example(self, name, tmp_path, update_references):
        script = EXAMPLES_DIR / f"{name}.py"
        assert script.exists(), f"Example not found: {script}"

        reinit_pyxel()
        try:
            if name in FLIP_EXAMPLES:
                plan = CAPTURE_PLANS[name]
                results = run_flip_example(script, plan, tmp_path)
                compare_or_update_all(
                    name, results, EXAMPLE_REFS_DIR, update_references
                )
                return

            captured = run_example(script)
            plan = CAPTURE_PLANS[name]
            results = capture_frames(captured, plan, tmp_path)
            compare_or_update_all(name, results, EXAMPLE_REFS_DIR, update_references)
        finally:
            restore_pyxel()
