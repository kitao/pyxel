# Exercise documentation recording separately from single-frame example tests.

import runpy
import subprocess
import sys
from itertools import zip_longest
from pathlib import Path

import pytest
from PIL import GifImagePlugin, Image, ImageChops, ImageSequence

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
def test_documentation_generator_full_pipeline(family, tmp_path, monkeypatch):
    # Unattended recording selects its own audio driver in every worker family.
    monkeypatch.setenv("SDL_AUDIODRIVER", "pyxel_test_invalid")
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
    assert "Failed to initialize audio device" not in result.stdout + result.stderr
    assert {path.name for path in output.iterdir()} == set(OUTPUTS[family])

    for filename, (size, duration) in OUTPUTS[family].items():
        actual = output / filename
        with Image.open(actual) as image:
            assert image.size == size, filename
            assert image.format == ("PNG" if duration is None else "GIF"), filename
            if duration is not None:
                assert image.info["loop"] == 0, filename
        # Published recordings are the baseline; ordinary tests never rewrite them.
        elapsed = _compare_recording(actual, ROOT / "docs" / "images" / filename)
        assert elapsed == (duration or 0), filename


def _compare_recording(actual_path, reference_path):
    with Image.open(actual_path) as actual, Image.open(reference_path) as reference:
        assert actual.size == reference.size, f"{actual_path.name}: size changed"
        assert actual.format == reference.format, f"{actual_path.name}: format changed"
        assert actual.info.get("loop") == reference.info.get("loop"), (
            f"{actual_path.name}: loop changed"
        )
        elapsed = 0
        for actual_frame, reference_frame in zip_longest(
            _recording_frames(actual), _recording_frames(reference)
        ):
            assert actual_frame is not None and reference_frame is not None, (
                f"{actual_path.name}: recording length changed at {elapsed} ms"
            )
            actual_pixels, actual_duration = actual_frame
            reference_pixels, reference_duration = reference_frame
            if actual_pixels != reference_pixels:
                diff_path = actual_path.with_name(f"{actual_path.stem}_diff.png")
                ImageChops.difference(
                    Image.frombytes("RGB", actual.size, actual_pixels),
                    Image.frombytes("RGB", reference.size, reference_pixels),
                ).save(diff_path)
                pytest.fail(
                    f"{actual_path.name}: pixels changed at {elapsed} ms; "
                    f"difference image: {diff_path}"
                )
            assert actual_duration == reference_duration, (
                f"{actual_path.name}: timing changed at "
                f"{elapsed + min(actual_duration, reference_duration)} ms "
                f"({actual_duration} ms instead of {reference_duration} ms)"
            )
            elapsed += actual_duration
        return elapsed


def _recording_frames(image):
    # Adjacent identical frames describe one display interval, regardless of encoding.
    pixels = None
    duration = 0
    for frame in ImageSequence.Iterator(image):
        frame_pixels = frame.convert("RGB").tobytes()
        frame_duration = frame.info.get("duration", 0) if image.format == "GIF" else 0
        if image.format == "GIF":
            assert frame_duration > 0, f"{image.filename}: nonpositive frame duration"
        if pixels is not None and frame_pixels != pixels:
            yield pixels, duration
            duration = 0
        pixels = frame_pixels
        duration += frame_duration
    if pixels is not None:
        yield pixels, duration


