import subprocess
import sys

import pytest

import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]


class TestTypeErrors:
    def test_sin_wrong_deg_type(self):
        with raises_exact(TypeError, "must be real number, not str"):
            pyxel.sin("abc")  # type: ignore[arg-type]

    def test_pset_wrong_x_type(self):
        with raises_exact(TypeError, "must be real number, not str"):
            pyxel.pset("a", 0, 0)  # type: ignore[arg-type]

    def test_clamp_wrong_x_type(self):
        with raises_exact(TypeError, "must be real number, not str"):
            pyxel.clamp("a", 0, 10)  # type: ignore[arg-type]

    def test_rect_wrong_types(self):
        with raises_exact(TypeError, "must be real number, not str"):
            pyxel.rect("a", "b", "c", "d", "e")  # type: ignore[arg-type]

    @pytest.mark.parametrize(
        ("operation", "message"),
        [
            (
                lambda: pyxel.play(0, 3.14),
                "snd must be int, list[int], Sound, list[Sound], or str",
            ),
            (
                lambda: pyxel.Channel().play(3.14),
                "snd must be int, list[int], Sound, list[Sound], or str",
            ),
            (
                lambda: pyxel.blt(0, 0, "bad", 0, 0, 8, 8),
                "img must be int or Image",
            ),
            (
                lambda: pyxel.bltm(0, 0, "bad", 0, 0, 8, 8),
                "tm must be int or Tilemap",
            ),
            (
                lambda: pyxel.blt3d(0, 0, 8, 8, "bad", (0, 0, 0), (0, 0, 0)),
                "img must be int or Image",
            ),
            (
                lambda: pyxel.bltm3d(0, 0, 8, 8, "bad", (0, 0, 0), (0, 0, 0)),
                "tm must be int or Tilemap",
            ),
            (
                lambda: pyxel.Image(8, 8).blt(0, 0, "bad", 0, 0, 8, 8),
                "img must be int or Image",
            ),
            (
                lambda: pyxel.Image(8, 8).bltm(0, 0, "bad", 0, 0, 8, 8),
                "tm must be int or Tilemap",
            ),
            (
                lambda: pyxel.Image(8, 8).blt3d(
                    0, 0, 8, 8, "bad", (0, 0, 0), (0, 0, 0)
                ),
                "img must be int or Image",
            ),
            (
                lambda: pyxel.Image(8, 8).bltm3d(
                    0, 0, 8, 8, "bad", (0, 0, 0), (0, 0, 0)
                ),
                "tm must be int or Tilemap",
            ),
            (
                lambda: pyxel.Tilemap(8, 8, "bad"),
                "img must be int or Image",
            ),
            (
                lambda: setattr(pyxel.Tilemap(8, 8, 0), "imgsrc", "bad"),
                "imgsrc must be int or Image",
            ),
            (
                lambda: pyxel.Tilemap(8, 8, 0).blt(0, 0, "bad", 0, 0, 8, 8),
                "tm must be int or Tilemap",
            ),
        ],
        ids=[
            "play-snd",
            "channel-play-snd",
            "blt-img",
            "bltm-tm",
            "blt3d-img",
            "bltm3d-tm",
            "image-blt-img",
            "image-bltm-tm",
            "image-blt3d-img",
            "image-bltm3d-tm",
            "tilemap-constructor-img",
            "tilemap-imgsrc",
            "tilemap-blt-tm",
        ],
    )
    def test_polymorphic_argument_rejects_unsupported_type(self, operation, message):
        with raises_exact(TypeError, message):
            operation()

    def test_btn_wrong_type(self):
        with raises_exact(
            TypeError, "'str' object cannot be interpreted as an integer"
        ):
            pyxel.btn("not_a_key")  # type: ignore[arg-type]

    def test_sound_set_wrong_speed_type(self):
        snd = pyxel.Sound()
        with raises_exact(
            TypeError, "'str' object cannot be interpreted as an integer"
        ):
            snd.set("c2", "s", "7", "n", "fast")  # type: ignore[arg-type]

    def test_image_set_wrong_data_type(self):
        img = pyxel.Image(8, 8)
        with raises_exact(TypeError, "'int' object is not an instance of 'Sequence'"):
            img.set(0, 0, 12345)  # type: ignore[arg-type]

    def test_btnp_wrong_type(self):
        with raises_exact(
            TypeError, "'str' object cannot be interpreted as an integer"
        ):
            pyxel.btnp("not_a_key")  # type: ignore[arg-type]

    def test_btnr_wrong_type(self):
        with raises_exact(
            TypeError, "'str' object cannot be interpreted as an integer"
        ):
            pyxel.btnr("not_a_key")  # type: ignore[arg-type]


