use std::sync::Once;

use pyo3::prelude::*;

use crate::channel_wrapper::Channel;
use crate::music_wrapper::Music;
use crate::pyxel_singleton::pyxel;
use crate::sound_wrapper::Sound;

static CHANNEL_ONCE: Once = Once::new();
static SOUND_ONCE: Once = Once::new();
static MUSIC_ONCE: Once = Once::new();

#[pyfunction]
#[pyo3(signature = (ch, snd, tick=None, r#loop=None, resume=None))]
fn play(
    ch: u32,
    snd: Bound<'_, PyAny>,
    tick: Option<u32>,
    r#loop: Option<bool>,
    resume: Option<bool>,
) -> PyResult<()> {
    cast_pyany! {
        snd,
        (u32, { pyxel().play1(ch, snd, tick, r#loop.unwrap_or(false), resume.unwrap_or(false)); }),
        (Vec<u32>, { pyxel().play(ch, &snd, tick, r#loop.unwrap_or(false), resume.unwrap_or(false)); }),
        (Sound, { pyxel().channels.lock()[ch as usize].lock().play1(snd.inner, tick, r#loop.unwrap_or(false), resume.unwrap_or(false)); }),
        (Vec<Sound>, {
            let sounds = snd.iter().map(|sound| sound.inner.clone()).collect();
            pyxel().channels.lock()[ch as usize].lock().play(sounds, tick, r#loop.unwrap_or(false), resume.unwrap_or(false));
        })
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (msc, tick=None, r#loop=None))]
fn playm(msc: u32, tick: Option<u32>, r#loop: Option<bool>) {
    pyxel().playm(msc, tick, r#loop.unwrap_or(false));
}

#[pyfunction]
#[pyo3(signature = (ch=None))]
fn stop(ch: Option<u32>) {
    ch.map_or_else(
        || {
            pyxel().stop0();
        },
        |ch| {
            pyxel().stop(ch);
        },
    );
}

#[pyfunction]
fn play_pos(ch: u32) -> Option<(u32, u32)> {
    pyxel().play_pos(ch)
}

#[pyfunction]
fn channel(ch: u32) -> Channel {
    CHANNEL_ONCE.call_once(|| {
        println!("pyxel.channel(ch) is deprecated, use pyxel.channels[ch] instead.");
    });
    Channel::wrap(pyxel().channels.lock()[ch as usize].clone())
}

#[pyfunction]
fn sound(snd: u32) -> Sound {
    SOUND_ONCE.call_once(|| {
        println!("pyxel.sound(snd) is deprecated, use pyxel.sounds[snd] instead.");
    });
    Sound::wrap(pyxel().sounds.lock()[snd as usize].clone())
}

#[pyfunction]
fn music(msc: u32) -> Music {
    MUSIC_ONCE.call_once(|| {
        println!("pyxel.music(msc) is deprecated, use pyxel.musics[msc] instead.");
    });
    Music::wrap(pyxel().musics.lock()[msc as usize].clone())
}

pub fn add_audio_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(play, m)?)?;
    m.add_function(wrap_pyfunction!(playm, m)?)?;
    m.add_function(wrap_pyfunction!(stop, m)?)?;
    m.add_function(wrap_pyfunction!(play_pos, m)?)?;

    // Deprecated functions
    m.add_function(wrap_pyfunction!(channel, m)?)?;
    m.add_function(wrap_pyfunction!(sound, m)?)?;
    m.add_function(wrap_pyfunction!(music, m)?)?;

    Ok(())
}
