import os

import pyxel


# Music class
class TestMusic:
    def test_new(self):
        pyxel.Music()

    def test_set(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3])
        assert len(msc.seqs) >= 2

    def test_seqs_property(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3], [4])
        assert len(msc.seqs) >= 3

    def test_save(self, tmp_path):
        pyxel.sounds[0].set("c2e2g2c3", "ssss", "7654", "nnnn", 10)
        msc = pyxel.Music()
        msc.set([0])
        path = str(tmp_path / "test_music.wav")
        msc.save(path, 1.0)
        assert os.path.exists(path)