class TestRecordingComparison:
    def test_equivalent_frame_splits_and_palette_encodings(self, tmp_path):
        actual = tmp_path / "actual.gif"
        reference = tmp_path / "reference.gif"
        _write_recording(actual, [1, 1, 2], [20, 30, 40])
        _write_recording(reference, [1, 2], [50, 40], reverse_palette=True)
        with Image.open(actual) as image:
            assert image.n_frames == 3
        assert actual.read_bytes() != reference.read_bytes()
        assert _compare_recording(actual, reference) == 90
        assert _compare_recording(reference, actual) == 90

    def test_one_pixel_change_reports_first_time_and_diff(self, tmp_path):
        actual = tmp_path / "actual.gif"
        reference = tmp_path / "reference.gif"
        _write_recording(actual, [1, 2], [50, 40], changed_pixel=True)
        _write_recording(reference, [1, 2], [50, 40])
        with pytest.raises(pytest.fail.Exception, match="pixels changed at 50 ms"):
            _compare_recording(actual, reference)
        with Image.open(tmp_path / "actual_diff.png") as diff:
            assert diff.getpixel((0, 0)) == (0, 0, 255)
            assert diff.getpixel((1, 0)) == (0, 0, 0)

    def test_frame_order_change_with_equal_total_duration(self, tmp_path):
        actual = tmp_path / "actual.gif"
        reference = tmp_path / "reference.gif"
        _write_recording(actual, [1, 2, 3], [50, 40, 30])
        _write_recording(reference, [1, 3, 2], [50, 30, 40])
        with pytest.raises(pytest.fail.Exception, match="pixels changed at 50 ms"):
            _compare_recording(actual, reference)

    def test_shifted_transition_with_equal_total_duration(self, tmp_path):
        actual = tmp_path / "actual.gif"
        reference = tmp_path / "reference.gif"
        _write_recording(actual, [1, 2], [40, 50])
        _write_recording(reference, [1, 2], [50, 40])
        with pytest.raises(AssertionError, match="timing changed at 40 ms"):
            _compare_recording(actual, reference)

    @pytest.mark.parametrize("extra_frame", [False, True])
    def test_added_or_removed_tail(self, tmp_path, extra_frame):
        actual = tmp_path / "actual.gif"
        reference = tmp_path / "reference.gif"
        _write_recording(actual, [1, 2], [50, 40])
        _write_recording(reference, [1], [50])
        if not extra_frame:
            actual, reference = reference, actual
        with pytest.raises(AssertionError, match="recording length changed at 50 ms"):
            _compare_recording(actual, reference)

    def test_loop_change(self, tmp_path):
        actual = tmp_path / "actual.gif"
        reference = tmp_path / "reference.gif"
        _write_recording(actual, [1, 2], [50, 40], loop=1)
        _write_recording(reference, [1, 2], [50, 40])
        with pytest.raises(AssertionError, match="loop changed"):
            _compare_recording(actual, reference)

    def test_nonpositive_duration(self, tmp_path):
        actual = tmp_path / "actual.gif"
        reference = tmp_path / "reference.gif"
        _write_recording(actual, [1, 2], [50, 0])
        _write_recording(reference, [1, 2], [50, 40])
        with pytest.raises(AssertionError, match="nonpositive frame duration"):
            _compare_recording(actual, reference)

    def test_static_image_pixels(self, tmp_path):
        actual = tmp_path / "actual.png"
        reference = tmp_path / "reference.png"
        Image.new("RGB", (2, 1), "red").save(actual)
        Image.new("RGB", (2, 1), "red").convert("P").save(reference)
        assert _compare_recording(actual, reference) == 0
        Image.new("RGB", (2, 1), "blue").save(actual)
        with pytest.raises(pytest.fail.Exception, match="pixels changed at 0 ms"):
            _compare_recording(actual, reference)

    def test_size_change(self, tmp_path):
        actual = tmp_path / "actual.png"
        reference = tmp_path / "reference.png"
        Image.new("RGB", (2, 1), "red").save(actual)
        Image.new("RGB", (1, 1), "red").save(reference)
        with pytest.raises(AssertionError, match="size changed"):
            _compare_recording(actual, reference)


