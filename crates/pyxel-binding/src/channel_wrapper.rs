use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;

use crate::sound_wrapper::Sound;

define_wrapper!(Channel, pyxel::Channel);

#[pymethods]
impl Channel {
    // Constructor

    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::Channel::new())
    }

    // Properties

    #[getter]
    fn gain(&self) -> pyxel::ChannelGain {
        self.inner_ref().gain
    }

    #[setter]
    fn set_gain(&self, gain: pyxel::ChannelGain) {
        self.inner_mut().gain = gain;
    }

    #[getter]
    fn detune(&self) -> pyxel::ChannelDetune {
        self.inner_ref().detune
    }

    #[setter]
    fn set_detune(&self, detune: pyxel::ChannelDetune) {
        self.inner_mut().detune = detune;
    }

    // Playback

    #[pyo3(signature = (snd, sec=None, r#loop=None, resume=None, tick=None))]
    fn play(
        &self,
        snd: Bound<'_, PyAny>,
        sec: Option<f32>,
        r#loop: Option<bool>,
        resume: Option<bool>,
        tick: Option<u32>,
    ) -> PyResult<()> {
        let sec = if let Some(tick) = tick {
            deprecation_warning!(
                PLAY_TICK_ONCE,
                "tick option of Channel.play is deprecated. Use sec option instead."
            );
            Some(tick as f32 / 120.0)
        } else {
            sec
        };
        let should_loop = r#loop.unwrap_or(false);
        let resume = resume.unwrap_or(false);

        cast_pyany! {
            snd,

            (u32, {
                let sound = pyxel::sounds().get(snd as usize).copied()
                    .ok_or_else(|| PyValueError::new_err("Invalid sound index"))?;
                self.inner_mut().play_sound(sound, sec, should_loop, resume);
            }),

            (Vec<u32>, {
                let all_sounds = pyxel::sounds();
                for &i in &snd {
                    validate_index!(i, all_sounds.len(), "sound");
                }
                let sounds = snd.iter().map(|&i| all_sounds[i as usize]).collect();
                self.inner_mut().play(sounds, sec, should_loop, resume);
            }),

            (Sound, {
                self.inner_mut().play_sound(snd.inner, sec, should_loop, resume);
            }),

            (Vec<Sound>, {
                let sounds = snd.iter().map(|sound| sound.inner).collect();
                self.inner_mut().play(sounds, sec, should_loop, resume);
            }),

            (String, {
                self.inner_mut()
                    .play_mml(&snd, sec, should_loop, resume)
                    .map_err(PyException::new_err)?;
            })
        }

        Ok(())
    }

    fn stop(&self) {
        self.inner_mut().stop();
    }

    fn play_pos(&self) -> Option<(u32, f32)> {
        self.inner_mut().play_position()
    }
}

pub fn add_channel_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Channel>()?;
    Ok(())
}