class TestIndexErrors:
    def test_images_out_of_range(self):
        with raises_exact(IndexError, "list index out of range"):
            _ = pyxel.images[999]

    def test_images_negative_out_of_range(self):
        with raises_exact(IndexError, "list index out of range"):
            _ = pyxel.images[-999]

    def test_sounds_out_of_range(self):
        with raises_exact(IndexError, "list index out of range"):
            _ = pyxel.sounds[999]

    def test_tilemaps_out_of_range(self):
        with raises_exact(IndexError, "list index out of range"):
            _ = pyxel.tilemaps[999]

    def test_colors_negative_out_of_range(self):
        with raises_exact(IndexError, "list index out of range"):
            _ = pyxel.colors[-9999]

    def test_channels_out_of_range(self):
        with raises_exact(IndexError, "list index out of range"):
            _ = pyxel.channels[999]

    def test_tones_out_of_range(self):
        with raises_exact(IndexError, "list index out of range"):
            _ = pyxel.tones[999]

    def test_musics_out_of_range(self):
        with raises_exact(IndexError, "list index out of range"):
            _ = pyxel.musics[999]

    def test_images_boundary_valid(self):
        assert isinstance(pyxel.images[pyxel.NUM_IMAGES - 1], pyxel.Image)
        assert isinstance(pyxel.images[-1], pyxel.Image)

    def test_sounds_boundary_valid(self):
        assert isinstance(pyxel.sounds[pyxel.NUM_SOUNDS - 1], pyxel.Sound)
        assert isinstance(pyxel.sounds[-1], pyxel.Sound)

    def test_tilemaps_boundary_valid(self):
        assert isinstance(pyxel.tilemaps[pyxel.NUM_TILEMAPS - 1], pyxel.Tilemap)
        assert isinstance(pyxel.tilemaps[-1], pyxel.Tilemap)

    @pytest.mark.parametrize(
        "operation",
        [
            lambda: pyxel.Sound().notes.pop(),
            lambda: pyxel.Music().seqs.pop(),
        ],
        ids=["generic-sequence", "music-seqs"],
    )
    def test_pop_from_empty_sequence(self, operation):
        with raises_exact(IndexError, "pop from empty list"):
            operation()


class TestAttributeErrors:
    def test_nonexistent_attribute(self):
        with raises_exact(
            AttributeError, "module 'pyxel' has no attribute 'nonexistent_attribute'"
        ):
            _ = pyxel.nonexistent_attribute  # type: ignore[attr-defined]

    def test_nonexistent_constant(self):
        with raises_exact(
            AttributeError, "module 'pyxel' has no attribute 'FAKE_CONSTANT'"
        ):
            _ = pyxel.FAKE_CONSTANT  # type: ignore[attr-defined]


