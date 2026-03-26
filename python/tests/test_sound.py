import os

import pyxel


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

    def test_set_verifies_notes(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7654", "nnnn", 10)
        assert len(snd.notes) == 4
        assert snd.speed == 10

    def test_set_verifies_tones(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "sp", "77", "nn", 10)
        assert list(snd.tones) == [1, 2]  # s=1, p=2

    def test_set_verifies_volumes(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "75", "nn", 10)
        assert list(snd.volumes) == [7, 5]

    def test_set_verifies_effects(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2c3", "ssss", "7777", "nsvf", 10)
        assert list(snd.effects) == [0, 1, 2, 3]  # n=0, s=1, v=2, f=3

    def test_mml_produces_audio(self):
        # In MML mode, notes array stays empty; verify audio via total_sec
        snd = pyxel.Sound()
        snd.mml("T120 O4 L4 CDEF")
        assert snd.total_sec() > 0

    def test_speed_setter(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        snd.speed = 20
        assert snd.speed == 20

    def test_notes_append(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        original_len = len(snd.notes)
        snd.notes.append(60)  # Add a note
        assert len(snd.notes) == original_len + 1

    def test_notes_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        snd.notes[0] = 99
        assert snd.notes[0] == 99

    def test_notes_delitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        original_len = len(snd.notes)
        del snd.notes[-1]
        assert len(snd.notes) == original_len - 1

    def test_tones_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        snd.tones[0] = 2  # pulse
        assert snd.tones[0] == 2

    def test_volumes_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        snd.volumes[0] = 3
        assert snd.volumes[0] == 3

    def test_effects_setitem(self):
        snd = pyxel.Sound()
        snd.set("c2e2", "ss", "77", "nn", 10)
        snd.effects[0] = 2  # vibrato
        assert snd.effects[0] == 2


