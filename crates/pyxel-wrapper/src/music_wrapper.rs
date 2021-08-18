use parking_lot::Mutex;
use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;
use std::sync::Arc;

use pyxel::Music as PyxelMusic;

#[pyclass]
#[derive(Clone)]
pub struct Sequence {
    pyxel_music: Arc<Mutex<PyxelMusic>>,
    channel_no: u32,
}

#[pyproto]
impl PySequenceProtocol for Sequence {
    fn __len__(&self) -> usize {
        self.pyxel_music.lock().sequences[self.channel_no as usize].len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<u32> {
        Ok(self.pyxel_music.lock().sequences[self.channel_no as usize][idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, sound_no: u32) {
        self.pyxel_music.lock().sequences[self.channel_no as usize][idx as usize] = sound_no;
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Sequences {
    music: Arc<Mutex<PyxelMusic>>,
}

#[pyproto]
impl PySequenceProtocol for Sequences {
    fn __len__(&self) -> usize {
        self.music.lock().sequences.len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Sequence> {
        Ok(Sequence {
            pyxel_music: self.music.clone(),
            channel_no: idx as u32,
        })
    }

    /*
    fn __setitem__(&mut self, idx: isize, sequcne: Sequence) {
        self.music.lock().sequences[self.channel_no as usize][idx as usize] = sound_no;
    }
    */
}

#[pyclass]
#[derive(Clone)]
pub struct Music {
    pyxel_music: Arc<Mutex<PyxelMusic>>,
}

pub fn wrap_pyxel_music(pyxel_music: Arc<Mutex<PyxelMusic>>) -> Music {
    Music {
        pyxel_music: pyxel_music,
    }
}

#[pymethods]
impl Music {
    #[new]
    pub fn new() -> Music {
        wrap_pyxel_music(PyxelMusic::with_arc_mutex())
    }

    pub fn set(&self, sequences: Vec<Vec<u32>>) {
        self.pyxel_music.lock().set(&sequences);
    }
}

pub fn add_music_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Sequence>()?;
    m.add_class::<Sequences>()?;
    m.add_class::<Music>()?;

    Ok(())
}
