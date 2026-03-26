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

    def test_seqs_inner_seq_access(self):
        msc = pyxel.Music()
        msc.set([0, 1, 2], [3, 4])
        seq0 = msc.seqs[0]
        assert len(seq0) == 3
        assert seq0[0] == 0
        assert seq0[1] == 1

    def test_seqs_inner_seq_setitem(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3])
        msc.seqs[0][0] = 5
        assert msc.seqs[0][0] == 5

    def test_seqs_inner_seq_append(self):
        msc = pyxel.Music()
        msc.set([0])
        original_len = len(msc.seqs[0])
        msc.seqs[0].append(10)
        assert len(msc.seqs[0]) == original_len + 1
        assert msc.seqs[0][-1] == 10

    def test_seqs_append_new_channel(self):
        msc = pyxel.Music()
        msc.set([0])
        original_channels = len(msc.seqs)
        msc.seqs.append([5, 6])
        assert len(msc.seqs) == original_channels + 1

    def test_seqs_setitem_channel(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3])
        msc.seqs[0] = [10, 11, 12]
        assert len(msc.seqs[0]) == 3
        assert msc.seqs[0][0] == 10

    def test_seqs_delitem(self):
        msc = pyxel.Music()
        msc.set([0], [1], [2])
        # set() pads to NUM_CHANNELS (4)
        original_len = len(msc.seqs)
        del msc.seqs[-1]
        assert len(msc.seqs) == original_len - 1

    def test_seqs_insert(self):
        msc = pyxel.Music()
        msc.set([0], [1])
        original_len = len(msc.seqs)
        msc.seqs.insert(1, [5, 6])
        assert len(msc.seqs) == original_len + 1
        assert msc.seqs[1][0] == 5

    def test_seqs_pop(self):
        msc = pyxel.Music()
        msc.set([0], [1], [2])
        original_len = len(msc.seqs)
        popped = msc.seqs.pop()
        assert len(msc.seqs) == original_len - 1
        assert isinstance(popped, list)

    def test_seqs_clear(self):
        msc = pyxel.Music()
        msc.set([0], [1])
        msc.seqs.clear()
        assert len(msc.seqs) == 0

    def test_seqs_extend(self):
        msc = pyxel.Music()
        msc.set([0])
        original_len = len(msc.seqs)
        msc.seqs.extend([[1, 2], [3, 4]])
        assert len(msc.seqs) == original_len + 2

    def test_seqs_slice_access(self):
        msc = pyxel.Music()
        msc.set([0], [1], [2])
        sliced = msc.seqs[0:2]
        assert isinstance(sliced, list)
        assert len(sliced) == 2

    def test_seqs_reversed(self):
        msc = pyxel.Music()
        msc.set([0], [1], [2])
        original_len = len(msc.seqs)
        rev = list(reversed(msc.seqs))
        assert len(rev) == original_len

    def test_seqs_repr(self):
        msc = pyxel.Music()
        msc.set([0, 1])
        r = repr(msc.seqs)
        assert isinstance(r, str)
        assert "Seqs" in r

    def test_seqs_bool(self):
        msc = pyxel.Music()
        msc.set([0])
        assert bool(msc.seqs)
        msc.seqs.clear()
        assert not bool(msc.seqs)
