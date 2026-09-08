import subprocess
import sys
from pathlib import Path

import pytest
import pyxel


class TestSound:
    def test_new_defaults(self):
        snd = pyxel.Sound()
        assert snd.speed == 30
        assert len(snd.notes) == 0
        assert len(snd.tones) == 0
        assert len(snd.volumes) == 0
        assert len(snd.effects) == 0

    def test_set_notes_values(self):
        snd = pyxel.Sound()
        snd.set_notes("c2d2e2f2g2a2b2")
        # C major scale at octave 2: note = letter offset + octave * 12.
        assert list(snd.notes) == [24, 26, 28, 29, 31, 33, 35]

    def test_set_notes_rest(self):
        snd = pyxel.Sound()
        snd.set_notes("c2r e2")
        assert list(snd.notes) == [24, -1, 28]

    def test_set_all_tone_types(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "tspn", "7777", "nnnn", 10)
        assert list(snd.notes) == [24, 28, 31, 36]
        assert snd.speed == 10
        assert list(snd.tones) == [0, 1, 2, 3]  # t=Triangle, s=Square, p=Pulse, n=Noise

    def test_set_verifies_volumes(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "75", "nn", 10)
        assert list(snd.volumes) == [7, 5]

    def test_set_all_effect_types(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3e3g3", "ssssss", "777777", "nsvfhq", 10)
        assert list(snd.effects) == [0, 1, 2, 3, 4, 5]

    def test_set_tones_string(self):
        snd = pyxel.Sound()
        snd.set_tones("ttss ppnn")
        assert list(snd.tones) == [0, 0, 1, 1, 2, 2, 3, 3]

    def test_set_volumes_string(self):
        snd = pyxel.Sound()
        snd.set_volumes("7654 3210")
        assert list(snd.volumes) == [7, 6, 5, 4, 3, 2, 1, 0]

    def test_set_effects_string(self):
        snd = pyxel.Sound()
        snd.set_effects("nsvf hqnn")
        assert list(snd.effects) == [0, 1, 2, 3, 4, 5, 0, 0]

    def test_save_is_byte_deterministic(self, tmp_path):
        tone = pyxel.tones[0]
        original_tone = (
            tone.mode,
            tone.sample_bits,
            list(tone.wavetable),
            tone.gain,
        )

        try:
            tone.mode = 0
            tone.sample_bits = 2
            tone.wavetable[:] = [0, 3, 3, 0]
            tone.gain = 0.75

            snd = pyxel.Sound()
            snd.set("c2r e2g2", "0000", "7531", "nfhq", 3)
            first_path = str(tmp_path / "first.wav")
            second_path = str(tmp_path / "second.wav")

            # Include a loop restart in both renders.
            snd.save(first_path, 0.2)
            snd.save(second_path, 0.2)
            actual = Path(first_path).read_bytes()
            assert Path(second_path).read_bytes() == actual
        finally:
            tone.mode = original_tone[0]
            tone.sample_bits = original_tone[1]
            tone.wavetable[:] = original_tone[2]
            tone.gain = original_tone[3]

    def test_save_out_of_range_tone_falls_back(self, tmp_path):
        # Tone numbers beyond the tone list fall back to tone 0, like playback.
        snd = pyxel.Sound()
        snd.set("c2e2g2", "9", "7", "n", 30)
        path = str(tmp_path / "test_snd_tone9.wav")
        snd.save(path, 0.5)

        expected_path = str(tmp_path / "test_snd_tone0.wav")
        snd.set_tones("0")
        snd.save(expected_path, 0.5)
        assert Path(path).read_bytes() == Path(expected_path).read_bytes()

    def test_save_before_init(self, tmp_path):
        # Sound.save renders without a window and must work before pyxel.init.
        path = tmp_path / "no_init.wav"
        code = (
            "import pyxel\n"
            "snd = pyxel.Sound()\n"
            'snd.set("c2e2", "tt", "77", "nn", 10)\n'
            f"snd.save({str(path)!r}, 0.1)\n"
        )

        result = subprocess.run(
            [sys.executable, "-c", code],
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
        assert result.returncode == 0, result.stderr
        assert path.stat().st_size > 0

    def test_total_sec(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7654", "nnnn", 30)
        # 4 notes x 30 ticks at 120 ticks per second.
        assert snd.total_sec() == 1.0

    def test_speed_setter(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        snd.speed = 20
        assert snd.speed == 20

    def test_set_overwrites_previous(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7654", "nnnn", 10)
        assert len(snd.notes) == 4

        snd.set("c2e2", "ss", "77", "nn", 20)
        assert len(snd.notes) == 2
        assert snd.speed == 20


class TestSoundMml:
    def test_mml(self):
        snd = pyxel.Sound()
        snd.mml("T120 O4 L4 CDEF")
        # 4 quarter notes at 120 BPM = 2 s (within clock rounding).
        assert snd.total_sec() == 1.9999496936798096

    @pytest.mark.parametrize("args", [(), (None,)])
    def test_mml_clear_exits_mml_mode(self, args):
        snd = pyxel.Sound()
        snd.mml("T120 O4 CDEF")
        assert snd.total_sec() > 0.0

        snd.mml(*args)
        assert snd.total_sec() == 0.0

    def test_mml_after_set(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        snd.mml("T120 O4 L4 CDEF")
        # MML mode takes over: 2 s instead of the notes-based 0.25 s.
        assert snd.total_sec() == 1.9999496936798096


class TestSoundPcm:
    def test_pcm(self, assets_dir):
        snd = pyxel.Sound()
        snd.pcm(str(assets_dir / "audio_bgm1.ogg"))
        # The bundled asset decodes to a fixed gapless-trimmed length at 22050 Hz.
        assert snd.total_sec() == 53.33333206176758

    @pytest.mark.parametrize("args", [(), (None,)])
    def test_pcm_clear_exits_pcm_mode(self, assets_dir, args):
        snd = pyxel.Sound()
        snd.pcm(str(assets_dir / "audio_bgm1.ogg"))
        assert snd.total_sec() > 0.0

        snd.pcm(*args)
        assert snd.total_sec() == 0.0


class TestSoundProperties:
    def test_notes_append(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        original_len = len(snd.notes)
        snd.notes.append(60)
        assert len(snd.notes) == original_len + 1
        assert snd.notes[-1] == 60

    def test_notes_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        original_note1 = snd.notes[1]
        snd.notes[0] = 99
        assert snd.notes[0] == 99
        assert snd.notes[1] == original_note1

    def test_notes_delitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        original_len = len(snd.notes)
        del snd.notes[-1]
        assert len(snd.notes) == original_len - 1

    def test_tones_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        snd.tones[0] = 2
        assert snd.tones[0] == 2
        assert snd.tones[1] == 1

    def test_volumes_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        snd.volumes[0] = 3
        assert snd.volumes[0] == 3
        assert snd.volumes[1] == 7

    def test_effects_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        snd.effects[0] = 2
        assert snd.effects[0] == 2
        assert snd.effects[1] == 0
