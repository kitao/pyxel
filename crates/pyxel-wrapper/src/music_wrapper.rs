use std::sync::Once;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

use crate::utils::python_warn;

static SNDS_LIST_ONCE: Once = Once::new();

wrap_as_python_list!(
    Seq,
    u32,
    pyxel::SharedSeq,
    (|inner: &pyxel::SharedSeq| inner.lock().len()),
    (|inner: &pyxel::SharedSeq, index| inner.lock()[index]),
    (|inner: &pyxel::SharedSeq, index, value| inner.lock()[index] = value),
    (|inner: &pyxel::SharedSeq, index, value| inner.lock().insert(index, value)),
    (|inner: &pyxel::SharedSeq, index| inner.lock().remove(index))
);

wrap_as_python_list!(
    Seqs,
    Seq,
    pyxel::SharedMusic,
    (|inner: &pyxel::SharedMusic| inner.lock().seqs.len()),
    (|inner: &pyxel::SharedMusic, index: usize| Seq::wrap(inner.lock().seqs[index].clone())),
    (|inner: &pyxel::SharedMusic, index, value: Seq| inner.lock().seqs[index] = value.inner),
    (|inner: &pyxel::SharedMusic, index, value: Seq| inner.lock().seqs.insert(index, value.inner)),
    (|inner: &pyxel::SharedMusic, index| inner.lock().seqs.remove(index))
);

#[pyclass]
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
    pub fn set(&self, seqs: &PyTuple) {
        let mut rust_seqs: Vec<Vec<u32>> = Vec::new();
        for i in 0..seqs.len() {
            let seq = seqs.get_item(i).unwrap().downcast::<PyList>().unwrap();
            let rust_seq = seq.extract::<Vec<u32>>().unwrap();
            rust_seqs.push(rust_seq);
        }
        self.inner.lock().set(&rust_seqs);
    }

    #[getter]
    pub fn snds_list(&self) -> Seqs {
        SNDS_LIST_ONCE.call_once(|| {
            python_warn("Music.snds_list[ch] is deprecated, use Music.seqs[ch] instead.");
        });
        Seqs::wrap(self.inner.clone())
    }
}

pub fn add_music_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Seqs>()?;
    m.add_class::<Music>()?;
    Ok(())
}