class TestPartialArgErrors:
    def test_clip_partial_args(self):
        with raises_exact(TypeError, "clip() takes 0 or 4 arguments"):
            pyxel.clip(10, 20)  # type: ignore[call-overload]

    def test_clip_three_args(self):
        with raises_exact(TypeError, "clip() takes 0 or 4 arguments"):
            pyxel.clip(10, 20, 30)  # type: ignore[call-overload]

    def test_camera_one_arg(self):
        with raises_exact(TypeError, "camera() takes 0 or 2 arguments"):
            pyxel.camera(10)  # type: ignore[call-overload]

    def test_pal_one_arg(self):
        with raises_exact(TypeError, "pal() takes 0 or 2 arguments"):
            pyxel.pal(1)  # type: ignore[call-overload]


class TestValueErrors:
    @pytest.mark.parametrize(
        ("factory", "args", "message"),
        [
            (pyxel.Image, (65536, 65536), "image dimensions are too large"),
            (
                pyxel.Tilemap,
                (65536, 65536, 0),
                "tilemap dimensions are too large",
            ),
        ],
        ids=["image", "tilemap"],
    )
    def test_canvas_constructor_rejects_oversized_dimensions(
        self, factory, args, message
    ):
        with raises_exact(ValueError, message):
            factory(*args)

    @pytest.mark.parametrize(
        ("operation", "message"),
        [
            (lambda: pyxel.play(999, 0), "ch must be a valid channel index"),
            (lambda: pyxel.play(0, 9999), "snd must be a valid sound index"),
            (
                lambda: pyxel.play(0, [0, 9999]),
                "snd must contain only valid sound indices",
            ),
            (lambda: pyxel.playm(9999), "msc must be a valid music index"),
            (lambda: pyxel.stop(999), "ch must be a valid channel index"),
            (lambda: pyxel.play_pos(999), "ch must be a valid channel index"),
            (
                lambda: pyxel.blt(0, 0, 999, 0, 0, 8, 8),
                "img must be a valid image index",
            ),
            (
                lambda: pyxel.bltm(0, 0, 999, 0, 0, 8, 8),
                "tm must be a valid tilemap index",
            ),
            (
                lambda: pyxel.blt3d(0, 0, 8, 8, 999, (0, 0, 0), (0, 0, 0)),
                "img must be a valid image index",
            ),
            (
                lambda: pyxel.bltm3d(0, 0, 8, 8, 999, (0, 0, 0), (0, 0, 0)),
                "tm must be a valid tilemap index",
            ),
            (
                lambda: pyxel.Channel().play(999),
                "snd must be a valid sound index",
            ),
            (
                lambda: pyxel.Channel().play([0, 999]),
                "snd must contain only valid sound indices",
            ),
            (
                lambda: pyxel.Image(8, 8).blt(0, 0, 999, 0, 0, 8, 8),
                "img must be a valid image index",
            ),
            (
                lambda: pyxel.Image(8, 8).bltm(0, 0, 999, 0, 0, 8, 8),
                "tm must be a valid tilemap index",
            ),
            (
                lambda: pyxel.Image(8, 8).blt3d(0, 0, 8, 8, 999, (0, 0, 0), (0, 0, 0)),
                "img must be a valid image index",
            ),
            (
                lambda: pyxel.Image(8, 8).bltm3d(0, 0, 8, 8, 999, (0, 0, 0), (0, 0, 0)),
                "tm must be a valid tilemap index",
            ),
            (
                lambda: pyxel.Tilemap(8, 8, 0).blt(0, 0, 999, 0, 0, 8, 8),
                "tm must be a valid tilemap index",
            ),
        ],
        ids=[
            "play-ch",
            "play-snd",
            "play-snd-list",
            "playm-msc",
            "stop-ch",
            "play-pos-ch",
            "blt-img",
            "bltm-tm",
            "blt3d-img",
            "bltm3d-tm",
            "channel-play-snd",
            "channel-play-snd-list",
            "image-blt-img",
            "image-bltm-tm",
            "image-blt3d-img",
            "image-bltm3d-tm",
            "tilemap-blt-tm",
        ],
    )
    def test_resource_index_constraint_message(self, operation, message):
        with raises_exact(ValueError, message):
            operation()

    def test_legacy_resource_index_messages_in_isolated_process(self):
        code = """
import pyxel


def assert_value_error(operation, expected):
    try:
        operation()
    except ValueError as exc:
        assert str(exc) == expected
    else:
        raise AssertionError(f"ValueError not raised: {expected}")


checks = [
    (lambda: pyxel.channel(999), "ch must be a valid channel index"),
    (lambda: pyxel.sound(999), "snd must be a valid sound index"),
    (lambda: pyxel.music(999), "msc must be a valid music index"),
    (lambda: pyxel.image(999), "img must be a valid image index"),
    (lambda: pyxel.tilemap(999), "tm must be a valid tilemap index"),
]
for operation, expected in checks:
    assert_value_error(operation, expected)

tilemap = pyxel.Tilemap(8, 8, 0)
tilemap.refimg = 999
assert_value_error(
    lambda: tilemap.image,
    "imgsrc references an invalid image index",
)
"""

        result = subprocess.run(
            [sys.executable, "-c", code],
            capture_output=True,
            text=True,
        )

        assert result.returncode == 0, result.stdout + result.stderr

    @pytest.mark.parametrize(
        "dimensions",
        [{}, {"width": 8, "height": 8}],
        ids=["neither", "both"],
    )
    def test_scroll_bar_requires_exactly_one_dimension(self, dimensions):
        from pyxel.editor.widgets.scroll_bar import ScrollBar

        with raises_exact(
            ValueError, "width or height must be specified, but not both"
        ):
            ScrollBar(
                None,
                0,
                0,
                scroll_amount=10,
                slider_amount=1,
                value=0,
                **dimensions,
            )


