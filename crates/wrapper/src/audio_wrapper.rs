use pyo3::prelude::*;

use crate::channel_wrapper::{wrap_pyxel_channel, Channel};
use crate::instance;
use crate::music_wrapper::{wrap_pyxel_music, Music};
use crate::sound_wrapper::{wrap_pyxel_sound, Sound};

#[pyfunction]
fn channel(channel_no: u32) -> Channel {
    wrap_pyxel_channel(instance().channel(channel_no))
}

#[pyfunction]
fn sound(sound_no: u32) -> Sound {
    wrap_pyxel_sound(instance().sound(sound_no))
}

#[pyfunction]
fn music(music_no: u32) -> Music {
    wrap_pyxel_music(instance().music(music_no))
}

#[pyfunction]
fn play(channel: u32, sequence: &PyAny, is_looping: Option<bool>) {
    if let Ok(sequence) = sequence.extract::<Vec<u32>>() {
        instance().play(channel, &sequence, is_looping.unwrap_or(false));
    } else if let Ok(sound_no) = sequence.extract::<u32>() {
        instance().play1(channel, sound_no, is_looping.unwrap_or(false));
    } else {
        panic!("Invalid arguments for play");
    }
}

#[pyfunction]
fn playm(music_no: u32, is_looping: Option<bool>) {
    instance().playm(music_no, is_looping.unwrap_or(false));
}

#[pyfunction]
fn stop(channel_no: Option<u32>) {
    if let Some(channel_no) = channel_no {
        instance().stop(channel_no);
    } else {
        instance().stop0();
    }
}

pub fn add_audio_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(channel, m)?)?;
    m.add_function(wrap_pyfunction!(sound, m)?)?;
    m.add_function(wrap_pyfunction!(music, m)?)?;
    m.add_function(wrap_pyfunction!(play, m)?)?;
    m.add_function(wrap_pyfunction!(playm, m)?)?;
    m.add_function(wrap_pyfunction!(stop, m)?)?;

    Ok(())
}
