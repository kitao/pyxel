import pyxel


class TestPlay:
    """Test play() with all Union[int, Seq[int], Sound, Seq[Sound], str] variants."""

    def test_play_with_int(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, 0)

    def test_play_with_seq_int(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.sounds[1].set("a2b2", "ss", "77", "nn", 10)
        pyxel.play(3, [0, 1])

    def test_play_with_sound_instance(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, snd)

    def test_play_with_seq_sound(self):
        snd1 = pyxel.Sound()
        snd1.set("c2e2g2", "sss", "777", "nnn", 10)
        snd2 = pyxel.Sound()
        snd2.set("a2b2", "ss", "77", "nn", 10)
        pyxel.play(3, [snd1, snd2])

    def test_play_with_mml_string(self):
        pyxel.play(3, "T120 O4 L4 CDEF")


class TestPlaym:
    def test_playm_basic(self):
        pyxel.musics[0].set([0], [1])
        pyxel.playm(0)

    def test_playm_with_loop(self):
        pyxel.musics[0].set([0])
        pyxel.playm(0, loop=True)
        pyxel.stop()


class TestStop:
    def test_stop_all(self):
        pyxel.stop()

    def test_stop_specific_channel(self):
        pyxel.stop(3)


class TestPlayPos:
    def test_play_pos_returns_optional_tuple(self):
        result = pyxel.play_pos(3)
        # When not playing, should return None
        assert result is None or isinstance(result, tuple)


class TestGenBgm:
    def test_returns_list_of_str(self):
        result = pyxel.gen_bgm(0, 0)
        assert isinstance(result, list)
        assert all(isinstance(s, str) for s in result)

    def test_seed_reproducible(self):
        result1 = pyxel.gen_bgm(0, 0, seed=42)
        result2 = pyxel.gen_bgm(0, 0, seed=42)
        assert result1 == result2
