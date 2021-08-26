use array_macro::array;
use blip_buf::BlipBuf;
use parking_lot::Mutex;
use std::sync::Arc;

use crate::channel::{Channel, SharedChannel};
use crate::music::{Music, SharedMusic};
use crate::platform::{AudioCallback, Platform};
use crate::settings::{
    CHANNEL_COUNT, CLOCK_RATE, MUSIC_COUNT, SAMPLE_COUNT, SAMPLE_RATE, SOUND_COUNT,
    TICK_CLOCK_COUNT,
};
use crate::sound::{SharedSound, Sound};
use crate::Pyxel;

struct AudioCore {
    blip_buf: BlipBuf,
    channels: [SharedChannel; CHANNEL_COUNT as usize],
}

pub struct Audio {
    channels: [SharedChannel; CHANNEL_COUNT as usize],
    sounds: [SharedSound; SOUND_COUNT as usize],
    musics: [SharedMusic; MUSIC_COUNT as usize],
}

impl Audio {
    pub fn new<T: Platform>(platform: &mut T) -> Audio {
        let mut blip_buf = BlipBuf::new(SAMPLE_COUNT);
        let channels = array![_ => Channel::new(); CHANNEL_COUNT as usize];
        let sounds = array![_ => Sound::new(); SOUND_COUNT as usize];
        let musics = array![_ => Music::new(); MUSIC_COUNT as usize];

        blip_buf.set_rates(CLOCK_RATE as f64, SAMPLE_RATE as f64);

        let audio_core = Arc::new(Mutex::new(AudioCore {
            blip_buf: blip_buf,
            channels: channels.clone(),
        }));

        let audio = Audio {
            channels: channels,
            sounds: sounds,
            musics: musics,
        };

        platform.start_audio(SAMPLE_RATE, SAMPLE_COUNT, audio_core);

        audio
    }
}

impl AudioCallback for AudioCore {
    fn update(&mut self, out: &mut [i16]) {
        let mut samples = self.blip_buf.read_samples(out, false);

        while samples < out.len() {
            for channel in &mut self.channels {
                channel.lock().update(&mut self.blip_buf);
            }

            self.blip_buf.end_frame(TICK_CLOCK_COUNT);

            samples += self.blip_buf.read_samples(&mut out[samples..], false);
        }
    }
}

impl Pyxel {
    pub fn channel(&self, channel_no: u32) -> SharedChannel {
        self.audio.channels[channel_no as usize].clone()
    }

    pub fn sound(&self, sound_no: u32) -> SharedSound {
        self.audio.sounds[sound_no as usize].clone()
    }

    pub fn music(&self, music_no: u32) -> SharedMusic {
        self.audio.musics[music_no as usize].clone()
    }

    pub fn play(&mut self, channel: u32, sequence: &[u32], is_looping: bool) {
        if sequence.len() == 0 {
            return;
        }

        let sounds = sequence
            .iter()
            .map(|sound_no| self.audio.sounds[*sound_no as usize].lock().clone())
            .collect();

        self.audio.channels[channel as usize]
            .lock()
            .play(sounds, is_looping);
    }

    pub fn play1(&mut self, channel: u32, sound_no: u32, is_looping: bool) {
        self.audio.channels[channel as usize].lock().play1(
            self.audio.sounds[sound_no as usize].lock().clone(),
            is_looping,
        );
    }

    pub fn playm(&mut self, music_no: u32, is_looping: bool) {
        let music = self.audio.musics[music_no as usize].clone();

        for i in 0..CHANNEL_COUNT {
            self.play(i, &music.lock().sequences[i as usize], is_looping);
        }
    }

    pub fn stop(&mut self, channel_no: u32) {
        self.audio.channels[channel_no as usize].lock().stop();
    }

    pub fn stop0(&mut self) {
        for i in 0..CHANNEL_COUNT {
            self.stop(i);
        }
    }
}
