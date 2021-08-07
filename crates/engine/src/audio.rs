use array_macro::array;
use blip_buf::BlipBuf;
use std::sync::{Arc, Mutex};

use crate::channel::Channel;
use crate::platform::{AudioCallback, Platform};
use crate::settings::{
    CHANNEL_COUNT, CLOCK_RATE, MUSIC_COUNT, SAMPLE_COUNT, SAMPLE_RATE, TICK_CLOCK_COUNT,
};
use crate::Pyxel;

pub struct Audio {
    blip_buf: BlipBuf,
    channels: [Channel; CHANNEL_COUNT as usize],
}

pub type AtomicAudio = Arc<Mutex<Audio>>;

impl Audio {
    pub fn new<T: Platform>(platform: &mut T) -> AtomicAudio {
        let mut blip_buf = BlipBuf::new(SAMPLE_COUNT);
        let channels = array![_ => Channel::new(); CHANNEL_COUNT as usize];

        blip_buf.set_rates(CLOCK_RATE as f64, SAMPLE_RATE as f64);

        let audio = Arc::new(Mutex::new(Audio {
            blip_buf: blip_buf,
            channels: channels,
        }));

        platform.start_audio(SAMPLE_RATE, SAMPLE_COUNT, audio.clone());

        audio
    }
}

impl AudioCallback for Audio {
    fn update(&mut self, out: &mut [i16]) {
        let mut samples = self.blip_buf.read_samples(out, false);

        while samples < out.len() {
            for channel in &mut self.channels {
                channel.update(&mut self.blip_buf);
            }

            self.blip_buf.end_frame(TICK_CLOCK_COUNT);
            samples += self.blip_buf.read_samples(&mut out[samples..], false);
        }
    }
}

impl Pyxel {
    pub fn is_playing(&self, channel: u32) -> bool {
        self.audio.lock().unwrap().channels[channel as usize].is_playing()
    }

    pub fn is_looping(&self, channel: u32) -> bool {
        self.audio.lock().unwrap().channels[channel as usize].is_looping()
    }

    pub fn play_pos(&self, channel: u32) -> (u32, u32) {
        let channel = &self.audio.lock().unwrap().channels[channel as usize];
        (channel.sound_index(), channel.note_index())
    }

    pub fn play(&mut self, channel: u32, sequence: &[u32], is_looping: bool) {
        let sounds = sequence
            .iter()
            .map(|sound_no| self.sounds[*sound_no as usize].borrow().clone())
            .collect();

        self.audio.lock().unwrap().channels[channel as usize].play(sounds, is_looping);
    }

    pub fn playm(&mut self, music_no: u32, looping: bool) {
        for i in 0..MUSIC_COUNT {
            let music = self.musics[music_no as usize].clone();
            self.play(i, &music.borrow().sequences[i as usize], looping);
        }
    }

    pub fn stop(&mut self, channel: u32) {
        self.audio.lock().unwrap().channels[channel as usize].stop();
    }

    pub fn stop_(&mut self) {
        for i in 0..CHANNEL_COUNT {
            self.stop(i);
        }
    }
}
