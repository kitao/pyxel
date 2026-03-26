import pyxel


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

    def test_stop(self):
        ch = pyxel.Channel()
        ch.play("T120 O4 L4 CDEF")
        ch.stop()

    def test_play_pos(self):
        ch = pyxel.Channel()
        result = ch.play_pos()
        assert result is None or isinstance(result, tuple)
