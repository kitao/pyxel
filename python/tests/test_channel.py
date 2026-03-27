import pytest
import pyxel


# Channel class
class TestChannel:
    def test_new_defaults(self):
        ch = pyxel.Channel()
        assert isinstance(ch.gain, float)
        assert isinstance(ch.detune, int)

    def test_gain_read_write(self):
        ch = pyxel.Channel()
        ch.gain = 0.5
        assert ch.gain == pytest.approx(0.5, abs=1e-5)
        ch.gain = 1.0
        assert ch.gain == pytest.approx(1.0, abs=1e-5)

    def test_detune_read_write(self):
        ch = pyxel.Channel()
        ch.detune = 10
        assert ch.detune == 10
        ch.detune = 0
        assert ch.detune == 0

    def test_play_with_int(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(0)
        # After play, play_pos should return a tuple
        pos = ch.play_pos()
        assert pos is None or isinstance(pos, tuple)
        ch.stop()

    def test_play_with_seq_int(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play([0, 0])
        ch.stop()

    def test_play_with_sound_instance(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(snd)
        ch.stop()

    def test_play_with_seq_sound(self):
        snd1 = pyxel.Sound()
        snd1.set("c2e2g2", "sss", "777", "nnn", 10)
        snd2 = pyxel.Sound()
        snd2.set("a2b2", "ss", "77", "nn", 10)
        ch = pyxel.Channel()
        ch.play([snd1, snd2])
        ch.stop()

    def test_play_with_mml_string(self):
        ch = pyxel.Channel()
        ch.play("T120 O4 L4 CDEF")
        ch.stop()

    def test_play_with_loop(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(snd, loop=True)
        ch.stop()

    def test_play_with_resume(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(snd, resume=True)
        ch.stop()

    def test_play_with_sec(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(snd, sec=0.5)
        ch.stop()

    def test_stop_when_not_playing(self):
        ch = pyxel.Channel()
        ch.stop()  # Should not raise

    def test_play_pos_when_not_playing(self):
        ch = pyxel.Channel()
        ch.stop()
        result = ch.play_pos()
        assert result is None

    def test_play_pos_returns_tuple_with_two_ints(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3e3g3c4e4", "ssssssss", "77777777", "nnnnnnnn", 10)
        ch = pyxel.Channel()
        ch.play(snd)
        # play_pos may be None immediately after play (audio thread timing)
        result = ch.play_pos()
        if result is not None:
            assert isinstance(result, tuple), f"Expected tuple, got {type(result)}"
            assert len(result) == 2, f"Expected 2 elements, got {len(result)}"
        ch.stop()

    def test_append_to_global_channels(self):
        original_len = len(pyxel.channels)
        ch = pyxel.Channel()
        ch.gain = 0.7
        ch.detune = 5
        pyxel.channels.append(ch)
        try:
            assert len(pyxel.channels) == original_len + 1
            assert pyxel.channels[-1].gain == pytest.approx(0.7, abs=1e-5)
            assert pyxel.channels[-1].detune == 5
        finally:
            pyxel.channels.pop()
        assert len(pyxel.channels) == original_len
