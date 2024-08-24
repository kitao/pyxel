use pyo3::prelude::*;

use crate::pyxel_singleton::pyxel;
use crate::sound_wrapper::Sound;

#[pyclass]
#[derive(Clone)]
pub struct Channel {
    pub(crate) inner: pyxel::SharedChannel,
}

impl Channel {
    pub fn wrap(inner: pyxel::SharedChannel) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Channel {
    #[new]
    pub fn new() -> Self {
        Self::wrap(pyxel::Channel::new())
    }

    #[getter]
    pub fn get_gain(&self) -> pyxel::Gain {
        self.inner.lock().gain
    }

    #[setter]
    pub fn set_gain(&self, gain: pyxel::Gain) {
        self.inner.lock().gain = gain;
    }

    #[getter]
    pub fn get_detune(&self) -> pyxel::Detune {
        self.inner.lock().detune
    }

    #[setter]
    pub fn set_detune(&self, detune: pyxel::Detune) {
        self.inner.lock().detune = detune;
    }

    #[pyo3(signature = (snd, tick=None, r#loop=None, resume=None))]
    pub fn play(
        &self,
        snd: Bound<'_, PyAny>,
        tick: Option<u32>,
        r#loop: Option<bool>,
        resume: Option<bool>,
    ) -> PyResult<()> {
        let loop_ = r#loop.unwrap_or(false);
        cast_pyany! {
            snd,
            (u32, {
                let sound = pyxel().sounds.lock()[snd as usize].clone();
                self.inner.lock().play1(sound, tick, loop_, resume.unwrap_or(false));
            }),
            (Vec<u32>, {
                let sounds = snd.iter().map(|snd| pyxel().sounds.lock()[*snd as usize].clone()).collect();
                self.inner.lock().play(sounds, tick, loop_, resume.unwrap_or(false));
            }),
            (Sound, { self.inner.lock().play1(snd.inner, tick, loop_, resume.unwrap_or(false)); }),
            (Vec<Sound>, {
                let sounds = snd.iter().map(|sound| sound.inner.clone()).collect();
                self.inner.lock().play(sounds, tick, loop_, resume.unwrap_or(false));
            })
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        self.inner.lock().stop();
    }

    pub fn play_pos(&self) -> Option<(u32, u32)> {
        self.inner.lock().play_pos()
    }
}

pub fn add_channel_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Channel>()?;
    Ok(())
}
