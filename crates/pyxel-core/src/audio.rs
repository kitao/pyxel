use std::array;

use crate::blipbuf::BlipBuf;
use crate::channel::{Channel, SharedChannel};
use crate::music::{Music, SharedMusic};
use crate::platform::{AudioCallback, Platform};
use crate::settings::{
    CLOCK_RATE, NUM_CHANNELS, NUM_CLOCKS_PER_TICK, NUM_MUSICS, NUM_SAMPLES, NUM_SOUNDS, SAMPLE_RATE,
};
use crate::sound::{SharedSound, Sound};

struct AudioCore {
    blip_buf: BlipBuf,
    channels: [SharedChannel; NUM_CHANNELS as usize],
}

impl AudioCallback for AudioCore {
    fn update(&mut self, out: &mut [i16]) {
        let mut samples = self.blip_buf.read_samples(out, false);
        while samples < out.len() {
            for channel in &mut self.channels {
                channel.lock().update(&mut self.blip_buf);
            }
            self.blip_buf.end_frame(NUM_CLOCKS_PER_TICK as u64);
            samples += self.blip_buf.read_samples(&mut out[samples..], false);
        }
    }
}

pub struct Audio {
    channels: [SharedChannel; NUM_CHANNELS as usize],
    sounds: [SharedSound; NUM_SOUNDS as usize],
    musics: [SharedMusic; NUM_MUSICS as usize],
}

unsafe_singleton!(Audio);

impl Audio {
    pub fn init() {
        let mut blip_buf = BlipBuf::new(NUM_SAMPLES as usize);
        blip_buf.set_rates(CLOCK_RATE as f64, SAMPLE_RATE as f64);
        let channels = array::from_fn(|_| Channel::new());
        let sounds = array::from_fn(|_| Sound::new());
        let musics = array::from_fn(|_| Music::new());

        Platform::instance().start_audio(
            SAMPLE_RATE,
            NUM_SAMPLES,
            new_shared_type!(AudioCore {
                blip_buf,
                channels: channels.clone(),
            }),
        );

        Self::set_instance(Self {
            channels,
            sounds,
            musics,
        });
    }
}

pub fn channel(channel_no: u32) -> SharedChannel {
    Audio::instance().channels[channel_no as usize].clone()
}

pub fn sound(sound_no: u32) -> SharedSound {
    Audio::instance().sounds[sound_no as usize].clone()
}

pub fn music(music_no: u32) -> SharedMusic {
    Audio::instance().musics[music_no as usize].clone()
}

pub fn play_pos(channel_no: u32) -> Option<(u32, u32)> {
    crate::channel(channel_no).lock().play_pos()
}

pub fn play(channel_no: u32, sequence: &[u32], start_tick: Option<u32>, should_loop: bool) {
    if sequence.is_empty() {
        return;
    }
    let sounds = sequence
        .iter()
        .map(|sound_no| crate::sound(*sound_no))
        .collect();
    crate::channel(channel_no)
        .lock()
        .play(sounds, start_tick, should_loop);
}

pub fn play1(channel_no: u32, sound_no: u32, start_tick: Option<u32>, should_loop: bool) {
    crate::channel(channel_no)
        .lock()
        .play1(crate::sound(sound_no), start_tick, should_loop);
}

pub fn playm(music_no: u32, start_tick: Option<u32>, should_loop: bool) {
    let music = crate::music(music_no);
    let music = music.lock();
    for i in 0..NUM_CHANNELS {
        crate::play(i, &music.sounds_list[i as usize], start_tick, should_loop);
    }
}

pub fn stop(channel_no: u32) {
    crate::channel(channel_no).lock().stop();
}

pub fn stop0() {
    for i in 0..NUM_CHANNELS {
        crate::stop(i);
    }
}
