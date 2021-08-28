use pyo3::prelude::*;

use pyxel::Channel as PyxelChannel;
use pyxel::SharedChannel as PyxelSharedChannel;
use pyxel::Volume;

use crate::instance;
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
    pub fn new() -> PyResult<Channel> {
        Ok(wrap_pyxel_channel(PyxelChannel::new()))
    }

    #[getter]
    pub fn get_volume(&self) -> PyResult<Volume> {
        Ok(self.pyxel_channel.lock().volume)
    }

    #[setter]
    pub fn set_volume(&self, volume: Volume) -> PyResult<()> {
        self.pyxel_channel.lock().volume = volume;

        Ok(())
    }

    pub fn play_pos(&self) -> PyResult<Option<(u32, u32)>> {
        Ok(self.pyxel_channel.lock().play_pos())
    }

    pub fn play(&self, sounds: &PyAny, is_looping: Option<bool>) -> PyResult<()> {
        type_switch! {
            sounds,
            Vec<Sound>,
            {
                let sounds = sounds
                    .iter()
                    .map(|sound| sound.pyxel_sound.lock().clone())
                    .collect();

                self.pyxel_channel
                    .lock()
                    .play(sounds, is_looping.unwrap_or(false));
            },
            Sound,
            {
                self.pyxel_channel.lock().play1(
                    sounds.pyxel_sound.lock().clone(),
                    is_looping.unwrap_or(false),
                );
            },
            Vec<u32>,
            {
                let sounds = sounds
                    .iter()
                    .map(|sound_no| instance().sound(*sound_no).lock().clone())
                    .collect();

                self.pyxel_channel
                    .lock()
                    .play(sounds, is_looping.unwrap_or(false));
            },
            u32,
            {
                self.pyxel_channel.lock().play1(
                    instance().sound(sounds).lock().clone(),
                    is_looping.unwrap_or(false),
                );
            }
        }

        Ok(())
    }

    pub fn stop(&mut self) -> PyResult<()> {
        self.pyxel_channel.lock().stop();

        Ok(())
    }
}

pub fn add_channel_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Channel>()?;

    Ok(())
}
