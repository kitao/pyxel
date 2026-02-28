use std::sync::Once;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyList, PySlice, PyTuple};

static SNDS_LIST_ONCE: Once = Once::new();

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
    pub fn wrap(inner: *mut pyxel::Music) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Seqs {
    fn __len__(&self) -> usize {
        unsafe { &*self.inner }.seqs.len()
    }

    fn __getitem__<'py>(&self, py: Python<'py>, key: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let len = unsafe { &*self.inner }.seqs.len();
            let indices = slice.indices(len as isize)?;
            let idx_list = collect_slice_indices!(indices.start, indices.stop, indices.step);
            let items: Vec<Seq> = idx_list
                .iter()
                .map(|&i| {
                    Seq::wrap(SeqRef {
                        music: self.inner,
                        index: i,
                    })
                })
                .collect();
            let list = PyList::new(py, items)?;
            Ok(list.into_any().unbind())
        } else {
            let idx: isize = key.extract()?;
            let len = unsafe { &*self.inner }.seqs.len();
            let i = resolve_index!(idx, len)?;
            let seq = Seq::wrap(SeqRef {
                music: self.inner,
                index: i,
            });
            Ok(seq.into_pyobject(py)?.into_any().unbind())
        }
    }

    fn __setitem__<'py>(
        &self,
        _py: Python<'py>,
        key: &Bound<'py, PyAny>,
        value: &Bound<'py, PyAny>,
    ) -> PyResult<()> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let music = unsafe { &mut *self.inner };
            let len = music.seqs.len();
            let indices = slice.indices(len as isize)?;
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
            let music = unsafe { &mut *self.inner };
            let len = music.seqs.len();
            let i = resolve_index!(idx, len)?;
            let val: Vec<u32> = value.extract()?;
            music.seqs[i] = val;
            Ok(())
        }
    }

    fn __delitem__<'py>(&self, _py: Python<'py>, key: &Bound<'py, PyAny>) -> PyResult<()> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let music = unsafe { &mut *self.inner };
            let len = music.seqs.len();
            let indices = slice.indices(len as isize)?;
            let mut idx_list = collect_slice_indices!(indices.start, indices.stop, indices.step);
            idx_list.sort_unstable_by(|a, b| b.cmp(a));
            for i in idx_list {
                music.seqs.remove(i);
            }
            Ok(())
        } else {
            let idx: isize = key.extract()?;
            let music = unsafe { &mut *self.inner };
            let len = music.seqs.len();
            let i = resolve_index!(idx, len)?;
            music.seqs.remove(i);
            Ok(())
        }
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let len = unsafe { &*self.inner }.seqs.len();
        let items: Vec<Seq> = (0..len)
            .map(|i| {
                Seq::wrap(SeqRef {
                    music: self.inner,
                    index: i,
                })
            })
            .collect();
        let list = PyList::new(py, items)?;
        let iter = list.call_method0("__iter__")?;
        Ok(iter.unbind())
    }

    fn __reversed__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let len = unsafe { &*self.inner }.seqs.len();
        let items: Vec<Seq> = (0..len)
            .rev()
            .map(|i| {
                Seq::wrap(SeqRef {
                    music: self.inner,
                    index: i,
                })
            })
            .collect();
        let list = PyList::new(py, items)?;
        let iter = list.call_method0("__iter__")?;
        Ok(iter.unbind())
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let music = unsafe { &*self.inner };
        let seqs: Vec<Vec<u32>> = music.seqs.clone();
        let list = PyList::new(py, seqs)?;
        let repr = list.repr()?;
        Ok(format!("Seqs{}", repr.to_string_lossy()))
    }

    fn __bool__(&self) -> bool {
        !unsafe { &*self.inner }.seqs.is_empty()
    }

    fn __iadd__(&self, values: Vec<Vec<u32>>) {
        let music = unsafe { &mut *self.inner };
        for val in values {
            music.seqs.push(val);
        }
    }

    pub fn append(&self, value: Vec<u32>) {
        unsafe { &mut *self.inner }.seqs.push(value);
    }

    pub fn extend(&self, values: Vec<Vec<u32>>) {
        let music = unsafe { &mut *self.inner };
        for val in values {
            music.seqs.push(val);
        }
    }

    #[pyo3(signature = (index, value))]
    pub fn insert(&self, index: isize, value: Vec<u32>) {
        let music = unsafe { &mut *self.inner };
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
    pub fn pop(&self, index: Option<isize>) -> PyResult<Seq> {
        let music = unsafe { &mut *self.inner };
        let len = music.seqs.len();
        if len == 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "pop from empty sequence",
            ));
        }
        let idx = index.unwrap_or(-1);
        let i = resolve_index!(idx, len)?;
        music.seqs.remove(i);
        let new_len = music.seqs.len();
        let safe_index = if new_len == 0 { 0 } else { i.min(new_len - 1) };
        Ok(Seq::wrap(SeqRef {
            music: self.inner,
            index: safe_index,
        }))
    }

    pub fn clear(&self) {
        unsafe { &mut *self.inner }.seqs.clear();
    }

    pub fn from_list(&self, list: Vec<Vec<u32>>) {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            println!("Seqs.from_list() is deprecated. Use slice assignment instead.");
        });
        unsafe { &mut *self.inner }.set(&list);
    }

    pub fn to_list(&self, py: Python) -> PyResult<Py<PyAny>> {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            println!("Seqs.to_list() is deprecated. Use list(seq) instead.");
        });
        let music = unsafe { &*self.inner };
        let seqs: Vec<Vec<u32>> = music.seqs.clone();
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
}

#[pymethods]
impl Music {
    #[new]
    pub fn new() -> Self {
        Self::wrap(pyxel::Music::new())
    }

    #[getter]
    pub fn seqs(&self) -> Seqs {
        Seqs::wrap(self.inner)
    }

    #[pyo3(signature = (*seqs))]
    pub fn set(&self, seqs: &Bound<'_, PyTuple>) -> PyResult<()> {
        let mut rust_seqs: Vec<Vec<u32>> = Vec::new();
        for i in 0..seqs.len() {
            let rust_seq: Vec<u32> = seqs.get_item(i)?.extract()?;
            rust_seqs.push(rust_seq);
        }

        unsafe { &mut *self.inner }.set(&rust_seqs);
        Ok(())
    }

    #[pyo3(signature = (filename, sec, ffmpeg=None))]
    pub fn save(&self, filename: &str, sec: f32, ffmpeg: Option<bool>) -> PyResult<()> {
        unsafe { &mut *self.inner }
            .save(filename, sec, ffmpeg)
            .map_err(PyException::new_err)
    }

    #[getter]
    pub fn snds_list(&self) -> Seqs {
        SNDS_LIST_ONCE.call_once(|| {
            println!("Music.snds_list[ch] is deprecated. Use Music.seqs[ch] instead.",);
        });

        Seqs::wrap(self.inner)
    }
}

pub fn add_music_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Seqs>()?;
    m.add_class::<Music>()?;
    Ok(())
}
