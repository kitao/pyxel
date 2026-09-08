import subprocess
import sys

import pytest
import pyxel
from _assertions import raises_exact  # type: ignore[reportMissingImports]

DEPRECATED_SEQUENCE_CASES = [
    pytest.param("music-seq", id="music-seq"),
    pytest.param("sound-notes", id="sound-notes"),
    pytest.param("sound-tones", id="sound-tones"),
    pytest.param("sound-volumes", id="sound-volumes"),
    pytest.param("sound-effects", id="sound-effects"),
    pytest.param("tone-wavetable", id="tone-wavetable"),
    pytest.param("colors", id="colors"),
    pytest.param("images", id="images"),
    pytest.param("tilemaps", id="tilemaps"),
    pytest.param("channels", id="channels"),
    pytest.param("tones", id="tones"),
    pytest.param("sounds", id="sounds"),
    pytest.param("musics", id="musics"),
]


class TestSeqLen:
    def test_colors_len(self):
        assert len(pyxel.colors) == pyxel.NUM_COLORS

    def test_images_len(self):
        assert len(pyxel.images) == pyxel.NUM_IMAGES

    def test_sounds_len(self):
        assert len(pyxel.sounds) == pyxel.NUM_SOUNDS

    def test_tilemaps_len(self):
        assert len(pyxel.tilemaps) == pyxel.NUM_TILEMAPS

    def test_channels_len(self):
        assert len(pyxel.channels) == pyxel.NUM_CHANNELS

    def test_tones_len(self):
        assert len(pyxel.tones) == pyxel.NUM_TONES

    def test_musics_len(self):
        assert len(pyxel.musics) == pyxel.NUM_MUSICS


class TestSeqGetitem:
    def test_images_index_access(self):
        img = pyxel.images[0]
        assert isinstance(img, pyxel.Image)

    def test_images_negative_index(self):
        img = pyxel.images[-1]
        assert isinstance(img, pyxel.Image)

    def test_images_slice_access(self):
        imgs = pyxel.images[0:2]
        assert isinstance(imgs, list)
        assert len(imgs) == 2
        assert all(isinstance(img, pyxel.Image) for img in imgs)

    def test_channels_index_access(self):
        ch = pyxel.channels[0]
        assert isinstance(ch, pyxel.Channel)

    def test_tones_index_access(self):
        tone = pyxel.tones[0]
        assert isinstance(tone, pyxel.Tone)

    def test_musics_index_access(self):
        msc = pyxel.musics[0]
        assert isinstance(msc, pyxel.Music)

    def test_sounds_full_range(self):
        for i in range(pyxel.NUM_SOUNDS):
            snd = pyxel.sounds[i]
            assert isinstance(snd, pyxel.Sound)


class TestSeqSetitem:
    @pytest.mark.parametrize("kind", ["notes", "seqs"])
    def test_out_of_range_raises_standard_message(self, kind):
        sequence = pyxel.Sound().notes if kind == "notes" else pyxel.Music().seqs
        value = 1 if kind == "notes" else [1]
        with raises_exact(IndexError, "list assignment index out of range"):
            sequence[0] = value

    def test_value_conversion_can_resize_sequence(self):
        notes = pyxel.Sound().notes
        notes[:] = [1]

        class Value:
            def __index__(self):
                notes.clear()
                return 2

        with raises_exact(IndexError, "list assignment index out of range"):
            notes[0] = Value()
        assert list(notes) == []

    def test_images_set_by_index(self):
        original = pyxel.images[0]
        new_img = pyxel.Image(64, 64)
        try:
            pyxel.images[0] = new_img
            assert pyxel.images[0].width == 64
        finally:
            pyxel.images[0] = original


