import os
import pyxel


# Image class
class TestImage:
    def test_new_dimensions(self):
        img = pyxel.Image(64, 48)
        assert img.width == 64
        assert img.height == 48

    def test_pset_pget(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.pset(3, 3, 8)
        assert img.pget(3, 3) == 8

    def test_set_data(self):
        img = pyxel.Image(4, 2)
        img.set(0, 0, ["0123", "4567"])
        assert img.pget(0, 0) == 0
        assert img.pget(3, 0) == 3
        assert img.pget(0, 1) == 4

    def test_clear(self):
        img = pyxel.Image(8, 8)
        img.pset(0, 0, 7)
        img.cls(0)
        assert img.pget(0, 0) == 0

    def test_blt_with_int(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.blt(0, 0, 0, 0, 0, 8, 8)

    def test_blt_with_image_instance(self):
        src = pyxel.Image(16, 16)
        src.cls(0)
        src.pset(0, 0, 5)
        dst = pyxel.Image(16, 16)
        dst.cls(0)
        dst.blt(0, 0, src, 0, 0, 8, 8)
        assert dst.pget(0, 0) == 5

    def test_bltm_with_int(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        img.bltm(0, 0, 0, 0, 0, 64, 64)

    def test_bltm_with_tilemap_instance(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        tm = pyxel.Tilemap(8, 8, 0)
        img.bltm(0, 0, tm, 0, 0, 64, 64)

    def test_from_image(self, assets_dir):
        img = pyxel.Image.from_image(os.path.join(assets_dir, "cat_16x16.png"))
        assert img.width == 16
        assert img.height == 16

    def test_load_image_file(self, assets_dir):
        img = pyxel.Image(32, 32)
        img.load(0, 0, os.path.join(assets_dir, "cat_16x16.png"))
        # Verify something was loaded (not all zeros)
        has_nonzero = any(img.pget(x, 0) != 0 for x in range(16))
        assert has_nonzero

    def test_save(self, tmp_path):
        img = pyxel.Image(8, 8)
        img.cls(0)
        img.pset(0, 0, 7)
        path = str(tmp_path / "test_img.png")
        img.save(path, 1)
        assert os.path.exists(path)

    def test_line(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.line(0, 0, 15, 0, 7)
        assert img.pget(0, 0) == 7
        assert img.pget(8, 0) == 7
        assert img.pget(0, 8) == 0  # Not on the line

    def test_rect(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rect(2, 2, 4, 4, 5)
        assert img.pget(3, 3) == 5  # Inside
        assert img.pget(0, 0) == 0  # Outside

    def test_rectb(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rectb(2, 2, 4, 4, 5)
        assert img.pget(2, 2) == 5  # Border
        assert img.pget(3, 3) == 0  # Inside hollow

    def test_circ(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.circ(16, 16, 5, 8)
        assert img.pget(16, 16) == 8  # Center

    def test_circb(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.circb(16, 16, 5, 8)
        assert img.pget(16, 16) == 0  # Center is hollow

    def test_elli(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.elli(8, 8, 16, 8, 3)
        assert img.pget(16, 12) == 3  # Inside ellipse center area

    def test_ellib(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.ellib(8, 8, 16, 8, 3)
        # Center should be hollow
        assert img.pget(16, 12) == 0

    def test_tri(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.tri(8, 0, 0, 15, 15, 15, 9)
        assert img.pget(8, 8) == 9  # Inside triangle

    def test_trib(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.trib(8, 0, 0, 15, 15, 15, 9)
        assert img.pget(8, 8) == 0  # Inside is hollow

    def test_fill(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.rect(2, 2, 8, 8, 5)
        img.fill(4, 4, 10)
        assert img.pget(4, 4) == 10

    def test_text(self):
        img = pyxel.Image(64, 16)
        img.cls(0)
        img.text(0, 0, "A", 7)
        # At least one pixel should be drawn
        has_text = any(img.pget(x, y) == 7 for x in range(4) for y in range(6))
        assert has_text

    def test_clip_restricts_drawing(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.clip(4, 4, 8, 8)
        img.rect(0, 0, 16, 16, 7)  # Try to fill entire image
        img.clip()  # Reset
        assert img.pget(0, 0) == 0  # Outside clip area
        assert img.pget(6, 6) == 7  # Inside clip area

    def test_camera_offsets_drawing(self):
        img = pyxel.Image(32, 32)
        img.cls(0)
        img.camera(10, 10)
        img.pset(10, 10, 7)  # With camera offset, draws at (0, 0)
        img.camera()  # Reset
        assert img.pget(0, 0) == 7

    def test_pal_color_replacement(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.pal(7, 8)  # Replace color 7 with 8 when drawing
        img.pset(0, 0, 7)
        img.pal()  # Reset
        assert img.pget(0, 0) == 8

    def test_dither(self):
        img = pyxel.Image(16, 16)
        img.cls(0)
        img.dither(0.5)
        img.rect(0, 0, 16, 16, 7)
        img.dither(1.0)  # Reset
        # With 50% dither, some pixels should be drawn, some not
        drawn = sum(1 for x in range(16) for y in range(16) if img.pget(x, y) == 7)
        assert 0 < drawn < 256  # Some but not all

    def test_blt3d(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        img.blt3d(0, 0, 64, 64, 0, (0, 0, 10), (45, 0, 0))

    def test_bltm3d(self):
        img = pyxel.Image(64, 64)
        img.cls(0)
        tm = pyxel.Tilemap(8, 8, 0)
        img.bltm3d(0, 0, 64, 64, tm, (0, 0, 10), (45, 0, 0))


# Tilemap class
class TestTilemap:
    def test_new_with_int(self):
        tm = pyxel.Tilemap(32, 32, 0)
        assert tm.width == 32
        assert tm.height == 32

    def test_new_with_image_instance(self):
        img = pyxel.Image(256, 256)
        tm = pyxel.Tilemap(16, 16, img)
        assert tm.width == 16

    def test_imgsrc_read_write_int(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.imgsrc = 1
        assert tm.imgsrc == 1

    def test_imgsrc_read_write_image(self):
        img = pyxel.Image(256, 256)
        tm = pyxel.Tilemap(8, 8, 0)
        tm.imgsrc = img

    def test_pset_pget_returns_tuple(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.pset(0, 0, (1, 2))
        result = tm.pget(0, 0)
        assert result == (1, 2)
        assert isinstance(result, tuple)

    def test_clear(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.pset(0, 0, (5, 5))
        tm.cls((0, 0))
        assert tm.pget(0, 0) == (0, 0)

    def test_blt_with_int(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.blt(0, 0, 0, 0, 0, 8, 8)

    def test_blt_with_tilemap_instance(self):
        src = pyxel.Tilemap(8, 8, 0)
        dst = pyxel.Tilemap(8, 8, 0)
        dst.blt(0, 0, src, 0, 0, 8, 8)

    def test_blt_with_tilekey(self):
        src = pyxel.Tilemap(8, 8, 0)
        dst = pyxel.Tilemap(8, 8, 0)
        dst.blt(0, 0, src, 0, 0, 8, 8, tilekey=(0, 0))

    def test_from_tmx(self, assets_dir):
        tm = pyxel.Tilemap.from_tmx(os.path.join(assets_dir, "urban_rpg.tmx"), 0)
        assert tm.width > 0
        assert tm.height > 0

    def test_collide_no_walls(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        dx, dy = tm.collide(0, 0, 8, 8, 5.0, 5.0, [])
        assert dx == 5.0
        assert dy == 5.0

    def test_set_data(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.set(0, 0, ["0001 0002", "0003 0004"])
        assert tm.pget(0, 0) == (0, 1)
        assert tm.pget(1, 0) == (0, 2)

    def test_load_tmx(self, assets_dir):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.load(0, 0, os.path.join(assets_dir, "urban_rpg.tmx"), 0)
        # Verify something was loaded
        has_nonzero = any(tm.pget(x, 0) != (0, 0) for x in range(32))
        assert has_nonzero

    def test_line(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.line(0, 0, 15, 0, (1, 1))
        assert tm.pget(0, 0) == (1, 1)
        assert tm.pget(8, 0) == (1, 1)

    def test_rect(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.rect(2, 2, 4, 4, (1, 2))
        assert tm.pget(3, 3) == (1, 2)
        assert tm.pget(0, 0) == (0, 0)

    def test_rectb(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.rectb(2, 2, 4, 4, (3, 3))
        assert tm.pget(2, 2) == (3, 3)
        assert tm.pget(3, 3) == (0, 0)

    def test_circ(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.circ(16, 16, 5, (2, 2))
        assert tm.pget(16, 16) == (2, 2)

    def test_circb(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.circb(16, 16, 5, (2, 2))
        assert tm.pget(16, 16) == (0, 0)

    def test_elli(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.elli(8, 8, 16, 8, (1, 1))
        assert tm.pget(16, 12) == (1, 1)

    def test_ellib(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.ellib(8, 8, 16, 8, (1, 1))
        assert tm.pget(16, 12) == (0, 0)

    def test_tri(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.tri(8, 0, 0, 15, 15, 15, (4, 4))
        assert tm.pget(8, 8) == (4, 4)

    def test_trib(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.trib(8, 0, 0, 15, 15, 15, (4, 4))
        assert tm.pget(8, 8) == (0, 0)

    def test_fill(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.rect(2, 2, 8, 8, (5, 5))
        tm.fill(4, 4, (9, 9))
        assert tm.pget(4, 4) == (9, 9)

    def test_clip_restricts_drawing(self):
        tm = pyxel.Tilemap(16, 16, 0)
        tm.cls((0, 0))
        tm.clip(4, 4, 8, 8)
        tm.rect(0, 0, 16, 16, (1, 1))
        tm.clip()
        assert tm.pget(0, 0) == (0, 0)
        assert tm.pget(6, 6) == (1, 1)

    def test_camera_offsets_drawing(self):
        tm = pyxel.Tilemap(32, 32, 0)
        tm.cls((0, 0))
        tm.camera(10, 10)
        tm.pset(10, 10, (3, 3))
        tm.camera()
        assert tm.pget(0, 0) == (3, 3)

    def test_collide_with_walls(self):
        tm = pyxel.Tilemap(8, 8, 0)
        tm.cls((0, 0))
        wall_tile = (1, 0)
        tm.pset(2, 0, wall_tile)  # Place a wall
        dx, dy = tm.collide(0, 0, 8, 8, 100.0, 0.0, [wall_tile])
        assert dx < 100.0  # Should be blocked


# Sound class
class TestSound:
    def test_new(self):
        snd = pyxel.Sound()
        assert snd.speed >= 0

    def test_set(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7654", "nnnn", 10)
        assert len(snd.notes) == 4

    def test_set_notes(self):
        snd = pyxel.Sound()
        snd.set_notes("c2e2g2")
        assert len(snd.notes) == 3

    def test_mml(self):
        snd = pyxel.Sound()
        snd.mml("T120 O4 L4 CDEF")

    def test_mml_none_exits_mml_mode(self):
        snd = pyxel.Sound()
        snd.mml("T120 O4 CDEF")
        snd.mml(None)

    def test_pcm(self, assets_dir):
        snd = pyxel.Sound()
        snd.pcm(os.path.join(assets_dir, "audio_bgm1.ogg"))

    def test_pcm_none_exits_pcm_mode(self):
        snd = pyxel.Sound()
        snd.pcm(None)

    def test_properties(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 15)
        assert snd.speed == 15
        assert len(snd.tones) == 2
        assert len(snd.volumes) == 2
        assert len(snd.effects) == 2

    def test_set_tones(self):
        snd = pyxel.Sound()
        snd.set_tones("ttss ppnn")
        assert len(snd.tones) == 8

    def test_set_volumes(self):
        snd = pyxel.Sound()
        snd.set_volumes("7654 3210")
        assert len(snd.volumes) == 8
        assert snd.volumes[0] == 7

    def test_set_effects(self):
        snd = pyxel.Sound()
        snd.set_effects("nsvf hqnn")
        assert len(snd.effects) == 8

    def test_save(self, tmp_path):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7654", "nnnn", 10)
        path = str(tmp_path / "test_snd.wav")
        snd.save(path, 1.0)
        assert os.path.exists(path)

    def test_total_sec_finite(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7654", "nnnn", 30)
        result = snd.total_sec()
        assert result is not None
        assert isinstance(result, float)
        assert result > 0


# Music class
class TestMusic:
    def test_new(self):
        pyxel.Music()

    def test_set(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3])
        assert len(msc.seqs) >= 2

    def test_seqs_property(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3], [4])
        assert len(msc.seqs) >= 3

    def test_save(self, tmp_path):
        pyxel.sounds[0].set("c2e2g2c3", "ssss", "7654", "nnnn", 10)
        msc = pyxel.Music()
        msc.set([0])
        path = str(tmp_path / "test_music.wav")
        msc.save(path, 1.0)
        assert os.path.exists(path)


# Tone class
class TestTone:
    def test_new(self):
        pyxel.Tone()

    def test_mode_read_write(self):
        tone = pyxel.Tone()
        tone.mode = 1
        assert tone.mode == 1

    def test_gain_read_write(self):
        tone = pyxel.Tone()
        tone.gain = 0.5
        assert tone.gain == 0.5

    def test_sample_bits_read_write(self):
        tone = pyxel.Tone()
        tone.sample_bits = 8
        assert tone.sample_bits == 8

    def test_wavetable_read_write(self):
        tone = pyxel.Tone()
        # New Tone has empty wavetable; append a value and verify round-trip
        tone.wavetable.append(127)
        assert tone.wavetable[0] == 127


# Channel class
class TestChannel:
    def test_new(self):
        pyxel.Channel()

    def test_gain_read_write(self):
        ch = pyxel.Channel()
        ch.gain = 0.5
        assert ch.gain == 0.5

    def test_detune_read_write(self):
        ch = pyxel.Channel()
        ch.detune = 10
        assert ch.detune == 10

    def test_play_with_int(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(0)

    def test_play_with_seq_int(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play([0, 0])

    def test_play_with_sound_instance(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(snd)

    def test_play_with_seq_sound(self):
        snd1 = pyxel.Sound()
        snd1.set("c2e2g2", "sss", "777", "nnn", 10)
        snd2 = pyxel.Sound()
        snd2.set("a2b2", "ss", "77", "nn", 10)
        ch = pyxel.Channel()
        ch.play([snd1, snd2])

    def test_play_with_mml_string(self):
        ch = pyxel.Channel()
        ch.play("T120 O4 L4 CDEF")

    def test_stop(self):
        ch = pyxel.Channel()
        ch.play("T120 O4 L4 CDEF")
        ch.stop()

    def test_play_pos(self):
        ch = pyxel.Channel()
        result = ch.play_pos()
        assert result is None or isinstance(result, tuple)


# Font class
class TestFont:
    def test_bdf(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        assert font.text_width("A") > 0

    def test_ttf(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "PixelMplus10-Regular.ttf"), 10)
        assert font.text_width("A") > 0

    def test_ttf_different_sizes(self, assets_dir):
        font_small = pyxel.Font(os.path.join(assets_dir, "PixelMplus10-Regular.ttf"), 8)
        font_large = pyxel.Font(
            os.path.join(assets_dir, "PixelMplus10-Regular.ttf"), 20
        )
        assert font_large.text_width("A") > font_small.text_width("A")

    def test_text_width_empty(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        assert font.text_width("") == 0

    def test_text_width_multibyte(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        width = font.text_width("あ")
        assert width > 0

    def test_text_width_multiline(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        w_single = font.text_width("AB")
        w_multi = font.text_width("AB\nA")
        # Multiline returns max line width
        assert w_multi == w_single

    def test_text_width_invisible_chars_skipped(self, assets_dir):
        font = pyxel.Font(os.path.join(assets_dir, "umplus_j10r.bdf"))
        w_plain = font.text_width("Hi")
        w_zwj = font.text_width("H\u200di")  # ZWJ
        w_vs = font.text_width("H\ufe0fi")  # Variation selector
        assert w_plain == w_zwj
        assert w_plain == w_vs


# Resource I/O
class TestResourceIO:
    def test_load_pyxres(self, assets_dir):
        pyxel.load(os.path.join(assets_dir, "sample.pyxres"))

    def test_save_load_roundtrip(self, tmp_path):
        # Set up known data
        img = pyxel.images[0]
        img.cls(0)
        img.pset(0, 0, 7)
        snd = pyxel.sounds[0]
        snd.set("c2e2g2", "sss", "777", "nnn", 10)

        # Save
        path = str(tmp_path / "test.pyxres")
        pyxel.save(path)

        # Modify data
        img.cls(0)
        snd.set("a2", "s", "7", "n", 5)

        # Load and verify restored
        pyxel.load(path)
        assert pyxel.images[0].pget(0, 0) == 7
        assert len(pyxel.sounds[0].notes) == 3

    def test_save_exclude_images(self, tmp_path):
        pyxel.images[0].cls(0)
        pyxel.images[0].pset(0, 0, 9)
        path = str(tmp_path / "test_excl.pyxres")
        pyxel.save(path, exclude_images=True)

        pyxel.images[0].cls(0)
        pyxel.load(path)
        # Image data was excluded, so pixel should still be 0
        assert pyxel.images[0].pget(0, 0) == 0

    def test_load_pal(self, assets_dir):
        pyxel.load_pal(os.path.join(assets_dir, "audio_bgm.pyxpal"))

    def test_save_load_pal_roundtrip(self, tmp_path):
        original_color = pyxel.colors[0]
        path = str(tmp_path / "test.pyxpal")
        pyxel.save_pal(path)

        pyxel.colors[0] = 0xFFFFFF
        pyxel.load_pal(path)
        assert pyxel.colors[0] == original_color

    def test_load_nonexistent_file_raises(self):
        import pytest

        with pytest.raises(Exception):
            pyxel.load("/nonexistent/path/file.pyxres")

    def test_screenshot(self, tmp_path):
        pyxel.cls(7)
        pyxel.flip()
        path = str(tmp_path / "test_screenshot.png")
        pyxel.screenshot(path)
        assert os.path.exists(path)

    def test_screencast(self, tmp_path):
        # In headless mode, flip() doesn't capture frames,
        # so screencast produces no GIF. Verify it doesn't raise.
        pyxel.cls(5)
        pyxel.flip()
        path = str(tmp_path / "test_screencast.gif")
        pyxel.screencast(path)

    def test_reset_screencast(self):
        pyxel.reset_screencast()
        # Should not raise
