import subprocess
import sys

import pytest

import pyxel


class TestTypeErrors:
    def test_sin_wrong_deg_type(self):
        with pytest.raises(TypeError, match="must be real number, not str"):
            pyxel.sin("abc")  # type: ignore[arg-type]

    def test_pset_wrong_x_type(self):
        with pytest.raises(TypeError, match="must be real number, not str"):
            pyxel.pset("a", 0, 0)  # type: ignore[arg-type]

    def test_clamp_wrong_x_type(self):
        with pytest.raises(TypeError, match="must be real number, not str"):
            pyxel.clamp("a", 0, 10)  # type: ignore[arg-type]

    def test_rect_wrong_types(self):
        with pytest.raises(TypeError, match="must be real number, not str"):
            pyxel.rect("a", "b", "c", "d", "e")  # type: ignore[arg-type]

    def test_blt_wrong_img_type(self):
        with pytest.raises(TypeError, match="must be u32, Image"):
            pyxel.blt(0, 0, "not_an_image", 0, 0, 8, 8)  # type: ignore[arg-type]

    def test_play_wrong_snd_type(self):
        with pytest.raises(TypeError, match="must be u32, Vec<u32>, Sound"):
            pyxel.play(0, 3.14)  # type: ignore[arg-type]

    def test_btn_wrong_type(self):
        with pytest.raises(
            TypeError, match="'str' object cannot be interpreted as an integer"
        ):
            pyxel.btn("not_a_key")  # type: ignore[arg-type]

    def test_tilemap_wrong_imgsrc_type(self):
        with pytest.raises(TypeError, match="must be u32, Image"):
            pyxel.Tilemap(8, 8, "bad")  # type: ignore[arg-type]

    def test_sound_set_wrong_speed_type(self):
        snd = pyxel.Sound()
        with pytest.raises(
            TypeError, match="'str' object cannot be interpreted as an integer"
        ):
            snd.set("c2", "s", "7", "n", "fast")  # type: ignore[arg-type]

    def test_image_set_wrong_data_type(self):
        img = pyxel.Image(8, 8)
        with pytest.raises(
            TypeError, match="'int' object is not an instance of 'Sequence'"
        ):
            img.set(0, 0, 12345)  # type: ignore[arg-type]

    def test_btnp_wrong_type(self):
        with pytest.raises(
            TypeError, match="'str' object cannot be interpreted as an integer"
        ):
            pyxel.btnp("not_a_key")  # type: ignore[arg-type]

    def test_btnr_wrong_type(self):
        with pytest.raises(
            TypeError, match="'str' object cannot be interpreted as an integer"
        ):
            pyxel.btnr("not_a_key")  # type: ignore[arg-type]


class TestIndexErrors:
    def test_images_out_of_range(self):
        with pytest.raises(IndexError, match="list index out of range"):
            _ = pyxel.images[999]

    def test_images_negative_out_of_range(self):
        with pytest.raises(IndexError, match="list index out of range"):
            _ = pyxel.images[-999]

    def test_sounds_out_of_range(self):
        with pytest.raises(IndexError, match="list index out of range"):
            _ = pyxel.sounds[999]

    def test_tilemaps_out_of_range(self):
        with pytest.raises(IndexError, match="list index out of range"):
            _ = pyxel.tilemaps[999]

    def test_colors_negative_out_of_range(self):
        with pytest.raises(IndexError, match="list index out of range"):
            _ = pyxel.colors[-9999]

    def test_channels_out_of_range(self):
        with pytest.raises(IndexError, match="list index out of range"):
            _ = pyxel.channels[999]

    def test_tones_out_of_range(self):
        with pytest.raises(IndexError, match="list index out of range"):
            _ = pyxel.tones[999]

    def test_musics_out_of_range(self):
        with pytest.raises(IndexError, match="list index out of range"):
            _ = pyxel.musics[999]

    def test_images_boundary_valid(self):
        # Last valid index should not raise
        assert isinstance(pyxel.images[pyxel.NUM_IMAGES - 1], pyxel.Image)
        assert isinstance(pyxel.images[-1], pyxel.Image)

    def test_sounds_boundary_valid(self):
        assert isinstance(pyxel.sounds[pyxel.NUM_SOUNDS - 1], pyxel.Sound)
        assert isinstance(pyxel.sounds[-1], pyxel.Sound)

    def test_tilemaps_boundary_valid(self):
        assert isinstance(pyxel.tilemaps[pyxel.NUM_TILEMAPS - 1], pyxel.Tilemap)
        assert isinstance(pyxel.tilemaps[-1], pyxel.Tilemap)


class TestAttributeErrors:
    def test_nonexistent_attribute(self):
        with pytest.raises(
            AttributeError,
            match="module 'pyxel' has no attribute 'nonexistent_attribute'",
        ):
            _ = pyxel.nonexistent_attribute  # type: ignore[attr-defined]

    def test_nonexistent_constant(self):
        with pytest.raises(
            AttributeError, match="module 'pyxel' has no attribute 'FAKE_CONSTANT'"
        ):
            _ = pyxel.FAKE_CONSTANT  # type: ignore[attr-defined]