class TestInlineDataErrors:
    @pytest.mark.parametrize(
        ("data", "message"),
        [
            ([], "Invalid image data: no rows"),
            (["  "], "Invalid image data at row 0: no pixels"),
            (
                ["01", "2"],
                "Invalid image data at row 1: expected 2 hexadecimal digits, got 1",
            ),
            (
                ["0g"],
                "Invalid image data at row 0, column 1: "
                "expected hexadecimal digit, got 'g'",
            ),
            (
                ["0あ"],
                "Invalid image data at row 0, column 1: "
                "expected hexadecimal digit, got 'あ'",
            ),
        ],
    )
    def test_image_set_rejects_malformed_data_without_writing(self, data, message):
        image = pyxel.Image(2, 2)
        image.cls(7)

        with raises_exact(ValueError, message):
            image.set(0, 0, data)
        assert [image.pget(x, y) for y in range(2) for x in range(2)] == [7] * 4

    @pytest.mark.parametrize(
        ("data", "message"),
        [
            ([], "Invalid tilemap data: no rows"),
            (["  "], "Invalid tilemap data at row 0: no tiles"),
            (
                ["000"],
                "Invalid tilemap data at row 0: hexadecimal digit count 3 "
                "is not divisible by 4",
            ),
            (
                ["00000000", "0000"],
                "Invalid tilemap data at row 1: expected 8 hexadecimal digits, got 4",
            ),
            (
                ["00z0"],
                "Invalid tilemap data at row 0, column 2: "
                "expected hexadecimal digit, got 'z'",
            ),
            (
                ["00あ0"],
                "Invalid tilemap data at row 0, column 2: "
                "expected hexadecimal digit, got 'あ'",
            ),
        ],
    )
    def test_tilemap_set_rejects_malformed_data_without_writing(self, data, message):
        tilemap = pyxel.Tilemap(2, 2, 0)
        tilemap.cls((7, 7))

        with raises_exact(ValueError, message):
            tilemap.set(0, 0, data)
        assert [tilemap.pget(x, y) for y in range(2) for x in range(2)] == [(7, 7)] * 4


