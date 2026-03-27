use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyList, PySlice, PyTuple};

#[derive(Clone, Copy)]
pub struct SeqRef {
    music: *mut pyxel::Music,
    index: usize,
}

unsafe impl Send for SeqRef {}
unsafe impl Sync for SeqRef {}

wrap_as_python_sequence!(
    Seq,
    SeqRef,
    (|inner: &SeqRef| unsafe { &*inner.music }.seqs[inner.index].len()),
    u32,
    (|inner: &SeqRef, index| unsafe { &*inner.music }.seqs[inner.index][index]),
    u32,
    (|inner: &SeqRef, index, value| unsafe { &mut *inner.music }.seqs[inner.index][index] = value),
    Vec<u32>,
    (|inner: &SeqRef, list| unsafe { &mut *inner.music }.seqs[inner.index] = list),
    (|inner: &SeqRef| unsafe { &*inner.music }.seqs[inner.index].clone())
);

// Seqs is hand-written because it returns Seq wrapper objects (asymmetric get/set types)
#[pyclass(sequence, skip_from_py_object)]
#[derive(Clone, Copy)]
pub struct Seqs {
    inner: *mut pyxel::Music,
}

unsafe impl Send for Seqs {}
unsafe impl Sync for Seqs {}

impl Seqs {
    fn wrap(inner: *mut pyxel::Music) -> Self {
        Self { inner }
    }

    fn inner_ref(&self) -> &pyxel::Music {
        unsafe { &*self.inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn inner_mut(&self) -> &mut pyxel::Music {
        unsafe { &mut *self.inner }
    }

    fn seq_ref(&self, index: usize) -> Seq {
        Seq::wrap(SeqRef {
            music: self.inner,
            index,
        })
    }
}

#[pymethods]
impl Seqs {
    // Read operations

    fn __len__(&self) -> usize {
        self.inner_ref().seqs.len()
    }

    fn __getitem__<'py>(&self, py: Python<'py>, key: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let indices = slice.indices(self.__len__() as isize)?;
            let idx_list = collect_slice_indices!(indices.start, indices.stop, indices.step);
            let items: Vec<Seq> = idx_list.iter().map(|&i| self.seq_ref(i)).collect();
            let list = PyList::new(py, items)?;
            Ok(list.into_any().unbind())
        } else {
            let idx: isize = key.extract()?;
            let i = resolve_index!(idx, self.__len__())?;
            Ok(self.seq_ref(i).into_pyobject(py)?.into_any().unbind())
        }
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let items: Vec<Seq> = (0..self.__len__()).map(|i| self.seq_ref(i)).collect();
        items_to_pyiter!(py, items)
    }

