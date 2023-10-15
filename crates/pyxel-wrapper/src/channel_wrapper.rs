use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;
use crate::sound_wrapper::Sound;

#[pyclass]
#[derive(Clone)]
pub struct Channel {
    pyxel_channel: pyxel::SharedChannel,
}

pub const fn wrap_pyxel_channel(pyxel_channel: pyxel::SharedChannel) -> Channel {
    Channel { pyxel_channel }
}

#[pymethods]
impl Channel {
    #[getter]
    pub fn get_gain(&self) -> pyxel::Volume {
        self.pyxel_channel.lock().gain
    }

    #[setter]
    pub fn set_gain(&self, gain: u8) {
        self.pyxel_channel.lock().gain = gain;
    }

    #[pyo3(text_signature = "($self, snd, *, tick, loop)")]
    pub fn play(&self, snd: &PyAny, tick: Option<u32>, r#loop: Option<bool>) -> PyResult<()> {
        let loop_ = r#loop.unwrap_or(false);
        type_switch! {
            snd,
            u32, {
                let sound = pyxel().sounds[snd as usize].clone();
                self.pyxel_channel.lock().play1(sound, tick, loop_);
            },
            Vec<u32>, {
                let sounds = snd.iter().map(|snd| pyxel().sounds[*snd as usize].clone()).collect();
                self.pyxel_channel.lock().play(sounds, tick, loop_);
            },
            Sound, {
                self.pyxel_channel.lock().play1(snd.pyxel_sound, tick, loop_);
            },
            Vec<Sound>, {
                let sounds = snd.iter().map(|sound| sound.pyxel_sound.clone()).collect();
                self.pyxel_channel.lock().play(sounds, tick, loop_);
            }
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        self.pyxel_channel.lock().stop();
    }

    pub fn play_pos(&self) -> Option<(u32, u32)> {
        self.pyxel_channel.lock().play_pos()
    }
}

pub fn add_channel_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Channel>()?;
    Ok(())
}