class TestMmlErrors:
    # Binding raises plain Exception; pin via message to verify error specificity.
    def test_sound_mml_invalid_syntax(self):
        snd = pyxel.Sound()
        with raises_exact(Exception, "MML:0: Unexpected character 'Z'"):
            snd.mml("Z")

    def test_play_mml_invalid_syntax(self):
        with raises_exact(Exception, "MML:0: Unexpected character 'Z'"):
            pyxel.play(0, "Z")

    def test_sound_mml_old_syntax_uses_legacy_error_contract(self):
        code = """
import pyxel

snd = pyxel.Sound()
try:
    snd.mml("x8c")
except Exception as exc:
    assert str(exc) == "Invalid envelope value '8' in MML"
else:
    raise AssertionError("invalid legacy MML succeeded")
"""
        result = subprocess.run(
            [sys.executable, "-c", code],
            capture_output=True,
            text=True,
            timeout=10,
        )
        assert result.returncode == 0, result.stderr

    def test_sound_set_notes_invalid(self):
        snd = pyxel.Sound()
        with raises_exact(Exception, "Invalid sound note 'z'"):
            snd.set_notes("ZZZZZZ!!!")

    def test_sound_set_tones_invalid(self):
        snd = pyxel.Sound()
        with raises_exact(Exception, "Invalid sound tone 'z'"):
            snd.set_tones("ZZZZZZ!!!")

    def test_sound_set_volumes_invalid(self):
        snd = pyxel.Sound()
        with raises_exact(Exception, "Invalid sound volume 'z'"):
            snd.set_volumes("ZZZZZZ!!!")

    def test_sound_set_effects_invalid(self):
        snd = pyxel.Sound()
        with raises_exact(Exception, "Invalid sound effect 'z'"):
            snd.set_effects("ZZZZZZ!!!")


class TestFileErrors:
    def test_load_nonexistent_pyxres(self):
        with raises_exact(
            Exception, "Failed to open file '/nonexistent/path/file.pyxres'"
        ):
            pyxel.load("/nonexistent/path/file.pyxres")

    def test_load_nonexistent_image(self):
        with raises_exact(
            Exception, "Failed to open file '/nonexistent/path/image.png'"
        ):
            pyxel.Image.from_image("/nonexistent/path/image.png")

    def test_failed_from_image_keeps_palette(self):
        # A failed load must not clear the shared palette even with include_colors.
        colors_before = list(pyxel.colors)
        with raises_exact(
            Exception, "Failed to open file '/nonexistent/path/image.png'"
        ):
            pyxel.Image.from_image("/nonexistent/path/image.png", include_colors=True)
        assert list(pyxel.colors) == colors_before

    def test_load_nonexistent_font(self):
        with raises_exact(
            Exception, "Failed to open file '/nonexistent/path/font.bdf'"
        ):
            pyxel.Font("/nonexistent/path/font.bdf")

    def test_load_nonexistent_pcm(self):
        snd = pyxel.Sound()
        with raises_exact(
            Exception, "Failed to open file '/nonexistent/path/sound.wav'"
        ):
            snd.pcm("/nonexistent/path/sound.wav")


class TestPanicErrors:
    # Pin the exact type so a future migration to ValueError shows up as a test diff.

    def test_btnv_non_analog_key_panics(self, panic_exception):
        with raises_exact(
            panic_exception, "button_value is called with a non-analog key 0x61"
        ):
            pyxel.btnv(pyxel.KEY_A)

    def test_gen_bgm_invalid_preset_panics(self, panic_exception):
        with raises_exact(panic_exception, "invalid preset"):
            pyxel.gen_bgm(99, 0, 0, 1)

    def test_gen_bgm_invalid_transpose_panics(self, panic_exception):
        with raises_exact(panic_exception, "invalid transpose"):
            pyxel.gen_bgm(0, 99, 0, 1)

    def test_gen_bgm_invalid_instrumentation_panics(self, panic_exception):
        with raises_exact(panic_exception, "invalid instrumentation"):
            pyxel.gen_bgm(0, 0, 99, 1)
