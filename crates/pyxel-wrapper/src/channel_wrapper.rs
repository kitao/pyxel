use parking_lot::Mutex;
use pyo3::prelude::*;
use std::sync::Arc;

use pyxel::Channel as PyxelChannel;
use pyxel::Volume;

#[pyclass]
#[derive(Clone)]
pub struct Channel {
    pub pyxel_channel: Arc<Mutex<PyxelChannel>>,
}

pub fn wrap_pyxel_channel(pyxel_channel: Arc<Mutex<PyxelChannel>>) -> Channel {
    Channel {
        pyxel_channel: pyxel_channel,
    }
}

#[pymethods]
impl Channel {
    #[new]
    pub fn new() -> Channel {
        wrap_pyxel_channel(PyxelChannel::new())
    }

    #[getter]
    pub fn get_volume(&self) -> Volume {
        self.pyxel_channel.lock().volume
    }

    #[setter]
    pub fn set_volume(&self, volume: Volume) {
        self.pyxel_channel.lock().volume = volume;
    }

    #[getter]
    pub fn is_playing(&self) -> bool {
        self.pyxel_channel.lock().is_playing()
    }

    #[getter]
    pub fn is_looping(&self) -> bool {
        self.pyxel_channel.lock().is_looping()
    }

    #[getter]
    pub fn sound_index(&self) -> u32 {
        self.pyxel_channel.lock().sound_index()
    }

    #[getter]
    pub fn note_index(&self) -> u32 {
        self.pyxel_channel.lock().note_index()
    }

    /*pub fn play(&mut self, sounds: Vec<Sound>, is_looping: bool) {
    }*/

    pub fn stop(&mut self) {
        self.pyxel_channel.lock().stop();
    }
}

pub fn add_channel_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Channel>()?;

    Ok(())
}
