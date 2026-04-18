from pathlib import Path

import pyxel


# Music class
class TestMusic:
    def test_new(self):
        msc = pyxel.Music()
        assert len(msc.seqs) == 0

    def test_set_pads_to_num_channels(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3])
        # set() pads to NUM_CHANNELS (4)
        assert len(msc.seqs) == pyxel.NUM_CHANNELS

    def test_set_preserves_data(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3])
        assert list(msc.seqs[0]) == [0, 1]
        assert list(msc.seqs[1]) == [2, 3]
        assert list(msc.seqs[2]) == []
        assert list(msc.seqs[3]) == []

    def test_set_single_channel(self):
        msc = pyxel.Music()
        msc.set([0, 1, 2])
        assert len(msc.seqs) == pyxel.NUM_CHANNELS
        assert list(msc.seqs[0]) == [0, 1, 2]

    def test_seqs_property(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3], [4])
        assert len(msc.seqs) == pyxel.NUM_CHANNELS
        assert list(msc.seqs[2]) == [4]

    def test_save(self, tmp_path):
        pyxel.sounds[0].set("c2e2g2c3", "ssss", "7654", "nnnn", 10)
        msc = pyxel.Music()
        msc.set([0])
        path = str(tmp_path / "test_music.wav")
        msc.save(path, 1.0)
        assert Path(path).exists()
        assert Path(path).stat().st_size > 0

    def test_seqs_inner_seq_access(self):
        msc = pyxel.Music()
        msc.set([0, 1, 2], [3, 4])
        seq0 = msc.seqs[0]
        assert len(seq0) == 3
        assert seq0[0] == 0
        assert seq0[1] == 1
        assert seq0[2] == 2

    def test_seqs_inner_seq_setitem(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3])
        msc.seqs[0][0] = 5
        assert msc.seqs[0][0] == 5
        # Other elements unchanged
        assert msc.seqs[0][1] == 1

    def test_seqs_inner_seq_append(self):
        msc = pyxel.Music()
        msc.set([0])
        original_len = len(msc.seqs[0])
        msc.seqs[0].append(10)
        assert len(msc.seqs[0]) == original_len + 1
        assert msc.seqs[0][-1] == 10

    def test_seqs_inner_seq_delitem(self):
        msc = pyxel.Music()
        msc.set([0, 1, 2])
        del msc.seqs[0][1]
        assert len(msc.seqs[0]) == 2
        assert list(msc.seqs[0]) == [0, 2]

    def test_seqs_append_new_channel(self):
        msc = pyxel.Music()
        msc.set([0])
        original_channels = len(msc.seqs)
        msc.seqs.append([5, 6])
        assert len(msc.seqs) == original_channels + 1
        assert list(msc.seqs[-1]) == [5, 6]

    def test_seqs_setitem_channel(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3])
        msc.seqs[0] = [10, 11, 12]
        assert len(msc.seqs[0]) == 3
        assert list(msc.seqs[0]) == [10, 11, 12]
        # Other channel unchanged
        assert list(msc.seqs[1]) == [2, 3]

    def test_seqs_delitem(self):
        msc = pyxel.Music()
        msc.set([0], [1], [2])
        original_len = len(msc.seqs)
        del msc.seqs[-1]
        assert len(msc.seqs) == original_len - 1

    def test_seqs_insert(self):
        msc = pyxel.Music()
        msc.set([0], [1])
        original_len = len(msc.seqs)
        msc.seqs.insert(1, [5, 6])
        assert len(msc.seqs) == original_len + 1
        assert list(msc.seqs[1]) == [5, 6]

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
        msc.set([10], [20], [30])
        rev = list(reversed(msc.seqs))
        assert len(rev) == len(msc.seqs)
        # Last channel becomes first in reversed
        assert list(rev[0]) == list(msc.seqs[-1])

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

    def test_seqs_iadd(self):
        msc = pyxel.Music()
        msc.set([0])
        original_len = len(msc.seqs)
        # seqs property is read-only, so use local variable for +=
        seqs = msc.seqs
        seqs += [[5, 6], [7, 8]]
        assert len(msc.seqs) == original_len + 2
        assert list(msc.seqs[-2]) == [5, 6]
        assert list(msc.seqs[-1]) == [7, 8]

    def test_set_overwrites_previous(self):
        msc = pyxel.Music()
        msc.set([0, 1, 2])
        msc.set([10])
        assert list(msc.seqs[0]) == [10]
        assert list(msc.seqs[1]) == []
