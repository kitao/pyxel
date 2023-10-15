use pyo3::prelude::*;

use crate::channel_wrapper::{wrap_pyxel_channel, Channel};
use crate::music_wrapper::{wrap_pyxel_music, Music};
use crate::pyxel_singleton::pyxel;
use crate::sound_wrapper::{wrap_pyxel_sound, Sound};

#[pyfunction]
fn channel(ch: u32) -> Channel {
    wrap_pyxel_channel(pyxel().channels[ch as usize].clone())
}

#[pyfunction]
fn sound(snd: u32) -> Sound {
    wrap_pyxel_sound(pyxel().sounds[snd as usize].clone())
}

#[pyfunction]
fn music(msc: u32) -> Music {
    wrap_pyxel_music(pyxel().musics[msc as usize].clone())
}

#[pyfunction]
#[pyo3(text_signature = "(ch, snd, *, tick, loop)")]
fn play(ch: u32, snd: &PyAny, tick: Option<u32>, r#loop: Option<bool>) -> PyResult<()> {
    type_switch! {
        snd,
        u32, {
            pyxel().play1(ch, snd, tick, r#loop.unwrap_or(false));
        },
        Vec<u32>, {
            pyxel().play(ch, &snd, tick, r#loop.unwrap_or(false));
        },
        Sound, {
            pyxel().channels[ch as usize].lock().play1(snd.pyxel_sound, tick, r#loop.unwrap_or(false));
        },
        Vec<Sound>, {
            let sounds = snd.iter().map(|sound| sound.pyxel_sound.clone()).collect();
            pyxel().channels[ch as usize].lock().play(sounds, tick, r#loop.unwrap_or(false));
        }
    }
    Ok(())
}

#[pyfunction]
#[pyo3(text_signature = "(msc, *, tick, loop)")]
fn playm(msc: u32, tick: Option<u32>, r#loop: Option<bool>) {
    pyxel().playm(msc, tick, r#loop.unwrap_or(false));
}

#[pyfunction]
fn stop(ch: Option<u32>) {
    if let Some(ch) = ch {
        pyxel().stop(ch);
    } else {
        pyxel().stop0();
    }
}

#[pyfunction]
fn play_pos(ch: u32) -> Option<(u32, u32)> {
    pyxel().play_pos(ch)
}

pub fn add_audio_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(channel, m)?)?;
    m.add_function(wrap_pyfunction!(sound, m)?)?;
    m.add_function(wrap_pyfunction!(music, m)?)?;
    m.add_function(wrap_pyfunction!(play, m)?)?;
    m.add_function(wrap_pyfunction!(playm, m)?)?;
    m.add_function(wrap_pyfunction!(stop, m)?)?;
    m.add_function(wrap_pyfunction!(play_pos, m)?)?;
    Ok(())
}
