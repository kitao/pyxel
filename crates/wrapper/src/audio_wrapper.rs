use pyo3::prelude::*;

use crate::channel_wrapper::{wrap_pyxel_channel, Channel};
use crate::instance;
use crate::music_wrapper::{wrap_pyxel_music, Music};
use crate::sound_wrapper::{wrap_pyxel_sound, Sound};

#[pyfunction]
fn channel(ch: u32) -> PyResult<Channel> {
    Ok(wrap_pyxel_channel(instance().channel(ch)))
}

#[pyfunction]
fn sound(snd: u32) -> PyResult<Sound> {
    Ok(wrap_pyxel_sound(instance().sound(snd)))
}

#[pyfunction]
fn music(msc: u32) -> PyResult<Music> {
    Ok(wrap_pyxel_music(instance().music(msc)))
}

#[pyfunction]
fn play_pos(ch: u32) -> PyResult<Option<(u32, u32)>> {
    Ok(instance().play_pos(ch))
}

#[pyfunction]
fn play(ch: u32, snd: &PyAny, loop_: Option<bool>) -> PyResult<()> {
    type_switch! {
        snd,
        Vec<u32>,
        {
            instance().play(ch, &snd, loop_.unwrap_or(false));
        },
        u32,
        {
            instance().play1(ch, snd, loop_.unwrap_or(false));
        }
    }

    Ok(())
}

#[pyfunction]
fn playm(msc: u32, loop_: Option<bool>) -> PyResult<()> {
    instance().playm(msc, loop_.unwrap_or(false));

    Ok(())
}

#[pyfunction]
fn stop(ch: Option<u32>) -> PyResult<()> {
    if let Some(ch) = ch {
        instance().stop(ch);
    } else {
        instance().stop0();
    }

    Ok(())
}

pub fn add_audio_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(channel, m)?)?;
    m.add_function(wrap_pyfunction!(sound, m)?)?;
    m.add_function(wrap_pyfunction!(music, m)?)?;
    m.add_function(wrap_pyfunction!(play_pos, m)?)?;
    m.add_function(wrap_pyfunction!(play, m)?)?;
    m.add_function(wrap_pyfunction!(playm, m)?)?;
    m.add_function(wrap_pyfunction!(stop, m)?)?;

    Ok(())
}
