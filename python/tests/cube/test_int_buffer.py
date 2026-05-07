import pytest

from pyxel.cube import IntBuffer


class TestConstruction:
    def test_default_empty(self):
        buf = IntBuffer()
        assert buf.size == 0
        assert len(buf) == 0

    def test_with_size(self):
        buf = IntBuffer(4)
        assert buf.size == 4
        assert len(buf) == 4
        assert list(buf) == [0, 0, 0, 0]

    def test_from_list(self):
        buf = IntBuffer([1, 2, 3])
        assert buf.size == 3
        assert list(buf) == [1, 2, 3]

    def test_negative_values(self):
        buf = IntBuffer([-1, -2, -3])
        assert list(buf) == [-1, -2, -3]

    def test_invalid_source_raises(self):
        with pytest.raises(TypeError):
            IntBuffer("invalid")


class TestPerElementAccess:
    def test_get_set(self):
        buf = IntBuffer(3)
        buf[0] = 42
        buf[1] = -7
        assert buf[0] == 42
        assert buf[1] == -7
        assert buf[2] == 0

    def test_negative_index(self):
        buf = IntBuffer([1, 2, 3])
        assert buf[-1] == 3
        assert buf[-3] == 1

    def test_out_of_range_get(self):
        buf = IntBuffer(3)
        with pytest.raises(IndexError):
            _ = buf[3]
        with pytest.raises(IndexError):
            _ = buf[-4]

    def test_out_of_range_set(self):
        buf = IntBuffer(3)
        with pytest.raises(IndexError):
            buf[3] = 0


class TestSlice:
    def test_slice_get(self):
        buf = IntBuffer([0, 1, 2, 3, 4])
        assert buf[1:4] == [1, 2, 3]
        assert buf[:] == [0, 1, 2, 3, 4]
        assert buf[::2] == [0, 2, 4]

    def test_slice_get_empty_range(self):
        buf = IntBuffer([1, 2, 3])
        assert buf[1:1] == []
        assert buf[100:200] == []

    def test_slice_set_list(self):
        buf = IntBuffer(5)
        buf[1:4] = [10, 20, 30]
        assert buf[:] == [0, 10, 20, 30, 0]

    def test_slice_set_buffer(self):
        dst = IntBuffer(5)
        src = IntBuffer([1, 2, 3])
        dst[0:3] = src
        assert dst[:] == [1, 2, 3, 0, 0]

    def test_slice_set_full_range(self):
        dst = IntBuffer(3)
        dst[:] = [1, 2, 3]
        assert dst[:] == [1, 2, 3]

    def test_slice_set_size_mismatch_list_raises(self):
        buf = IntBuffer(5)
        with pytest.raises(ValueError):
            buf[1:4] = [10, 20]

    def test_slice_set_size_mismatch_buffer_raises(self):
        buf = IntBuffer(5)
        with pytest.raises(ValueError):
            buf[1:4] = IntBuffer(2)

    def test_slice_with_step(self):
        buf = IntBuffer([0, 1, 2, 3, 4])
        buf[::2] = [10, 20, 30]
        assert buf[:] == [10, 1, 20, 3, 30]


class TestInPlaceOps:
    def test_fill(self):
        buf = IntBuffer(3)
        buf.fill(99)
        assert list(buf) == [99, 99, 99]

    def test_resize_grow_zero_fills_tail(self):
        buf = IntBuffer([1, 2])
        buf.resize(4)
        assert buf.size == 4
        assert list(buf) == [1, 2, 0, 0]

    def test_resize_shrink_truncates(self):
        buf = IntBuffer([1, 2, 3, 4])
        buf.resize(2)
        assert buf.size == 2
        assert list(buf) == [1, 2]

    def test_resize_to_zero(self):
        buf = IntBuffer([1, 2, 3])
        buf.resize(0)
        assert buf.size == 0
        assert list(buf) == []


class TestEquality:
    def test_eq_same_data(self):
        a = IntBuffer([1, 2, 3])
        b = IntBuffer([1, 2, 3])
        assert a == b

    def test_eq_different_size(self):
        a = IntBuffer([1, 2])
        b = IntBuffer([1, 2, 3])
        assert a != b

    def test_eq_different_values(self):
        a = IntBuffer([1, 2])
        b = IntBuffer([3, 4])
        assert a != b

    def test_eq_other_type(self):
        a = IntBuffer([1, 2])
        assert a != [1, 2]
        assert a != "string"
        assert a != 42


class TestRepr:
    def test_repr_format(self):
        buf = IntBuffer(5)
        assert repr(buf).startswith("IntBuffer(")
        assert "5" in repr(buf)


class TestIter:
    def test_iter(self):
        buf = IntBuffer([1, 2, 3])
        result = []
        for v in buf:
            result.append(v)
        assert result == [1, 2, 3]


class TestBufferProtocol:
    def test_memoryview_metadata(self):
        buf = IntBuffer([1, 2, 3])
        mv = memoryview(buf)
        try:
            assert mv.format == "i"
            assert mv.itemsize == 4
            assert mv.shape == (3,)
            assert mv.strides == (4,)
            assert mv.ndim == 1
            assert mv.readonly is False
        finally:
            mv.release()

    def test_memoryview_reads_underlying_storage(self):
        buf = IntBuffer([1, 2, 3])
        mv = memoryview(buf)
        try:
            assert list(mv) == [1, 2, 3]
        finally:
            mv.release()

    def test_memoryview_write_propagates(self):
        buf = IntBuffer([1, 2, 3])
        mv = memoryview(buf)
        try:
            mv[0] = 99
            mv[2] = -7
        finally:
            mv.release()
        assert buf[0] == 99
        assert buf[1] == 2
        assert buf[2] == -7

    def test_bytes_roundtrip(self):
        buf = IntBuffer([1, 2])
        mv = memoryview(buf)
        try:
            assert len(mv.tobytes()) == 2 * 4
        finally:
            mv.release()
