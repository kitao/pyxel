import subprocess
import sys

import pytest
import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]


class TestMusic:
    def test_new(self):
        msc = pyxel.Music()
        assert len(msc.seqs) == 0

    @pytest.mark.parametrize(
        ("seqs", "expected"),
        [
            ([[0, 1], [2, 3]], [[0, 1], [2, 3], [], []]),
            (
                [[0], [1], [2], [3], [4, 5], [6, 7]],
                [[0], [1], [2], [3], [4, 5], [6, 7]],
            ),
        ],
    )
    def test_set_preserves_data_and_pads_channels(self, seqs, expected):
        msc = pyxel.Music()
        msc.set(*seqs)
        assert [list(seq) for seq in msc.seqs] == expected

    def test_set_single_channel(self):
        msc = pyxel.Music()
        msc.set([0, 1, 2])
        assert [list(seq) for seq in msc.seqs] == [[0, 1, 2], [], [], []]

    def test_save_is_byte_deterministic(self, tmp_path):
        tone = pyxel.tones[0]
        original_tone = (
            tone.mode,
            tone.sample_bits,
            list(tone.wavetable),
            tone.gain,
        )
        original_sound_count = len(pyxel.sounds)
        try:
            tone.mode = 0
            tone.sample_bits = 2
            tone.wavetable[:] = [0, 3, 3, 0]
            tone.gain = 0.75
            first_sound = pyxel.Sound()
            second_sound = pyxel.Sound()
            first_sound.set("c2e2", "00", "75", "nf", 6)
            second_sound.set("g1r", "00", "53", "nq", 6)
            pyxel.sounds.extend([first_sound, second_sound])
            msc = pyxel.Music()
            msc.set([original_sound_count], [original_sound_count + 1])
            first_path = tmp_path / "first.wav"
            second_path = tmp_path / "second.wav"

            # Include a loop restart in both renders.
            msc.save(str(first_path), 0.2)
            msc.save(str(second_path), 0.2)
            actual = first_path.read_bytes()
            assert second_path.read_bytes() == actual
        finally:
            del pyxel.sounds[original_sound_count:]
            tone.mode = original_tone[0]
            tone.sample_bits = original_tone[1]
            tone.wavetable[:] = original_tone[2]
            tone.gain = original_tone[3]

    def test_set_overwrites_previous(self):
        msc = pyxel.Music()
        msc.set([0, 1, 2])
        msc.set([10])
        assert list(msc.seqs[0]) == [10]
        assert list(msc.seqs[1]) == []

    def test_snds_list_aliases_seqs_deprecated(self, capfd):
        msc = pyxel.Music()
        msc.set([0, 1])
        result = msc.snds_list  # type: ignore[attr-defined]
        assert len(result) == len(msc.seqs)
        assert list(result[0]) == [0, 1]
        result[0].append(2)
        assert list(msc.seqs[0]) == [0, 1, 2]
        out = capfd.readouterr().out
        assert out == "Music.snds_list[ch] is deprecated. Use Music.seqs[ch] instead.\n"


