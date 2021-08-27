use pyo3::prelude::*;

use pyxel::Channel as PyxelChannel;
use pyxel::SharedChannel as PyxelSharedChannel;
use pyxel::Volume;

use crate::sound_wrapper::Sound;

#[pyclass]
#[derive(Clone)]
pub struct Channel {
    pub pyxel_channel: PyxelSharedChannel,
}

pub fn wrap_pyxel_channel(pyxel_channel: PyxelSharedChannel) -> Channel {
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

    pub fn play(&self, sounds: &PyAny, is_looping: Option<bool>) {
        if let Ok(sounds) = sounds.extract::<Vec<Sound>>() {
            let sounds = sounds
                .iter()
                .map(|sound| sound.pyxel_sound.lock().clone())
                .collect();

            self.pyxel_channel
                .lock()
                .play(sounds, is_looping.unwrap_or(false));
        } else if let Ok(sound) = sounds.extract::<Sound>() {
            self.pyxel_channel.lock().play1(
                sound.pyxel_sound.lock().clone(),
                is_looping.unwrap_or(false),
            );
        } else {
            panic!("Invalid arguments for play");
        }
    }

    pub fn stop(&mut self) {
        self.pyxel_channel.lock().stop();
    }
}

pub fn add_channel_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Channel>()?;

    Ok(())
}
