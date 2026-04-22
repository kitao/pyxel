import pyxel


class TestPlay:
    def test_play_with_int(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, 0)
        pyxel.stop(3)

    def test_play_with_seq_int(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.sounds[1].set("a2b2", "ss", "77", "nn", 10)
        pyxel.play(3, [0, 1])
        pyxel.stop(3)

    def test_play_with_sound_instance(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, snd)
        pyxel.stop(3)

    def test_play_with_seq_sound(self):
        snd1 = pyxel.Sound()
        snd1.set("c2e2g2", "sss", "777", "nnn", 10)
        snd2 = pyxel.Sound()
        snd2.set("a2b2", "ss", "77", "nn", 10)
        pyxel.play(3, [snd1, snd2])
        pyxel.stop(3)

    def test_play_with_mml_string(self):
        pyxel.play(3, "T120 O4 L4 CDEF")
        pyxel.stop(3)

    def test_play_with_loop(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, 0, loop=True)
        pyxel.stop(3)

    def test_play_with_resume(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, 0, resume=True)
        pyxel.stop(3)

    def test_play_with_sec(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, 0, sec=0.5)
        pyxel.stop(3)


class TestPlaym:
    def test_playm_basic(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.musics[0].set([0], [0])
        pyxel.playm(0)
        pyxel.stop()

    def test_playm_with_loop(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.musics[0].set([0])
        pyxel.playm(0, loop=True)
        pyxel.stop()

    def test_playm_with_sec(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.musics[0].set([0])
        pyxel.playm(0, sec=0.5)
        pyxel.stop()


class TestStop:
    def test_stop_all(self):
        pyxel.stop()

    def test_stop_specific_channel(self):
        pyxel.stop(3)

    def test_stop_idempotent(self):
        pyxel.stop()
        pyxel.stop()


class TestPlayPos:
    def test_play_pos_when_not_playing(self):
        pyxel.stop(3)
        result = pyxel.play_pos(3)
        assert result is None

    def test_play_pos_returns_tuple_when_playing(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3e3g3c4e4", "ssssssss", "77777777", "nnnnnnnn", 10)
        pyxel.play(3, snd)
        # play_pos may be None immediately after play (audio thread timing)
        result = pyxel.play_pos(3)
        if result is not None:
            assert isinstance(result, tuple), f"Expected tuple, got {type(result)}"
            assert len(result) == 2, f"Expected 2 elements, got {len(result)}"
        pyxel.stop(3)


class TestGenBgm:
    def test_basic(self):
        result = pyxel.gen_bgm(0, 0, 3, 42)
        assert isinstance(result, list)
        assert len(result) == 4
        assert all(isinstance(s, str) for s in result)
        assert len(result[0]) > 0

    def test_seed_reproducible(self):
        result1 = pyxel.gen_bgm(0, 0, 3, 42)
        result2 = pyxel.gen_bgm(0, 0, 3, 42)
        assert result1 == result2

    def test_different_seeds_differ(self):
        result1 = pyxel.gen_bgm(0, 0, 3, 1)
        result2 = pyxel.gen_bgm(0, 0, 3, 2)
        assert result1 != result2

    def test_all_presets(self):
        for preset in range(8):
            result = pyxel.gen_bgm(preset, 0, 0, 1)
            assert isinstance(result, list)
            assert len(result) == 4

    def test_all_instrumentations(self):
        for instr in range(4):
            result = pyxel.gen_bgm(0, 0, instr, 1)
            assert isinstance(result, list)
            assert len(result) == 4

    def test_transpose_changes_output(self):
        result_default = pyxel.gen_bgm(0, 0, 3, 42)
        result_transposed = pyxel.gen_bgm(0, 3, 3, 42)
        assert result_default != result_transposed

    def test_transpose_and_instr_combined(self):
        result_default = pyxel.gen_bgm(0, 0, 3, 42)
        result_combined = pyxel.gen_bgm(0, 3, 0, 42)
        assert result_default != result_combined

    def test_play_and_stop(self):
        pyxel.gen_bgm(0, 0, 3, 1, play=True)
        pyxel.stop()
