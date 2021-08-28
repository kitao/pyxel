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

    pub fn play(&self, snd: &PyAny, r#loop: Option<bool>) -> PyResult<()> {
        type_switch! {
            snd,
            Vec<Sound>,
            {
                let snd = snd
                    .iter()
                    .map(|sound| sound.pyxel_sound.lock().clone())
                    .collect();

                self.pyxel_channel
                    .lock()
                    .play(snd, r#loop.unwrap_or(false));
            },
            Sound,
            {
                self.pyxel_channel.lock().play1(
                    snd.pyxel_sound.lock().clone(),
                    r#loop.unwrap_or(false),
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
