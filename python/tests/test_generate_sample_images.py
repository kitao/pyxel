# Exercise documentation recording separately from single-frame example tests.

import runpy
import subprocess
import sys
from pathlib import Path

import pytest
from PIL import Image, ImageSequence

ROOT = Path(__file__).parents[2]
SCRIPTS = ROOT / "scripts"
# Reviewed demo lengths and screen sizes, independent of generator settings.
# GIF times are rounded to hundredths; composites concatenate their source times.
OUTPUTS = {
    "example": {
        "01_hello_pyxel.gif": ((320, 240), 1000),
        "02_jump_game.gif": ((320, 240), 10000),
        "03_draw_api.gif": ((400, 300), 9330),
        "04_sound_api.gif": ((400, 300), 6000),
        "05_color_palette.png": ((510, 162), None),
        "06_click_game.gif": ((256, 256), 10000),
        "07_snake.gif": ((240, 300), 11000),
        # This demonstration intentionally samples every third update at 25 FPS.
        "08_triangle_api.gif": ((400, 300), 14840),
        "09_shooter.gif": ((240, 320), 9330),
        "10_platformer.gif": ((256, 256), 10000),
        "11_offscreen.gif": ((446, 184), 8000),
        "12_perlin_noise.gif": ((256, 256), 10000),
        "13_custom_font.png": ((256, 256), None),
        "14_synthesizer.gif": ((382, 528), 9330),
        "15_tiled_map_file.gif": ((928, 512), 6670),
        "16_transform.gif": ((400, 320), 5000),
        "17_app_launcher.gif": ((836, 346), 7170),
        "18_audio_playback.gif": ((512, 480), 10000),
        "19_perspective.gif": ((400, 300), 5500),
        "99_flip_animation.gif": ((256, 256), 1000),
    },
    "cube": {
        "c01_hello_cube.gif": ((400, 300), 6000),
        "c02_basic_shapes.gif": ((240, 240), 6000),
        "c03_custom_shapes.gif": ((256, 192), 7030),
        "c04_mesh_and_motion.gif": ((640, 480), 10000),
        "c05_3d_collision.gif": ((640, 480), 10000),
        "c06_3d_physics.gif": ((640, 480), 10000),
    },
    "app": {
        "30sec_of_daylight.gif": ((320, 240), 17600),
        "megaball.gif": ((320, 288), 10000),
    },
    "editor": {
        "image_editor.gif": ((480, 360), 5970),
        "tilemap_editor.gif": ((480, 360), 5370),
        "sound_editor.gif": ((480, 360), 7700),
        "music_editor.gif": ((480, 360), 8900),
        "image_tilemap_editor.gif": ((480, 360), 11340),
        "sound_music_editor.gif": ((480, 360), 16600),
        "pyxel_editor.gif": ((480, 360), 27940),
    },
}


@pytest.fixture
def capture_scripts(monkeypatch):
    monkeypatch.syspath_prepend(str(SCRIPTS))
    return runpy.run_path(str(SCRIPTS / "_image_capture.py"))


def test_gif_writer_preserves_colors_order_and_repeated_frame_time(
    capture_scripts, tmp_path
):
    frames = [
        Image.new("RGB", (2, 2), color) for color in ("red", "red", "blue", "lime")
    ]
    destination = tmp_path / "frames.gif"
    capture_scripts["save_gif"](frames, destination)
    with Image.open(destination) as image:
        decoded = [
            (frame.convert("RGB").getpixel((0, 0)), frame.info["duration"])
            for frame in ImageSequence.Iterator(image)
        ]
        assert image.info["loop"] == 0
    # Identical frames may merge; their display time must not disappear.
    assert decoded == [((255, 0, 0), 70), ((0, 0, 255), 30), ((0, 255, 0), 30)]


@pytest.mark.parametrize("family", OUTPUTS)
def test_documentation_generator_full_pipeline(family, tmp_path):
    output = tmp_path / family
    result = subprocess.run(
        [
            sys.executable,
            str(SCRIPTS / f"generate_{family}_images"),
            "--output-dir",
            str(output),
        ],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
        timeout=300,
    )
    assert result.returncode == 0, result.stdout + result.stderr
    assert {path.name for path in output.iterdir()} == set(OUTPUTS[family])

    for filename, (size, duration) in OUTPUTS[family].items():
        with Image.open(output / filename) as image:
            assert image.size == size, filename
            if duration is None:
                assert image.format == "PNG", filename
                image.load()
                continue
            assert image.format == "GIF", filename
            assert image.info["loop"] == 0, filename
            elapsed = 0
            for frame in ImageSequence.Iterator(image):
                frame.load()
                assert frame.info["duration"] > 0, filename
                elapsed += frame.info["duration"]
            assert elapsed == duration, filename


@pytest.mark.parametrize(
    ("family", "names"),
    [
        ("example", ["1", "2"]),
        ("cube", ["c01_hello_cube", "c02_basic_shapes"]),
        ("app", ["30sec_of_daylight", "megaball"]),
        ("editor", ["image", "tilemap"]),
    ],
)
def test_failed_worker_preserves_existing_outputs(
    family, names, tmp_path, monkeypatch, capture_scripts
):
    generator = runpy.run_path(str(SCRIPTS / f"generate_{family}_images"))
    destination = tmp_path / "published"
    destination.mkdir()
    originals = {name: b"previous verified output" for name in OUTPUTS[family]}
    for name, data in originals.items():
        (destination / name).write_bytes(data)
    calls = 0

    def worker(command, **kwargs):
        nonlocal calls
        calls += 1
        staging = Path(command[command.index("--output-dir") + 1])
        (staging / "partial.gif").write_bytes(b"incomplete candidate")
        if calls == 2:
            raise subprocess.CalledProcessError(1, command)

    monkeypatch.setattr(subprocess, "run", worker)
    monkeypatch.setattr(
        sys, "argv", ["generator", *names, "--output-dir", str(destination)]
    )
    with pytest.raises(subprocess.CalledProcessError):
        generator["main"]()
    assert calls == 2
    assert {path.name: path.read_bytes() for path in destination.iterdir()} == originals


def test_editor_default_output_updates_both_documentation_copies(
    tmp_path, monkeypatch, capture_scripts
):
    generator = runpy.run_path(str(SCRIPTS / "generate_editor_images"))
    docs, manual = tmp_path / "docs", tmp_path / "manual"
    docs.mkdir()
    manual.mkdir()
    for directory in (docs, manual):
        (directory / "image_editor.gif").write_bytes(b"previous output")

    def worker(command, **kwargs):
        staging = Path(command[command.index("--output-dir") + 1])
        (staging / "image_editor.gif").write_bytes(b"new complete output")

    monkeypatch.setitem(generator["main"].__globals__, "DOC_IMAGES", docs)
    monkeypatch.setitem(generator["main"].__globals__, "MANUAL_IMAGES", manual)
    monkeypatch.setattr(subprocess, "run", worker)
    monkeypatch.setattr(sys, "argv", ["generator", "image"])
    generator["main"]()
    for directory in (docs, manual):
        assert (directory / "image_editor.gif").read_bytes() == b"new complete output"
        assert len(list(directory.iterdir())) == 1
