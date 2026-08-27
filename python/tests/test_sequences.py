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

    def test_images_out_of_range_raises(self):
        with raises_exact(IndexError, "list index out of range"):
            _ = pyxel.images[999]

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
    def test_images_set_by_index(self):
        original = pyxel.images[0]
        new_img = pyxel.Image(64, 64)
        pyxel.images[0] = new_img
        assert pyxel.images[0].width == 64
        pyxel.images[0] = original


class TestSeqDelitem:
    def test_sounds_delete_appended_item(self):
        original_len = len(pyxel.sounds)
        pyxel.sounds.append(pyxel.Sound())
        assert len(pyxel.sounds) == original_len + 1
        del pyxel.sounds[-1]
        assert len(pyxel.sounds) == original_len


class TestSeqAppendPop:
    def test_sounds_append_and_pop(self):
        original_len = len(pyxel.sounds)
        snd = pyxel.Sound()
        pyxel.sounds.append(snd)
        assert len(pyxel.sounds) == original_len + 1
        popped = pyxel.sounds.pop()
        assert len(pyxel.sounds) == original_len
        assert isinstance(popped, pyxel.Sound)

    def test_channels_append_and_pop(self):
        original_len = len(pyxel.channels)
        ch = pyxel.Channel()
        pyxel.channels.append(ch)
        assert len(pyxel.channels) == original_len + 1
        pyxel.channels.pop()
        assert len(pyxel.channels) == original_len

    def test_tones_append_and_pop(self):
        original_len = len(pyxel.tones)
        tone = pyxel.Tone()
        pyxel.tones.append(tone)
        assert len(pyxel.tones) == original_len + 1
        pyxel.tones.pop()
        assert len(pyxel.tones) == original_len

    def test_musics_append_and_pop(self):
        original_len = len(pyxel.musics)
        msc = pyxel.Music()
        pyxel.musics.append(msc)
        assert len(pyxel.musics) == original_len + 1
        pyxel.musics.pop()
        assert len(pyxel.musics) == original_len

    def test_images_append_and_pop(self):
        original_len = len(pyxel.images)
        img = pyxel.Image(32, 32)
        pyxel.images.append(img)
        assert len(pyxel.images) == original_len + 1
        pyxel.images.pop()
        assert len(pyxel.images) == original_len

    def test_tilemaps_append_and_pop(self):
        original_len = len(pyxel.tilemaps)
        tm = pyxel.Tilemap(8, 8, 0)
        pyxel.tilemaps.append(tm)
        assert len(pyxel.tilemaps) == original_len + 1
        pyxel.tilemaps.pop()
        assert len(pyxel.tilemaps) == original_len


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
        pyxel.colors[0:2] = [0x000000, 0xFFFFFF]
        assert pyxel.colors[0] == 0x000000
        assert pyxel.colors[1] == 0xFFFFFF
        pyxel.colors[0:2] = original[0:2]

    def test_reversed_step_one_slice_assignment_inserts(self):
        original = list(pyxel.colors)
        pyxel.colors[2:0] = [0x123456]
        assert list(pyxel.colors[:4]) == [
            original[0],
            original[1],
            0x123456,
            original[2],
        ]
        pyxel.colors[:] = original

    def test_delitem_slice(self):
        for _ in range(3):
            pyxel.sounds.append(pyxel.Sound())
        before_len = len(pyxel.sounds)
        del pyxel.sounds[-3:]
        assert len(pyxel.sounds) == before_len - 3

    def test_getitem_slice_returns_list(self):
        sliced = pyxel.sounds[0:3]
        assert isinstance(sliced, list)
        assert len(sliced) == 3


