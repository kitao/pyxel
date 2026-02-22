use std::sync::Once;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyList, PySlice, PyTuple};

static SNDS_LIST_ONCE: Once = Once::new();

wrap_as_python_sequence!(
    Seq,
    pyxel::SharedSeq,
    (|inner: &pyxel::SharedSeq| inner.lock().len()),
    u32,
    (|inner: &pyxel::SharedSeq, index| inner.lock()[index]),
    u32,
    (|inner: &pyxel::SharedSeq, index, value| inner.lock()[index] = value),
    Vec<u32>,
    (|inner: &pyxel::SharedSeq, list| *inner.lock() = list),
    (|inner: &pyxel::SharedSeq| inner.lock().clone())
);

// Seqs is hand-written because it returns Seq wrapper objects (asymmetric get/set types)
#[pyclass(sequence, skip_from_py_object)]
#[derive(Clone)]
pub struct Seqs {
    inner: pyxel::SharedMusic,
}

impl Seqs {
    pub const fn wrap(inner: pyxel::SharedMusic) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Seqs {
    fn __len__(&self) -> usize {
        self.inner.lock().seqs.len()
    }

    fn __getitem__<'py>(&self, py: Python<'py>, key: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let len = self.inner.lock().seqs.len();
            let indices = slice.indices(len as isize)?;
            let idx_list = collect_slice_indices!(indices.start, indices.stop, indices.step);
            let items: Vec<Seq> = idx_list
                .iter()
                .map(|&i| Seq::wrap(self.inner.lock().seqs[i].clone()))
                .collect();
            let list = PyList::new(py, items)?;
            Ok(list.into_any().unbind())
        } else {
            let idx: isize = key.extract()?;
            let len = self.inner.lock().seqs.len();
            let i = resolve_index!(idx, len)?;
            let seq = Seq::wrap(self.inner.lock().seqs[i].clone());
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
            let len = self.inner.lock().seqs.len();
            let indices = slice.indices(len as isize)?;
            let new_values: Vec<Vec<u32>> = value.extract()?;
            if indices.step == 1 {
                let start = indices.start as usize;
                let end = indices.stop as usize;
                let new_seqs: Vec<pyxel::SharedSeq> = new_values
                    .into_iter()
                    .map(|v| std::sync::Arc::new(parking_lot::Mutex::new(v)))
                    .collect();
                let mut music = self.inner.lock();
                music.seqs.splice(start..end, new_seqs);
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
                    *self.inner.lock().seqs[pos].lock() = val;
                }
            }
            Ok(())
        } else {
            let idx: isize = key.extract()?;
            let len = self.inner.lock().seqs.len();
            let i = resolve_index!(idx, len)?;
            let val: Vec<u32> = value.extract()?;
            *self.inner.lock().seqs[i].lock() = val;
            Ok(())
        }
    }

    fn __delitem__<'py>(&self, _py: Python<'py>, key: &Bound<'py, PyAny>) -> PyResult<()> {
        if let Ok(slice) = key.cast::<PySlice>() {
            let len = self.inner.lock().seqs.len();
            let indices = slice.indices(len as isize)?;
            let mut idx_list = collect_slice_indices!(indices.start, indices.stop, indices.step);
            idx_list.sort_unstable_by(|a, b| b.cmp(a));
            let mut music = self.inner.lock();
            for i in idx_list {
                music.seqs.remove(i);
            }
            Ok(())
        } else {
            let idx: isize = key.extract()?;
            let len = self.inner.lock().seqs.len();
            let i = resolve_index!(idx, len)?;
            self.inner.lock().seqs.remove(i);
            Ok(())
        }
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let len = self.inner.lock().seqs.len();
        let items: Vec<Seq> = (0..len)
            .map(|i| Seq::wrap(self.inner.lock().seqs[i].clone()))
            .collect();
        let list = PyList::new(py, items)?;
        let iter = list.call_method0("__iter__")?;
        Ok(iter.unbind())
    }

    fn __reversed__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let len = self.inner.lock().seqs.len();
        let items: Vec<Seq> = (0..len)
            .rev()
            .map(|i| Seq::wrap(self.inner.lock().seqs[i].clone()))
            .collect();
        let list = PyList::new(py, items)?;
        let iter = list.call_method0("__iter__")?;
        Ok(iter.unbind())
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let inner = self.inner.lock();
        let seqs: Vec<Vec<u32>> = inner.seqs.iter().map(|seq| seq.lock().clone()).collect();
        let list = PyList::new(py, seqs)?;
        let repr = list.repr()?;
        Ok(format!("Seqs{}", repr.to_string_lossy()))
    }

    fn __bool__(&self) -> bool {
        !self.inner.lock().seqs.is_empty()
    }

    fn __iadd__(&self, values: Vec<Vec<u32>>) {
        let mut music = self.inner.lock();
        for val in values {
            music
                .seqs
                .push(std::sync::Arc::new(parking_lot::Mutex::new(val)));
        }
    }

    pub fn append(&self, value: Vec<u32>) {
        self.inner
            .lock()
            .seqs
            .push(std::sync::Arc::new(parking_lot::Mutex::new(value)));
    }

    pub fn extend(&self, values: Vec<Vec<u32>>) {
        let mut music = self.inner.lock();
        for val in values {
            music
                .seqs
                .push(std::sync::Arc::new(parking_lot::Mutex::new(val)));
        }
    }

    #[pyo3(signature = (index, value))]
    pub fn insert(&self, index: isize, value: Vec<u32>) {
        let mut music = self.inner.lock();
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
        music
            .seqs
            .insert(i, std::sync::Arc::new(parking_lot::Mutex::new(value)));
    }

    #[pyo3(signature = (index=None))]
    pub fn pop(&self, index: Option<isize>) -> PyResult<Seq> {
        let mut music = self.inner.lock();
        let len = music.seqs.len();
        if len == 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "pop from empty sequence",
            ));
        }
        let idx = index.unwrap_or(-1);
        let i = resolve_index!(idx, len)?;
        let removed = music.seqs.remove(i);
        Ok(Seq::wrap(removed))
    }

    pub fn clear(&self) {
        self.inner.lock().seqs.clear();
    }

    pub fn from_list(&self, list: Vec<Vec<u32>>) {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            println!("Seqs.from_list() is deprecated. Use slice assignment instead.");
        });
        self.inner.lock().set(&list);
    }

    pub fn to_list(&self, py: Python) -> PyResult<Py<PyAny>> {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            println!("Seqs.to_list() is deprecated. Use list(seq) instead.");
        });
        let inner = self.inner.lock();
        let seqs: Vec<Vec<u32>> = inner.seqs.iter().map(|seq| seq.lock().clone()).collect();
        let list = PyList::new(py, seqs)?;
        Ok(list.unbind().into_any())
    }
}

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct Music {
    pub(crate) inner: pyxel::SharedMusic,
}

