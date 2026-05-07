use pyo3::exceptions::{PyBufferError, PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyList, PySlice};

define_wrapper!(FloatBuffer, pyxel::cube::FloatBuffer);

#[pymethods]
impl FloatBuffer {
    // Constructor

    #[new]
    #[pyo3(signature = (source = None))]
    fn new(source: Option<&Bound<'_, PyAny>>) -> PyResult<Self> {
        let Some(s) = source else {
            return Ok(Self::wrap(pyxel::cube::FloatBuffer::with_size(0)));
        };
        if let Ok(size) = s.extract::<usize>() {
            return Ok(Self::wrap(pyxel::cube::FloatBuffer::with_size(size)));
        }
        if let Ok(values) = s.extract::<Vec<f32>>() {
            return Ok(Self::wrap(pyxel::cube::FloatBuffer::from_values(values)));
        }
        Err(PyTypeError::new_err(
            "FloatBuffer source must be int or list[float]",
        ))
    }

    // Attributes

    #[getter]
    fn size(&self) -> usize {
        self.inner_ref().size()
    }

    // Dunder methods

    fn __repr__(&self) -> String {
        format!("FloatBuffer(size={})", self.inner_ref().size())
    }

    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        let Ok(o) = other.cast::<Self>() else {
            return false;
        };
        let lhs = self.inner_ref();
        let rhs = o.borrow();
        lhs.data() == rhs.inner_ref().data()
    }

    fn __getitem__(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let n = self.inner_ref().size();
        if let Ok(slice) = key.cast::<PySlice>() {
            let ind = slice.indices(n as isize)?;
            let buf = self.inner_ref();
            let mut out = Vec::with_capacity(ind.slicelength);
            let mut idx = ind.start;
            for _ in 0..ind.slicelength {
                out.push(buf.get(idx as usize));
                idx += ind.step;
            }
            return Ok(PyList::new(py, out)?.into_any().unbind());
        }
        let i = key.extract::<isize>()?;
        let idx = normalize_index(i, n)?;
        Ok(self
            .inner_ref()
            .get(idx)
            .into_pyobject(py)?
            .into_any()
            .unbind())
    }

    fn __setitem__(&self, key: &Bound<'_, PyAny>, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let n = self.inner_ref().size();
        if let Ok(slice) = key.cast::<PySlice>() {
            let ind = slice.indices(n as isize)?;
            let slice_len = ind.slicelength;
            if let Ok(src) = value.cast::<Self>() {
                let src_b = src.borrow();
                let src_data: Vec<f32> = {
                    let src_buf = src_b.inner_ref();
                    if src_buf.size() != slice_len {
                        return Err(PyValueError::new_err("FloatBuffer slice size mismatch"));
                    }
                    src_buf.data().to_vec()
                };
                let buf = self.inner_mut();
                let mut idx = ind.start;
                for v in &src_data {
                    buf.set(idx as usize, *v);
                    idx += ind.step;
                }
                return Ok(());
            }
            let values = value.extract::<Vec<f32>>().map_err(|_| {
                PyTypeError::new_err("FloatBuffer slice value must be list[float] or FloatBuffer")
            })?;
            if values.len() != slice_len {
                return Err(PyValueError::new_err("FloatBuffer slice size mismatch"));
            }
            let buf = self.inner_mut();
            let mut idx = ind.start;
            for v in &values {
                buf.set(idx as usize, *v);
                idx += ind.step;
            }
            return Ok(());
        }
        let i = key.extract::<isize>()?;
        let idx = normalize_index(i, n)?;
        let v = value.extract::<f32>()?;
        self.inner_mut().set(idx, v);
        Ok(())
    }

    fn __iter__(slf: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let data = slf.inner_ref().data().to_vec();
        let list = PyList::new(py, data)?;
        Ok(list.call_method0("__iter__")?.unbind())
    }

    fn __len__(&self) -> usize {
        self.inner_ref().size()
    }

    // In-place operations

    fn fill(&self, value: f32) {
        self.inner_mut().fill(value);
    }

    fn resize(&self, size: usize) {
        self.inner_mut().resize(size);
    }

    // Buffer protocol

    unsafe fn __getbuffer__(
        slf: Bound<'_, Self>,
        view: *mut pyo3::ffi::Py_buffer,
        flags: std::os::raw::c_int,
    ) -> PyResult<()> {
        // "f" — struct format code for a 32-bit float.
        static FORMAT: [std::os::raw::c_char; 2] = [b'f' as std::os::raw::c_char, 0];

        if view.is_null() {
            return Err(PyBufferError::new_err("buffer view is null"));
        }
        let (buf_ptr, len) = {
            let borrowed = slf.borrow();
            let inner = borrowed.inner_ref();
            (inner.data().as_ptr(), inner.size())
        };
        let itemsize = std::mem::size_of::<f32>();
        let layout = Box::new(FloatBufferView1D {
            shape: len as pyo3::ffi::Py_ssize_t,
            strides: itemsize as pyo3::ffi::Py_ssize_t,
        });
        let layout_ptr = Box::into_raw(layout);

        (*view).buf = buf_ptr.cast::<std::ffi::c_void>().cast_mut();
        (*view).obj = slf.into_any().into_ptr();
        (*view).len = (len * itemsize) as pyo3::ffi::Py_ssize_t;
        (*view).readonly = 0;
        (*view).itemsize = itemsize as pyo3::ffi::Py_ssize_t;
        (*view).format = if (flags & pyo3::ffi::PyBUF_FORMAT) == pyo3::ffi::PyBUF_FORMAT {
            FORMAT.as_ptr().cast_mut()
        } else {
            std::ptr::null_mut()
        };
        (*view).ndim = 1;
        (*view).shape = if (flags & pyo3::ffi::PyBUF_ND) == pyo3::ffi::PyBUF_ND {
            &raw mut (*layout_ptr).shape
        } else {
            std::ptr::null_mut()
        };
        (*view).strides = if (flags & pyo3::ffi::PyBUF_STRIDES) == pyo3::ffi::PyBUF_STRIDES {
            &raw mut (*layout_ptr).strides
        } else {
            std::ptr::null_mut()
        };
        (*view).suboffsets = std::ptr::null_mut();
        (*view).internal = layout_ptr.cast::<std::ffi::c_void>();
        Ok(())
    }

    #[allow(clippy::unused_self)]
    unsafe fn __releasebuffer__(&self, view: *mut pyo3::ffi::Py_buffer) {
        if !view.is_null() && !(*view).internal.is_null() {
            drop(Box::from_raw((*view).internal.cast::<FloatBufferView1D>()));
        }
    }
}

pub fn add_float_buffer_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FloatBuffer>()?;
    Ok(())
}

// File-private helpers

#[repr(C)]
struct FloatBufferView1D {
    shape: pyo3::ffi::Py_ssize_t,
    strides: pyo3::ffi::Py_ssize_t,
}

fn normalize_index(i: isize, n: usize) -> PyResult<usize> {
    let n_isize = n as isize;
    let normalized = if i < 0 { i + n_isize } else { i };
    if normalized < 0 || normalized >= n_isize {
        return Err(PyIndexError::new_err("FloatBuffer index out of range"));
    }
    Ok(normalized as usize)
}
