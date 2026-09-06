import pytest
import pyxel


class TestChannel:
    def test_new_defaults(self):
        ch = pyxel.Channel()
        assert ch.gain == 0.125
        assert ch.detune == 0

    def test_gain_read_write(self):
        ch = pyxel.Channel()
        ch.gain = 0.5
        assert ch.gain == 0.5
        ch.gain = 1.0
        assert ch.gain == 1.0

    def test_detune_read_write(self):
        ch = pyxel.Channel()
        ch.detune = 10
        assert ch.detune == 10
        ch.detune = 0
        assert ch.detune == 0

    @pytest.mark.parametrize("form", ["index", "indices", "sound", "sounds", "mml"])
    def test_play_sound_forms(self, form):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        sounds = {
            "index": 0,
            "indices": [0, 0],
            "sound": snd,
            "sounds": [snd, snd],
            "mml": "T120 O4 L4 CDEF",
        }
        ch = pyxel.Channel()
        ch.play(sounds[form])
        assert ch.play_pos() == (0, 0.0)
        ch.stop()
        assert ch.play_pos() is None

    def test_play_loop_wraps_seek(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(snd, sec=0.5, loop=True)
        assert ch.play_pos() == (0, 2.626031346153468e-05)

    def test_play_resumes_after_seeking_past_interruption(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(snd, loop=True)
        ch.play("T120 L4 C", sec=1, resume=True)
        assert ch.play_pos() == (0, 5.196189522393979e-05)

    @pytest.mark.parametrize("options", [{}, {"tick": None}])
    def test_play_with_sec(self, options):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(snd, sec=0.125, **options)
        assert ch.play_pos() == (0, 0.1250002086162567)

    def test_play_pos_when_not_playing(self):
        ch = pyxel.Channel()
        ch.stop()
        result = ch.play_pos()
        assert result is None

    def test_append_to_global_channels(self):
        original_len = len(pyxel.channels)
        ch = pyxel.Channel()
        ch.gain = 0.5
        ch.detune = 5
        pyxel.channels.append(ch)
        try:
            assert len(pyxel.channels) == original_len + 1
            assert pyxel.channels[-1].gain == 0.5
            assert pyxel.channels[-1].detune == 5
        finally:
            pyxel.channels.pop()
        assert len(pyxel.channels) == original_len

    def test_play_with_tick_deprecated(self, capfd):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        ch = pyxel.Channel()
        ch.play(snd, sec=0, tick=15)  # type: ignore[call-arg]
        out = capfd.readouterr().out
        assert (
            out
            == "tick option of Channel.play is deprecated. Use sec in seconds (tick / 120) instead.\n"
        )
        assert ch.play_pos() == (0, 0.1250002086162567)
        ch.stop()
