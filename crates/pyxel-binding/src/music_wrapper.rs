use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyList, PySlice, PyTuple};

// Python sequence wrappers for the mutable music sequences

#[derive(Clone)]
pub struct SeqRef {
    inner: pyxel::RcMusic,
    index: usize,
}

struct MusicSeqMut<'a> {
    music: MutexGuard<'a, pyxel::Music>,
    index: usize,
}

impl Deref for MusicSeqMut<'_> {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.music.seqs[self.index]
    }
}

impl DerefMut for MusicSeqMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.music.seqs[self.index]
    }
}

fn seq_mut(inner: &SeqRef) -> MusicSeqMut<'_> {
    MusicSeqMut {
        music: audio_mut!(inner.inner),
        index: inner.index,
    }
}

wrap_as_python_primitive_sequence!(
    Seq,
    SeqRef,
    (|inner: &SeqRef| audio_ref!(inner.inner).seqs[inner.index].len()),
    u32,
    (|inner: &SeqRef, index| audio_ref!(inner.inner).seqs[inner.index][index]),
    u32,
    (|inner: &SeqRef, index, value| audio_mut!(inner.inner).seqs[inner.index][index] = value),
    seq_mut,
    Vec<u32>,
    (|inner: &SeqRef, list| audio_mut!(inner.inner).seqs[inner.index] = list),
    (|inner: &SeqRef| audio_ref!(inner.inner).seqs[inner.index].clone())
);

// Seqs is hand-written because it returns Seq wrapper objects (asymmetric get/set types)
#[pyclass(sequence, unsendable, skip_from_py_object)]
#[derive(Clone)]
pub struct Seqs {
    inner: pyxel::RcMusic,
}

impl Seqs {
    fn wrap(inner: pyxel::RcMusic) -> Self {
        Self { inner }
    }

    fn inner_ref(&self) -> MutexGuard<'_, pyxel::Music> {
        audio_ref!(self.inner)
    }

    fn inner_mut(&self) -> MutexGuard<'_, pyxel::Music> {
        audio_mut!(self.inner)
    }

    fn wrap_seq(&self, index: usize) -> Seq {
        Seq::wrap(SeqRef {
            inner: self.inner.clone(),
            index,
        })
    }
}

#[pymethods]
impl Seqs {
    // Sequence dunders

    fn __len__(&self) -> usize {
        self.inner_ref().seqs.len()
    }

    fn __getitem__<'py>(&self, py: Python<'py>, key: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let indices = slice.indices(self.__len__() as isize)?;
            let items =
                crate::utils::slice_indices(indices.start, indices.step, indices.slicelength)
                    .map(|i| self.wrap_seq(i));
            let list = PyList::new(py, items)?;
            Ok(list.into_any().unbind())
        } else {
            let idx: isize = key.extract()?;
            let i = resolve_index!(idx, self.__len__())?;
            Ok(self.wrap_seq(i).into_pyobject(py)?.into_any().unbind())
        }
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let items = (0..self.__len__()).map(|i| self.wrap_seq(i));
        items_to_pyiter!(py, items)
    }

    fn __reversed__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let items = (0..self.__len__()).rev().map(|i| self.wrap_seq(i));
        items_to_pyiter!(py, items)
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let music = self.inner_ref();
        let list = PyList::new(py, music.seqs.iter().cloned())?;
        Ok(format!(
            "{}{}",
            stringify!(Seqs),
            list.repr()?.to_string_lossy()
        ))
    }

    fn __bool__(&self) -> bool {
        !self.inner_ref().seqs.is_empty()
    }

    fn __setitem__<'py>(
        &self,
        _py: Python<'py>,
        key: &Bound<'py, PyAny>,
        value: &Bound<'py, PyAny>,
    ) -> PyResult<()> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let mut music = self.inner_mut();
            let indices = slice.indices(music.seqs.len() as isize)?;
            let new_values: Vec<Vec<u32>> = value.extract()?;
            if indices.step == 1 {
                let start = indices.start as usize;
                let end = indices.stop.max(indices.start) as usize;
                music.seqs.splice(start..end, new_values);
            } else {
                if new_values.len() != indices.slicelength {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "attempt to assign sequence of size {} to extended slice of size {}",
                        new_values.len(),
                        indices.slicelength
                    )));
                }
                let positions =
                    crate::utils::slice_indices(indices.start, indices.step, indices.slicelength);
                for (pos, val) in positions.zip(new_values) {
                    music.seqs[pos] = val;
                }
            }
            Ok(())
        } else {
            let idx: isize = key.extract()?;
            let mut music = self.inner_mut();
            let i = resolve_index!(idx, music.seqs.len())?;
            music.seqs[i] = value.extract()?;
            Ok(())
        }
    }

    fn __delitem__<'py>(&self, _py: Python<'py>, key: &Bound<'py, PyAny>) -> PyResult<()> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let mut music = self.inner_mut();
            let indices = slice.indices(music.seqs.len() as isize)?;
            let mut idx_list: Vec<usize> =
                crate::utils::slice_indices(indices.start, indices.step, indices.slicelength)
                    .collect();
            // Remove from end to preserve earlier indices
            idx_list.sort_unstable_by(|a, b| b.cmp(a));
            for i in idx_list {
                music.seqs.remove(i);
            }
            Ok(())
        } else {
            let idx: isize = key.extract()?;
            let mut music = self.inner_mut();
            let i = resolve_index!(idx, music.seqs.len())?;
            music.seqs.remove(i);
            Ok(())
        }
    }

    fn __iadd__(&self, values: Vec<Vec<u32>>) {
        self.inner_mut().seqs.extend(values);
    }

    // List operations

    fn append(&self, value: Vec<u32>) {
        self.inner_mut().seqs.push(value);
    }

    fn extend(&self, values: Vec<Vec<u32>>) {
        self.inner_mut().seqs.extend(values);
    }

    #[pyo3(signature = (index, value))]
    fn insert(&self, index: isize, value: Vec<u32>) {
        let mut music = self.inner_mut();
        let len = music.seqs.len();
        let i = (if index < 0 {
            index + len as isize
        } else {
            index
        })
        .clamp(0, len as isize) as usize;
        music.seqs.insert(i, value);
    }

    #[pyo3(signature = (index=None))]
    fn pop(&self, py: Python, index: Option<isize>) -> PyResult<Py<PyAny>> {
        let mut music = self.inner_mut();
        let len = music.seqs.len();
        if len == 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "pop from empty sequence",
            ));
        }
        let i = resolve_index!(index.unwrap_or(-1), len)?;
        let removed = music.seqs.remove(i);
        Ok(PyList::new(py, &removed)?.unbind().into_any())
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

define_audio_wrapper!(Music, pyxel::Music, pyxel::RcMusic);

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
        Seqs::wrap(self.inner.clone())
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
        let music = self.inner_ref().clone();
        music
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
        Seqs::wrap(self.inner.clone())
    }
}

// Module registration

pub fn add_music_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Seqs>()?;
    m.add_class::<Music>()?;
    Ok(())
}