class TestMusicSeqs:
    @pytest.mark.parametrize(
        ("method", "args"),
        [
            ("__len__", ()),
            ("__bool__", ()),
            ("__getitem__", (0,)),
            ("__getitem__", (slice(None),)),
            ("__iter__", ()),
            ("__reversed__", ()),
            ("__repr__", ()),
            ("__contains__", (1,)),
            ("__eq__", ([1],)),
            ("__eq__", (None,)),
            ("__add__", ([1],)),
            ("__mul__", (2,)),
            ("__setitem__", (0, 1)),
            ("__setitem__", (slice(None), [1])),
            ("__delitem__", (0,)),
            ("__delitem__", (slice(None),)),
            ("__iadd__", ([1],)),
            ("append", (1,)),
            ("extend", ([1],)),
            ("insert", (0, 1)),
            ("pop", ()),
            ("clear", ()),
        ],
    )
    def test_removed_channel_view_raises(self, method, args):
        msc = pyxel.Music()
        msc.set([1, 2])
        seq = msc.seqs[0]
        msc.seqs.clear()

        with raises_exact(IndexError, "list index out of range"):
            getattr(seq, method)(*args)

    def test_comparison_with_removed_channel_view_raises(self):
        msc = pyxel.Music()
        msc.set([1], [2])
        first, second = msc.seqs[:2]
        del msc.seqs[1:]
        with raises_exact(IndexError, "list index out of range"):
            first.__eq__(second)

    @pytest.mark.parametrize("slice_key", [False, True])
    def test_index_conversion_can_remove_channel(self, slice_key):
        msc = pyxel.Music()
        msc.set([1, 2])
        seq = msc.seqs[0]

        class Index:
            def __index__(self):
                msc.seqs.clear()
                return 0

        key = slice(Index(), None) if slice_key else Index()
        with raises_exact(IndexError, "list index out of range"):
            seq[key]

    def test_value_conversion_can_remove_channel(self):
        msc = pyxel.Music()
        msc.set([1, 2])
        seq = msc.seqs[0]

        class Value:
            def __index__(self):
                msc.seqs.clear()
                return 3

        with raises_exact(IndexError, "list index out of range"):
            seq[0] = Value()

    def test_seqs_property(self):
        msc = pyxel.Music()
        msc.set([0, 1], [2, 3], [4])
        assert len(msc.seqs) == pyxel.NUM_CHANNELS
        assert list(msc.seqs[2]) == [4]

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
        seq = msc.seqs[0]
        msc.seqs[0] = [10, 11, 12]
        assert list(seq) == [10, 11, 12]
        assert list(msc.seqs[0]) == list(seq)
        assert list(msc.seqs[1]) == [2, 3]

    def test_seqs_self_assignment(self):
        code = """
import pyxel

music = pyxel.Music()
music.set([1, 2], [3, 4])
music.seqs[:] = music.seqs
music.seqs[0] = music.seqs[0]
assert list(music.seqs[0]) == [1, 2]
assert list(music.seqs[1]) == [3, 4]
"""
        result = subprocess.run(
            [sys.executable, "-B", "-c", code],
            capture_output=True,
            text=True,
            timeout=5,
            check=False,
        )
        assert result.returncode == 0, result.stdout + result.stderr

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

    def test_seqs_reversed_step_one_slice_assignment_inserts(self):
        msc = pyxel.Music()
        msc.set([0], [1], [2])

        msc.seqs[2:0] = [[7]]

        assert [list(seq) for seq in msc.seqs] == [[0], [1], [7], [2], []]

    def test_seqs_pop(self):
        msc = pyxel.Music()
        msc.set([0], [1], [2])
        original_len = len(msc.seqs)
        popped = msc.seqs.pop()
        assert len(msc.seqs) == original_len - 1
        assert popped == []

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
        assert list(msc.seqs[-2]) == [1, 2]
        assert list(msc.seqs[-1]) == [3, 4]

    def test_seqs_slice_access(self):
        msc = pyxel.Music()
        msc.set([0], [1], [2])
        sliced = msc.seqs[0:2]
        assert sliced == [[0], [1]]

    def test_seqs_reversed(self):
        msc = pyxel.Music()
        msc.set([10], [20], [30])
        assert [list(seq) for seq in reversed(msc.seqs)] == [[], [30], [20], [10]]

    def test_seqs_repr(self):
        msc = pyxel.Music()
        msc.set([0, 1])
        assert repr(msc.seqs) == "Seqs[[0, 1], [], [], []]"

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
        # seqs property is read-only, so use a local variable for +=.
        seqs = msc.seqs
        seqs += [[5, 6], [7, 8]]
        assert len(msc.seqs) == original_len + 2
        assert list(msc.seqs[-2]) == [5, 6]
        assert list(msc.seqs[-1]) == [7, 8]

    def test_seqs_from_list_deprecated(self, capfd):
        msc = pyxel.Music()
        msc.seqs.from_list([[10, 20], [30, 40]])  # type: ignore[attr-defined]
        assert [list(seq) for seq in msc.seqs] == [[10, 20], [30, 40], [], []]
        out = capfd.readouterr().out
        assert out == "Seqs.from_list() is deprecated. Use Music.set(*seqs) instead.\n"

    def test_seqs_to_list_deprecated(self, capfd):
        msc = pyxel.Music()
        msc.set([5, 6])
        result = msc.seqs.to_list()  # type: ignore[attr-defined]
        assert isinstance(result, list)
        assert result == [[5, 6], [], [], []]
        out = capfd.readouterr().out
        assert (
            out
            == "Seqs.to_list() is deprecated. Use [list(seq) for seq in seqs] instead.\n"
        )