impl Music {
    pub fn wrap(inner: pyxel::SharedMusic) -> Self {
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
        Seqs::wrap(self.inner.clone())
    }

    #[pyo3(signature = (*seqs))]
    pub fn set(&self, seqs: &Bound<'_, PyTuple>) {
        let mut rust_seqs: Vec<Vec<u32>> = Vec::new();
        for i in 0..seqs.len() {
            let rust_seq = seqs
                .get_item(i)
                .unwrap()
                .cast::<PyList>()
                .unwrap()
                .extract::<Vec<u32>>()
                .unwrap();
            rust_seqs.push(rust_seq);
        }

        self.inner.lock().set(&rust_seqs);
    }

    #[pyo3(signature = (filename, sec, ffmpeg=None))]
    pub fn save(&self, filename: &str, sec: f32, ffmpeg: Option<bool>) -> PyResult<()> {
        self.inner
            .lock()
            .save(filename, sec, ffmpeg)
            .map_err(PyException::new_err)
    }

    #[getter]
    pub fn snds_list(&self) -> Seqs {
        SNDS_LIST_ONCE.call_once(|| {
            println!("Music.snds_list[ch] is deprecated. Use Music.seqs[ch] instead.",);
        });

        Seqs::wrap(self.inner.clone())
    }
}

pub fn add_music_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Seqs>()?;
    m.add_class::<Music>()?;
    Ok(())
}
