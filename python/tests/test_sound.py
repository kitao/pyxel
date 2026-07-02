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

    def test_set(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7654", "nnnn", 10)
        assert len(snd.notes) == 4
        assert snd.speed == 10

    def test_set_notes(self):
        snd = pyxel.Sound()
        snd.set_notes("c2e2g2")
        assert len(snd.notes) == 3

    def test_set_notes_values(self):
        snd = pyxel.Sound()
        snd.set_notes("c2d2e2f2g2a2b2")
        # C major scale at octave 2: note = letter offset + octave * 12.
        assert list(snd.notes) == [24, 26, 28, 29, 31, 33, 35]

    def test_set_notes_rest(self):
        snd = pyxel.Sound()
        snd.set_notes("c2r e2")
        assert len(snd.notes) == 3
        assert snd.notes[1] == -1  # Rest note

    def test_set_verifies_tones(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "sp", "77", "nn", 10)
        assert list(snd.tones) == [1, 2]  # s=Square(1), p=Pulse(2)

    def test_set_all_tone_types(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "tspn", "7777", "nnnn", 10)
        assert list(snd.tones) == [0, 1, 2, 3]  # t=Triangle, s=Square, p=Pulse, n=Noise

    def test_set_verifies_volumes(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "75", "nn", 10)
        assert list(snd.volumes) == [7, 5]

    def test_set_verifies_effects(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7777", "nsvf", 10)
        assert list(snd.effects) == [
            0,
            1,
            2,
            3,
        ]  # n=None, s=Slide, v=Vibrato, f=FadeOut

    def test_set_all_effect_types(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3e3g3", "ssssss", "777777", "nsvfhq", 10)
        assert list(snd.effects) == [0, 1, 2, 3, 4, 5]

    def test_set_tones_string(self):
        snd = pyxel.Sound()
        snd.set_tones("ttss ppnn")
        assert len(snd.tones) == 8
        assert snd.tones[0] == 0  # t=Triangle
        assert snd.tones[4] == 2  # p=Pulse

    def test_set_volumes_string(self):
        snd = pyxel.Sound()
        snd.set_volumes("7654 3210")
        assert len(snd.volumes) == 8
        assert snd.volumes[0] == 7
        assert snd.volumes[7] == 0

    def test_set_effects_string(self):
        snd = pyxel.Sound()
        snd.set_effects("nsvf hqnn")
        assert len(snd.effects) == 8
        assert snd.effects[0] == 0  # n=None
        assert snd.effects[2] == 2  # v=Vibrato

    def test_save(self, tmp_path):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7654", "nnnn", 10)
        path = str(tmp_path / "test_snd.wav")
        snd.save(path, 1.0)
        assert Path(path).exists()
        assert Path(path).stat().st_size > 0

    def test_save_out_of_range_tone_falls_back(self, tmp_path):
        # Tone numbers beyond the tone list fall back to tone 0, like playback.
        snd = pyxel.Sound()
        snd.set("c2e2g2", "9", "7", "n", 30)
        path = str(tmp_path / "test_snd_tone9.wav")
        snd.save(path, 0.5)
        assert Path(path).stat().st_size > 0

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
        assert snd.total_sec() == pytest.approx(2.0, abs=1e-3)

    def test_mml_none_exits_mml_mode(self):
        snd = pyxel.Sound()
        snd.mml("T120 O4 CDEF")
        assert snd.total_sec() > 0.0
        snd.mml(None)
        # Back on the notes-based path, which is empty for a fresh sound.
        assert snd.total_sec() == 0.0

    def test_mml_after_set(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        snd.mml("T120 O4 L4 CDEF")
        # MML mode takes over: 2 s instead of the notes-based 0.25 s.
        assert snd.total_sec() == pytest.approx(2.0, abs=1e-3)

    def test_mml_old_syntax_emits_deprecation(self, capfd):
        snd = pyxel.Sound()
        # 'x' character triggers old MML syntax detection.
        snd.mml("T120 X1 O4 CDEF")
        out = capfd.readouterr().out
        assert "deprecated" in out.lower()
        # Old syntax: 4 notes x 4 steps (default l8) x 7 ticks (t120).
        assert snd.total_sec() == pytest.approx(16 * 7 / 120, abs=1e-3)

    def test_old_mml_emits_deprecation(self, capfd):
        snd = pyxel.Sound()
        snd.old_mml("T120 O4 L4 CDEF")  # type: ignore[attr-defined]
        out = capfd.readouterr().out
        assert "deprecated" in out.lower()
        # 4 notes x 8 steps (l4) x 7 ticks (t120).
        assert snd.total_sec() == pytest.approx(32 * 7 / 120, abs=1e-3)


class TestSoundPcm:
    def test_pcm(self, assets_dir):
        snd = pyxel.Sound()
        snd.pcm(str(assets_dir / "audio_bgm1.ogg"))
        # The bundled asset decodes to a fixed gapless-trimmed length at 22050 Hz.
        assert snd.total_sec() == pytest.approx(53.333332, abs=1e-4)

    def test_pcm_none_exits_pcm_mode(self, assets_dir):
        snd = pyxel.Sound()
        snd.pcm(str(assets_dir / "audio_bgm1.ogg"))
        assert snd.total_sec() > 0.0
        snd.pcm(None)
        # Back on the notes-based path, which is empty for a fresh sound.
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
        assert snd.notes[1] == original_note1  # Other notes unchanged

    def test_notes_delitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        original_len = len(snd.notes)
        del snd.notes[-1]
        assert len(snd.notes) == original_len - 1

    def test_tones_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        snd.tones[0] = 2  # Pulse
        assert snd.tones[0] == 2
        assert snd.tones[1] == 1  # Unchanged

    def test_volumes_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        snd.volumes[0] = 3
        assert snd.volumes[0] == 3
        assert snd.volumes[1] == 7  # Unchanged

    def test_effects_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        snd.effects[0] = 2  # Vibrato
        assert snd.effects[0] == 2
        assert snd.effects[1] == 0  # Unchanged