@pytest.mark.parametrize("kind", ["notes", "seqs"])
@pytest.mark.parametrize("operation", ["get", "set", "delete"])
def test_slice_index_conversion_can_resize_sequence(kind, operation):
    result = subprocess.run(
        [
            sys.executable,
            "-c",
            f"""
import pyxel

seq = pyxel.Sound().notes if {kind!r} == "notes" else pyxel.Music().seqs
value = 1 if {kind!r} == "notes" else [1]
seq[:] = [value, value]


class Start:
    def __index__(self):
        seq.clear()
        return 0


if {operation!r} == "get":
    assert seq[Start():] == []
elif {operation!r} == "set":
    seq[Start():] = [value]
    assert list(seq) == [value]
else:
    del seq[Start():]
    assert list(seq) == []
""",
        ],
        capture_output=True,
        text=True,
        timeout=5,
        check=False,
    )
    assert result.returncode == 0, result.stderr


def test_add_conversion_can_resize_sequence():
    notes = pyxel.Sound().notes
    notes[:] = [1]

    class Value:
        def __index__(self):
            notes.clear()
            return 2

    assert notes + [Value()] == [2]


def test_slice_value_conversion_can_resize_sequence():
    notes = pyxel.Sound().notes
    notes[:] = [1, 2]

    class Value:
        def __index__(self):
            notes.clear()
            return 3

    notes[:] = [Value()]
    assert list(notes) == [3]


class TestSeqDelitem:
    @pytest.mark.parametrize("kind", ["notes", "seqs"])
    def test_out_of_range_raises_standard_message(self, kind):
        sequence = pyxel.Sound().notes if kind == "notes" else pyxel.Music().seqs
        with raises_exact(IndexError, "list assignment index out of range"):
            del sequence[0]

    def test_sounds_delete_appended_item(self):
        original = list(pyxel.sounds)
        try:
            pyxel.sounds.append(pyxel.Sound())
            assert len(pyxel.sounds) == len(original) + 1

            del pyxel.sounds[-1]
            assert len(pyxel.sounds) == len(original)
        finally:
            pyxel.sounds[:] = original


class TestSeqAppendPop:
    @pytest.mark.parametrize("kind", ["notes", "seqs"])
    def test_out_of_range_raises_standard_message(self, kind):
        sequence = pyxel.Sound().notes if kind == "notes" else pyxel.Music().seqs
        value = 1 if kind == "notes" else [1]
        sequence.append(value)
        with raises_exact(IndexError, "pop index out of range"):
            sequence.pop(1)

    @pytest.mark.parametrize(
        ("bank_name", "item_type", "args"),
        [
            ("sounds", pyxel.Sound, ()),
            ("musics", pyxel.Music, ()),
            ("images", pyxel.Image, (32, 32)),
            ("tilemaps", pyxel.Tilemap, (8, 8, 0)),
        ],
    )
    def test_append_and_pop(self, bank_name, item_type, args):
        sequence = getattr(pyxel, bank_name)
        original = list(sequence)
        item = item_type(*args)
        try:
            sequence.append(item)
            assert len(sequence) == len(original) + 1

            popped = sequence.pop()
            assert len(sequence) == len(original)
            assert isinstance(popped, item_type)
        finally:
            sequence[:] = original


class TestSeqIteration:
    def test_iter_images(self):
        count = 0
        for img in pyxel.images:
            assert isinstance(img, pyxel.Image)
            count += 1
        assert count == len(pyxel.images)

    def test_contains_with_value_type(self):
        # Object types (Image etc.) create new wrappers each access,
        # so test __contains__ with value types (colors) instead.
        col = pyxel.colors[0]
        assert col in pyxel.colors

    def test_not_contains(self):
        assert 0x999999 not in pyxel.colors

    def test_iter_channels(self):
        count = 0
        for ch in pyxel.channels:
            assert isinstance(ch, pyxel.Channel)
            count += 1
        assert count == len(pyxel.channels)

    def test_iter_tones(self):
        count = 0
        for tone in pyxel.tones:
            assert isinstance(tone, pyxel.Tone)
            count += 1
        assert count == len(pyxel.tones)

    def test_iter_musics(self):
        count = 0
        for msc in pyxel.musics:
            assert isinstance(msc, pyxel.Music)
            count += 1
        assert count == len(pyxel.musics)


