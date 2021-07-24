use std::sync::{Arc, Mutex};

use blip_buf::BlipBuf;

use crate::channel::Channel;
use crate::platform::AudioCallback;
use crate::settings::{
    CHANNEL_COUNT, CLOCK_RATE, MUSIC_COUNT, SAMPLE_COUNT, SAMPLE_RATE, TICK_CLOCK_COUNT,
};
use crate::sound::Sound;
use crate::Pyxel;

pub struct Audio {
    blip_buf: BlipBuf,
    channels: Vec<Channel>,
}

impl AudioCallback for Audio {
    fn audio_callback(&mut self, out: &mut [i16]) {
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

impl Audio {
    pub fn new() -> Arc<Mutex<Audio>> {
        let mut blip_buf = BlipBuf::new(SAMPLE_COUNT);
        blip_buf.set_rates(CLOCK_RATE as f64, SAMPLE_RATE as f64);

        let channels = (0..CHANNEL_COUNT).map(|_| Channel::new()).collect();

        Arc::new(Mutex::new(Audio {
            blip_buf: blip_buf,
            channels: channels,
        }))
    }
}

impl Pyxel {
    pub fn playing(&self, ch: u32) -> bool {
        self.audio.lock().unwrap().channels[ch as usize].is_playing()
    }

    pub fn looping(&self, ch: u32) -> bool {
        self.audio.lock().unwrap().channels[ch as usize].is_looping()
    }

    pub fn play_pos(&self, ch: u32) -> (u32, u32) {
        let channel = &self.audio.lock().unwrap().channels[ch as usize];
        (channel.sound_index(), channel.note_index())
    }

    pub fn play(&mut self, ch: u32, seq: &[u32], looping: bool) {
        let mut sound_list: Vec<Sound> = Vec::new();

        for snd in seq {
            sound_list.push(self.sound[*snd as usize].clone());
        }

        self.audio.lock().unwrap().channels[ch as usize].play(sound_list, looping);
    }

    pub fn playm(&mut self, msc: u32, looping: bool) {
        for i in 0..MUSIC_COUNT {
            let seq = self.music[msc as usize].seq[i as usize].clone();
            self.play(i, &seq, looping);
        }
    }

    pub fn stop(&mut self, ch: u32) {
        self.audio.lock().unwrap().channels[ch as usize].stop();
    }

    pub fn stop_(&mut self) {
        for i in 0..CHANNEL_COUNT {
            self.stop(i);
        }
    }
}
