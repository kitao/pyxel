use std::sync::Once;

use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;

use crate::channel_wrapper::Channel;
use crate::music_wrapper::Music;
use crate::pyxel_singleton::pyxel;
use crate::sound_wrapper::Sound;

static PLAY_TICK_ONCE: Once = Once::new();
static PLAYM_TICK_ONCE: Once = Once::new();
static CHANNEL_ONCE: Once = Once::new();
static SOUND_ONCE: Once = Once::new();
static MUSIC_ONCE: Once = Once::new();

#[pyfunction]
#[pyo3(signature = (ch, snd, sec=None, r#loop=None, resume=None, tick=None))]
fn play(
    ch: u32,
    snd: Bound<'_, PyAny>,
    sec: Option<f32>,
    r#loop: Option<bool>,
    resume: Option<bool>,
    tick: Option<u32>,
) -> PyResult<()> {
    let sec = if let Some(tick) = tick {
        PLAY_TICK_ONCE.call_once(|| {
            println!("tick option of pyxel.play is deprecated. Use sec option instead.");
        });

        Some(tick as f32 / 120.0)
    } else {
        sec
    };

    if ch as usize >= pyxel::channels().len() {
        return Err(PyValueError::new_err("Invalid channel index"));
    }

    cast_pyany! {
        snd,
        (u32, {
            if snd as usize >= pyxel::sounds().len() {
                return Err(PyValueError::new_err("Invalid sound index"));
            }
            pyxel().play_sound(ch, snd, sec, r#loop.unwrap_or(false), resume.unwrap_or(false));
        }),
        (Vec<u32>, {
            let num_sounds = pyxel::sounds().len();
            for &s in &snd {
                if s as usize >= num_sounds {
                    return Err(PyValueError::new_err("Invalid sound index"));
                }
            }
            pyxel().play(ch, &snd, sec, r#loop.unwrap_or(false), resume.unwrap_or(false));
        }),
        (Sound, {
            unsafe { &mut *pyxel::channels()[ch as usize] }.play_sound(snd.inner, sec, r#loop.unwrap_or(false), resume.unwrap_or(false));
        }),
        (Vec<Sound>, {
            let sounds = snd.iter().map(|sound| sound.inner).collect();
            unsafe { &mut *pyxel::channels()[ch as usize] }.play(sounds, sec, r#loop.unwrap_or(false), resume.unwrap_or(false));
        }),
        (String, {
            pyxel()
                .play_mml(ch, &snd, sec, r#loop.unwrap_or(false), resume.unwrap_or(false))
                .map_err(PyException::new_err)?;
        })
    }

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (msc, sec=None, r#loop=None, tick=None))]
fn playm(msc: u32, sec: Option<f32>, r#loop: Option<bool>, tick: Option<u32>) -> PyResult<()> {
    let sec = if let Some(tick) = tick {
        PLAYM_TICK_ONCE.call_once(|| {
            println!("tick option of pyxel.playm is deprecated. Use sec option instead.");
        });

        Some(tick as f32 / 120.0)
    } else {
        sec
    };

    if msc as usize >= pyxel::musics().len() {
        return Err(PyValueError::new_err("Invalid music index"));
    }

    pyxel().play_music(msc, sec, r#loop.unwrap_or(false));
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (ch=None))]
fn stop(ch: Option<u32>) -> PyResult<()> {
    if let Some(ch) = ch {
        if ch as usize >= pyxel::channels().len() {
            return Err(PyValueError::new_err("Invalid channel index"));
        }
        pyxel().stop_channel(ch);
    } else {
        pyxel().stop_all_channels();
    }
    Ok(())
}

#[pyfunction]
fn play_pos(ch: u32) -> PyResult<Option<(u32, f32)>> {
    if ch as usize >= pyxel::channels().len() {
        return Err(PyValueError::new_err("Invalid channel index"));
    }
    Ok(pyxel().play_position(ch))
}

#[pyfunction]
fn channel(ch: u32) -> PyResult<Channel> {
    CHANNEL_ONCE.call_once(|| {
        println!("pyxel.channel(ch) is deprecated. Use pyxel.channels[ch] instead.");
    });

    pyxel::channels()
        .get(ch as usize)
        .copied()
        .map(Channel::wrap)
        .ok_or_else(|| PyValueError::new_err("Invalid channel index"))
}

#[pyfunction]
fn sound(snd: u32) -> PyResult<Sound> {
    SOUND_ONCE.call_once(|| {
        println!("pyxel.sound(snd) is deprecated. Use pyxel.sounds[snd] instead.");
    });

    pyxel::sounds()
        .get(snd as usize)
        .copied()
        .map(Sound::wrap)
        .ok_or_else(|| PyValueError::new_err("Invalid sound index"))
}

#[pyfunction]
fn music(msc: u32) -> PyResult<Music> {
    MUSIC_ONCE.call_once(|| {
        println!("pyxel.music(msc) is deprecated. Use pyxel.musics[msc] instead.");
    });

    pyxel::musics()
        .get(msc as usize)
        .copied()
        .map(Music::wrap)
        .ok_or_else(|| PyValueError::new_err("Invalid music index"))
}

#[pyfunction]
#[pyo3(signature = (preset, instr, seed=None, play=None))]
fn gen_bgm(preset: i32, instr: i32, seed: Option<u64>, play: Option<bool>) -> Vec<String> {
    pyxel().gen_bgm(preset, instr, seed, play)
}

pub fn add_audio_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(play, m)?)?;
    m.add_function(wrap_pyfunction!(playm, m)?)?;
    m.add_function(wrap_pyfunction!(stop, m)?)?;
    m.add_function(wrap_pyfunction!(play_pos, m)?)?;
    m.add_function(wrap_pyfunction!(gen_bgm, m)?)?;

    // Deprecated functions
    m.add_function(wrap_pyfunction!(channel, m)?)?;
    m.add_function(wrap_pyfunction!(sound, m)?)?;
    m.add_function(wrap_pyfunction!(music, m)?)?;

    Ok(())
}