class TestSeqSliceOperations:
    def test_setitem_slice(self):
        original = list(pyxel.colors)
        try:
            pyxel.colors[0:2] = [0x000000, 0xFFFFFF]
            assert pyxel.colors[0] == 0x000000
            assert pyxel.colors[1] == 0xFFFFFF
        finally:
            pyxel.colors[:] = original

    def test_reversed_step_one_slice_assignment_inserts(self):
        original = list(pyxel.colors)
        try:
            pyxel.colors[2:0] = [0x123456]
            assert list(pyxel.colors[:4]) == [
                original[0],
                original[1],
                0x123456,
                original[2],
            ]
        finally:
            pyxel.colors[:] = original

    def test_setitem_extended_slice_with_primitive_values(self):
        original = [pyxel.colors[0], pyxel.colors[2]]
        try:
            pyxel.colors[0:3:2] = [0x123456, 0xABCDEF]
            assert pyxel.colors[0] == 0x123456
            assert pyxel.colors[2] == 0xABCDEF
        finally:
            pyxel.colors[0:3:2] = original

    def test_setitem_extended_slice_with_object_values(self):
        original = [pyxel.images[0], pyxel.images[2]]
        untouched_size = (pyxel.images[1].width, pyxel.images[1].height)
        try:
            pyxel.images[0:3:2] = [pyxel.Image(3, 5), pyxel.Image(7, 9)]
            assert (pyxel.images[0].width, pyxel.images[0].height) == (3, 5)
            assert (pyxel.images[1].width, pyxel.images[1].height) == untouched_size
            assert (pyxel.images[2].width, pyxel.images[2].height) == (7, 9)
        finally:
            pyxel.images[0:3:2] = original

    @pytest.mark.parametrize("kind", ["notes", "seqs"])
    @pytest.mark.parametrize(
        "key",
        [
            slice(-3, None),
            slice(None, None, 2),
            slice(None, None, -2),
            slice(2, 2),
            slice(1, None, sys.maxsize),
            slice(1, None, -sys.maxsize),
        ],
    )
    def test_delitem_slice(self, kind, key):
        seq = pyxel.Sound().notes if kind == "notes" else pyxel.Music().seqs
        values = list(range(6)) if kind == "notes" else [[i] for i in range(6)]
        seq[:] = values
        del seq[key]
        del values[key]
        assert list(seq) == values

    def test_getitem_slice_returns_list(self):
        sliced = pyxel.sounds[0:3]
        assert isinstance(sliced, list)
        assert len(sliced) == 3


class TestSeqExtendClear:
    def test_sounds_extend(self):
        original = list(pyxel.sounds)
        new_sounds = [pyxel.Sound(), pyxel.Sound()]
        try:
            pyxel.sounds.extend(new_sounds)
            assert len(pyxel.sounds) == len(original) + 2
        finally:
            pyxel.sounds[:] = original

    def test_colors_clear_and_restore(self):
        original_colors = list(pyxel.colors)
        try:
            pyxel.colors.clear()
            assert len(pyxel.colors) == 0
        finally:
            pyxel.colors[:] = original_colors


class TestColorsExtendBeyondDefault:
    def test_append_beyond_16(self):
        original = list(pyxel.colors)
        try:
            pyxel.colors.append(0x123456)
            assert len(pyxel.colors) == len(original) + 1
            assert pyxel.colors[-1] == 0x123456
        finally:
            pyxel.colors[:] = original

    def test_multiple_appends(self):
        original = list(pyxel.colors)
        try:
            for i in range(10):
                pyxel.colors.append(0x100000 + i)
            assert len(pyxel.colors) == len(original) + 10
        finally:
            pyxel.colors[:] = original


class TestSeqInsert:
    def test_insert_colors(self):
        original = list(pyxel.colors)
        try:
            pyxel.colors.insert(0, 0xABCDEF)
            assert pyxel.colors[0] == 0xABCDEF
            assert len(pyxel.colors) == len(original) + 1
            assert pyxel.colors[1] == original[0]
        finally:
            pyxel.colors[:] = original

    def test_insert_sounds(self):
        original = list(pyxel.sounds)
        snd = pyxel.Sound()
        try:
            pyxel.sounds.insert(0, snd)
            assert len(pyxel.sounds) == len(original) + 1
        finally:
            pyxel.sounds[:] = original


