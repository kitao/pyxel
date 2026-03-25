import pyxel


class TestSeqLen:
    def test_colors_len(self):
        assert len(pyxel.colors) >= 16

    def test_images_len(self):
        assert len(pyxel.images) == pyxel.NUM_IMAGES

    def test_sounds_len(self):
        assert len(pyxel.sounds) == pyxel.NUM_SOUNDS

    def test_tilemaps_len(self):
        assert len(pyxel.tilemaps) == pyxel.NUM_TILEMAPS


class TestSeqGetitem:
    def test_index_access(self):
        img = pyxel.images[0]
        assert isinstance(img, pyxel.Image)

    def test_negative_index(self):
        img = pyxel.images[-1]
        assert isinstance(img, pyxel.Image)

    def test_slice_access(self):
        imgs = pyxel.images[0:2]
        assert isinstance(imgs, list)
        assert len(imgs) == 2

    def test_out_of_range_raises(self):
        import pytest

        with pytest.raises(IndexError):
            _ = pyxel.images[999]


class TestSeqSetitem:
    def test_set_by_index(self):
        new_img = pyxel.Image(256, 256)
        pyxel.images[0] = new_img


class TestSeqDelitem:
    def test_delete_appended_item(self):
        original_len = len(pyxel.sounds)
        pyxel.sounds.append(pyxel.Sound())
        assert len(pyxel.sounds) == original_len + 1
        del pyxel.sounds[-1]
        assert len(pyxel.sounds) == original_len


class TestSeqAppendPop:
    def test_append_and_pop(self):
        original_len = len(pyxel.sounds)
        snd = pyxel.Sound()
        pyxel.sounds.append(snd)
        assert len(pyxel.sounds) == original_len + 1
        _popped = pyxel.sounds.pop()
        assert len(pyxel.sounds) == original_len


class TestSeqIteration:
    def test_iter(self):
        count = 0
        for img in pyxel.images:
            assert isinstance(img, pyxel.Image)
            count += 1
        assert count == len(pyxel.images)

    def test_contains_with_value_type(self):
        # Object types (Image etc.) create new wrappers each access,
        # so test __contains__ with value types (colors) instead
        col = pyxel.colors[0]
        assert col in pyxel.colors


class TestSeqSliceOperations:
    def test_setitem_slice(self):
        original = list(pyxel.colors)
        pyxel.colors[0:2] = [0x000000, 0xFFFFFF]
        assert pyxel.colors[0] == 0x000000
        assert pyxel.colors[1] == 0xFFFFFF
        # Restore
        pyxel.colors[0:2] = original[0:2]

    def test_delitem_slice(self):
        for _ in range(3):
            pyxel.sounds.append(pyxel.Sound())
        before_len = len(pyxel.sounds)
        del pyxel.sounds[-3:]
        assert len(pyxel.sounds) == before_len - 3


class TestSeqExtendClear:
    def test_extend(self):
        original_len = len(pyxel.sounds)
        new_sounds = [pyxel.Sound(), pyxel.Sound()]
        pyxel.sounds.extend(new_sounds)
        assert len(pyxel.sounds) == original_len + 2
        # Clean up
        pyxel.sounds.pop()
        pyxel.sounds.pop()

    def test_clear_and_restore(self):
        # Save original state
        original_colors = list(pyxel.colors)
        pyxel.colors.clear()
        assert len(pyxel.colors) == 0
        # Restore
        for c in original_colors:
            pyxel.colors.append(c)
        assert len(pyxel.colors) == len(original_colors)
