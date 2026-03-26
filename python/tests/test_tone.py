import pyxel


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
