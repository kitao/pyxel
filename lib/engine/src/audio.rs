use array_macro::array;
use blip_buf::BlipBuf;

use crate::channel::{Channel, SharedChannel};
use crate::music::{Music, SharedMusic};
use crate::platform::{AudioCallback, Platform};
use crate::settings::{
    CLOCK_RATE, NUM_CHANNELS, NUM_CLOCKS_PER_TICK, NUM_MUSICS, NUM_SAMPLES, NUM_SOUNDS, SAMPLE_RATE,
};
use crate::sound::{SharedSound, Sound};
use crate::Pyxel;

struct AudioCore {
    blip_buf: BlipBuf,
    channels: [SharedChannel; NUM_CHANNELS as usize],
}

pub struct Audio {
    channels: [SharedChannel; NUM_CHANNELS as usize],
    sounds: [SharedSound; NUM_SOUNDS as usize],
    musics: [SharedMusic; NUM_MUSICS as usize],
}

impl Audio {
    pub fn new<T: Platform>(platform: &mut T) -> Self {
        let mut blip_buf = BlipBuf::new(NUM_SAMPLES);
        blip_buf.set_rates(CLOCK_RATE as f64, SAMPLE_RATE as f64);
        let channels = array![_ => Channel::new(); NUM_CHANNELS as usize];
        let sounds = array![_ => Sound::new(); NUM_SOUNDS as usize];
        let musics = array![_ => Music::new(); NUM_MUSICS as usize];

        platform.start_audio(
            SAMPLE_RATE,
            NUM_SAMPLES,
            new_shared_type!(AudioCore {
                blip_buf,
                channels: channels.clone(),
            }),
        );

        Self {
            channels,
            sounds,
            musics,
        }
    }
}

impl AudioCallback for AudioCore {
    fn update(&mut self, out: &mut [i16]) {
        let mut samples = self.blip_buf.read_samples(out, false);
        while samples < out.len() {
            for channel in &mut self.channels {
                channel.lock().update(&mut self.blip_buf);
            }
            self.blip_buf.end_frame(NUM_CLOCKS_PER_TICK);
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

    pub fn play_pos(&mut self, channel_no: u32) -> Option<(u32, u32)> {
        self.audio.channels[channel_no as usize].lock().play_pos()
    }

    pub fn play(
        &mut self,
        channel_no: u32,
        sequence: &[u32],
        start_tick: Option<u32>,
        should_loop: bool,
    ) {
        if sequence.is_empty() {
            return;
        }
        let sounds = sequence
            .iter()
            .map(|sound_no| self.audio.sounds[*sound_no as usize].clone())
            .collect();
        self.audio.channels[channel_no as usize]
            .lock()
            .play(sounds, start_tick, should_loop);
    }

    pub fn play1(
        &mut self,
        channel_no: u32,
        sound_no: u32,
        start_tick: Option<u32>,
        should_loop: bool,
    ) {
        self.audio.channels[channel_no as usize].lock().play1(
            self.audio.sounds[sound_no as usize].clone(),
            start_tick,
            should_loop,
        );
    }

    pub fn playm(&mut self, music_no: u32, start_tick: Option<u32>, should_loop: bool) {
        let music = self.audio.musics[music_no as usize].clone();
        for i in 0..NUM_CHANNELS {
            self.play(i, &music.lock().sounds[i as usize], start_tick, should_loop);
        }
    }

    pub fn stop(&mut self, channel_no: u32) {
        self.audio.channels[channel_no as usize].lock().stop();
    }

    pub fn stop0(&mut self) {
        for i in 0..NUM_CHANNELS {
            self.stop(i);
        }
    }
}
