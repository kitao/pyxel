use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;

use pyxel::Music as PyxelMusic;
use pyxel::SharedMusic as PyxelSharedMusic;

#[pyclass]
#[derive(Clone)]
pub struct Sequence {
    pyxel_music: PyxelSharedMusic,
    channel_no: u32,
}

#[pyproto]
impl PySequenceProtocol for Sequence {
    fn __len__(&self) -> PyResult<usize> {
        sequence_len!(self.pyxel_music.lock().sequences[self.channel_no as usize])
    }

    fn __getitem__(&self, idx: isize) -> PyResult<u32> {
        sequence_get!(
            self.pyxel_music.lock().sequences[self.channel_no as usize],
            idx
        )
    }

    fn __setitem__(&mut self, idx: isize, value: u32) -> PyResult<()> {
        sequence_set!(
            self.pyxel_music.lock().sequences[self.channel_no as usize],
            idx,
            value
        )
    }

    fn __delitem__(&mut self, idx: isize) -> PyResult<()> {
        sequence_del!(
            self.pyxel_music.lock().sequences[self.channel_no as usize],
            idx
        )
    }

    /*fn __inplace_concat__(&mut self, other: Vec<u32>) -> PyResult<()> {
        self.pyxel_music.lock().sequences[self.channel_no as usize].append(&mut other);

        Ok(())
    }*/
}

#[pyclass]
#[derive(Clone)]
pub struct Sequences {
    pyxel_music: PyxelSharedMusic,
}

#[pyproto]
impl PySequenceProtocol for Sequences {
    fn __len__(&self) -> PyResult<usize> {
        sequence_len!(self.pyxel_music.lock().sequences)
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Sequence> {
        match sequence_get!(self.pyxel_music.lock().sequences, idx) {
            Ok(_) => Ok(Sequence {
                pyxel_music: self.pyxel_music.clone(),
                channel_no: idx as u32,
            }),
            Err(err) => Err(err),
        }
    }

    fn __setitem__(&mut self, idx: isize, sequence: Vec<u32>) -> PyResult<()> {
        sequence_set!(self.pyxel_music.lock().sequences, idx, sequence)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Music {
    pyxel_music: PyxelSharedMusic,
}

pub fn wrap_pyxel_music(pyxel_music: PyxelSharedMusic) -> Music {
    Music { pyxel_music }
}

#[pymethods]
impl Music {
    #[new]
    pub fn new() -> PyResult<Music> {
        Ok(wrap_pyxel_music(PyxelMusic::new()))
    }

    pub fn set(
        &self,
        seq0: Vec<u32>,
        seq1: Vec<u32>,
        seq2: Vec<u32>,
        seq3: Vec<u32>,
    ) -> PyResult<()> {
        self.pyxel_music.lock().set(&seq0, &seq1, &seq2, &seq3);

        Ok(())
    }

    #[getter]
    pub fn sequences(&self) -> PyResult<Sequences> {
        Ok(Sequences {
            pyxel_music: self.pyxel_music.clone(),
        })
    }
}

pub fn add_music_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Sequence>()?;
    m.add_class::<Sequences>()?;
    m.add_class::<Music>()?;

    Ok(())
}