def _write_recording(
    path, pixels, durations, *, reverse_palette=False, changed_pixel=False, loop=0
):
    colors = [(0, 0, 0), (255, 0, 0), (0, 0, 255), (0, 255, 0)]
    if reverse_palette:
        colors.reverse()
        pixels = [len(colors) - 1 - pixel for pixel in pixels]
    palette = [channel for color in colors for channel in color]
    palette += [0] * (768 - len(palette))
    frames = [Image.new("P", (2, 1), pixel) for pixel in pixels]
    for frame in frames:
        frame.putpalette(palette)
    if changed_pixel:
        frames[-1].putpixel((0, 0), 0)

    # Write frame blocks directly so Pillow's saver cannot merge the split-frame case.
    with path.open("wb") as output:
        output.writelines(GifImagePlugin.getheader(frames[0], info={"loop": loop})[0])
        for frame, duration in zip(frames, durations, strict=True):
            output.writelines(GifImagePlugin.getdata(frame, duration=duration))
        output.write(b";")


@pytest.mark.parametrize("stream", ["stdout", "stderr"])
def test_worker_rejects_audio_initialization_warning(capture_scripts, stream):
    command = [
        sys.executable,
        "-c",
        f"import sys; print('Failed to initialize audio device', file=sys.{stream})",
    ]
    with pytest.raises(RuntimeError, match="Failed to initialize audio device"):
        capture_scripts["run_worker"](command)


def test_worker_preserves_failure_output_and_status(capture_scripts, capsys):
    command = [
        sys.executable,
        "-c",
        "import sys; print('capture failed', file=sys.stderr); sys.exit(3)",
    ]
    with pytest.raises(subprocess.CalledProcessError) as error:
        capture_scripts["run_worker"](command)
    assert error.value.returncode == 3
    assert capsys.readouterr().err == "capture failed\n"


def test_worker_preserves_timeout_output(capture_scripts, capsys, monkeypatch):
    command = [sys.executable, "capture.py"]

    def worker(*args, **kwargs):
        raise subprocess.TimeoutExpired(
            command, 120, output=b"capture started\n", stderr=b"capture stalled\n"
        )

    monkeypatch.setattr(subprocess, "run", worker)
    with pytest.raises(subprocess.TimeoutExpired):
        capture_scripts["run_worker"](command)
    output = capsys.readouterr()
    assert output.out == "capture started\n"
    assert output.err == "capture stalled\n"


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
        return subprocess.CompletedProcess(command, 0, stdout="", stderr="")

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
        return subprocess.CompletedProcess(command, 0, stdout="", stderr="")

    monkeypatch.setitem(generator["main"].__globals__, "DOC_IMAGES", docs)
    monkeypatch.setitem(generator["main"].__globals__, "MANUAL_IMAGES", manual)
    monkeypatch.setattr(subprocess, "run", worker)
    monkeypatch.setattr(sys, "argv", ["generator", "image"])
    generator["main"]()
    for directory in (docs, manual):
        assert (directory / "image_editor.gif").read_bytes() == b"new complete output"
        assert len(list(directory.iterdir())) == 1


@pytest.mark.parametrize(
    ("family", "name", "filename"),
    [
        ("app", "megaball", "megaball.gif"),
        ("cube", "c01_hello_cube", "c01_hello_cube.gif"),
        ("editor", "image", "image_editor.gif"),
        ("example", "1", "01_hello_pyxel.gif"),
    ],
)
def test_repeated_selection_generates_and_publishes_once(
    family, name, filename, tmp_path, monkeypatch, capture_scripts
):
    generator = runpy.run_path(str(SCRIPTS / f"generate_{family}_images"))
    calls = []

    def worker(command, **kwargs):
        calls.append(command)
        staging = Path(command[command.index("--output-dir") + 1])
        (staging / filename).write_bytes(b"complete output")
        return subprocess.CompletedProcess(command, 0, stdout="", stderr="")

    monkeypatch.setattr(subprocess, "run", worker)
    monkeypatch.setattr(
        sys, "argv", ["generator", name, name, "--output-dir", str(tmp_path)]
    )
    generator["main"]()
    assert len(calls) == 1
    assert (tmp_path / filename).read_bytes() == b"complete output"
    assert list(tmp_path.iterdir()) == [tmp_path / filename]