class TestSeqReversed:
    def test_reversed_colors(self):
        colors_list = list(pyxel.colors)
        rev = list(reversed(pyxel.colors))
        assert rev == list(reversed(colors_list))

    def test_reversed_images(self):
        original = list(pyxel.images)
        try:
            pyxel.images[:] = [pyxel.Image(1, 2), pyxel.Image(3, 4)]
            assert [(img.width, img.height) for img in reversed(pyxel.images)] == [
                (3, 4),
                (1, 2),
            ]
        finally:
            pyxel.images[:] = original


class TestSeqRepr:
    def test_colors_repr(self):
        assert repr(pyxel.colors) == f"Colors{list(pyxel.colors)!r}"


class TestSeqBool:
    def test_nonempty_is_truthy(self):
        assert bool(pyxel.colors)
        assert bool(pyxel.images)
        assert bool(pyxel.sounds)

    def test_empty_is_falsy(self):
        original = list(pyxel.colors)
        try:
            pyxel.colors.clear()
            assert not bool(pyxel.colors)
        finally:
            pyxel.colors[:] = original


class TestSeqIadd:
    def test_iadd_colors(self):
        original = list(pyxel.colors)
        try:
            pyxel.colors += [0xAAAAAA, 0xBBBBBB]
            assert len(pyxel.colors) == len(original) + 2
            assert pyxel.colors[-2] == 0xAAAAAA
            assert pyxel.colors[-1] == 0xBBBBBB
        finally:
            pyxel.colors[:] = original

    def test_iadd_sounds(self):
        original = list(pyxel.sounds)
        try:
            pyxel.sounds += [pyxel.Sound(), pyxel.Sound()]
            assert len(pyxel.sounds) == len(original) + 2
        finally:
            pyxel.sounds[:] = original

    def test_iadd_empty_list(self):
        original_len = len(pyxel.colors)
        pyxel.colors += []
        assert len(pyxel.colors) == original_len


class TestSeqValueOps:
    def test_eq_same_content(self):
        colors_list = list(pyxel.colors)
        assert pyxel.colors == colors_list

    def test_neq_different_content(self):
        colors_list = list(pyxel.colors)
        colors_list[0] = 0x999999
        assert pyxel.colors != colors_list

    def test_add(self):
        colors = list(pyxel.colors)
        result = pyxel.colors + colors
        assert result == colors + colors
        assert isinstance(result, list)

    def test_mul(self):
        colors = list(pyxel.colors)
        result = pyxel.colors * 2
        assert result == colors * 2
        assert isinstance(result, list)


class TestSeqReentry:
    @pytest.mark.parametrize(
        ("operation", "expected"),
        [
            ("iter(seq)", [1, 2, 3]),
            ("repr(seq)", "Wavetable[1, 2, 3]"),
            ("operator.mul(seq, 2)", [1, 2, 3, 1, 2, 3]),
        ],
    )
    def test_sequence_results_allow_garbage_collection_reentry(
        self, operation, expected
    ):
        code = """
import gc
import operator

import pyxel

seq = pyxel.Tone().wavetable
seq[:] = [1, 2, 3]
active = False
collected = False


def collect(phase, info):
    global collected
    if active and phase == "start":
        seq.clear()
        collected = True


gc.callbacks.append(collect)
gc.collect()
gc.disable()
# Exhaust the list free list so constructing a result can trigger collection.
retained = [[] for _ in range(1000)]
gc.set_threshold(1, 1000000, 1000000)
active = True
gc.enable()

result = OPERATION
active = False
gc.callbacks.remove(collect)

assert collected
if not isinstance(result, str):
    result = list(result)
assert result == EXPECTED
assert len(seq) == 0
    """
        code = code.replace("OPERATION", operation).replace("EXPECTED", repr(expected))
        result = subprocess.run(
            [sys.executable, "-B", "-c", code],
            capture_output=True,
            text=True,
            timeout=5,
            check=False,
        )
        assert result.returncode == 0, result.stdout + result.stderr


