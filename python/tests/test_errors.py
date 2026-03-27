import pytest
import pyxel


class TestTypeErrors:
    def test_sin_with_string(self):
        with pytest.raises(TypeError):
            pyxel.sin("abc")

    def test_pset_with_string_x(self):
        with pytest.raises(TypeError):
            pyxel.pset("a", 0, 0)

    def test_clamp_with_string(self):
        with pytest.raises(TypeError):
            pyxel.clamp("a", 0, 10)

    def test_rect_wrong_types(self):
        with pytest.raises(TypeError):
            pyxel.rect("a", "b", "c", "d", "e")

    def test_blt_wrong_img_type(self):
        with pytest.raises(TypeError):
            pyxel.blt(0, 0, "not_an_image", 0, 0, 8, 8)

    def test_play_wrong_snd_type(self):
        with pytest.raises(TypeError):
            pyxel.play(0, 3.14)

    def test_btn_wrong_type(self):
        with pytest.raises(TypeError):
            pyxel.btn("not_a_key")

    def test_tilemap_wrong_imgsrc_type(self):
        with pytest.raises(TypeError):
            pyxel.Tilemap(8, 8, "bad")

    def test_sound_set_wrong_speed_type(self):
        with pytest.raises(TypeError):
            snd = pyxel.Sound()
            snd.set("c2", "s", "7", "n", "fast")

    def test_image_set_wrong_data_type(self):
        with pytest.raises(TypeError):
            img = pyxel.Image(8, 8)
            img.set(0, 0, 12345)

    def test_btnp_wrong_type(self):
        with pytest.raises(TypeError):
            pyxel.btnp("not_a_key")

    def test_btnr_wrong_type(self):
        with pytest.raises(TypeError):
            pyxel.btnr("not_a_key")


class TestIndexErrors:
    def test_images_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.images[999]

    def test_images_negative_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.images[-999]

    def test_sounds_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.sounds[999]

    def test_tilemaps_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.tilemaps[999]

    def test_colors_negative_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.colors[-9999]

    def test_channels_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.channels[999]

    def test_tones_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.tones[999]

    def test_musics_out_of_range(self):
        with pytest.raises(IndexError):
            _ = pyxel.musics[999]

    def test_images_boundary_valid(self):
        # Last valid index should not raise
        _ = pyxel.images[pyxel.NUM_IMAGES - 1]
        _ = pyxel.images[-1]

    def test_sounds_boundary_valid(self):
        _ = pyxel.sounds[pyxel.NUM_SOUNDS - 1]
        _ = pyxel.sounds[-1]

    def test_tilemaps_boundary_valid(self):
        _ = pyxel.tilemaps[pyxel.NUM_TILEMAPS - 1]
        _ = pyxel.tilemaps[-1]


class TestAttributeErrors:
    def test_nonexistent_attribute(self):
        with pytest.raises(AttributeError):
            _ = pyxel.nonexistent_attribute

    def test_nonexistent_constant(self):
        with pytest.raises(AttributeError):
            _ = pyxel.FAKE_CONSTANT


class TestPartialArgErrors:
    def test_clip_partial_args(self):
        with pytest.raises(TypeError):
            pyxel.clip(10, 20)

    def test_clip_three_args(self):
        with pytest.raises(TypeError):
            pyxel.clip(10, 20, 30)

    def test_camera_one_arg(self):
        with pytest.raises(TypeError):
            pyxel.camera(10)

    def test_pal_one_arg(self):
        with pytest.raises(TypeError):
            pyxel.pal(1)


class TestValueErrors:
    def test_play_invalid_channel(self):
        with pytest.raises(ValueError):
            pyxel.play(999, 0)

    def test_play_invalid_sound_index(self):
        with pytest.raises(ValueError):
            pyxel.play(0, 9999)

    def test_playm_invalid_music_index(self):
        with pytest.raises(ValueError):
            pyxel.playm(9999)

    def test_stop_invalid_channel(self):
        with pytest.raises(ValueError):
            pyxel.stop(999)

    def test_play_pos_invalid_channel(self):
        with pytest.raises(ValueError):
            pyxel.play_pos(999)

    def test_play_invalid_sound_list(self):
        with pytest.raises(ValueError):
            pyxel.play(0, [0, 9999])


class TestMmlErrors:
    def test_sound_mml_invalid_syntax(self):
        snd = pyxel.Sound()
        with pytest.raises(Exception):
            snd.mml("ZZZZZZ!!!")

    def test_play_mml_invalid_syntax(self):
        with pytest.raises(Exception):
            pyxel.play(0, "ZZZZZZ!!!")

    def test_sound_set_notes_invalid(self):
        snd = pyxel.Sound()
        with pytest.raises(Exception):
            snd.set_notes("ZZZZZZ!!!")

    def test_sound_set_tones_invalid(self):
        snd = pyxel.Sound()
        with pytest.raises(Exception):
            snd.set_tones("ZZZZZZ!!!")


class TestFileErrors:
    def test_load_nonexistent_pyxres(self):
        with pytest.raises(Exception):
            pyxel.load("/nonexistent/path/file.pyxres")

    def test_load_nonexistent_image(self):
        with pytest.raises(Exception):
            pyxel.Image.from_image("/nonexistent/path/image.png")

    def test_font_nonexistent(self):
        with pytest.raises(Exception):
            pyxel.Font("/nonexistent/path/font.bdf")
