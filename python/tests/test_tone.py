import pyxel


class TestTone:
    def test_new_defaults(self):
        tone = pyxel.Tone()
        assert isinstance(tone.mode, int)
        assert isinstance(tone.gain, float)
        assert isinstance(tone.sample_bits, int)
        assert len(tone.wavetable) == 0

    def test_mode_read_write(self):
        tone = pyxel.Tone()
        tone.mode = 1
        assert tone.mode == 1
        tone.mode = 0
        assert tone.mode == 0

    def test_gain_read_write(self):
        tone = pyxel.Tone()
        tone.gain = 0.5
        assert tone.gain == 0.5
        tone.gain = 1.0
        assert tone.gain == 1.0

    def test_sample_bits_read_write(self):
        tone = pyxel.Tone()
        tone.sample_bits = 8
        assert tone.sample_bits == 8
        tone.sample_bits = 16
        assert tone.sample_bits == 16

    def test_wavetable_append_and_read(self):
        tone = pyxel.Tone()
        tone.wavetable.append(0)
        tone.wavetable.append(127)
        tone.wavetable.append(255)
        assert len(tone.wavetable) == 3
        assert tone.wavetable[0] == 0
        assert tone.wavetable[1] == 127
        assert tone.wavetable[2] == 255

    def test_wavetable_setitem(self):
        tone = pyxel.Tone()
        tone.wavetable.append(0)
        tone.wavetable[0] = 64
        assert tone.wavetable[0] == 64

    def test_append_to_global_tones(self):
        original_len = len(pyxel.tones)
        tone = pyxel.Tone()
        tone.mode = 1
        pyxel.tones.append(tone)
        try:
            assert len(pyxel.tones) == original_len + 1
        finally:
            pyxel.tones.pop()
        assert len(pyxel.tones) == original_len