class TestDeprecatedSequenceMethods:
    @pytest.mark.parametrize("case", DEPRECATED_SEQUENCE_CASES)
    def test_from_list_replaces_values_and_warns(self, capfd, case):
        sequence, replacements = make_deprecated_sequence_case(case)
        wrapper_name = type(sequence).__name__
        original = list(sequence)
        try:
            result = sequence.from_list(replacements)  # type: ignore[attr-defined]
            assert result is None
            assert sequence_snapshot(case, sequence) == sequence_snapshot(
                case, replacements
            )
            assert (
                capfd.readouterr().out == f"{wrapper_name}.from_list() is deprecated. "
                "Use slice assignment instead.\n"
            )
        finally:
            sequence[:] = original

    @pytest.mark.parametrize("case", DEPRECATED_SEQUENCE_CASES)
    def test_to_list_returns_values_and_warns(self, capfd, case):
        sequence, _ = make_deprecated_sequence_case(case)
        wrapper_name = type(sequence).__name__
        result = sequence.to_list()  # type: ignore[attr-defined]
        assert isinstance(result, list)
        assert sequence_snapshot(case, result) == sequence_snapshot(case, sequence)
        assert (
            capfd.readouterr().out
            == f"{wrapper_name}.to_list() is deprecated. Use list(seq) instead.\n"
        )


def make_deprecated_sequence_case(case):
    if case == "music-seq":
        music = pyxel.Music()
        music.set([1, 2])
        return music.seqs[0], [3, 4]

    if case.startswith("sound-"):
        sound = pyxel.Sound()
        sound.set("c2e2", "sp", "76", "nf", 10)
        attribute = case.removeprefix("sound-")
        replacements = {
            "notes": [12, 24],
            "tones": [0, 2],
            "volumes": [3, 7],
            "effects": [0, 2],
        }
        return getattr(sound, attribute), replacements[attribute]

    if case == "tone-wavetable":
        tone = pyxel.Tone()
        tone.wavetable[:] = [1, 2]
        return tone.wavetable, [3, 12]

    if case == "colors":
        return pyxel.colors, [0x123456, 0xABCDEF]
    if case == "images":
        return pyxel.images, [pyxel.Image(3, 5)]
    if case == "tilemaps":
        return pyxel.tilemaps, [pyxel.Tilemap(4, 6, 0)]

    if case == "channels":
        channel = pyxel.Channel()
        channel.gain = 0.25
        channel.detune = 7
        return pyxel.channels, [channel]

    if case == "tones":
        tone = pyxel.Tone()
        tone.mode = 2
        tone.gain = 0.5
        return pyxel.tones, [tone]

    if case == "sounds":
        sound = pyxel.Sound()
        sound.set("c2e2", "sp", "76", "nf", 9)
        return pyxel.sounds, [sound]

    if case == "musics":
        music = pyxel.Music()
        music.set([3, 4])
        return pyxel.musics, [music]

    raise AssertionError(f"unknown deprecated sequence case: {case}")


def sequence_snapshot(case, sequence):
    if case == "images":
        return [(item.width, item.height) for item in sequence]
    if case == "tilemaps":
        return [(item.width, item.height, item.imgsrc) for item in sequence]
    if case == "channels":
        return [(item.gain, item.detune) for item in sequence]

    if case == "tones":
        return [
            (item.mode, item.sample_bits, list(item.wavetable), item.gain)
            for item in sequence
        ]

    if case == "sounds":
        return [
            (
                list(item.notes),
                list(item.tones),
                list(item.volumes),
                list(item.effects),
                item.speed,
            )
            for item in sequence
        ]

    if case == "musics":
        return [[list(seq) for seq in item.seqs] for item in sequence]

    return list(sequence)