class TestSeqExtendClear:
    def test_sounds_extend(self):
        original_len = len(pyxel.sounds)
        new_sounds = [pyxel.Sound(), pyxel.Sound()]
        pyxel.sounds.extend(new_sounds)
        assert len(pyxel.sounds) == original_len + 2
        pyxel.sounds.pop()
        pyxel.sounds.pop()

    def test_colors_clear_and_restore(self):
        original_colors = list(pyxel.colors)
        pyxel.colors.clear()
        assert len(pyxel.colors) == 0
        for c in original_colors:
            pyxel.colors.append(c)
        assert len(pyxel.colors) == len(original_colors)


class TestColorsExtendBeyondDefault:
    def test_append_beyond_16(self):
        original = list(pyxel.colors)
        pyxel.colors.append(0x123456)
        assert len(pyxel.colors) == len(original) + 1
        assert pyxel.colors[-1] == 0x123456
        pyxel.colors.pop()
        assert len(pyxel.colors) == len(original)

    def test_multiple_appends(self):
        original = list(pyxel.colors)
        for i in range(10):
            pyxel.colors.append(0x100000 + i)
        assert len(pyxel.colors) == len(original) + 10
        for _ in range(10):
            pyxel.colors.pop()
        assert len(pyxel.colors) == len(original)


class TestSeqInsert:
    def test_insert_colors(self):
        original = list(pyxel.colors)
        pyxel.colors.insert(0, 0xABCDEF)
        assert pyxel.colors[0] == 0xABCDEF
        assert len(pyxel.colors) == len(original) + 1
        assert pyxel.colors[1] == original[0]
        del pyxel.colors[0]
        assert len(pyxel.colors) == len(original)

    def test_insert_sounds(self):
        original_len = len(pyxel.sounds)
        snd = pyxel.Sound()
        pyxel.sounds.insert(0, snd)
        assert len(pyxel.sounds) == original_len + 1
        del pyxel.sounds[0]
        assert len(pyxel.sounds) == original_len


class TestSeqReversed:
    def test_reversed_colors(self):
        colors_list = list(pyxel.colors)
        rev = list(reversed(pyxel.colors))
        assert rev == list(reversed(colors_list))

    def test_reversed_images(self):
        rev = list(reversed(pyxel.images))
        assert len(rev) == len(pyxel.images)


class TestSeqRepr:
    def test_colors_repr(self):
        # Value-type sequences have a deterministic wrapper-name + list repr.
        assert repr(pyxel.colors) == f"Colors{list(pyxel.colors)!r}"

    def test_images_repr(self):
        r = repr(pyxel.images)
        assert isinstance(r, str)

    def test_sounds_repr(self):
        r = repr(pyxel.sounds)
        assert isinstance(r, str)


class TestSeqBool:
    def test_nonempty_is_truthy(self):
        assert bool(pyxel.colors)
        assert bool(pyxel.images)
        assert bool(pyxel.sounds)

    def test_empty_is_falsy(self):
        original = list(pyxel.colors)
        pyxel.colors.clear()
        assert not bool(pyxel.colors)
        for c in original:
            pyxel.colors.append(c)


class TestSeqIadd:
    def test_iadd_colors(self):
        original = list(pyxel.colors)
        pyxel.colors += [0xAAAAAA, 0xBBBBBB]
        assert len(pyxel.colors) == len(original) + 2
        assert pyxel.colors[-2] == 0xAAAAAA
        assert pyxel.colors[-1] == 0xBBBBBB
        pyxel.colors.pop()
        pyxel.colors.pop()
        assert len(pyxel.colors) == len(original)

    def test_iadd_sounds(self):
        original_len = len(pyxel.sounds)
        pyxel.sounds += [pyxel.Sound(), pyxel.Sound()]
        assert len(pyxel.sounds) == original_len + 2
        pyxel.sounds.pop()
        pyxel.sounds.pop()
        assert len(pyxel.sounds) == original_len

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
        result = pyxel.colors + list(pyxel.colors)
        assert len(result) == len(pyxel.colors) * 2
        assert isinstance(result, list)

    def test_mul(self):
        original_len = len(pyxel.colors)
        result = pyxel.colors * 2
        assert len(result) == original_len * 2
        assert isinstance(result, list)


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