class TestPartialArgErrors:
    def test_clip_partial_args(self):
        with pytest.raises(TypeError, match=r"clip\(\) takes 0 or 4 arguments"):
            pyxel.clip(10, 20)  # type: ignore[call-overload]

    def test_clip_three_args(self):
        with pytest.raises(TypeError, match=r"clip\(\) takes 0 or 4 arguments"):
            pyxel.clip(10, 20, 30)  # type: ignore[call-overload]

    def test_camera_one_arg(self):
        with pytest.raises(TypeError, match=r"camera\(\) takes 0 or 2 arguments"):
            pyxel.camera(10)  # type: ignore[call-overload]

    def test_pal_one_arg(self):
        with pytest.raises(TypeError, match=r"pal\(\) takes 0 or 2 arguments"):
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
        with pytest.raises(ValueError) as exc:
            factory(*args)

        assert str(exc.value) == message

    def test_play_invalid_channel(self):
        with pytest.raises(ValueError, match="Invalid channel index"):
            pyxel.play(999, 0)

    def test_play_invalid_sound_index(self):
        with pytest.raises(ValueError, match="Invalid sound index"):
            pyxel.play(0, 9999)

    def test_playm_invalid_music_index(self):
        with pytest.raises(ValueError, match="Invalid music index"):
            pyxel.playm(9999)

    def test_stop_invalid_channel(self):
        with pytest.raises(ValueError, match="Invalid channel index"):
            pyxel.stop(999)

    def test_play_pos_invalid_channel(self):
        with pytest.raises(ValueError, match="Invalid channel index"):
            pyxel.play_pos(999)

    def test_play_invalid_sound_list(self):
        with pytest.raises(ValueError, match="Invalid sound index"):
            pyxel.play(0, [0, 9999])  # type: ignore[arg-type]


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

        with pytest.raises(ValueError) as exc:
            image.set(0, 0, data)

        assert str(exc.value) == message
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

        with pytest.raises(ValueError) as exc:
            tilemap.set(0, 0, data)

        assert str(exc.value) == message
        assert [tilemap.pget(x, y) for y in range(2) for x in range(2)] == [(7, 7)] * 4


class TestMmlErrors:
    # Binding raises plain Exception; pin via message to verify error specificity.
    def test_sound_mml_invalid_syntax(self):
        snd = pyxel.Sound()
        with pytest.raises(Exception) as exc:
            snd.mml("Z")
        assert str(exc.value) == "MML:0: Unexpected character 'Z'"

    def test_play_mml_invalid_syntax(self):
        with pytest.raises(Exception) as exc:
            pyxel.play(0, "Z")
        assert str(exc.value) == "MML:0: Unexpected character 'Z'"

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
        with pytest.raises(Exception, match="Invalid sound note"):
            snd.set_notes("ZZZZZZ!!!")

    def test_sound_set_tones_invalid(self):
        snd = pyxel.Sound()
        with pytest.raises(Exception, match="Invalid sound tone"):
            snd.set_tones("ZZZZZZ!!!")

    def test_sound_set_volumes_invalid(self):
        snd = pyxel.Sound()
        with pytest.raises(Exception, match="Invalid sound volume"):
            snd.set_volumes("ZZZZZZ!!!")

    def test_sound_set_effects_invalid(self):
        snd = pyxel.Sound()
        with pytest.raises(Exception, match="Invalid sound effect"):
            snd.set_effects("ZZZZZZ!!!")


class TestFileErrors:
    def test_load_nonexistent_pyxres(self):
        with pytest.raises(Exception, match="Failed to open file"):
            pyxel.load("/nonexistent/path/file.pyxres")

    def test_load_nonexistent_image(self):
        with pytest.raises(Exception, match="Failed to open file"):
            pyxel.Image.from_image("/nonexistent/path/image.png")

    def test_failed_from_image_keeps_palette(self):
        # A failed load must not clear the shared palette even with include_colors.
        colors_before = list(pyxel.colors)
        with pytest.raises(Exception, match="Failed to open file"):
            pyxel.Image.from_image("/nonexistent/path/image.png", include_colors=True)
        assert list(pyxel.colors) == colors_before

    def test_load_nonexistent_font(self):
        with pytest.raises(Exception, match="Failed to open file"):
            pyxel.Font("/nonexistent/path/font.bdf")

    def test_load_nonexistent_pcm(self):
        snd = pyxel.Sound()
        with pytest.raises(Exception, match="Failed to open file"):
            snd.pcm("/nonexistent/path/sound.wav")


class TestPanicErrors:
    # Pin the exact type so a future migration to ValueError shows up as a test diff.

    def test_btnv_non_analog_key_panics(self, panic_exception):
        with pytest.raises(panic_exception, match="non-analog key"):
            pyxel.btnv(pyxel.KEY_A)

    def test_gen_bgm_invalid_preset_panics(self, panic_exception):
        with pytest.raises(panic_exception, match="invalid preset"):
            pyxel.gen_bgm(99, 0, 0, 1)

    def test_gen_bgm_invalid_transpose_panics(self, panic_exception):
        with pytest.raises(panic_exception, match="invalid transpose"):
            pyxel.gen_bgm(0, 99, 0, 1)

    def test_gen_bgm_invalid_instrumentation_panics(self, panic_exception):
        with pytest.raises(panic_exception, match="invalid instrumentation"):
            pyxel.gen_bgm(0, 0, 99, 1)
