use std::sync::Once;

use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;

use crate::sound_wrapper::Sound;

static PLAY_TICK_ONCE: Once = Once::new();

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub struct Channel {
    pub(crate) inner: *mut pyxel::Channel,
}

unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}

impl Channel {
    pub fn wrap(inner: *mut pyxel::Channel) -> Self {
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
    pub fn get_gain(&self) -> pyxel::ChannelGain {
        unsafe { &*self.inner }.gain
    }

    #[setter]
    pub fn set_gain(&self, gain: pyxel::ChannelGain) {
        unsafe { &mut *self.inner }.gain = gain;
    }

    #[getter]
    pub fn get_detune(&self) -> pyxel::ChannelDetune {
        unsafe { &*self.inner }.detune
    }

    #[setter]
    pub fn set_detune(&self, detune: pyxel::ChannelDetune) {
        unsafe { &mut *self.inner }.detune = detune;
    }

    #[pyo3(signature = (snd, sec=None, r#loop=None, resume=None, tick=None))]
    pub fn play(
        &self,
        snd: Bound<'_, PyAny>,
        sec: Option<f32>,
        r#loop: Option<bool>,
        resume: Option<bool>,
        tick: Option<u32>,
    ) -> PyResult<()> {
        let sec = if let Some(tick) = tick {
            PLAY_TICK_ONCE.call_once(|| {
                println!("tick option of Channel.play is deprecated. Use sec option instead.");
            });

            Some(tick as f32 / 120.0)
        } else {
            sec
        };

        let loop_ = r#loop.unwrap_or(false);

        cast_pyany! {
            snd,
            (u32, {
                let sound = pyxel::sounds().get(snd as usize).copied()
                    .ok_or_else(|| PyValueError::new_err("Invalid sound index"))?;
                unsafe { &mut *self.inner }.play_sound(sound, sec, loop_, resume.unwrap_or(false));
            }),
            (Vec<u32>, {
                let all_sounds = pyxel::sounds();
                for i in &snd {
                    if *i as usize >= all_sounds.len() {
                        return Err(PyValueError::new_err("Invalid sound index"));
                    }
                }
                let sounds = snd.iter().map(|i| all_sounds[*i as usize]).collect();
                unsafe { &mut *self.inner }.play(sounds, sec, loop_, resume.unwrap_or(false));
            }),
            (Sound, { unsafe { &mut *self.inner }.play_sound(snd.inner, sec, loop_, resume.unwrap_or(false)); }),
            (Vec<Sound>, {
                let sounds = snd.iter().map(|sound| sound.inner).collect();
                unsafe { &mut *self.inner }.play(sounds, sec, loop_, resume.unwrap_or(false));
            }),
            (String, {
                unsafe { &mut *self.inner }
                    .play_mml(&snd, sec, r#loop.unwrap_or(false), resume.unwrap_or(false))
                    .map_err(PyException::new_err)?;
            })
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        unsafe { &mut *self.inner }.stop();
    }

    pub fn play_pos(&self) -> Option<(u32, f32)> {
        unsafe { &mut *self.inner }.play_position()
    }
}

pub fn add_channel_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Channel>()?;
    Ok(())
}
