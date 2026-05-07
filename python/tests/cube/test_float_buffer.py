import pytest

from pyxel.cube import FloatBuffer


class TestConstruction:
    def test_default_empty(self):
        buf = FloatBuffer()
        assert buf.size == 0
        assert len(buf) == 0

    def test_with_size(self):
        buf = FloatBuffer(4)
        assert buf.size == 4
        assert len(buf) == 4
        assert list(buf) == [0.0, 0.0, 0.0, 0.0]

    def test_from_list(self):
        buf = FloatBuffer([1.0, 2.0, 3.0])
        assert buf.size == 3
        assert list(buf) == [1.0, 2.0, 3.0]

    def test_invalid_source_raises(self):
        with pytest.raises(TypeError):
            FloatBuffer("invalid")


class TestPerElementAccess:
    def test_get_set(self):
        buf = FloatBuffer(3)
        buf[0] = 1.5
        buf[1] = -2.5
        assert buf[0] == 1.5
        assert buf[1] == -2.5
        assert buf[2] == 0.0

    def test_negative_index(self):
        buf = FloatBuffer([1.0, 2.0, 3.0])
        assert buf[-1] == 3.0
        assert buf[-3] == 1.0

    def test_out_of_range_get(self):
        buf = FloatBuffer(3)
        with pytest.raises(IndexError):
            _ = buf[3]
        with pytest.raises(IndexError):
            _ = buf[-4]

    def test_out_of_range_set(self):
        buf = FloatBuffer(3)
        with pytest.raises(IndexError):
            buf[3] = 0.0


class TestSlice:
    def test_slice_get(self):
        buf = FloatBuffer([0.0, 1.0, 2.0, 3.0, 4.0])
        assert buf[1:4] == [1.0, 2.0, 3.0]
        assert buf[:] == [0.0, 1.0, 2.0, 3.0, 4.0]
        assert buf[::2] == [0.0, 2.0, 4.0]

    def test_slice_get_empty_range(self):
        buf = FloatBuffer([1.0, 2.0, 3.0])
        assert buf[1:1] == []
        assert buf[100:200] == []

    def test_slice_set_list(self):
        buf = FloatBuffer(5)
        buf[1:4] = [10.0, 20.0, 30.0]
        assert buf[:] == [0.0, 10.0, 20.0, 30.0, 0.0]

    def test_slice_set_buffer(self):
        dst = FloatBuffer(5)
        src = FloatBuffer([1.0, 2.0, 3.0])
        dst[0:3] = src
        assert dst[:] == [1.0, 2.0, 3.0, 0.0, 0.0]

    def test_slice_set_full_range(self):
        dst = FloatBuffer(3)
        dst[:] = [1.0, 2.0, 3.0]
        assert dst[:] == [1.0, 2.0, 3.0]

    def test_slice_set_size_mismatch_list_raises(self):
        buf = FloatBuffer(5)
        with pytest.raises(ValueError):
            buf[1:4] = [10.0, 20.0]

    def test_slice_set_size_mismatch_buffer_raises(self):
        buf = FloatBuffer(5)
        with pytest.raises(ValueError):
            buf[1:4] = FloatBuffer(2)

    def test_slice_with_step(self):
        buf = FloatBuffer([0.0, 1.0, 2.0, 3.0, 4.0])
        buf[::2] = [10.0, 20.0, 30.0]
        assert buf[:] == [10.0, 1.0, 20.0, 3.0, 30.0]


class TestInPlaceOps:
    def test_fill(self):
        buf = FloatBuffer(3)
        buf.fill(7.5)
        assert list(buf) == [7.5, 7.5, 7.5]

    def test_resize_grow_zero_fills_tail(self):
        buf = FloatBuffer([1.0, 2.0])
        buf.resize(4)
        assert buf.size == 4
        assert list(buf) == [1.0, 2.0, 0.0, 0.0]

    def test_resize_shrink_truncates(self):
        buf = FloatBuffer([1.0, 2.0, 3.0, 4.0])
        buf.resize(2)
        assert buf.size == 2
        assert list(buf) == [1.0, 2.0]

    def test_resize_to_zero(self):
        buf = FloatBuffer([1.0, 2.0, 3.0])
        buf.resize(0)
        assert buf.size == 0
        assert list(buf) == []


class TestEquality:
    def test_eq_same_data(self):
        a = FloatBuffer([1.0, 2.0, 3.0])
        b = FloatBuffer([1.0, 2.0, 3.0])
        assert a == b

    def test_eq_different_size(self):
        a = FloatBuffer([1.0, 2.0])
        b = FloatBuffer([1.0, 2.0, 3.0])
        assert a != b

    def test_eq_different_values(self):
        a = FloatBuffer([1.0, 2.0])
        b = FloatBuffer([3.0, 4.0])
        assert a != b

    def test_eq_other_type(self):
        a = FloatBuffer([1.0, 2.0])
        assert a != [1.0, 2.0]
        assert a != "string"
        assert a != 42


class TestRepr:
    def test_repr_format(self):
        buf = FloatBuffer(5)
        assert repr(buf).startswith("FloatBuffer(")
        assert "5" in repr(buf)


class TestIter:
    def test_iter(self):
        buf = FloatBuffer([1.0, 2.0, 3.0])
        result = []
        for v in buf:
            result.append(v)
        assert result == [1.0, 2.0, 3.0]


class TestBufferProtocol:
    def test_memoryview_metadata(self):
        buf = FloatBuffer([1.0, 2.0, 3.0])
        mv = memoryview(buf)
        try:
            assert mv.format == "f"
            assert mv.itemsize == 4
            assert mv.shape == (3,)
            assert mv.strides == (4,)
            assert mv.ndim == 1
            assert mv.readonly is False
        finally:
            mv.release()

    def test_memoryview_reads_underlying_storage(self):
        buf = FloatBuffer([1.0, 2.0, 3.0])
        mv = memoryview(buf)
        try:
            assert list(mv) == [1.0, 2.0, 3.0]
        finally:
            mv.release()

    def test_memoryview_write_propagates(self):
        buf = FloatBuffer([1.0, 2.0, 3.0])
        mv = memoryview(buf)
        try:
            mv[0] = 99.0
            mv[2] = -7.5
        finally:
            mv.release()
        assert buf[0] == 99.0
        assert buf[1] == 2.0
        assert buf[2] == -7.5

    def test_bytes_roundtrip(self):
        buf = FloatBuffer([1.0, 2.0])
        mv = memoryview(buf)
        try:
            assert len(mv.tobytes()) == 2 * 4
        finally:
            mv.release()