    fn __reversed__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let items: Vec<Seq> = (0..self.__len__()).rev().map(|i| self.seq_ref(i)).collect();
        items_to_pyiter!(py, items)
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let seqs: Vec<Vec<u32>> = self.inner_ref().seqs.clone();
        let list = PyList::new(py, seqs)?;
        Ok(format!("Seqs{}", list.repr()?.to_string_lossy()))
    }

    fn __bool__(&self) -> bool {
        !self.inner_ref().seqs.is_empty()
    }

    // Write operations

    fn __setitem__<'py>(
        &self,
        _py: Python<'py>,
        key: &Bound<'py, PyAny>,
        value: &Bound<'py, PyAny>,
    ) -> PyResult<()> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let music = self.inner_mut();
            let indices = slice.indices(music.seqs.len() as isize)?;
            let new_values: Vec<Vec<u32>> = value.extract()?;
            if indices.step == 1 {
                let start = indices.start as usize;
                let end = indices.stop as usize;
                music.seqs.splice(start..end, new_values);
            } else {
                let idx_list = collect_slice_indices!(indices.start, indices.stop, indices.step);
                if new_values.len() != idx_list.len() {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "attempt to assign sequence of size {} to extended slice of size {}",
                        new_values.len(),
                        idx_list.len()
                    )));
                }
                for (pos, val) in idx_list.into_iter().zip(new_values) {
                    music.seqs[pos] = val;
                }
            }
            Ok(())
        } else {
            let idx: isize = key.extract()?;
            let music = self.inner_mut();
            let i = resolve_index!(idx, music.seqs.len())?;
            music.seqs[i] = value.extract()?;
            Ok(())
        }
    }

    fn __delitem__<'py>(&self, _py: Python<'py>, key: &Bound<'py, PyAny>) -> PyResult<()> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let music = self.inner_mut();
            let indices = slice.indices(music.seqs.len() as isize)?;
            let mut idx_list = collect_slice_indices!(indices.start, indices.stop, indices.step);
            // Remove from end to preserve earlier indices
            idx_list.sort_unstable_by(|a, b| b.cmp(a));
            for i in idx_list {
                music.seqs.remove(i);
            }
            Ok(())
        } else {
            let idx: isize = key.extract()?;
            let music = self.inner_mut();
            let i = resolve_index!(idx, music.seqs.len())?;
            music.seqs.remove(i);
            Ok(())
        }
    }

    fn __iadd__(&self, values: Vec<Vec<u32>>) {
        self.inner_mut().seqs.extend(values);
    }

    fn append(&self, value: Vec<u32>) {
        self.inner_mut().seqs.push(value);
    }

    fn extend(&self, values: Vec<Vec<u32>>) {
        self.inner_mut().seqs.extend(values);
    }

    #[pyo3(signature = (index, value))]
    fn insert(&self, index: isize, value: Vec<u32>) {
        let music = self.inner_mut();
        let len = music.seqs.len();
        let i = if index < 0 {
            let resolved = index + len as isize;
            if resolved < 0 {
                0
            } else {
                resolved as usize
            }
        } else if index as usize > len {
            len
        } else {
            index as usize
        };
        music.seqs.insert(i, value);
    }

    #[pyo3(signature = (index=None))]
    fn pop(&self, py: Python, index: Option<isize>) -> PyResult<Py<PyAny>> {
        let music = self.inner_mut();
        let len = music.seqs.len();
        if len == 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "pop from empty sequence",
            ));
        }
        let i = resolve_index!(index.unwrap_or(-1), len)?;
        let removed = music.seqs.remove(i);
        Ok(PyList::new(py, &removed)?.into_any().unbind())
    }

    fn clear(&self) {
        self.inner_mut().seqs.clear();
    }

    // Deprecated methods

    fn from_list(&self, list: Vec<Vec<u32>>) {
        deprecation_warning!(
            FROM_LIST_ONCE,
            "Seqs.from_list() is deprecated. Use slice assignment instead."
        );
        self.inner_mut().set(&list);
    }

    fn to_list(&self, py: Python) -> PyResult<Py<PyAny>> {
        deprecation_warning!(
            TO_LIST_ONCE,
            "Seqs.to_list() is deprecated. Use list(seq) instead."
        );
        let seqs: Vec<Vec<u32>> = self.inner_ref().seqs.clone();
        let list = PyList::new(py, seqs)?;
        Ok(list.unbind().into_any())
    }
}

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub struct Music {
    pub(crate) inner: *mut pyxel::Music,
}

unsafe impl Send for Music {}
unsafe impl Sync for Music {}

impl Music {
    pub fn wrap(inner: *mut pyxel::Music) -> Self {
        Self { inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn inner_mut(&self) -> &mut pyxel::Music {
        unsafe { &mut *self.inner }
    }
}

#[pymethods]
impl Music {
    // Constructor

    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::Music::new())
    }

    // Properties

    #[getter]
    fn seqs(&self) -> Seqs {
        Seqs::wrap(self.inner)
    }

    // Data operations

    #[pyo3(signature = (*seqs))]
    fn set(&self, seqs: &Bound<'_, PyTuple>) -> PyResult<()> {
        let rust_seqs: Vec<Vec<u32>> = seqs
            .iter()
            .map(|item| item.extract())
            .collect::<PyResult<_>>()?;
        self.inner_mut().set(&rust_seqs);
        Ok(())
    }

    // File operations

    #[pyo3(signature = (filename, sec, ffmpeg=None))]
    fn save(&self, filename: &str, sec: f32, ffmpeg: Option<bool>) -> PyResult<()> {
        self.inner_mut()
            .save(filename, sec, ffmpeg)
            .map_err(PyException::new_err)
    }

    // Deprecated property

    #[getter]
    fn snds_list(&self) -> Seqs {
        deprecation_warning!(
            SNDS_LIST_ONCE,
            "Music.snds_list[ch] is deprecated. Use Music.seqs[ch] instead."
        );
        Seqs::wrap(self.inner)
    }
}

pub fn add_music_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Seqs>()?;
    m.add_class::<Music>()?;
    Ok(())
}
